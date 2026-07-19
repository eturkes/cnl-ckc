:- module(inference_kernel, [validate_program_terms/4, run_terms/2]).

:- set_prolog_flag(encoding, utf8).

:- use_module(library(lists), [append/3]).
:- use_module(explanation, [assemble_result_terms/6]).

:- dynamic cnl_program_db:program_clause/4.

/*
Program-record validator and deterministic positive-Datalog kernel. Input terms
have already passed strict UTF-8, syntax, and canonical-byte framing gates.
Validation rejects with ir_reject(Class, Detail). Evaluation installs only data
facts in private module cnl_program_db and tears them down around every run.
*/
validate_program_terms(Terms, Document, Clauses, Goal) :-
    envelope_pass(Terms, Document, Items),
    shape_pass(Document, Items),
    identity_pass(Document, Items),
    ordering_pass(Items),
    scope_pass(Items),
    safety_naf_pass(Items),
    cycle_pass(Items),
    collect_program(Items, 1, Clauses, Goal).

run_terms(Terms, ResultTerms) :-
    validate_program_terms(Terms, Document, Clauses, Goal),
    length(Clauses, ClauseCount),
    setup_call_cleanup(
        install_program(Clauses),
        evaluate_program(Document, Goal, ClauseCount, ResultTerms),
        teardown_program).

/* Pass 4: exact envelope, one final goal, and facts before rules. */
envelope_pass([], _, _) :-
    reject(envelope, term(1, missing_header)).
envelope_pass([Header|Rest], Document, Items) :-
    ( Header == cnl_program_record(1) ->
        true
    ; reject(envelope, term(1, expected(cnl_program_record(1))))
    ),
    require_document(Rest, Document, RawItems),
    index_terms(RawItems, 3, Items),
    goal_positions(Items, GoalPositions),
    require_one_goal(GoalPositions),
    split_goal(Items, Prefix, _Goal, Tail),
    ( Tail == [] ->
        true
    ; Tail = [indexed(Index, _)|_],
      reject(envelope, term(Index, trailing_after_goal))
    ),
    section_order(Prefix, facts).

require_document([], _, _) :-
    reject(envelope, term(2, missing_document)).
require_document([Document|Items], Document, Items) :-
    ( has_functor(Document, document, 3) ->
        true
    ; reject(envelope, term(2, expected(document/3)))
    ).

index_terms([], _, []).
index_terms([Term|Terms], Index, [indexed(Index, Term)|Indexed]) :-
    Next is Index + 1,
    index_terms(Terms, Next, Indexed).

goal_positions([], []).
goal_positions([indexed(Index, Term)|Items], Positions) :-
    ( has_functor(Term, goal, 2) ->
        Positions = [Index|Rest]
    ; Positions = Rest
    ),
    goal_positions(Items, Rest).

require_one_goal([_]) :-
    !.
require_one_goal(Positions) :-
    length(Positions, Count),
    ( Positions = [_, Second|_] ->
        reject(query_count, count(Count, second_term(Second)))
    ; reject(query_count, count(Count))
    ).

split_goal([Item|Items], Prefix, Goal, Tail) :-
    Item = indexed(_, Term),
    ( has_functor(Term, goal, 2) ->
        Prefix = [],
        Goal = Item,
        Tail = Items
    ; Prefix = [Item|Rest],
      split_goal(Items, Rest, Goal, Tail)
    ).

section_order([], _).
section_order([indexed(Index, Term)|Items], State0) :-
    clause_section(Term, Section),
    ( Section == fact ->
        ( State0 == rules ->
            reject(section_order, term(Index, fact_after_rule))
        ; State = facts
        )
    ; Section == rule ->
        State = rules
    ; State = State0
    ),
    section_order(Items, State).

clause_section(Term, fact) :-
    has_functor(Term, clause, 3),
    arg(1, Term, Id),
    has_functor(Id, fact_id, 2),
    !.
clause_section(Term, rule) :-
    has_functor(Term, clause, 3),
    arg(1, Term, Id),
    has_functor(Id, rule_id, 2),
    !.
clause_section(_, other).

/* Pass 5: exact constructors, proper lists, and admitted atomic kinds. */
shape_pass(Document, Items) :-
    ( shape_document(Document) ->
        true
    ; reject(shape, term(2, document))
    ),
    shape_items(Items).

shape_document(Document) :-
    has_functor(Document, document, 3),
    arg(1, Document, Docid),
    arg(2, Document, SourceHash),
    arg(3, Document, Ulex),
    shape_atom_wrapper(Docid, docid),
    shape_atom_wrapper(SourceHash, source_sha256),
    shape_ulex(Ulex).

shape_atom_wrapper(Term, Name) :-
    has_functor(Term, Name, 1),
    arg(1, Term, Value),
    atom(Value).

shape_ulex(Ulex) :-
    Ulex == ulex(none),
    !.
shape_ulex(Ulex) :-
    has_functor(Ulex, ulex, 1),
    arg(1, Ulex, Sha256),
    shape_atom_wrapper(Sha256, sha256).

shape_items([]).
shape_items([indexed(Index, Term)|Items]) :-
    ( has_functor(Term, clause, 3) ->
        ( shape_clause(Term) -> true
        ; reject(shape, term(Index, clause))
        )
    ; has_functor(Term, goal, 2) ->
        ( shape_goal(Term) -> true
        ; reject(shape, term(Index, goal))
        )
    ; reject(shape, term(Index, item))
    ),
    shape_items(Items).

shape_clause(Term) :-
    arg(1, Term, Id),
    arg(2, Term, Head),
    arg(3, Term, Body),
    shape_id(Id),
    shape_predicate(Head),
    shape_body(Body).

shape_goal(Term) :-
    arg(1, Term, Id),
    arg(2, Term, Predicate),
    shape_id(Id),
    shape_predicate(Predicate).

shape_id(Id) :-
    compound(Id),
    functor(Id, Name, 2),
    admitted_id_name(Name),
    arg(1, Id, Sentence),
    arg(2, Id, Clause),
    shape_integer_wrapper(Sentence, sentence),
    shape_integer_wrapper(Clause, clause).

admitted_id_name(fact_id).
admitted_id_name(rule_id).
admitted_id_name(query_id).

shape_integer_wrapper(Term, Name) :-
    has_functor(Term, Name, 1),
    arg(1, Term, Value),
    integer(Value).

shape_predicate(Predicate) :-
    has_functor(Predicate, pred, 2),
    arg(1, Predicate, Name),
    arg(2, Predicate, Args),
    atom(Name),
    is_list(Args),
    Args = [_|_],
    shape_args(Args).

shape_args([]).
shape_args([Arg|Args]) :-
    shape_arg(Arg),
    shape_args(Args).

shape_arg(Arg) :-
    has_functor(Arg, named, 1),
    arg(1, Arg, Name),
    atom(Name),
    !.
shape_arg(Arg) :-
    has_functor(Arg, var, 1),
    arg(1, Arg, Number),
    integer(Number).

shape_body(Body) :-
    has_functor(Body, body, 1),
    arg(1, Body, Literals),
    is_list(Literals),
    shape_literals(Literals).

shape_literals([]).
shape_literals([Literal|Literals]) :-
    shape_literal(Literal),
    shape_literals(Literals).

shape_literal(Literal) :-
    has_functor(Literal, pred, 2),
    shape_predicate(Literal),
    !.
shape_literal(Literal) :-
    has_functor(Literal, naf, 1),
    arg(1, Literal, Predicate),
    shape_predicate(Predicate).

/* Pass 6: document identity, ID kinds, and positive ordinals. */
identity_pass(Document, Items) :-
    arg(1, Document, DocidTerm),
    arg(2, Document, SourceHashTerm),
    arg(3, Document, UlexTerm),
    arg(1, DocidTerm, Docid),
    arg(1, SourceHashTerm, SourceHash),
    arg(1, UlexTerm, Ulex),
    ( valid_docid(Docid) -> true
    ; reject(identity, term(2, docid))
    ),
    ( valid_sha256(SourceHash) -> true
    ; reject(identity, term(2, source_sha256))
    ),
    ( valid_ulex_identity(Ulex) -> true
    ; reject(identity, term(2, ulex))
    ),
    identity_items(Items).

valid_docid(Docid) :-
    atom_codes(Docid, [First|Rest]),
    First =\= 0'-,
    docid_code(First),
    docid_codes(Rest).

docid_codes([]).
docid_codes([Code|Codes]) :-
    docid_code(Code),
    docid_codes(Codes).

docid_code(Code) :-
    ( Code >= 0'a, Code =< 0'z
    ; Code >= 0'0, Code =< 0'9
    ; Code =:= 0'-
    ).

valid_sha256(Hash) :-
    atom_codes(Hash, Codes),
    length(Codes, 64),
    lower_hex_codes(Codes).

lower_hex_codes([]).
lower_hex_codes([Code|Codes]) :-
    ( Code >= 0'0, Code =< 0'9
    ; Code >= 0'a, Code =< 0'f
    ),
    lower_hex_codes(Codes).

valid_ulex_identity(none).
valid_ulex_identity(sha256(Hash)) :-
    valid_sha256(Hash).

identity_items([]).
identity_items([indexed(Index, Term)|Items]) :-
    ( has_functor(Term, clause, 3) ->
        arg(1, Term, Id),
        arg(3, Term, BodyTerm),
        arg(1, BodyTerm, Body),
        id_parts(Id, ActualKind, Sentence, Clause),
        expected_clause_kind(Body, ExpectedKind),
        ( ActualKind == ExpectedKind -> true
        ; reject(identity, term(Index, id_kind(ExpectedKind, ActualKind)))
        )
    ; arg(1, Term, Id),
      id_parts(Id, ActualKind, Sentence, Clause),
      ( ActualKind == query -> true
      ; reject(identity, term(Index, id_kind(query, ActualKind)))
      )
    ),
    require_positive(Index, id_sentence, Sentence),
    require_positive(Index, id_clause, Clause),
    identity_items(Items).

expected_clause_kind([], fact) :-
    !.
expected_clause_kind(_, rule).

id_parts(Id, Kind, Sentence, Clause) :-
    functor(Id, IdName, 2),
    id_kind_name(IdName, Kind),
    arg(1, Id, SentenceTerm),
    arg(2, Id, ClauseTerm),
    arg(1, SentenceTerm, Sentence),
    arg(1, ClauseTerm, Clause).

id_kind_name(fact_id, fact).
id_kind_name(rule_id, rule).
id_kind_name(query_id, query).

require_positive(_, _, Value) :-
    Value >= 1,
    !.
require_positive(Index, Field, Value) :-
    reject(identity, term(Index, ordinal(Field, Value))).

/* Pass 7: global ID uniqueness and strict order within clause sections. */
ordering_pass(Items) :-
    ordering_items(Items, [], state(none, none)).

ordering_items([], _, _).
ordering_items([indexed(Index, Term)|Items], Seen, State0) :-
    item_id(Term, Id),
    id_parts(Id, Section, Sentence, Clause),
    Pair = pair(Sentence, Clause),
    ( member_eq(Pair, Seen) ->
        reject(ordering, term(Index, duplicate_id(Pair)))
    ; true
    ),
    check_section_ordering(Section, Pair, State0, State, Index),
    ordering_items(Items, [Pair|Seen], State).

item_id(Term, Id) :-
    arg(1, Term, Id).

check_section_ordering(fact, Pair, State0, State, Index) :-
    !,
    State0 = state(Last, Rule),
    check_pair_after(Last, Pair, Index),
    State = state(Pair, Rule).
check_section_ordering(rule, Pair, State0, State, Index) :-
    !,
    State0 = state(Fact, Last),
    check_pair_after(Last, Pair, Index),
    State = state(Fact, Pair).
check_section_ordering(query, _, State, State, _).

check_pair_after(none, _, _) :-
    !.
check_pair_after(Last, Pair, Index) :-
    ( pair_less(Last, Pair) -> true
    ; reject(ordering, term(Index, section_id_after(Last)))
    ).

pair_less(pair(Sentence0, Clause0), pair(Sentence, Clause)) :-
    ( Sentence0 < Sentence
    ; Sentence0 =:= Sentence,
      Clause0 < Clause
    ).

/* Pass 8: variables occur only in rules and are densely numbered there. */
scope_pass(Items) :-
    scope_items(Items).

scope_items([]).
scope_items([indexed(Index, Term)|Items]) :-
    ( has_functor(Term, clause, 3) ->
        arg(1, Term, Id),
        id_parts(Id, Kind, _, _),
        arg(2, Term, Head),
        arg(3, Term, BodyTerm),
        arg(1, BodyTerm, Body),
        check_clause_scope(Kind, Index, Head, Body)
    ; arg(2, Term, Predicate),
      reject_if_predicate_variable(Index, Predicate)
    ),
    scope_items(Items).

check_clause_scope(fact, Index, Head, _) :-
    reject_if_predicate_variable(Index, Head).
check_clause_scope(rule, Index, Head, Body) :-
    predicate_vars(Head, HeadVars),
    body_vars(Body, BodyVars),
    append(HeadVars, BodyVars, Vars),
    dense_first_occurrence(Index, Vars, [], 1, 1).

reject_if_predicate_variable(Index, Predicate) :-
    predicate_vars(Predicate, Vars),
    ( Vars = [Number|_] ->
        reject(scope, term(Index, var_outside_rule(Number)))
    ; true
    ).

predicate_vars(Predicate, Vars) :-
    arg(2, Predicate, Args),
    argument_vars(Args, Vars, []).

argument_vars([], Vars, Vars).
argument_vars([Arg|Args], Vars0, Vars) :-
    ( has_functor(Arg, var, 1) ->
        arg(1, Arg, Number),
        Vars0 = [Number|Rest]
    ; Rest = Vars0
    ),
    argument_vars(Args, Rest, Vars).

body_vars([], []).
body_vars([Literal|Literals], Vars) :-
    literal_predicate(Literal, Predicate),
    predicate_vars(Predicate, Here),
    body_vars(Literals, Rest),
    append(Here, Rest, Vars).

literal_predicate(Literal, Predicate) :-
    ( has_functor(Literal, naf, 1) ->
        arg(1, Literal, Predicate)
    ; Predicate = Literal
    ).

dense_first_occurrence(_, [], _, _, _).
dense_first_occurrence(Index, [Number|Numbers], Seen, Next, Position) :-
    ( member_number(Number, Seen) ->
        Seen1 = Seen,
        Next1 = Next
    ; Number =:= Next ->
        Seen1 = [Number|Seen],
        Next1 is Next + 1
    ; reject(scope,
          term(Index,
              variable_sequence(expected(Next), found(Number),
                  occurrence(Position))))
    ),
    Position1 is Position + 1,
    dense_first_occurrence(Index, Numbers, Seen1, Next1, Position1).

/* Pass 9: NAF is reserved/rejected and rule heads are body-covered. */
safety_naf_pass(Items) :-
    safety_naf_items(Items).

safety_naf_items([]).
safety_naf_items([indexed(Index, Term)|Items]) :-
    ( has_functor(Term, clause, 3) ->
        arg(1, Term, Id),
        id_parts(Id, Kind, _, _),
        ( Kind == rule ->
            arg(2, Term, Head),
            arg(3, Term, BodyTerm),
            arg(1, BodyTerm, Body),
            validate_rule_safety(Index, Head, Body)
        ; true
        )
    ; true
    ),
    safety_naf_items(Items).

validate_rule_safety(Index, _Head, Body) :-
    first_naf(Body, 1, Position),
    !,
    reject(naf, term(Index, body_literal(Position))).
validate_rule_safety(Index, Head, Body) :-
    predicate_vars(Head, HeadVars),
    positive_body_vars(Body, BodyVars),
    ( first_missing_var(HeadVars, BodyVars, Missing) ->
        reject(safety, term(Index, head_var_not_in_body(Missing)))
    ; true
    ).

first_naf([Literal|_], Position, Position) :-
    has_functor(Literal, naf, 1),
    !.
first_naf([_|Literals], Position0, Position) :-
    Position1 is Position0 + 1,
    first_naf(Literals, Position1, Position).

positive_body_vars([], []).
positive_body_vars([Predicate|Literals], Vars) :-
    predicate_vars(Predicate, Here),
    positive_body_vars(Literals, Rest),
    append(Here, Rest, Vars).

first_missing_var([Number|_], BodyVars, Number) :-
    \+ member_number(Number, BodyVars),
    !.
first_missing_var([_|Numbers], BodyVars, Missing) :-
    first_missing_var(Numbers, BodyVars, Missing).

/* Pass 10: reject the first positive dependency edge closing a cycle. */
cycle_pass(Items) :-
    cycle_items(Items, []).

cycle_items([], _).
cycle_items([indexed(Index, Term)|Items], Edges0) :-
    ( has_functor(Term, clause, 3) ->
        arg(1, Term, Id),
        id_parts(Id, Kind, _, _),
        ( Kind == rule ->
            arg(2, Term, Head),
            arg(3, Term, BodyTerm),
            arg(1, BodyTerm, Body),
            predicate_key(Head, HeadKey),
            add_body_edges(Body, Index, 1, HeadKey, Edges0, Edges)
        ; Edges = Edges0
        )
    ; Edges = Edges0
    ),
    cycle_items(Items, Edges).

add_body_edges([], _, _, _, Edges, Edges).
add_body_edges([Predicate|Predicates], Index, Position, HeadKey,
        Edges0, Edges) :-
    predicate_key(Predicate, BodyKey),
    ( creates_cycle(HeadKey, BodyKey, Edges0) ->
        reject(cycle,
            term(Index,
                body_literal(Position, dependency(HeadKey, BodyKey))))
    ; Edges1 = [edge(HeadKey, BodyKey)|Edges0]
    ),
    Position1 is Position + 1,
    add_body_edges(Predicates, Index, Position1, HeadKey, Edges1, Edges).

predicate_key(Predicate, pred(Name, Arity)) :-
    arg(1, Predicate, Name),
    arg(2, Predicate, Args),
    length(Args, Arity).

creates_cycle(From, To, _) :-
    From == To,
    !.
creates_cycle(From, To, Edges) :-
    reachable(To, From, Edges, []).

reachable(Node, Target, _, _) :-
    Node == Target,
    !.
reachable(Node, Target, Edges, Visited) :-
    \+ member_eq(Node, Visited),
    edge_from(Node, Edges, Next),
    reachable(Next, Target, Edges, [Node|Visited]).

edge_from(Node, [edge(From, To)|_], To) :-
    From == Node.
edge_from(Node, [_|Edges], Next) :-
    edge_from(Node, Edges, Next).

/* Preserve stream order while assigning private database sequence numbers. */
collect_program([indexed(_, Term)|Items], Seq0, Clauses, Goal) :-
    ( has_functor(Term, clause, 3) ->
        arg(1, Term, Id),
        arg(2, Term, Head),
        arg(3, Term, BodyTerm),
        arg(1, BodyTerm, Body),
        Clauses = [validated_clause(Seq0, Id, Head, Body)|Rest],
        Seq is Seq0 + 1,
        collect_program(Items, Seq, Rest, Goal)
    ; arg(1, Term, GoalId),
      arg(2, Term, GoalAtom),
      Clauses = [],
      Goal = goal(GoalId, GoalAtom)
    ).

install_program(Clauses) :-
    retractall(cnl_program_db:program_clause(_, _, _, _)),
    assert_program_clauses(Clauses).

assert_program_clauses([]).
assert_program_clauses([validated_clause(Seq, Id, Head, Body)|Clauses]) :-
    assertz(cnl_program_db:program_clause(Seq, Id, Head, Body)),
    assert_program_clauses(Clauses).

teardown_program :-
    retractall(cnl_program_db:program_clause(_, _, _, _)).

evaluate_program(Document, Goal, ClauseCount, ResultTerms) :-
    least_model(ClauseCount, Store),
    arg(1, Goal, GoalId),
    arg(2, Goal, GoalAtom),
    ( atom_present(GoalAtom, Store) ->
        Outcome = proved
    ; Outcome = not_proved
    ),
    assemble_result_terms(
        Document, GoalId, GoalAtom, Outcome, Store, ResultTerms).

/*
Repeated-pass schedule. Each clause sees a snapshot taken at clause entry;
new atoms become visible only to later clauses in the same pass. Body solutions
use leftmost-outermost DFS over snapshot insertion order and are deduplicated
into the growing store as they are enumerated.
*/
least_model(ClauseCount, Store) :-
    fixpoint(ClauseCount, [], Store).

fixpoint(ClauseCount, Store0, Store) :-
    program_pass(1, ClauseCount, Store0, Store1, false, Added),
    ( Added == true ->
        fixpoint(ClauseCount, Store1, Store)
    ; Store = Store1
    ).

program_pass(Seq, ClauseCount, Store, Store, Added, Added) :-
    Seq > ClauseCount,
    !.
program_pass(Seq, ClauseCount, Store0, Store, Added0, Added) :-
    once(cnl_program_db:program_clause(Seq, Id, Head, Body)),
    Snapshot = Store0,
    add_clause_solutions(Head, Body, Snapshot, Id,
        Store0, Store1, Added0, Added1),
    Next is Seq + 1,
    program_pass(Next, ClauseCount, Store1, Store, Added1, Added).

/*
forall preserves schedule enumeration while discarding solution bindings;
nb_setarg mutates only this holder and copies each new list, leaving Snapshot
untouched while retaining only the growing deduplicated store.
*/
add_clause_solutions(Head, Body, Snapshot, Id,
        Store0, Store, Added0, Added) :-
    State = candidate_state(Store0, Added0),
    forall(
        ( body_solution(Body, Snapshot, [], Bindings, GroundBody),
          substitute_predicate(Head, Bindings, GroundHead)
        ),
        insert_clause_solution(GroundHead, GroundBody, Id, State)),
    arg(1, State, Store),
    arg(2, State, Added).

insert_clause_solution(Atom, BodyAtoms, Id, State) :-
    arg(1, State, Store0),
    ( atom_present(Atom, Store0) ->
        true
    ; append(Store0, [entry(Atom, by(Id, BodyAtoms))], Store),
      nb_setarg(1, State, Store),
      nb_setarg(2, State, true)
    ).

body_solution(Body, _, Bindings, Bindings, []) :-
    Body == [],
    !.
body_solution(Body, Snapshot, Bindings0, Bindings, [Ground|Grounds]) :-
    arg(1, Body, Pattern),
    arg(2, Body, Rest),
    store_atom(Snapshot, Ground),
    match_predicate(Pattern, Ground, Bindings0, Bindings1),
    body_solution(Rest, Snapshot, Bindings1, Bindings, Grounds).

store_atom([entry(Atom, _)|_], Atom).
store_atom([_|Entries], Atom) :-
    store_atom(Entries, Atom).

match_predicate(Pattern, Ground, Bindings0, Bindings) :-
    arg(1, Pattern, PatternName),
    arg(2, Pattern, PatternArgs),
    arg(1, Ground, GroundName),
    arg(2, Ground, GroundArgs),
    PatternName == GroundName,
    match_arguments(PatternArgs, GroundArgs, Bindings0, Bindings).

match_arguments(PatternArgs, GroundArgs, Bindings, Bindings) :-
    PatternArgs == [],
    GroundArgs == [],
    !.
match_arguments(PatternArgs, GroundArgs, Bindings0, Bindings) :-
    PatternArgs \== [],
    GroundArgs \== [],
    arg(1, PatternArgs, PatternArg),
    arg(2, PatternArgs, PatternRest),
    arg(1, GroundArgs, GroundArg),
    arg(2, GroundArgs, GroundRest),
    match_argument(PatternArg, GroundArg, Bindings0, Bindings1),
    match_arguments(PatternRest, GroundRest, Bindings1, Bindings).

match_argument(Pattern, Ground, Bindings, Bindings) :-
    has_functor(Pattern, named, 1),
    Pattern == Ground,
    !.
match_argument(Pattern, Ground, Bindings0, Bindings) :-
    has_functor(Pattern, var, 1),
    named_ground(Ground),
    arg(1, Pattern, Number),
    ( binding_value(Number, Bindings0, Bound) ->
        Bound == Ground,
        Bindings = Bindings0
    ; Bindings = [binding(Number, Ground)|Bindings0]
    ).

named_ground(Ground) :-
    has_functor(Ground, named, 1),
    arg(1, Ground, Name),
    atom(Name).

binding_value(Number, [binding(Here, Value)|_], Value) :-
    Here =:= Number,
    !.
binding_value(Number, [_|Bindings], Value) :-
    binding_value(Number, Bindings, Value).

substitute_predicate(Pattern, Bindings, pred(Name, GroundArgs)) :-
    arg(1, Pattern, Name),
    arg(2, Pattern, Args),
    substitute_arguments(Args, Bindings, GroundArgs).

substitute_arguments(Args, _, []) :-
    Args == [],
    !.
substitute_arguments(Args, Bindings, [Ground|Grounds]) :-
    arg(1, Args, Arg),
    arg(2, Args, Rest),
    substitute_argument(Arg, Bindings, Ground),
    substitute_arguments(Rest, Bindings, Grounds).

substitute_argument(Arg, _, Arg) :-
    has_functor(Arg, named, 1),
    !.
substitute_argument(Arg, Bindings, Ground) :-
    arg(1, Arg, Number),
    binding_value(Number, Bindings, Ground).

atom_present(Atom, [entry(Stored, _)|_]) :-
    Stored == Atom,
    !.
atom_present(Atom, [_|Entries]) :-
    atom_present(Atom, Entries).

member_eq(Term, [Member|_]) :-
    Member == Term,
    !.
member_eq(Term, [_|Members]) :-
    member_eq(Term, Members).

member_number(Number, [Member|_]) :-
    Member =:= Number,
    !.
member_number(Number, [_|Members]) :-
    member_number(Number, Members).

has_functor(Term, Name, Arity) :-
    compound(Term),
    functor(Term, Name, Arity).

reject(Class, Detail) :-
    throw(ir_reject(Class, Detail)).

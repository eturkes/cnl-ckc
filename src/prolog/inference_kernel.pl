:- module(inference_kernel, [validate_program_terms/4, run_terms/3]).

:- set_prolog_flag(encoding, utf8).

:- use_module(library(lists), [append/3]).
:- use_module(explanation, [assemble_result_terms/7]).

:- dynamic cnl_program_db:program_clause/4.
:- dynamic cnl_program_db:program_stratum/2.

/*
Program-record validator and deterministic stratified-Datalog kernel. Input
terms have already passed strict UTF-8, syntax, and canonical-byte framing
gates. Validation rejects with ir_reject(Class, Detail). Evaluation installs
only data facts in private module cnl_program_db and tears them down around
every run.
*/
validate_program_terms(Terms, Document, Clauses, Goal) :-
    envelope_pass(Terms, Document, Items),
    shape_pass(Document, Items),
    identity_pass(Document, Items),
    ordering_pass(Items),
    scope_pass(Items),
    safety_naf_pass(Items),
    cycle_pass(Items, Edges),
    collect_program(Items, 1, RawClauses, Goal),
    assign_clause_strata(RawClauses, Edges, Clauses).

run_terms(Terms, ProgramDigest, ResultTerms) :-
    validate_program_terms(Terms, Document, Clauses, Goal),
    arg(1, Goal, GoalKind),
    arg(2, Goal, GoalId),
    ( GoalKind == wh(who) ->
        reject(wh_query, goal(GoalId))
    ; valid_sha256(ProgramDigest) ->
        length(Clauses, ClauseCount),
        max_clause_stratum(Clauses, MaxStratum),
        setup_call_cleanup(
            install_program(Clauses),
            evaluate_program(Document, ProgramDigest, Goal, ClauseCount,
                MaxStratum, ResultTerms),
            teardown_program)
    ; kernel_invariant(program_digest(ProgramDigest))
    ).

/* Pass 4: exact envelope, one final goal, and facts before rules. */
envelope_pass([], _, _) :-
    reject(envelope, term(1, missing_header)).
envelope_pass([Header|Rest], Document, Items) :-
    ( Header == cnl_program_record(2) ->
        true
    ; reject(envelope, term(1, expected(cnl_program_record(2))))
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
    ( goal_term(Term) ->
        Positions = [Index|Rest]
    ; Positions = Rest
    ),
    goal_positions(Items, Rest).

goal_term(Term) :-
    has_functor(Term, goal, 2),
    !.
goal_term(Term) :-
    has_functor(Term, goal, 3).

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
    ( goal_term(Term) ->
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
    ; goal_term(Term) ->
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
    has_functor(Term, goal, 2),
    arg(1, Term, Id),
    arg(2, Term, Predicate),
    shape_id(Id),
    shape_predicate(Predicate),
    !.
shape_goal(Term) :-
    has_functor(Term, goal, 3),
    arg(1, Term, Id),
    arg(2, Term, Marker),
    arg(3, Term, Predicate),
    shape_id(Id),
    Marker == wh(who),
    shape_wh_predicate(Predicate).

shape_wh_predicate(Predicate) :-
    has_functor(Predicate, pred, 2),
    arg(1, Predicate, Name),
    arg(2, Predicate, Args),
    atom(Name),
    Args == [var(1)].

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
    ; has_functor(Term, goal, 2) ->
        arg(2, Term, Predicate),
        reject_if_predicate_variable(Index, Predicate)
    ; true
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

/*
Pass 9: within each rule, reject in this exact order: a positive literal after
NAF, an empty body, an NAF variable not covered by a positive literal, then a
head variable not covered by a positive literal.
*/
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
    first_positive_after_naf(Body, 1, false, Position),
    !,
    reject(safety, term(Index, positive_after_naf(Position))).
validate_rule_safety(Index, _Head, Body) :-
    Body == [],
    !,
    reject(safety, term(Index, empty_body)).
validate_rule_safety(Index, Head, Body) :-
    positive_body_vars(Body, PositiveVars),
    naf_body_vars(Body, NafVars),
    ( first_missing_var(NafVars, PositiveVars, MissingNaf) ->
        reject(safety,
            term(Index, naf_var_not_in_positive_body(MissingNaf)))
    ; predicate_vars(Head, HeadVars),
      ( first_missing_var(HeadVars, PositiveVars, MissingHead) ->
          reject(safety,
              term(Index, head_var_not_in_positive_body(MissingHead)))
      ; true
      )
    ).

first_positive_after_naf([], _, _, _) :-
    fail.
first_positive_after_naf([Literal|Literals], Position0, SeenNaf, Position) :-
    ( has_functor(Literal, naf, 1) ->
        SeenNaf1 = true,
        Position1 is Position0 + 1,
        first_positive_after_naf(
            Literals, Position1, SeenNaf1, Position)
    ; SeenNaf == true ->
        Position = Position0
    ; Position1 is Position0 + 1,
      first_positive_after_naf(
          Literals, Position1, SeenNaf, Position)
    ).

positive_body_vars([], []).
positive_body_vars([Literal|Literals], Vars) :-
    ( has_functor(Literal, naf, 1) ->
        Here = []
    ; predicate_vars(Literal, Here)
    ),
    positive_body_vars(Literals, Rest),
    append(Here, Rest, Vars).

naf_body_vars([], []).
naf_body_vars([Literal|Literals], Vars) :-
    ( has_functor(Literal, naf, 1) ->
        arg(1, Literal, Predicate),
        predicate_vars(Predicate, Here)
    ; Here = []
    ),
    naf_body_vars(Literals, Rest),
    append(Here, Rest, Vars).

first_missing_var([Number|_], CoveredVars, Number) :-
    \+ member_number(Number, CoveredVars),
    !.
first_missing_var([_|Numbers], CoveredVars, Missing) :-
    first_missing_var(Numbers, CoveredVars, Missing).

/* Pass 10: reject the first signed dependency edge closing any cycle. */
cycle_pass(Items, Edges) :-
    cycle_items(Items, [], Edges).

cycle_items([], Edges, Edges).
cycle_items([indexed(Index, Term)|Items], Edges0, Edges) :-
    ( has_functor(Term, clause, 3) ->
        arg(1, Term, Id),
        id_parts(Id, Kind, _, _),
        ( Kind == rule ->
            arg(2, Term, Head),
            arg(3, Term, BodyTerm),
            arg(1, BodyTerm, Body),
            predicate_key(Head, HeadKey),
            add_body_edges(Body, Index, 1, HeadKey, Edges0, Edges1)
        ; Edges1 = Edges0
        )
    ; Edges1 = Edges0
    ),
    cycle_items(Items, Edges1, Edges).

add_body_edges([], _, _, _, Edges, Edges).
add_body_edges([Literal|Literals], Index, Position, HeadKey,
        Edges0, Edges) :-
    dependency_literal(Literal, Polarity, Predicate),
    predicate_key(Predicate, BodyKey),
    ( creates_cycle(HeadKey, BodyKey, Edges0) ->
        reject(cycle,
            term(Index,
                body_literal(Position,
                    signed_dependency(Polarity, HeadKey, BodyKey))))
    ; Edges1 = [edge(Polarity, HeadKey, BodyKey)|Edges0]
    ),
    Position1 is Position + 1,
    add_body_edges(Literals, Index, Position1, HeadKey, Edges1, Edges).

dependency_literal(Literal, naf, Predicate) :-
    has_functor(Literal, naf, 1),
    arg(1, Literal, Predicate),
    !.
dependency_literal(Predicate, positive, Predicate).

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

edge_from(Node, [edge(_, From, To)|_], To) :-
    From == Node.
edge_from(Node, [_|Edges], Next) :-
    edge_from(Node, Edges, Next).

/*
A predicate's stratum is the maximum required by every clause defining that
predicate: positive dependencies retain their target stratum, NAF dependencies
add one, and every predicate has minimum stratum 1. The signed graph is already
known finite and cycle-free here.
*/
assign_clause_strata(RawClauses, Edges, Clauses) :-
    clause_head_keys(RawClauses, HeadKeys),
    compute_predicate_strata(HeadKeys, Edges, [], Strata),
    attach_clause_strata(RawClauses, Strata, Clauses).

clause_head_keys([], []).
clause_head_keys([validated_clause(_, _, Head, _)|Clauses],
        [Key|Keys]) :-
    predicate_key(Head, Key),
    clause_head_keys(Clauses, Keys).

compute_predicate_strata([], _, Strata, Strata).
compute_predicate_strata([Key|Keys], Edges, Strata0, Strata) :-
    predicate_stratum(Key, Edges, Strata0, Strata1, _),
    compute_predicate_strata(Keys, Edges, Strata1, Strata).

predicate_stratum(Key, _, Strata, Strata, Value) :-
    stratum_value(Key, Strata, Value),
    !.
predicate_stratum(Key, Edges, Strata0, Strata, Value) :-
    outgoing_dependencies(Key, Edges, Dependencies),
    dependency_max_stratum(
        Dependencies, Edges, Strata0, Strata1, 1, Value),
    Strata = [stratum(Key, Value)|Strata1].

outgoing_dependencies(_, [], []).
outgoing_dependencies(Key, [edge(Polarity, From, To)|Edges], Dependencies) :-
    ( From == Key ->
        Dependencies = [dependency(Polarity, To)|Rest]
    ; Dependencies = Rest
    ),
    outgoing_dependencies(Key, Edges, Rest).

dependency_max_stratum([], _, Strata, Strata, Value, Value).
dependency_max_stratum([dependency(Polarity, Key)|Dependencies], Edges,
        Strata0, Strata, Value0, Value) :-
    predicate_stratum(Key, Edges, Strata0, Strata1, DependencyStratum),
    required_stratum(Polarity, DependencyStratum, Required),
    ( Required > Value0 -> Value1 = Required ; Value1 = Value0 ),
    dependency_max_stratum(
        Dependencies, Edges, Strata1, Strata, Value1, Value).

required_stratum(positive, DependencyStratum, DependencyStratum).
required_stratum(naf, DependencyStratum, Required) :-
    Required is DependencyStratum + 1.

stratum_value(Key, [stratum(StoredKey, Value)|_], Value) :-
    StoredKey == Key,
    !.
stratum_value(Key, [_|Strata], Value) :-
    stratum_value(Key, Strata, Value).

attach_clause_strata([], _, []).
attach_clause_strata(
        [validated_clause(Seq, Id, Head, Body)|RawClauses], Strata,
        [validated_clause(Seq, Stratum, Id, Head, Body)|Clauses]) :-
    predicate_key(Head, Key),
    stratum_value(Key, Strata, Stratum),
    attach_clause_strata(RawClauses, Strata, Clauses).

max_clause_stratum([], 0).
max_clause_stratum([validated_clause(_, Stratum, _, _, _)|Clauses], Max) :-
    max_clause_stratum(Clauses, RestMax),
    ( Stratum > RestMax -> Max = Stratum ; Max = RestMax ).

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
    ; has_functor(Term, goal, 2) ->
        arg(1, Term, GoalId),
        arg(2, Term, GoalAtom),
        Clauses = [],
        Goal = goal(yes_no, GoalId, GoalAtom)
    ; arg(1, Term, GoalId),
      arg(2, Term, GoalKind),
      arg(3, Term, GoalAtom),
      Clauses = [],
      Goal = goal(GoalKind, GoalId, GoalAtom)
    ).

install_program(Clauses) :-
    retractall(cnl_program_db:program_clause(_, _, _, _)),
    retractall(cnl_program_db:program_stratum(_, _)),
    assert_program_clauses(Clauses).

assert_program_clauses([]).
assert_program_clauses(
        [validated_clause(Seq, Stratum, Id, Head, Body)|Clauses]) :-
    assertz(cnl_program_db:program_clause(Seq, Id, Head, Body)),
    assertz(cnl_program_db:program_stratum(Seq, Stratum)),
    assert_program_clauses(Clauses).

teardown_program :-
    retractall(cnl_program_db:program_clause(_, _, _, _)),
    retractall(cnl_program_db:program_stratum(_, _)).

evaluate_program(Document, ProgramDigest, Goal, ClauseCount, MaxStratum,
        ResultTerms) :-
    stratified_model(MaxStratum, ClauseCount, Store),
    arg(2, Goal, GoalId),
    arg(3, Goal, GoalAtom),
    ( atom_present(GoalAtom, Store) ->
        Outcome = proved
    ; Outcome = not_proved
    ),
    assemble_result_terms(
        Document, ProgramDigest, GoalId, GoalAtom, Outcome, Store,
        ResultTerms).

/*
Strata run in ascending order over one insertion-ordered store. Within each
stratum, the v1 repeated-pass schedule is unchanged: each participating clause
sees a snapshot taken at clause entry, new atoms become visible only to later
clauses in the same pass, and passes repeat to a fixpoint.
*/
stratified_model(MaxStratum, ClauseCount, Store) :-
    evaluate_strata(1, MaxStratum, ClauseCount, [], Store).

evaluate_strata(Stratum, MaxStratum, _, Store, Store) :-
    Stratum > MaxStratum,
    !.
evaluate_strata(Stratum, MaxStratum, ClauseCount, Store0, Store) :-
    stratum_fixpoint(Stratum, ClauseCount, Store0, Store1),
    Next is Stratum + 1,
    evaluate_strata(Next, MaxStratum, ClauseCount, Store1, Store).

stratum_fixpoint(Stratum, ClauseCount, Store0, Store) :-
    stratum_program_pass(
        1, ClauseCount, Stratum, Store0, Store1, false, Added),
    ( Added == true ->
        stratum_fixpoint(Stratum, ClauseCount, Store1, Store)
    ; Store = Store1
    ).

stratum_program_pass(Seq, ClauseCount, _, Store, Store, Added, Added) :-
    Seq > ClauseCount,
    !.
stratum_program_pass(Seq, ClauseCount, Stratum, Store0, Store,
        Added0, Added) :-
    once(cnl_program_db:program_clause(Seq, Id, Head, Body)),
    once(cnl_program_db:program_stratum(Seq, ClauseStratum)),
    ( ClauseStratum =:= Stratum ->
        Snapshot = Store0,
        add_clause_solutions(Head, Body, Snapshot, Id,
            Store0, Store1, Added0, Added1)
    ; Store1 = Store0,
      Added1 = Added0
    ),
    Next is Seq + 1,
    stratum_program_pass(
        Next, ClauseCount, Stratum, Store1, Store, Added1, Added).

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
body_solution(Body, Snapshot, Bindings0, Bindings,
        [naf(GroundAtom)|Grounds]) :-
    arg(1, Body, Literal),
    has_functor(Literal, naf, 1),
    !,
    arg(1, Literal, Pattern),
    ( substitute_predicate(Pattern, Bindings0, GroundAtom),
      kernel_ground_predicate(GroundAtom) ->
        true
    ; kernel_invariant(naf_not_ground(Pattern))
    ),
    \+ atom_present(GroundAtom, Snapshot),
    arg(2, Body, Rest),
    body_solution(Rest, Snapshot, Bindings0, Bindings, Grounds).
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

kernel_ground_predicate(Predicate) :-
    has_functor(Predicate, pred, 2),
    arg(1, Predicate, Name),
    arg(2, Predicate, Args),
    atom(Name),
    Args = [_|_],
    kernel_ground_arguments(Args),
    ground(Predicate).

kernel_ground_arguments([]).
kernel_ground_arguments([Arg|Args]) :-
    named_ground(Arg),
    kernel_ground_arguments(Args).

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

kernel_invariant(Detail) :-
    throw(inference_invariant(Detail)).

reject(Class, Detail) :-
    throw(ir_reject(Class, Detail)).

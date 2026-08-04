:- module(inference_kernel, [validate_program_terms/4, run_terms/3]).

:- set_prolog_flag(encoding, utf8).

:- use_module(library(lists), [append/3]).
:- use_module(drs_canon, [canonical_line/2]).
:- use_module(explanation, [assemble_result_terms/8]).
:- use_module(validation_common, [
    branch_shape_detail/2,
    shape_optional_branch/2,
    terms_pairwise_distinct/1,
    shape_argument/1,
    shape_ground_argument/1,
    shape_predicate_term/1,
    shape_data_atom/1,
    shape_quantity_compare/1,
    data_atom_vars/2,
    quantity_compare_vars/2,
    ground_argument/1,
    ground_data_atom/1,
    quantity_compare_outcome/3,
    temporal_window_error/2,
    data_atom_key/2
]).

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
    constructor_semantics_pass(Items),
    identity_pass(Document, Items),
    ordering_pass(Items),
    scope_pass(Items),
    safety_naf_pass(Items),
    exception_pass(Items),
    cycle_pass(Items, Edges),
    collect_program(Items, 1, RawClauses, Goal),
    assign_clause_strata(RawClauses, Edges, Clauses).

run_terms(Terms, ProgramDigest, ResultTerms) :-
    validate_program_terms(Terms, Document, Clauses, Goal),
    collect_program_alternatives(Terms, AlternativeSets),
    ( valid_sha256(ProgramDigest) ->
        length(Clauses, ClauseCount),
        max_clause_stratum(Clauses, MaxStratum),
        setup_call_cleanup(
            install_program(Clauses),
            evaluate_program(Document, ProgramDigest, Goal, ClauseCount,
                MaxStratum, AlternativeSets, ResultTerms),
            teardown_program)
    ; kernel_invariant(program_digest(ProgramDigest))
    ).

collect_program_alternatives([], []).
collect_program_alternatives([Term|Terms], AlternativeSets) :-
    ( has_functor(Term, alternative_set, 6) ->
        AlternativeSets = [Term|Rest]
    ; AlternativeSets = Rest
    ),
    collect_program_alternatives(Terms, Rest).

/* Pass 4: exact envelope, one final goal, and facts before rules. */
envelope_pass([], _, _) :-
    reject(envelope, term(1, missing_header)).
envelope_pass([Header|Rest], Document, Items) :-
    ( Header == cnl_program_record(3) ->
        true
    ; reject(envelope, term(1, expected(cnl_program_record(3))))
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
    program_item_section(Term, Section),
    section_transition(State0, Section, Index, State),
    section_order(Items, State).

program_item_section(Term, fact) :-
    has_functor(Term, clause, 3),
    arg(1, Term, Id),
    has_functor(Id, fact_id, 2),
    !.
program_item_section(Term, closed_world) :-
    has_functor(Term, closed_world, 3),
    !.
program_item_section(Term, rule) :-
    has_functor(Term, clause, 3),
    arg(1, Term, Id),
    compound(Id),
    functor(Id, rule_id, _),
    !.
program_item_section(Term, alternative_set) :-
    has_functor(Term, alternative_set, 6),
    !.
program_item_section(_, unknown).

section_transition(facts, fact, _, facts).
section_transition(facts, closed_world, _, closed_world).
section_transition(facts, rule, _, rules).
section_transition(facts, alternative_set, _, alternatives).
section_transition(closed_world, closed_world, _, closed_world).
section_transition(closed_world, rule, _, rules).
section_transition(closed_world, alternative_set, _, alternatives).
section_transition(rules, rule, _, rules).
section_transition(rules, alternative_set, _, alternatives).
section_transition(alternatives, alternative_set, _, alternatives).
section_transition(State, unknown, _, State).
section_transition(State, fact, Index, _) :-
    State \== facts,
    reject(section_order, term(Index, fact_after_rule)).
section_transition(rules, closed_world, Index, _) :-
    reject(section_order, term(Index, closed_world_after_rule)).
section_transition(alternatives, closed_world, Index, _) :-
    reject(section_order, term(Index, closed_world_after_alternative_set)).
section_transition(alternatives, rule, Index, _) :-
    reject(section_order, term(Index, rule_after_alternative_set)).

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
    ( branch_shape_detail(Term, Detail) ->
        reject(shape, term(Index, Detail))
    ; has_functor(Term, clause, 3) ->
        ( shape_clause(Term) -> true
        ; reject(shape, term(Index, clause))
        )
    ; has_functor(Term, closed_world, 3) ->
        ( shape_closed_world(Term) -> true
        ; reject(shape, term(Index, closed_world))
        )
    ; has_functor(Term, alternative_set, 6) ->
        ( shape_alternative_set(Term) -> true
        ; reject(shape, term(Index, alternative_set))
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
    shape_body(Body),
    arg(1, Body, Literals),
    ( Literals == [] ->
        shape_data_atom(Head)
    ; shape_predicate(Head)
    ).

shape_closed_world(closed_world(ExceptionId, Affects, PredicateKey)) :-
    shape_exception_id(ExceptionId),
    has_functor(Affects, affects, 1),
    arg(1, Affects, RuleId),
    shape_rule_id(RuleId),
    shape_predicate_key(PredicateKey).

shape_exception_id(ExceptionId) :-
    has_functor(ExceptionId, exception_id, 2),
    arg(1, ExceptionId, Rule),
    arg(2, ExceptionId, Literal),
    has_functor(Rule, rule, 1),
    arg(1, Rule, RuleId),
    shape_rule_id(RuleId),
    shape_integer_wrapper(Literal, literal).

shape_predicate_key(PredicateKey) :-
    has_functor(PredicateKey, predicate_key, 2),
    arg(1, PredicateKey, Name),
    arg(2, PredicateKey, Arity),
    atom(Name),
    shape_integer_wrapper(Arity, arity).

shape_alternative_set(alternative_set(Id, MembersTerm, Body,
        Satisfaction, Exclusivity, Exhaustiveness)) :-
    shape_alternative_set_id(Id),
    has_functor(MembersTerm, members, 1),
    arg(1, MembersTerm, Members),
    is_list(Members),
    Members = [_,_|_],
    shape_predicates(Members),
    terms_pairwise_distinct(Members),
    shape_alternative_body(Body),
    Satisfaction == satisfaction(any_member),
    Exclusivity == exclusivity(not_asserted),
    Exhaustiveness == exhaustiveness(not_asserted).

shape_predicates([]).
shape_predicates([Predicate|Predicates]) :-
    shape_predicate(Predicate),
    shape_predicates(Predicates).

shape_alternative_set_id(Id) :-
    compound(Id),
    functor(Id, alternative_set_id, Arity),
    ( Arity =:= 2 ; Arity =:= 3 ),
    arg(1, Id, Sentence),
    arg(2, Id, Clause),
    shape_integer_wrapper(Sentence, sentence),
    shape_integer_wrapper(Clause, clause),
    shape_optional_branch(Id, Arity).

shape_rule_id(Id) :-
    compound(Id),
    functor(Id, rule_id, Arity),
    ( Arity =:= 2 ; Arity =:= 3 ),
    arg(1, Id, Sentence),
    arg(2, Id, Clause),
    shape_integer_wrapper(Sentence, sentence),
    shape_integer_wrapper(Clause, clause),
    shape_optional_branch(Id, Arity).

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
    functor(Id, Name, Arity),
    admitted_id_name(Name),
    ( Arity =:= 2 ; Arity =:= 3 ),
    arg(1, Id, Sentence),
    arg(2, Id, Clause),
    shape_integer_wrapper(Sentence, sentence),
    shape_integer_wrapper(Clause, clause),
    shape_optional_branch(Id, Arity).

admitted_id_name(fact_id).
admitted_id_name(rule_id).
admitted_id_name(query_id).

shape_integer_wrapper(Term, Name) :-
    has_functor(Term, Name, 1),
    arg(1, Term, Value),
    integer(Value).

shape_predicate(Predicate) :-
    shape_predicate_term(Predicate).

shape_args([]).
shape_args([Arg|Args]) :-
    shape_arg(Arg),
    shape_args(Args).

shape_arg(Arg) :-
    shape_argument(Arg).

shape_body(Body) :-
    has_functor(Body, body, 1),
    arg(1, Body, Literals),
    is_list(Literals),
    shape_literals(Literals).

shape_alternative_body(Body) :-
    has_functor(Body, body, 1),
    arg(1, Body, Literals),
    is_list(Literals),
    shape_alternative_literals(Literals).

shape_alternative_literals([]).
shape_alternative_literals([Literal|Literals]) :-
    ( shape_predicate(Literal)
    ; has_functor(Literal, naf, 1),
      arg(1, Literal, BarePredicate),
      shape_predicate(BarePredicate)
    ; has_functor(Literal, naf, 2),
      arg(1, Literal, ExceptionId),
      arg(2, Literal, LabeledPredicate),
      shape_exception_id(ExceptionId),
      shape_predicate(LabeledPredicate)
    ),
    shape_alternative_literals(Literals).

shape_literals([]).
shape_literals([Literal|Literals]) :-
    shape_literal(Literal),
    shape_literals(Literals).

shape_literal(Literal) :-
    shape_data_atom(Literal),
    !.
shape_literal(Literal) :-
    shape_quantity_compare(Literal),
    !.
shape_literal(Literal) :-
    has_functor(Literal, naf, 1),
    arg(1, Literal, Predicate),
    shape_predicate(Predicate),
    !.
shape_literal(Literal) :-
    has_functor(Literal, naf, 2),
    arg(1, Literal, ExceptionId),
    arg(2, Literal, Predicate),
    shape_exception_id(ExceptionId),
    shape_predicate(Predicate).

constructor_semantics_pass(Items) :-
    constructor_semantics_items(Items).

constructor_semantics_items([]).
constructor_semantics_items([indexed(Index, Term)|Items]) :-
    ( has_functor(Term, clause, 3) ->
        arg(2, Term, Head),
        arg(3, Term, body(Body)),
        ( Body == [] ->
            check_data_atom_semantics(Index, fact, 0, Head)
        ; check_body_constructor_semantics(Body, Index, 1)
        )
    ; true
    ),
    constructor_semantics_items(Items).

check_body_constructor_semantics([], _, _).
check_body_constructor_semantics([Literal|Literals], Index, Position) :-
    ( shape_quantity_compare(Literal) ->
        arg(1, Literal, Actual),
        arg(2, Literal, Bound),
        quantity_compare_outcome(Actual, Bound, Outcome),
        ( Outcome = cross_unit(ActualUnit, BoundUnit) ->
            reject(quantity,
                term(Index,
                    body_literal(Position,
                        cross_unit(ActualUnit, BoundUnit))))
        ; true
        )
    ; shape_data_atom(Literal) ->
        check_data_atom_semantics(Index, body, Position, Literal)
    ; true
    ),
    Next is Position + 1,
    check_body_constructor_semantics(Literals, Index, Next).

check_data_atom_semantics(Index, Context, Position, Atom) :-
    ( has_functor(Atom, temporal_window, 4),
      temporal_window_error(Atom, Error) ->
        temporal_error_detail(Context, Position, Error, Detail),
        reject(temporal, term(Index, Detail))
    ; true
    ).

temporal_error_detail(fact, _, Error, fact_temporal_window(Error)).
temporal_error_detail(body, Position, Error,
    body_literal(Position, temporal_window(Error))).

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
    ( has_functor(Term, closed_world, 3) ->
        true
    ; has_functor(Term, clause, 3) ->
        arg(1, Term, Id),
        arg(3, Term, BodyTerm),
        arg(1, BodyTerm, Body),
        id_parts(Id, ActualKind, Sentence, Clause),
        expected_clause_kind(Body, ExpectedKind),
        ( ActualKind == ExpectedKind -> true
        ; reject(identity, term(Index, id_kind(ExpectedKind, ActualKind)))
        ),
        require_id_branch(Index, ExpectedKind, Id),
        require_positive(Index, id_sentence, Sentence),
        require_positive(Index, id_clause, Clause)
    ; has_functor(Term, alternative_set, 6) ->
        arg(1, Term, Id),
        id_parts(Id, ActualKind, Sentence, Clause),
        ( ActualKind == alternative_set -> true
        ; reject(identity,
              term(Index, id_kind(alternative_set, ActualKind)))
        ),
        require_id_branch(Index, alternative_set, Id),
        require_positive(Index, id_sentence, Sentence),
        require_positive(Index, id_clause, Clause)
    ; arg(1, Term, Id),
      id_parts(Id, ActualKind, Sentence, Clause),
      ( ActualKind == query -> true
      ; reject(identity, term(Index, id_kind(query, ActualKind)))
      ),
      require_id_branch(Index, query, Id),
      require_positive(Index, id_sentence, Sentence),
      require_positive(Index, id_clause, Clause)
    ),
    identity_items(Items).

expected_clause_kind([], fact) :-
    !.
expected_clause_kind(_, rule).

id_parts(Id, Kind, Sentence, Clause) :-
    functor(Id, IdName, _),
    id_kind_name(IdName, Kind),
    arg(1, Id, SentenceTerm),
    arg(2, Id, ClauseTerm),
    arg(1, SentenceTerm, Sentence),
    arg(1, ClauseTerm, Clause).

id_kind_name(fact_id, fact).
id_kind_name(rule_id, rule).
id_kind_name(alternative_set_id, alternative_set).
id_kind_name(query_id, query).

require_id_branch(Index, Kind, Id) :-
    functor(Id, _, Arity),
    ( Arity =:= 2 ->
        true
    ; arg(3, Id, branch(Branch)),
      ( ( Kind == rule ; Kind == alternative_set ) ->
          require_positive(Index, id_branch, Branch)
      ; reject(identity, term(Index, branch_id_not_admitted(Kind)))
      )
    ).

require_positive(_, _, Value) :-
    Value >= 1,
    !.
require_positive(Index, Field, Value) :-
    reject(identity, term(Index, ordinal(Field, Value))).

/* Pass 7: canonical item order, origin ownership, and branch density. */
ordering_pass(Items) :-
    ordering_items(Items, [], [], state(none, none, none, none)),
    branch_group_pass(Items, none),
    branch_payload_pass(Items, none).

ordering_items([], _, _, _).
ordering_items([indexed(Index, Term)|Items], SeenIds, SeenOrigins, State0) :-
    ( has_functor(Term, closed_world, 3) ->
        SeenIds1 = SeenIds,
        SeenOrigins1 = SeenOrigins,
        State = State0
    ; program_item_id(Term, Section, Id),
      id_parts(Id, _, Sentence, Clause),
      id_branch_number(Id, Branch),
      Origin = pair(Sentence, Clause),
      ( member_eq(Id, SeenIds) ->
          reject(ordering, term(Index, duplicate_id(Id)))
      ; true
      ),
      check_origin_owner(
          Index, Section, Origin, Branch, SeenOrigins),
      state_last(Section, State0, Last),
      Key = key(Sentence, Clause, Branch),
      ( Last == none -> true
      ; order_key_less(Last, Key) -> true
      ; reject(ordering, term(Index, section_id_after(Last)))
      ),
      state_put(Section, Key, State0, State),
      SeenIds1 = [Id|SeenIds],
      SeenOrigins1 = [seen_origin(Section, Origin, Branch)|SeenOrigins]
    ),
    ordering_items(Items, SeenIds1, SeenOrigins1, State).

program_item_id(Term, Section, Id) :-
    ( has_functor(Term, clause, 3) ->
        arg(1, Term, Id),
        id_parts(Id, Section, _, _)
    ; has_functor(Term, alternative_set, 6) ->
        arg(1, Term, Id),
        Section = alternative_set
    ; arg(1, Term, Id),
      Section = query
    ).

id_branch_number(Id, Branch) :-
    functor(Id, _, Arity),
    ( Arity =:= 2 -> Branch = 0
    ; arg(3, Id, branch(Branch))
    ).

check_origin_owner(_, _, _, _, []) :-
    !.
check_origin_owner(Index, Section, Origin, Branch,
        [seen_origin(SeenSection, SeenOrigin, SeenBranch)|Seen]) :-
    ( SeenOrigin == Origin ->
        ( Section == SeenSection,
          Branch > 0,
          SeenBranch > 0 ->
            true
        ; reject(ordering, term(Index, duplicate_id(Origin)))
        )
    ; check_origin_owner(Index, Section, Origin, Branch, Seen)
    ).

state_last(fact, state(Fact, _, _, _), Fact).
state_last(rule, state(_, Rule, _, _), Rule).
state_last(alternative_set, state(_, _, Alternative, _), Alternative).
state_last(query, state(_, _, _, Query), Query).

state_put(fact, Key, state(_, Rule, Alternative, Query),
    state(Key, Rule, Alternative, Query)).
state_put(rule, Key, state(Fact, _, Alternative, Query),
    state(Fact, Key, Alternative, Query)).
state_put(alternative_set, Key, state(Fact, Rule, _, Query),
    state(Fact, Rule, Key, Query)).
state_put(query, Key, state(Fact, Rule, Alternative, _),
    state(Fact, Rule, Alternative, Key)).

order_key_less(key(S0, C0, B0), key(S, C, B)) :-
    ( S0 < S
    ; S0 =:= S, C0 < C
    ; S0 =:= S, C0 =:= C, B0 < B
    ).

branch_group_pass([], Group) :-
    finalize_branch_group(Group).
branch_group_pass([indexed(Index, Term)|Items], Group0) :-
    ( branch_item(Term, Section, Origin, Branch) ->
        advance_branch_group(
            Group0, Section, Origin, Branch, Index, Group)
    ; finalize_branch_group(Group0),
      Group = none
    ),
    branch_group_pass(Items, Group).

branch_item(Term, Section, pair(Sentence, Clause), Branch) :-
    program_item_id(Term, Section, Id),
    ( Section == rule ; Section == alternative_set ),
    functor(Id, _, 3),
    arg(1, Id, sentence(Sentence)),
    arg(2, Id, clause(Clause)),
    arg(3, Id, branch(Branch)).

advance_branch_group(none, Section, Origin, Branch, Index,
        group(Section, Origin, 2, 1, Index)) :-
    ( Branch =:= 1 -> true
    ; reject(ordering,
          term(Index, branch_sequence(expected(1), found(Branch))))
    ).
advance_branch_group(group(Section0, Origin0, Next0, Count0, Start),
        Section, Origin, Branch, Index, Group) :-
    ( Section == Section0,
      Origin == Origin0 ->
        ( Branch =:= Next0 ->
            Next is Next0 + 1,
            Count is Count0 + 1,
            Group = group(Section0, Origin0, Next, Count, Start)
        ; reject(ordering,
              term(Index,
                  branch_sequence(expected(Next0), found(Branch))))
        )
    ; finalize_branch_group(
          group(Section0, Origin0, Next0, Count0, Start)),
      advance_branch_group(none, Section, Origin, Branch, Index, Group)
    ).

finalize_branch_group(none).
finalize_branch_group(group(Section, Origin, _, Count, Index)) :-
    ( Count >= 2 -> true
    ; reject(ordering,
          term(Index, branch_group_singleton(Section, Origin)))
    ).

branch_payload_pass([], _).
branch_payload_pass([indexed(Index, Term)|Items], State0) :-
    ( branch_item(Term, Section, Origin, _) ->
        branch_item_payload(Section, Term, Payload),
        advance_branch_payload(
            State0, Section, Origin, Payload, Index, State)
    ; State = none
    ),
    branch_payload_pass(Items, State).

branch_item_payload(rule, Term, rule_head(Head)) :-
    arg(2, Term, Head).
branch_item_payload(alternative_set, Term,
        alternative_head(Members, Satisfaction, Exclusivity,
            Exhaustiveness)) :-
    arg(2, Term, Members),
    arg(4, Term, Satisfaction),
    arg(5, Term, Exclusivity),
    arg(6, Term, Exhaustiveness).

advance_branch_payload(none, Section, Origin, Payload, _,
        payload_group(Section, Origin, Payload)).
advance_branch_payload(
        payload_group(Section0, Origin0, Expected),
        Section, Origin, Payload, Index, State) :-
    ( Section == Section0,
      Origin == Origin0 ->
        ( Payload == Expected ->
            State = payload_group(Section0, Origin0, Expected)
        ; reject(ordering,
              term(Index,
                  branch_payload_mismatch(Section, Origin)))
        )
    ; State = payload_group(Section, Origin, Payload)
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
        arg(3, Term, body(Body)),
        check_clause_scope(Kind, Index, Head, Body)
    ; has_functor(Term, alternative_set, 6) ->
        arg(2, Term, members(Members)),
        arg(3, Term, body(Body)),
        predicate_list_vars(Members, MemberVars),
        body_vars(Body, BodyVars),
        append(MemberVars, BodyVars, Vars),
        dense_first_occurrence(Index, Vars, [], 1, 1)
    ; has_functor(Term, goal, 2) ->
        arg(2, Term, Predicate),
        reject_if_predicate_variable(Index, Predicate)
    ; true
    ),
    scope_items(Items).

check_clause_scope(fact, Index, Head, _) :-
    reject_if_data_atom_variable(Index, Head).
check_clause_scope(rule, Index, Head, Body) :-
    predicate_vars(Head, HeadVars),
    body_vars(Body, BodyVars),
    append(HeadVars, BodyVars, Vars),
    dense_first_occurrence(Index, Vars, [], 1, 1).

reject_if_data_atom_variable(Index, Atom) :-
    data_atom_vars(Atom, Vars),
    reject_if_variable_list(Index, Vars).

reject_if_predicate_variable(Index, Predicate) :-
    predicate_vars(Predicate, Vars),
    reject_if_variable_list(Index, Vars).

reject_if_variable_list(Index, Vars) :-
    ( Vars = [Number|_] ->
        reject(scope, term(Index, var_outside_rule(Number)))
    ; true
    ).

predicate_vars(Predicate, Vars) :-
    data_atom_vars(Predicate, Vars).

predicate_list_vars([], []).
predicate_list_vars([Predicate|Predicates], Vars) :-
    predicate_vars(Predicate, Here),
    predicate_list_vars(Predicates, Rest),
    append(Here, Rest, Vars).

body_vars([], []).
body_vars([Literal|Literals], Vars) :-
    ( shape_quantity_compare(Literal) ->
        quantity_compare_vars(Literal, Here)
    ; literal_data_atom(Literal, Atom) ->
        data_atom_vars(Atom, Here)
    ),
    body_vars(Literals, Rest),
    append(Here, Rest, Vars).

literal_data_atom(Literal, Predicate) :-
    has_functor(Literal, naf, 1),
    arg(1, Literal, Predicate),
    !.
literal_data_atom(Literal, Predicate) :-
    has_functor(Literal, naf, 2),
    arg(2, Literal, Predicate),
    !.
literal_data_atom(Literal, Literal) :-
    shape_data_atom(Literal).

literal_predicate(Literal, Predicate) :-
    ( has_functor(Literal, naf, 1) ->
        arg(1, Literal, Predicate)
    ; has_functor(Literal, naf, 2) ->
        arg(2, Literal, Predicate)
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
    ; has_functor(Term, alternative_set, 6) ->
        arg(2, Term, members(Members)),
        arg(3, Term, body(Body)),
        validate_alternative_set_safety(Index, Members, Body)
    ; true
    ),
    safety_naf_items(Items).

validate_alternative_set_safety(Index, Members, Body) :-
    ( first_naf_literal(Body, 1, Position) ->
        reject(safety, term(Index, alternative_set_naf(Position)))
    ; positive_body_vars(Body, PositiveVars),
      predicate_list_vars(Members, MemberVars),
      ( first_missing_var(MemberVars, PositiveVars, Missing) ->
          reject(safety,
              term(Index,
                  alternative_member_var_not_in_body(Missing)))
      ; true
      )
    ).

first_naf_literal([], _, _) :-
    fail.
first_naf_literal([Literal|_], Position, Position) :-
    naf_literal(Literal),
    !.
first_naf_literal([_|Literals], Position0, Position) :-
    Position1 is Position0 + 1,
    first_naf_literal(Literals, Position1, Position).

validate_rule_safety(Index, _Head, Body) :-
    first_positive_after_naf(Body, 1, false, Position),
    !,
    reject(safety, term(Index, positive_after_naf(Position))).
validate_rule_safety(Index, _Head, Body) :-
    first_unbound_quantity_compare(Body, 1, [], Position, Missing),
    !,
    reject(safety,
        term(Index,
            quantity_var_not_bound(Missing, body_literal(Position)))).
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

naf_literal(Literal) :-
    has_functor(Literal, naf, 1),
    !.
naf_literal(Literal) :-
    has_functor(Literal, naf, 2).

first_positive_after_naf([], _, _, _) :-
    fail.
first_positive_after_naf([Literal|Literals], Position0, SeenNaf, Position) :-
    ( naf_literal(Literal) ->
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

first_unbound_quantity_compare([], _, _, _, _) :-
    fail.
first_unbound_quantity_compare([Literal|Literals], Position, Covered0,
        FoundPosition, Missing) :-
    ( shape_quantity_compare(Literal) ->
        quantity_compare_vars(Literal, CompareVars),
        ( first_missing_var(CompareVars, Covered0, Missing0) ->
            FoundPosition = Position,
            Missing = Missing0
        ; Next is Position + 1,
          first_unbound_quantity_compare(
              Literals, Next, Covered0, FoundPosition, Missing)
        )
    ; naf_literal(Literal) ->
        Next is Position + 1,
        first_unbound_quantity_compare(
            Literals, Next, Covered0, FoundPosition, Missing)
    ; data_atom_vars(Literal, Here),
      append(Here, Covered0, Covered),
      Next is Position + 1,
      first_unbound_quantity_compare(
          Literals, Next, Covered, FoundPosition, Missing)
    ).

positive_body_vars([], []).
positive_body_vars([Literal|Literals], Vars) :-
    ( naf_literal(Literal) ->
        Here = []
    ; shape_quantity_compare(Literal) ->
        Here = []
    ; data_atom_vars(Literal, Here)
    ),
    positive_body_vars(Literals, Rest),
    append(Here, Rest, Vars).

naf_body_vars([], []).
naf_body_vars([Literal|Literals], Vars) :-
    ( naf_literal(Literal) ->
        literal_predicate(Literal, Predicate),
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

/* Pass 9b: labeled NAF has one exact complete-target declaration. */
exception_pass(Items) :-
    collect_closed_world(Items, Declarations),
    collect_defined_keys(Items, DefinedKeys),
    validate_closed_world_declarations(
        Declarations, Items, DefinedKeys, [], none),
    validate_labeled_exception_items(Items, Declarations, Used),
    require_all_declarations_used(Declarations, Used).

collect_closed_world([], []).
collect_closed_world([indexed(Index, Term)|Items], Declarations) :-
    ( has_functor(Term, closed_world, 3) ->
        Declarations = [indexed(Index, Term)|Rest]
    ; Declarations = Rest
    ),
    collect_closed_world(Items, Rest).

collect_defined_keys([], []).
collect_defined_keys([indexed(_, Term)|Items], Keys) :-
    ( has_functor(Term, clause, 3),
      arg(2, Term, Head),
      has_functor(Head, pred, 2) ->
        record_predicate_key(Head, Key),
        Keys = [Key|Rest]
    ; Keys = Rest
    ),
    collect_defined_keys(Items, Rest).

validate_closed_world_declarations([], _, _, _, _).
validate_closed_world_declarations(
        [indexed(Index, Declaration)|Declarations], Items, DefinedKeys,
        Seen, LastKey) :-
    Declaration = closed_world(ExceptionId, affects(Affected), TargetKey),
    ExceptionId = exception_id(rule(Embedded), literal(Literal)),
    ( Embedded == Affected -> true
    ; reject(exception,
          term(Index, affected_rule_mismatch(Embedded, Affected)))
    ),
    ( Literal >= 1 -> true
    ; reject(exception, term(Index, literal_ordinal(Literal)))
    ),
    TargetKey = predicate_key(_, arity(Arity)),
    ( Arity >= 1 -> true
    ; reject(exception, term(Index, target_arity(Arity)))
    ),
    ( rule_id_present(Affected, Items) -> true
    ; reject(exception, term(Index, affected_rule_missing(Affected)))
    ),
    ( member_eq(TargetKey, DefinedKeys) -> true
    ; reject(exception, term(Index, target_not_defined(TargetKey)))
    ),
    ( member_eq(ExceptionId, Seen) ->
        reject(exception, term(Index, duplicate_exception_id(ExceptionId)))
    ; true
    ),
    exception_order_key(ExceptionId, Key),
    ( LastKey == none -> true
    ; order_key_less_extended(LastKey, Key) -> true
    ; reject(exception, term(Index, declaration_order_after(LastKey)))
    ),
    validate_closed_world_declarations(Declarations, Items, DefinedKeys,
        [ExceptionId|Seen], Key).

rule_id_present(RuleId, [indexed(_, Term)|_]) :-
    has_functor(Term, clause, 3),
    arg(1, Term, Stored),
    Stored == RuleId,
    !.
rule_id_present(RuleId, [_|Items]) :-
    rule_id_present(RuleId, Items).

exception_order_key(
        exception_id(rule(RuleId), literal(Literal)),
        key(Sentence, Clause, Branch, Literal)) :-
    id_parts(RuleId, rule, Sentence, Clause),
    id_branch_number(RuleId, Branch).

order_key_less_extended(key(S0, C0, B0, L0), key(S, C, B, L)) :-
    ( S0 < S
    ; S0 =:= S, C0 < C
    ; S0 =:= S, C0 =:= C, B0 < B
    ; S0 =:= S, C0 =:= C, B0 =:= B, L0 < L
    ).

validate_labeled_exception_items([], _, []).
validate_labeled_exception_items([indexed(Index, Term)|Items],
        Declarations, Used) :-
    ( has_functor(Term, clause, 3),
      arg(1, Term, RuleId),
      compound(RuleId),
      functor(RuleId, rule_id, _) ->
        arg(3, Term, body(Body)),
        validate_labeled_exception_body(
            Body, 1, Index, RuleId, Declarations, HereUsed)
    ; HereUsed = []
    ),
    validate_labeled_exception_items(Items, Declarations, RestUsed),
    append(HereUsed, RestUsed, Used).

validate_labeled_exception_body([], _, _, _, _, []).
validate_labeled_exception_body([Literal|Literals], Position, Index,
        RuleId, Declarations, Used) :-
    ( has_functor(Literal, naf, 2) ->
        arg(1, Literal, ExceptionId),
        arg(2, Literal, Predicate),
        ExpectedId = exception_id(rule(RuleId), literal(Position)),
        ( ExceptionId == ExpectedId -> true
        ; reject(exception,
              term(Index,
                  label_position(expected(ExpectedId), found(ExceptionId))))
        ),
        record_predicate_key(Predicate, TargetKey),
        ( exact_closed_world_declaration(
              ExceptionId, RuleId, TargetKey, Declarations) ->
            Used = [ExceptionId|RestUsed]
        ; reject(exception,
              term(Index, undeclared_labeled_target(ExceptionId)))
        )
    ; Used = RestUsed
    ),
    Next is Position + 1,
    validate_labeled_exception_body(Literals, Next, Index, RuleId,
        Declarations, RestUsed).

exact_closed_world_declaration(ExceptionId, RuleId, TargetKey,
        [indexed(_, Declaration)|_]) :-
    Declaration = closed_world(StoredId, affects(StoredRule), StoredKey),
    StoredId == ExceptionId,
    StoredRule == RuleId,
    StoredKey == TargetKey,
    !.
exact_closed_world_declaration(ExceptionId, RuleId, TargetKey,
        [_|Declarations]) :-
    exact_closed_world_declaration(
        ExceptionId, RuleId, TargetKey, Declarations).

require_all_declarations_used([], _).
require_all_declarations_used([indexed(Index, Declaration)|Declarations],
        Used) :-
    arg(1, Declaration, ExceptionId),
    ( member_eq(ExceptionId, Used) -> true
    ; reject(exception, term(Index, unused_declaration(ExceptionId)))
    ),
    require_all_declarations_used(Declarations, Used).

record_predicate_key(pred(Name, Args),
    predicate_key(Name, arity(Arity))) :-
    length(Args, Arity).

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
    ( dependency_literal(Literal, Polarity, Atom) ->
        predicate_key(Atom, BodyKey),
        ( creates_cycle(HeadKey, BodyKey, Edges0) ->
            reject(cycle,
                term(Index,
                    body_literal(Position,
                        signed_dependency(Polarity, HeadKey, BodyKey))))
        ; Edges1 = [edge(Polarity, HeadKey, BodyKey)|Edges0]
        )
    ; Edges1 = Edges0
    ),
    Position1 is Position + 1,
    add_body_edges(Literals, Index, Position1, HeadKey, Edges1, Edges).

dependency_literal(Literal, naf, Predicate) :-
    has_functor(Literal, naf, 1),
    arg(1, Literal, Predicate),
    !.
dependency_literal(Literal, naf, Predicate) :-
    has_functor(Literal, naf, 2),
    arg(2, Literal, Predicate),
    !.
dependency_literal(Literal, positive, Literal) :-
    shape_data_atom(Literal).

predicate_key(Atom, Key) :-
    data_atom_key(Atom, Key).

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
    ; has_functor(Term, closed_world, 3) ->
        collect_program(Items, Seq0, Clauses, Goal)
    ; has_functor(Term, alternative_set, 6) ->
        collect_program(Items, Seq0, Clauses, Goal)
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
        AlternativeSets, ResultTerms) :-
    stratified_model(MaxStratum, ClauseCount, Store),
    arg(1, Goal, GoalKind),
    arg(2, Goal, GoalId),
    arg(3, Goal, GoalAtom),
    evaluation_outcome(GoalKind, GoalAtom, Store, Outcome, Roots),
    certificate_preflight(Roots, Store),
    assemble_result_terms(
        Document, ProgramDigest, GoalId, GoalAtom, Outcome, Store,
        AlternativeSets, ResultTerms).

evaluation_outcome(GoalKind, GoalAtom, Store, Outcome, Roots) :-
    ( GoalKind == yes_no ->
        ( atom_present(GoalAtom, Store) ->
            Outcome = proved,
            Roots = [GoalAtom]
        ; Outcome = not_proved,
          Roots = []
        )
    ; GoalKind == wh(who) ->
        wh_answer_atoms(GoalAtom, Store, Answers),
        Outcome = answers(Answers),
        Roots = Answers
    ; kernel_invariant(goal_kind(GoalKind))
    ).

wh_answer_atoms(Pattern, Store, Answers) :-
    ( wh_pattern_name(Pattern, Name) ->
        collect_wh_store_atoms(Name, Store, Atoms),
        canonical_atom_pairs(Atoms, Pairs),
        keysort(Pairs, SortedPairs),
        pair_atoms(SortedPairs, Answers)
    ; kernel_invariant(wh_pattern(Pattern))
    ).

wh_pattern_name(Pattern, Name) :-
    has_functor(Pattern, pred, 2),
    arg(1, Pattern, Name),
    atom(Name),
    arg(2, Pattern, Args),
    has_functor(Args, '[|]', 2),
    arg(1, Args, Variable),
    arg(2, Args, Tail),
    Variable == var(1),
    Tail == [].

collect_wh_store_atoms(_, [], []).
collect_wh_store_atoms(Name, [Entry|Entries], Atoms) :-
    arg(1, Entry, Atom),
    ( wh_store_instance(Name, Atom, AnswerArg) ->
        ( wh_named_answer_argument(AnswerArg) ->
            Atoms = [Atom|Rest]
        ; reject(wh_query, answer_argument_not_named(AnswerArg))
        )
    ; Atoms = Rest
    ),
    collect_wh_store_atoms(Name, Entries, Rest).

wh_store_instance(Name, Atom, AnswerArg) :-
    has_functor(Atom, pred, 2),
    arg(1, Atom, StoredName),
    StoredName == Name,
    arg(2, Atom, Args),
    has_functor(Args, '[|]', 2),
    arg(1, Args, AnswerArg),
    arg(2, Args, Tail),
    Tail == [],
    kernel_ground_predicate(Atom).

wh_named_answer_argument(AnswerArg) :-
    has_functor(AnswerArg, named, 1),
    arg(1, AnswerArg, Name),
    atom(Name).

canonical_atom_pairs([], []).
canonical_atom_pairs([Atom|Atoms], [Key-Atom|Pairs]) :-
    copy_term(Atom, Copy),
    ( catch(canonical_line(Copy, Line), _, fail) ->
        string_codes(Line, Key)
    ; kernel_invariant(canonical_wh_atom)
    ),
    canonical_atom_pairs(Atoms, Pairs).

pair_atoms([], []).
pair_atoms([_-Atom|Pairs], [Atom|Atoms]) :-
    pair_atoms(Pairs, Atoms).

certificate_node_cap(1000000).

certificate_preflight(Roots, Store) :-
    certificate_node_cap(Cap),
    certificate_preflight_with_cap(Roots, Store, Cap).

certificate_preflight_with_cap(Roots, Store, Cap) :-
    Limit is Cap + 1,
    certificate_root_total(
        Roots, Store, Limit, [], _, 0, Total),
    ( Total > Cap ->
        reject(resource, certificate_nodes_exceed_cap(Cap))
    ; true
    ).

certificate_root_total(_, _, Limit, Memo, Memo, Total, Total) :-
    Total >= Limit,
    !.
certificate_root_total([], _, _, Memo, Memo, Total, Total).
certificate_root_total([Atom|Atoms], Store, Limit, Memo0, Memo,
        Total0, Total) :-
    certificate_atom_count(Atom, Store, Limit, Memo0, Memo1, Count),
    saturating_add(Total0, Count, Limit, Total1),
    certificate_root_total(
        Atoms, Store, Limit, Memo1, Memo, Total1, Total).

certificate_atom_count(Atom, Store, Limit, Memo0, Memo, Count) :-
    ( certificate_memo_count(Atom, Memo0, Count) ->
        Memo = Memo0
    ; certificate_store_body(Atom, Store, Body) ->
        certificate_body_count(
            Body, Store, Limit, Memo0, Memo1, Count),
        Memo = [certificate_count(Atom, Count)|Memo1]
    ; kernel_invariant(certificate_missing_witness(Atom))
    ).

certificate_body_count(Body, Store, Limit, Memo0, Memo, Count) :-
    certificate_body_count_(
        Body, Store, Limit, Memo0, Memo, 1, Count).

certificate_body_count_(_, _, Limit, Memo, Memo, Count, Count) :-
    Count >= Limit,
    !.
certificate_body_count_(Body, _, _, Memo, Memo, Count, Count) :-
    Body == [],
    !.
certificate_body_count_(Body, Store, Limit, Memo0, Memo,
        Count0, Count) :-
    ( has_functor(Body, '[|]', 2) ->
        arg(1, Body, Item),
        arg(2, Body, Rest),
        certificate_item_count(
            Item, Store, Limit, Memo0, Memo1, ItemCount),
        saturating_add(Count0, ItemCount, Limit, Count1),
        certificate_body_count_(
            Rest, Store, Limit, Memo1, Memo, Count1, Count)
    ; kernel_invariant(certificate_body_shape)
    ).

certificate_item_count(Item, _, _, Memo, Memo, 1) :-
    naf_literal(Item),
    !.
certificate_item_count(Item, _, _, Memo, Memo, 1) :-
    shape_quantity_compare(Item),
    !.
certificate_item_count(Item, Store, Limit, Memo0, Memo, Count) :-
    ( shape_data_atom(Item) ->
        certificate_atom_count(
            Item, Store, Limit, Memo0, Memo, Count)
    ; kernel_invariant(certificate_body_item)
    ).

certificate_store_body(Atom, [Entry|_], Body) :-
    arg(1, Entry, Stored),
    Stored == Atom,
    arg(2, Entry, Witness),
    has_functor(Witness, by, 2),
    arg(2, Witness, Body),
    !.
certificate_store_body(Atom, [_|Entries], Body) :-
    certificate_store_body(Atom, Entries, Body).

certificate_memo_count(Atom,
        [certificate_count(Stored, Count)|_], Count) :-
    Stored == Atom,
    !.
certificate_memo_count(Atom, [_|Memo], Count) :-
    certificate_memo_count(Atom, Memo, Count).

saturating_add(Left, Right, Limit, Sum) :-
    ( Left >= Limit ->
        Sum = Limit
    ; Right >= Limit ->
        Sum = Limit
    ; Remaining is Limit - Left,
      ( Right >= Remaining ->
          Sum = Limit
      ; Sum is Left + Right
      )
    ).

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
          substitute_data_atom(Head, Bindings, GroundHead)
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
        [Evidence|Grounds]) :-
    arg(1, Body, Literal),
    naf_literal(Literal),
    literal_predicate(Literal, Pattern),
    ( substitute_predicate(Pattern, Bindings0, GroundAtom),
      kernel_ground_predicate(GroundAtom) ->
        true
    ; kernel_invariant(naf_not_ground(Pattern))
    ),
    \+ atom_present(GroundAtom, Snapshot),
    naf_evidence(Literal, GroundAtom, Evidence),
    arg(2, Body, Rest),
    body_solution(Rest, Snapshot, Bindings0, Bindings, Grounds).
body_solution(Body, Snapshot, Bindings0, Bindings,
        [Evidence|Grounds]) :-
    arg(1, Body, Literal),
    shape_quantity_compare(Literal),
    !,
    arg(1, Literal, ActualPattern),
    arg(2, Literal, Bound),
    substitute_argument(ActualPattern, Bindings0, Actual),
    quantity_compare_outcome(Actual, Bound, Outcome),
    quantity_compare_solution(
        Outcome, Actual, Bound, Evidence),
    arg(2, Body, Rest),
    body_solution(Rest, Snapshot, Bindings0, Bindings, Grounds).
body_solution(Body, Snapshot, Bindings0, Bindings, [Ground|Grounds]) :-
    arg(1, Body, Pattern),
    arg(2, Body, Rest),
    store_atom(Snapshot, Ground),
    match_data_atom(Pattern, Ground, Bindings0, Bindings1),
    body_solution(Rest, Snapshot, Bindings1, Bindings, Grounds).

quantity_compare_solution(true, Actual, Bound,
    quantity_compare(Actual, Bound)).
quantity_compare_solution(false, _, _, _) :-
    fail.
quantity_compare_solution(cross_unit(ActualUnit, BoundUnit), _, _, _) :-
    reject(quantity, runtime_cross_unit(ActualUnit, BoundUnit)).
quantity_compare_solution(invalid_actual(Actual), _, _, _) :-
    reject(quantity, runtime_actual_not_quantity(Actual)).

naf_evidence(Literal, GroundAtom, naf(GroundAtom)) :-
    has_functor(Literal, naf, 1),
    !.
naf_evidence(Literal, GroundAtom, naf(ExceptionId, GroundAtom)) :-
    arg(1, Literal, ExceptionId).

store_atom([entry(Atom, _)|_], Atom).
store_atom([_|Entries], Atom) :-
    store_atom(Entries, Atom).

match_data_atom(Pattern, Ground, Bindings0, Bindings) :-
    ( has_functor(Pattern, pred, 2),
      has_functor(Ground, pred, 2) ->
        match_predicate(Pattern, Ground, Bindings0, Bindings)
    ; has_functor(Pattern, temporal, 3),
      has_functor(Ground, temporal, 3) ->
        arg(1, Pattern, PatternRelation),
        arg(1, Ground, GroundRelation),
        PatternRelation == GroundRelation,
        arg(2, Pattern, PatternEvent),
        arg(2, Ground, GroundEvent),
        match_predicate(
            PatternEvent, GroundEvent, Bindings0, Bindings1),
        arg(3, Pattern, PatternAnchor),
        arg(3, Ground, GroundAnchor),
        match_anchor(
            PatternAnchor, GroundAnchor, Bindings1, Bindings)
    ; has_functor(Pattern, temporal_window, 4),
      has_functor(Ground, temporal_window, 4),
      arg(1, Pattern, PatternDirection),
      arg(1, Ground, GroundDirection),
      PatternDirection == GroundDirection,
      arg(2, Pattern, PatternEvent),
      arg(2, Ground, GroundEvent),
      match_predicate(
          PatternEvent, GroundEvent, Bindings0, Bindings1),
      arg(3, Pattern, PatternAnchor),
      arg(3, Ground, GroundAnchor),
      match_anchor(
          PatternAnchor, GroundAnchor, Bindings1, Bindings),
      arg(4, Pattern, PatternInterval),
      arg(4, Ground, GroundInterval),
      PatternInterval == GroundInterval
    ).

match_anchor(Pattern, Ground, Bindings0, Bindings) :-
    arg(1, Pattern, PatternArg),
    arg(1, Ground, GroundArg),
    match_argument(PatternArg, GroundArg, Bindings0, Bindings).

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
    shape_ground_argument(Pattern),
    Pattern == Ground,
    !.
match_argument(Pattern, Ground, Bindings0, Bindings) :-
    has_functor(Pattern, var, 1),
    ground_argument(Ground),
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
    ground_argument(Arg),
    kernel_ground_arguments(Args).

kernel_ground_data_atom(Atom) :-
    ground_data_atom(Atom).

binding_value(Number, [binding(Here, Value)|_], Value) :-
    Here =:= Number,
    !.
binding_value(Number, [_|Bindings], Value) :-
    binding_value(Number, Bindings, Value).

substitute_data_atom(Pattern, Bindings, Ground) :-
    ( has_functor(Pattern, pred, 2) ->
        substitute_predicate(Pattern, Bindings, Ground)
    ; has_functor(Pattern, temporal, 3) ->
        arg(1, Pattern, Relation),
        arg(2, Pattern, EventPattern),
        arg(3, Pattern, AnchorPattern),
        substitute_predicate(EventPattern, Bindings, Event),
        substitute_anchor(AnchorPattern, Bindings, Anchor),
        Ground = temporal(Relation, Event, Anchor)
    ; has_functor(Pattern, temporal_window, 4),
      arg(1, Pattern, Direction),
      arg(2, Pattern, EventPattern),
      arg(3, Pattern, AnchorPattern),
      arg(4, Pattern, Interval),
      substitute_predicate(EventPattern, Bindings, Event),
      substitute_anchor(AnchorPattern, Bindings, Anchor),
      Ground = temporal_window(Direction, Event, Anchor, Interval)
    ).

substitute_anchor(Pattern, Bindings, anchor(GroundArg)) :-
    arg(1, Pattern, Arg),
    substitute_argument(Arg, Bindings, GroundArg).

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
    shape_ground_argument(Arg),
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

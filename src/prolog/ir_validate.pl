:- module(ir_validate, [validate_terms/1]).

:- set_prolog_flag(encoding, utf8).

:- use_module(library(lists), [append/3, memberchk/2]).
:- use_module(validation_common, [
    branch_shape_detail/2,
    shape_optional_branch/2,
    terms_pairwise_distinct/1,
    shape_argument/1,
    shape_predicate_term/1,
    shape_data_atom/1,
    shape_quantity_compare/1,
    data_atom_vars/2,
    quantity_compare_vars/2,
    quantity_compare_outcome/3,
    temporal_window_error/2,
    data_atom_key/2
]).

/*
IR v3 semantic validator. Input is the parsed term stream after strict UTF-8,
syntax, and canonical-byte gates. Pass order is part of the public contract:
envelope, shape, identity, ordering, scope, safety/NAF, dependency cycles.
Every rejection throws ir_reject(Class, Detail); callers own framing and exits.
*/
validate_terms(Terms) :-
    envelope_pass(Terms, Document, Items),
    shape_pass(Document, Items),
    constructor_semantics_pass(Items),
    identity_pass(Document, Items),
    ordering_pass(Items),
    scope_pass(Items),
    safety_naf_pass(Items),
    exception_pass(Items),
    cycle_pass(Items).

/* Pass 4: record envelope, one query, and facts -> rules -> query order. */
envelope_pass([], _, _) :-
    reject(envelope, term(1, missing_header)).
envelope_pass([Header|Rest], Document, Items) :-
    ( Header == cnl_ir_record(3) ->
        true
    ; reject(envelope, term(1, expected(cnl_ir_record(3))))
    ),
    require_document(Rest, Document, RawItems),
    index_terms(RawItems, 3, Items),
    query_positions(Items, QueryPositions),
    require_one_query(QueryPositions),
    split_query(Items, Prefix, _Query, Tail),
    ( Tail == [] ->
        true
    ; Tail = [indexed(Index, _)|_],
      reject(envelope, term(Index, trailing_after_query))
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

query_positions([], []).
query_positions([indexed(Index, Term)|Items], Positions) :-
    ( query_term(Term) ->
        Positions = [Index|Rest]
    ; Positions = Rest
    ),
    query_positions(Items, Rest).

query_term(Term) :-
    has_functor(Term, query, 3),
    !.
query_term(Term) :-
    has_functor(Term, query, 4).

require_one_query([_]) :-
    !.
require_one_query(Positions) :-
    length(Positions, Count),
    ( Positions = [_, Second|_] ->
        reject(query_count, count(Count, second_term(Second)))
    ; reject(query_count, count(Count))
    ).

split_query([Item|Items], Prefix, Query, Tail) :-
    Item = indexed(_, Term),
    ( query_term(Term) ->
        Prefix = [],
        Query = Item,
        Tail = Items
    ; Prefix = [Item|Rest],
      split_query(Items, Rest, Query, Tail)
    ).

section_order([], _).
section_order([indexed(Index, Term)|Items], State0) :-
    item_section(Term, Section),
    section_transition(State0, Section, Index, State),
    section_order(Items, State).

item_section(Term, fact) :-
    has_functor(Term, fact, 3),
    !.
item_section(Term, closed_world) :-
    has_functor(Term, closed_world, 3),
    !.
item_section(Term, rule) :-
    has_functor(Term, rule, 4),
    !.
item_section(Term, alternative_set) :-
    has_functor(Term, alternative_set, 7),
    !.
item_section(_, unknown).

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

shape_document(document(Docid, SourceHash, Ulex)) :-
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
    ; has_functor(Term, fact, 3) ->
        ( shape_fact(Term) -> true
        ; reject(shape, term(Index, fact))
        )
    ; has_functor(Term, closed_world, 3) ->
        ( shape_closed_world(Term) -> true
        ; reject(shape, term(Index, closed_world))
        )
    ; has_functor(Term, rule, 4) ->
        ( shape_rule(Term) -> true
        ; reject(shape, term(Index, rule))
        )
    ; has_functor(Term, alternative_set, 7) ->
        ( shape_alternative_set(Term) -> true
        ; reject(shape, term(Index, alternative_set))
        )
    ; query_term(Term) ->
        ( shape_query(Term) -> true
        ; reject(shape, term(Index, query))
        )
    ; reject(shape, term(Index, item))
    ),
    shape_items(Items).

shape_fact(fact(Id, Atom, Source)) :-
    shape_id(Id),
    shape_data_atom(Atom),
    shape_source(Source).

shape_rule(rule(Id, Head, Body, Source)) :-
    shape_id(Id),
    shape_predicate(Head),
    shape_body(Body),
    shape_source(Source).

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
        Satisfaction, Exclusivity, Exhaustiveness, Source)) :-
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
    Exhaustiveness == exhaustiveness(not_asserted),
    shape_source(Source).

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

shape_query(query(Id, Predicate, Source)) :-
    shape_id(Id),
    shape_predicate(Predicate),
    shape_source(Source),
    !.
shape_query(Term) :-
    has_functor(Term, query, 4),
    arg(1, Term, Id),
    arg(2, Term, Marker),
    arg(3, Term, Predicate),
    arg(4, Term, Source),
    shape_id(Id),
    Marker == wh(who),
    shape_wh_predicate(Predicate),
    shape_source(Source).

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

shape_body(body(Literals)) :-
    is_list(Literals),
    shape_literals(Literals).

shape_alternative_body(body(Literals)) :-
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

shape_source(source(Sentence, Tokens)) :-
    shape_integer_wrapper(Sentence, sentence),
    has_functor(Tokens, tokens, 1),
    arg(1, Tokens, Ordinals),
    is_list(Ordinals),
    Ordinals = [_|_],
    integer_list(Ordinals).

integer_list([]).
integer_list([Value|Values]) :-
    integer(Value),
    integer_list(Values).

constructor_semantics_pass(Items) :-
    constructor_semantics_items(Items).

constructor_semantics_items([]).
constructor_semantics_items([indexed(Index, Term)|Items]) :-
    ( has_functor(Term, fact, 3) ->
        arg(2, Term, Atom),
        check_data_atom_semantics(Index, fact, 0, Atom)
    ; has_functor(Term, rule, 4) ->
        arg(3, Term, body(Body)),
        check_body_constructor_semantics(Body, Index, 1)
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

/* Pass 6: document identity, ID kind/bounds, and provenance agreement. */
identity_pass(document(docid(Docid), source_sha256(SourceHash), ulex(Ulex)),
        Items) :-
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
    ; item_identity_parts(Term, ExpectedKind, Id, Source),
      id_parts(Id, ActualKind, Sentence, Clause),
      ( ActualKind == ExpectedKind -> true
      ; reject(identity, term(Index, id_kind(ExpectedKind, ActualKind)))
      ),
      require_id_branch(Index, ExpectedKind, Id),
      require_positive(Index, id_sentence, Sentence),
      require_positive(Index, id_clause, Clause),
      source_parts(Source, SourceSentence, Tokens),
      require_positive(Index, source_sentence, SourceSentence),
      ( Sentence =:= SourceSentence -> true
      ; reject(identity,
            term(Index,
                sentence_mismatch(id(Sentence), source(SourceSentence))))
      ),
      require_positive_tokens(Index, Tokens, 1)
    ),
    identity_items(Items).

item_identity_parts(fact(Id, _, Source), fact, Id, Source).
item_identity_parts(rule(Id, _, _, Source), rule, Id, Source).
item_identity_parts(alternative_set(Id, _, _, _, _, _, Source),
    alternative_set, Id, Source).
item_identity_parts(query(Id, _, Source), query, Id, Source).
item_identity_parts(query(Id, _, _, Source), query, Id, Source).

id_parts(Id, Kind, Sentence, Clause) :-
    functor(Id, IdName, _),
    id_kind_name(IdName, Kind),
    arg(1, Id, sentence(Sentence)),
    arg(2, Id, clause(Clause)).

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

source_parts(source(sentence(Sentence), tokens(Tokens)), Sentence, Tokens).

require_positive(_, _, Value) :-
    Value >= 1,
    !.
require_positive(Index, Field, Value) :-
    reject(identity, term(Index, ordinal(Field, Value))).

require_positive_tokens(_, [], _).
require_positive_tokens(Index, [Token|Tokens], Position) :-
    ( Token >= 1 -> true
    ; reject(identity, term(Index, token(Position, Token)))
    ),
    Next is Position + 1,
    require_positive_tokens(Index, Tokens, Next).

/* Pass 7: canonical item order, origin ownership, branch density, tokens. */
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
    ; item_identity_parts(Term, Section, Id, Source),
      id_parts(Id, _, Sentence, Clause),
      id_branch_number(Id, Branch),
      Origin = pair(Sentence, Clause),
      ( term_member_eq(Id, SeenIds) ->
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
      source_parts(Source, _, Tokens),
      ( strictly_ascending(Tokens) -> true
      ; reject(ordering, term(Index, tokens_not_strictly_ascending))
      ),
      state_put(Section, Key, State0, State),
      SeenIds1 = [Id|SeenIds],
      SeenOrigins1 = [seen_origin(Section, Origin, Branch)|SeenOrigins]
    ),
    ordering_items(Items, SeenIds1, SeenOrigins1, State).

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
    ; S0 =:= S,
      C0 < C
    ; S0 =:= S,
      C0 =:= C,
      B0 < B
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
    item_identity_parts(Term, Section, Id, _),
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

strictly_ascending([_]).
strictly_ascending([First, Second|Rest]) :-
    First < Second,
    strictly_ascending([Second|Rest]).

term_member_eq(Term, [Member|_]) :-
    Term == Member,
    !.
term_member_eq(Term, [_|Members]) :-
    term_member_eq(Term, Members).

/* Pass 8: variables only in rules; rule numbering is dense first-occurrence. */
scope_pass(Items) :-
    scope_items(Items).

scope_items([]).
scope_items([indexed(Index, Term)|Items]) :-
    ( has_functor(Term, fact, 3) ->
        arg(2, Term, Atom),
        reject_if_data_atom_variable(Index, Atom)
    ; has_functor(Term, query, 3) ->
        arg(2, Term, Predicate),
        reject_if_predicate_variable(Index, Predicate)
    ; has_functor(Term, rule, 4) ->
        arg(2, Term, Head),
        arg(3, Term, BodyTerm),
        arg(1, BodyTerm, Body),
        predicate_vars(Head, HeadVars),
        literal_vars(Body, BodyVars),
        append(HeadVars, BodyVars, Vars),
        dense_first_occurrence(Index, Vars, [], 1, 1)
    ; has_functor(Term, alternative_set, 7) ->
        arg(2, Term, members(Members)),
        arg(3, Term, body(Body)),
        predicate_list_vars(Members, MemberVars),
        literal_vars(Body, BodyVars),
        append(MemberVars, BodyVars, Vars),
        dense_first_occurrence(Index, Vars, [], 1, 1)
    ; true
    ),
    scope_items(Items).

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

literal_vars([], []).
literal_vars([Literal|Literals], Vars) :-
    ( shape_quantity_compare(Literal) ->
        quantity_compare_vars(Literal, Here)
    ; literal_data_atom(Literal, Atom) ->
        data_atom_vars(Atom, Here)
    ),
    literal_vars(Literals, Rest),
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
    ( memberchk(Number, Seen) ->
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
    ( has_functor(Term, rule, 4) ->
        arg(2, Term, Head),
        arg(3, Term, BodyTerm),
        arg(1, BodyTerm, Body),
        validate_rule_safety(Index, Head, Body)
    ; has_functor(Term, alternative_set, 7) ->
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

validate_rule_safety(Index, _Head, Body) :-
    first_positive_after_naf(Body, 1, false, Position),
    !,
    reject(safety, term(Index, positive_after_naf(Position))).
validate_rule_safety(Index, _Head, Body) :-
    Body == [],
    !,
    reject(safety, term(Index, empty_body)).
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

member_number(Number, [Member|_]) :-
    Member =:= Number,
    !.
member_number(Number, [_|Members]) :-
    member_number(Number, Members).

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
    ( has_functor(Term, fact, 3),
      arg(2, Term, Head),
      has_functor(Head, pred, 2) ->
        record_predicate_key(Head, Key),
        Keys = [Key|Rest]
    ; has_functor(Term, rule, 4) ->
        arg(2, Term, Head),
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
    ( term_member_eq(TargetKey, DefinedKeys) -> true
    ; reject(exception, term(Index, target_not_defined(TargetKey)))
    ),
    ( term_member_eq(ExceptionId, Seen) ->
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
    has_functor(Term, rule, 4),
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
    ( has_functor(Term, rule, 4) ->
        arg(1, Term, RuleId),
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
    ( term_member_eq(ExceptionId, Used) -> true
    ; reject(exception, term(Index, unused_declaration(ExceptionId)))
    ),
    require_all_declarations_used(Declarations, Used).

record_predicate_key(pred(Name, Args),
    predicate_key(Name, arity(Arity))) :-
    length(Args, Arity).

/* Pass 10: reject the first signed dependency edge closing any cycle. */
cycle_pass(Items) :-
    cycle_items(Items, []).

cycle_items([], _).
cycle_items([indexed(Index, Term)|Items], Edges0) :-
    ( has_functor(Term, rule, 4) ->
        arg(2, Term, Head),
        arg(3, Term, BodyTerm),
        arg(1, BodyTerm, Body),
        predicate_key(Head, HeadKey),
        add_body_edges(Body, Index, 1, HeadKey, Edges0, Edges)
    ; Edges = Edges0
    ),
    cycle_items(Items, Edges).

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
    \+ memberchk(Node, Visited),
    edge_from(Node, Edges, Next),
    reachable(Next, Target, Edges, [Node|Visited]).

edge_from(Node, [edge(_, From, To)|_], To) :-
    From == Node.
edge_from(Node, [_|Edges], Next) :-
    edge_from(Node, Edges, Next).

has_functor(Term, Name, Arity) :-
    compound(Term),
    functor(Term, Name, Arity).

reject(Class, Detail) :-
    throw(ir_reject(Class, Detail)).

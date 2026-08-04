:- module(drs_to_ir, [lower_terms/2]).

:- set_prolog_flag(encoding, utf8).

:- use_module(library(lists), [append/3, memberchk/2]).
:- use_module(ir_validate, [validate_terms/1]).

/*
Lossless lowering for the deliberately small IR v3 DRS profile. The caller owns
UTF-8, syntax, canonical-byte framing, output buffering, and error emission.
This module owns the M2 envelope and every admitted DRS semantic decision.
*/
lower_terms(Terms, IrTerms) :-
    lower_envelope(Terms, Document, RootDomain, RawRootConditions),
    normalize_condition_list(root, RawRootConditions, RootConditions),
    reject_real_quantities(RootConditions),
    reject_invalid_quantity_objects(RootConditions),
    reject_false_friend_windows(RootConditions),
    reject_scalar_cross_units(RootConditions),
    validate_domain(root, RootDomain),
    require_final_question(RootConditions, FactualConditions, Question),
    validate_nested_domain_declarations(RootConditions, RootDomain),
    index_conditions(RootConditions, 1, AllConditions),
    index_conditions(FactualConditions, 1, IndexedConditions),
    lower_root_items(IndexedConditions, AllConditions, RootDomain, [],
        [], Counters0, FactualItems, RootEvents, RootEntities),
    validate_scope_accounting(root, RootDomain, RootEvents, RootEntities),
    lower_question(Question, QueryDraft),
    finalize_draft(QueryDraft, Counters0, _Counters, Query),
    require_factual_section_order(FactualItems),
    split_factual_items(FactualItems, Facts, Rules0, AlternativeSets),
    label_defined_exceptions(Facts, Rules0, Rules, ClosedWorld),
    require_section_order(fact, Facts),
    require_section_order(rule, Rules),
    require_section_order(alternative_set, AlternativeSets),
    append(Facts, ClosedWorld, Prefix0),
    append(Prefix0, Rules, Prefix1),
    append(Prefix1, AlternativeSets, Prefix),
    append([cnl_ir_record(3), Document|Prefix], [Query], IrTerms),
    validate_generated_ir(IrTerms).

/* M2 record envelope and durable document identity. */
lower_envelope([], _, _, _) :-
    reject(envelope, term(1, missing_header)).
lower_envelope([Header|Rest], Document, Domain, Conditions) :-
    ( Header == ace_front_end_record(1) ->
        true
    ; reject(envelope, term(1, expected(ace_front_end_record(1))))
    ),
    require_m2_document(Rest, Document, AfterDocument),
    require_m2_drs(AfterDocument, Domain, Conditions, Tail),
    ( Tail == [] ->
        true
    ; reject(envelope, term(4, trailing_term))
    ).

require_m2_document([], _, _) :-
    reject(envelope, term(2, missing_document)).
require_m2_document([Document|Rest], Document, Rest) :-
    ( valid_m2_document(Document) ->
        true
    ; reject(envelope, term(2, document))
    ).

require_m2_drs([], _, _, _) :-
    reject(envelope, term(3, missing_drs)).
require_m2_drs([Drs|Rest], Domain, Conditions, Rest) :-
    ( has_functor(Drs, drs, 2) ->
        arg(1, Drs, Domain0),
        arg(2, Drs, Conditions0),
        ( is_list(Domain0), is_list(Conditions0) ->
            Domain = Domain0,
            Conditions = Conditions0
        ; reject(envelope, term(3, drs_lists))
        )
    ; reject(envelope, term(3, expected(drs/2)))
    ).

valid_m2_document(Document) :-
    has_functor(Document, document, 3),
    arg(1, Document, DocidTerm),
    arg(2, Document, SourceTerm),
    arg(3, Document, UlexTerm),
    has_functor(DocidTerm, docid, 1),
    arg(1, DocidTerm, Docid),
    has_functor(SourceTerm, source_sha256, 1),
    arg(1, SourceTerm, SourceHash),
    has_functor(UlexTerm, ulex, 1),
    arg(1, UlexTerm, Ulex),
    valid_docid(Docid),
    valid_sha256(SourceHash),
    valid_ulex(Ulex).

valid_docid(Docid) :-
    atom(Docid),
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
    atom(Hash),
    atom_codes(Hash, Codes),
    length(Codes, 64),
    lower_hex_codes(Codes).

lower_hex_codes([]).
lower_hex_codes([Code|Codes]) :-
    ( Code >= 0'0, Code =< 0'9
    ; Code >= 0'a, Code =< 0'f
    ),
    lower_hex_codes(Codes).

valid_ulex(Ulex) :-
    Ulex == none,
    !.
valid_ulex(Ulex) :-
    has_functor(Ulex, sha256, 1),
    arg(1, Ulex, Hash),
    valid_sha256(Hash).

normalize_condition_list(_, [], []).
normalize_condition_list(Location, [Condition0|Conditions0],
        [Condition|Conditions]) :-
    normalize_condition_wrapper(Location, Condition0, Condition),
    normalize_condition_list(Location, Conditions0, Conditions).

normalize_condition_wrapper(_, [Condition], Condition) :-
    anchored_condition(Condition, Inner, _),
    unit_bound_object_parts(Inner, _, _, _, Relation, _),
    Relation == leq,
    !.
normalize_condition_wrapper(Location, Condition, _) :-
    is_list(Condition),
    !,
    reject(quantity, Location-condition_wrapper).
normalize_condition_wrapper(_, Condition, Condition).

reject_real_quantities(Term) :-
    ( var(Term) ->
        true
    ; has_functor(Term, real, 2) ->
        reject(quantity, real_not_integer)
    ; compound(Term) ->
        functor(Term, _, Arity),
        reject_real_quantity_args(Term, 1, Arity)
    ; true
    ).

reject_real_quantity_args(_, Index, Arity) :-
    Index > Arity,
    !.
reject_real_quantity_args(Term, Index, Arity) :-
    arg(Index, Term, Arg),
    reject_real_quantities(Arg),
    Next is Index + 1,
    reject_real_quantity_args(Term, Next, Arity).

reject_invalid_quantity_objects(Term) :-
    ( var(Term) ->
        true
    ; has_functor(Term, object, 6),
      arg(3, Term, Countability),
      Countability == mass,
      arg(4, Term, Unit),
      atom(Unit),
      arg(5, Term, Relation),
      arg(6, Term, Value),
      integer(Value),
      \+ admitted_quantity_relation(Relation) ->
        reject(quantity, object_bound_relation(Relation))
    ; compound(Term) ->
        functor(Term, _, Arity),
        reject_invalid_quantity_object_args(Term, 1, Arity)
    ; true
    ).

reject_invalid_quantity_object_args(_, Index, Arity) :-
    Index > Arity,
    !.
reject_invalid_quantity_object_args(Term, Index, Arity) :-
    arg(Index, Term, Arg),
    reject_invalid_quantity_objects(Arg),
    Next is Index + 1,
    reject_invalid_quantity_object_args(Term, Next, Arity).

reject_false_friend_windows(Term) :-
    ( var(Term) ->
        true
    ; is_list(Term) ->
        reject_false_friend_condition_list(Term),
        reject_false_friend_members(Term)
    ; compound(Term) ->
        functor(Term, _, Arity),
        reject_false_friend_args(Term, 1, Arity)
    ; true
    ).

reject_false_friend_condition_list(Conditions) :-
    ( condition_modifier(Conditions, Event, within, WithinArg),
      has_functor(WithinArg, int, 1),
      condition_modifier(Conditions, ToEvent, to, _),
      Event == ToEvent ->
        reject(temporal, false_friend_within_to_range)
    ; true
    ).

reject_false_friend_members([]).
reject_false_friend_members([Term|Terms]) :-
    reject_false_friend_windows(Term),
    reject_false_friend_members(Terms).

reject_false_friend_args(_, Index, Arity) :-
    Index > Arity,
    !.
reject_false_friend_args(Term, Index, Arity) :-
    arg(Index, Term, Arg),
    reject_false_friend_windows(Arg),
    Next is Index + 1,
    reject_false_friend_args(Term, Next, Arity).

reject_scalar_cross_units(Term) :-
    ( var(Term) ->
        true
    ; is_list(Term) ->
        reject_scalar_cross_units_in_list(Term, Term),
        reject_scalar_cross_unit_members(Term)
    ; compound(Term) ->
        functor(Term, _, Arity),
        reject_scalar_cross_unit_args(Term, 1, Arity)
    ; true
    ).

reject_scalar_cross_units_in_list([], _).
reject_scalar_cross_units_in_list([Condition|Conditions], All) :-
    ( anchored_condition(Condition, Property, _),
      scalar_comparison_property(Property, Carrier, Comparison),
      scalar_comparison_be(All, Carrier, Actual),
      scalar_quantity_operand(Actual, All, _, ActualUnit),
      scalar_quantity_operand(Comparison, All, _, ComparisonUnit),
      ActualUnit \== ComparisonUnit ->
        reject(quantity,
            scalar_cross_unit(ActualUnit, ComparisonUnit))
    ; true
    ),
    reject_scalar_cross_units_in_list(Conditions, All).

reject_scalar_cross_unit_members([]).
reject_scalar_cross_unit_members([Term|Terms]) :-
    reject_scalar_cross_units(Term),
    reject_scalar_cross_unit_members(Terms).

reject_scalar_cross_unit_args(_, Index, Arity) :-
    Index > Arity,
    !.
reject_scalar_cross_unit_args(Term, Index, Arity) :-
    arg(Index, Term, Arg),
    reject_scalar_cross_units(Arg),
    Next is Index + 1,
    reject_scalar_cross_unit_args(Term, Next, Arity).

scalar_comparison_property(Property, Carrier, Comparison) :-
    has_functor(Property, property, 4),
    arg(1, Property, Carrier),
    arg(2, Property, Adjective),
    arg(3, Property, Degree),
    arg(4, Property, Comparison),
    Adjective == great,
    Degree == comp_than.

scalar_comparison_be(Conditions, Carrier, Actual) :-
    condition_inner(Conditions, Be),
    predicate4_parts(Be, _, Verb, Actual, Object),
    Verb == be,
    Object == Carrier.

scalar_quantity_operand(Term, _, Quantity, Unit) :-
    quantity_from_int(Term, Quantity, Unit),
    !.
scalar_quantity_operand(Term, Conditions, Quantity, Unit) :-
    var(Term),
    condition_inner(Conditions, Object),
    count_quantity_object_parts(
        Object, Referent, Unit, eq, Value),
    Referent == Term,
    Quantity = quantity(integer(Value), unit(Unit)).

condition_modifier(Conditions, Event, Relation, Arg) :-
    condition_inner(Conditions, Modifier),
    has_functor(Modifier, modifier_pp, 3),
    arg(1, Modifier, Event),
    arg(2, Modifier, Relation),
    arg(3, Modifier, Arg).

condition_inner([Condition|_], Inner) :-
    anchored_condition(Condition, Inner, _).
condition_inner([_|Conditions], Inner) :-
    condition_inner(Conditions, Inner).

quantity_from_int(Term, quantity(integer(Value), unit(Unit)), Unit) :-
    has_functor(Term, int, 2),
    arg(1, Term, Value),
    arg(2, Term, Unit),
    integer(Value),
    atom(Unit).

unit_bound_object_parts(Object, Referent, Class, Unit, Relation, Value) :-
    has_functor(Object, object, 6),
    arg(1, Object, Referent),
    arg(2, Object, Class),
    arg(3, Object, Countability),
    arg(4, Object, Unit),
    arg(5, Object, Relation),
    arg(6, Object, Value),
    var(Referent),
    atom(Class),
    Countability == mass,
    atom(Unit),
    admitted_quantity_relation(Relation),
    integer(Value).

count_quantity_object_parts(Object, Referent, Unit, Relation, Value) :-
    has_functor(Object, object, 6),
    arg(1, Object, Referent),
    arg(2, Object, Unit),
    arg(3, Object, Countability),
    arg(4, Object, Definiteness),
    arg(5, Object, Relation),
    arg(6, Object, Value),
    var(Referent),
    atom(Unit),
    Countability == countable,
    Definiteness == na,
    admitted_quantity_relation(Relation),
    integer(Value),
    ( Relation \== eq ; Value =\= 1 ).

admitted_quantity_relation(Relation) :-
    ( Relation == eq
    ; Relation == geq
    ; Relation == leq
    ; Relation == greater
    ; Relation == less
    ).

quantity_bound_from_relation(Relation, Value, Unit,
        quantity_bound(Relation, Endpoint,
            quantity(integer(Value), unit(Unit)))) :-
    quantity_relation_endpoint(Relation, Endpoint).

quantity_relation_endpoint(Relation, Endpoint) :-
    ( Relation == eq -> Endpoint = closed
    ; Relation == geq -> Endpoint = closed
    ; Relation == leq -> Endpoint = closed
    ; Relation == greater -> Endpoint = open
    ; Relation == less -> Endpoint = open
    ).

/* A root record must have one final yes/no question. */
require_final_question(Conditions, Factual, Question) :-
    question_positions(Conditions, 1, Positions),
    ( Positions == [] ->
        reject(question_count, count(0))
    ; Positions = [Only] ->
        length(Conditions, Count),
        ( Only =:= Count ->
            append(Factual, [Question], Conditions)
        ; reject(question_count, non_final(position(Only)))
        )
    ; Positions = [_, Second|_],
      length(Positions, Count),
      reject(question_count, count(Count, second_position(Second)))
    ).

question_positions([], _, []).
question_positions([Condition|Conditions], Position, Positions) :-
    ( has_functor(Condition, question, 1) ->
        Positions = [Position|Rest]
    ; Positions = Rest
    ),
    Next is Position + 1,
    question_positions(Conditions, Next, Rest).

/* A referent is declared by exactly one DRS domain in the admitted tree. */
validate_nested_domain_declarations(Conditions, RootDomain) :-
    validate_nested_domain_declarations(Conditions, RootDomain, _).

validate_nested_domain_declarations([], Seen, Seen).
validate_nested_domain_declarations([Condition|Conditions], Seen0, Seen) :-
    direct_nested_domains(Condition, Domains),
    add_fresh_domains(Domains, Seen0, Seen1),
    validate_nested_domain_declarations(Conditions, Seen1, Seen).

direct_nested_domains(Condition, Domains) :-
    condition_nested_domains(Condition, Domains).

condition_nested_domains(Condition, Domains) :-
    has_functor(Condition, '=>', 2),
    arg(1, Condition, Antecedent),
    arg(2, Condition, Consequent),
    nested_drs_domains(Antecedent, AnteDomains),
    nested_drs_domains(Consequent, ConsequentDomains),
    AnteDomains = [_|_],
    ConsequentDomains = [_|_],
    !,
    append(AnteDomains, ConsequentDomains, Domains).
condition_nested_domains(Condition, Domains) :-
    has_functor(Condition, v, 2),
    arg(1, Condition, Left),
    arg(2, Condition, Right),
    nested_drs_domains(Left, LeftDomains),
    nested_drs_domains(Right, RightDomains),
    LeftDomains = [_|_],
    RightDomains = [_|_],
    !,
    append(LeftDomains, RightDomains, Domains).
condition_nested_domains(Condition, Domains) :-
    has_functor(Condition, question, 1),
    arg(1, Condition, Drs),
    nested_drs_domains(Drs, Domains),
    Domains = [_|_],
    !.
condition_nested_domains(Condition, Domains) :-
    unary_negation_argument(Condition, Argument),
    !,
    ( nested_drs_domains(Argument, Direct), Direct = [_|_] ->
        Domains = Direct
    ; condition_nested_domains(Argument, Domains)
    ).
condition_nested_domains(_, []).

nested_drs_domains(Drs, Domains) :-
    ( has_functor(Drs, drs, 2) ->
        arg(1, Drs, Domain),
        arg(2, Drs, Conditions),
        ( is_list(Domain), is_list(Conditions) ->
            nested_condition_domains(Conditions, Nested),
            Domains = [Domain|Nested]
        ; Domains = []
        )
    ; Domains = []
    ).

nested_condition_domains([], []).
nested_condition_domains([Condition|Conditions], Domains) :-
    condition_nested_domains(Condition, Here),
    nested_condition_domains(Conditions, Rest),
    append(Here, Rest, Domains).

add_fresh_domains([], Seen, Seen).
add_fresh_domains([Domain|Domains], Seen0, Seen) :-
    require_fresh_domain_refs(Domain, Seen0),
    append(Domain, Seen0, Seen1),
    add_fresh_domains(Domains, Seen1, Seen).

require_fresh_domain_refs([], _).
require_fresh_domain_refs([Referent|Referents], Seen) :-
    ( ref_member(Referent, Seen) ->
        reject(referent, redeclared_domain_referent)
    ; true
    ),
    require_fresh_domain_refs(Referents, Seen).

/* Root factual conditions are emitted in DRS order. Copula pairs consume two. */
index_conditions([], _, []).
index_conditions([Condition|Conditions], Position,
        [indexed(Position, Condition)|Indexed]) :-
    Next is Position + 1,
    index_conditions(Conditions, Next, Indexed).

lower_root_items([], _, _, _, Counters, Counters, [], [], []).
lower_root_items([indexed(Position, Condition)|Conditions], All, Domain,
        Consumed, Counters0, Counters, Items, Events, Entities) :-
    ( memberchk(Position, Consumed) ->
        lower_root_items(Conditions, All, Domain, Consumed,
            Counters0, Counters, Items, Events, Entities)
    ; lower_root_condition_drafts(Position, Condition, All, Domain,
          Consumed, Consumed1, Drafts, HereEvents, HereEntities),
      finalize_draft_group(Drafts, Counters0, Counters1, HereItems),
      lower_root_items(Conditions, All, Domain, Consumed1,
          Counters1, Counters, RestItems, RestEvents, RestEntities),
      append(HereItems, RestItems, Items),
      append(HereEvents, RestEvents, Events),
      append(HereEntities, RestEntities, Entities)
    ).

lower_root_condition_drafts(Position, Condition, _All, _Domain, Consumed,
        Consumed1, Drafts, [], []) :-
    has_functor(Condition, '=>', 2),
    !,
    lower_rule(Position, Condition, Drafts),
    Consumed1 = [Position|Consumed].
lower_root_condition_drafts(Position, Condition, _All, _Domain, Consumed,
        Consumed1, [Draft], [], []) :-
    has_functor(Condition, v, 2),
    !,
    lower_root_alternative_set(Position, Condition, Draft),
    Consumed1 = [Position|Consumed].
lower_root_condition_drafts(Position, Condition, All, Domain, Consumed,
        Consumed1, [Draft], Events, Entities) :-
    lower_root_condition(Position, Condition, All, Domain, Consumed,
        Consumed1, Draft, Events, Entities).

lower_root_condition(Position, Condition, All, Domain, Consumed,
        Consumed1, Draft, Events, Entities) :-
    temporal_window_profile(Position, Condition, All, Window),
    !,
    lower_root_temporal_window(Position, Window, All, Domain,
        Consumed, Consumed1, Draft, Events, Entities).
lower_root_condition(Position, Condition, All, Domain, Consumed,
        Consumed1, Draft, Events, Entities) :-
    anchored_condition(Condition, Inner, Anchor),
    !,
    ( contains_negation(Inner) ->
        reject(negation, root_condition(Position))
    ; has_functor(Inner, query, 2) ->
        reject(wh_query, root_condition(Position))
    ; unit_bound_object_parts(Inner, _, _, _, _, _) ->
        lower_root_unit_bound_object(Position, Inner, Anchor, Domain,
            Consumed, Consumed1, Draft, Events, Entities)
    ; measurement_object_pair(Inner, All, _) ->
        lower_root_measurement(Position, Inner, Anchor, All, Domain,
            Consumed, Consumed1, Draft, Events, Entities)
    ; count_quantity_object_parts(Inner, _, _, _, _),
      root_count_quantity_pair(Inner, All, _) ->
        lower_root_count_quantity(Position, Inner, Anchor, All, Domain,
            Consumed, Consumed1, Draft, Events, Entities)
    ; functor_name(Inner, object) ->
        lower_copula_from_object(Position, Inner, Anchor, All, Domain,
            Consumed, Consumed1, Draft, Events, Entities)
    ; predicate_named_be(Inner) ->
        lower_copula_from_be(Position, Inner, Anchor, All, Domain,
            Consumed, Consumed1, Draft, Events, Entities)
    ; scalar_comparison_property(Inner, _, _) ->
        lower_root_scalar_comparison(Position, Inner, Anchor, All, Domain,
            Consumed, Consumed1, Draft, Events, Entities)
    ; predicate_event(Inner, ModifierEvent),
      event_has_modifier(ModifierEvent, All) ->
        lower_root_temporal(Position, Inner, Anchor, All, Domain,
            Consumed, Consumed1, Draft, Events, Entities)
    ; functor_name(Inner, property) ->
        lower_root_property(Position, Inner, Anchor, All, Domain,
            Consumed, Consumed1, Draft, Events, Entities)
    ; has_functor(Inner, predicate, 4) ->
        lower_root_transitive(Position, Inner, Anchor, All, Domain, Draft,
            Events, Entities),
        Consumed1 = [Position|Consumed]
    ; has_functor(Inner, predicate, 3) ->
        lower_root_predicate(Position, Inner, Anchor, Domain, Draft,
            Events, Entities),
        Consumed1 = [Position|Consumed]
    ; has_functor(Inner, relation, 3) ->
        lower_root_relation(Position, Inner, Anchor, All, Domain, Draft,
            Events, Entities),
        Consumed1 = [Position|Consumed]
    ; has_functor(Inner, '=>', 2) ->
        reject(unsupported, root_condition(Position, anchored_implication))
    ; unsupported_condition(root_condition(Position), Inner)
    ).
lower_root_condition(Position, Condition, _All, _Domain, _Consumed,
        _, _, _, _) :-
    ( contains_negation(Condition) ->
        reject(negation, root_condition(Position))
    ; contains_query2(Condition) ->
        reject(wh_query, root_condition(Position))
    ; unsupported_condition(root_condition(Position), Condition)
    ).

lower_root_alternative_set(Position, Disjunction, Draft) :-
    arg(1, Disjunction, LeftDrs),
    arg(2, Disjunction, RightDrs),
    lower_root_alternative_member(
        LeftDrs, Position, left, Left, LeftAnchors),
    lower_root_alternative_member(
        RightDrs, Position, right, Right, RightAnchors),
    Members = [Left, Right],
    require_distinct_alternative_members(root_condition(Position), Members),
    append(LeftAnchors, RightAnchors, Anchors),
    source_from_anchors(
        root_condition(Position), Anchors, Sentence, Tokens),
    Draft = draft_alternative_set(Members, [], Sentence, Tokens).

lower_root_alternative_member(Drs, Position, Side, Member, [Anchor]) :-
    ( has_functor(Drs, drs, 2) ->
        arg(1, Drs, Domain),
        arg(2, Drs, Conditions)
    ; reject(alternative_set,
          root_condition(Position, branch(Side, drs_shape)))
    ),
    ( is_list(Domain), is_list(Conditions) -> true
    ; reject(alternative_set,
          root_condition(Position, branch(Side, drs_lists)))
    ),
    validate_domain(alternative_root(Position, Side), Domain),
    ( Conditions = [Condition],
      anchored_condition(Condition, Predicate, Anchor),
      predicate4_parts(Predicate, Event, Verb, Subject, Object) ->
        require_non_be_verb(root_condition(Position), Verb),
        require_erasable_event(
            alternative_root(Position, Side), Event, Domain, Conditions),
        require_alternative_ground_argument(
            Position, Side, 1, Subject, SubjectArg),
        require_alternative_ground_argument(
            Position, Side, 2, Object, ObjectArg),
        validate_scope_accounting(
            alternative_root(Position, Side), Domain, [Event], []),
        Member = pred(Verb, [SubjectArg, ObjectArg])
    ; reject(alternative_set,
          root_condition(Position, branch(Side, profile)))
    ).

require_alternative_ground_argument(_, _, _, Term, named(Name)) :-
    named_atom(Term, Name),
    !.
require_alternative_ground_argument(Position, Side, Index, _, _) :-
    reject(alternative_set,
        root_condition(Position, branch(Side, argument(Index)))).

require_distinct_alternative_members(Location, Members) :-
    ( first_duplicate_term(Members, _) ->
        reject(alternative_set, Location-duplicate_member)
    ; true
    ).

first_duplicate_term([Term|Terms], Term) :-
    term_member_eq(Term, Terms),
    !.
first_duplicate_term([_|Terms], Duplicate) :-
    first_duplicate_term(Terms, Duplicate).

term_member_eq(Term, [Member|_]) :-
    Term == Member,
    !.
term_member_eq(Term, [_|Members]) :-
    term_member_eq(Term, Members).

predicate_event(Predicate, Event) :-
    compound(Predicate),
    functor(Predicate, predicate, Arity),
    ( Arity =:= 3 ; Arity =:= 4 ),
    arg(1, Predicate, Event).

event_has_modifier(Event, Conditions) :-
    event_modifier_entries(Event, Conditions, [_|_]).

event_modifier_entries(_, [], []).
event_modifier_entries(Event,
        [indexed(Position, Condition)|Conditions], Entries) :-
    ( anchored_condition(Condition, Modifier, Anchor),
      has_functor(Modifier, modifier_pp, 3),
      arg(1, Modifier, StoredEvent),
      StoredEvent == Event ->
        arg(2, Modifier, Relation),
        arg(3, Modifier, Argument),
        Entries = [modifier_entry(
            Position, Relation, Argument, Anchor)|Rest]
    ; Entries = Rest
    ),
    event_modifier_entries(Event, Conditions, Rest).

admitted_temporal_relation(Relation) :-
    ( Relation == before
    ; Relation == after
    ; Relation == during
    ; Relation == within
    ).

lower_root_temporal(Position, Predicate, PredicateAnchor, All, Domain,
        Consumed0, Consumed, Draft, [Event], Entities) :-
    predicate_event(Predicate, Event),
    event_modifier_entries(Event, All, Modifiers),
    ( Modifiers = [modifier_entry(
          ModifierPosition, Relation, AnchorTerm, ModifierAnchor)],
      admitted_temporal_relation(Relation),
      named_atom(AnchorTerm, AnchorName) ->
        true
    ; reject(temporal, root_condition(Position, modifier_profile))
    ),
    require_event_occurrences(
        root_condition(Position), Event, All, 2),
    root_temporal_event_atom(
        Position, Predicate, Domain, EventAtom, Entities),
    source_from_anchors(root_condition(Position),
        [PredicateAnchor, ModifierAnchor], Sentence, Tokens),
    Temporal = temporal(Relation, EventAtom, anchor(named(AnchorName))),
    Draft = draft_fact(Temporal, Sentence, Tokens),
    Consumed = [Position, ModifierPosition|Consumed0].

root_temporal_event_atom(Position, Predicate, Domain,
        pred(Verb, [Arg]), []) :-
    predicate3_parts(Predicate, Event, Verb, Subject),
    !,
    require_non_be_verb(root_condition(Position), Verb),
    require_local_event(root_condition(Position), Event, Domain),
    require_ground_subject(root_condition(Position), Subject, Domain, Arg).
root_temporal_event_atom(Position, Predicate, Domain,
        pred(Verb, [SubjectArg, ObjectArg]), []) :-
    predicate4_parts(Predicate, Event, Verb, Subject, Object),
    require_non_be_verb(root_condition(Position), Verb),
    require_local_event(root_condition(Position), Event, Domain),
    require_ground_argument(
        root_condition(Position), 1, Subject, Domain, SubjectArg),
    require_ground_argument(
        root_condition(Position), 2, Object, Domain, ObjectArg).

require_event_occurrences(Location, Event, Conditions, Expected) :-
    ref_occurrence_count(Event, Conditions, Count),
    ( Count =:= Expected ->
        true
    ; reject(temporal, Location-event_in_use)
    ).

temporal_window_profile(Position, Condition, All,
        window(Event, Predicate, Direction, AnchorName,
            Range, UpperReferent, Lower, Upper, Unit,
            Positions, SourceAnchors)) :-
    anchored_condition(Condition, LowerPart, LowerPartAnchor),
    has_functor(LowerPart, has_part, 2),
    arg(1, LowerPart, Range),
    arg(2, LowerPart, LowerTerm),
    has_functor(LowerTerm, int, 1),
    arg(1, LowerTerm, Lower),
    integer(Lower),
    window_between_entry(Range, All,
        BetweenPosition, Event, BetweenAnchor),
    window_direction_entry(Event, All,
        DirectionPosition, Direction, AnchorName, DirectionAnchor),
    window_predicate_entry(Event, All,
        PredicatePosition, Predicate, PredicateAnchor),
    window_upper_part_entry(Range, All,
        UpperPartPosition, UpperReferent, UpperPartAnchor),
    window_upper_object_entry(UpperReferent, All,
        UpperObjectPosition, Unit, Upper, UpperAnchor),
    window_range_object_entry(
        Range, All, RangeObjectPosition, RangeObjectAnchor),
    require_window_order(Lower, Upper),
    require_exact_ref_occurrences(
        temporal_window, Event, All, 3),
    require_exact_ref_occurrences(
        temporal_window, Range, All, 4),
    require_exact_ref_occurrences(
        temporal_window, UpperReferent, All, 2),
    Positions = [Position, PredicatePosition, DirectionPosition,
        BetweenPosition, UpperObjectPosition, UpperPartPosition,
        RangeObjectPosition],
    require_distinct_positions(Positions),
    AllSourceAnchors = [LowerPartAnchor, PredicateAnchor,
        DirectionAnchor, BetweenAnchor, UpperAnchor,
        UpperPartAnchor, RangeObjectAnchor],
    window_profile_source_anchors(
        Position, AllSourceAnchors, SourceAnchors).

temporal_window_profile(_, _, _, _) :-
    fail.

window_between_entry(Range,
        [indexed(Position, Condition)|_], Position, Event, Anchor) :-
    anchored_condition(Condition, Modifier, Anchor),
    has_functor(Modifier, modifier_pp, 3),
    arg(1, Modifier, Event),
    arg(2, Modifier, Relation),
    arg(3, Modifier, StoredRange),
    Relation == between,
    StoredRange == Range,
    !.
window_between_entry(Range, [_|Conditions], Position, Event, Anchor) :-
    window_between_entry(Range, Conditions, Position, Event, Anchor).

window_direction_entry(Event,
        [indexed(Position, Condition)|_], Position,
        Direction, AnchorName, Anchor) :-
    anchored_condition(Condition, Modifier, Anchor),
    has_functor(Modifier, modifier_pp, 3),
    arg(1, Modifier, StoredEvent),
    arg(2, Modifier, Direction),
    arg(3, Modifier, AnchorTerm),
    StoredEvent == Event,
    ( Direction == after ; Direction == before ),
    named_atom(AnchorTerm, AnchorName),
    !.
window_direction_entry(Event, [_|Conditions], Position,
        Direction, AnchorName, Anchor) :-
    window_direction_entry(Event, Conditions, Position,
        Direction, AnchorName, Anchor).

window_predicate_entry(Event,
        [indexed(Position, Condition)|_], Position, Predicate, Anchor) :-
    anchored_condition(Condition, Predicate, Anchor),
    predicate_event(Predicate, StoredEvent),
    StoredEvent == Event,
    !.
window_predicate_entry(Event, [_|Conditions], Position, Predicate, Anchor) :-
    window_predicate_entry(Event, Conditions, Position, Predicate, Anchor).

window_upper_part_entry(Range,
        [indexed(Position, Condition)|_], Position, UpperReferent, Anchor) :-
    anchored_condition(Condition, Part, Anchor),
    has_functor(Part, has_part, 2),
    arg(1, Part, StoredRange),
    arg(2, Part, UpperReferent),
    StoredRange == Range,
    var(UpperReferent),
    !.
window_upper_part_entry(Range, [_|Conditions], Position,
        UpperReferent, Anchor) :-
    window_upper_part_entry(
        Range, Conditions, Position, UpperReferent, Anchor).

window_upper_object_entry(UpperReferent,
        [indexed(Position, Condition)|_], Position,
        Unit, Upper, Anchor) :-
    anchored_condition(Condition, Object, Anchor),
    has_functor(Object, object, 6),
    arg(1, Object, StoredReferent),
    arg(2, Object, Unit),
    arg(3, Object, Countability),
    arg(4, Object, Definiteness),
    arg(5, Object, Relation),
    arg(6, Object, Upper),
    StoredReferent == UpperReferent,
    atom(Unit),
    Countability == countable,
    Definiteness == na,
    Relation == eq,
    integer(Upper),
    !.
window_upper_object_entry(UpperReferent, [_|Conditions], Position,
        Unit, Upper, Anchor) :-
    window_upper_object_entry(UpperReferent, Conditions,
        Position, Unit, Upper, Anchor).

window_range_object_entry(Range,
        [indexed(Position, Condition)|_], Position, Anchor) :-
    anchored_condition(Condition, Object, Anchor),
    has_functor(Object, object, 6),
    arg(1, Object, StoredRange),
    arg(2, Object, Class),
    arg(3, Object, Countability),
    arg(4, Object, Definiteness),
    arg(5, Object, Relation),
    arg(6, Object, Count),
    StoredRange == Range,
    Class == na,
    Countability == countable,
    Definiteness == na,
    Relation == eq,
    Count == 2,
    !.
window_range_object_entry(Range, [_|Conditions], Position, Anchor) :-
    window_range_object_entry(Range, Conditions, Position, Anchor).

require_window_order(Lower, Upper) :-
    ( Lower =< Upper ->
        true
    ; reject(temporal, reversed_window(Lower, Upper))
    ).

require_exact_ref_occurrences(Location, Referent, Conditions, Expected) :-
    ref_occurrence_count(Referent, Conditions, Count),
    ( Count =:= Expected ->
        true
    ; reject(temporal, Location-referent_in_use)
    ).

require_distinct_positions(Positions) :-
    ( first_duplicate_term(Positions, _) ->
        reject(temporal, window_condition_reuse)
    ; true
    ).

window_profile_source_anchors(Position, [First|Anchors], SourceAnchors) :-
    window_anchor_parts(Position, First, Sentence, FirstKeep),
    window_profile_source_anchors_(
        Anchors, Position, Sentence, RestKeep),
    append(FirstKeep, RestKeep, SourceAnchors).

window_profile_source_anchors_([], _, _, []).
window_profile_source_anchors_([Anchor|Anchors], Position, Sentence,
        SourceAnchors) :-
    window_anchor_parts(Position, Anchor, HereSentence, HereKeep),
    ( HereSentence =:= Sentence ->
        true
    ; reject(temporal,
          window_condition(Position, mixed_sentence_anchors))
    ),
    window_profile_source_anchors_(
        Anchors, Position, Sentence, RestKeep),
    append(HereKeep, RestKeep, SourceAnchors).

window_anchor_parts(Position, Anchor, Sentence, Keep) :-
    ( has_functor(Anchor, '/', 2) ->
        arg(1, Anchor, Sentence0),
        arg(2, Anchor, Token),
        ( integer(Sentence0), Sentence0 >= 1 ->
            Sentence = Sentence0
        ; reject(temporal,
              window_condition(Position, anchor_ordinals))
        ),
        ( integer(Token), Token >= 1 ->
            Keep = [Anchor]
        ; Token == '' ->
            Keep = []
        ; reject(temporal,
              window_condition(Position, anchor_ordinals))
        )
    ; reject(temporal, window_condition(Position, anchor_shape))
    ).

lower_root_temporal_window(Position,
        window(Event, Predicate, Direction, AnchorName,
            Range, UpperReferent, Lower, Upper, Unit,
            Positions, SourceAnchors), _All, Domain,
        Consumed0, Consumed, Draft, [Event], Entities) :-
    require_local_event(root_condition(Position), Event, Domain),
    require_declared_entity(root_condition(Position), Range, Domain),
    require_declared_entity(
        root_condition(Position), UpperReferent, Domain),
    root_temporal_event_atom(
        Position, Predicate, Domain, EventAtom, EventEntities),
    quantity_bound_from_relation(geq, Lower, Unit, LowerBound),
    quantity_bound_from_relation(leq, Upper, Unit, UpperBound),
    Window = temporal_window(Direction, EventAtom,
        anchor(named(AnchorName)), interval(LowerBound, UpperBound)),
    source_from_anchors(root_condition(Position), SourceAnchors,
        Sentence, Tokens),
    Draft = draft_fact(Window, Sentence, Tokens),
    append(Positions, Consumed0, Consumed),
    append([Range, UpperReferent], EventEntities, Entities).

measurement_object_pair(Object, All, BePosition) :-
    exact_object(Object, Referent, _),
    measurement_be_entry(Referent, All,
        BePosition, _, _, _).

measurement_be_entry(Referent,
        [indexed(Position, Condition)|_], Position,
        Event, Quantity, Anchor) :-
    anchored_condition(Condition, Be, Anchor),
    predicate4_parts(Be, Event, Verb, Subject, QuantityTerm),
    Verb == be,
    Subject == Referent,
    quantity_from_int(QuantityTerm, Quantity, _),
    !.
measurement_be_entry(Referent, [_|Conditions],
        Position, Event, Quantity, Anchor) :-
    measurement_be_entry(Referent, Conditions,
        Position, Event, Quantity, Anchor).

lower_root_measurement(Position, Object, ObjectAnchor, All, Domain,
        Consumed0, Consumed, Draft, [Event], [Referent]) :-
    exact_object(Object, Referent, Class),
    require_lemma(root_condition(Position), object_class, Class),
    require_declared_entity(root_condition(Position), Referent, Domain),
    measurement_be_entry(Referent, All,
        BePosition, Event, Quantity, BeAnchor),
    require_erasable_event(root_condition(Position), Event, Domain, All),
    source_from_anchors(root_condition(Position),
        [ObjectAnchor, BeAnchor], Sentence, Tokens),
    Draft = draft_fact(pred(Class, [Quantity]), Sentence, Tokens),
    Consumed = [Position, BePosition|Consumed0].

lower_root_unit_bound_object(Position, Object, Anchor, Domain,
        Consumed0, [Position|Consumed0], Draft, [], [Referent]) :-
    unit_bound_object_parts(
        Object, Referent, Class, Unit, Relation, Value),
    require_declared_entity(root_condition(Position), Referent, Domain),
    require_lemma(root_condition(Position), object_class, Class),
    quantity_bound_from_relation(Relation, Value, Unit, Bound),
    source_from_anchors(root_condition(Position), [Anchor],
        Sentence, Tokens),
    Draft = draft_fact(pred(Class, [Bound]), Sentence, Tokens).

root_count_quantity_pair(Object, All, PredicatePosition) :-
    count_quantity_object_parts(Object, Referent, _, _, _),
    quantity_transitive_entry(Referent, All,
        PredicatePosition, _, _, _, _).

lower_root_count_quantity(Position, Object, ObjectAnchor, All, Domain,
        Consumed0, Consumed, Draft, [Event], [Referent]) :-
    count_quantity_object_parts(
        Object, Referent, Unit, Relation, Value),
    require_declared_entity(root_condition(Position), Referent, Domain),
    quantity_transitive_entry(Referent, All,
        PredicatePosition, Event, Verb, Subject, PredicateAnchor),
    require_non_be_verb(root_condition(Position), Verb),
    require_erasable_event(root_condition(Position), Event, Domain, All),
    require_ground_argument(
        root_condition(Position), 1, Subject, Domain, SubjectArg),
    ( Relation == eq ->
        QuantityArg = quantity(integer(Value), unit(Unit))
    ; quantity_bound_from_relation(Relation, Value, Unit, QuantityArg)
    ),
    source_from_anchors(root_condition(Position),
        [ObjectAnchor, PredicateAnchor], Sentence, Tokens),
    Draft = draft_fact(
        pred(Verb, [SubjectArg, QuantityArg]), Sentence, Tokens),
    Consumed = [Position, PredicatePosition|Consumed0].

quantity_transitive_entry(Referent,
        [indexed(Position, Condition)|_], Position,
        Event, Verb, Subject, Anchor) :-
    anchored_condition(Condition, Predicate, Anchor),
    predicate4_parts(Predicate, Event, Verb, Subject, Object),
    Object == Referent,
    Verb \== be,
    !.
quantity_transitive_entry(Referent, [_|Conditions], Position,
        Event, Verb, Subject, Anchor) :-
    quantity_transitive_entry(Referent, Conditions, Position,
        Event, Verb, Subject, Anchor).

lower_copula_from_object(Position, Object, ObjectAnchor, All, Domain,
        Consumed, Consumed1, Draft, [Event], [Referent]) :-
    require_root_object(Position, Object, Referent, Class),
    require_declared_entity(root_condition(Position), Referent, Domain),
    matching_be_entries(Referent, All, Consumed, Matches),
    require_one_be(Position, Matches, BePosition, Event, Name, BeAnchor),
    require_local_event(root_condition(Position), Event, Domain),
    source_from_anchors(root_condition(Position), [ObjectAnchor, BeAnchor],
        Sentence, Tokens),
    Draft = draft_fact(pred(Class, [named(Name)]), Sentence, Tokens),
    Consumed1 = [Position, BePosition|Consumed].

lower_copula_from_be(Position, Be, BeAnchor, All, Domain, Consumed,
        Consumed1, Draft, [Event], [Referent]) :-
    require_root_be(Position, Be, Event, Name, Referent),
    require_local_event(root_condition(Position), Event, Domain),
    require_declared_entity(root_condition(Position), Referent, Domain),
    matching_object_entries(Referent, All, Consumed, Matches),
    require_one_object(Position, Matches, ObjectPosition, Class,
        ObjectAnchor),
    source_from_anchors(root_condition(Position), [BeAnchor, ObjectAnchor],
        Sentence, Tokens),
    Draft = draft_fact(pred(Class, [named(Name)]), Sentence, Tokens),
    Consumed1 = [Position, ObjectPosition|Consumed].

require_root_object(Position, Object, Referent, Class) :-
    ( exact_object(Object, Referent0, Class0) ->
        require_lemma(root_condition(Position), object_class, Class0),
        Referent = Referent0,
        Class = Class0
    ; reject(copula, root_condition(Position, object_fields))
    ).

exact_object(Object, Referent, Class) :-
    has_functor(Object, object, 6),
    arg(1, Object, Referent),
    arg(2, Object, Class),
    arg(3, Object, Countability),
    arg(4, Object, Definiteness),
    arg(5, Object, Relation),
    arg(6, Object, Quantity),
    var(Referent),
    atom(Class),
    Countability == countable,
    Definiteness == na,
    Relation == eq,
    Quantity == 1.

require_root_be(Position, Be, Event, Name, Referent) :-
    ( exact_be(Be, Event0, Name0, Referent0) ->
        Event = Event0,
        Name = Name0,
        Referent = Referent0
    ; reject(copula, root_condition(Position, be_shape))
    ).

exact_be(Be, Event, Name, Referent) :-
    has_functor(Be, predicate, 4),
    arg(1, Be, Event),
    arg(2, Be, Verb),
    arg(3, Be, Subject),
    arg(4, Be, Referent),
    Verb == be,
    var(Event),
    named_atom(Subject, Name),
    var(Referent).

named_atom(Term, Name) :-
    has_functor(Term, named, 1),
    arg(1, Term, Name),
    atom(Name).

matching_be_entries(_, [], _, []).
matching_be_entries(Referent, [indexed(Position, Condition)|Conditions],
        Consumed, Matches) :-
    ( \+ memberchk(Position, Consumed),
      anchored_condition(Condition, Inner, Anchor),
      has_functor(Inner, predicate, 4),
      arg(2, Inner, Verb),
      Verb == be,
      arg(4, Inner, ObjectReferent),
      ObjectReferent == Referent ->
        arg(1, Inner, Event),
        arg(3, Inner, Subject),
        Matches = [be_match(Position, Event, Subject, Anchor)|Rest]
    ; Matches = Rest
    ),
    matching_be_entries(Referent, Conditions, Consumed, Rest).

matching_object_entries(_, [], _, []).
matching_object_entries(Referent,
        [indexed(Position, Condition)|Conditions], Consumed, Matches) :-
    ( \+ memberchk(Position, Consumed),
      anchored_condition(Condition, Inner, Anchor),
      functor_name(Inner, object),
      compound(Inner),
      arg(1, Inner, ObjectReferent),
      ObjectReferent == Referent ->
        Matches = [object_match(Position, Inner, Anchor)|Rest]
    ; Matches = Rest
    ),
    matching_object_entries(Referent, Conditions, Consumed, Rest).

require_one_be(Position, [], _, _, _, _) :-
    reject(copula, root_condition(Position, unpaired_object)).
require_one_be(Position, [be_match(BePosition, Event, Subject, Anchor)],
        BePosition, Event, Name, Anchor) :-
    !,
    ( var(Event), named_atom(Subject, Name0) ->
        Name = Name0
    ; reject(copula, root_condition(Position, be_shape))
    ).
require_one_be(Position, [_,_|_], _, _, _, _) :-
    reject(copula, root_condition(Position, multiple_be)).

require_one_object(Position, [], _, _, _) :-
    reject(copula, root_condition(Position, unpaired_be)).
require_one_object(Position,
        [object_match(ObjectPosition, Object, Anchor)], ObjectPosition,
        Class, Anchor) :-
    !,
    require_root_object(Position, Object, _Referent, Class).
require_one_object(Position, [_,_|_], _, _, _) :-
    reject(copula, root_condition(Position, multiple_object)).

lower_root_scalar_comparison(Position, Property, PropertyAnchor, All,
        Domain, Consumed0, Consumed, Draft, [Event], Entities) :-
    scalar_comparison_property(Property, Carrier, ComparisonTerm),
    require_declared_entity(root_condition(Position), Carrier, Domain),
    require_scalar_comparison_be(
        root_condition(Position), Carrier, All,
        BePosition, Event, ActualTerm, BeAnchor),
    require_erasable_event(root_condition(Position), Event, Domain, All),
    require_scalar_quantity_operand(
        root_condition(Position), ActualTerm, All, Actual, ActualUnit),
    scalar_root_operand(ComparisonTerm, All, Comparison,
        ComparisonUnit, OperandPositions, OperandAnchors, OperandEntities),
    ( ActualUnit == ComparisonUnit ->
        true
    ; reject(quantity,
          scalar_cross_unit(ActualUnit, ComparisonUnit))
    ),
    property_parts(root_condition(Position), Property, _, Name,
        [ComparisonTerm]),
    quantity_bound_from_relation(
        greater, ComparisonValue, ComparisonUnit, Bound),
    quantity_value(Comparison, ComparisonValue),
    append([PropertyAnchor, BeAnchor], OperandAnchors, Anchors),
    source_from_anchors(root_condition(Position), Anchors,
        Sentence, Tokens),
    Head = pred(Name, [Actual, Comparison]),
    Draft = draft_rule(Head,
        [quantity_compare(Actual, Bound)], Sentence, Tokens),
    append([Position, BePosition|Consumed0], OperandPositions, Consumed),
    Entities = [Carrier|OperandEntities].

require_scalar_comparison_be(Location, Carrier, Conditions,
        Position, Event, Actual, Anchor) :-
    scalar_comparison_be_entries(Carrier, Conditions, Entries),
    ( Entries = [scalar_be(Position0, Event0, Actual0, Anchor0)] ->
        Position = Position0,
        Event = Event0,
        Actual = Actual0,
        Anchor = Anchor0
    ; Entries == [] ->
        reject(quantity, Location-scalar_comparison_be(missing))
    ; reject(quantity, Location-scalar_comparison_be(multiple))
    ).

scalar_comparison_be_entries(_, [], []).
scalar_comparison_be_entries(Carrier,
        [indexed(Position, Condition)|Conditions], Entries) :-
    ( anchored_condition(Condition, Be, Anchor),
      predicate4_parts(Be, Event, Verb, Actual, Object),
      Verb == be,
      Object == Carrier ->
        Entries = [scalar_be(Position, Event, Actual, Anchor)|Rest]
    ; Entries = Rest
    ),
    scalar_comparison_be_entries(Carrier, Conditions, Rest).

require_scalar_quantity_operand(Location, Term, Conditions,
        Quantity, Unit) :-
    ( scalar_quantity_operand(Term, Conditions, Quantity0, Unit0) ->
        Quantity = Quantity0,
        Unit = Unit0
    ; reject(quantity, Location-scalar_comparison_actual_operand)
    ).

scalar_root_operand(Term, _, Quantity, Unit, [], [], []) :-
    quantity_from_int(Term, Quantity, Unit),
    !.
scalar_root_operand(Term, All, Quantity, Unit,
        [Position], [Anchor], [Term]) :-
    var(Term),
    scalar_object_entry(Term, All, Position, Unit, Value, Anchor),
    Quantity = quantity(integer(Value), unit(Unit)),
    !.
scalar_root_operand(_, _, _, _, _, _, _) :-
    reject(quantity, scalar_comparison_operand).

scalar_object_entry(Referent,
        [indexed(Position, Condition)|_], Position, Unit, Value, Anchor) :-
    anchored_condition(Condition, Object, Anchor),
    count_quantity_object_parts(Object, Stored, Unit, eq, Value),
    Stored == Referent,
    !.
scalar_object_entry(Referent, [_|Conditions],
        Position, Unit, Value, Anchor) :-
    scalar_object_entry(
        Referent, Conditions, Position, Unit, Value, Anchor).

quantity_value(Quantity, Value) :-
    arg(1, Quantity, IntegerTerm),
    arg(1, IntegerTerm, Value).

lower_root_property(Position, Property, PropertyAnchor, All, Domain,
        Consumed, Consumed1, Draft, Events, [Carrier]) :-
    property_parts(root_condition(Position), Property, Carrier, Name,
        ComparisonTerms),
    require_declared_entity(root_condition(Position), Carrier, Domain),
    root_property_be_matches(Carrier, All, Matches),
    require_one_root_property_be(Position, Matches, BePosition, Event,
        SubjectName, BeAnchor),
    require_erasable_event(root_condition(Position), Event, Domain, All),
    root_property_comparison_args(ComparisonTerms, Position, Domain, 2,
        ComparisonArgs),
    ( memberchk(BePosition, Consumed) ->
        Events = [],
        Consumed1 = [Position|Consumed]
    ; Events = [Event],
      Consumed1 = [Position, BePosition|Consumed]
    ),
    source_from_anchors(root_condition(Position),
        [PropertyAnchor, BeAnchor], Sentence, Tokens),
    Draft = draft_fact(
        pred(Name, [named(SubjectName)|ComparisonArgs]), Sentence, Tokens).

root_property_be_matches(_, [], []).
root_property_be_matches(Carrier,
        [indexed(Position, Condition)|Conditions], Matches) :-
    ( anchored_condition(Condition, Inner, Anchor),
      exact_be(Inner, Event, Name, ObjectReferent),
      ObjectReferent == Carrier ->
        Matches = [property_be_match(
            Position, Event, Name, Anchor)|Rest]
    ; Matches = Rest
    ),
    root_property_be_matches(Carrier, Conditions, Rest).

require_one_root_property_be(Position, [], _, _, _, _) :-
    reject(referent, root_condition(Position, property_subject(unbound))).
require_one_root_property_be(_,
        [property_be_match(BePosition, Event, Name, Anchor)],
        BePosition, Event, Name, Anchor) :-
    !.
require_one_root_property_be(Position, [_,_|_], _, _, _, _) :-
    reject(referent, root_condition(Position, property_subject(ambiguous))).

root_property_comparison_args([], _, _, _, []).
root_property_comparison_args([Term|Terms], Position, Domain, Index,
        [Arg|Args]) :-
    require_ground_argument(
        root_condition(Position), Index, Term, Domain, Arg),
    Next is Index + 1,
    root_property_comparison_args(Terms, Position, Domain, Next, Args).

lower_root_predicate(Position, Predicate, Anchor, Domain, Draft,
        [Event], []) :-
    predicate3_parts(Predicate, Event, Verb, Subject),
    require_non_be_verb(root_condition(Position), Verb),
    require_local_event(root_condition(Position), Event, Domain),
    require_ground_subject(root_condition(Position), Subject, Domain, Arg),
    source_from_anchors(root_condition(Position), [Anchor], Sentence, Tokens),
    Draft = draft_fact(pred(Verb, [Arg]), Sentence, Tokens).

predicate3_parts(Predicate, Event, Verb, Subject) :-
    has_functor(Predicate, predicate, 3),
    arg(1, Predicate, Event),
    arg(2, Predicate, Verb),
    arg(3, Predicate, Subject).

predicate4_parts(Predicate, Event, Verb, Subject, Object) :-
    has_functor(Predicate, predicate, 4),
    arg(1, Predicate, Event),
    arg(2, Predicate, Verb),
    arg(3, Predicate, Subject),
    arg(4, Predicate, Object).

relation3_parts(Relation, Left, Name, Right) :-
    has_functor(Relation, relation, 3),
    arg(1, Relation, Left),
    arg(2, Relation, Name),
    arg(3, Relation, Right).

property_parts(Location, Property, Referent, Name, ComparisonTerms) :-
    compound(Property),
    functor(Property, property, Arity),
    ( Arity =:= 3 ->
        arg(1, Property, Referent),
        arg(2, Property, Adjective),
        arg(3, Property, Degree),
        require_lemma(Location, adjective, Adjective),
        ( Degree == pos ->
            Name = Adjective,
            ComparisonTerms = []
        ; ( Degree == comp ; Degree == sup ),
          ( Adjective == low ; Adjective == high ) ->
            atom_concat(Adjective, ' ', Prefix),
            atom_concat(Prefix, Degree, Name),
            ComparisonTerms = []
        ; reject(unsupported, Location-property_degree(Degree))
        )
    ; Arity =:= 4 ->
        arg(1, Property, Referent),
        arg(2, Property, Adjective),
        arg(3, Property, Degree),
        arg(4, Property, Comparison),
        require_lemma(Location, adjective, Adjective),
        ( ( Degree == comp_than ; Degree == pos_as ) ->
            atom_concat(Adjective, ' ', Prefix),
            atom_concat(Prefix, Degree, Name),
            ComparisonTerms = [Comparison]
        ; reject(unsupported, Location-property_degree(Degree))
        )
    ; reject(unsupported, Location-property_arity(Arity))
    ).

require_non_be_verb(Location, Verb) :-
    require_lemma(Location, verb, Verb),
    ( Verb \== be ->
        true
    ; reject(unsupported, Location-predicate_name)
    ).

require_lemma(Location, Kind, Lemma) :-
    ( atom(Lemma) ->
        ( sub_atom(Lemma, _, 1, _, ' ') ->
            reject(unsupported, Location-lemma_space(Kind))
        ; true
        )
    ; reject(unsupported, Location-lemma_shape(Kind))
    ).

lower_root_transitive(Position, Predicate, Anchor, All, Domain, Draft,
        [Event], []) :-
    predicate4_parts(Predicate, Event, Verb, Subject, Object),
    require_non_be_verb(root_condition(Position), Verb),
    require_erasable_event(
        root_condition(Position), Event, Domain, All),
    require_ground_argument(
        root_condition(Position), 1, Subject, Domain, SubjectArg),
    require_ground_argument(
        root_condition(Position), 2, Object, Domain, ObjectArg),
    source_from_anchors(root_condition(Position), [Anchor], Sentence, Tokens),
    Draft = draft_fact(
        pred(Verb, [SubjectArg, ObjectArg]), Sentence, Tokens).

lower_root_relation(Position, Relation, Anchor, All, Domain, Draft,
        [], Entities) :-
    require_of_relation(root_condition(Position), Relation, Left, Right),
    root_relation_arguments([Left, Right], Position, All, Domain, 1,
        Args, BindingAnchors, Entities),
    source_from_anchors(root_condition(Position), [Anchor|BindingAnchors],
        Sentence, Tokens),
    Draft = draft_fact(pred(of, Args), Sentence, Tokens).

require_of_relation(Location, Relation, Left, Right) :-
    relation3_parts(Relation, Left, Name, Right),
    require_lemma(Location, relation, Name),
    ( Name == of ->
        true
    ; reject(unsupported, Location-relation_name)
    ).

root_relation_arguments([], _, _, _, _, [], [], []).
root_relation_arguments([Term|Terms], Position, All, Domain, Index,
        [Arg|Args], Anchors, Entities) :-
    root_relation_argument(Term, Position, All, Domain, Index, Arg,
        HereAnchors, HereEntities),
    Next is Index + 1,
    root_relation_arguments(Terms, Position, All, Domain, Next,
        Args, RestAnchors, RestEntities),
    append(HereAnchors, RestAnchors, Anchors),
    append(HereEntities, RestEntities, Entities).

root_relation_argument(Term, _, _, _, _, named(Name), [], []) :-
    named_atom(Term, Name),
    !.
root_relation_argument(Term, Position, All, Domain, Index, named(Name),
        [Anchor], [Term]) :-
    var(Term),
    !,
    require_declared_entity(root_condition(Position), Term, Domain),
    root_named_binding(Term, Position, Index, All, Name, Anchor).
root_relation_argument(_, Position, _, _, Index, _, _, _) :-
    reject(unsupported,
        root_condition(Position, relation_argument(Index, shape))).

root_named_binding(Referent, Position, Index, All, Name, Anchor) :-
    root_named_binding_matches(Referent, All, Matches),
    ( Matches = [named_binding(Name0, Anchor0)] ->
        Name = Name0,
        Anchor = Anchor0
    ; Matches == [] ->
        reject(referent,
            root_condition(Position, relation_argument(Index, unbound)))
    ; reject(referent,
          root_condition(Position, relation_argument(Index, ambiguous)))
    ).

root_named_binding_matches(_, [], []).
root_named_binding_matches(Referent,
        [indexed(_, Condition)|Conditions], Matches) :-
    ( anchored_condition(Condition, Inner, Anchor),
      exact_be(Inner, _Event, Name, ObjectReferent),
      ObjectReferent == Referent ->
        Matches = [named_binding(Name, Anchor)|Rest]
    ; Matches = Rest
    ),
    root_named_binding_matches(Referent, Conditions, Rest).

require_ground_argument(_, _, Term, _, named(Name)) :-
    named_atom(Term, Name),
    !.
require_ground_argument(_, _, Term, _, Quantity) :-
    quantity_from_int(Term, Quantity, _),
    !.
require_ground_argument(Location, Index, Term, Domain, _) :-
    var(Term),
    !,
    ( ref_member(Term, Domain) ->
        reject(unsupported, Location-nonground_argument(Index))
    ; reject(referent, Location-undeclared_argument(Index))
    ).
require_ground_argument(Location, Index, _, _, _) :-
    reject(unsupported, Location-argument(Index)).

require_ground_subject(Location, Subject, Domain, Arg) :-
    ( named_atom(Subject, Name) ->
        Arg = named(Name)
    ; var(Subject) ->
        ( ref_member(Subject, Domain) ->
            reject(unsupported, Location-nonground_fact)
        ; reject(referent, Location-undeclared_subject)
        )
    ; reject(unsupported, Location-subject)
    ).

/* One implication becomes one rule per deterministic antecedent DNF branch. */
lower_rule(Position, Rule, Drafts) :-
    arg(1, Rule, Antecedent),
    arg(2, Rule, Consequent),
    require_nested_drs(rule(Position, antecedent), Antecedent,
        AnteDomain, AnteConditions),
    require_nested_drs(rule(Position, consequent), Consequent,
        ConsequentDomain, ConsequentConditions),
    validate_domain(antecedent(Position), AnteDomain),
    validate_domain(consequent(Position), ConsequentDomain),
    require_disjoint_domains(rule(Position), AnteDomain, ConsequentDomain),
    lower_rule_consequent(Position, ConsequentConditions, AnteDomain,
        ConsequentDomain, ConsequentSpec, Bindings1, Next1, HeadAnchors,
        ConsequentEvents, ConsequentEntities, HeadOuterRefs),
    expand_rule_antecedent(
        Position, AnteDomain, AnteConditions, Branches),
    length(Branches, BranchCount),
    lower_rule_branches(Branches, 1, BranchCount, Position,
        ConsequentDomain, ConsequentSpec, Bindings1, Next1, HeadAnchors,
        ConsequentEvents, ConsequentEntities, HeadOuterRefs, Drafts).

lower_rule_consequent(Position, Conditions, AnteDomain, ConsequentDomain,
        alternative(Members), Bindings, Next, Anchors, [], [], OuterRefs) :-
    Conditions = [Disjunction],
    has_functor(Disjunction, v, 2),
    !,
    ( ConsequentDomain == [] -> true
    ; reject(alternative_set,
          rule(Position, consequent_outer_domain))
    ),
    lower_rule_alternative_members(Disjunction, Position, AnteDomain,
        [], Bindings, 1, Next, Members, Anchors, OuterRefs),
    require_distinct_alternative_members(
        rule(Position, consequent), Members).
lower_rule_consequent(Position, Conditions, AnteDomain, ConsequentDomain,
        rule(Head), Bindings, Next, Anchors, Events, Entities, OuterRefs) :-
    lower_rule_head(Position, Conditions, AnteDomain, ConsequentDomain,
        [], Bindings, 1, Next, Head, Anchors, Events, Entities,
        OuterRefs).

lower_rule_alternative_members(Disjunction, Position, AnteDomain,
        Bindings0, Bindings, Next0, Next, [Left, Right], Anchors,
        OuterRefs) :-
    arg(1, Disjunction, LeftDrs),
    arg(2, Disjunction, RightDrs),
    lower_rule_alternative_member(LeftDrs, Position, left, AnteDomain,
        Bindings0, Bindings1, Next0, Next1, Left, LeftAnchors,
        LeftRefs),
    lower_rule_alternative_member(RightDrs, Position, right, AnteDomain,
        Bindings1, Bindings, Next1, Next, Right, RightAnchors,
        RightRefs),
    append(LeftAnchors, RightAnchors, Anchors),
    append(LeftRefs, RightRefs, OuterRefs).

lower_rule_alternative_member(Drs, Position, Side, AnteDomain,
        Bindings0, Bindings, Next0, Next, Member, [Anchor], OuterRefs) :-
    ( has_functor(Drs, drs, 2) ->
        arg(1, Drs, Domain),
        arg(2, Drs, Conditions)
    ; reject(alternative_set,
          rule(Position, consequent_branch(Side, drs_shape)))
    ),
    ( is_list(Domain), is_list(Conditions) -> true
    ; reject(alternative_set,
          rule(Position, consequent_branch(Side, drs_lists)))
    ),
    validate_domain(alternative_consequent(Position, Side), Domain),
    require_disjoint_domains(
        alternative_consequent(Position, Side), AnteDomain, Domain),
    ( Conditions = [Condition],
      anchored_condition(Condition, Predicate, Anchor),
      predicate4_parts(Predicate, Event, Verb, Subject, Object) ->
        require_non_be_verb(rule(Position, consequent), Verb),
        require_erasable_event(
            alternative_consequent(Position, Side), Event, Domain,
            Conditions),
        rule_head_arguments([Subject, Object], Position, AnteDomain,
            Domain, Bindings0, Bindings, Next0, Next, 1, Args,
            OuterRefs),
        validate_scope_accounting(
            alternative_consequent(Position, Side), Domain, [Event], []),
        Member = pred(Verb, Args)
    ; reject(alternative_set,
          rule(Position, consequent_branch(Side, profile)))
    ).

lower_rule_branches([], _, _, _, _, _, _, _, _, _, _, _, []).
lower_rule_branches([branch(Domain, Conditions)|Branches], Index,
        BranchCount, Position, ConsequentDomain, ConsequentSpec,
        Bindings0, Next0, HeadAnchors, ConsequentEvents,
        ConsequentEntities, HeadOuterRefs, [Draft|Drafts]) :-
    require_naf_suffix(Position, Conditions),
    require_alternative_positive_body(
        Position, ConsequentSpec, Conditions),
    lower_rule_body(Position, Conditions, Domain,
        Bindings0, _Bindings, Next0, _Next, Body, BodyAnchors,
        AnteEvents, BodyEntityRefs, BodyPositiveRefs),
    ( Body == [] ->
        reject(unsupported, rule(Position, empty_antecedent))
    ; true
    ),
    require_bound_head_refs(Position, HeadOuterRefs, BodyPositiveRefs),
    validate_scope_accounting(antecedent(Position), Domain,
        AnteEvents, BodyEntityRefs),
    ( ConsequentSpec = rule(_) ->
        validate_scope_accounting(consequent(Position), ConsequentDomain,
            ConsequentEvents, ConsequentEntities)
    ; true
    ),
    append(BodyAnchors, HeadAnchors, Anchors),
    source_from_anchors(rule(Position), Anchors, Sentence, Tokens),
    make_rule_draft(ConsequentSpec, BranchCount, Index, Body,
        Sentence, Tokens, Draft),
    NextIndex is Index + 1,
    lower_rule_branches(Branches, NextIndex, BranchCount, Position,
        ConsequentDomain, ConsequentSpec, Bindings0, Next0, HeadAnchors,
        ConsequentEvents, ConsequentEntities, HeadOuterRefs, Drafts).

require_alternative_positive_body(Position, alternative(_), Conditions) :-
    !,
    ( contains_tilde(Conditions) ->
        reject(alternative_set, rule(Position, alternative_body_naf))
    ; true
    ).
require_alternative_positive_body(_, _, _).

make_rule_draft(rule(Head), 1, _, Body, Sentence, Tokens,
        draft_rule(Head, Body, Sentence, Tokens)).
make_rule_draft(rule(Head), Count, Index, Body, Sentence, Tokens,
        draft_rule_branch(Index, Head, Body, Sentence, Tokens)) :-
    Count > 1.
make_rule_draft(alternative(Members), 1, _, Body, Sentence, Tokens,
        draft_alternative_set(Members, Body, Sentence, Tokens)).
make_rule_draft(alternative(Members), Count, Index, Body, Sentence, Tokens,
        draft_alternative_set_branch(
            Index, Members, Body, Sentence, Tokens)) :-
    Count > 1.

dnf_branch_cap(64).

expand_rule_antecedent(Position, OuterDomain, Conditions, Branches) :-
    dnf_branch_cap(Cap),
    Limit is Cap + 1,
    dnf_count_conditions(Position, Conditions, Limit, Count),
    ( Count > Cap ->
        reject(disjunction,
            rule(Position, antecedent_branch_cap_exceeded(Cap)))
    ; dnf_expand_conditions(Position, Conditions, Expanded),
      add_outer_domain(Expanded, OuterDomain, Branches)
    ).

dnf_count_conditions(_, [], _, 1).
dnf_count_conditions(Position, [Condition|Conditions], Limit, Count) :-
    dnf_count_condition(Position, Condition, Limit, Here),
    dnf_count_conditions(Position, Conditions, Limit, Rest),
    saturating_multiply(Here, Rest, Limit, Count).

dnf_count_condition(Position, Condition, Limit, Count) :-
    ( has_functor(Condition, v, 2) ->
        require_dnf_disjunction(Position, Condition,
            LeftDomain, LeftConditions, RightDomain, RightConditions),
        validate_domain(dnf_branch(Position, left), LeftDomain),
        validate_domain(dnf_branch(Position, right), RightDomain),
        dnf_count_conditions(Position, LeftConditions, Limit, LeftCount),
        dnf_count_conditions(Position, RightConditions, Limit, RightCount),
        saturating_add_count(LeftCount, RightCount, Limit, Count)
    ; has_functor(Condition, '~', 1),
      contains_v(Condition) ->
        reject(disjunction,
            rule(Position, v_under_naf))
    ; contains_v(Condition) ->
        reject(disjunction,
            rule(Position, antecedent_malformed_nested_v))
    ; Count = 1
    ).

require_dnf_disjunction(Position, Disjunction, LeftDomain, LeftConditions,
        RightDomain, RightConditions) :-
    arg(1, Disjunction, Left),
    arg(2, Disjunction, Right),
    require_dnf_branch_drs(Position, left, Left,
        LeftDomain, LeftConditions),
    require_dnf_branch_drs(Position, right, Right,
        RightDomain, RightConditions),
    ( contains_tilde(LeftConditions) ->
        reject(disjunction,
            rule(Position, naf_inside_disjunct(left)))
    ; contains_tilde(RightConditions) ->
        reject(disjunction,
            rule(Position, naf_inside_disjunct(right)))
    ; true
    ).

require_dnf_branch_drs(Position, Side, Drs, Domain, Conditions) :-
    ( has_functor(Drs, drs, 2) ->
        arg(1, Drs, Domain0),
        arg(2, Drs, Conditions0),
        ( is_list(Domain0), is_list(Conditions0) ->
            ( Conditions0 = [_|_] ->
                Domain = Domain0,
                Conditions = Conditions0
            ; reject(disjunction,
                  rule(Position, empty_disjunct(Side)))
            )
        ; reject(disjunction,
              rule(Position, disjunct_lists(Side)))
        )
    ; reject(disjunction,
          rule(Position, disjunct_shape(Side)))
    ).

dnf_expand_conditions(_, [], [dnf([], [])]).
dnf_expand_conditions(Position, [Condition|Conditions], Expanded) :-
    dnf_expand_condition(Position, Condition, Here),
    dnf_expand_conditions(Position, Conditions, Rest),
    dnf_product(Here, Rest, Expanded).

dnf_expand_condition(Position, Condition, Expanded) :-
    has_functor(Condition, v, 2),
    !,
    require_dnf_disjunction(Position, Condition,
        LeftDomain, LeftConditions, RightDomain, RightConditions),
    dnf_expand_conditions(Position, LeftConditions, Left0),
    dnf_expand_conditions(Position, RightConditions, Right0),
    prepend_dnf_domain(Left0, LeftDomain, Left),
    prepend_dnf_domain(Right0, RightDomain, Right),
    append(Left, Right, Expanded).
dnf_expand_condition(_, Condition, [dnf([], [Condition])]).

dnf_product([], _, []).
dnf_product([dnf(LeftDomain, LeftConditions)|Left], Right, Product) :-
    dnf_product_row(LeftDomain, LeftConditions, Right, Row),
    dnf_product(Left, Right, Rest),
    append(Row, Rest, Product).

dnf_product_row(_, _, [], []).
dnf_product_row(LeftDomain, LeftConditions,
        [dnf(RightDomain, RightConditions)|Right],
        [dnf(Domain, Conditions)|Rows]) :-
    append(LeftDomain, RightDomain, Domain),
    append(LeftConditions, RightConditions, Conditions),
    dnf_product_row(LeftDomain, LeftConditions, Right, Rows).

prepend_dnf_domain([], _, []).
prepend_dnf_domain([dnf(Domain0, Conditions)|Branches], Prefix,
        [dnf(Domain, Conditions)|Rest]) :-
    append(Prefix, Domain0, Domain),
    prepend_dnf_domain(Branches, Prefix, Rest).

add_outer_domain([], _, []).
add_outer_domain([dnf(ExtraDomain, Conditions)|Expanded], OuterDomain,
        [branch(Domain, Conditions)|Branches]) :-
    append(OuterDomain, ExtraDomain, Domain),
    add_outer_domain(Expanded, OuterDomain, Branches).

saturating_add_count(Left, Right, Limit, Count) :-
    ( Left >= Limit -> Count = Limit
    ; Right >= Limit -> Count = Limit
    ; Sum is Left + Right,
      ( Sum >= Limit -> Count = Limit ; Count = Sum )
    ).

saturating_multiply(Left, Right, Limit, Count) :-
    ( Left =:= 0 -> Count = 0
    ; Right =:= 0 -> Count = 0
    ; Left >= Limit -> Count = Limit
    ; Right >= Limit -> Count = Limit
    ; Left > Limit // Right -> Count = Limit
    ; Product is Left * Right,
      ( Product >= Limit -> Count = Limit ; Count = Product )
    ).

require_nested_drs(Location, Drs, Domain, Conditions) :-
    ( has_functor(Drs, drs, 2) ->
        arg(1, Drs, Domain0),
        arg(2, Drs, Conditions0),
        ( is_list(Domain0), is_list(Conditions0) ->
            Domain = Domain0,
            normalize_condition_list(Location, Conditions0, Conditions)
        ; reject(unsupported, Location-drs_lists)
        )
    ; reject(unsupported, Location-drs_shape)
    ).

require_naf_suffix(Position, Conditions) :-
    require_naf_suffix(Position, Conditions, 1, false).

require_naf_suffix(_, [], _, _).
require_naf_suffix(Position, [Condition|Conditions], Index, SeenNaf) :-
    ( has_functor(Condition, '~', 1) ->
        SeenNaf1 = true
    ; SeenNaf == true ->
        reject(negation,
            rule(Position, antecedent_condition(Index, positive_after_naf)))
    ; SeenNaf1 = false
    ),
    Next is Index + 1,
    require_naf_suffix(Position, Conditions, Next, SeenNaf1).

lower_rule_head(Position, Conditions, AnteDomain, ConsequentDomain,
        Bindings0, Bindings, Next0, Next, Head, Anchors,
        ConsequentEvents, ConsequentEntities, OuterRefs) :-
    ( contains_negation(Conditions) ->
        reject(negation, rule(Position, consequent))
    ; contains_query2(Conditions) ->
        reject(wh_query, rule(Position, consequent))
    ; true
    ),
    lower_rule_head_conditions(Position, Conditions, AnteDomain,
        ConsequentDomain, Bindings0, Bindings, Next0, Next, Head,
        Anchors, ConsequentEvents, ConsequentEntities, OuterRefs).

lower_rule_head_conditions(Position, Conditions, AnteDomain,
        ConsequentDomain, Bindings0, Bindings, Next0, Next, Head,
        [CarrierAnchor, BeAnchor], [Event], [Carrier], OuterRefs) :-
    Conditions = [CarrierCondition, BeCondition],
    anchored_condition(CarrierCondition, CarrierTerm, CarrierAnchor),
    copular_carrier_term(CarrierTerm),
    !,
    rule_copular_carrier(Position, CarrierTerm, Carrier,
        PredicateName, ComparisonTerms),
    ( anchored_condition(BeCondition, Be, BeAnchor) ->
        true
    ; reject(unsupported, rule(Position, consequent_copula))
    ),
    require_rule_copular_be(Position, Be, Carrier, Event, Subject),
    require_declared_entity(rule(Position, consequent),
        Carrier, ConsequentDomain),
    require_erasable_event(rule(Position, consequent), Event,
        ConsequentDomain, Conditions),
    rule_head_arguments([Subject|ComparisonTerms], Position, AnteDomain,
        ConsequentDomain, Bindings0, Bindings, Next0, Next, 1,
        Args, OuterRefs),
    Head = pred(PredicateName, Args).
lower_rule_head_conditions(Position, Conditions, AnteDomain,
        ConsequentDomain, Bindings0, Bindings, Next0, Next, Head,
        [Anchor], ConsequentEvents, [], OuterRefs) :-
    ( Conditions = [Condition] ->
        true
    ; length(Conditions, Count),
      reject(unsupported, rule(Position, consequent_count(Count)))
    ),
    ( anchored_condition(Condition, Inner, Anchor) ->
        true
    ; unsupported_condition(rule(Position, consequent), Condition)
    ),
    ( predicate_named_be(Inner) ->
        reject(unsupported, rule(Position, consequent_be))
    ; predicate3_parts(Inner, Event, Verb, Subject) ->
        require_non_be_verb(rule(Position, consequent), Verb),
        require_local_event(rule(Position, consequent), Event,
            ConsequentDomain),
        rule_head_subject(Position, Subject, AnteDomain, ConsequentDomain,
            Bindings0, Bindings, Next0, Next, Arg, OuterRefs),
        Head = pred(Verb, [Arg]),
        ConsequentEvents = [Event]
    ; predicate4_parts(Inner, Event, Verb, Subject, Object) ->
        require_non_be_verb(rule(Position, consequent), Verb),
        require_erasable_event(rule(Position, consequent), Event,
            ConsequentDomain, Conditions),
        rule_head_arguments([Subject, Object], Position, AnteDomain,
            ConsequentDomain, Bindings0, Bindings, Next0, Next, 1,
            Args, OuterRefs),
        Head = pred(Verb, Args),
        ConsequentEvents = [Event]
    ; has_functor(Inner, relation, 3) ->
        require_of_relation(rule(Position, consequent),
            Inner, Left, Right),
        rule_head_arguments([Left, Right], Position, AnteDomain,
            ConsequentDomain, Bindings0, Bindings, Next0, Next, 1,
            Args, OuterRefs),
        Head = pred(of, Args),
        ConsequentEvents = []
    ; functor_name(Inner, property) ->
        reject(unsupported, rule(Position, consequent_copula))
    ; functor_name(Inner, object) ->
        reject(unsupported, rule(Position, consequent_copula))
    ; unsupported_condition(rule(Position, consequent), Inner)
    ).

copular_carrier_term(Term) :-
    functor_name(Term, object),
    !.
copular_carrier_term(Term) :-
    functor_name(Term, property).

rule_copular_carrier(Position, CarrierTerm, Carrier,
        PredicateName, ComparisonTerms) :-
    ( functor_name(CarrierTerm, object) ->
        ( exact_object(CarrierTerm, Carrier0, Class) ->
            require_lemma(rule(Position, consequent), object_class, Class),
            Carrier = Carrier0,
            PredicateName = Class,
            ComparisonTerms = []
        ; reject(unsupported, rule(Position, consequent_object))
        )
    ; property_parts(rule(Position, consequent), CarrierTerm, Carrier,
          PredicateName, ComparisonTerms)
    ).

require_rule_copular_be(Position, Be, Carrier, Event, Subject) :-
    ( predicate4_parts(Be, Event0, Verb, Subject0, Object),
      Verb == be,
      var(Event0),
      Object == Carrier ->
        Event = Event0,
        Subject = Subject0
    ; reject(unsupported, rule(Position, consequent_copula))
    ).

rule_head_subject(Position, Subject, AnteDomain, ConsequentDomain,
        Bindings0, Bindings, Next0, Next, Arg, OuterRefs) :-
    ( named_atom(Subject, Name) ->
        Bindings = Bindings0,
        Next = Next0,
        Arg = named(Name),
        OuterRefs = []
    ; var(Subject) ->
        OuterRefs = [Subject],
        ( ref_member(Subject, AnteDomain) ->
            binding_arg(Subject, Bindings0, Bindings, Next0, Next, Arg)
        ; ref_member(Subject, ConsequentDomain) ->
            reject(referent, rule(Position, consequent_local_subject))
        ; reject(referent, rule(Position, undeclared_head_subject))
        )
    ; reject(unsupported, rule(Position, head_subject))
    ).

rule_head_arguments([], _, _, _, Bindings, Bindings, Next, Next, _,
        [], []).
rule_head_arguments([Term|Terms], Position, AnteDomain, ConsequentDomain,
        Bindings0, Bindings, Next0, Next, Index, [Arg|Args], OuterRefs) :-
    rule_head_argument(Term, Position, AnteDomain, ConsequentDomain,
        Bindings0, Bindings1, Next0, Next1, Index, Arg, HereRefs),
    NextIndex is Index + 1,
    rule_head_arguments(Terms, Position, AnteDomain, ConsequentDomain,
        Bindings1, Bindings, Next1, Next, NextIndex, Args, RestRefs),
    append(HereRefs, RestRefs, OuterRefs).

rule_head_argument(Term, _, _, _, Bindings, Bindings, Next, Next, _,
        named(Name), []) :-
    named_atom(Term, Name),
    !.
rule_head_argument(Term, _, _, _, Bindings, Bindings, Next, Next, _,
        Quantity, []) :-
    quantity_from_int(Term, Quantity, _),
    !.
rule_head_argument(Term, Position, AnteDomain, ConsequentDomain,
        Bindings0, Bindings, Next0, Next, Index, Arg, [Term]) :-
    var(Term),
    !,
    ( ref_member(Term, AnteDomain) ->
        binding_arg(Term, Bindings0, Bindings, Next0, Next, Arg)
    ; ref_member(Term, ConsequentDomain) ->
        reject(referent,
            rule(Position, consequent_local_argument(Index)))
    ; reject(referent, rule(Position, undeclared_head_argument(Index)))
    ).
rule_head_argument(_, Position, _, _, _, _, _, _, Index, _, _) :-
    reject(unsupported, rule(Position, head_argument(Index))).

lower_rule_body(Position, Conditions, Domain, Bindings0, Bindings,
        Next0, Next, Literals, Anchors, Events, Entities, PositiveRefs) :-
    index_conditions(Conditions, 1, Indexed),
    lower_rule_body_conditions(Position, Indexed, Indexed, Domain,
        [], Bindings0, Bindings, Next0, Next, [], PositiveRefs,
        Literals, Anchors, Events, Entities).

lower_rule_body_conditions(_, [], _, _, _, Bindings, Bindings,
        Next, Next, PositiveRefs, PositiveRefs, [], [], [], []).
lower_rule_body_conditions(Position,
        [indexed(Index, Condition)|Conditions], All, Domain, Consumed,
        Bindings0, Bindings, Next0, Next, PositiveRefs0, PositiveRefs,
        Literals, Anchors, Events, Entities) :-
    ( memberchk(Index, Consumed) ->
        lower_rule_body_conditions(Position, Conditions, All, Domain,
            Consumed, Bindings0, Bindings, Next0, Next,
            PositiveRefs0, PositiveRefs, Literals, Anchors, Events,
            Entities)
    ; lower_rule_condition_group(Position, Index, Condition, All, Domain,
          PositiveRefs0, Consumed, Consumed1, Bindings0, Bindings1,
          Next0, Next1, HereLiterals, HereAnchors, HereEvents,
          HereEntities, HerePositiveRefs),
      append(HerePositiveRefs, PositiveRefs0, PositiveRefs1),
      lower_rule_body_conditions(Position, Conditions, All, Domain,
          Consumed1, Bindings1, Bindings, Next1, Next,
          PositiveRefs1, PositiveRefs, RestLiterals, RestAnchors,
          RestEvents, RestEntities),
      append(HereLiterals, RestLiterals, Literals),
      append(HereAnchors, RestAnchors, Anchors),
      append(HereEvents, RestEvents, Events),
      append(HereEntities, RestEntities, Entities)
    ).

lower_rule_condition_group(Position, Index, Condition, All, Domain,
        _PositiveRefs, Consumed0, Consumed, Bindings0, Bindings,
        Next0, Next, Literals, Anchors, Events, Entities,
        HerePositiveRefs) :-
    temporal_window_profile(Index, Condition, All, Window),
    !,
    lower_rule_temporal_window(Position, Index, Window, Domain,
        Consumed0, Consumed, Bindings0, Bindings, Next0, Next,
        Literals, Anchors, Events, Entities, HerePositiveRefs).
lower_rule_condition_group(Position, Index, Condition, All, Domain,
        _PositiveRefs, Consumed0, Consumed, Bindings0, Bindings,
        Next0, Next, Literals, Anchors, Events, Entities,
        HerePositiveRefs) :-
    anchored_condition(Condition, Object, ObjectAnchor),
    count_quantity_object_parts(Object, _, _, _, _),
    root_count_quantity_pair(Object, All, _),
    !,
    lower_rule_numeric_quantity(Position, Index, Object, ObjectAnchor,
        All, Domain, Consumed0, Consumed, Bindings0, Bindings,
        Next0, Next, Literals, Anchors, Events, Entities,
        HerePositiveRefs).
lower_rule_condition_group(Position, Index, Condition, All, Domain,
        _PositiveRefs, Consumed0, Consumed, Bindings0, Bindings,
        Next0, Next, Literals, Anchors, Events, Entities,
        HerePositiveRefs) :-
    anchored_condition(Condition, Predicate, PredicateAnchor),
    predicate_event(Predicate, Event),
    event_has_modifier(Event, All),
    !,
    lower_rule_temporal(Position, Index, Predicate, PredicateAnchor,
        All, Domain, Consumed0, Consumed, Bindings0, Bindings,
        Next0, Next, Literals, Anchors, Events, Entities,
        HerePositiveRefs).
lower_rule_condition_group(Position, Index, Condition, All, Domain,
        PositiveRefs, Consumed0, Consumed, Bindings0, Bindings,
        Next0, Next, [Literal], Anchors, Events, Entities,
        HerePositiveRefs) :-
    lower_rule_condition(Position, Index, Condition, All, Domain,
        PositiveRefs, Consumed0, Consumed, Bindings0, Bindings,
        Next0, Next, Literal, Anchors, Events, Entities,
        HerePositiveRefs).

lower_rule_numeric_quantity(Position, Index, Object, ObjectAnchor,
        All, Domain, Consumed0, Consumed, Bindings0, Bindings,
        Next0, Next, [PredicateLiteral, CompareLiteral],
        [ObjectAnchor, PredicateAnchor], [Event], Entities,
        PositiveRefs) :-
    count_quantity_object_parts(
        Object, Referent, Unit, Relation, Value),
    require_declared_entity(rule(Position, antecedent), Referent, Domain),
    quantity_transitive_entry(Referent, All,
        PredicatePosition, Event, Verb, Subject, PredicateAnchor),
    require_non_be_verb(rule(Position, antecedent), Verb),
    require_erasable_event(rule(Position, antecedent), Event, Domain, All),
    rule_body_argument(Subject, Position, Domain,
        Bindings0, Bindings1, Next0, Next1, 1,
        SubjectArg, SubjectRefs),
    binding_arg(Referent, Bindings1, Bindings, Next1, Next, QuantityArg),
    quantity_bound_from_relation(Relation, Value, Unit, Bound),
    PredicateLiteral = pred(Verb, [SubjectArg, QuantityArg]),
    CompareLiteral = quantity_compare(QuantityArg, Bound),
    Consumed = [Index, PredicatePosition|Consumed0],
    append(SubjectRefs, [Referent], Entities),
    PositiveRefs = Entities.

lower_rule_temporal(Position, Index, Predicate, PredicateAnchor,
        All, Domain, Consumed0, Consumed, Bindings0, Bindings,
        Next0, Next, [Temporal], [PredicateAnchor, ModifierAnchor],
        [Event], EntityRefs, EntityRefs) :-
    predicate_event(Predicate, Event),
    event_modifier_entries(Event, All, Modifiers),
    ( Modifiers = [modifier_entry(
          ModifierPosition, Relation, AnchorTerm, ModifierAnchor)],
      admitted_temporal_relation(Relation),
      named_atom(AnchorTerm, AnchorName) ->
        true
    ; reject(temporal,
          rule(Position, antecedent_condition(Index, modifier_profile)))
    ),
    require_event_occurrences(
        rule(Position, antecedent_condition(Index)), Event, All, 2),
    rule_temporal_event_atom(Position, Predicate, Domain,
        Bindings0, Bindings, Next0, Next, EventAtom, EntityRefs),
    Temporal = temporal(Relation, EventAtom, anchor(named(AnchorName))),
    Consumed = [Index, ModifierPosition|Consumed0].

rule_temporal_event_atom(Position, Predicate, Domain,
        Bindings0, Bindings, Next0, Next,
        pred(Verb, [Arg]), EntityRefs) :-
    predicate3_parts(Predicate, Event, Verb, Subject),
    !,
    require_non_be_verb(rule(Position, antecedent), Verb),
    require_local_event(rule(Position, antecedent), Event, Domain),
    rule_body_subject(Position, Subject, Domain,
        Bindings0, Bindings, Next0, Next, Arg, EntityRefs).
rule_temporal_event_atom(Position, Predicate, Domain,
        Bindings0, Bindings, Next0, Next,
        pred(Verb, Args), EntityRefs) :-
    predicate4_parts(Predicate, Event, Verb, Subject, Object),
    require_non_be_verb(rule(Position, antecedent), Verb),
    require_local_event(rule(Position, antecedent), Event, Domain),
    rule_body_arguments([Subject, Object], Position, Domain,
        Bindings0, Bindings, Next0, Next, 1, Args, EntityRefs).

lower_rule_temporal_window(Position, Index,
        window(Event, Predicate, Direction, AnchorName,
            Range, UpperReferent, Lower, Upper, Unit,
            Positions, SourceAnchors), Domain,
        Consumed0, Consumed, Bindings0, Bindings, Next0, Next,
        [Window], SourceAnchors, [Event], Entities, PositiveRefs) :-
    require_local_event(
        rule(Position, antecedent_condition(Index)), Event, Domain),
    require_declared_entity(
        rule(Position, antecedent), Range, Domain),
    require_declared_entity(
        rule(Position, antecedent), UpperReferent, Domain),
    rule_temporal_event_atom(Position, Predicate, Domain,
        Bindings0, Bindings, Next0, Next, EventAtom, EventEntities),
    quantity_bound_from_relation(geq, Lower, Unit, LowerBound),
    quantity_bound_from_relation(leq, Upper, Unit, UpperBound),
    Window = temporal_window(Direction, EventAtom,
        anchor(named(AnchorName)), interval(LowerBound, UpperBound)),
    append(Positions, Consumed0, Consumed),
    append([Range, UpperReferent], EventEntities, Entities),
    PositiveRefs = EventEntities.

lower_rule_condition(Position, Index, Condition, All, Domain,
        PositiveRefs, Consumed0, Consumed, Bindings0, Bindings,
        Next0, Next, Literal, Anchors, Events, Entities,
        HerePositiveRefs) :-
    ( has_functor(Condition, '~', 1) ->
        lower_naf_body_literal(Position, Index, Condition, Domain,
            PositiveRefs, Bindings0, Literal, Anchors),
        Consumed = [Index|Consumed0],
        Bindings = Bindings0,
        Next = Next0,
        Events = [],
        Entities = [],
        HerePositiveRefs = []
    ; contains_negation(Condition) ->
        reject(negation, rule(Position, antecedent_condition(Index)))
    ; anchored_condition(Condition, Inner, Anchor) ->
        ( functor_name(Inner, property) ->
            lower_body_property(Position, Index, Inner, Anchor, All,
                Domain, Consumed0, Consumed, Bindings0, Bindings,
                Next0, Next, Literal, Anchors, Events, Entities,
                HerePositiveRefs)
        ; lower_body_literal(Position, Inner, All, Domain,
              Bindings0, Bindings, Next0, Next, Literal, Events,
              Entities),
          Consumed = [Index|Consumed0],
          Anchors = [Anchor],
          HerePositiveRefs = Entities
        )
    ; contains_query2(Condition) ->
        reject(wh_query, rule(Position, antecedent_condition(Index)))
    ; unsupported_condition(
          rule(Position, antecedent_condition(Index)), Condition)
    ).

lower_body_property(Position, Index, Property, PropertyAnchor, All,
        Domain, Consumed0, Consumed, Bindings0, Bindings,
        Next0, Next, Literal, Anchors, Events, Entities, PositiveRefs) :-
    Location = rule(Position, antecedent_condition(Index)),
    property_parts(Location, Property, Carrier, Name, ComparisonTerms),
    require_declared_entity(rule(Position, antecedent), Carrier, Domain),
    rule_property_be_matches(Carrier, All, Matches),
    lower_body_property_with_matches(Matches, Position, Index, Carrier,
        Name, ComparisonTerms, PropertyAnchor, All, Domain, Consumed0,
        Consumed, Bindings0, Bindings, Next0, Next, Literal, Anchors,
        Events, Entities, PositiveRefs).

lower_body_property_with_matches([], Position, Index, Carrier, Name,
        ComparisonTerms, PropertyAnchor, _All, Domain, Consumed0,
        [Index|Consumed0], Bindings0, Bindings, Next0, Next,
        pred(Name, Args), [PropertyAnchor], [], EntityRefs, EntityRefs) :-
    rule_body_arguments([Carrier|ComparisonTerms], Position, Domain,
        Bindings0, Bindings, Next0, Next, 1, Args, EntityRefs).
lower_body_property_with_matches(
        [property_be_match(BePosition, Event, Subject, BeAnchor)],
        Position, Index, Carrier, Name, ComparisonTerms, PropertyAnchor,
        All, Domain, Consumed0, Consumed, Bindings0, Bindings,
        Next0, Next, pred(Name, Args), [PropertyAnchor, BeAnchor],
        Events, [Carrier|EntityRefs], EntityRefs) :-
    !,
    require_erasable_event(rule(Position, antecedent), Event, Domain, All),
    rule_body_arguments([Subject|ComparisonTerms], Position, Domain,
        Bindings0, Bindings, Next0, Next, 1, Args, EntityRefs),
    ( memberchk(BePosition, Consumed0) ->
        Events = [],
        Consumed = [Index|Consumed0]
    ; Events = [Event],
      Consumed = [Index, BePosition|Consumed0]
    ).
lower_body_property_with_matches([_,_|_], Position, Index, _, _, _, _,
        _, _, _, _, _, _, _, _, _, _, _, _, _) :-
    reject(referent,
        rule(Position, antecedent_condition(
            Index, property_subject(ambiguous)))).

rule_property_be_matches(_, [], []).
rule_property_be_matches(Carrier,
        [indexed(Position, Condition)|Conditions], Matches) :-
    ( anchored_condition(Condition, Inner, Anchor),
      predicate4_parts(Inner, Event, Verb, Subject, Object),
      Verb == be,
      Object == Carrier ->
        Matches = [property_be_match(
            Position, Event, Subject, Anchor)|Rest]
    ; Matches = Rest
    ),
    rule_property_be_matches(Carrier, Conditions, Rest).

lower_naf_body_literal(Position, Index, Condition, OuterDomain,
        PositiveRefs, Bindings, Literal, Anchors) :-
    arg(1, Condition, Drs),
    ( contains_v(Drs) ->
        reject(disjunction,
            rule(Position, antecedent_condition(Index, v_under_naf)))
    ; true
    ),
    ( has_functor(Drs, drs, 2) ->
        arg(1, Drs, Domain),
        arg(2, Drs, Conditions),
        ( is_list(Domain), is_list(Conditions) ->
            true
        ; reject(negation,
              rule(Position, antecedent_condition(Index, drs_lists)))
        )
    ; reject(negation,
          rule(Position, antecedent_condition(Index, drs_shape)))
    ),
    ( naf_intransitive_profile(Domain, Conditions, OuterDomain,
          PositiveRefs, Bindings, Literal, Anchors) ->
        true
    ; naf_copula_profile(Domain, Conditions, OuterDomain,
          PositiveRefs, Bindings, Literal, Anchors) ->
        true
    ; reject(negation,
          rule(Position, antecedent_condition(Index, profile)))
    ).

naf_anchor(Anchor, Sentence) :-
    has_functor(Anchor, '/', 2),
    arg(1, Anchor, Sentence),
    arg(2, Anchor, Token),
    integer(Sentence),
    Sentence > 0,
    integer(Token),
    Token > 0.

naf_intransitive_profile(Domain, Conditions, OuterDomain, PositiveRefs,
        Bindings, Literal, [Anchor]) :-
    Conditions = [Condition],
    anchored_condition(Condition, Predicate, Anchor),
    naf_anchor(Anchor, _),
    predicate3_parts(Predicate, Event, Verb, Subject),
    var(Event),
    atom(Verb),
    require_lemma(rule_naf, verb, Verb),
    Verb \== be,
    var(Subject),
    Domain == [Event],
    ref_member(Subject, OuterDomain),
    ref_member(Subject, PositiveRefs),
    lookup_binding(Subject, Bindings, Number),
    Literal = naf(pred(Verb, [var(Number)])).

naf_copula_profile(Domain, Conditions, OuterDomain, PositiveRefs,
        Bindings, Literal, [ObjectAnchor, BeAnchor]) :-
    Conditions = [ObjectCondition, BeCondition],
    anchored_condition(ObjectCondition, Object, ObjectAnchor),
    naf_anchor(ObjectAnchor, Sentence),
    exact_object(Object, ObjectReferent, Class),
    require_lemma(rule_naf, object_class, Class),
    anchored_condition(BeCondition, Be, BeAnchor),
    naf_anchor(BeAnchor, BeSentence),
    Sentence =:= BeSentence,
    has_functor(Be, predicate, 4),
    arg(1, Be, Event),
    arg(2, Be, Verb),
    arg(3, Be, Subject),
    arg(4, Be, BeObjectReferent),
    Verb == be,
    var(Event),
    var(Subject),
    BeObjectReferent == ObjectReferent,
    ObjectReferent \== Event,
    Domain == [ObjectReferent, Event],
    ref_member(Subject, OuterDomain),
    ref_member(Subject, PositiveRefs),
    lookup_binding(Subject, Bindings, Number),
    Literal = naf(pred(Class, [var(Number)])).

lower_body_literal(Position, Inner, _All, Domain,
        Bindings0, Bindings, Next0, Next, Literal, [], [Referent]) :-
    functor_name(Inner, object),
    !,
    ( exact_object(Inner, Referent0, Class) ->
        require_lemma(rule(Position, antecedent), object_class, Class),
        Referent = Referent0
    ; reject(unsupported, rule(Position, antecedent_object))
    ),
    require_declared_entity(rule(Position, antecedent), Referent, Domain),
    binding_arg(Referent, Bindings0, Bindings, Next0, Next, Arg),
    Literal = pred(Class, [Arg]).
lower_body_literal(Position, Inner, _All, Domain,
        Bindings0, Bindings, Next0, Next, Literal, [], EntityRefs) :-
    has_functor(Inner, relation, 3),
    !,
    require_of_relation(rule(Position, antecedent), Inner, Left, Right),
    rule_body_arguments([Left, Right], Position, Domain,
        Bindings0, Bindings, Next0, Next, 1, Args, EntityRefs),
    Literal = pred(of, Args).
lower_body_literal(Position, Inner, All, Domain,
        Bindings0, Bindings, Next0, Next, Literal, Events, EntityRefs) :-
    functor_name(Inner, predicate),
    !,
    ( predicate_named_be(Inner) ->
        reject(unsupported, rule(Position, antecedent_be))
    ; predicate3_parts(Inner, Event, Verb, Subject) ->
        require_non_be_verb(rule(Position, antecedent), Verb),
        require_local_event(rule(Position, antecedent), Event, Domain),
        rule_body_subject(Position, Subject, Domain, Bindings0, Bindings,
            Next0, Next, Arg, EntityRefs),
        Literal = pred(Verb, [Arg]),
        Events = [Event]
    ; predicate4_parts(Inner, Event, Verb, Subject, Object) ->
        require_non_be_verb(rule(Position, antecedent), Verb),
        require_erasable_event(rule(Position, antecedent), Event,
            Domain, All),
        rule_body_arguments([Subject, Object], Position, Domain,
            Bindings0, Bindings, Next0, Next, 1, Args, EntityRefs),
        Literal = pred(Verb, Args),
        Events = [Event]
    ; reject(unsupported, rule(Position, antecedent_predicate))
    ).
lower_body_literal(Position, Inner, _, _, _, _, _, _, _, _, _) :-
    ( contains_query2(Inner) ->
        reject(wh_query, rule(Position, antecedent))
    ; has_functor(Inner, '=>', 2) ->
        reject(unsupported, rule(Position, nested_implication))
    ; unsupported_condition(rule(Position, antecedent), Inner)
    ).

rule_body_subject(Position, Subject, Domain, Bindings0, Bindings,
        Next0, Next, Arg, EntityRefs) :-
    ( named_atom(Subject, Name) ->
        Bindings = Bindings0,
        Next = Next0,
        Arg = named(Name),
        EntityRefs = []
    ; var(Subject) ->
        EntityRefs = [Subject],
        ( ref_member(Subject, Domain) ->
            binding_arg(Subject, Bindings0, Bindings, Next0, Next, Arg)
        ; reject(referent, rule(Position, undeclared_body_subject))
        )
    ; reject(unsupported, rule(Position, body_subject))
    ).

rule_body_arguments([], _, _, Bindings, Bindings, Next, Next, _, [], []).
rule_body_arguments([Term|Terms], Position, Domain,
        Bindings0, Bindings, Next0, Next, Index, [Arg|Args], EntityRefs) :-
    rule_body_argument(Term, Position, Domain, Bindings0, Bindings1,
        Next0, Next1, Index, Arg, HereRefs),
    NextIndex is Index + 1,
    rule_body_arguments(Terms, Position, Domain, Bindings1, Bindings,
        Next1, Next, NextIndex, Args, RestRefs),
    append(HereRefs, RestRefs, EntityRefs).

rule_body_argument(Term, _, _, Bindings, Bindings, Next, Next, _,
        named(Name), []) :-
    named_atom(Term, Name),
    !.
rule_body_argument(Term, _, _, Bindings, Bindings, Next, Next, _,
        Quantity, []) :-
    quantity_from_int(Term, Quantity, _),
    !.
rule_body_argument(Term, Position, Domain, Bindings0, Bindings,
        Next0, Next, Index, Arg, [Term]) :-
    var(Term),
    !,
    ( ref_member(Term, Domain) ->
        binding_arg(Term, Bindings0, Bindings, Next0, Next, Arg)
    ; reject(referent, rule(Position, undeclared_body_argument(Index)))
    ).
rule_body_argument(_, Position, _, _, _, _, _, Index, _, _) :-
    reject(unsupported, rule(Position, body_argument(Index))).

binding_arg(Referent, Bindings, Bindings, Next, Next, var(Number)) :-
    lookup_binding(Referent, Bindings, Number),
    !.
binding_arg(Referent, Bindings0, [binding(Referent, Next0)|Bindings0],
        Next0, Next, var(Next0)) :-
    Next is Next0 + 1.

lookup_binding(Referent, [binding(Existing, Number)|_], Number) :-
    Referent == Existing,
    !.
lookup_binding(Referent, [_|Bindings], Number) :-
    lookup_binding(Referent, Bindings, Number).

require_bound_head_refs(_, [], _).
require_bound_head_refs(Position, [Referent|Referents], BodyRefs) :-
    ( ref_member(Referent, BodyRefs) ->
        true
    ; reject(referent, rule(Position, unbound_head_referent))
    ),
    require_bound_head_refs(Position, Referents, BodyRefs).

/* A final question is either ground yes/no or the exact admitted who shape. */
lower_question(Question, Draft) :-
    ( has_functor(Question, question, 1) ->
        arg(1, Question, Drs)
    ; reject(unsupported, question(shape))
    ),
    require_nested_drs(question, Drs, Domain, Conditions),
    validate_domain(question, Domain),
    ( contains_v(Conditions) ->
        reject(disjunction, question)
    ; contains_negation(Conditions) ->
        reject(negation, question)
    ; contains_query2(Conditions) ->
        lower_wh_question(Domain, Conditions, Draft)
    ; lower_yes_no_question(Domain, Conditions, Draft)
    ).

lower_yes_no_question(Domain, Conditions, Draft) :-
    ( scalar_question_candidate(Conditions) ->
        lower_scalar_question(Domain, Conditions, Predicate, Anchors,
            Events, Entities)
    ; question_copular_candidate(Conditions) ->
        lower_question_copula(Domain, Conditions, Predicate, Anchors,
            Events, Entities)
    ; lower_question_single(Domain, Conditions, Predicate, Anchors,
          Events, Entities)
    ),
    validate_scope_accounting(question, Domain, Events, Entities),
    source_from_anchors(question, Anchors, Sentence, Tokens),
    Draft = draft_query(Predicate, Sentence, Tokens).

scalar_question_candidate(Conditions) :-
    condition_inner(Conditions, Property),
    scalar_comparison_property(Property, _, _),
    !.

lower_scalar_question(Domain, Conditions, Predicate,
        [PropertyAnchor, BeAnchor], [Event], Entities) :-
    scalar_question_parts(Conditions, Property, PropertyAnchor,
        Carrier, ComparisonTerm, Event, ActualTerm, BeAnchor),
    require_declared_entity(question, Carrier, Domain),
    require_erasable_event(question, Event, Domain, Conditions),
    require_scalar_quantity_operand(
        question, ActualTerm, Conditions, Actual, ActualUnit),
    scalar_question_operand(ComparisonTerm, Conditions,
        Comparison, ComparisonUnit, OperandEntities),
    ( ActualUnit == ComparisonUnit ->
        true
    ; reject(quantity,
          scalar_cross_unit(ActualUnit, ComparisonUnit))
    ),
    property_parts(question, Property, _, Name, [ComparisonTerm]),
    Predicate = pred(Name, [Actual, Comparison]),
    Entities = [Carrier|OperandEntities].

scalar_question_parts([PropertyCondition, BeCondition], Property,
        PropertyAnchor, Carrier, Comparison, Event, Actual, BeAnchor) :-
    anchored_condition(PropertyCondition, Property, PropertyAnchor),
    scalar_comparison_property(Property, Carrier, Comparison),
    anchored_condition(BeCondition, Be, BeAnchor),
    predicate4_parts(Be, Event, Verb, Actual, Object),
    Verb == be,
    Object == Carrier,
    !.
scalar_question_parts(_, _, _, _, _, _, _, _) :-
    reject(quantity, question_scalar_comparison_profile).

scalar_question_operand(Term, _, Quantity, Unit, []) :-
    quantity_from_int(Term, Quantity, Unit),
    !.
scalar_question_operand(Term, Conditions, Quantity, Unit, [Term]) :-
    var(Term),
    condition_inner(Conditions, Object),
    count_quantity_object_parts(Object, Stored, Unit, eq, Value),
    Stored == Term,
    Quantity = quantity(integer(Value), unit(Unit)),
    !.
scalar_question_operand(_, _, _, _, _) :-
    reject(quantity, question_scalar_comparison_operand).

question_copular_candidate([Condition, _]) :-
    anchored_condition(Condition, Inner, _),
    copular_carrier_term(Inner).

lower_question_copula(Domain, Conditions, Predicate,
        [CarrierAnchor, BeAnchor], [Event], [Carrier]) :-
    Conditions = [CarrierCondition, BeCondition],
    anchored_condition(CarrierCondition, CarrierTerm, CarrierAnchor),
    question_copular_carrier(
        CarrierTerm, Carrier, Name, ComparisonTerms),
    ( anchored_condition(BeCondition, Be, BeAnchor) ->
        true
    ; reject(unsupported, question(copula_profile))
    ),
    require_question_copular_be(Be, Carrier, Event, SubjectName),
    require_declared_entity(question, Carrier, Domain),
    require_erasable_event(question, Event, Domain, Conditions),
    question_property_args(ComparisonTerms, Domain, 2, ComparisonArgs),
    Predicate = pred(Name, [named(SubjectName)|ComparisonArgs]).

question_copular_carrier(CarrierTerm, Carrier, Name, ComparisonTerms) :-
    ( functor_name(CarrierTerm, object) ->
        ( exact_object(CarrierTerm, Carrier0, Class) ->
            require_lemma(question, object_class, Class),
            Carrier = Carrier0,
            Name = Class,
            ComparisonTerms = []
        ; reject(unsupported, question(copula_profile))
        )
    ; property_parts(question, CarrierTerm, Carrier, Name,
          ComparisonTerms)
    ).

require_question_copular_be(Be, Carrier, Event, SubjectName) :-
    ( predicate4_parts(Be, Event0, Verb, Subject, Object),
      Verb == be,
      var(Event0),
      named_atom(Subject, Name),
      Object == Carrier ->
        Event = Event0,
        SubjectName = Name
    ; reject(unsupported, question(copula_profile))
    ).

question_property_args([], _, _, []).
question_property_args([Term|Terms], Domain, Index, [Arg|Args]) :-
    require_ground_argument(question, Index, Term, Domain, Arg),
    Next is Index + 1,
    question_property_args(Terms, Domain, Next, Args).

lower_question_single(Domain, [Condition], Predicate,
        [Anchor], [], [Referent]) :-
    anchored_condition(Condition, Object, Anchor),
    unit_bound_object_parts(
        Object, Referent, Class, Unit, Relation, Value),
    !,
    require_declared_entity(question, Referent, Domain),
    require_lemma(question, object_class, Class),
    quantity_bound_from_relation(Relation, Value, Unit, Bound),
    Predicate = pred(Class, [Bound]).
lower_question_single(Domain, Conditions, Predicate, [Anchor], Events, []) :-
    ( Conditions = [Condition] ->
        true
    ; length(Conditions, Count),
      reject(unsupported, question(condition_count(Count)))
    ),
    ( anchored_condition(Condition, Inner, Anchor) ->
        true
    ; unsupported_condition(question, Condition)
    ),
    ( predicate_named_be(Inner) ->
        reject(unsupported, question(copula))
    ; predicate3_parts(Inner, Event, Verb, Subject) ->
        require_non_be_verb(question, Verb),
        require_local_event(question, Event, Domain),
        require_query_subject(Subject, Domain, Arg),
        Predicate = pred(Verb, [Arg]),
        Events = [Event]
    ; predicate4_parts(Inner, Event, Verb, Subject, Object) ->
        require_non_be_verb(question, Verb),
        require_erasable_event(question, Event, Domain, Conditions),
        require_ground_argument(question, 1, Subject, Domain, SubjectArg),
        require_ground_argument(question, 2, Object, Domain, ObjectArg),
        Predicate = pred(Verb, [SubjectArg, ObjectArg]),
        Events = [Event]
    ; has_functor(Inner, relation, 3) ->
        require_of_relation(question, Inner, Left, Right),
        require_ground_argument(question, 1, Left, Domain, LeftArg),
        require_ground_argument(question, 2, Right, Domain, RightArg),
        Predicate = pred(of, [LeftArg, RightArg]),
        Events = []
    ; functor_name(Inner, property) ->
        reject(unsupported, question(copula_profile))
    ; functor_name(Inner, object) ->
        reject(unsupported, question(copula_profile))
    ; unsupported_condition(question, Inner)
    ).

lower_wh_question(Domain, Conditions, Draft) :-
    ( wh_question_profile(Domain, Conditions, Verb, QueryAnchor,
          PredicateAnchor, QueryReferent, Event) ->
        require_lemma(question, verb, Verb),
        validate_scope_accounting(question, Domain, [Event],
            [QueryReferent]),
        source_from_anchors(question, [QueryAnchor, PredicateAnchor],
            Sentence, Tokens),
        Draft = draft_wh_query(wh(who), pred(Verb, [var(1)]),
            Sentence, Tokens)
    ; reject(wh_query, question)
    ).

wh_question_profile(Domain, Conditions, Verb, QueryAnchor,
        PredicateAnchor, QueryReferent, Event) :-
    Conditions = [QueryCondition, PredicateCondition],
    anchored_condition(QueryCondition, Query, QueryAnchor),
    has_functor(Query, query, 2),
    arg(1, Query, QueryReferent),
    arg(2, Query, Marker),
    var(QueryReferent),
    Marker == who,
    anchored_condition(PredicateCondition, Predicate, PredicateAnchor),
    predicate3_parts(Predicate, Event, Verb, Subject),
    var(Event),
    atom(Verb),
    Verb \== be,
    Subject == QueryReferent,
    Domain == [QueryReferent, Event].

require_query_subject(Subject, Domain, Arg) :-
    ( named_atom(Subject, Name) ->
        Arg = named(Name)
    ; var(Subject) ->
        ( ref_member(Subject, Domain) ->
            reject(unsupported, question(nonground_subject))
        ; reject(referent, question(undeclared_subject))
        )
    ; reject(unsupported, question(subject))
    ).

/* Referential accounting: event refs are local, single-use, and erasable. */
validate_domain(Location, Domain) :-
    validate_domain_entries(Domain, Location, [], 1).

validate_domain_entries([], _, _, _).
validate_domain_entries([Referent|Referents], Location, Seen, Position) :-
    ( var(Referent) ->
        true
    ; reject(referent, Location-domain_entry(Position, non_variable))
    ),
    ( ref_member(Referent, Seen) ->
        reject(referent, Location-domain_entry(Position, duplicate))
    ; true
    ),
    Next is Position + 1,
    validate_domain_entries(Referents, Location, [Referent|Seen], Next).

require_disjoint_domains(Location, Left, Right) :-
    ( first_shared_ref(Left, Right, _) ->
        reject(referent, Location-redeclared_referent)
    ; true
    ).

first_shared_ref([Referent|_], Other, Referent) :-
    ref_member(Referent, Other),
    !.
first_shared_ref([_|Referents], Other, Shared) :-
    first_shared_ref(Referents, Other, Shared).

require_local_event(Location, Event, Domain) :-
    ( var(Event) ->
        ( ref_member(Event, Domain) ->
            true
        ; reject(referent, Location-undeclared_event)
        )
    ; reject(referent, Location-event_not_variable)
    ).

require_erasable_event(Location, Event, Domain, Conditions) :-
    require_local_event(Location, Event, Domain),
    ref_occurrence_count(Event, Conditions, Count),
    ( Count =:= 1 ->
        true
    ; reject(referent, Location-event_in_use)
    ).

ref_occurrence_count(Referent, Term, Count) :-
    ( var(Term) ->
        ( Term == Referent -> Count = 1 ; Count = 0 )
    ; compound(Term) ->
        functor(Term, _, Arity),
        ref_occurrence_count_args(Referent, Term, 1, Arity, 0, Count)
    ; Count = 0
    ).

ref_occurrence_count_args(_, _, Index, Arity, Count, Count) :-
    Index > Arity,
    !.
ref_occurrence_count_args(Referent, Term, Index, Arity, Count0, Count) :-
    arg(Index, Term, Arg),
    ref_occurrence_count(Referent, Arg, Here),
    Count1 is Count0 + Here,
    Next is Index + 1,
    ref_occurrence_count_args(
        Referent, Term, Next, Arity, Count1, Count).

require_declared_entity(Location, Referent, Domain) :-
    ( var(Referent), ref_member(Referent, Domain) ->
        true
    ; reject(referent, Location-undeclared_entity)
    ).

validate_scope_accounting(Location, Domain, Events, Entities) :-
    ( first_duplicate_ref(Events, _) ->
        reject(referent, Location-event_reuse)
    ; true
    ),
    ( first_shared_ref(Events, Entities, _) ->
        reject(referent, Location-event_entity_reuse)
    ; true
    ),
    require_all_declared(Location, Events, Domain, event),
    require_all_declared(Location, Entities, Domain, entity),
    require_all_consumed(Location, Domain, Events, Entities).

first_duplicate_ref([Referent|Referents], Referent) :-
    ref_member(Referent, Referents),
    !.
first_duplicate_ref([_|Referents], Duplicate) :-
    first_duplicate_ref(Referents, Duplicate).

require_all_declared(_, [], _, _).
require_all_declared(Location, [Referent|Referents], Domain, Role) :-
    ( ref_member(Referent, Domain) ->
        true
    ; reject(referent, Location-undeclared(Role))
    ),
    require_all_declared(Location, Referents, Domain, Role).

require_all_consumed(_, [], _, _).
require_all_consumed(Location, [Referent|Referents], Events, Entities) :-
    ( ref_member(Referent, Events)
    ; ref_member(Referent, Entities)
    ),
    !,
    require_all_consumed(Location, Referents, Events, Entities).
require_all_consumed(Location, [_|_], _, _) :-
    reject(referent, Location-unconsumed_domain_referent).

ref_member(Referent, [Existing|_]) :-
    Referent == Existing,
    !.
ref_member(Referent, [_|Referents]) :-
    ref_member(Referent, Referents).

/* Provenance and stable per-sentence clause IDs. */
source_from_anchors(Location, Anchors, Sentence, Tokens) :-
    ( Anchors = [First|Rest] ->
        anchor_parts(Location, First, Sentence0, FirstToken),
        source_anchor_tokens(Rest, Location, Sentence0, OtherTokens),
        sort([FirstToken|OtherTokens], Tokens),
        Sentence = Sentence0
    ; reject(unsupported, Location-missing_anchor)
    ).

source_anchor_tokens([], _, _, []).
source_anchor_tokens([Anchor|Anchors], Location, Sentence, [Token|Tokens]) :-
    anchor_parts(Location, Anchor, HereSentence, Token),
    ( HereSentence =:= Sentence ->
        true
    ; reject(unsupported, Location-mixed_sentence_anchors)
    ),
    source_anchor_tokens(Anchors, Location, Sentence, Tokens).

anchor_parts(Location, Anchor, Sentence, Token) :-
    ( has_functor(Anchor, '/', 2) ->
        arg(1, Anchor, Sentence0),
        arg(2, Anchor, Token0),
        ( integer(Sentence0), Sentence0 >= 1,
          integer(Token0), Token0 >= 1 ->
            Sentence = Sentence0,
            Token = Token0
        ; reject(unsupported, Location-anchor_ordinals)
        )
    ; reject(unsupported, Location-anchor_shape)
    ).

finalize_draft_group([draft_rule_branch(
        Branch, Head, Body, Sentence, Tokens)|Drafts], Counters0, Counters,
        Items) :-
    !,
    next_clause(Sentence, Counters0, Counters, Clause),
    finalize_rule_branch_drafts(
        [draft_rule_branch(Branch, Head, Body, Sentence, Tokens)|Drafts],
        Sentence, Clause, Items).
finalize_draft_group([draft_alternative_set_branch(
        Branch, Members, Body, Sentence, Tokens)|Drafts],
        Counters0, Counters, Items) :-
    !,
    next_clause(Sentence, Counters0, Counters, Clause),
    finalize_alternative_branch_drafts(
        [draft_alternative_set_branch(
            Branch, Members, Body, Sentence, Tokens)|Drafts],
        Sentence, Clause, Items).
finalize_draft_group([Draft], Counters0, Counters, [Item]) :-
    finalize_draft(Draft, Counters0, Counters, Item).

finalize_rule_branch_drafts([], _, _, []).
finalize_rule_branch_drafts(
        [draft_rule_branch(Branch, Head, Body, Sentence, Tokens)|Drafts],
        ExpectedSentence, Clause,
        [rule(rule_id(sentence(Sentence), clause(Clause), branch(Branch)),
            Head, body(Body),
            source(sentence(Sentence), tokens(Tokens)))|Items]) :-
    ( Sentence =:= ExpectedSentence -> true
    ; reject(unsupported, rule(branch_sentence_mismatch))
    ),
    finalize_rule_branch_drafts(
        Drafts, ExpectedSentence, Clause, Items).

finalize_alternative_branch_drafts([], _, _, []).
finalize_alternative_branch_drafts(
        [draft_alternative_set_branch(
            Branch, Members, Body, Sentence, Tokens)|Drafts],
        ExpectedSentence, Clause,
        [alternative_set(
            alternative_set_id(
                sentence(Sentence), clause(Clause), branch(Branch)),
            members(Members), body(Body), satisfaction(any_member),
            exclusivity(not_asserted), exhaustiveness(not_asserted),
            source(sentence(Sentence), tokens(Tokens)))|Items]) :-
    ( Sentence =:= ExpectedSentence -> true
    ; reject(unsupported, alternative_set(branch_sentence_mismatch))
    ),
    finalize_alternative_branch_drafts(
        Drafts, ExpectedSentence, Clause, Items).

finalize_draft(draft_fact(Predicate, Sentence, Tokens), Counters0, Counters,
        fact(fact_id(sentence(Sentence), clause(Clause)), Predicate,
            source(sentence(Sentence), tokens(Tokens)))) :-
    next_clause(Sentence, Counters0, Counters, Clause).
finalize_draft(draft_rule(Head, Body, Sentence, Tokens), Counters0, Counters,
        rule(rule_id(sentence(Sentence), clause(Clause)), Head, body(Body),
            source(sentence(Sentence), tokens(Tokens)))) :-
    next_clause(Sentence, Counters0, Counters, Clause).
finalize_draft(draft_alternative_set(Members, Body, Sentence, Tokens),
        Counters0, Counters,
        alternative_set(
            alternative_set_id(sentence(Sentence), clause(Clause)),
            members(Members), body(Body), satisfaction(any_member),
            exclusivity(not_asserted), exhaustiveness(not_asserted),
            source(sentence(Sentence), tokens(Tokens)))) :-
    next_clause(Sentence, Counters0, Counters, Clause).
finalize_draft(draft_query(Predicate, Sentence, Tokens), Counters0, Counters,
        query(query_id(sentence(Sentence), clause(Clause)), Predicate,
            source(sentence(Sentence), tokens(Tokens)))) :-
    next_clause(Sentence, Counters0, Counters, Clause).
finalize_draft(draft_wh_query(Marker, Predicate, Sentence, Tokens),
        Counters0, Counters,
        query(query_id(sentence(Sentence), clause(Clause)), Marker,
            Predicate, source(sentence(Sentence), tokens(Tokens)))) :-
    next_clause(Sentence, Counters0, Counters, Clause).

next_clause(Sentence, [], [counter(Sentence, 2)], 1).
next_clause(Sentence, [counter(Here, Next0)|Counters], Updated, Clause) :-
    ( Sentence =:= Here ->
        Clause = Next0,
        Next is Next0 + 1,
        Updated = [counter(Here, Next)|Counters]
    ; Updated = [counter(Here, Next0)|Rest],
      next_clause(Sentence, Counters, Rest, Clause)
    ).

label_defined_exceptions(Facts, Rules0, Rules, ClosedWorld) :-
    defined_predicate_keys(Facts, Rules0, Keys),
    label_exception_rules(Rules0, Keys, Rules, ClosedWorld).

defined_predicate_keys(Facts, Rules, Keys) :-
    item_head_keys(Facts, FactKeys),
    item_head_keys(Rules, RuleKeys),
    append(FactKeys, RuleKeys, RawKeys),
    sort(RawKeys, Keys).

item_head_keys([], []).
item_head_keys([Item|Items], Keys) :-
    ( has_functor(Item, fact, 3) -> arg(2, Item, Head)
    ; arg(2, Item, Head)
    ),
    ( has_functor(Head, pred, 2) ->
        lower_predicate_key(Head, Key),
        Keys = [Key|Rest]
    ; Keys = Rest
    ),
    item_head_keys(Items, Rest).

label_exception_rules([], _, [], []).
label_exception_rules([Rule0|Rules0], Keys, [Rule|Rules], ClosedWorld) :-
    Rule0 = rule(Id, Head, body(Body0), Source),
    label_exception_body(Body0, 1, Id, Keys, Body, HereClosedWorld),
    Rule = rule(Id, Head, body(Body), Source),
    label_exception_rules(Rules0, Keys, Rules, RestClosedWorld),
    append(HereClosedWorld, RestClosedWorld, ClosedWorld).

label_exception_body([], _, _, _, [], []).
label_exception_body([Literal0|Literals0], Position, RuleId, Keys,
        [Literal|Literals], ClosedWorld) :-
    ( has_functor(Literal0, naf, 1),
      arg(1, Literal0, Predicate),
      lower_predicate_key(Predicate, Key),
      term_member_eq(Key, Keys) ->
        Key = predicate_key(Name, arity(Arity)),
        ExceptionId = exception_id(rule(RuleId), literal(Position)),
        Literal = naf(ExceptionId, Predicate),
        ClosedWorld = [closed_world(ExceptionId, affects(RuleId),
            predicate_key(Name, arity(Arity)))|RestClosedWorld]
    ; Literal = Literal0,
      ClosedWorld = RestClosedWorld
    ),
    Next is Position + 1,
    label_exception_body(Literals0, Next, RuleId, Keys,
        Literals, RestClosedWorld).

lower_predicate_key(pred(Name, Args),
        predicate_key(Name, arity(Arity))) :-
    length(Args, Arity).

require_factual_section_order(Items) :-
    require_factual_section_order(Items, facts).

require_factual_section_order([], _).
require_factual_section_order([Item|Items], State0) :-
    ( has_functor(Item, fact, 3) ->
        ( State0 == rules ->
            reject(unsupported, root_section_order)
        ; State = facts
        )
    ; has_functor(Item, rule, 4) ->
        State = rules
    ; has_functor(Item, alternative_set, 7) ->
        State = State0
    ; reject(unsupported, generated_item)
    ),
    require_factual_section_order(Items, State).

split_factual_items([], [], [], []).
split_factual_items([Item|Items], Facts, Rules, AlternativeSets) :-
    ( has_functor(Item, fact, 3) ->
        Facts = [Item|RestFacts],
        Rules = RestRules,
        AlternativeSets = RestAlternatives
    ; has_functor(Item, rule, 4) ->
        Facts = RestFacts,
        Rules = [Item|RestRules],
        AlternativeSets = RestAlternatives
    ; has_functor(Item, alternative_set, 7) ->
        Facts = RestFacts,
        Rules = RestRules,
        AlternativeSets = [Item|RestAlternatives]
    ; reject(unsupported, generated_item)
    ),
    split_factual_items(
        Items, RestFacts, RestRules, RestAlternatives).

require_section_order(_, []).
require_section_order(Section, [Item|Items]) :-
    item_order_key(Item, First),
    require_section_order_after(Section, Items, First).

require_section_order_after(_, [], _).
require_section_order_after(Section, [Item|Items], Previous) :-
    item_order_key(Item, Key),
    ( order_key_less(Previous, Key) ->
        true
    ; reject(unsupported, section_order(Section))
    ),
    require_section_order_after(Section, Items, Key).

item_order_key(fact(fact_id(sentence(S), clause(C)), _, _),
    key(S, C, 0)).
item_order_key(rule(rule_id(sentence(S), clause(C)), _, _, _),
    key(S, C, 0)).
item_order_key(rule(
        rule_id(sentence(S), clause(C), branch(B)), _, _, _),
    key(S, C, B)).
item_order_key(alternative_set(
        alternative_set_id(sentence(S), clause(C)), _, _, _, _, _, _),
    key(S, C, 0)).
item_order_key(alternative_set(
        alternative_set_id(sentence(S), clause(C), branch(B)),
        _, _, _, _, _, _),
    key(S, C, B)).

order_key_less(key(S0, C0, B0), key(S, C, B)) :-
    ( S0 < S
    ; S0 =:= S,
      C0 < C
    ; S0 =:= S,
      C0 =:= C,
      B0 < B
    ).

/* Structural helpers and generated-IR backstop. */
anchored_condition(Term, Inner, Anchor) :-
    has_functor(Term, '-', 2),
    arg(1, Term, Inner),
    arg(2, Term, Anchor).

predicate_named_be(Term) :-
    compound(Term),
    functor(Term, predicate, Arity),
    Arity >= 2,
    arg(2, Term, Verb),
    Verb == be.

functor_name(Term, Name) :-
    compound(Term),
    functor(Term, Name, _).

has_functor(Term, Name, Arity) :-
    compound(Term),
    functor(Term, Name, Arity).

contains_v(Term) :-
    contains_functor(Term, v, 2).

contains_tilde(Term) :-
    contains_functor(Term, '~', 1).

contains_functor(Term, Name, Arity) :-
    nonvar(Term),
    compound(Term),
    ( functor(Term, Name, Arity) ->
        true
    ; functor(Term, _, TermArity),
      contains_functor_arg(1, TermArity, Term, Name, Arity)
    ).

contains_functor_arg(Index, Arity, _, _, _) :-
    Index > Arity,
    !,
    fail.
contains_functor_arg(Index, Arity, Term, Name, WantedArity) :-
    arg(Index, Term, Arg),
    ( contains_functor(Arg, Name, WantedArity) ->
        true
    ; Next is Index + 1,
      contains_functor_arg(
          Next, Arity, Term, Name, WantedArity)
    ).

contains_negation(Term) :-
    nonvar(Term),
    compound(Term),
    ( unary_negation_argument(Term, _) ->
        true
    ; functor(Term, _, Arity),
      contains_negation_arg(1, Arity, Term)
    ).

contains_negation_arg(Index, Arity, _) :-
    Index > Arity,
    !,
    fail.
contains_negation_arg(Index, Arity, Term) :-
    arg(Index, Term, Arg),
    ( contains_negation(Arg) ->
        true
    ; Next is Index + 1,
      contains_negation_arg(Next, Arity, Term)
    ).

unary_negation_argument(Term, Argument) :-
    ( has_functor(Term, '-', 1)
    ; has_functor(Term, '~', 1)
    ),
    arg(1, Term, Argument).

contains_query2(Term) :-
    nonvar(Term),
    compound(Term),
    ( functor(Term, query, 2) ->
        true
    ; functor(Term, _, Arity),
      contains_query2_arg(1, Arity, Term)
    ).

contains_query2_arg(Index, Arity, _) :-
    Index > Arity,
    !,
    fail.
contains_query2_arg(Index, Arity, Term) :-
    arg(Index, Term, Arg),
    ( contains_query2(Arg) ->
        true
    ; Next is Index + 1,
      contains_query2_arg(Next, Arity, Term)
    ).

unsupported_condition(Location, Term) :-
    term_signature(Term, Signature),
    reject(unsupported, Location-constructor(Signature)).

term_signature(Term, variable) :-
    var(Term),
    !.
term_signature(Term, atom) :-
    atom(Term),
    !.
term_signature(Term, integer) :-
    integer(Term),
    !.
term_signature(Term, float) :-
    float(Term),
    !.
term_signature(Term, Name/Arity) :-
    compound(Term),
    !,
    functor(Term, Name, Arity).
term_signature(_, other).

validate_generated_ir(IrTerms) :-
    catch(validate_terms(IrTerms),
        ir_reject(Class, Detail),
        handle_generated_ir_rejection(Class, Detail)).

handle_generated_ir_rejection(cycle, Detail) :-
    !,
    reject(cycle, Detail).
handle_generated_ir_rejection(Class, Detail) :-
    throw(error(generated_record_invalid(Class, Detail),
        context(drs_to_ir, ir_validation))).

reject(Class, Detail) :-
    throw(ir_reject(Class, Detail)).

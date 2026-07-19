:- module(drs_to_ir, [lower_terms/2]).

:- set_prolog_flag(encoding, utf8).

:- use_module(library(lists), [append/3, memberchk/2]).
:- use_module(ir_validate, [validate_terms/1]).

/*
Lossless lowering for the deliberately small M3.2 DRS profile. The caller owns
UTF-8, syntax, canonical-byte framing, output buffering, and error emission.
This module owns the M2 envelope and every admitted DRS semantic decision.
*/
lower_terms(Terms, IrTerms) :-
    lower_envelope(Terms, Document, RootDomain, RootConditions),
    validate_domain(root, RootDomain),
    require_final_question(RootConditions, FactualConditions, Question),
    validate_nested_domain_declarations(RootConditions, RootDomain),
    index_conditions(FactualConditions, 1, IndexedConditions),
    lower_root_items(IndexedConditions, IndexedConditions, RootDomain, [],
        [], Counters0, FactualItems, RootEvents, RootEntities),
    validate_scope_accounting(root, RootDomain, RootEvents, RootEntities),
    lower_question(Question, QueryDraft),
    finalize_draft(QueryDraft, Counters0, _Counters, Query),
    require_factual_section_order(FactualItems),
    split_factual_items(FactualItems, Facts, Rules),
    require_section_order(fact, Facts),
    require_section_order(rule, Rules),
    append(Facts, Rules, Prefix),
    append([cnl_ir_record(1), Document|Prefix], [Query], IrTerms),
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

direct_nested_domains(Condition, [AnteDomain, ConsequentDomain]) :-
    has_functor(Condition, '=>', 2),
    arg(1, Condition, Antecedent),
    arg(2, Condition, Consequent),
    has_functor(Antecedent, drs, 2),
    has_functor(Consequent, drs, 2),
    arg(1, Antecedent, AnteDomain),
    arg(1, Consequent, ConsequentDomain),
    is_list(AnteDomain),
    is_list(ConsequentDomain),
    !.
direct_nested_domains(Condition, [Domain]) :-
    has_functor(Condition, question, 1),
    arg(1, Condition, Drs),
    has_functor(Drs, drs, 2),
    arg(1, Drs, Domain),
    is_list(Domain),
    !.
direct_nested_domains(_, []).

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
    ; lower_root_condition(Position, Condition, All, Domain, Consumed,
          Consumed1, Draft, HereEvents, HereEntities),
      finalize_draft(Draft, Counters0, Counters1, Item),
      lower_root_items(Conditions, All, Domain, Consumed1,
          Counters1, Counters, RestItems, RestEvents, RestEntities),
      Items = [Item|RestItems],
      append(HereEvents, RestEvents, Events),
      append(HereEntities, RestEntities, Entities)
    ).

lower_root_condition(Position, Condition, _All, _Domain, Consumed,
        Consumed1, Draft, Events, Entities) :-
    has_functor(Condition, '=>', 2),
    !,
    lower_rule(Position, Condition, Draft),
    Consumed1 = [Position|Consumed],
    Events = [],
    Entities = [].
lower_root_condition(Position, Condition, All, Domain, Consumed,
        Consumed1, Draft, Events, Entities) :-
    anchored_condition(Condition, Inner, Anchor),
    !,
    ( has_functor(Inner, query, 2) ->
        reject(wh_query, root_condition(Position))
    ; functor_name(Inner, object) ->
        lower_copula_from_object(Position, Inner, Anchor, All, Domain,
            Consumed, Consumed1, Draft, Events, Entities)
    ; predicate_named_be(Inner) ->
        lower_copula_from_be(Position, Inner, Anchor, All, Domain,
            Consumed, Consumed1, Draft, Events, Entities)
    ; has_functor(Inner, predicate, 3) ->
        lower_root_predicate(Position, Inner, Anchor, Domain, Draft,
            Events, Entities),
        Consumed1 = [Position|Consumed]
    ; has_functor(Inner, '=>', 2) ->
        reject(unsupported, root_condition(Position, anchored_implication))
    ; unsupported_condition(root_condition(Position), Inner)
    ).
lower_root_condition(Position, Condition, _All, _Domain, _Consumed,
        _, _, _, _) :-
    ( contains_query2(Condition) ->
        reject(wh_query, root_condition(Position))
    ; unsupported_condition(root_condition(Position), Condition)
    ).

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

lower_root_predicate(Position, Predicate, Anchor, Domain, Draft,
        [Event], []) :-
    predicate3_parts(Predicate, Event, Verb, Subject),
    ( atom(Verb), Verb \== be ->
        true
    ; reject(unsupported, root_condition(Position, predicate_name))
    ),
    require_local_event(root_condition(Position), Event, Domain),
    require_ground_subject(root_condition(Position), Subject, Domain, Arg),
    source_from_anchors(root_condition(Position), [Anchor], Sentence, Tokens),
    Draft = draft_fact(pred(Verb, [Arg]), Sentence, Tokens).

predicate3_parts(Predicate, Event, Verb, Subject) :-
    has_functor(Predicate, predicate, 3),
    arg(1, Predicate, Event),
    arg(2, Predicate, Verb),
    arg(3, Predicate, Subject).

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

/* One positive implication becomes one IR rule. */
lower_rule(Position, Rule, Draft) :-
    arg(1, Rule, Antecedent),
    arg(2, Rule, Consequent),
    require_nested_drs(rule(Position, antecedent), Antecedent,
        AnteDomain, AnteConditions),
    require_nested_drs(rule(Position, consequent), Consequent,
        ConsequentDomain, ConsequentConditions),
    validate_domain(antecedent(Position), AnteDomain),
    validate_domain(consequent(Position), ConsequentDomain),
    require_disjoint_domains(rule(Position), AnteDomain, ConsequentDomain),
    ( ( contains_query2(AnteConditions)
      ; contains_query2(ConsequentConditions)
      ) ->
        reject(wh_query, rule(Position))
    ; true
    ),
    lower_rule_head(Position, ConsequentConditions, AnteDomain,
        ConsequentDomain, [], Bindings1, 1, Next1, Head, HeadAnchors,
        ConsequentEvents, HeadOuterRefs),
    lower_rule_body(Position, AnteConditions, AnteDomain,
        Bindings1, _Bindings, Next1, _Next, Body, BodyAnchors,
        AnteEvents, BodyEntityRefs),
    ( Body == [] ->
        reject(unsupported, rule(Position, empty_antecedent))
    ; true
    ),
    require_bound_head_refs(Position, HeadOuterRefs, BodyEntityRefs),
    validate_scope_accounting(antecedent(Position), AnteDomain,
        AnteEvents, BodyEntityRefs),
    validate_scope_accounting(consequent(Position), ConsequentDomain,
        ConsequentEvents, []),
    append(BodyAnchors, HeadAnchors, Anchors),
    source_from_anchors(rule(Position), Anchors, Sentence, Tokens),
    Draft = draft_rule(Head, Body, Sentence, Tokens).

require_nested_drs(Location, Drs, Domain, Conditions) :-
    ( has_functor(Drs, drs, 2) ->
        arg(1, Drs, Domain0),
        arg(2, Drs, Conditions0),
        ( is_list(Domain0), is_list(Conditions0) ->
            Domain = Domain0,
            Conditions = Conditions0
        ; reject(unsupported, Location-drs_lists)
        )
    ; reject(unsupported, Location-drs_shape)
    ).

lower_rule_head(Position, Conditions, AnteDomain, ConsequentDomain,
        Bindings0, Bindings, Next0, Next, Head, [Anchor], [Event],
        OuterRefs) :-
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
    ; predicate3_parts(Inner, Event, Verb, Subject), atom(Verb) ->
        true
    ; unsupported_condition(rule(Position, consequent), Inner)
    ),
    require_local_event(rule(Position, consequent), Event,
        ConsequentDomain),
    rule_head_subject(Position, Subject, AnteDomain, ConsequentDomain,
        Bindings0, Bindings, Next0, Next, Arg, OuterRefs),
    Head = pred(Verb, [Arg]).

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

lower_rule_body(_, [], _, Bindings, Bindings, Next, Next, [], [], [], []).
lower_rule_body(Position, [Condition|Conditions], Domain,
        Bindings0, Bindings, Next0, Next, [Literal|Literals],
        [Anchor|Anchors], Events, Entities) :-
    ( anchored_condition(Condition, Inner, Anchor) ->
        true
    ; ( contains_query2(Condition) ->
          reject(wh_query, rule(Position, antecedent))
      ; unsupported_condition(rule(Position, antecedent), Condition)
      )
    ),
    lower_body_literal(Position, Inner, Domain, Bindings0, Bindings1,
        Next0, Next1, Literal, HereEvents, HereEntities),
    lower_rule_body(Position, Conditions, Domain, Bindings1, Bindings,
        Next1, Next, Literals, Anchors, RestEvents, RestEntities),
    append(HereEvents, RestEvents, Events),
    append(HereEntities, RestEntities, Entities).

lower_body_literal(Position, Inner, Domain, Bindings0, Bindings,
        Next0, Next, Literal, [], [Referent]) :-
    functor_name(Inner, object),
    !,
    ( exact_object(Inner, Referent0, Class) ->
        Referent = Referent0
    ; reject(unsupported, rule(Position, antecedent_object))
    ),
    require_declared_entity(rule(Position, antecedent), Referent, Domain),
    binding_arg(Referent, Bindings0, Bindings, Next0, Next, Arg),
    Literal = pred(Class, [Arg]).
lower_body_literal(Position, Inner, Domain, Bindings0, Bindings,
        Next0, Next, Literal, [Event], EntityRefs) :-
    functor_name(Inner, predicate),
    !,
    ( predicate_named_be(Inner) ->
        reject(unsupported, rule(Position, antecedent_be))
    ; predicate3_parts(Inner, Event, Verb, Subject), atom(Verb) ->
        true
    ; reject(unsupported, rule(Position, antecedent_predicate))
    ),
    require_local_event(rule(Position, antecedent), Event, Domain),
    rule_body_subject(Position, Subject, Domain, Bindings0, Bindings,
        Next0, Next, Arg, EntityRefs),
    Literal = pred(Verb, [Arg]).
lower_body_literal(Position, Inner, _, _, _, _, _, _, _, _) :-
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

/* A final question is one anchored, ground, intransitive predicate. */
lower_question(Question, Draft) :-
    ( contains_query2(Question) ->
        reject(wh_query, question)
    ; true
    ),
    ( has_functor(Question, question, 1) ->
        arg(1, Question, Drs)
    ; reject(unsupported, question(shape))
    ),
    require_nested_drs(question, Drs, Domain, Conditions),
    validate_domain(question, Domain),
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
    ; predicate3_parts(Inner, Event, Verb, Subject), atom(Verb) ->
        true
    ; unsupported_condition(question, Inner)
    ),
    require_local_event(question, Event, Domain),
    require_query_subject(Subject, Domain, Arg),
    validate_scope_accounting(question, Domain, [Event], []),
    source_from_anchors(question, [Anchor], Sentence, Tokens),
    Draft = draft_query(pred(Verb, [Arg]), Sentence, Tokens).

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

finalize_draft(draft_fact(Predicate, Sentence, Tokens), Counters0, Counters,
        fact(fact_id(sentence(Sentence), clause(Clause)), Predicate,
            source(sentence(Sentence), tokens(Tokens)))) :-
    next_clause(Sentence, Counters0, Counters, Clause).
finalize_draft(draft_rule(Head, Body, Sentence, Tokens), Counters0, Counters,
        rule(rule_id(sentence(Sentence), clause(Clause)), Head, body(Body),
            source(sentence(Sentence), tokens(Tokens)))) :-
    next_clause(Sentence, Counters0, Counters, Clause).
finalize_draft(draft_query(Predicate, Sentence, Tokens), Counters0, Counters,
        query(query_id(sentence(Sentence), clause(Clause)), Predicate,
            source(sentence(Sentence), tokens(Tokens)))) :-
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
    ; reject(unsupported, generated_item)
    ),
    require_factual_section_order(Items, State).

split_factual_items([], [], []).
split_factual_items([Item|Items], Facts, Rules) :-
    ( has_functor(Item, fact, 3) ->
        Facts = [Item|RestFacts],
        Rules = RestRules
    ; has_functor(Item, rule, 4) ->
        Facts = RestFacts,
        Rules = [Item|RestRules]
    ; reject(unsupported, generated_item)
    ),
    split_factual_items(Items, RestFacts, RestRules).

require_section_order(_, []).
require_section_order(Section, [Item|Items]) :-
    item_pair(Item, First),
    require_section_order_after(Section, Items, First).

require_section_order_after(_, [], _).
require_section_order_after(Section, [Item|Items], Previous) :-
    item_pair(Item, Pair),
    ( pair_less(Previous, Pair) ->
        true
    ; reject(unsupported, section_order(Section))
    ),
    require_section_order_after(Section, Items, Pair).

item_pair(fact(fact_id(sentence(S), clause(C)), _, _), pair(S, C)).
item_pair(rule(rule_id(sentence(S), clause(C)), _, _, _), pair(S, C)).

pair_less(pair(S0, C0), pair(S, C)) :-
    ( S0 < S
    ; S0 =:= S,
      C0 < C
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
        reject(unsupported, generated_ir(Class, Detail))).

reject(Class, Detail) :-
    throw(ir_reject(Class, Detail)).

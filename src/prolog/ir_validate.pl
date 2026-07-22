:- module(ir_validate, [validate_terms/1]).

:- set_prolog_flag(encoding, utf8).

:- use_module(library(lists), [append/3, memberchk/2]).

/*
IR v2 semantic validator. Input is the parsed term stream after strict UTF-8,
syntax, and canonical-byte gates. Pass order is part of the public contract:
envelope, shape, identity, ordering, scope, safety/NAF, dependency cycles.
Every rejection throws ir_reject(Class, Detail); callers own framing and exits.
*/
validate_terms(Terms) :-
    envelope_pass(Terms, Document, Items),
    shape_pass(Document, Items),
    identity_pass(Document, Items),
    ordering_pass(Items),
    scope_pass(Items),
    safety_naf_pass(Items),
    cycle_pass(Items).

/* Pass 4: record envelope, one query, and facts -> rules -> query order. */
envelope_pass([], _, _) :-
    reject(envelope, term(1, missing_header)).
envelope_pass([Header|Rest], Document, Items) :-
    ( Header == cnl_ir_record(2) ->
        true
    ; reject(envelope, term(1, expected(cnl_ir_record(2))))
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
    ( has_functor(Term, fact, 3) ->
        ( State0 == rules ->
            reject(section_order, term(Index, fact_after_rule))
        ; State = facts
        )
    ; has_functor(Term, rule, 4) ->
        State = rules
    ; State = State0
    ),
    section_order(Items, State).

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
    ( has_functor(Term, fact, 3) ->
        ( shape_fact(Term) -> true
        ; reject(shape, term(Index, fact))
        )
    ; has_functor(Term, rule, 4) ->
        ( shape_rule(Term) -> true
        ; reject(shape, term(Index, rule))
        )
    ; query_term(Term) ->
        ( shape_query(Term) -> true
        ; reject(shape, term(Index, query))
        )
    ; reject(shape, term(Index, item))
    ),
    shape_items(Items).

shape_fact(fact(Id, Predicate, Source)) :-
    shape_id(Id),
    shape_predicate(Predicate),
    shape_source(Source).

shape_rule(rule(Id, Head, Body, Source)) :-
    shape_id(Id),
    shape_predicate(Head),
    shape_body(Body),
    shape_source(Source).

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

shape_predicate(pred(Name, Args)) :-
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

shape_body(body(Literals)) :-
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
    item_identity_parts(Term, ExpectedKind, Id, Source),
    id_parts(Id, ActualKind, Sentence, Clause),
    ( ActualKind == ExpectedKind -> true
    ; reject(identity, term(Index, id_kind(ExpectedKind, ActualKind)))
    ),
    require_positive(Index, id_sentence, Sentence),
    require_positive(Index, id_clause, Clause),
    source_parts(Source, SourceSentence, Tokens),
    require_positive(Index, source_sentence, SourceSentence),
    ( Sentence =:= SourceSentence -> true
    ; reject(identity,
          term(Index, sentence_mismatch(id(Sentence), source(SourceSentence))))
    ),
    require_positive_tokens(Index, Tokens, 1),
    identity_items(Items).

item_identity_parts(fact(Id, _, Source), fact, Id, Source).
item_identity_parts(rule(Id, _, _, Source), rule, Id, Source).
item_identity_parts(query(Id, _, Source), query, Id, Source).
item_identity_parts(query(Id, _, _, Source), query, Id, Source).

id_parts(Id, Kind, Sentence, Clause) :-
    functor(Id, IdName, 2),
    id_kind_name(IdName, Kind),
    arg(1, Id, sentence(Sentence)),
    arg(2, Id, clause(Clause)).

id_kind_name(fact_id, fact).
id_kind_name(rule_id, rule).
id_kind_name(query_id, query).

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

/* Pass 7: section ordering, global (S,C) uniqueness, token ordering. */
ordering_pass(Items) :-
    ordering_items(Items, [], state(none, none, none)).

ordering_items([], _, _).
ordering_items([indexed(Index, Term)|Items], Seen, State0) :-
    item_identity_parts(Term, Section, Id, Source),
    id_parts(Id, _, Sentence, Clause),
    Pair = pair(Sentence, Clause),
    ( memberchk(Pair, Seen) ->
        reject(ordering, term(Index, duplicate_id(Pair)))
    ; true
    ),
    state_last(Section, State0, Last),
    ( Last == none -> true
    ; pair_less(Last, Pair) -> true
    ; reject(ordering, term(Index, section_id_after(Last)))
    ),
    source_parts(Source, _, Tokens),
    ( strictly_ascending(Tokens) -> true
    ; reject(ordering, term(Index, tokens_not_strictly_ascending))
    ),
    state_put(Section, Pair, State0, State),
    ordering_items(Items, [Pair|Seen], State).

state_last(fact, state(Fact, _, _), Fact).
state_last(rule, state(_, Rule, _), Rule).
state_last(query, state(_, _, Query), Query).

state_put(fact, Pair, state(_, Rule, Query), state(Pair, Rule, Query)).
state_put(rule, Pair, state(Fact, _, Query), state(Fact, Pair, Query)).
state_put(query, Pair, state(Fact, Rule, _), state(Fact, Rule, Pair)).

pair_less(pair(Sentence0, Clause0), pair(Sentence, Clause)) :-
    ( Sentence0 < Sentence
    ; Sentence0 =:= Sentence,
      Clause0 < Clause
    ).

strictly_ascending([_]).
strictly_ascending([First, Second|Rest]) :-
    First < Second,
    strictly_ascending([Second|Rest]).

/* Pass 8: variables only in rules; rule numbering is dense first-occurrence. */
scope_pass(Items) :-
    scope_items(Items).

scope_items([]).
scope_items([indexed(Index, Term)|Items]) :-
    ( has_functor(Term, fact, 3) ->
        arg(2, Term, Predicate),
        reject_if_predicate_variable(Index, Predicate)
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
    ; true
    ),
    scope_items(Items).

reject_if_predicate_variable(Index, Predicate) :-
    predicate_vars(Predicate, Vars),
    ( Vars = [Number|_] ->
        reject(scope, term(Index, var_outside_rule(Number)))
    ; true
    ).

predicate_vars(pred(_, Args), Vars) :-
    arg_vars(Args, Vars, []).

arg_vars([], Vars, Vars).
arg_vars([named(_)|Args], Vars0, Vars) :-
    arg_vars(Args, Vars0, Vars).
arg_vars([var(Number)|Args], [Number|Vars0], Vars) :-
    arg_vars(Args, Vars0, Vars).

literal_vars([], []).
literal_vars([Literal|Literals], Vars) :-
    literal_predicate(Literal, Predicate),
    predicate_vars(Predicate, Here),
    literal_vars(Literals, Rest),
    append(Here, Rest, Vars).

literal_predicate(Predicate, Predicate) :-
    has_functor(Predicate, pred, 2).
literal_predicate(naf(Predicate), Predicate).

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

member_number(Number, [Member|_]) :-
    Member =:= Number,
    !.
member_number(Number, [_|Members]) :-
    member_number(Number, Members).

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

predicate_key(pred(Name, Args), pred(Name, Arity)) :-
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

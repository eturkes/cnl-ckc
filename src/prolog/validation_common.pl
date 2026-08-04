:- module(validation_common, [
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
    quantity_bound_parts/5,
    quantity_compare_outcome/3,
    temporal_window_error/2,
    data_atom_key/2
]).

:- set_prolog_flag(encoding, utf8).

branch_shape_detail(Term, branch_id) :-
    compound(Term),
    functor(Term, ItemName, ItemArity),
    branch_item_shape(ItemName, ItemArity),
    arg(1, Term, Id),
    compound(Id),
    functor(Id, Name, 3),
    branch_id_name(Name),
    arg(3, Id, Branch),
    \+ shape_branch_wrapper(Branch).

branch_item_shape(fact, 3).
branch_item_shape(rule, 4).
branch_item_shape(alternative_set, 7).
branch_item_shape(query, 3).
branch_item_shape(query, 4).
branch_item_shape(clause, 3).
branch_item_shape(alternative_set, 6).
branch_item_shape(goal, 2).
branch_item_shape(goal, 3).

branch_id_name(fact_id).
branch_id_name(rule_id).
branch_id_name(alternative_set_id).
branch_id_name(query_id).

shape_optional_branch(_, 2).
shape_optional_branch(Id, 3) :-
    arg(3, Id, Branch),
    shape_branch_wrapper(Branch).

shape_branch_wrapper(Branch) :-
    compound(Branch),
    functor(Branch, branch, 1),
    arg(1, Branch, Value),
    integer(Value).

terms_pairwise_distinct([]).
terms_pairwise_distinct([Term|Terms]) :-
    \+ term_member_eq(Term, Terms),
    terms_pairwise_distinct(Terms).

term_member_eq(Term, [Member|_]) :-
    Term == Member,
    !.
term_member_eq(Term, [_|Members]) :-
    term_member_eq(Term, Members).

shape_argument(Arg) :-
    shape_ground_argument(Arg),
    !.
shape_argument(Arg) :-
    compound(Arg),
    functor(Arg, var, 1),
    arg(1, Arg, Number),
    integer(Number).

shape_ground_argument(Arg) :-
    compound(Arg),
    functor(Arg, named, 1),
    arg(1, Arg, Name),
    atom(Name),
    !.
shape_ground_argument(Arg) :-
    shape_quantity(Arg),
    !.
shape_ground_argument(Arg) :-
    shape_quantity_bound(Arg).

shape_quantity(Term) :-
    compound(Term),
    functor(Term, quantity, 2),
    arg(1, Term, IntegerTerm),
    arg(2, Term, UnitTerm),
    compound(IntegerTerm),
    functor(IntegerTerm, integer, 1),
    arg(1, IntegerTerm, Value),
    integer(Value),
    compound(UnitTerm),
    functor(UnitTerm, unit, 1),
    arg(1, UnitTerm, Unit),
    atom(Unit).

shape_quantity_bound(Term) :-
    compound(Term),
    functor(Term, quantity_bound, 3),
    arg(1, Term, Operator),
    arg(2, Term, Endpoint),
    arg(3, Term, Quantity),
    admitted_bound_pair(Operator, Endpoint),
    shape_quantity(Quantity).

admitted_bound_pair(Operator, Endpoint) :-
    ( Operator == eq, Endpoint == closed
    ; Operator == geq, Endpoint == closed
    ; Operator == leq, Endpoint == closed
    ; Operator == greater, Endpoint == open
    ; Operator == less, Endpoint == open
    ).

shape_predicate_term(Term) :-
    compound(Term),
    functor(Term, pred, 2),
    arg(1, Term, Name),
    arg(2, Term, Args),
    atom(Name),
    is_list(Args),
    Args = [_|_],
    shape_arguments(Args).

shape_arguments([]).
shape_arguments([Arg|Args]) :-
    shape_argument(Arg),
    shape_arguments(Args).

shape_data_atom(Term) :-
    shape_predicate_term(Term),
    !.
shape_data_atom(Term) :-
    compound(Term),
    functor(Term, temporal, 3),
    arg(1, Term, Relation),
    arg(2, Term, Event),
    arg(3, Term, Anchor),
    admitted_temporal_relation(Relation),
    shape_predicate_term(Event),
    shape_anchor(Anchor),
    !.
shape_data_atom(Term) :-
    compound(Term),
    functor(Term, temporal_window, 4),
    arg(1, Term, Direction),
    arg(2, Term, Event),
    arg(3, Term, Anchor),
    arg(4, Term, Interval),
    admitted_window_direction(Direction),
    shape_predicate_term(Event),
    shape_anchor(Anchor),
    shape_interval(Interval).

admitted_temporal_relation(Relation) :-
    ( Relation == before
    ; Relation == after
    ; Relation == during
    ; Relation == within
    ).

admitted_window_direction(Direction) :-
    ( Direction == after
    ; Direction == before
    ).

shape_anchor(Term) :-
    compound(Term),
    functor(Term, anchor, 1),
    arg(1, Term, Arg),
    shape_entity_argument(Arg).

shape_entity_argument(Arg) :-
    compound(Arg),
    functor(Arg, named, 1),
    arg(1, Arg, Name),
    atom(Name),
    !.
shape_entity_argument(Arg) :-
    compound(Arg),
    functor(Arg, var, 1),
    arg(1, Arg, Number),
    integer(Number).

shape_interval(Term) :-
    compound(Term),
    functor(Term, interval, 2),
    arg(1, Term, Lower),
    arg(2, Term, Upper),
    shape_quantity_bound(Lower),
    shape_quantity_bound(Upper),
    quantity_bound_parts(Lower, LowerOperator, _, _, _),
    quantity_bound_parts(Upper, UpperOperator, _, _, _),
    ( LowerOperator == geq ; LowerOperator == greater ),
    ( UpperOperator == leq ; UpperOperator == less ).

shape_quantity_compare(Term) :-
    compound(Term),
    functor(Term, quantity_compare, 2),
    arg(1, Term, Actual),
    arg(2, Term, Bound),
    ( shape_quantity(Actual)
    ; compound(Actual),
      functor(Actual, var, 1),
      arg(1, Actual, Number),
      integer(Number)
    ),
    shape_quantity_bound(Bound).

data_atom_vars(Term, Vars) :-
    ( functor(Term, pred, 2) ->
        arg(2, Term, Args),
        argument_vars(Args, Vars)
    ; functor(Term, temporal, 3) ->
        arg(2, Term, Event),
        arg(3, Term, Anchor),
        arg(2, Event, Args),
        argument_vars(Args, EventVars),
        anchor_vars(Anchor, AnchorVars),
        append(EventVars, AnchorVars, Vars)
    ; functor(Term, temporal_window, 4),
      arg(2, Term, Event),
      arg(3, Term, Anchor),
      arg(2, Event, Args),
      argument_vars(Args, EventVars),
      anchor_vars(Anchor, AnchorVars),
      append(EventVars, AnchorVars, Vars)
    ).

argument_vars([], []).
argument_vars([Arg|Args], Vars) :-
    ( compound(Arg), functor(Arg, var, 1) ->
        arg(1, Arg, Number),
        Vars = [Number|Rest]
    ; Vars = Rest
    ),
    argument_vars(Args, Rest).

anchor_vars(Anchor, Vars) :-
    arg(1, Anchor, Arg),
    ( compound(Arg), functor(Arg, var, 1) ->
        arg(1, Arg, Number),
        Vars = [Number]
    ; Vars = []
    ).

quantity_compare_vars(Term, Vars) :-
    arg(1, Term, Actual),
    ( compound(Actual), functor(Actual, var, 1) ->
        arg(1, Actual, Number),
        Vars = [Number]
    ; Vars = []
    ).

ground_argument(Arg) :-
    shape_ground_argument(Arg),
    ground(Arg).

ground_data_atom(Term) :-
    shape_data_atom(Term),
    ground(Term).

quantity_bound_parts(Term, Operator, Endpoint, Value, Unit) :-
    shape_quantity_bound(Term),
    arg(1, Term, Operator),
    arg(2, Term, Endpoint),
    arg(3, Term, Quantity),
    arg(1, Quantity, IntegerTerm),
    arg(2, Quantity, UnitTerm),
    arg(1, IntegerTerm, Value),
    arg(1, UnitTerm, Unit).

quantity_parts(Term, Value, Unit) :-
    shape_quantity(Term),
    arg(1, Term, IntegerTerm),
    arg(2, Term, UnitTerm),
    arg(1, IntegerTerm, Value),
    arg(1, UnitTerm, Unit).

quantity_compare_outcome(Actual, Bound, Outcome) :-
    ( quantity_parts(Actual, ActualValue, ActualUnit) ->
        quantity_bound_parts(
            Bound, Operator, _, BoundValue, BoundUnit),
        ( ActualUnit == BoundUnit ->
            ( quantity_relation_holds(
                  Operator, ActualValue, BoundValue) ->
                Outcome = true
            ; Outcome = false
            )
        ; Outcome = cross_unit(ActualUnit, BoundUnit)
        )
    ; Outcome = invalid_actual(Actual)
    ).

quantity_relation_holds(Operator, Left, Right) :-
    ( Operator == eq -> Left =:= Right
    ; Operator == geq -> Left >= Right
    ; Operator == leq -> Left =< Right
    ; Operator == greater -> Left > Right
    ; Operator == less -> Left < Right
    ).

temporal_window_error(Term, Error) :-
    arg(4, Term, Interval),
    arg(1, Interval, Lower),
    arg(2, Interval, Upper),
    quantity_bound_parts(
        Lower, _, LowerEndpoint, LowerValue, LowerUnit),
    quantity_bound_parts(
        Upper, _, UpperEndpoint, UpperValue, UpperUnit),
    ( LowerUnit \== UpperUnit ->
        Error = cross_unit(LowerUnit, UpperUnit)
    ; LowerValue > UpperValue ->
        Error = reversed_interval(LowerValue, UpperValue)
    ; LowerValue =:= UpperValue,
      ( LowerEndpoint \== closed ; UpperEndpoint \== closed ) ->
        Error = empty_interval(LowerValue)
    ).

data_atom_key(Term, Key) :-
    ( functor(Term, pred, 2) ->
        arg(1, Term, Name),
        arg(2, Term, Args),
        length(Args, Arity),
        Key = pred(Name, Arity)
    ; functor(Term, temporal, 3) ->
        arg(1, Term, Relation),
        arg(2, Term, Event),
        arg(1, Event, Name),
        arg(2, Event, Args),
        length(Args, Arity),
        Key = temporal(Relation, Name, Arity)
    ; functor(Term, temporal_window, 4),
      arg(1, Term, Direction),
      arg(2, Term, Event),
      arg(1, Event, Name),
      arg(2, Event, Args),
      length(Args, Arity),
      Key = temporal_window(Direction, Name, Arity)
    ).

:- module(validation_common, [
    branch_shape_detail/2,
    shape_optional_branch/2,
    terms_pairwise_distinct/1
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

:- module(drs_canon, [canonical_line/2]).

:- set_prolog_flag(encoding, utf8).

/* Validate, number variables left-to-right, and emit one canonical line. */
canonical_line(Term, Line) :-
    canonical_term(Term),
    numbervars(Term, 0, _),
    with_output_to(string(Line),
        ( write_term(Term,
              [ quoted(true),
                ignore_ops(true),
                numbervars(true),
                character_escapes(true)
              ]),
          put_code(46),
          put_code(10)
        )).

canonical_term(Term) :-
    acyclic_term(Term),
    term_attvars(Term, AttVars),
    AttVars == [],
    canonical_tree(Term).

canonical_tree(Term) :-
    var(Term),
    !.
canonical_tree([]) :-
    !.
canonical_tree(Term) :-
    atom(Term),
    !.
canonical_tree(Term) :-
    integer(Term),
    !.
canonical_tree(Term) :-
    float(Term),
    !.
canonical_tree(Term) :-
    compound(Term),
    functor(Term, Name, Arity),
    \+ ( Name == '$VAR', Arity =:= 1 ),
    canonical_args(1, Arity, Term).

canonical_args(Index, Arity, _) :-
    Index > Arity,
    !.
canonical_args(Index, Arity, Term) :-
    arg(Index, Term, Arg),
    canonical_tree(Arg),
    Next is Index + 1,
    canonical_args(Next, Arity, Term).

:- module(drs_canon, [canonical_line/2]).

/* Term → quoted, operator-free, left-to-right-numbered bytes; suffix = ".\n". */
canonical_line(Term, Line) :-
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

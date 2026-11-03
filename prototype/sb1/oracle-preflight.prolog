:- use_module('producer.prolog').
:- use_module(library(time)).

sb1_batch_main :-
    current_prolog_flag(argv, [Root, Input]),
    directory_file_path(Root, 'guidelines/schema.pl', Schema),
    load_files(sb1_world:Schema, [silent(true), if(true)]),
    setup_call_cleanup(open(Input, read, Stream, [encoding(utf8)]), sb1_batch_read(Stream), close(Stream)),
    halt.

sb1_batch_read(Stream) :-
    read_term(Stream, Term, [syntax_errors(error)]),
    ( Term == end_of_file -> true
    ; Term = sb1_case(Id, Identity, Codes),
      catch((call_with_time_limit(5, sb1_batch_case(Identity, Codes, Model, Claims)) -> true ; throw(goal_failed)), Error, true),
      ( var(Error)
      -> format('B\t~w~nM~n', [Id]), sb1_producer:sb1_write_model(Model),
         format('C~n'), sb1_producer:sb1_write_certificate(Identity, Claims), format('E\t~w~n', [Id])
      ; format('F\t~w\t', [Id]), sb1_producer:sb1_write_term(Error), nl ),
      sb1_batch_read(Stream)
    ).
sb1_batch_case(_, Codes, Model, Claims) :-
    sb1_producer:sb1_reset,
    sb1_batch_clauses(Codes, 1, Clauses),
    forall(member(clause(_, H, B, _), Clauses), sb1_producer:sb1_assert(sb1_world, H, B)),
    sb1_producer:sb1_evaluate(Model), sb1_producer:sb1_certify(Clauses, Model, Claims).
sb1_batch_clauses([], _, []).
sb1_batch_clauses([Code0|Codes], Ordinal, [clause(1-Ordinal, H, B, Vars)|Clauses]) :-
    copy_term(Code0, Code), ( Code = (H :- B) -> true ; H = Code, B = true ),
    sb1_producer:sb1_nonlocal(H, B, Vars), sb1_producer:sb1_validate_body(B),
    sb1_producer:sb1_safe(B, Vars, [], Bound), term_variables(H, HeadVars), sb1_producer:sb1_bound(HeadVars, Bound),
    Next is Ordinal + 1, sb1_batch_clauses(Codes, Next, Clauses).

:- use_module(library(crypto)).

main :-
    current_prolog_flag(argv, [Schema,Model,Query]),
    load_files(Schema,[silent(true)]),
    setup_call_cleanup(open(Model,read,S), load_model(S), close(S)),
    ( Query == 'q-all' -> all_answers
    ; setup_call_cleanup(open(Query,read,Q), read_query(Q,Id,Goal,Answers), close(Q)),
      crypto_file_hash(Query,Hash,[algorithm(sha256)]),
      ( Answers == [] -> (call(Goal) -> Result=yes ; Result=no)
      ; answer_values(Answers,Values), findall(sol(Values),Goal,Rows),
        sort(Rows,Sorted), Result=solutions(Sorted)
      ),
      format('% ~w answered against the loaded composition by ckc queries; do not edit.~n',[Id]),
      line('$guideline_answers'(v1,Id,query_sha256(Hash),result(Result)))
    ), halt.

load_model(S) :-
    read_term(S,T,[]),
    ( T == end_of_file -> true ; assertz(T), assertz(model_fact(T)), load_model(S) ).

all_answers :-
    findall(box(C,O,Payload),
        ( model_fact(guideline_operator(actual,C,O)), memberchk(O,[should,must,may]),
          findall(F,(model_fact(F),arg(1,F,X),X==C),P), sort(P,Payload)
        ), Rows),
    sort(Rows,Sorted), maplist(line,Sorted).

read_query(S,Id,Goal,Answers) :-
    read_term(S,'$guideline_query'(v2,Id,_,_,_),[]),
    read_term(S,'$guideline_query_projection'(goal(Goal),answers(Answers)),[]).

answer_values([],[]).
answer_values([answer(V,_)|As],[V|Vs]) :- answer_values(As,Vs).
line(T) :- write_term(T,[quoted(true),numbervars(true),character_escapes(true),ignore_ops(true)]),write('.'),nl.

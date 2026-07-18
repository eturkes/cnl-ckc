:- module(adapter, [main/0]).

:- use_module(drs_canon, [canonical_line/2]).

:- multifile user:message_hook/3.

user:message_hook(_, Level, _) :-
    nb_current(cnl_ckc_adapter_load_capture, true),
    ( Level == error -> nb_setval(cnl_ckc_adapter_load_failed, true)
    ; true
    ).

/*
Run: swipl -q -f none -F none -s src/prolog/adapter.pl -g main -t 'halt(9)' -- <ape-tree-dir>
I/O: UTF-8 ACE stdin → accepted canonical DRS stdout; rejection stdout = 0 bytes.
Exit: 0=accepted; 1=ape_messages|empty_drs; 2=usage|ape_load|uncaught.
Error: stderr = one canonical adapter_error(Class, Detail) line.
*/
main :-
    catch(run, Error, emit_error(uncaught, Error, 2)).

run :-
    set_stream(user_input, encoding(utf8)),
    set_stream(user_output, encoding(utf8)),
    set_stream(user_error, encoding(utf8)),
    current_prolog_flag(argv, Argv),
    require_tree_arg(Argv, Tree),
    load_ape(Tree),
    read_string(user_input, _, Input),
    atom_string(Text, Input),
    ( ace_to_drs:acetext_to_drs(Text, off, off, _Sentences, _SyntaxTrees,
          Drs, Messages, _Time) ->
        accept_or_reject(Drs, Messages)
    ; throw(error(ape_call_failed, context(adapter:run/0, Text)))
    ).

require_tree_arg([Tree], Tree) :-
    !.
require_tree_arg(Argv, _) :-
    emit_error(usage, argv(Argv), 2).

load_ape(Tree) :-
    catch(load_ape_checked(Tree), Error, emit_error(ape_load, Error, 2)).

load_ape_checked(Tree) :-
    directory_file_path(Tree, 'prolog/parser/ace_to_drs.pl', Parser),
    ( load_ape_module(Parser) ->
        true
    ; throw(error(ape_load_failed(Parser), context(adapter:load_ape/1, Tree)))
    ),
    ( current_predicate(ace_to_drs:acetext_to_drs/8) ->
        true
    ; throw(error(existence_error(procedure, ace_to_drs:acetext_to_drs/8),
          context(adapter:load_ape/1, Parser)))
    ).

load_ape_module(Parser) :-
    setup_call_cleanup(
        ( nb_setval(cnl_ckc_adapter_load_capture, true),
          nb_setval(cnl_ckc_adapter_load_failed, false)
        ),
        use_module(Parser, [acetext_to_drs/8]),
        finish_ape_load(Parser)).

finish_ape_load(Parser) :-
    nb_getval(cnl_ckc_adapter_load_failed, Failed),
    nb_setval(cnl_ckc_adapter_load_capture, false),
    ( Failed == false -> true
    ; throw(error(ape_load_errors(Parser), context(adapter:load_ape/1, Parser)))
    ).

accept_or_reject(_Drs, Messages) :-
    Messages \== [],
    !,
    emit_error(ape_messages, Messages, 1).
accept_or_reject(Drs, []) :-
    Drs == drs([], []),
    !,
    emit_error(empty_drs, Drs, 1).
accept_or_reject(Drs, []) :-
    ( Drs = drs(Domain, Conditions),
      is_list(Domain),
      is_list(Conditions) ->
        canonical_line(Drs, Output),
        format(user_output, '~s', [Output]),
        halt(0)
    ; throw(error(invalid_drs(Drs),
          context(adapter:accept_or_reject/2, invalid_shape)))
    ).

emit_error(Class, Detail, Status) :-
    ( catch(canonical_line(adapter_error(Class, Detail), Output), _, fail) ->
        true
    ; fallback_error_line(Class, Output)
    ),
    format(user_error, '~s', [Output]),
    halt(Status).

fallback_error_line(ape_messages, "adapter_error(ape_messages,unserializable).\n") :- !.
fallback_error_line(empty_drs, "adapter_error(empty_drs,unserializable).\n") :- !.
fallback_error_line(usage, "adapter_error(usage,unserializable).\n") :- !.
fallback_error_line(ape_load, "adapter_error(ape_load,unserializable).\n") :- !.
fallback_error_line(uncaught, "adapter_error(uncaught,unserializable).\n") :- !.
fallback_error_line(_, "adapter_error(uncaught,unserializable).\n").

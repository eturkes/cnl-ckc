:- module(adapter, [main/0]).

:- set_prolog_flag(encoding, utf8).

:- use_module(library(readutil), [read_stream_to_codes/2]).
:- use_module(drs_canon, [canonical_line/2]).

:- meta_predicate quarantined_call(+, +, +, 0).

:- multifile user:message_hook/3.

user:message_hook(_, Level, _) :-
    nb_current(cnl_ckc_adapter_load_capture, true),
    ( ( Level == warning ; Level == error ) ->
        nb_setval(cnl_ckc_adapter_load_failed, true)
    ; true
    ).

/*
Run: swipl -q -f none -F none -s src/prolog/adapter.pl -g main -t 'halt(9)' -- <ape-tree-dir>
  or swipl -q -f none -F none -s src/prolog/adapter.pl -g main -t 'halt(9)' -- <ape-tree-dir> <ulex-file>
I/O: ACE stdin = strict RFC 3629 UTF-8 bytes; success stdout = 1 canonical line, stderr = 0.
Reject: stdout = 0 bytes; stderr = 1 canonical adapter_error(Class, Detail) line.
Ulex: strict UTF-8; physical EOF required; lexicon messages reject before ACE input.
DRS: acyclic; no attvars or '$VAR'/1; atom|integer|float atomics; validate before numbervars.
Isolation: empty vendor stdin; discard APE-load/Ulex/parse output; load warning|error -> ape_load/2.
Exit: 0=accepted; 1=input_utf8|ape_messages|empty_drs; 2=usage|ape_load|ulex_load|uncaught.
*/
main :-
    current_input(Input),
    current_output(Output),
    stream_property(ErrorStream, alias(user_error)),
    catch(run(Input, Output, ErrorStream),
        Error,
        emit_error(ErrorStream, uncaught, Error, 2)).

run(Input, Output, ErrorStream) :-
    prompt(_, ''),
    set_stream(Output, encoding(utf8)),
    set_stream(ErrorStream, encoding(utf8)),
    set_stream(Input, type(binary)),
    prompt(_, ''),
    current_prolog_flag(argv, Argv),
    require_args(Argv, Tree, Ulex, ErrorStream),
    load_ape(Tree, Input, Output, ErrorStream),
    maybe_load_ulex(Ulex, Input, Output, ErrorStream),
    read_input(Input, ErrorStream, Text),
    ( quarantined_call(Input, Output, ErrorStream,
          ace_to_drs:acetext_to_drs(Text, off, off, _Sentences, _SyntaxTrees,
              Drs, Messages, _Time)) ->
        accept_or_reject(Drs, Messages, Output, ErrorStream)
    ; throw(error(ape_call_failed, context(adapter:run/3, Text)))
    ).

read_input(Input, ErrorStream, Text) :-
    catch(
        ( read_utf8_input(Input, Text) ->
            true
        ; throw(error(input_utf8_failed,
              context(adapter:read_input/3, plain_failure)))
        ),
        Error,
        emit_error(ErrorStream, input_utf8, Error, 1)).

read_utf8_input(Input, Text) :-
    read_stream_to_codes(Input, Bytes),
    decode_utf8(Bytes, Codes, 0),
    atom_codes(Text, Codes).

decode_utf8([], [], _).
decode_utf8([Byte|Bytes], [Code|Codes], Offset) :-
    ( decode_utf8_unit(Byte, Bytes, Code, Rest, Width) ->
        Next is Offset + Width,
        decode_utf8(Rest, Codes, Next)
    ; throw(error(syntax_error(invalid_utf8),
          context(adapter:read_utf8_input/2, byte_offset(Offset))))
    ).

decode_utf8_unit(Byte, Bytes, Byte, Bytes, 1) :-
    Byte >= 0x00,
    Byte =< 0x7f.
decode_utf8_unit(Byte0, [Byte1|Bytes], Code, Bytes, 2) :-
    Byte0 >= 0xc2,
    Byte0 =< 0xdf,
    continuation_byte(Byte1),
    Code is ((Byte0 /\ 0x1f) << 6) \/ (Byte1 /\ 0x3f).
decode_utf8_unit(Byte0, [Byte1, Byte2|Bytes], Code, Bytes, 3) :-
    Byte0 >= 0xe0,
    Byte0 =< 0xef,
    continuation_byte(Byte1),
    continuation_byte(Byte2),
    Code is ((Byte0 /\ 0x0f) << 12) \/
        ((Byte1 /\ 0x3f) << 6) \/ (Byte2 /\ 0x3f),
    Code >= 0x0800,
    \+ ( Code >= 0xd800, Code =< 0xdfff ).
decode_utf8_unit(Byte0, [Byte1, Byte2, Byte3|Bytes], Code, Bytes, 4) :-
    Byte0 >= 0xf0,
    Byte0 =< 0xf4,
    continuation_byte(Byte1),
    continuation_byte(Byte2),
    continuation_byte(Byte3),
    Code is ((Byte0 /\ 0x07) << 18) \/
        ((Byte1 /\ 0x3f) << 12) \/
        ((Byte2 /\ 0x3f) << 6) \/ (Byte3 /\ 0x3f),
    Code >= 0x10000,
    Code =< 0x10ffff.

continuation_byte(Byte) :-
    Byte >= 0x80,
    Byte =< 0xbf.

require_args([Tree], Tree, none, _) :-
    !.
require_args([Tree, File], Tree, file(File), _) :-
    !.
require_args(Argv, _, _, ErrorStream) :-
    emit_error(ErrorStream, usage, argv(Argv), 2).

maybe_load_ulex(none, _, _, _).
maybe_load_ulex(file(File), Input, Output, ErrorStream) :-
    catch(
        ( quarantined_call(Input, Output, ErrorStream,
              load_ulex_checked(File, Messages)) ->
            true
        ; throw(error(ulex_load_failed(File),
              context(adapter:maybe_load_ulex/4, plain_failure)))
        ),
        Error,
        emit_error(ErrorStream, ulex_load, Error, 2)),
    ( Messages == [] ->
        true
    ; emit_error(ErrorStream, ape_messages, Messages, 1)
    ).

load_ulex_checked(File, Messages) :-
    ulex:discard_ulex,
    read_utf8_file(File, Text),
    error_logger:clear_messages(lexicon),
    setup_call_cleanup(
        open_string(Text, Stream),
        ( ulex:read_ulex(Stream),
          error_logger:get_messages_with_type(lexicon, InitialMessages),
          ensure_ulex_consumed(Stream, InitialMessages),
          error_logger:get_messages_with_type(lexicon, Messages)
        ),
        close(Stream)).

read_utf8_file(File, Text) :-
    setup_call_cleanup(
        open(File, read, Stream, [type(binary)]),
        read_utf8_input(Stream, Text),
        close(Stream)).

ensure_ulex_consumed(_, Messages) :-
    Messages \== [],
    !.
ensure_ulex_consumed(Stream, []) :-
    stream_property(Stream, end_of_stream(End)),
    ( ( End == at ; End == past ) ->
        true
    ; error_logger:add_error_message_once(
          lexicon, '', 'Malformed entry.',
          'The end_of_file term is not allowed.')
    ).

load_ape(Tree, Input, Output, ErrorStream) :-
    catch(
        ( quarantined_call(Input, Output, ErrorStream,
              load_ape_checked(Tree)) ->
            true
        ; throw(error(ape_load_failed(Tree),
              context(adapter:load_ape/4, plain_failure)))
        ),
        Error,
        emit_error(ErrorStream, ape_load, Error, 2)).

load_ape_checked(Tree) :-
    directory_file_path(Tree, 'prolog/parser/ace_to_drs.pl', Parser),
    ( load_ape_module(Parser) ->
        true
    ; throw(error(ape_load_failed(Parser), context(adapter:load_ape/4, Tree)))
    ),
    ( current_predicate(ace_to_drs:acetext_to_drs/8) ->
        true
    ; throw(error(existence_error(procedure, ace_to_drs:acetext_to_drs/8),
          context(adapter:load_ape/4, Parser)))
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
    ; throw(error(ape_load_errors(Parser), context(adapter:load_ape/4, Parser)))
    ).

quarantined_call(Input, Output, ErrorStream, Goal) :-
    setup_call_cleanup(
        prompt(OldPrompt, ''),
        setup_call_cleanup(
            open_string("", EmptyInput),
            setup_call_cleanup(
                open_null_stream(NullOutput),
                setup_call_cleanup(
                    open_null_stream(NullError),
                    setup_call_cleanup(
                        set_prolog_IO(EmptyInput, NullOutput, NullError),
                        once(Goal),
                        set_prolog_IO(Input, Output, ErrorStream)),
                    close_quietly(NullError)),
                close_quietly(NullOutput)),
            close_quietly(EmptyInput)),
        prompt(_, OldPrompt)).

close_quietly(Stream) :-
    catch(close(Stream), _, true).

accept_or_reject(_Drs, Messages, _, ErrorStream) :-
    Messages \== [],
    !,
    emit_error(ErrorStream, ape_messages, Messages, 1).
accept_or_reject(Drs, [], _, ErrorStream) :-
    Drs == drs([], []),
    !,
    emit_error(ErrorStream, empty_drs, Drs, 1).
accept_or_reject(Drs, [], OutputStream, _) :-
    ( Drs = drs(Domain, Conditions),
      is_list(Domain),
      is_list(Conditions) ->
        true
    ; throw(error(invalid_drs(Drs),
          context(adapter:accept_or_reject/4, invalid_shape)))
    ),
    ( canonical_line(Drs, Output) ->
        format(OutputStream, '~s', [Output]),
        halt(0)
    ; throw(error(invalid_drs(Drs),
          context(adapter:accept_or_reject/4, invalid_term)))
    ).

emit_error(ErrorStream, Class, Detail, Status) :-
    ( catch(canonical_line(adapter_error(Class, Detail), Output), _, fail) ->
        true
    ; fallback_error_line(Class, Output)
    ),
    format(ErrorStream, '~s', [Output]),
    halt(Status).

fallback_error_line(input_utf8, "adapter_error(input_utf8,unserializable).\n") :- !.
fallback_error_line(ape_messages, "adapter_error(ape_messages,unserializable).\n") :- !.
fallback_error_line(empty_drs, "adapter_error(empty_drs,unserializable).\n") :- !.
fallback_error_line(usage, "adapter_error(usage,unserializable).\n") :- !.
fallback_error_line(ape_load, "adapter_error(ape_load,unserializable).\n") :- !.
fallback_error_line(ulex_load, "adapter_error(ulex_load,unserializable).\n") :- !.
fallback_error_line(uncaught, "adapter_error(uncaught,unserializable).\n") :- !.
fallback_error_line(_, "adapter_error(uncaught,unserializable).\n").

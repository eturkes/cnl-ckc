:- module(ir_tool, [main/0]).

:- set_prolog_flag(encoding, utf8).

:- use_module(library(readutil), [read_stream_to_codes/2]).
:- use_module(library(crypto), [crypto_data_hash/3]).
:- use_module(drs_canon, [canonical_line/2]).
:- use_module(drs_to_ir, [lower_terms/2]).
:- use_module(ir_validate, [validate_terms/1]).
:- use_module(ir_to_prolog, [compile_terms/2]).
:- use_module(inference_kernel, [run_terms/3]).
:- use_module(explanation, [validate_answer_terms/1]).

/*
Run: swipl -q -f none -F none -s src/prolog/ir_tool.pl -g main -t 'halt(9)' --
     lower|validate|compile|run
Input: strict RFC 3629 UTF-8 canonical term stream. Validate writes no bytes;
other commands write one buffered canonical record of their stage's output kind.
Failure: stdout empty; stderr is one canonical ir_tool_error(Stage,Class,Detail).
Exit: 0=success; 1=input-content rejection; 2=usage or uncaught internal error.
All stage output is captured in memory and flushed to real stdout only on success.
*/
main :-
    catch(start,
        Error,
        handle_error(cli, user_error, Error)).

start :-
    pin_flags,
    current_input(Input),
    current_output(Output),
    stream_property(ErrorStream, alias(user_error)),
    set_stream(Input, type(binary)),
    set_stream(Output, encoding(utf8)),
    set_stream(ErrorStream, encoding(utf8)),
    prompt(_, ''),
    current_prolog_flag(argv, Argv),
    error_stage(Argv, ErrorStage),
    catch(run_cli(Argv, Input, Output),
        Error,
        handle_error(ErrorStage, ErrorStream, Error)).

pin_flags :-
    set_prolog_flag(encoding, utf8),
    set_prolog_flag(double_quotes, string),
    set_prolog_flag(back_quotes, codes),
    set_prolog_flag(character_escapes, true),
    set_prolog_flag(var_prefix, false),
    set_prolog_flag(rational_syntax, compatibility),
    set_prolog_flag(prefer_rationals, false).

error_stage([validate], validate) :-
    !.
error_stage([lower], lower) :-
    !.
error_stage([compile], compile) :-
    !.
error_stage([run], run) :-
    !.
error_stage(_, cli).

run_cli([validate], Input, Output) :-
    !,
    with_output_to(string(Buffer), validate_input(Input)),
    format(Output, '~s', [Buffer]),
    flush_output(Output),
    halt(0).
run_cli([lower], Input, Output) :-
    !,
    with_output_to(string(Buffer), lower_input(Input)),
    format(Output, '~s', [Buffer]),
    flush_output(Output),
    halt(0).
run_cli([compile], Input, Output) :-
    !,
    with_output_to(string(Buffer), compile_input(Input)),
    format(Output, '~s', [Buffer]),
    flush_output(Output),
    halt(0).
run_cli([run], Input, Output) :-
    !,
    with_output_to(string(Buffer), run_input(Input)),
    format(Output, '~s', [Buffer]),
    flush_output(Output),
    halt(0).
run_cli(Argv, _, _) :-
    throw(ir_tool_failure(cli, usage, argv(Argv), 2)).

validate_input(Input) :-
    read_canonical_terms(Input, Terms),
    validate_terms(Terms).

lower_input(Input) :-
    read_canonical_terms(Input, Terms),
    lower_terms(Terms, IrTerms),
    generated_record_call(lower_serialization,
        canonical_codes(IrTerms, 1, Codes)),
    format('~s', [Codes]).

compile_input(Input) :-
    read_canonical_terms(Input, Terms),
    compile_terms(Terms, ProgramTerms),
    self_checked_canonical_codes(ProgramTerms, Codes),
    format('~s', [Codes]).

run_input(Input) :-
    read_canonical_terms(Input, Terms, Bytes),
    program_digest(Bytes, ProgramDigest),
    run_terms(Terms, ProgramDigest, ResultTerms),
    self_checked_answer_codes(ResultTerms, Codes),
    format('~s', [Codes]).

program_digest(Bytes, Digest) :-
    crypto_data_hash(
        Bytes, Digest, [algorithm(sha256), encoding(octet)]),
    ( valid_digest_atom(Digest) ->
        true
    ; throw(error(invalid_program_digest(Digest),
          context(ir_tool, program_digest)))
    ).

valid_digest_atom(Digest) :-
    atom_codes(Digest, Codes),
    length(Codes, 64),
    lower_hex_codes(Codes).

lower_hex_codes([]).
lower_hex_codes([Code|Codes]) :-
    ( Code >= 0'0, Code =< 0'9
    ; Code >= 0'a, Code =< 0'f
    ),
    lower_hex_codes(Codes).

self_checked_canonical_codes(Terms, Codes) :-
    generated_record_call(output_self_check,
        self_checked_canonical_codes_(Terms, Codes)).

self_checked_canonical_codes_(Terms, Codes) :-
    canonical_codes(Terms, 1, Codes),
    string_codes(Text, Codes),
    parse_terms(Text, Parsed),
    canonical_fixed_point(Parsed, Codes),
    ( Parsed == Terms ->
        true
    ; throw(error(generated_term_round_trip, context(ir_tool, output)))
    ).

self_checked_answer_codes(Terms, Codes) :-
    generated_record_call(output_self_check,
        self_checked_answer_codes_(Terms, Codes)).

self_checked_answer_codes_(Terms, Codes) :-
    canonical_codes(Terms, 1, Codes),
    string_codes(Text, Codes),
    parse_terms(Text, Parsed),
    canonical_fixed_point(Parsed, Codes),
    validate_answer_terms(Parsed),
    ( Parsed == Terms ->
        true
    ; throw(error(generated_term_round_trip, context(ir_tool, output)))
    ).

generated_record_call(Context, Goal) :-
    catch(Goal,
        ir_reject(Class, Detail),
        throw(error(generated_record_invalid(Class, Detail),
            context(ir_tool, Context)))).

read_canonical_terms(Input, Terms) :-
    read_canonical_terms(Input, Terms, _).

read_canonical_terms(Input, Terms, Bytes) :-
    read_utf8_input(Input, Text, Codes, Bytes),
    parse_terms(Text, Terms),
    canonical_fixed_point(Terms, Codes).

read_utf8_input(Input, Text, Codes) :-
    read_utf8_input(Input, Text, Codes, _).

read_utf8_input(Input, Text, Codes, Bytes) :-
    read_stream_to_codes(Input, Bytes),
    decode_utf8(Bytes, Codes, 0),
    string_codes(Text, Codes).

decode_utf8([], [], _).
decode_utf8([Byte|Bytes], [Code|Codes], Offset) :-
    ( decode_utf8_unit(Byte, Bytes, Code, Rest, Width) ->
        Next is Offset + Width,
        decode_utf8(Rest, Codes, Next)
    ; throw(ir_reject(input_utf8, byte_offset(Offset)))
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

parse_terms(Text, Terms) :-
    setup_call_cleanup(
        open_string(Text, Stream),
        read_terms(Stream, 1, Terms),
        close(Stream)).

read_terms(Stream, Index, Terms) :-
    catch(read_term(Stream, Term,
              [ module(ir_tool),
                syntax_errors(error),
                variable_names(_),
                singletons(_)
              ]),
        Error,
        handle_read_error(Index, Error)),
    ( Term == end_of_file ->
        Terms = []
    ; Terms = [Term|Rest],
      Next is Index + 1,
      read_terms(Stream, Next, Rest)
    ).

handle_read_error(Index, error(syntax_error(_), _)) :-
    !,
    throw(ir_reject(syntax, term(Index))).
handle_read_error(_, Error) :-
    throw(Error).

canonical_fixed_point(Terms, InputCodes) :-
    canonical_codes(Terms, 1, CanonicalCodes),
    ( InputCodes == CanonicalCodes ->
        true
    ; first_difference(InputCodes, CanonicalCodes, 0, Offset),
      throw(ir_reject(canonical, codepoint_offset(Offset)))
    ).

canonical_codes([], _, []).
canonical_codes([Term|Terms], Index, Codes) :-
    copy_term(Term, Copy),
    ( catch(canonical_record_line(Index, Copy, Line), _, fail) ->
        string_codes(Line, Here)
    ; throw(ir_reject(canonical, term(Index, unserializable)))
    ),
    Next is Index + 1,
    canonical_codes(Terms, Next, Rest),
    append(Here, Rest, Codes).

/*
Preserve forced quoting only after the complete document or answer-digest line
shape has been proved. All malformed lookalikes fall through to canonical_line.
*/
canonical_record_line(2, Term, Line) :-
    compound(Term),
    functor(Term, document, 3),
    canonical_document_line(Term, Line),
    !.
canonical_record_line(3, Term, Line) :-
    canonical_program_digest_line(Term, Line),
    !.
canonical_record_line(_, Term, Line) :-
    canonical_line(Term, Line).

canonical_program_digest_line(Term, Line) :-
    compound(Term),
    functor(Term, program, 1),
    arg(1, Term, Sha256),
    compound(Sha256),
    functor(Sha256, sha256, 1),
    arg(1, Sha256, Digest),
    atom(Digest),
    forced_quoted_atom(Digest, DigestText),
    format(string(Line), "program(sha256(~s)).\n", [DigestText]).

canonical_document_line(
        document(docid(Docid), source_sha256(SourceHash), ulex(Ulex)), Line) :-
    forced_quoted_atom(Docid, DocidText),
    forced_quoted_atom(SourceHash, SourceHashText),
    canonical_ulex_text(Ulex, UlexText),
    format(string(Line),
        "document(docid(~s),source_sha256(~s),ulex(~s)).\n",
        [DocidText, SourceHashText, UlexText]).

canonical_ulex_text(Ulex, "none") :-
    Ulex == none.
canonical_ulex_text(Ulex, Text) :-
    compound(Ulex),
    functor(Ulex, sha256, 1),
    arg(1, Ulex, Hash),
    forced_quoted_atom(Hash, HashText),
    format(string(Text), "sha256(~s)", [HashText]).

forced_quoted_atom(Atom, Text) :-
    atom(Atom),
    with_output_to(string(Canonical),
        write_term(Atom,
            [ quoted(true),
              ignore_ops(true),
              numbervars(true),
              character_escapes(true)
            ])),
    string_codes(Canonical, Codes),
    ( Codes = [0''|_] ->
        Text = Canonical
    ; format(string(Text), "'~s'", [Canonical])
    ).

first_difference([], [], Offset, Offset).
first_difference([], [_|_], Offset, Offset) :-
    !.
first_difference([_|_], [], Offset, Offset) :-
    !.
first_difference([Left|Lefts], [Right|Rights], Offset0, Offset) :-
    ( Left =:= Right ->
        Offset1 is Offset0 + 1,
        first_difference(Lefts, Rights, Offset1, Offset)
    ; Offset = Offset0
    ).

handle_error(_, ErrorStream,
        ir_tool_failure(Stage, Class, Detail, Status)) :-
    !,
    emit_error(ErrorStream, Stage, Class, Detail, Status).
handle_error(Stage, ErrorStream, ir_reject(Class, Detail)) :-
    !,
    emit_error(ErrorStream, Stage, Class, Detail, 1).
handle_error(Stage, ErrorStream, Error) :-
    emit_error(ErrorStream, Stage, uncaught, Error, 2).

emit_error(ErrorStream, Stage, Class, Detail, Status) :-
    ( catch(canonical_line(ir_tool_error(Stage, Class, Detail), Line), _, fail) ->
        true
    ; fallback_error_line(Stage, Class, Line)
    ),
    format(ErrorStream, '~s', [Line]),
    flush_output(ErrorStream),
    halt(Status).

fallback_error_line(Stage, Class, Line) :-
    ( catch(canonical_line(ir_tool_error(Stage, Class, unserializable), Line),
          _, fail) ->
        true
    ; Line = "ir_tool_error(cli,uncaught,unserializable).\n"
    ).

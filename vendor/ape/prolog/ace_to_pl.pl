% ace_to_pl.pl — ACE → plain-Prolog compiler over the APE parser.
% Added to this APE fork 2026-08-06, not part of upstream APE (GPL-3.0
% §5(a) via LGPL: modification notice + a relevant date); git history is
% the change record for every later edit.
% Copyright 2026 Emir Turkes. Derivative of APE; licensed under
% LGPL-3.0-or-later like the surrounding tree (see ../LICENSE.txt).
%
% Modes (argv after --):
%   <ape-tree-dir> <docid>            compile onto the v1 schema; ACE bytes on
%                                     stdin -> Prolog on stdout
%   <ape-tree-dir> <docid> <ulex>     compile with user lexicon file
%   ... proof                         derive + emit the proof payload (one ground
%                                     '$guideline_proof'/5 term per group) instead of
%                                     the product; same derivation check either way
%   question <ape-tree-dir> <qid> [<ulex>]
%                                     compile one ACE question (exactly one
%                                     sentence line parsing to one root question
%                                     box) into a '$guideline_query'/4 record and
%                                     one '$guideline_query_projection'/2 term:
%                                     goal(<','/2 conjunction over the v1
%                                     indicator vocabulary>) + answers([answer(
%                                     Var,noun(Noun,Class)|wh(who|what)),...]).
%                                     <qid> follows the docid grammar; rejects
%                                     ride the same canonical classes (new
%                                     unsupported details: query_expected/1,
%                                     query_sentences/1, query_unsupported/2)
%   v2|v3 <ape-tree-dir> <docid> <temporal.tsv> [<ulex>] [proof]
%   question v2|v3 <ape-tree-dir> <qid> <temporal.tsv> [<ulex>]
%                                     schema v2 (contract m7t): the v1 projection
%                                     + guideline_interval/6 and
%                                     guideline_recurrence/4 annotations read
%                                     through the temporal.tsv table; record
%                                     guideline_document/4 (question record /5)
%                                     ending temporal(sha256(H)); rejects add
%                                     unsupported temporal_shape(Why,S) and the
%                                     load class temporal_load (exit 2); schema
%                                     v3 (contract q12) adds guideline_frequency/5,
%                                     guideline_order/4 and
%                                     guideline_recurrence_window/7 (a table whose
%                                     header names another version fails
%                                     temporal_load); law = docs/REFERENCE.md
%                                     "Schema v2", "Schema v3"
% The composition consumption modes (check, aggregate-check, recursion-check,
% answer, trace) run in the verified Rust kernel as `ckc v1 <mode>`.
%
% Compile contract: stdin = strict RFC 3629 UTF-8; one ACE sentence per non-empty LF line.
% <docid> = nonempty [a-z0-9-] with no leading dash.
% Success: stdout = compiled Prolog document, stderr = 0 bytes, exit 0.
% Reject: stdout = 0 bytes; stderr = one canonical ace_to_pl_error(Class,Detail) line.
% Exit: 0=compiled; 1=input_utf8|ape_messages|empty_drs|sentence_lines|unsupported|safety|proof;
%       2=usage|ape_load|ulex_load|temporal_load|uncaught.
% v1 assurance: every v1 compile (product or proof emission) derives per-group
% witness worlds + obligations and replays them against the document's own
% clauses; an underivable obligation rejects the document (class proof).
% v1 stays frozen: a v1 compile projects onto it byte for byte, and only the
% explicit `v2` selector adds annotations. The v1 path lives under "v1 schema
% projection" near the end of this file and is documented in docs/REFERENCE.md
% "Compiled Prolog schema (v1, v2)". v1
% admits copula/ground facts and Horn rules plus modal/classical-negation
% operator wrappers (reified as guideline_operator/3 edges over context
% ids), consequent currying, antecedent Horn splits over one disjunction,
% and executable negation-as-failure for top-level antecedent NAF boxes.
% Translation is total: a sentence yields an ordered bundle of one or
% more clauses; any unrecognized shape rejects the whole document, and
% authored questions reject (class unsupported,
% question_not_supported(Form,S) with Form = wh(Tag)|universal|yesno,
% when the question is the first unsupported construct the processing
% order reaches — an earlier unsupported construct keeps its own detail).
% Questions compile only through the explicit question mode above.
% Discourse: one invocation = one document = one APE discourse. A later
% sentence may resolve a definite NP or personal pronoun against an
% accessible earlier sentence's referent; APE resolves silently and the
% product pins the resolution explicitly — the referent keeps the
% '$guideline_id' of its introducing sentence, and the resolving
% sentence's guideline_arg/guideline_pp rows cite that id verbatim, so a
% cross-sentence edge is a first-class product fact. Every non-resolving
% reference is fail-closed: an absent or inaccessible antecedent and an
% unresolved pronoun each raise an APE anaphor message (warning or
% error), and any APE message rejects the document (class ape_messages),
% so silent accommodation never reaches the product.

:- module(ace_to_pl, [main/0]).

:- set_prolog_flag(encoding, utf8).

:- use_module(library(readutil), [read_stream_to_codes/2]).
:- use_module(library(crypto), [crypto_data_hash/3]).

:- meta_predicate quarantined_call(+, +, +, 0).

:- multifile user:message_hook/3.

user:message_hook(_, Level, _) :-
    nb_current(ace_to_pl_load_capture, true),
    ( ( Level == warning ; Level == error ) ->
        nb_setval(ace_to_pl_load_failed, true)
    ; true
    ).

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
    current_prolog_flag(argv, Argv),
    dispatch(Argv, Input, Output, ErrorStream).
/* v2 | v3 (contracts m7t D2/D9, q12 D1): a leading `v2` | `v3` token selects
   the schema and names the guideline's temporal.tsv; both are reserved as a
   first token. */
schema_token(v2, 2).
schema_token(v3, 3).

dispatch([question, V, Tree, QId, Temporal], Input, Output, ErrorStream) :-
    schema_token(V, N),
    !,
    validated_docid(QId, ErrorStream),
    compile_mode(query, Tree, QId, none, file(N, Temporal), Input, Output,
        ErrorStream).
dispatch([question, V, Tree, QId, Temporal, Ulex], Input, Output,
        ErrorStream) :-
    schema_token(V, N),
    !,
    validated_docid(QId, ErrorStream),
    validated_ulex_arg(Ulex, ErrorStream),
    compile_mode(query, Tree, QId, file(Ulex), file(N, Temporal), Input, Output,
        ErrorStream).
dispatch([V, Tree, DocId, Temporal], Input, Output, ErrorStream) :-
    schema_token(V, N),
    !,
    validated_docid(DocId, ErrorStream),
    compile_mode(v1(product), Tree, DocId, none, file(N, Temporal), Input,
        Output, ErrorStream).
dispatch([V, Tree, DocId, Temporal, proof], Input, Output, ErrorStream) :-
    schema_token(V, N),
    !,
    validated_docid(DocId, ErrorStream),
    compile_mode(v1(proof), Tree, DocId, none, file(N, Temporal), Input, Output,
        ErrorStream).
dispatch([V, Tree, DocId, Temporal, Ulex], Input, Output, ErrorStream) :-
    schema_token(V, N),
    !,
    validated_docid(DocId, ErrorStream),
    validated_ulex_arg(Ulex, ErrorStream),
    compile_mode(v1(product), Tree, DocId, file(Ulex), file(N, Temporal), Input,
        Output, ErrorStream).
dispatch([V, Tree, DocId, Temporal, Ulex, proof], Input, Output,
        ErrorStream) :-
    schema_token(V, N),
    !,
    validated_docid(DocId, ErrorStream),
    validated_ulex_arg(Ulex, ErrorStream),
    compile_mode(v1(proof), Tree, DocId, file(Ulex), file(N, Temporal), Input,
        Output, ErrorStream).
dispatch([question, Tree, QId], Input, Output, ErrorStream) :-
    !,
    validated_docid(QId, ErrorStream),
    compile_mode(query, Tree, QId, none, Input, Output, ErrorStream).
dispatch([question, Tree, QId, Ulex], Input, Output, ErrorStream) :-
    !,
    validated_docid(QId, ErrorStream),
    validated_ulex_arg(Ulex, ErrorStream),
    compile_mode(query, Tree, QId, file(Ulex), Input, Output, ErrorStream).
dispatch([question|Rest], _, _, ErrorStream) :-
    !,
    emit_error(ErrorStream, usage, argv([question|Rest]), 2).
dispatch([Tree, DocId], Input, Output, ErrorStream) :-
    !,
    validated_docid(DocId, ErrorStream),
    compile_mode(v1(product), Tree, DocId, none, Input, Output, ErrorStream).
dispatch([Tree, DocId, proof], Input, Output, ErrorStream) :-
    !,
    validated_docid(DocId, ErrorStream),
    compile_mode(v1(proof), Tree, DocId, none, Input, Output, ErrorStream).
dispatch([Tree, DocId, Ulex], Input, Output, ErrorStream) :-
    !,
    validated_docid(DocId, ErrorStream),
    validated_ulex_arg(Ulex, ErrorStream),
    compile_mode(v1(product), Tree, DocId, file(Ulex), Input, Output,
        ErrorStream).
dispatch([Tree, DocId, Ulex, proof], Input, Output, ErrorStream) :-
    !,
    validated_docid(DocId, ErrorStream),
    validated_ulex_arg(Ulex, ErrorStream),
    compile_mode(v1(proof), Tree, DocId, file(Ulex), Input, Output,
        ErrorStream).
dispatch(Argv, _, _, ErrorStream) :-
    emit_error(ErrorStream, usage, argv(Argv), 2).

/* `proof` is a reserved argv token: a bare third slot spelling `proof`
   reads as the proof mode, so a user lexicon file literally named
   `proof` is unaddressable; the explicit ulex+proof form
   (`<tree> <docid> proof proof`) rejects it as usage — pass a lexicon
   under any other name. */
reserved_argv_token(proof).

validated_ulex_arg(Ulex, ErrorStream) :-
    ( reserved_argv_token(Ulex) ->
        emit_error(ErrorStream, usage, ulex_arg(Ulex), 2)
    ; true
    ).

/* Docid lands in the header term and a comment line; the grammar keeps
   both injection-free without escaping. */
validated_docid(DocId, ErrorStream) :-
    ( atom(DocId),
      atom_codes(DocId, Codes),
      Codes = [First|_],
      First =\= 0'-,
      docid_codes(Codes) ->
        true
    ; emit_error(ErrorStream, usage, docid(DocId), 2)
    ).

docid_codes([]).
docid_codes([Code|Codes]) :-
    docid_code(Code),
    docid_codes(Codes).

docid_code(Code) :- Code >= 0'a, Code =< 0'z, !.
docid_code(Code) :- Code >= 0'0, Code =< 0'9, !.
docid_code(0'-).

/* ---------- compile mode ---------- */

compile_mode(Mode, Tree, DocId, Ulex, Input, Output, ErrorStream) :-
    compile_mode(Mode, Tree, DocId, Ulex, none, Input, Output, ErrorStream).

compile_mode(Mode, Tree, DocId, Ulex, Temporal, Input, Output, ErrorStream) :-
    set_stream(Input, type(binary)),
    prompt(_, ''),
    load_ape(Tree, Input, Output, ErrorStream),
    maybe_load_ulex(Ulex, Input, Output, ErrorStream, UlexDigest),
    maybe_load_temporal(Temporal, ErrorStream),
    read_input(Input, ErrorStream, Bytes, Text),
    mode_line_check(Mode, Text, ErrorStream),
    ( quarantined_call(Input, Output, ErrorStream,
          ace_to_drs:acetext_to_drs(Text, off, off, Sentences, _SyntaxTrees,
              Drs, Messages, _Time)) ->
        accept_or_reject(Mode, DocId, Bytes, Text, UlexDigest, Sentences, Drs,
            Messages, Output, ErrorStream)
    ; throw(error(ape_call_failed, context(ace_to_pl:compile_mode/8, Text)))
    ).

/* Question mode pins its one-line law before parsing: premise
   injection through extra sentence lines dies here, ahead of any APE
   diagnostic; empty stdin reads query_sentences(0) here. Byte-non-
   empty lines count (whitespace-only included). Post-parse, the
   query sentence law (accept_or_reject) fires before the shared
   empty-DRS check, so a lone whitespace-only line reads
   query_sentences(0). Document mode keeps its post-parse
   line/sentence law. */
mode_line_check(v1(_), _, _).
mode_line_check(query, Text, ErrorStream) :-
    input_lines(Text, Lines),
    length(Lines, LineCount),
    ( LineCount =:= 1 ->
        true
    ; emit_error(ErrorStream, unsupported, query_sentences(LineCount), 1)
    ).

read_input(Input, ErrorStream, Bytes, Text) :-
    catch(
        ( read_utf8_input(Input, Bytes, Text) ->
            true
        ; throw(error(input_utf8_failed,
              context(ace_to_pl:read_input/4, plain_failure)))
        ),
        Error,
        emit_error(ErrorStream, input_utf8, Error, 1)).

read_utf8_input(Input, Bytes, Text) :-
    read_stream_to_codes(Input, Bytes),
    decode_utf8(Bytes, Codes, 0),
    atom_codes(Text, Codes).

decode_utf8([], [], _).
decode_utf8([Byte|Bytes], [Code|Codes], Offset) :-
    ( decode_utf8_unit(Byte, Bytes, Code, Rest, Width) ->
        Next is Offset + Width,
        decode_utf8(Rest, Codes, Next)
    ; throw(error(syntax_error(invalid_utf8),
          context(ace_to_pl:read_utf8_input/3, byte_offset(Offset))))
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

/* ---------- APE + ulex loading (quarantined, fail-closed) ---------- */

load_ape(Tree, Input, Output, ErrorStream) :-
    catch(
        ( quarantined_call(Input, Output, ErrorStream,
              load_ape_checked(Tree)) ->
            true
        ; throw(error(ape_load_failed(Tree),
              context(ace_to_pl:load_ape/4, plain_failure)))
        ),
        Error,
        emit_error(ErrorStream, ape_load, Error, 2)).

load_ape_checked(Tree) :-
    directory_file_path(Tree, 'prolog/parser/ace_to_drs.pl', Parser),
    ( load_ape_module(Parser) ->
        true
    ; throw(error(ape_load_failed(Parser), context(ace_to_pl:load_ape/4, Tree)))
    ),
    ( current_predicate(ace_to_drs:acetext_to_drs/8) ->
        true
    ; throw(error(existence_error(procedure, ace_to_drs:acetext_to_drs/8),
          context(ace_to_pl:load_ape/4, Parser)))
    ).

load_ape_module(Parser) :-
    setup_call_cleanup(
        ( nb_setval(ace_to_pl_load_capture, true),
          nb_setval(ace_to_pl_load_failed, false)
        ),
        use_module(Parser, [acetext_to_drs/8]),
        finish_ape_load(Parser)).

finish_ape_load(Parser) :-
    nb_getval(ace_to_pl_load_failed, Failed),
    nb_setval(ace_to_pl_load_capture, false),
    ( Failed == false -> true
    ; throw(error(ape_load_errors(Parser), context(ace_to_pl:load_ape/4, Parser)))
    ).

maybe_load_ulex(none, _, _, _, none).
maybe_load_ulex(file(File), Input, Output, ErrorStream, sha256(Digest)) :-
    catch(
        ( quarantined_call(Input, Output, ErrorStream,
              load_ulex_checked(File, Digest, Messages)) ->
            true
        ; throw(error(ulex_load_failed(File),
              context(ace_to_pl:maybe_load_ulex/5, plain_failure)))
        ),
        Error,
        emit_error(ErrorStream, ulex_load, Error, 2)),
    ( Messages == [] ->
        true
    ; emit_error(ErrorStream, ape_messages, Messages, 1)
    ).

load_ulex_checked(File, Digest, Messages) :-
    ulex:discard_ulex,
    read_utf8_file(File, Bytes, Text),
    crypto_data_hash(Bytes, Digest, [algorithm(sha256), encoding(octet)]),
    v1_scan_ulex_reserved(Text),
    error_logger:clear_messages(lexicon),
    setup_call_cleanup(
        open_string(Text, Stream),
        ( ulex:read_ulex(Stream),
          error_logger:get_messages_with_type(lexicon, InitialMessages),
          ensure_ulex_consumed(Stream, InitialMessages),
          error_logger:get_messages_with_type(lexicon, Messages)
        ),
        close(Stream)).

read_utf8_file(File, Bytes, Text) :-
    setup_call_cleanup(
        open(File, read, Stream, [type(binary)]),
        read_utf8_input(Stream, Bytes, Text),
        close(Stream)).

/* v1 ulex-wide reserved scan (P3): every entry, used or not; the first
   offender in file order parks in a global that projection rejects
   through the canonical machinery. A term that fails to re-read as
   Prolog ends the scan (APE's own reader already vetted the entry
   stream); the parsed-DRS scan stays the used-entry backstop. */
v1_scan_ulex_reserved(Text) :-
    nb_setval(ace_to_pl_ulex_reserved, none),
    setup_call_cleanup(
        open_string(Text, Stream),
        v1_scan_ulex_stream(Stream),
        close(Stream)).

v1_scan_ulex_stream(Stream) :-
    catch(read_term(Stream, Term, []), _, Term = end_of_file),
    ( Term == end_of_file ->
        true
    ; ( nb_getval(ace_to_pl_ulex_reserved, none),
        v1_ulex_reserved_detail(Term, Detail) ->
          nb_setval(ace_to_pl_ulex_reserved, Detail)
      ; true
      ),
      v1_scan_ulex_stream(Stream)
    ).

/* Every ulex category writes the surface word form in the first argument
   and the product-bound data — lemma, class, preposition — after it, so
   the scan skips that first slot: a surface form never reaches the
   compiled document, and skipping it keeps one canonical detail per
   offending entry, identical to the parsed-DRS scan's. */
v1_ulex_reserved_detail(Term, Detail) :-
    compound(Term),
    functor(Term, Name, _),
    v1_reserved_atom(Name),
    !,
    Detail = reserved_constructor_collision(Term).
v1_ulex_reserved_detail(Term, Detail) :-
    compound(Term),
    !,
    Term =.. [_, _Surface | Rest],
    member(Arg, Rest),
    v1_reserved_subterm(Arg, Detail).
v1_ulex_reserved_detail(Term, Detail) :-
    v1_reserved_subterm(Term, Detail).

v1_reserved_subterm(Term, Detail) :-
    sub_term(Sub, Term),
    nonvar(Sub),
    ( compound(Sub),
      functor(Sub, Name, _),
      v1_reserved_atom(Name) ->
        Detail = reserved_constructor_collision(Sub)
    ; atom(Sub),
      v1_reserved_atom(Sub),
      Detail = reserved_name_collision(Sub)
    ).

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

/* ---------- accept/reject ---------- */

accept_or_reject(_, _, _, _, _, _, _, Messages, _, ErrorStream) :-
    Messages \== [],
    !,
    emit_error(ErrorStream, ape_messages, Messages, 1).
accept_or_reject(query, _, _, _, _, Sentences, _, [], _, ErrorStream) :-
    length(Sentences, SentenceCount),
    SentenceCount =\= 1,
    !,
    emit_error(ErrorStream, unsupported, query_sentences(SentenceCount), 1).
accept_or_reject(_, _, _, _, _, _, Drs, [], _, ErrorStream) :-
    Drs == drs([], []),
    !,
    emit_error(ErrorStream, empty_drs, Drs, 1).
accept_or_reject(Mode, DocId, Bytes, Text, UlexDigest, Sentences, Drs, [],
        Output, ErrorStream) :-
    catch(
        ( v3_set_decls(Drs),
          mode_translate(Mode, DocId, Bytes, Text, UlexDigest, Sentences, Drs,
              OutCodes) ->
            Result = ok(OutCodes)
        ; Result = internal_failure
        ),
        ace_to_pl_reject(Class, Detail),
        Result = rejected(Class, Detail)),
    ( Result = ok(Out) ->
        format(Output, '~s', [Out]),
        halt(0)
    ; Result = rejected(RClass, RDetail) ->
        emit_error(ErrorStream, RClass, RDetail, 1)
    ; throw(error(translate_failed(DocId),
          context(ace_to_pl:accept_or_reject/10, plain_failure)))
    ).

mode_translate(v1(Emit), DocId, Bytes, Text, UlexDigest, Sentences, Drs,
        OutCodes) :-
    v1_translate_document(Emit, DocId, Bytes, Text, UlexDigest, Sentences,
        Drs, OutCodes).
mode_translate(query, QId, Bytes, Text, UlexDigest, Sentences, Drs,
        OutCodes) :-
    v1_translate_query(QId, Bytes, Text, UlexDigest, Sentences, Drs,
        OutCodes).

reject(Class, Detail) :-
    throw(ace_to_pl_reject(Class, Detail)).

/* ---------- shared translation helpers (consumed by the v1 projection) ---------- */

write_naf_goals([Goal]) :-
    !,
    write_canonical_part(Goal).
write_naf_goals([Goal|Goals]) :-
    write_canonical_part(Goal),
    write(', '),
    write_naf_goals(Goals).

write_body_literal(pos(Lit)) :-
    write_canonical_part(Lit).
write_body_literal(naf_conj([Goal])) :-
    !,
    write('\\+ '),
    write_canonical_part(Goal).
write_body_literal(naf_conj(Goals)) :-
    write('\\+ ('),
    write_naf_goals(Goals),
    write(')').

add_var(Ref, Vars0, Vars) :-
    ( strict_member(Ref, Vars0) ->
        Vars = Vars0
    ; Vars = [Ref|Vars0]
    ).

canonical_args(Index, Arity, _) :-
    Index > Arity,
    !.
canonical_args(Index, Arity, Term) :-
    arg(Index, Term, Arg),
    canonical_tree(Arg),
    Next is Index + 1,
    canonical_args(Next, Arity, Term).

cond_occurrences_args(Index, Arity, _, _, Count, Count) :-
    Index > Arity,
    !.
cond_occurrences_args(Index, Arity, Term, Var, Count0, Count) :-
    arg(Index, Term, Arg),
    cond_occurrences(Var, Arg, ArgCount),
    Count1 is Count0 + ArgCount,
    Next is Index + 1,
    cond_occurrences_args(Next, Arity, Term, Var, Count1, Count).

split_lf(Codes, [Line|Lines]) :-
    append(Line, [0'\n|Rest], Codes),
    !,
    split_lf(Rest, Lines).
split_lf(Codes, [Codes]).

/* Anchors live at condition positions only: the walk descends through
   box carriers (drs, modal/question wrappers, connectives, condition
   lists) and the anchored wrapper's own inner condition, never into
   leaf payload arguments — a payload pair shaped -(X, /(S,C)) inside
   formula/predicate/named arguments stays opaque data instead of
   minting a phantom sentence. Nonvar guards keep the walk from
   instantiating open tails (non-instantiation law). */
sub_anchor(Term, S) :-
    nonvar(Term),
    ( functor(Term, -, 2),
      arg(2, Term, Anchor),
      nonvar(Anchor),
      anchor_sentence(Anchor, S0) ->
        ( S = S0
        ; arg(1, Term, Inner),
          sub_anchor(Inner, S)
        )
    ; anchor_carrier_arg(Term, Arg),
      sub_anchor(Arg, S)
    ).

anchor_carrier_arg(Term, Arg) :-
    functor(Term, Name, Arity),
    anchor_carrier(Name, Arity),
    between(1, Arity, Index),
    arg(Index, Term, Arg).

anchor_carrier(drs, 2).
anchor_carrier(question, 1).
anchor_carrier(should, 1).
anchor_carrier(must, 1).
anchor_carrier(can, 1).
anchor_carrier(may, 1).
anchor_carrier(=>, 2).
anchor_carrier(v, 2).
anchor_carrier(-, 1).
anchor_carrier(~, 1).
anchor_carrier('[|]', 2).

take_sentence([], _, [], []).
take_sentence([Sx-Cond|Tagged], S, [Cond|Group], Rest) :-
    Sx =:= S,
    !,
    take_sentence(Tagged, S, Group, Rest).
take_sentence([Sx-Cond|Tagged], S, Group, [Sx-Cond|Rest]) :-
    take_sentence(Tagged, S, Group, Rest).

validate_emittable(Term) :-
    ( acyclic_term(Term),
      term_attvars(Term, []),
      canonical_tree(Term) ->
        true
    ; reject(unsupported, unserializable_term)
    ).

write_body([Lit]) :-
    !,
    write_body_literal(Lit).
write_body([Lit|Lits]) :-
    write_body_literal(Lit),
    write(', '),
    write_body(Lits).

write_canonical_part(Term) :-
    write_term(Term,
        [ quoted(true),
          numbervars(true),
          character_escapes(true),
          ignore_ops(true)
        ]).

input_lines(Text, Lines) :-
    atom_codes(Text, Codes),
    split_lf(Codes, RawLines),
    exclude(==([]), RawLines, Lines).

/* Tag every root condition with its sentence id. Anchor extraction
   decomposes with arg/3 and never unifies into the DRS. */

anchor_sentence(Anchor, S) :-
    nonvar(Anchor),
    functor(Anchor, /, 2),
    arg(1, Anchor, S),
    arg(2, Anchor, T),
    integer(S),
    integer(T).

/* One sentence id shared by every anchored condition inside a subterm. */
inner_sentence(Term, S) :-
    findall(Sx, sub_anchor(Term, Sx), Ss),
    sort(Ss, Sorted),
    ( Sorted = [Single] ->
        S = Single
    ; reject(unsupported, mixed_or_missing_sentence_anchors(Sorted))
    ).

/* Partition tagged conditions into per-sentence groups, preserving order
   and variable identity (no copying); every condition must land in 1..N. */
group_by_sentence(S, SentenceCount, Tagged, []) :-
    S > SentenceCount,
    !,
    ( Tagged == [] ->
        true
    ; reject(unsupported, condition_outside_sentence_range)
    ).
group_by_sentence(S, SentenceCount, Tagged, [S-Group|Groups]) :-
    take_sentence(Tagged, S, Group, Rest),
    Next is S + 1,
    group_by_sentence(Next, SentenceCount, Rest, Groups).

/* Domain lists are binders, not uses: cond_occurrences skips the domain
   argument of every drs/2 box while counting occurrences at condition
   positions. */

cond_occurrences(Var, Term, Count) :-
    ( Term == Var ->
        Count = 1
    ; var(Term) ->
        Count = 0
    ; compound(Term),
      functor(Term, drs, 2) ->
        arg(2, Term, Conds),
        cond_occurrences(Var, Conds, Count)
    ; compound(Term) ->
        functor(Term, _, Arity),
        cond_occurrences_args(1, Arity, Term, Var, 0, Count)
    ; Count = 0
    ).

strict_member(X, [Y|Ys]) :-
    ( X == Y ->
        true
    ; strict_member(X, Ys)
    ).

merge_vars([], Vars, Vars).
merge_vars([V|Vs], Vars0, Vars) :-
    add_var(V, Vars0, Vars1),
    merge_vars(Vs, Vars1, Vars).

/* ---------- rendering ---------- */

header_term(DocId, AceDigest, none,
    guideline_document(DocId, ace_sha256(AceDigest), ulex(none))).
header_term(DocId, AceDigest, sha256(Digest),
    guideline_document(DocId, ace_sha256(AceDigest), ulex(sha256(Digest)))).

render_item(rule(Head, Body)) :-
    validate_emittable(rule(Head, Body)),
    copy_term(rule(Head, Body), rule(HeadCopy, BodyCopy)),
    numbervars(rule(HeadCopy, BodyCopy), 0, _),
    write_canonical_part(HeadCopy),
    write(' :- '),
    write_body(BodyCopy),
    write('.'),
    nl.

/* Validate the original, then number and write a copy: rendering never
   instantiates variables still shared with the DRS or later items. */
render_term_line(Term) :-
    validate_emittable(Term),
    copy_term(Term, Copy),
    numbervars(Copy, 0, _),
    write_canonical_part(Copy),
    write('.'),
    nl.

/* Emittable terms: acyclic, no attvars, atom/integer/float atomics,
   no pre-existing '$VAR'/1. Numeric law: arbitrary-precision integers
   and FINITE floats, values opaque; inf/nan floats sit outside the
   frozen artifact vocabulary, so the producer refuses to mint them
   (hostile artifacts carrying them reject as unserializable). */

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
    !,
    float_class(Term, Class),
    Class \== infinite,
    Class \== nan.
canonical_tree(Term) :-
    compound(Term),
    functor(Term, Name, Arity),
    \+ ( Name == '$VAR', Arity =:= 1 ),
    canonical_args(1, Arity, Term).

/* ---------- v2 temporal vocabulary (contract m7t D1) ----------
   `temporal.tsv` maps source lemmas onto closed ids, so the projection
   names no source word. The table parks in a global the projection reads;
   none = v1. */
maybe_load_temporal(none, _) :-
    nb_setval(ace_to_pl_temporal, none).
maybe_load_temporal(file(N, File), ErrorStream) :-
    nb_setval(ace_to_pl_schema, N),
    catch(
        ( v2_read_temporal(File, Table) ->
            true
        ; throw(error(temporal_load_failed(File),
              context(ace_to_pl:maybe_load_temporal/2, plain_failure)))
        ),
        Error,
        emit_error(ErrorStream, temporal_load, Error, 2)),
    nb_setval(ace_to_pl_temporal, Table).

v2_temporal_header(2, Codes) :-
    atom_codes('# format: kind\tlemma\tvalue\n# kind: unit (value: second|minute|hour|day|week|month|year) | relation (value: duration|within|after|before) | spacing (value: frame noun lemma)\n',
        Codes).
v2_temporal_header(3, Codes) :-
    atom_codes('# format: kind\tlemma\tvalue\n# kind: unit (value: second|minute|hour|day|week|month|year) | relation (value: duration|within|after|before) | spacing (value: frame noun lemma) | window (value: frame noun lemma) | frequency (value: period) | approximation (value: about) | range (value: minimum)\n',
        Codes).

/* The header must be the selected version's (q12 D1: no cross-version read). */
v2_read_temporal(File, table(N, Units, Relations, Spacings, Windows, Frequencies,
        Digest)) :-
    nb_getval(ace_to_pl_schema, N),
    setup_call_cleanup(
        open(File, read, Stream, [type(binary)]),
        read_stream_to_codes(Stream, Bytes),
        close(Stream)),
    crypto_data_hash(Bytes, Digest, [algorithm(sha256), encoding(octet)]),
    v2_temporal_header(N, Header),
    ( append(Header, Body, Bytes) -> true ; v2_bad(header) ),
    ( Body \== [] -> true ; v2_bad(no_rows) ),
    ( append(Lines, [0'\n], Body) -> true ; v2_bad(final_newline) ),
    split_lf(Lines, Rows),
    v2_rows(Rows, 1, t(N, [], [], [], [], []),
        t(N, Units, Relations, Spacings, Windows, Frequencies)).

v2_bad(Why) :-
    throw(error(temporal(Why), context(ace_to_pl:v2_read_temporal/2, _))).

v2_rows([], _, T, T).
v2_rows([Row|Rows], N, T0, T) :-
    v2_row(Row, N, T0, T1),
    N1 is N + 1,
    v2_rows(Rows, N1, T1, T).

/* One row; checks run in the spec's order (fields, lemma, kind, value,
   duplicates). A noun is a unit or a frame, never both; a v3 frequency
   preposition is never a relation preposition. */
v2_row(Row, N, t(Ver, U0, R0, S0, W0, F0), T) :-
    v2_split_tab(Row, Fields),
    ( Fields = [K, L, V] -> true ; v2_bad(row(N, field_count)) ),
    ( v2_lemma(L, Lemma) -> true ; v2_bad(row(N, lemma)) ),
    atom_codes(Kind, K),
    ( Kind == unit ->
        ( v2_closed_id(V, [second, minute, hour, day, week, month, year], Unit) ->
            true
        ; v2_bad(row(N, unit_id))
        ),
        ( ( memberchk(Lemma-_, U0) ; memberchk(_-Lemma, S0) ; memberchk(_-Lemma, W0) ) ->
            v2_bad(row(N, duplicate_noun))
        ; true
        ),
        append(U0, [Lemma-Unit], U),
        T = t(Ver, U, R0, S0, W0, F0)
    ; Kind == relation ->
        ( v2_closed_id(V, [duration, within, after, before], Role) ->
            true
        ; v2_bad(row(N, role_id))
        ),
        ( ( memberchk(Lemma-_, R0) ; memberchk(Lemma, F0) ) ->
            v2_bad(row(N, duplicate_preposition))
        ; true
        ),
        append(R0, [Lemma-Role], R),
        T = t(Ver, U0, R, S0, W0, F0)
    ; ( Kind == spacing ; Ver =:= 3, Kind == window ) ->
        ( v2_lemma(V, Frame) -> true ; v2_bad(row(N, frame_lemma)) ),
        ( ( memberchk(Lemma-Frame, S0) ; memberchk(Lemma-Frame, W0)
          ; memberchk(Frame-_, U0) ) ->
            v2_bad(row(N, duplicate_noun))
        ; true
        ),
        ( Kind == spacing ->
            append(S0, [Lemma-Frame], S),
            T = t(Ver, U0, R0, S, W0, F0)
        ; append(W0, [Lemma-Frame], W),
          T = t(Ver, U0, R0, S0, W, F0)
        )
    ; Ver =:= 3, Kind == frequency ->
        ( atom_codes(period, V) -> true ; v2_bad(row(N, frequency_value)) ),
        ( ( memberchk(Lemma, F0) ; memberchk(Lemma-_, R0) ) ->
            v2_bad(row(N, duplicate_preposition))
        ; true
        ),
        append(F0, [Lemma], F),
        T = t(Ver, U0, R0, S0, W0, F)
    ; v2_bad(row(N, kind))
    ).

v2_split_tab(Codes, [Field|Fields]) :-
    ( append(Field, [0'\t|Rest], Codes) ->
        v2_split_tab(Rest, Fields)
    ; Field = Codes,
      Fields = []
    ).

/* A lemma field = nonempty UTF-8 bytes, none of them a space, control or
   DEL byte; Atom = its decoded text (spec temporal.rs lemma_ok). */
v2_lemma([B|Bs], Atom) :-
    forall(member(X, [B|Bs]), ( X > 0x20, X =\= 0x7f )),
    catch(decode_utf8([B|Bs], Codes, 0), _, fail),
    atom_codes(Atom, Codes).

v2_closed_id(Codes, Ids, Id) :-
    member(Id, Ids),
    atom_codes(Id, Codes),
    !.

v2_table(T) :-
    nb_current(ace_to_pl_temporal, table(V, U, R, S, W, F, _)),
    T = t(V, U, R, S, W, F).

/* ---------- v2 annotation pass (contract m7t D4/D5) ----------
   One scope = one flat item list; a NAF payload is its own scope. A
   pattern's conditions share their context term (sub-list nesting is
   irrelevant). The annotation item follows its pp; v1_condition renders
   it. Reads the DRS through arg/3 + == (non-instantiation law). */
v2_annotate(S, Items0, Items) :-
    ( v2_table(T) ->
        v2_annotate_scope(T, S, Items0, Items)
    ; Items = Items0
    ).

v2_annotate_scope(T, S, Items0, Items) :-
    v2_anns(Items0, T, S, Items0, Anns),
    v2_claims(Anns, Claims),
    ( v2_dup(Claims) ->
        reject(unsupported, temporal_shape(shared_quantity, S))
    ; true
    ),
    ( v3_windows_ok(Anns, Anns) ->
        true
    ; reject(unsupported, temporal_shape(window_shape, S))
    ),
    v2_consumed(Anns, Links),
    v2_rebuild(Items0, T, S, Items0, Anns, Links, Items).

v2_anns([], _, _, _, []).
v2_anns([Item|Items], T, S, Scope, Anns) :-
    ( v2_pp_item(Item, Ctx, PP) ->
        v2_pp_ann(T, S, Scope, Ctx, PP, Ann),
        ( Ann == none -> Anns = Rest ; Anns = [Ctx-Ann|Rest] )
    ; Anns = Rest
    ),
    v2_anns(Items, T, S, Scope, Rest).

v2_pp_item(Item, Ctx, PP) :-
    nonvar(Item),
    functor(Item, anchored, 2),
    arg(1, Item, Ctx),
    arg(2, Item, PP),
    nonvar(PP),
    functor(PP, modifier_pp, 3).

v2_pp_ann(T, S, Scope, Ctx, PP, Ann) :-
    T = t(_, U, R, Sp, W, _),
    append(Sp, W, Frames),
    arg(1, PP, E),
    arg(2, PP, P),
    arg(3, PP, X),
    v2_of_links(Scope, Ctx, X, Links),
    length(Links, NL),
    ( atom(P), memberchk(P-Role, R), v2_qty(U, Scope, Ctx, X, Unit, O) ->
        ( v2_bound_why(O, Why) -> reject(unsupported, temporal_shape(Why, S)) ; true ),
        ( NL > 1 ->
            reject(unsupported, temporal_shape(anchor_count, S))
        ; NL =:= 0 ->
            Ann = interval(E, Role, X, Unit, none)
        ; Role == duration ->
            reject(unsupported, temporal_shape(duration_anchor, S))
        ; Links = [A],
          ( ( nonvar(A)
            ; v2_qty(U, Scope, Ctx, A, _, _)
            ; v2_is_frame(Frames, Scope, Ctx, A)
            ) ->
              reject(unsupported, temporal_shape(anchor_shape, S))
          ; Ann = interval(E, Role, X, Unit, A)
          )
        )
    ; v2_obj_of(Scope, Ctx, X, F) ->
        v2_noun(F, FL),
        ( atom(P), memberchk(P-FL, Sp) ->
            ( arg(3, F, countable), arg(4, F, na), arg(5, F, eq), arg(6, F, 1) ->
                true
            ; reject(unsupported, temporal_shape(frame_shape, S))
            ),
            ( NL =:= 1 -> true ; reject(unsupported, temporal_shape(anchor_count, S)) ),
            Links = [Q],
            ( v2_qty(U, Scope, Ctx, Q, Unit, O) ->
                true
            ; reject(unsupported, temporal_shape(frame_shape, S))
            ),
            ( v2_bound_why(O, Why) -> reject(unsupported, temporal_shape(Why, S)) ; true ),
            v2_of_links(Scope, Ctx, Q, QLinks),
            ( QLinks == [] -> true ; reject(unsupported, temporal_shape(anchor_count, S)) ),
            Ann = recurrence(E, X, Q, Unit)
        ; v3_pp_ann(T, S, Scope, Ctx, PP, Ann)
        )
    ; v3_pp_ann(T, S, Scope, Ctx, PP, Ann)
    ).

/* The first same-context object(V, …) condition. */
v2_obj_of([Item|Items], Ctx, V, O) :-
    ( nonvar(Item),
      functor(Item, anchored, 2),
      arg(1, Item, C),
      C == Ctx,
      arg(2, Item, Inner),
      nonvar(Inner),
      functor(Inner, object, 6),
      arg(1, Inner, Ref),
      Ref == V ->
        O = Inner
    ; v2_obj_of(Items, Ctx, V, O)
    ).

v2_of_links([], _, _, []).
v2_of_links([Item|Items], Ctx, V, Links) :-
    ( nonvar(Item),
      functor(Item, anchored, 2),
      arg(1, Item, C),
      C == Ctx,
      arg(2, Item, Inner),
      nonvar(Inner),
      functor(Inner, relation, 3),
      arg(1, Inner, X),
      X == V,
      arg(2, Inner, Of),
      Of == of ->
        arg(3, Inner, Y),
        Links = [Y|Rest]
    ; Links = Rest
    ),
    v2_of_links(Items, Ctx, V, Rest).

v2_noun(O, L) :-
    arg(2, O, Noun),
    ( atom(Noun) -> L = Noun ; L = '' ).

v2_qty(U, Scope, Ctx, V, Unit, O) :-
    v2_obj_of(Scope, Ctx, V, O),
    v2_noun(O, L),
    memberchk(L-Unit, U).

v2_is_frame(Sp, Scope, Ctx, V) :-
    v2_obj_of(Scope, Ctx, V, O),
    v2_noun(O, L),
    memberchk(_-L, Sp).

/* A quantity's bound: a count noun with a comparison and a count >= 1. */
v2_bound_why(O, Why) :-
    arg(3, O, Class),
    arg(4, O, UnitField),
    arg(5, O, Op),
    arg(6, O, N),
    ( \+ ( Class == countable, UnitField == na, Op \== na ) ->
        Why = no_bound
    ; integer(N) ->
        N < 1,
        Why = zero_bound
    ; Why = no_bound
    ).

v2_claims([], []).
v2_claims([Ctx-interval(_, _, Q, _, _)|Anns], [Ctx-Q|Claims]) :-
    v2_claims(Anns, Claims).
v2_claims([Ctx-recurrence(_, F, Q, _)|Anns], [Ctx-F, Ctx-Q|Claims]) :-
    v2_claims(Anns, Claims).
v2_claims([Ctx-window(_, F, L, _, _)|Anns], [Ctx-F, Ctx-L|Claims]) :-
    v2_claims(Anns, Claims).
v2_claims([Ctx-frequency(_, C, W, _)|Anns], [Ctx-C, Ctx-W|Claims]) :-
    v2_claims(Anns, Claims).
v2_claims([_-order(_, _, _)|Anns], Claims) :-
    v2_claims(Anns, Claims).

v2_dup([C|Cs]) :-
    ( strict_member(C, Cs) -> true ; v2_dup(Cs) ).

v2_consumed([], []).
v2_consumed([Ctx-interval(_, _, Q, _, A)|Anns], Links) :-
    ( var(A) -> Links = [Ctx-Q|Rest] ; Links = Rest ),
    v2_consumed(Anns, Rest).
v2_consumed([Ctx-recurrence(_, F, _, _)|Anns], [Ctx-F|Links]) :-
    v2_consumed(Anns, Links).
v2_consumed([Ctx-window(_, F, L, _, _)|Anns], [Ctx-F, Ctx-L|Links]) :-
    v2_consumed(Anns, Links).
v2_consumed([_-frequency(_, _, _, _)|Anns], Links) :-
    v2_consumed(Anns, Links).
v2_consumed([_-order(_, _, _)|Anns], Links) :-
    v2_consumed(Anns, Links).

v2_rebuild(_, _, _, [], _, _, []).
v2_rebuild(Scope, T, S, [Item|Items], Anns, Links, Out) :-
    v2_rebuild_item(Scope, T, S, Item, Anns, Links, Head),
    v2_rebuild(Scope, T, S, Items, Anns, Links, Tail),
    append(Head, Tail, Out).

v2_rebuild_item(Scope, T, S, Item, Anns, Links, Out) :-
    ( v2_pp_item(Item, Ctx, PP) ->
        v2_pp_ann(T, S, Scope, Ctx, PP, Ann),
        ( Ann == none ->
            Out = [Item]
        ; v2_ann_inner(Ann, Anns, Ctx, Inner) ->
            Out = [Item, anchored(Ctx, Inner)]
        ; Out = [Item]
        )
    ; nonvar(Item),
      functor(Item, anchored, 2),
      arg(1, Item, Ctx),
      arg(2, Item, Inner),
      nonvar(Inner),
      functor(Inner, relation, 3),
      arg(2, Inner, Of),
      Of == of,
      arg(1, Inner, X),
      v2_linked(Links, Ctx, X) ->
        Out = []
    ; nonvar(Item),
      functor(Item, naf, 2) ->
        arg(1, Item, Dom),
        arg(2, Item, Payload),
        v2_annotate_scope(T, S, Payload, Payload2),
        Out = [naf(Dom, Payload2)]
    ; Out = [Item]
    ).

v2_linked([C-X0|Links], Ctx, X) :-
    ( C == Ctx, X0 == X -> true ; v2_linked(Links, Ctx, X) ).

/* The reserved item an annotation adds after its pp; a recurrence under a
   window becomes a scoped recurrence (q12 D6); a window adds none. */
v2_ann_inner(interval(E, Role, Q, Unit, A), _, _,
    '$guideline_interval'(E, Role, Q, Unit, A)).
v2_ann_inner(recurrence(E, _, Q, Unit), Anns, Ctx, Inner) :-
    ( v3_window_of(Anns, Ctx, E, L, LUnit, A) ->
        Inner = '$guideline_recurrence_window'(E, Q, Unit, A, L, LUnit)
    ; Inner = '$guideline_recurrence'(E, Q, Unit)
    ).
v2_ann_inner(frequency(E, C, W, Unit), _, _, '$guideline_frequency'(E, C, W, Unit)).
v2_ann_inner(order(E, Role, A), _, _, '$guideline_order'(E, Role, A)).

/* ---------- v3 patterns (contract q12 D3-D6) ----------
   Referents classify by their object condition anywhere in the DRS (the
   declarations park in a backtrackable global, so DRS variables keep their
   identity); window, frequency, then order follow the v2 patterns. */
v3_set_decls(Drs) :-
    v3_objects(Drs, Objects, []),
    v3_decls(Objects, Decls),
    b_setval(ace_to_pl_decls, Decls).

v3_objects(T, Os, Rest) :-
    ( var(T) ->
        Os = Rest
    ; compound(T) ->
        ( functor(T, object, 6) ->
            Os = [T|Rest]
        ; T =.. [_|Args],
          v3_objects_list(Args, Os, Rest)
        )
    ; Os = Rest
    ).

v3_objects_list([], Os, Os).
v3_objects_list([A|As], Os, Rest) :-
    v3_objects(A, Os, Mid),
    v3_objects_list(As, Mid, Rest).

v3_decls([], []).
v3_decls([O|Os], Decls) :-
    arg(1, O, V),
    ( var(V) ->
        v2_noun(O, Noun),
        ( v2_bound_why(O, _) -> Bounded = false ; Bounded = true ),
        Decls = [V-d(Noun, Bounded)|Rest]
    ; Decls = Rest
    ),
    v3_decls(Os, Rest).

v3_decl(V, d(Noun, Bounded)) :-
    b_getval(ace_to_pl_decls, Decls),
    v3_decl_in(Decls, V, Noun, Bounded).

v3_decl_in([V0-d(N0, B0)|Ds], V, Noun, Bounded) :-
    ( V0 == V ->
        Noun = N0,
        Bounded = B0
    ; v3_decl_in(Ds, V, Noun, Bounded)
    ).

/* A plain referent: a variable whose noun is neither a unit nor a frame noun. */
v3_plain(t(_, U, _, Sp, W, _), X) :-
    var(X),
    v3_decl(X, d(Noun, _)),
    atom(Noun),
    Noun \== '',
    \+ memberchk(Noun-_, U),
    \+ memberchk(_-Noun, Sp),
    \+ memberchk(_-Noun, W).

v3_frame_shaped(O) :-
    arg(3, O, Class), Class == countable,
    arg(4, O, UnitField), UnitField == na,
    arg(5, O, Op), Op == eq,
    arg(6, O, N), N == 1.

v3_pp_ann(T, S, Scope, Ctx, PP, Ann) :-
    T = t(Ver, _, R, _, W, F),
    ( Ver =:= 3 ->
        arg(1, PP, E),
        arg(2, PP, P),
        arg(3, PP, X),
        ( atom(P), memberchk(P-Role, R), memberchk(Role, [before, after]) ->
            Ordered = Role
        ; Ordered = none
        ),
        ( v2_obj_of(Scope, Ctx, X, O) ->
            v2_noun(O, NL),
            ( atom(P), memberchk(P-NL, W) ->
                v3_window(T, S, Scope, Ctx, PP, O, Ann)
            ; atom(P), memberchk(P, F) ->
                v3_frequency(T, S, Scope, Ctx, PP, Ann)
            ; Ordered \== none, v3_plain(T, X) ->
                Ann = order(E, Ordered, X)
            ; Ann = none
            )
        ; Ordered \== none, v3_plain(T, X) ->
            Ann = order(E, Ordered, X)
        ; Ann = none
        )
    ; Ann = none
    ).

/* D6: the only window pp on E, F frame-shaped, F of L (a time quantity), L of A
   (a plain referent). */
v3_window(T, S, Scope, Ctx, PP, O, window(E, F, L, Unit, A)) :-
    T = t(_, U, _, _, _, _),
    arg(1, PP, E),
    arg(3, PP, F),
    v2_of_links(Scope, Ctx, F, FL),
    v3_window_pps(Scope, Scope, T, Ctx, E, NW),
    ( NW =< 1, v3_frame_shaped(O), FL = [L] ->
        true
    ; reject(unsupported, temporal_shape(window_shape, S))
    ),
    ( v2_qty(U, Scope, Ctx, L, Unit, LO) ->
        true
    ; reject(unsupported, temporal_shape(window_shape, S))
    ),
    ( v2_bound_why(LO, Why) -> reject(unsupported, temporal_shape(Why, S)) ; true ),
    v2_of_links(Scope, Ctx, L, AL),
    ( AL = [A], v3_plain(T, A) ->
        true
    ; reject(unsupported, temporal_shape(window_shape, S))
    ).

/* D5: W an unanchored exact period; the counted item = E's second participant. */
v3_frequency(T, S, Scope, Ctx, PP, Ann) :-
    T = t(_, U, _, _, _, _),
    arg(1, PP, E),
    arg(3, PP, W),
    ( v2_qty(U, Scope, Ctx, W, Unit, WO) ->
        ( v2_bound_why(WO, Why) -> reject(unsupported, temporal_shape(Why, S)) ; true ),
        arg(5, WO, Op),
        v2_of_links(Scope, Ctx, W, WL),
        ( Op == eq, WL == [] ->
            true
        ; reject(unsupported, temporal_shape(frequency_shape, S))
        ),
        ( v3_counted(T, Scope, Ctx, E, C) ->
            true
        ; reject(unsupported, temporal_shape(frequency_shape, S))
        ),
        Ann = frequency(E, C, W, Unit)
    ; Ann = none
    ).

v3_pred_of([Item|Items], Ctx, E, Pred) :-
    ( nonvar(Item),
      functor(Item, anchored, 2),
      arg(1, Item, C0),
      C0 == Ctx,
      arg(2, Item, Inner),
      nonvar(Inner),
      ( functor(Inner, predicate, 4) ; functor(Inner, predicate, 5) ),
      arg(1, Inner, E0),
      E0 == E ->
        Pred = Inner
    ; v3_pred_of(Items, Ctx, E, Pred)
    ).

v3_counted(T, Scope, Ctx, E, C) :-
    v3_pred_of(Scope, Ctx, E, Pred),
    arg(4, Pred, C0),
    var(C0),
    v3_decl(C0, d(_, true)),
    v3_plain(T, C0),
    C = C0.

v3_window_of([C0-window(E0, _, L0, U0, A0)|_], Ctx, E, L, Unit, A) :-
    C0 == Ctx,
    E0 == E,
    !,
    L = L0,
    Unit = U0,
    A = A0.
v3_window_of([_|Anns], Ctx, E, L, Unit, A) :-
    v3_window_of(Anns, Ctx, E, L, Unit, A).

/* D6: the same-context window pps on E among Items (Scope = the whole scope). */
v3_window_pps([], _, _, _, _, 0).
v3_window_pps([Item|Items], Scope, T, Ctx, E, N) :-
    v3_window_pps(Items, Scope, T, Ctx, E, N0),
    T = t(_, _, _, _, W, _),
    (   nonvar(Item),
        functor(Item, anchored, 2),
        arg(1, Item, C0),
        C0 == Ctx,
        arg(2, Item, Inner),
        nonvar(Inner),
        functor(Inner, modifier_pp, 3),
        arg(1, Inner, E0),
        E0 == E,
        arg(2, Inner, P),
        atom(P),
        arg(3, Inner, X),
        v2_obj_of(Scope, Ctx, X, O),
        v2_noun(O, NL),
        memberchk(P-NL, W)
    ->  N is N0 + 1
    ;   N = N0
    ).

/* D6: each window's event carries >= 1 recurrence in the same context. */
v3_windows_ok([], _).
v3_windows_ok([Ctx-Ann|Anns], All) :-
    ( Ann = window(E, _, _, _, _) ->
        v3_count(All, Ctx, E, recurrence, NR),
        NR >= 1
    ; true
    ),
    v3_windows_ok(Anns, All).

v3_count([], _, _, _, 0).
v3_count([C0-Ann|Anns], Ctx, E, Kind, N) :-
    v3_count(Anns, Ctx, E, Kind, N0),
    ( C0 == Ctx, functor(Ann, Kind, _), arg(1, Ann, E0), E0 == E ->
        N is N0 + 1
    ; N = N0
    ).

/* Schema version of the current compile: 2 iff a temporal table loaded. */
v2_version(V) :-
    ( v2_table(t(V0, _, _, _, _, _)) -> V = V0 ; V = 1 ).

v2_indicators(Indicators) :-
    ( v2_table(_) ->
        Indicators = [
            guideline_schema_version/1,
            guideline_document/4,
            guideline_entity/4,
            guideline_cardinality/5,
            guideline_event/3,
            guideline_arg/4,
            guideline_pp/4,
            guideline_property/4,
            guideline_operator/3,
            guideline_interval/6,
            guideline_recurrence/4
        |V3]
    ; v1_indicators(Indicators)
    ),
    ( v2_version(3) ->
        V3 = [
            guideline_frequency/5,
            guideline_order/4,
            guideline_recurrence_window/7,
            guideline_range/3
        ]
    ; V3 = []
    ).

/* v2 records end in temporal(sha256(H)). */
v2_record(Record0, Record) :-
    ( nb_current(ace_to_pl_temporal, table(_, _, _, _, _, _, Digest)) ->
        Record0 =.. List0,
        append(List0, [temporal(sha256(Digest))], List),
        Record =.. List
    ; Record = Record0
    ).

/* ---------- v1 schema projection (frozen) ----------

   The sole compile path. Closed reserved vocabulary: source lemmas
   (nouns/verbs/adjectives/prepositions) are opaque data atoms, never
   predicate functors. M3.2 operator boxes use generated context ids plus
   guideline_operator/3 edges; antecedent edges remain existential joins.
   Consequent-local referents Skolemize to ground
   '$guideline_id'(product, DocId, S, ref(N), Deps) terms, Deps = the
   concatenation of every curried antecedent segment's domain in DRS
   order (a Horn-split variant appends its arm's domain). Referent
   ordinal N = position in the sentence's first-occurrence enumeration
   over referent slots in condition order (document-repeat referents
   occupy positions; minted identities use their position; NAF payload
   slots count in place; the Horn split numbers from the UNSPLIT
   traversal, so both variants mint the same ref(N) distinguished by
   Deps). Every clause of a rule sentence repeats one identical
   rendered body per Horn-split variant. Top-level antecedent NAF
   renders executable \+ over the box's expansion; every other NAF
   position rejects. Reject details introduced by this section:
   operator_scoped_rule(Op,S), disjunctive_root(S),
   disjunctive_antecedent(S), disjunctive_consequent(S),
   forbidden_operator(Pos,naf,S), deferred_operator(Pos,Op,S),
   naf_shape(Conds), naf_variable_not_bound, naf_local_escape,
   invalid_drs_shape, condition_shape(C), sentence_shape(Conds).
   See README section "Compiled Prolog schema (v1)". */

/* Uniform declaration block (F2): every v1 document declares all nine
   indicators regardless of population. */
v1_indicators([
    guideline_schema_version/1,
    guideline_document/3,
    guideline_entity/4,
    guideline_cardinality/5,
    guideline_event/3,
    guideline_arg/4,
    guideline_pp/4,
    guideline_property/4,
    guideline_operator/3
]).

v1_translate_document(Emit, DocId, Bytes, Text, UlexDigest, Sentences, Drs,
        OutCodes) :-
    ( nonvar(Drs),
      functor(Drs, drs, 2),
      arg(1, Drs, Dom),
      is_list(Dom),
      arg(2, Drs, Conds),
      is_list(Conds) ->
        true
    ; reject(unsupported, invalid_drs_shape)
    ),
    length(Sentences, SentenceCount),
    input_lines(Text, Lines),
    length(Lines, LineCount),
    ( LineCount =:= SentenceCount ->
        true
    ; reject(sentence_lines, counts(lines(LineCount), sentences(SentenceCount)))
    ),
    crypto_data_hash(Bytes, AceDigest, [algorithm(sha256), encoding(octet)]),
    v1_ulex_reserved_check,
    v1_collision_scan(Conds),
    v1_tag_conditions(Conds, Tagged),
    group_by_sentence(1, SentenceCount, Tagged, Groups),
    v1_translate_groups(Groups, DocId, [], Bundles),
    v1_derive_proofs(Bundles, DocId, Payload),
    ( Emit == product ->
        v1_render_document(DocId, AceDigest, UlexDigest, Lines, Bundles,
            OutCodes)
    ; v1_render_payload(Payload, OutCodes)
    ).

/* The ulex-wide reserved scan runs at load time (every entry, used or
   not) and parks its first offender for the compile path to reject
   inside the canonical reject machinery. */
v1_ulex_reserved_check :-
    ( nb_current(ace_to_pl_ulex_reserved, Detail),
      Detail \== none ->
        reject(unsupported, Detail)
    ; true
    ).

/* v1 root tagging covers plain roots plus reified unary operator boxes.
   Nested anchors still select exactly one source sentence, preserving
   the common grouping/provenance machinery. */
v1_tag_conditions([], []).
v1_tag_conditions([Cond|Conds], [S-Tagged|Rest]) :-
    v1_tag_condition(Cond, S, Tagged),
    v1_tag_conditions(Conds, Rest).

v1_tag_condition(Cond, S, anchored(Inner)) :-
    nonvar(Cond),
    functor(Cond, -, 2),
    arg(1, Cond, Inner),
    arg(2, Cond, Anchor),
    anchor_sentence(Anchor, S),
    nonvar(Inner),
    !.
v1_tag_condition(Cond, S, rule(Ante, Cons)) :-
    nonvar(Cond),
    functor(Cond, =>, 2),
    arg(1, Cond, Ante),
    arg(2, Cond, Cons),
    !,
    inner_sentence(Cond, S).
v1_tag_condition(Cond, S, question(QDrs)) :-
    nonvar(Cond),
    functor(Cond, question, 1),
    arg(1, Cond, QDrs),
    !,
    inner_sentence(Cond, S).
v1_tag_condition(Cond, S, boxed(Cond)) :-
    nonvar(Cond),
    functor(Cond, Op, 1),
    memberchk(Op, ['-', should, must, can, may]),
    !,
    inner_sentence(Cond, S).
v1_tag_condition(Cond, S, _) :-
    nonvar(Cond),
    functor(Cond, ~, 1),
    !,
    inner_sentence(Cond, S),
    reject(unsupported, forbidden_operator(root, naf, S)).
v1_tag_condition(Cond, _, _) :-
    reject(unsupported, root_condition(Cond)).

/* Reserved vocabulary: any source lemma beginning `guideline_` or
   `$guideline_`, and any source compound whose functor name begins
   either prefix (any arity), rejects the whole document (first offender
   in depth-first term order). */
v1_collision_scan(Conds) :-
    ( v1_collision(Conds, Detail) ->
        reject(unsupported, Detail)
    ; true
    ).

v1_collision(Term, Detail) :-
    sub_term(Sub, Term),
    nonvar(Sub),
    ( compound(Sub),
      functor(Sub, Name, _),
      v1_reserved_atom(Name) ->
        Detail = reserved_constructor_collision(Sub)
    ; v1_lemma_slot(Sub, Lemma),
      atom(Lemma),
      v1_reserved_atom(Lemma),
      Detail = reserved_name_collision(Lemma)
    ).

v1_lemma_slot(Sub, Lemma) :-
    functor(Sub, object, 6),
    arg(2, Sub, Lemma).
v1_lemma_slot(Sub, Lemma) :-
    functor(Sub, predicate, Arity),
    Arity >= 3,
    arg(2, Sub, Lemma).
v1_lemma_slot(Sub, Lemma) :-
    functor(Sub, property, 3),
    arg(2, Sub, Lemma).
v1_lemma_slot(Sub, Lemma) :-
    functor(Sub, modifier_pp, 3),
    arg(2, Sub, Lemma).

v1_reserved_atom(Atom) :-
    ( sub_atom(Atom, 0, _, _, guideline_) ->
        true
    ; sub_atom(Atom, 0, _, _, '$guideline_')
    ).

/* Sentence groups are first-class IR (P2): a bundle holds an ordered
   list of group(Kind, K, WitnessPairs, Clauses) — the root fact
   cluster-set = one fact group, each Horn variant = one rule group.
   WitnessPairs = Var-WitnessId assignments over the group's positive
   body (P3-P4); the renderer flattens groups in order, so emitted
   bytes stay identical to the pre-IR pipeline. */
v1_translate_groups([], _, _, []).
v1_translate_groups([S-Group|Groups], DocId, Map0,
        [bundle(S, SGroups)|Bundles]) :-
    v1_sentence(Group, S, DocId, Map0, Map, SGroups),
    v1_groups_clause_count(SGroups, ClauseCount),
    ( ClauseCount =:= 0 ->
        reject(unsupported, sentence_shape(Group))
    ; true
    ),
    v1_translate_groups(Groups, DocId, Map, Bundles).

v1_groups_clause_count([], 0).
v1_groups_clause_count([group(_, _, _, Clauses)|Groups], Count) :-
    length(Clauses, Head),
    v1_groups_clause_count(Groups, Tail),
    Count is Head + Tail.

v1_sentence(Group, S, DocId, Map0, Map, SGroups) :-
    ( Group = [rule(Ante, Cons)] ->
        Map = Map0,
        v1_rule(Ante, Cons, S, DocId, Map0, SGroups)
    ; Group = [question(QDrs)] ->
        query_form(QDrs, Form),
        reject(unsupported, question_not_supported(Form, S))
    ; v1_root_group(Group) ->
        v1_fact_bundle(Group, S, DocId, Map0, Map, Clauses),
        SGroups = [group(fact, 1, [], Clauses)]
    ; reject(unsupported, sentence_shape(Group))
    ).

v1_root_group([]).
v1_root_group([anchored(_)|Items]) :-
    v1_root_group(Items).
v1_root_group([boxed(_)|Items]) :-
    v1_root_group(Items).

/* ---------- v1 facts ---------- */

v1_fact_bundle(Group, S, DocId, Map0, Map, Clauses) :-
    v1_flatten_items(Group, root, S, DocId, [], actual, none, 1, _, Items0),
    v2_annotate(S, Items0, Items),
    v1_ref_slots(Items, Slots),
    v1_first_occurrence(Slots, [], Ordered),
    v1_mint_ordinals(Ordered, 1, S, DocId, Map0, Map),
    v1_expand_items(Items, Map, [], Heads),
    maplist(v1_fact_clause, Heads, Clauses),
    maplist(v1_check_safety, Clauses).

v1_fact_clause(Head, clause(Head, [])).

v1_mint_ordinals([], _, _, _, Map, Map).
v1_mint_ordinals([V|Vs], N, S, DocId, Map0, Map) :-
    ( v1_lookup(Map0, V, _) ->
        Map1 = Map0
    ; append(Map0, [V-'$guideline_id'(product, DocId, S, ref(N), [])], Map1)
    ),
    N2 is N + 1,
    v1_mint_ordinals(Vs, N2, S, DocId, Map1, Map).

/* ---------- v1 rules ---------- */

/* Rules: consequent currying first (P3) — a singleton bare implication
   inside an empty-domain consequent box folds its antecedent into the
   body, recursively; then the Horn split (P5) when the collected
   top-level antecedent conditions hold exactly one v/2 disjunction. */
v1_rule(Ante, Cons, S, DocId, Map, SGroups) :-
    v1_curry(Ante, Cons, Segs, FinalCons),
    v1_box(FinalCons, _CDom, CConds),
    v1_seg_domain(Segs, ADom),
    v1_split_scan(Segs, Shared, Vs),
    ( Vs == [] ->
        v1_plain_rule(Segs, CConds, ADom, S, DocId, Map, SGroups)
    ; Vs = [v(Arm1, Arm2)] ->
        v1_split_rule(Shared, Arm1, Arm2, CConds, ADom, S, DocId, Map,
            SGroups)
    ; reject(unsupported, disjunctive_antecedent(S))
    ).

v1_curry(Ante, Cons, [seg(ADom, AConds)|Segs], FinalCons) :-
    v1_box(Ante, ADom, AConds),
    ( v1_curry_step(Cons, Ante2, Cons2) ->
        v1_curry(Ante2, Cons2, Segs, FinalCons)
    ; Segs = [],
      FinalCons = Cons
    ).

/* Curry candidate: singleton bare implication under an EMPTY
   intermediate domain (decomposed with arg/3 + ==; a non-empty domain
   or sibling conditions fall through to the ordinary flatten rejects). */
v1_curry_step(Cons, Ante2, Cons2) :-
    nonvar(Cons),
    functor(Cons, drs, 2),
    arg(1, Cons, CDom),
    CDom == [],
    arg(2, Cons, CConds),
    nonvar(CConds),
    CConds = [Single|Rest],
    Rest == [],
    nonvar(Single),
    functor(Single, =>, 2),
    arg(1, Single, Ante2),
    arg(2, Single, Cons2).

v1_seg_domain([], []).
v1_seg_domain([seg(Dom, _)|Segs], ADom) :-
    v1_seg_domain(Segs, Tail),
    append(Dom, Tail, ADom).

/* Top-level v/2 scan across curried segments (P5): shared conditions
   keep DRS order; exactly one v splits, two or more reject. */
v1_split_scan([], [], []).
v1_split_scan([seg(_, Conds)|Segs], Shared, Vs) :-
    v1_split_conds(Conds, SharedHead, VsHead),
    v1_split_scan(Segs, SharedTail, VsTail),
    append(SharedHead, SharedTail, Shared),
    append(VsHead, VsTail, Vs).

v1_split_conds([], [], []).
v1_split_conds([C|Cs], Shared, [v(Arm1, Arm2)|Vs]) :-
    nonvar(C),
    functor(C, v, 2),
    !,
    arg(1, C, Arm1),
    arg(2, C, Arm2),
    v1_split_conds(Cs, Shared, Vs).
v1_split_conds([C|Cs], [C|Shared], Vs) :-
    v1_split_conds(Cs, Shared, Vs).

v1_plain_rule(Segs, CConds, ADom, S, DocId, Map, SGroups) :-
    v1_seg_items(Segs, S, DocId, 1, N1, AItems0),
    v2_annotate(S, AItems0, AItems),
    v1_flatten_items(CConds, consequent, S, DocId, ADom, actual, none,
        N1, _, CItems0),
    v2_annotate(S, CItems0, CItems),
    v1_ante_refs(AItems, CItems, Map, Ordered, AnteRefs),
    v1_cons_locals(Ordered, AnteRefs, Map, ConsLocals),
    v1_skolem_map(ConsLocals, Ordered, ADom, S, DocId, Sko),
    v1_variant(AItems, CItems, Map, Sko, S, Clauses),
    v1_witness_pairs(AItems, Ordered, Map, DocId, S, 1, Pairs),
    SGroups = [group(rule, 1, Pairs, Clauses)].

v1_seg_items([], _, _, N, N, []).
v1_seg_items([seg(_, Conds)|Segs], S, DocId, N0, N, Items) :-
    v1_flatten_items(Conds, antecedent, S, DocId, [], actual, none, N0,
        N1, Head),
    v1_seg_items(Segs, S, DocId, N1, N, Tail),
    append(Head, Tail, Items).

/* Horn split (P5): variant k's body = shared conditions then arm-k, its
   Skolem Deps = outer domain then arm-k domain; referent ordinals and
   box numbers come from the one UNSPLIT traversal — shared, arm 1,
   arm 2, consequent — so both variants mint identical ref(N)/box(B)
   values, distinguished by Deps alone. Bundle = variant-1 clauses then
   variant-2 clauses under one sentence comment. */
v1_split_rule(Shared, Arm1, Arm2, CConds, ADom, S, DocId, Map, SGroups) :-
    v1_arm_box(Arm1, S, Dom1, Conds1),
    v1_arm_box(Arm2, S, Dom2, Conds2),
    v1_flatten_items(Shared, antecedent, S, DocId, [], actual, none,
        1, NS, SharedItems0),
    v2_annotate(S, SharedItems0, SharedItems),
    v1_flatten_items(Conds1, antecedent, S, DocId, [], actual, none,
        NS, NA1, Arm1Items0),
    v2_annotate(S, Arm1Items0, Arm1Items),
    v1_flatten_items(Conds2, antecedent, S, DocId, [], actual, none,
        NA1, NA2, Arm2Items0),
    v2_annotate(S, Arm2Items0, Arm2Items),
    append(ADom, Dom1, Deps1),
    append(ADom, Dom2, Deps2),
    v1_flatten_items(CConds, consequent, S, DocId, Deps1, actual, none,
        NA2, _, CItems10),
    v2_annotate(S, CItems10, CItems1),
    v1_flatten_items(CConds, consequent, S, DocId, Deps2, actual, none,
        NA2, _, CItems20),
    v2_annotate(S, CItems20, CItems2),
    append(Arm1Items, Arm2Items, ArmItems),
    append(SharedItems, ArmItems, AnteAll),
    v1_ante_refs(AnteAll, CItems1, Map, Ordered, AnteRefs),
    v1_cons_locals(Ordered, AnteRefs, Map, ConsLocals),
    v1_skolem_map(ConsLocals, Ordered, Deps1, S, DocId, Sko1),
    v1_skolem_map(ConsLocals, Ordered, Deps2, S, DocId, Sko2),
    append(SharedItems, Arm1Items, Ante1),
    append(SharedItems, Arm2Items, Ante2),
    v1_variant(Ante1, CItems1, Map, Sko1, S, Clauses1),
    v1_variant(Ante2, CItems2, Map, Sko2, S, Clauses2),
    v1_witness_pairs(Ante1, Ordered, Map, DocId, S, 1, Pairs1),
    v1_witness_pairs(Ante2, Ordered, Map, DocId, S, 2, Pairs2),
    SGroups = [group(rule, 1, Pairs1, Clauses1),
               group(rule, 2, Pairs2, Clauses2)].

v1_arm_box(Arm, S, _, _) :-
    nonvar(Arm),
    functor(Arm, v, 2),
    !,
    reject(unsupported, disjunctive_antecedent(S)).
v1_arm_box(Arm, _, Dom, Conds) :-
    v1_box(Arm, Dom, Conds).

v1_ante_refs(AItems, CItems, _, Ordered, AnteRefs) :-
    append(AItems, CItems, AllItems),
    v1_ref_slots(AllItems, Slots),
    v1_ref_slots(AItems, AnteSlots),
    v1_first_occurrence(Slots, [], Ordered),
    v1_first_occurrence(AnteSlots, [], AnteRefs).

v1_variant(AItems, CItems, Map, Sko, S, Clauses) :-
    v1_expand_items(AItems, Map, Sko, Goals),
    ( Goals == [] ->
        reject(unsupported, sentence_shape(rule_without_antecedent(S)))
    ; true
    ),
    v1_expand_items(CItems, Map, Sko, Heads),
    ( Heads == [] ->
        reject(unsupported, sentence_shape(rule_without_consequent(S)))
    ; true
    ),
    v1_rule_clauses(Heads, Goals, Clauses),
    v1_naf_safety(Clauses),
    maplist(v1_check_safety, Clauses).

v1_box(Box, Dom, Conds) :-
    ( nonvar(Box),
      Box = drs(Dom, Conds),
      is_list(Dom),
      is_list(Conds) ->
        true
    ; reject(unsupported, invalid_drs_shape)
    ).

/* Consequent-local referents = sentence referents that occur in no
   antecedent payload (including operator-box locals) and are not
   document-introduced. Context Skolem dependencies remain ADom. */
v1_cons_locals([], _, _, []).
v1_cons_locals([V|Vs], AnteRefs, Map, Locals) :-
    ( ( v1_var_member(V, AnteRefs)
      ; v1_lookup(Map, V, _)
      ) ->
        Locals = Locals1
    ; Locals = [V|Locals1]
    ),
    v1_cons_locals(Vs, AnteRefs, Map, Locals1).

v1_skolem_map([], _, _, _, _, []).
v1_skolem_map([V|Vs], Ordered, ADom, S, DocId, [V-Id|Pairs]) :-
    v1_nth_var(Ordered, V, 1, N),
    Id = '$guideline_id'(product, DocId, S, ref(N), ADom),
    v1_skolem_map(Vs, Ordered, ADom, S, DocId, Pairs).

v1_rule_clauses([], _, []).
v1_rule_clauses([Head|Heads], Goals, [clause(Head, Goals)|Clauses]) :-
    v1_rule_clauses(Heads, Goals, Clauses).

/* Box conditions flatten left-to-right. Every modal/classical wrapper
   contributes one operator edge before its recursively flattened payload
   (F2); a top-level antecedent NAF box becomes one executable \+ goal
   over its payload expansion (P4). Consequent/root contexts are
   generated Skolems; antecedent contexts are body variables joined
   through guideline_operator/3. Encl = none | op(Op) | naf names the
   immediately enclosing wrapper. */
v1_flatten_items([], _, _, _, _, _, _, N, N, []).
v1_flatten_items([C|Cs], Where, S, DocId, Deps, Outer, Encl, N0, N,
        Items) :-
    v1_flatten_cond(C, Where, S, DocId, Deps, Outer, Encl, N0, N1, Head),
    v1_flatten_items(Cs, Where, S, DocId, Deps, Outer, Encl, N1, N, Tail),
    append(Head, Tail, Items).

v1_flatten_cond(C, Where, S, DocId, Deps, Outer, Encl, N0, N, Items) :-
    nonvar(C),
    is_list(C),
    !,
    v1_flatten_items(C, Where, S, DocId, Deps, Outer, Encl, N0, N, Items).
v1_flatten_cond(anchored(Inner), _, _, _, _, Outer, _, N, N,
        [anchored(Outer, Inner)]) :-
    !.
v1_flatten_cond(boxed(C), Where, S, DocId, Deps, Outer, Encl, N0, N,
        Items) :-
    !,
    v1_flatten_cond(C, Where, S, DocId, Deps, Outer, Encl, N0, N, Items).
v1_flatten_cond(C, _, _, _, _, Outer, _, N, N, [anchored(Outer, Inner)]) :-
    nonvar(C),
    functor(C, -, 2),
    arg(2, C, Anchor),
    nonvar(Anchor),
    anchor_sentence(Anchor, _),
    !,
    arg(1, C, Inner).
v1_flatten_cond(C, _, S, _, _, _, Encl, _, _, _) :-
    nonvar(C),
    functor(C, =>, 2),
    !,
    ( Encl = op(Op) ->
        reject(unsupported, operator_scoped_rule(Op, S))
    ; reject(unsupported, condition_shape(C))
    ).
v1_flatten_cond(C, Where, S, _, _, _, _, _, _, _) :-
    nonvar(C),
    functor(C, v, 2),
    !,
    v1_disjunction_reject(Where, S).
v1_flatten_cond(C, Where, S, DocId, Deps, Outer, Encl, N0, N, Items) :-
    nonvar(C),
    functor(C, ~, 1),
    !,
    arg(1, C, Box),
    v1_naf_items(Where, Encl, Box, S, DocId, Deps, Outer, N0, N, Items).
v1_flatten_cond(C, Where, S, DocId, Deps, Outer, Encl, N0, N, Items) :-
    nonvar(C),
    functor(C, Op, 1),
    memberchk(Op, ['-', should, must, can, may]),
    !,
    ( Encl == naf ->
        reject(unsupported, deferred_operator(Where, Op, S))
    ; arg(1, C, Box),
      v1_operator_items(Op, Box, C, Where, S, DocId, Deps, Outer, N0, N,
          Items)
    ).
v1_flatten_cond(C, _, _, _, _, _, _, _, _, _) :-
    reject(unsupported, condition_shape(C)).

/* The flatten item records the consumed preorder number B (bookkeeping
   for witness box(B) slots); the emitted edge stays guideline_operator/3. */
v1_operator_items(Op, Box, Original, Where, S, DocId, Deps, Outer,
        N0, N, [operator(N0, Outer, Inner, Op)|Payload]) :-
    v1_box(Box, _Dom, Conds),
    v1_operator_context(Where, DocId, S, Deps, N0, N1, Inner),
    v1_flatten_items(Conds, Where, S, DocId, Deps, Inner, op(Op), N1, N,
        Payload),
    ( v1_payload_condition(Payload) ->
        true
    ; reject(unsupported, condition_shape(Original))
    ).

/* box(B) numbering: whole-sentence preorder over operator boxes,
   antecedent wrappers included — they consume a number and mint
   nothing (their context stays an existential body variable). */
v1_operator_context(antecedent, _, _, _, N, N2, _) :-
    !,
    N2 is N + 1.
v1_operator_context(_, DocId, S, Deps, N, N2,
        '$guideline_id'(context, DocId, S, box(N), Deps)) :-
    N2 is N + 1.

v1_payload_condition([anchored(_, _)|_]) :-
    !.
v1_payload_condition([_|Items]) :-
    v1_payload_condition(Items).

/* NAF by position x enclosure (P4): top-level antecedent NAF becomes
   one executable goal; consequent NAF is forbidden; NAF nested inside
   any wrapper stays a named deferral. */
v1_naf_items(antecedent, none, Box, S, DocId, Deps, Outer, N0, N,
        [naf(NDom, Payload)]) :-
    !,
    v1_box(Box, NDom, NConds),
    v1_flatten_items(NConds, antecedent, S, DocId, Deps, Outer, naf,
        N0, N, Payload),
    ( v1_payload_condition(Payload) ->
        true
    ; reject(unsupported, naf_shape(NConds))
    ).
v1_naf_items(consequent, none, _, S, _, _, _, _, _, _) :-
    !,
    reject(unsupported, forbidden_operator(consequent, naf, S)).
v1_naf_items(Where, _, _, S, _, _, _, _, _, _) :-
    reject(unsupported, deferred_operator(Where, naf, S)).

v1_disjunction_reject(antecedent, S) :-
    reject(unsupported, disjunctive_antecedent(S)).
v1_disjunction_reject(consequent, S) :-
    reject(unsupported, disjunctive_consequent(S)).
v1_disjunction_reject(root, S) :-
    reject(unsupported, disjunctive_root(S)).

/* ---------- v1 condition expansion (shared by facts, bodies, heads) ---------- */

v1_expand_items([], _, _, []).
v1_expand_items([operator(_, Outer, Inner, Op)|Items], Map, Sko,
        [guideline_operator(Outer, Inner, Op)|Terms]) :-
    v1_expand_items(Items, Map, Sko, Terms).
v1_expand_items([naf(NDom, Payload)|Items], Map, Sko,
        [naf(NDom, Goals)|Terms]) :-
    !,
    v1_expand_items(Payload, Map, Sko, Goals),
    v1_expand_items(Items, Map, Sko, Terms).
v1_expand_items([anchored(Context, Inner)|Items], Map, Sko, Terms) :-
    v1_condition(Context, Inner, Map, Sko, Head),
    v1_expand_items(Items, Map, Sko, Tail),
    append(Head, Tail, Terms).

v1_condition(Context, Inner, Map, Sko,
        [guideline_entity(Context, Ref, Noun, Class),
         guideline_cardinality(Context, Ref, Unit, Op, Count)]) :-
    nonvar(Inner),
    functor(Inner, object, 6),
    !,
    Inner = object(Ref0, Noun, Class, Unit, Op, Count),
    v1_check_operator(Op),
    v1_ref(Ref0, Map, Sko, Ref).
v1_condition(Context, Inner, Map, Sko,
        [guideline_event(Context, E, Lemma)|ArgTerms]) :-
    nonvar(Inner),
    functor(Inner, predicate, Arity),
    Arity >= 3,
    !,
    ( Arity =< 5 ->
        true
    ; reject(unsupported, condition_shape(Inner))
    ),
    arg(1, Inner, E0),
    arg(2, Inner, Lemma),
    v1_ref(E0, Map, Sko, E),
    v1_participants(3, Arity, Inner, Map, Sko, Context, E, 1, ArgTerms).
v1_condition(Context, Inner, Map, Sko,
        [guideline_pp(Context, E, Prep, Obj)]) :-
    nonvar(Inner),
    functor(Inner, modifier_pp, 3),
    !,
    Inner = modifier_pp(E0, Prep, Obj0),
    v1_ref(E0, Map, Sko, E),
    v1_ref(Obj0, Map, Sko, Obj).
v1_condition(Context, Inner, Map, Sko,
        [guideline_property(Context, P, Lemma, pos)]) :-
    nonvar(Inner),
    functor(Inner, property, 3),
    !,
    Inner = property(P0, Lemma, Polarity),
    ( Polarity == pos ->
        true
    ; reject(unsupported, property_polarity(Polarity))
    ),
    v1_ref(P0, Map, Sko, P).
v1_condition(Context, Inner, Map, Sko,
        [guideline_interval(Context, E, Role, Q, Unit, A)]) :-
    nonvar(Inner),
    functor(Inner, '$guideline_interval', 5),
    !,
    arg(1, Inner, E0),
    arg(2, Inner, Role),
    arg(3, Inner, Q0),
    arg(4, Inner, Unit),
    arg(5, Inner, A0),
    v1_ref(E0, Map, Sko, E),
    v1_ref(Q0, Map, Sko, Q),
    ( A0 == none -> A = none ; v1_ref(A0, Map, Sko, A) ).
v1_condition(Context, Inner, Map, Sko,
        [guideline_recurrence(Context, E, Q, Unit)]) :-
    nonvar(Inner),
    functor(Inner, '$guideline_recurrence', 3),
    !,
    arg(1, Inner, E0),
    arg(2, Inner, Q0),
    arg(3, Inner, Unit),
    v1_ref(E0, Map, Sko, E),
    v1_ref(Q0, Map, Sko, Q).
v1_condition(Context, Inner, Map, Sko,
        [guideline_frequency(Context, E, C, W, Unit)]) :-
    nonvar(Inner),
    functor(Inner, '$guideline_frequency', 4),
    !,
    arg(1, Inner, E0),
    arg(2, Inner, C0),
    arg(3, Inner, W0),
    arg(4, Inner, Unit),
    v1_ref(E0, Map, Sko, E),
    v1_ref(C0, Map, Sko, C),
    v1_ref(W0, Map, Sko, W).
v1_condition(Context, Inner, Map, Sko,
        [guideline_order(Context, E, Role, A)]) :-
    nonvar(Inner),
    functor(Inner, '$guideline_order', 3),
    !,
    arg(1, Inner, E0),
    arg(2, Inner, Role),
    arg(3, Inner, A0),
    v1_ref(E0, Map, Sko, E),
    v1_ref(A0, Map, Sko, A).
v1_condition(Context, Inner, Map, Sko,
        [guideline_recurrence_window(Context, E, Q, Unit, A, L, LUnit)]) :-
    nonvar(Inner),
    functor(Inner, '$guideline_recurrence_window', 6),
    !,
    arg(1, Inner, E0),
    arg(2, Inner, Q0),
    arg(3, Inner, Unit),
    arg(4, Inner, A0),
    arg(5, Inner, L0),
    arg(6, Inner, LUnit),
    v1_ref(E0, Map, Sko, E),
    v1_ref(Q0, Map, Sko, Q),
    v1_ref(A0, Map, Sko, A),
    v1_ref(L0, Map, Sko, L).
v1_condition(_, Inner, _, _, _) :-
    reject(unsupported, condition_shape(Inner)).

v1_participants(Index, Arity, _, _, _, _, _, _, []) :-
    Index > Arity,
    !.
v1_participants(Index, Arity, Inner, Map, Sko, Context, E,
        Pos, [guideline_arg(Context, E, Pos, Ref)|Terms]) :-
    arg(Index, Inner, Arg),
    v1_ref(Arg, Map, Sko, Ref),
    NextIndex is Index + 1,
    NextPos is Pos + 1,
    v1_participants(NextIndex, Arity, Inner, Map, Sko, Context, E,
        NextPos, Terms).

v1_check_operator(Op) :-
    ( atom(Op),
      memberchk(Op, [eq, geq, greater, leq, less, exactly, na]) ->
        true
    ; reject(unsupported, object_operator(Op))
    ).

/* Referent resolution: participants must be DRS variables; a variable
   resolves to its document identity, else its Skolem identity, else
   stays a body variable. */
v1_ref(Arg, _, _, _) :-
    nonvar(Arg),
    !,
    reject(unsupported, unresolved_argument(Arg)).
v1_ref(Var, Map, _, Id) :-
    v1_lookup(Map, Var, Id),
    !.
v1_ref(Var, _, Sko, Id) :-
    v1_lookup(Sko, Var, Id),
    !.
v1_ref(Var, _, _, Var).

/* ---------- v1 referent bookkeeping ---------- */

v1_ref_slots([], []).
v1_ref_slots([operator(_, _, _, _)|Items], Slots) :-
    v1_ref_slots(Items, Slots).
v1_ref_slots([naf(_, Payload)|Items], Slots) :-
    !,
    v1_ref_slots(Payload, Head),
    v1_ref_slots(Items, Tail),
    append(Head, Tail, Slots).
v1_ref_slots([anchored(_, Inner)|Items], Slots) :-
    v1_cond_refs(Inner, Head),
    v1_ref_slots(Items, Tail),
    append(Head, Tail, Slots).

v1_cond_refs(Inner, [Ref]) :-
    nonvar(Inner),
    functor(Inner, object, 6),
    !,
    arg(1, Inner, Ref).
v1_cond_refs(Inner, [E|Args]) :-
    nonvar(Inner),
    functor(Inner, predicate, Arity),
    Arity >= 3,
    !,
    arg(1, Inner, E),
    v1_arg_list(3, Arity, Inner, Args).
v1_cond_refs(Inner, [E, Obj]) :-
    nonvar(Inner),
    functor(Inner, modifier_pp, 3),
    !,
    arg(1, Inner, E),
    arg(3, Inner, Obj).
v1_cond_refs(Inner, [P]) :-
    nonvar(Inner),
    functor(Inner, property, 3),
    !,
    arg(1, Inner, P).
v1_cond_refs(_, []).

v1_arg_list(Index, Arity, _, []) :-
    Index > Arity,
    !.
v1_arg_list(Index, Arity, Inner, [Arg|Args]) :-
    arg(Index, Inner, Arg),
    Next is Index + 1,
    v1_arg_list(Next, Arity, Inner, Args).

v1_first_occurrence([], Acc, Ordered) :-
    reverse(Acc, Ordered).
v1_first_occurrence([V|Vs], Acc, Ordered) :-
    ( var(V),
      \+ v1_var_member(V, Acc) ->
        v1_first_occurrence(Vs, [V|Acc], Ordered)
    ; v1_first_occurrence(Vs, Acc, Ordered)
    ).

v1_lookup([Var0-Id0|Pairs], Var, Id) :-
    ( Var0 == Var ->
        Id = Id0
    ; v1_lookup(Pairs, Var, Id)
    ).

v1_var_member(V, [X|Xs]) :-
    ( V == X ->
        true
    ; v1_var_member(V, Xs)
    ).

v1_nth_var([X|Xs], V, K, N) :-
    ( X == V ->
        N = K
    ; K2 is K + 1,
      v1_nth_var(Xs, V, K2, N)
    ).

/* Witness pair assembly (P3-P4): walk a variant's flattened antecedent
   items, assigning each distinct positive-body variable its ground
   witness identity — anchored referent slots take ref(N) from the
   unsplit first-occurrence enumeration, operator items map their
   existential context variable to the recorded box(B), NAF items
   contribute nothing (controlled absence). Document-map definites stay
   ground product identities and take no pair. */
v1_witness_pairs(Items, Ordered, Map, DocId, S, K, Pairs) :-
    v1_witness_items(Items, Ordered, Map, DocId, S, K, [], Rev),
    reverse(Rev, Pairs).

v1_witness_items([], _, _, _, _, _, Acc, Acc).
v1_witness_items([operator(B, _, Inner, _)|Items], Ordered, Map, DocId, S,
        K, Acc0, Acc) :-
    !,
    ( var(Inner),
      \+ v1_pair_member(Inner, Acc0) ->
        Acc1 = [Inner-'$guideline_id'(witness, DocId, S, box(B),
            variant(K))|Acc0]
    ; Acc1 = Acc0
    ),
    v1_witness_items(Items, Ordered, Map, DocId, S, K, Acc1, Acc).
v1_witness_items([naf(_, _)|Items], Ordered, Map, DocId, S, K, Acc0, Acc) :-
    !,
    v1_witness_items(Items, Ordered, Map, DocId, S, K, Acc0, Acc).
v1_witness_items([anchored(_, Inner)|Items], Ordered, Map, DocId, S, K,
        Acc0, Acc) :-
    v1_cond_refs(Inner, Refs),
    v1_witness_refs(Refs, Ordered, Map, DocId, S, K, Acc0, Acc1),
    v1_witness_items(Items, Ordered, Map, DocId, S, K, Acc1, Acc).

v1_witness_refs([], _, _, _, _, _, Acc, Acc).
v1_witness_refs([Ref|Refs], Ordered, Map, DocId, S, K, Acc0, Acc) :-
    ( var(Ref),
      \+ v1_lookup(Map, Ref, _),
      \+ v1_pair_member(Ref, Acc0),
      v1_nth_var(Ordered, Ref, 1, N) ->
        Acc1 = [Ref-'$guideline_id'(witness, DocId, S, ref(N),
            variant(K))|Acc0]
    ; Acc1 = Acc0
    ),
    v1_witness_refs(Refs, Ordered, Map, DocId, S, K, Acc1, Acc).

v1_pair_member(V, [X-_|Pairs]) :-
    ( V == X ->
        true
    ; v1_pair_member(V, Pairs)
    ).

/* P10: head vars must be bound by POSITIVE body goals — a variable
   occurring only under \+ is unbound at call time. */
v1_check_safety(clause(Head, Goals)) :-
    term_variables(Head, HeadVars),
    v1_positive_vars(Goals, [], BodyVars),
    ( v1_vars_subset(HeadVars, BodyVars) ->
        true
    ; reject(safety, head_variable_not_bound_in_body)
    ).

v1_positive_vars([], Vars, Vars).
v1_positive_vars([naf(_, _)|Goals], Vars0, Vars) :-
    !,
    v1_positive_vars(Goals, Vars0, Vars).
v1_positive_vars([Goal|Goals], Vars0, Vars) :-
    term_variables(Goal, GoalVars),
    merge_vars(GoalVars, Vars0, Vars1),
    v1_positive_vars(Goals, Vars1, Vars).

/* P4 NAF safety: inside a \+ goal, NAF-box-local variables (the box
   domain) stay scoped; every other variable must be bound by an earlier
   positive goal in the same body. P10: a box-local variable occurring
   anywhere outside its own \+ goal is an escape. */
v1_naf_safety(Clauses) :-
    maplist(v1_naf_safe_clause, Clauses).

v1_naf_safe_clause(clause(Head, Goals)) :-
    v1_strip_naf(Goals, Stripped),
    v1_naf_walk(Goals, [], clause(Head, Stripped)).

/* Escape scanning counts occurrences in the clause as EMITTED: the
   naf bookkeeping wrapper carries the box domain, which is not an
   occurrence, so it is stripped to the payload goals first. */
v1_strip_naf([], []).
v1_strip_naf([naf(_, Sub)|Goals], [Sub|Stripped]) :-
    !,
    v1_strip_naf(Goals, Stripped).
v1_strip_naf([Goal|Goals], [Goal|Stripped]) :-
    v1_strip_naf(Goals, Stripped).

v1_naf_walk([], _, _).
v1_naf_walk([naf(NDom, Sub)|Goals], Bound, Clause) :-
    !,
    term_variables(Sub, Vars),
    v1_naf_vars_ok(Vars, NDom, Bound),
    v1_naf_escape_scan(NDom, Sub, Clause),
    v1_naf_walk(Goals, Bound, Clause).
v1_naf_walk([Goal|Goals], Bound0, Clause) :-
    term_variables(Goal, GoalVars),
    merge_vars(GoalVars, Bound0, Bound),
    v1_naf_walk(Goals, Bound, Clause).

v1_naf_vars_ok([], _, _).
v1_naf_vars_ok([V|Vs], NDom, Bound) :-
    ( strict_member(V, NDom) ->
        true
    ; strict_member(V, Bound) ->
        true
    ; reject(safety, naf_variable_not_bound)
    ),
    v1_naf_vars_ok(Vs, NDom, Bound).

/* Occurrence counting: clauses hold no drs/2 boxes, so the
   condition-position counter counts plain occurrences here. */
v1_naf_escape_scan([], _, _).
v1_naf_escape_scan([V|Vs], Sub, Clause) :-
    cond_occurrences(V, Clause, Total),
    cond_occurrences(V, Sub, Inside),
    ( Total =:= Inside ->
        true
    ; reject(safety, naf_local_escape)
    ),
    v1_naf_escape_scan(Vs, Sub, Clause).

v1_vars_subset([], _).
v1_vars_subset([V|Vs], Vars) :-
    v1_var_member(V, Vars),
    v1_vars_subset(Vs, Vars).

/* ---------- v1 rendering ---------- */

v1_render_document(DocId, AceDigest, UlexDigest, Lines, Bundles, OutCodes) :-
    header_term(DocId, AceDigest, UlexDigest, Header0),
    v2_record(Header0, Header),
    v2_indicators(Indicators),
    v2_version(Version),
    with_output_to(string(Out),
        ( format('% ~w.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.~n',
              [DocId]),
          v1_render_decls(Indicators),
          render_term_line(guideline_schema_version(Version)),
          render_term_line(Header),
          v1_render_bundles(Bundles, Lines)
        )),
    string_codes(Out, OutCodes).

v1_render_decls([]).
v1_render_decls([Name/Arity|Keys]) :-
    format(':- multifile(~q/~d).~n', [Name, Arity]),
    format(':- discontiguous(~q/~d).~n', [Name, Arity]),
    v1_render_decls(Keys).

v1_render_bundles([], _).
v1_render_bundles([bundle(S, SGroups)|Bundles], Lines) :-
    nth1(S, Lines, LineCodes),
    format('% S~w: ~s~n', [S, LineCodes]),
    v1_render_groups(SGroups),
    v1_render_bundles(Bundles, Lines).

v1_render_groups([]).
v1_render_groups([group(_, _, _, Clauses)|Groups]) :-
    v1_render_items(Clauses),
    v1_render_groups(Groups).

v1_render_items([]).
v1_render_items([Clause|Clauses]) :-
    v1_render_item(Clause),
    v1_render_items(Clauses).

v1_render_item(clause(Head, [])) :-
    !,
    render_term_line(Head).
v1_render_item(clause(Head, Goals)) :-
    v1_wrap_goals(Goals, Wrapped),
    render_item(rule(Head, Wrapped)).

v1_wrap_goals([], []).
v1_wrap_goals([naf(_, Sub)|Goals], [naf_conj(Sub)|Wrapped]) :-
    !,
    v1_wrap_goals(Goals, Wrapped).
v1_wrap_goals([Goal|Goals], [pos(Goal)|Wrapped]) :-
    v1_wrap_goals(Goals, Wrapped).

/* ---------- question mode (v1 query projection) ---------- */

/* One invocation = one query: exactly one sentence line parsing to one
   root question(drs(...)) box, projected onto a single
   '$guideline_query_projection'(goal(Conj), answers(Manifest)) term.
   Goal rendering = the rule-antecedent body pipeline verbatim (root
   context `actual`, operator edges first over existential context
   variables, no minted '$guideline_id'); referents stay projection
   variables shared between goal and manifest, numbered by one
   render_term_line pass. The projection adds no document indicator,
   declarations block, guideline_document/3, proof obligation, or
   aggregate participation. All query_* rejects ride class unsupported
   (exit 1). */

v1_translate_query(QId, Bytes, Text, UlexDigest, Sentences, Drs,
        OutCodes) :-
    ( nonvar(Drs),
      functor(Drs, drs, 2),
      arg(1, Drs, Dom),
      is_list(Dom),
      arg(2, Drs, Conds),
      is_list(Conds) ->
        true
    ; reject(unsupported, invalid_drs_shape)
    ),
    length(Sentences, SentenceCount),
    ( SentenceCount =:= 1 ->
        true
    ; reject(unsupported, query_sentences(SentenceCount))
    ),
    crypto_data_hash(Bytes, AceDigest, [algorithm(sha256), encoding(octet)]),
    v1_ulex_reserved_check,
    v1_collision_scan(Conds),
    query_root(Dom, Conds, QDrs),
    query_anchor(QDrs),
    query_scan_box(QDrs),
    query_markers(QDrs, Answers),
    query_strip_box(QDrs, Clean),
    query_goals(Clean, QId, Goals),
    ( Goals == [] ->
        reject(unsupported, query_unsupported(empty_goal, 1))
    ; true
    ),
    v1_proof_conj(Goals, Conj),
    term_variables(Conj, GoalVars),
    query_liveness(Answers, GoalVars),
    query_render(QId, AceDigest, UlexDigest, Text, Conj, Answers,
        OutCodes).

/* Root law: the sole root condition is one bare question/1 over a
   well-formed box; no root domain referent and no sibling root
   condition (premise-injection defense). */
query_root(Dom, Conds, QDrs) :-
    ( nonvar(Conds),
      functor(Conds, '[|]', 2),
      arg(2, Conds, Tail),
      Tail == [],
      arg(1, Conds, Single),
      nonvar(Single),
      functor(Single, question, 1) ->
        ( Dom == [] ->
            true
        ; reject(unsupported, query_unsupported(root_siblings, 1))
        ),
        arg(1, Single, QDrs),
        ( nonvar(QDrs),
          functor(QDrs, drs, 2) ->
            true
        ; reject(unsupported, query_unsupported(leaf(question/1), 1))
        ),
        query_box_check(QDrs)
    ; query_root_present(Conds) ->
        reject(unsupported, query_unsupported(root_siblings, 1))
    ; reject(unsupported, query_expected(1))
    ).

query_root_present(Conds) :-
    member(Cond, Conds),
    nonvar(Cond),
    query_cond_inner(Cond, Inner),
    nonvar(Inner),
    functor(Inner, question, 1),
    !.

/* Every anchored condition inside the question box carries S=1. */
query_anchor(QDrs) :-
    inner_sentence(QDrs, S),
    ( S =:= 1 ->
        true
    ; reject(unsupported, mixed_or_missing_sentence_anchors([S]))
    ).

/* Anchor wrappers strip to the inner condition; everything else is
   its own inner term. */
query_cond_inner(Cond, Inner) :-
    ( nonvar(Cond),
      functor(Cond, -, 2),
      arg(2, Cond, Anchor),
      nonvar(Anchor),
      anchor_sentence(Anchor, _) ->
        arg(1, Cond, Inner)
    ; Inner = Cond
    ).

/* Stage B: pre-order first-match blocker scan (structural blockers
   before marker validation; a supported modal box scans its payload
   before later siblings). Supported leaves pass at functor/arity
   level only — payload defects (named arguments, non-pos property
   degree, bad object operators, oversize predicates) reject later
   inside the shared v1 condition pipeline with its established
   details. Unknown shapes reject leaf(Name/Arity): closure is total
   by construction. */
query_scan_box(QDrs) :-
    query_box_check(QDrs),
    arg(2, QDrs, Conds),
    query_scan_conds(Conds).

/* Inner-box shape law: every question or modal box must be a drs/2
   with proper-list domain and conditions. Malformed shapes reject
   before any traversal touches them, so no traversal can bind an
   open list tail or read a non-list spine (non-instantiation law). */
query_box_check(Box) :-
    ( nonvar(Box),
      functor(Box, drs, 2),
      arg(1, Box, Dom),
      is_list(Dom),
      arg(2, Box, Conds),
      is_list(Conds) ->
        true
    ; reject(unsupported, invalid_drs_shape)
    ).

query_scan_conds([]).
query_scan_conds([Cond|Conds]) :-
    query_cond_inner(Cond, Inner),
    query_scan_leaf(Inner),
    query_scan_conds(Conds).

query_scan_leaf(Leaf) :-
    var(Leaf),
    !,
    reject(unsupported, query_unsupported(leaf(var), 1)).
query_scan_leaf(Leaf) :-
    is_list(Leaf),
    !,
    ( v2_table(_), Leaf \== [] ->
        query_scan_conds(Leaf)
    ; reject(unsupported, query_unsupported(leaf(list), 1))
    ).
query_scan_leaf(Leaf) :-
    query_blocker(Leaf, Blocker),
    !,
    reject(unsupported, query_unsupported(Blocker, 1)).
query_scan_leaf(Leaf) :-
    functor(Leaf, Op, 1),
    memberchk(Op, [should, must, can, may]),
    !,
    arg(1, Leaf, Box),
    query_scan_box(Box).
query_scan_leaf(Leaf) :-
    functor(Leaf, query, 2),
    !,
    arg(2, Leaf, Tag),
    ( atom(Tag),
      memberchk(Tag, [who, which, what]) ->
        true
    ; reject(unsupported, query_unsupported(wh(Tag), 1))
    ).
query_scan_leaf(Leaf) :-
    functor(Leaf, Name, Arity),
    query_supported_leaf(Name, Arity),
    !.
query_scan_leaf(Leaf) :-
    functor(Leaf, Name, Arity),
    reject(unsupported, query_unsupported(leaf(Name/Arity), 1)).

query_blocker(Leaf, universal) :-
    functor(Leaf, =>, 2).
query_blocker(Leaf, disjunction) :-
    functor(Leaf, v, 2).
query_blocker(Leaf, classical_negation) :-
    functor(Leaf, -, 1).
query_blocker(Leaf, naf) :-
    functor(Leaf, ~, 1).

query_supported_leaf(object, 6).
query_supported_leaf(predicate, 3).
query_supported_leaf(predicate, 4).
query_supported_leaf(predicate, 5).
query_supported_leaf(modifier_pp, 3).
query_supported_leaf(property, 3).
query_supported_leaf(relation, 3) :-
    v2_table(_).

/* Markers collect per box in pre-order together with that box's
   object/6 sources (same-box law), then validate in marker order:
   nonvar referent, duplicate referent, noun-source uniqueness.
   Exactly one same-box source names the answer noun(Noun,Class); a
   source-less who/what falls back to wh(Tag); a source-less which
   rejects. Liveness (every answer variable inside the goal) runs
   after goal rendering. */
query_markers(QDrs, Answers) :-
    query_box_markers(QDrs, [], Pairs),
    query_validate_markers(Pairs, [], Answers).

query_box_markers(Box, Acc0, Acc) :-
    arg(2, Box, Conds),
    query_box_objects(Conds, Objects),
    query_conds_markers(Conds, Objects, Acc0, Acc).

query_box_objects([], []).
query_box_objects([Cond|Conds], Objects) :-
    query_cond_inner(Cond, Inner),
    ( nonvar(Inner),
      functor(Inner, object, 6) ->
        arg(1, Inner, Ref),
        arg(2, Inner, Noun),
        arg(3, Inner, Class),
        Objects = [source(Ref, Noun, Class)|Rest]
    ; Objects = Rest
    ),
    query_box_objects(Conds, Rest).

query_conds_markers([], _, Acc, Acc).
query_conds_markers([Cond|Conds], Objects, Acc0, Acc) :-
    query_cond_inner(Cond, Inner),
    ( nonvar(Inner),
      functor(Inner, query, 2) ->
        arg(1, Inner, Ref),
        arg(2, Inner, Tag),
        append(Acc0, [marker(Ref, Tag, Objects)], Acc1)
    ; nonvar(Inner),
      functor(Inner, Op, 1),
      memberchk(Op, [should, must, can, may]),
      arg(1, Inner, Box),
      nonvar(Box),
      functor(Box, drs, 2) ->
        query_box_markers(Box, Acc0, Acc1)
    ; Acc1 = Acc0
    ),
    query_conds_markers(Conds, Objects, Acc1, Acc).

query_validate_markers([], _, []).
query_validate_markers([marker(Ref, Tag, Objects)|Pairs], Seen,
        [answer(Ref, Desc)|Answers]) :-
    ( var(Ref) ->
        true
    ; reject(unsupported, query_unsupported(marker(nonvar), 1))
    ),
    ( strict_member(Ref, Seen) ->
        reject(unsupported, query_unsupported(marker(duplicate), 1))
    ; true
    ),
    query_marker_desc(Ref, Tag, Objects, Desc),
    query_validate_markers(Pairs, [Ref|Seen], Answers).

query_marker_desc(Ref, Tag, Objects, Desc) :-
    query_ref_sources(Objects, Ref, Sources),
    ( Sources = [source(_, Noun, Class)] ->
        Desc = noun(Noun, Class)
    ; Sources = [_, _|_] ->
        reject(unsupported, query_unsupported(marker(conflict), 1))
    ; Tag == who ->
        Desc = wh(who)
    ; Tag == what ->
        Desc = wh(what)
    ; reject(unsupported, query_unsupported(marker(source), 1))
    ).

query_ref_sources([], _, []).
query_ref_sources([source(R, Noun, Class)|Objects], Ref, Sources) :-
    ( R == Ref ->
        Sources = [source(R, Noun, Class)|Rest]
    ; Sources = Rest
    ),
    query_ref_sources(Objects, Ref, Rest).

query_liveness([], _).
query_liveness([answer(Ref, _)|Answers], GoalVars) :-
    ( strict_member(Ref, GoalVars) ->
        true
    ; reject(unsupported, query_unsupported(marker(unbound), 1))
    ),
    query_liveness(Answers, GoalVars).

/* Marker removal rebuilds list spines and modal wrappers only; kept
   conditions stay the identical terms, so variable identity survives
   into the shared projection term. Modal classification reads the
   anchor-stripped inner term (same law as scan and markers); an
   anchored modal rebuilds bare — its anchor is already validated by
   query_anchor and carries no meaning past this point. */
query_strip_box(Box, drs(Dom, Clean)) :-
    arg(1, Box, Dom),
    arg(2, Box, Conds),
    query_strip_conds(Conds, Clean).

query_strip_conds([], []).
query_strip_conds([Cond|Conds], Clean) :-
    ( query_strip_cond(Cond, Kept) ->
        Clean = [Kept|Rest]
    ; Clean = Rest
    ),
    query_strip_conds(Conds, Rest).

query_strip_cond(Cond, _) :-
    query_cond_inner(Cond, Inner),
    nonvar(Inner),
    functor(Inner, query, 2),
    !,
    fail.
query_strip_cond(Cond, Kept) :-
    query_cond_inner(Cond, Inner),
    nonvar(Inner),
    functor(Inner, Op, 1),
    memberchk(Op, [should, must, can, may]),
    arg(1, Inner, Box),
    nonvar(Box),
    functor(Box, drs, 2),
    !,
    query_strip_box(Box, CleanBox),
    Kept =.. [Op, CleanBox].
query_strip_cond(Cond, Cond).

/* Goal rendering: rule-antecedent pipeline — no document map, no
   Skolem map, so referents stay body variables and operator boxes
   keep existential context variables (never minted contexts). */
query_goals(Clean, QId, Goals) :-
    arg(2, Clean, Conds),
    v1_flatten_items(Conds, antecedent, 1, QId, [], actual, none, 1, _,
        Items0),
    v2_annotate(1, Items0, Items),
    v1_expand_items(Items, [], [], Goals).

query_header_term(QId, AceDigest, none,
    '$guideline_query'(v1, QId, ace_sha256(AceDigest), ulex(none))).
query_header_term(QId, AceDigest, sha256(Digest),
    '$guideline_query'(v1, QId, ace_sha256(AceDigest),
        ulex(sha256(Digest)))).

/* Q1 comment = the sentence line bytes minus a trailing CR (comments
   stay one line); the ACE digest keeps the raw bytes. */
query_comment_codes(Text, Comment) :-
    input_lines(Text, [Line]),
    ( append(Body, [0'\r], Line) ->
        Comment = Body
    ; Comment = Line
    ).

query_render(QId, AceDigest, UlexDigest, Text, Conj, Answers, OutCodes) :-
    query_header_term(QId, AceDigest, UlexDigest, Record1),
    ( v2_version(V), V > 1 ->
        schema_token(Token, V),
        Record1 =.. [F, _|Args1],
        Record0 =.. [F, Token|Args1]
    ; Record0 = Record1
    ),
    v2_record(Record0, Record),
    query_comment_codes(Text, Comment),
    with_output_to(string(Out),
        ( format('% ~w compiled from ACE question by ace_to_pl question mode; do not edit.~n',
              [QId]),
          render_term_line(Record),
          format('% Q1: ~s~n', [Comment]),
          render_term_line('$guideline_query_projection'(goal(Conj),
              answers(Answers)))
        )),
    string_codes(Out, OutCodes).

/* Stage A: total question-form classifier for the document-mode
   diagnostic — the first query/2 marker in pre-order names wh(Tag);
   else a top-level implication reads universal; else yesno. */
query_form(QDrs, Form) :-
    ( query_first_marker(QDrs, Tag) ->
        Form = wh(Tag)
    ; query_top_implication(QDrs) ->
        Form = universal
    ; Form = yesno
    ).

/* Marker search = pre-order over condition lists, descending only
   into the box arguments of box-carrying condition functors (modal,
   implication, disjunction, negation, nested question). Payload
   arguments of atomic leaves are never inspected, so a decoy query/2
   — or a whole decoy drs/2 — inside an atomic condition cannot
   outrank a real condition marker. Total: malformed shapes fail the
   search without binding input. */
query_first_marker(QDrs, Tag) :-
    nonvar(QDrs),
    functor(QDrs, drs, 2),
    arg(2, QDrs, Conds),
    query_conds_first_marker(Conds, Tag).

query_conds_first_marker(Conds, Tag) :-
    nonvar(Conds),
    functor(Conds, '[|]', 2),
    arg(1, Conds, Cond),
    ( query_cond_first_marker(Cond, Tag0) ->
        Tag = Tag0
    ; arg(2, Conds, Rest),
      query_conds_first_marker(Rest, Tag)
    ).

query_cond_first_marker(Cond, Tag) :-
    query_cond_inner(Cond, Inner),
    nonvar(Inner),
    ( functor(Inner, query, 2) ->
        arg(2, Inner, Tag)
    ; query_box_carrier(Inner) ->
        query_args_first_marker(Inner, 1, Tag)
    ).

query_box_carrier(Inner) :-
    functor(Inner, Op, Arity),
    ( Arity =:= 1 ->
        memberchk(Op, [should, must, can, may, -, ~, question])
    ; Arity =:= 2,
      memberchk(Op, [=>, v])
    ).

query_args_first_marker(Term, N, Tag) :-
    functor(Term, _, Arity),
    N =< Arity,
    ( arg(N, Term, A),
      query_first_marker(A, Tag0) ->
        Tag = Tag0
    ; N1 is N + 1,
      query_args_first_marker(Term, N1, Tag)
    ).

query_top_implication(QDrs) :-
    nonvar(QDrs),
    functor(QDrs, drs, 2),
    arg(2, QDrs, Conds),
    is_list(Conds),
    member(Cond, Conds),
    query_cond_inner(Cond, Inner),
    nonvar(Inner),
    functor(Inner, =>, 2),
    !.

/* ---------- v1 derived proof obligations (P4-P7) ---------- */

v1_proof_depth_limit(4000).
v1_proof_inference_limit(1000000).

/* Bounded obligation call of the per-document replay: the depth-limited search runs under a global inference
   budget, so mutually-recursive rule clauses (a body negation goal
   resolving against another rule's negation-edge head) fail finitely
   instead of searching exponentially. Exceeding either bound counts
   as ordinary underivability (DR06). */
v1_bounded_head_call(Goal) :-
    v1_proof_depth_limit(Depth),
    v1_proof_inference_limit(Inferences),
    call_with_inference_limit(
        call_with_depth_limit(Goal, Depth, DepthResult), Inferences,
        InferenceResult),
    !,
    InferenceResult \== inference_limit_exceeded,
    DepthResult \== depth_limit_exceeded.

/* Every v1 compile derives one ground obligation term per group and
   replays it against the document's own clauses in a private module:
   witness facts asserted front-of-predicate with refs so obligation
   search reaches them before any rule clause, each obligation head
   called under the shared bounds, refs erased. Failure rejects the
   document (class proof) — an underivable rule can never fire in any
   world satisfying its body. Obligation construction copies clauses together with the
   witness pairs, then binds the copies; the originals stay
   variable-clean for product rendering. */
v1_derive_proofs(Bundles, DocId, Payload) :-
    v1_load_proof_world(Bundles),
    v1_prove_bundles(Bundles, DocId, Payload).

v1_load_proof_world(Bundles) :-
    v2_indicators(Indicators),
    v1_declare_proof_world(Indicators),
    v1_load_proof_bundles(Bundles).

v1_declare_proof_world([]).
v1_declare_proof_world([Name/Arity|Keys]) :-
    dynamic(ace_to_pl_proof_world:Name/Arity),
    v1_declare_proof_world(Keys).

v1_load_proof_bundles([]).
v1_load_proof_bundles([bundle(_, SGroups)|Bundles]) :-
    v1_load_proof_groups(SGroups),
    v1_load_proof_bundles(Bundles).

v1_load_proof_groups([]).
v1_load_proof_groups([group(_, _, _, Clauses)|Groups]) :-
    v1_assert_doc_clauses(Clauses),
    v1_load_proof_groups(Groups).

v1_assert_doc_clauses([]).
v1_assert_doc_clauses([Clause|Clauses]) :-
    v1_assert_doc_clause(Clause),
    v1_assert_doc_clauses(Clauses).

v1_assert_doc_clause(clause(Head, [])) :-
    !,
    assertz(ace_to_pl_proof_world:Head).
v1_assert_doc_clause(clause(Head, Goals)) :-
    v1_proof_body(Goals, Body),
    assertz(ace_to_pl_proof_world:(Head :- Body)).

v1_proof_body([Goal], Converted) :-
    !,
    v1_proof_goal(Goal, Converted).
v1_proof_body([Goal|Goals], (Converted, Rest)) :-
    v1_proof_goal(Goal, Converted),
    v1_proof_body(Goals, Rest).

v1_proof_goal(naf(_, Sub), \+ Conj) :-
    !,
    v1_proof_conj(Sub, Conj).
v1_proof_goal(Goal, Goal).

v1_proof_conj([Goal], Goal) :-
    !.
v1_proof_conj([Goal|Goals], (Goal, Rest)) :-
    v1_proof_conj(Goals, Rest).

v1_prove_bundles([], _, []).
v1_prove_bundles([bundle(S, SGroups)|Bundles], DocId, Payload) :-
    v1_prove_groups(SGroups, DocId, S, Head),
    v1_prove_bundles(Bundles, DocId, Tail),
    append(Head, Tail, Payload).

v1_prove_groups([], _, _, []).
v1_prove_groups([Group|Groups], DocId, S, [Obligation|Obligations]) :-
    v1_group_obligation(Group, DocId, S, Obligation),
    v1_check_obligation(Obligation),
    v1_prove_groups(Groups, DocId, S, Obligations).

v1_group_obligation(group(Kind, K, Pairs, Clauses), DocId, S, Obligation) :-
    copy_term(Clauses-Pairs, CopiedClauses-CopiedPairs),
    v1_bind_witness(CopiedPairs),
    ( Kind == fact ->
        Facts = []
    ; CopiedClauses = [clause(_, Goals)|_],
      v1_positive_goals(Goals, Facts)
    ),
    v1_heads(CopiedClauses, Heads),
    Obligation = '$guideline_proof'(DocId, S, variant(K), witness(Facts),
        prove(Heads)),
    ( ground(Obligation) ->
        true
    ; reject(proof, nonground_obligation(S, variant(K)))
    ).

v1_bind_witness([]).
v1_bind_witness([Var-Id|Pairs]) :-
    Var = Id,
    v1_bind_witness(Pairs).

v1_positive_goals([], []).
v1_positive_goals([naf(_, _)|Goals], Facts) :-
    !,
    v1_positive_goals(Goals, Facts).
v1_positive_goals([Goal|Goals], [Goal|Facts]) :-
    v1_positive_goals(Goals, Facts).

v1_heads([], []).
v1_heads([clause(Head, _)|Clauses], [Head|Heads]) :-
    v1_heads(Clauses, Heads).

v1_check_obligation('$guideline_proof'(_, S, variant(K), witness(Facts),
        prove(Heads))) :-
    v1_assert_witness(Facts, Refs),
    ( v1_prove_heads(Heads) ->
        v1_erase_refs(Refs)
    ; v1_erase_refs(Refs),
      reject(proof, underivable_obligation(S, variant(K)))
    ).

v1_assert_witness([], []).
v1_assert_witness([Fact|Facts], [Ref|Refs]) :-
    asserta(ace_to_pl_proof_world:Fact, Ref),
    v1_assert_witness(Facts, Refs).

v1_erase_refs([]).
v1_erase_refs([Ref|Refs]) :-
    erase(Ref),
    v1_erase_refs(Refs).

v1_prove_heads([]).
v1_prove_heads([Head|Heads]) :-
    v1_bounded_head_call(ace_to_pl_proof_world:Head),
    v1_prove_heads(Heads).

/* Payload rendering (P6): one ground term per line through the shared
   canonical writer; no comments, no environment traces. */
v1_render_payload(Payload, OutCodes) :-
    with_output_to(string(Out), v1_render_payload_terms(Payload)),
    string_codes(Out, OutCodes).

v1_render_payload_terms([]).
v1_render_payload_terms([Term|Terms]) :-
    render_term_line(Term),
    v1_render_payload_terms(Terms).

/* ---------- canonical error emission ---------- */

emit_error(ErrorStream, Class, Detail, Status) :-
    ( catch(canonical_error_line(ace_to_pl_error(Class, Detail), Line), _,
          fail) ->
        true
    ; fallback_error_line(Class, Line)
    ),
    format(ErrorStream, '~s', [Line]),
    halt(Status).

/* Error details additionally admit SWI strings: query-file guards
   surface caller-typed payloads ("v1", "foreign") verbatim, so the
   frozen collision-free query_file(<why>) vocabulary must not
   collapse to the unserializable fallback. Artifact rendering
   (validate_emittable) stays string-free under the frozen ABI. */
canonical_error_tree(Term) :-
    string(Term),
    !.
canonical_error_tree(Term) :-
    compound(Term),
    !,
    functor(Term, Name, Arity),
    \+ ( Name == '$VAR', Arity =:= 1 ),
    canonical_error_args(1, Arity, Term).
canonical_error_tree(Term) :-
    canonical_tree(Term).

canonical_error_args(Index, Arity, _) :-
    Index > Arity,
    !.
canonical_error_args(Index, Arity, Term) :-
    Index =< Arity,
    arg(Index, Term, Arg),
    canonical_error_tree(Arg),
    Next is Index + 1,
    canonical_error_args(Next, Arity, Term).

canonical_error_line(Term, Line) :-
    acyclic_term(Term),
    term_attvars(Term, []),
    canonical_error_tree(Term),
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

fallback_error_line(input_utf8, "ace_to_pl_error(input_utf8,unserializable).\n") :- !.
fallback_error_line(ape_messages, "ace_to_pl_error(ape_messages,unserializable).\n") :- !.
fallback_error_line(empty_drs, "ace_to_pl_error(empty_drs,unserializable).\n") :- !.
fallback_error_line(sentence_lines, "ace_to_pl_error(sentence_lines,unserializable).\n") :- !.
fallback_error_line(unsupported, "ace_to_pl_error(unsupported,unserializable).\n") :- !.
fallback_error_line(safety, "ace_to_pl_error(safety,unserializable).\n") :- !.
fallback_error_line(proof, "ace_to_pl_error(proof,unserializable).\n") :- !.
fallback_error_line(usage, "ace_to_pl_error(usage,unserializable).\n") :- !.
fallback_error_line(ape_load, "ace_to_pl_error(ape_load,unserializable).\n") :- !.
fallback_error_line(ulex_load, "ace_to_pl_error(ulex_load,unserializable).\n") :- !.
fallback_error_line(_, "ace_to_pl_error(uncaught,unserializable).\n").

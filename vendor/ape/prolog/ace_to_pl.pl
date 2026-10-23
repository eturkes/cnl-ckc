% ace_to_pl.pl — ACE → plain-Prolog compiler over the APE parser.
% Added to this APE fork 2026-08-06, not part of upstream APE (GPL-3.0
% §5(a) via LGPL: modification notice + a relevant date); git history is
% the change record for every later edit.
% Copyright 2026 Emir Turkes. Derivative of APE; licensed under
% LGPL-3.0-or-later like the surrounding tree (see ../LICENSE.txt).
%
% Modes (argv after --; contract c1 D5; `-` = no file):
%   <ape-tree-dir> <docid> <vocabulary.tsv|-> <ulex|->
%                                     compile onto the schema; ACE bytes on
%                                     stdin -> Prolog on stdout
%   ... proof                         derive + emit the proof payload (one ground
%                                     '$guideline_proof'/5 term per group) instead of
%                                     the product; same derivation check either way
%   question <ape-tree-dir> <qid> <vocabulary.tsv|-> <ulex|->
%                                     compile one ACE question (exactly one
%                                     sentence line parsing to one root question
%                                     box) into a '$guideline_query'/5 record and
%                                     one '$guideline_query_projection'/2 term:
%                                     goal(<','/2 conjunction over the
%                                     indicator vocabulary>) + answers([answer(
%                                     Var,noun(Noun,Class)|wh(who|what)),...]).
%                                     <qid> follows the docid grammar; rejects
%                                     ride the same canonical classes (new
%                                     unsupported details: query_expected/1,
%                                     query_sentences/1, query_unsupported/2)
%   `proof` in the vocabulary or ulex slot rejects usage vocabulary_arg(proof) |
%   ulex_arg(proof); a leading schema word (v1|v2|v3, after `question` too) and
%   every other vector reject usage argv(Argv) — each before any file loads.
%   A vocabulary.tsv (the guideline's lemma -> closed-id table) adds the
%   interval, recurrence, order, frequency, scoped-recurrence, approximation and
%   range annotations its rows reach (contracts m7t, q12, q13; the v2_/v3_
%   predicate prefixes below name those contracts' pattern families), in
%   documents the grade + preference annotations of its strength, evidence,
%   preference and alternative rows (contract s1; the s1_ prefix) and the
%   qualifier annotations of its qualifier rows (contract s2; the s2_ prefix),
%   a guideline_unit/3 literal after every time quantity of its unit rows and,
%   in antecedents + questions, value comparisons through guideline_satisfies/4|6
%   (contract nt1; the nt1_ prefix; definitions = guidelines/schema.pl),
%   with its scope + exclusion pair an exclusion rule's record
%   guideline_exclusion(Key) and \+ guideline_exclusion(Key) on every
%   should/must/may edge clause (contract cs1; the cs1_ prefix),
%   and, with or without a table, a rule consequent's formula `Y = E` as a
%   guideline_product/6 | guideline_quotient/6 chain whose result is Y's
%   cardinality (contract da2; the da2_ prefix),
%   and admits relation/3 leaves + upper-bound condition lists in questions;
%   rejects add unsupported temporal_shape(Why,S), grade_shape(Why,S),
%   preference_shape(Why,S), qualifier_shape(Why,S), scope_shape(Why,S),
%   formula_shape(Why,S) and the load class
%   vocabulary_load (exit 2). Law = docs/REFERENCE.md "Compiled Prolog schema".
% The composition consumption modes (check, aggregate-check, recursion-check,
% answer, trace) run in the verified Rust kernel as `ckc v1 <mode>`.
%
% Compile contract: stdin = strict RFC 3629 UTF-8; one ACE sentence per non-empty LF line.
% <docid> = nonempty [a-z0-9-] with no leading dash.
% Success: stdout = compiled Prolog document, stderr = 0 bytes, exit 0.
% Reject: stdout = 0 bytes; stderr = one canonical ace_to_pl_error(Class,Detail) line.
% Exit: 0=compiled; 1=input_utf8|ape_messages|empty_drs|sentence_lines|unsupported|safety|proof;
%       2=usage|ape_load|ulex_load|vocabulary_load|uncaught.
% Assurance: every compile (product or proof emission) derives per-group
% witness worlds + obligations and replays them against the document's own
% clauses; an underivable obligation rejects the document (class proof).
% One schema, identifier 1 (contract c1): every document + question; the
% projection lives under "schema projection" near the end of this file. It
% admits copula/ground facts and Horn rules plus modal/classical-negation
% operator wrappers (reified as guideline_operator/3 edges over context
% ids), consequent currying, antecedent Horn splits over one disjunction,
% and executable negation-as-failure for top-level antecedent NAF boxes;
% a root condition list joins its sentence's facts when its root-context
% upper bounds are all `exactly N`, N >= 1 (contract root-exact).
% Rule bodies run binding-first (contract bf, bf_order/2): the safety
% checks read the sentence order; the written body takes the leftmost goal
% whose waited values are bound, else the leftmost goal.
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
        uncaught(ErrorStream, Error)).

/* halt/1 inside run/3 raises unwind(halt(Status)) (SWI-Prolog >= 9.3.13),
   which catch/3 rethrows once the recovery goal ends: the recovery stays
   silent, so the halt completes with its own status. */
uncaught(_, unwind(_)) :-
    !.
uncaught(ErrorStream, Error) :-
    emit_error(ErrorStream, uncaught, Error, 2).

run(Input, Output, ErrorStream) :-
    prompt(_, ''),
    set_stream(Output, encoding(utf8)),
    set_stream(ErrorStream, encoding(utf8)),
    current_prolog_flag(argv, Argv),
    dispatch(Argv, Input, Output, ErrorStream).

/* c1 D5: every slot is positional, so each vector names its whole input;
   validation order = shape, id, vocabulary slot, ulex slot. */
dispatch([question|Rest], Input, Output, ErrorStream) :-
    !,
    ( Rest = [Tree, QId, Vocabulary, Ulex], \+ schema_word(Tree) ->
        validated_docid(QId, ErrorStream),
        validated_file_arg(vocabulary_arg, Vocabulary, ErrorStream, Table),
        validated_file_arg(ulex_arg, Ulex, ErrorStream, Lexicon),
        compile_mode(query, Tree, QId, Lexicon, Table, Input, Output,
            ErrorStream)
    ; emit_error(ErrorStream, usage, argv([question|Rest]), 2)
    ).
dispatch(Argv, Input, Output, ErrorStream) :-
    ( Argv = [Tree, DocId, Vocabulary, Ulex|Tail],
      \+ schema_word(Tree),
      proof_tail(Tail, Mode) ->
        validated_docid(DocId, ErrorStream),
        validated_file_arg(vocabulary_arg, Vocabulary, ErrorStream, Table),
        validated_file_arg(ulex_arg, Ulex, ErrorStream, Lexicon),
        compile_mode(Mode, Tree, DocId, Lexicon, Table, Input, Output,
            ErrorStream)
    ; emit_error(ErrorStream, usage, argv(Argv), 2)
    ).

proof_tail([], v1(product)).
proof_tail([proof], v1(proof)).

/* A pre-c1 selector vector (`v2 <tree> …`, `question v3 <tree> …`) is
   refused whole rather than read as a tree named v2. */
schema_word(v1).
schema_word(v2).
schema_word(v3).

/* `-` = no file; `proof` is reserved in both file slots, so the pre-c1
   `<tree> <docid> <ulex> proof` form rejects; a file literally named `-` or
   `proof` is unaddressable. */
validated_file_arg(Slot, Arg, ErrorStream, File) :-
    ( Arg == proof ->
        Detail =.. [Slot, Arg],
        emit_error(ErrorStream, usage, Detail, 2)
    ; Arg == '-' ->
        File = none
    ; File = file(Arg)
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

compile_mode(Mode, Tree, DocId, Ulex, Vocabulary, Input, Output, ErrorStream) :-
    set_stream(Input, type(binary)),
    prompt(_, ''),
    load_ape(Tree, Input, Output, ErrorStream),
    maybe_load_ulex(Ulex, Input, Output, ErrorStream, UlexDigest),
    maybe_load_vocabulary(Vocabulary, ErrorStream),
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

/* ---------- vocabulary table (contracts m7t D1, q12 D1, q13 D1, c1 D4) ----------
   `vocabulary.tsv` maps source lemmas onto closed ids, so the projection
   names no source word. The table parks in a global the projection reads;
   none = no table. */
maybe_load_vocabulary(none, _) :-
    nb_setval(ace_to_pl_vocabulary, none).
maybe_load_vocabulary(file(File), ErrorStream) :-
    catch(
        ( read_vocabulary(File, Table) ->
            true
        ; throw(error(vocabulary_load_failed(File),
              context(ace_to_pl:maybe_load_vocabulary/2, plain_failure)))
        ),
        Error,
        emit_error(ErrorStream, vocabulary_load, Error, 2)),
    nb_setval(ace_to_pl_vocabulary, Table).

vocabulary_header(Codes) :-
    atom_codes('# format: kind\tlemma\tvalue\n', Codes).

read_vocabulary(File, table(Units, Relations, Spacings, Windows, Frequencies,
        Approximations, Ranges, Strength, Digest)) :-
    setup_call_cleanup(
        open(File, read, Stream, [type(binary)]),
        read_stream_to_codes(Stream, Bytes),
        close(Stream)),
    crypto_data_hash(Bytes, Digest, [algorithm(sha256), encoding(octet)]),
    vocabulary_header(Header),
    ( append(Header, Body, Bytes) -> true ; v2_bad(header) ),
    ( Body \== [] -> true ; v2_bad(no_rows) ),
    ( append(Lines, [0'\n], Body) -> true ; v2_bad(final_newline) ),
    split_lf(Lines, Rows),
    v2_rows(Rows, 1, t([], [], [], [], [], [], [], k([], [], [], [], [], [], [])),
        t(Units, Relations, Spacings, Windows, Frequencies, Approximations, Ranges,
          Strength)),
    v2_pairing(Strength).

/* cs1 D1: the scope + exclusion rows come as a pair; a lone row reports
   after the last row. */
v2_pairing(k(_, _, _, _, _, Sc, Ex)) :-
    ( Sc = [scope(_, _, N)], Ex == [] ->
        v2_bad(row(N, scope_without_exclusion))
    ; Ex = [exclusion(_, N)], Sc == [] ->
        v2_bad(row(N, exclusion_without_scope))
    ; true
    ).

v2_bad(Why) :-
    throw(error(vocabulary(Why), context(ace_to_pl:read_vocabulary/2, _))).

v2_rows([], _, T, T).
v2_rows([Row|Rows], N, T0, T) :-
    v2_row(Row, N, T0, T1),
    N1 is N + 1,
    v2_rows(Rows, N1, T1, T).

/* One row; checks run in the spec's order (comment, fields, lemma, kind,
   value, duplicates). A `#` row is a comment, never a row, so a pre-c1
   two-line header fails at row 1. A noun is one of unit, frame and grade
   noun; a preposition maps to at most one of relation, frequency and range,
   and an alternative preposition to no other kind; a lemma is at most one of
   an approximation, a preference and a qualifier (s1 D2, s2 D2); one scope
   row (a noun that is no unit, frame or grade noun + the guideline key) and
   one exclusion row (its verb) (cs1 D1). K = k(Strengths, Evidences,
   Preferences, Alternatives, Qualifiers, Scope, Exclusion). */
v2_row(Row, N, t(U0, R0, S0, W0, F0, A0, G0, K0), T) :-
    K0 = k(GS0, GE0, P0, Alt0, Q0, Sc0, Ex0),
    ( Row = [0'#|_] -> v2_bad(row(N, comment)) ; true ),
    v2_split_tab(Row, Fields),
    ( Fields = [K, L, V] -> true ; v2_bad(row(N, field_count)) ),
    ( v2_lemma(L, Lemma) -> true ; v2_bad(row(N, lemma)) ),
    atom_codes(Kind, K),
    ( Kind == unit ->
        ( v2_closed_id(V, [second, minute, hour, day, week, month, year], Unit) ->
            true
        ; v2_bad(row(N, unit_id))
        ),
        ( ( memberchk(Lemma-_, U0) ; memberchk(_-Lemma, S0) ; memberchk(_-Lemma, W0)
          ; memberchk(Lemma-_, GS0) ; memberchk(Lemma-_, GE0)
          ; memberchk(scope(Lemma, _, _), Sc0) ) ->
            v2_bad(row(N, duplicate_noun))
        ; true
        ),
        append(U0, [Lemma-Unit], U),
        T = t(U, R0, S0, W0, F0, A0, G0, K0)
    ; Kind == relation ->
        ( v2_closed_id(V, [duration, within, after, before], Role) ->
            true
        ; v2_bad(row(N, role_id))
        ),
        ( ( memberchk(Lemma-_, R0) ; memberchk(Lemma, F0) ; memberchk(Lemma, G0)
          ; memberchk(Lemma, Alt0) ) ->
            v2_bad(row(N, duplicate_preposition))
        ; true
        ),
        append(R0, [Lemma-Role], R),
        T = t(U0, R, S0, W0, F0, A0, G0, K0)
    ; ( Kind == spacing ; Kind == window ) ->
        ( v2_lemma(V, Frame) -> true ; v2_bad(row(N, frame_lemma)) ),
        ( ( memberchk(Lemma-Frame, S0) ; memberchk(Lemma-Frame, W0)
          ; memberchk(Frame-_, U0) ; memberchk(Frame-_, GS0) ; memberchk(Frame-_, GE0)
          ; memberchk(scope(Frame, _, _), Sc0) ) ->
            v2_bad(row(N, duplicate_noun))
        ; true
        ),
        ( memberchk(Lemma, Alt0) -> v2_bad(row(N, duplicate_preposition)) ; true ),
        ( Kind == spacing ->
            append(S0, [Lemma-Frame], S),
            T = t(U0, R0, S, W0, F0, A0, G0, K0)
        ; append(W0, [Lemma-Frame], W),
          T = t(U0, R0, S0, W, F0, A0, G0, K0)
        )
    ; Kind == frequency ->
        ( atom_codes(period, V) -> true ; v2_bad(row(N, frequency_value)) ),
        ( ( memberchk(Lemma, F0) ; memberchk(Lemma-_, R0) ; memberchk(Lemma, G0)
          ; memberchk(Lemma, Alt0) ) ->
            v2_bad(row(N, duplicate_preposition))
        ; true
        ),
        append(F0, [Lemma], F),
        T = t(U0, R0, S0, W0, F, A0, G0, K0)
    ; Kind == approximation ->
        ( atom_codes(about, V) -> true ; v2_bad(row(N, approximation_value)) ),
        ( ( memberchk(Lemma, A0) ; memberchk(Lemma, P0) ; memberchk(Lemma-_, Q0) ) ->
            v2_bad(row(N, duplicate_adjective))
        ; true
        ),
        append(A0, [Lemma], A),
        T = t(U0, R0, S0, W0, F0, A, G0, K0)
    ; Kind == range ->
        ( atom_codes(minimum, V) -> true ; v2_bad(row(N, range_value)) ),
        ( ( memberchk(Lemma, G0) ; memberchk(Lemma-_, R0) ; memberchk(Lemma, F0)
          ; memberchk(Lemma, Alt0) ) ->
            v2_bad(row(N, duplicate_preposition))
        ; true
        ),
        append(G0, [Lemma], G),
        T = t(U0, R0, S0, W0, F0, A0, G, K0)
    ; ( Kind == strength ; Kind == evidence ) ->
        ( Kind == strength ->
            Ids = [strong, conditional], Why = strength_value
        ; Ids = [high, moderate, low, very_low], Why = evidence_value
        ),
        ( v2_closed_id(V, Ids, Value) -> true ; v2_bad(row(N, Why)) ),
        ( ( memberchk(Lemma-_, GS0) ; memberchk(Lemma-_, GE0) ; memberchk(Lemma-_, U0)
          ; memberchk(_-Lemma, S0) ; memberchk(_-Lemma, W0)
          ; memberchk(scope(Lemma, _, _), Sc0) ) ->
            v2_bad(row(N, duplicate_noun))
        ; true
        ),
        ( Kind == strength ->
            append(GS0, [Lemma-Value], GS),
            T = t(U0, R0, S0, W0, F0, A0, G0, k(GS, GE0, P0, Alt0, Q0, Sc0, Ex0))
        ; append(GE0, [Lemma-Value], GE),
          T = t(U0, R0, S0, W0, F0, A0, G0, k(GS0, GE, P0, Alt0, Q0, Sc0, Ex0))
        )
    ; Kind == preference ->
        ( atom_codes(preferred, V) -> true ; v2_bad(row(N, preference_value)) ),
        ( ( memberchk(Lemma, P0) ; memberchk(Lemma, A0) ; memberchk(Lemma-_, Q0) ) ->
            v2_bad(row(N, duplicate_adjective))
        ; true
        ),
        append(P0, [Lemma], P),
        T = t(U0, R0, S0, W0, F0, A0, G0, k(GS0, GE0, P, Alt0, Q0, Sc0, Ex0))
    ; Kind == alternative ->
        ( atom_codes(alternative, V) -> true ; v2_bad(row(N, alternative_value)) ),
        ( ( memberchk(Lemma, Alt0) ; memberchk(Lemma-_, R0) ; memberchk(Lemma-_, S0)
          ; memberchk(Lemma-_, W0) ; memberchk(Lemma, F0) ; memberchk(Lemma, G0) ) ->
            v2_bad(row(N, duplicate_preposition))
        ; true
        ),
        append(Alt0, [Lemma], Alt),
        T = t(U0, R0, S0, W0, F0, A0, G0, k(GS0, GE0, P0, Alt, Q0, Sc0, Ex0))
    ; Kind == qualifier ->
        ( v2_closed_id(V, [usuality, ideality, focus, manner, force, possibility,
              applicability], Class) ->
            true
        ; v2_bad(row(N, qualifier_value))
        ),
        ( memberchk(Lemma-_, Q0) -> v2_bad(row(N, duplicate_qualifier)) ; true ),
        ( ( memberchk(Lemma, A0) ; memberchk(Lemma, P0) ) ->
            v2_bad(row(N, duplicate_adjective))
        ; true
        ),
        append(Q0, [Lemma-Class], Q),
        T = t(U0, R0, S0, W0, F0, A0, G0, k(GS0, GE0, P0, Alt0, Q, Sc0, Ex0))
    ; Kind == scope ->
        ( v2_scope_key(V, Key) -> true ; v2_bad(row(N, scope_value)) ),
        ( Sc0 \== [] -> v2_bad(row(N, duplicate_scope)) ; true ),
        ( ( memberchk(Lemma-_, U0) ; memberchk(_-Lemma, S0) ; memberchk(_-Lemma, W0)
          ; memberchk(Lemma-_, GS0) ; memberchk(Lemma-_, GE0) ) ->
            v2_bad(row(N, duplicate_noun))
        ; true
        ),
        T = t(U0, R0, S0, W0, F0, A0, G0,
            k(GS0, GE0, P0, Alt0, Q0, [scope(Lemma, Key, N)], Ex0))
    ; Kind == exclusion ->
        ( atom_codes(scope, V) -> true ; v2_bad(row(N, exclusion_value)) ),
        ( Ex0 \== [] -> v2_bad(row(N, duplicate_exclusion)) ; true ),
        T = t(U0, R0, S0, W0, F0, A0, G0,
            k(GS0, GE0, P0, Alt0, Q0, Sc0, [exclusion(Lemma, N)]))
    ; v2_bad(row(N, kind))
    ).

/* cs1 D1: a guideline key = the docid grammar (spec v1text name_ok). */
v2_scope_key([C|Cs], Key) :-
    C =\= 0'-,
    forall(member(X, [C|Cs]), docid_code(X)),
    atom_codes(Key, [C|Cs]).

v2_split_tab(Codes, [Field|Fields]) :-
    ( append(Field, [0'\t|Rest], Codes) ->
        v2_split_tab(Rest, Fields)
    ; Field = Codes,
      Fields = []
    ).

/* A lemma field = nonempty UTF-8 bytes, none of them a space, control or
   DEL byte; Atom = its decoded text (spec vocabulary.rs lemma_ok). */
v2_lemma([B|Bs], Atom) :-
    forall(member(X, [B|Bs]), ( X > 0x20, X =\= 0x7f )),
    catch(decode_utf8([B|Bs], Codes, 0), _, fail),
    atom_codes(Atom, Codes).

v2_closed_id(Codes, Ids, Id) :-
    member(Id, Ids),
    atom_codes(Id, Codes),
    !.

v2_table(T) :-
    nb_current(ace_to_pl_vocabulary, table(U, R, S, W, F, A, G, _, _)),
    T = t(U, R, S, W, F, A, G).

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
    v3_range_lows(Anns, Items0, Anns, S, Lows),
    ( v2_dup(Lows) ->
        reject(unsupported, temporal_shape(range_shape, S))
    ; true
    ),
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
    v3_abouts(Anns, T, Items0, Approx),
    v2_rebuild(Items0, T, S, Items0, Anns, Links, Approx, Items).

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
    T = t(U, R, Sp, W, _, _, _),
    append(Sp, W, Frames),
    arg(1, PP, E),
    arg(2, PP, P),
    arg(3, PP, X),
    v2_of_links(Scope, Ctx, X, Links),
    length(Links, NL),
    ( atom(P), memberchk(P-Role, R), v2_qty(U, Scope, Ctx, X, Unit, O) ->
        ( v2_bound_why(O, Why) -> reject(unsupported, temporal_shape(Why, S)) ; true ),
        v3_approx_check(T, S, Scope, Ctx, X, X, O),
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
            v3_approx_check(T, S, Scope, Ctx, Q, X, O),
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
v2_claims([Ctx-range(_, H, _)|Anns], [Ctx-H|Claims]) :-
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
v2_consumed([_-range(_, _, _)|Anns], Links) :-
    v2_consumed(Anns, Links).

v2_rebuild(_, _, _, [], _, _, _, []).
v2_rebuild(Scope, T, S, [Item|Items], Anns, Links, Approx, Out) :-
    v2_rebuild_item(Scope, T, S, Item, Anns, Links, Approx, Head),
    v2_rebuild(Scope, T, S, Items, Anns, Links, Approx, Tail),
    append(Head, Tail, Out).

v2_rebuild_item(Scope, T, S, Item, Anns, Links, Approx, Out) :-
    ( v2_pp_item(Item, Ctx, PP) ->
        v2_pp_ann(T, S, Scope, Ctx, PP, Ann),
        ( Ann == none ->
            Out = [Item]
        ; v2_ann_inner(Ann, Scope, Anns, Ctx, Inner) ->
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
      functor(Item, anchored, 2),
      arg(1, Item, Ctx),
      arg(2, Item, Inner),
      nonvar(Inner),
      functor(Inner, object, 6) ->
        arg(1, Inner, Q),
        ( v2_linked(Approx, Ctx, Q) ->
            arg(2, Inner, Noun),
            arg(3, Inner, Class),
            arg(4, Inner, UnitField),
            arg(6, Inner, Count),
            Obj = object(Q, Noun, Class, UnitField, about, Count)
        ; Obj = Inner
        ),
        T = t(U, _, _, _, _, _, _),
        v2_noun(Inner, L),
        % nt1 D2: a time quantity carries its unit id beside its cardinality
        ( memberchk(L-Unit, U) ->
            Out = [anchored(Ctx, Obj), anchored(Ctx, '$guideline_unit'(Q, Unit))]
        ; Out = [anchored(Ctx, Obj)]
        )
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
   window becomes a scoped recurrence (q12 D6); a window adds none; a range
   names its partner minimum (q13 D4). */
v2_ann_inner(interval(E, Role, Q, Unit, A), _, _, _,
    '$guideline_interval'(E, Role, Q, Unit, A)).
v2_ann_inner(recurrence(E, _, Q, Unit), _, Anns, Ctx, Inner) :-
    ( v3_window_of(Anns, Ctx, E, L, LUnit, A) ->
        Inner = '$guideline_recurrence_window'(E, Q, Unit, A, L, LUnit)
    ; Inner = '$guideline_recurrence'(E, Q, Unit)
    ).
v2_ann_inner(frequency(E, C, W, Unit), _, _, _, '$guideline_frequency'(E, C, W, Unit)).
v2_ann_inner(order(E, Role, A), _, _, _, '$guideline_order'(E, Role, A)).
v2_ann_inner(range(E, H, _), Scope, Anns, Ctx, '$guideline_range'(L, H)) :-
    v3_partners(Anns, Scope, Ctx, E, [L-_]).

/* ---------- s1 strength pass (contracts s1 D3/D4, s2 D3: documents with a table) ----------
   After a scope's temporal pass (spec emit.rs `strength`): a grade right
   after its root-fact copula; a preference right after its alternative pp
   or, without one, after its copula; a qualifier in place of its adverb, or
   right after its copula. Where = root | antecedent | consequent: a grade
   comes from a root scope alone; an attenuating adverb qualifier on a head
   in context actual outside the antecedent rejects (s2 D3.7). Endpoints
   classify document-wide (v3_plain); a violation reports at its item;
   question mode never runs it. */
s1_strength(S, Where, Items0, Items) :-
    ( nb_current(ace_to_pl_vocabulary, table(U, R, Sp, W, F, A, G, K, _)) ->
        s1_items(Items0, t(U, R, Sp, W, F, A, G), K, S, Where, Items0, Items)
    ; Items = Items0
    ).

s1_items([], _, _, _, _, _, []).
s1_items([Item|Items], T, K, S, Where, Scope, Out) :-
    s1_item(Item, T, K, S, Where, Scope, Head),
    s1_items(Items, T, K, S, Where, Scope, Tail),
    append(Head, Tail, Out).

s1_item(Item, T, K, S, Where, Scope, Out) :-
    ( s1_anchored(Item, Ctx, Inner), s1_is_pred(Inner) ->
        arg(1, Inner, E),
        ( Where == root, s1_grade(T, K, Ctx, Inner, Dim, Value) ->
            arg(3, Inner, Rec),
            arg(4, Inner, Label),
            Gs = [anchored(Ctx, '$guideline_grade'(Rec, Dim, Value, Label))]
        ; Gs = []
        ),
        s2_copula_qualifiers(K, Scope, Ctx, Inner, Qs),
        append(Gs, Qs, GQs),
        s1_preference_of(T, K, Scope, Ctx, Inner, Pref),
        ( Pref = err(Why) ->
            reject(unsupported, preference_shape(Why, S))
        ; Pref = some(P, A), A == none ->
            append([Item|GQs], [anchored(Ctx, '$guideline_preference'(E, P, none))], Out)
        ; Out = [Item|GQs]
        )
    ; s1_anchored(Item, Ctx, Inner),
      functor(Inner, modifier_pp, 3),
      arg(2, Inner, Prep),
      atom(Prep),
      K = k(_, _, _, Alt, _, _, _),
      memberchk(Prep, Alt) ->
        arg(1, Inner, E),
        ( s1_event_pred(Scope, Ctx, E, Pred),
          s1_preference_of(T, K, Scope, Ctx, Pred, some(P, A)) ->
            Out = [Item, anchored(Ctx, '$guideline_preference'(E, P, A))]
        ; Out = [Item]
        )
    ; s1_anchored(Item, Ctx, Inner),
      functor(Inner, modifier_adv, 3),
      arg(2, Inner, Adv),
      atom(Adv),
      K = k(_, _, _, _, Quals, _, _),
      memberchk(Adv-Class, Quals) ->
        arg(1, Inner, E),
        arg(3, Inner, Degree),
        ( Degree \== pos ->
            reject(unsupported, qualifier_shape(degree, S))
        ; s2_attenuating(Class), Where \== antecedent, \+ s2_under_modal(Scope, Ctx) ->
            reject(unsupported, qualifier_shape(asserted, S))
        ; Out = [anchored(Ctx, '$guideline_qualifier'(E, Class, Adv))]
        )
    ; nonvar(Item),
      functor(Item, naf, 2) ->
        arg(1, Item, Dom),
        arg(2, Item, Payload),
        s1_items(Payload, T, K, S, antecedent, Payload, Payload2),
        Out = [naf(Dom, Payload2)]
    ; Out = [Item]
    ).

/* s2 D3.7: the classes whose adverb weakens what its clause asserts. */
s2_attenuating(usuality).
s2_attenuating(ideality).
s2_attenuating(possibility).
s2_attenuating(applicability).

/* s2 D3.7: Ctx lies under a modal operator on its path from actual;
   classical negations alone leave it asserted. */
s2_under_modal(Scope, Ctx) :-
    Ctx \== actual,
    s2_edge(Scope, Ctx, Outer, Op),
    ( memberchk(Op, [should, must, can, may]) ->
        true
    ; s2_under_modal(Scope, Outer)
    ).

s2_edge([Item|Items], Ctx, Outer, Op) :-
    ( nonvar(Item),
      functor(Item, operator, 4),
      arg(3, Item, Inner),
      Inner == Ctx ->
        arg(2, Item, Outer),
        arg(4, Item, Op)
    ; s2_edge(Items, Ctx, Outer, Op)
    ).

/* s2 D3.2: one qualifier per same-context property(P, Adj, pos) of a copula
   be(X, P) whose complement P names no thing (no object declaration) and
   whose Adj is a qualifier lemma, in item order. */
s2_copula_qualifiers(k(_, _, _, _, Quals, _, _), Scope, Ctx, Pred, Qs) :-
    ( functor(Pred, predicate, 4),
      arg(2, Pred, Verb),
      Verb == be,
      arg(4, Pred, P),
      var(P),
      \+ v3_decl(P, _) ->
        arg(1, Pred, E),
        s2_property_qualifiers(Scope, Quals, Ctx, E, P, Qs)
    ; Qs = []
    ).

s2_property_qualifiers([], _, _, _, _, []).
s2_property_qualifiers([Item|Items], Quals, Ctx, E, P, Qs) :-
    ( s1_anchored(Item, C0, Inner),
      C0 == Ctx,
      functor(Inner, property, 3),
      arg(1, Inner, P0),
      P0 == P,
      arg(3, Inner, Pol),
      Pol == pos,
      arg(2, Inner, Adj),
      atom(Adj),
      memberchk(Adj-Class, Quals) ->
        Qs = [anchored(Ctx, '$guideline_qualifier'(E, Class, Adj))|Rest]
    ; Qs = Rest
    ),
    s2_property_qualifiers(Items, Quals, Ctx, E, P, Rest).

s1_anchored(Item, Ctx, Inner) :-
    nonvar(Item),
    functor(Item, anchored, 2),
    arg(1, Item, Ctx),
    arg(2, Item, Inner),
    nonvar(Inner).

s1_is_pred(Inner) :-
    functor(Inner, predicate, Arity),
    Arity >= 3,
    Arity =< 5.

/* E's first same-context predicate/3..5 condition. */
s1_event_pred([Item|Items], Ctx, E, Pred) :-
    ( s1_anchored(Item, C0, Inner),
      C0 == Ctx,
      s1_is_pred(Inner),
      arg(1, Inner, E0),
      E0 == E ->
        Pred = Inner
    ; s1_event_pred(Items, Ctx, E, Pred)
    ).

/* A same-context property(X, A, pos) with A in Lemmas (an approximation lemma,
   q13 D3; a preference adjective, s1 D4). The cut follows every guard, so a late
   match still succeeds; referents compare by ==, never bound. */
pos_property(Lemmas, Items, Ctx, X) :-
    member(Item, Items),
    s1_anchored(Item, C0, Inner),
    C0 == Ctx,
    functor(Inner, property, 3),
    arg(1, Inner, X0),
    X0 == X,
    arg(3, Inner, Degree),
    Degree == pos,
    arg(2, Inner, A),
    atom(A),
    memberchk(A, Lemmas),
    !.

s1_preferred_prop(Items, Prefs, Ctx, P) :-
    pos_property(Prefs, Items, Ctx, P).

/* The objects of E's same-context alternative pps, in item order. */
s1_alternatives([], _, _, _, []).
s1_alternatives([Item|Items], Alt, Ctx, E, Objs) :-
    ( s1_anchored(Item, C0, Inner),
      C0 == Ctx,
      functor(Inner, modifier_pp, 3),
      arg(1, Inner, E0),
      E0 == E,
      arg(2, Inner, Prep),
      atom(Prep),
      memberchk(Prep, Alt) ->
        arg(3, Inner, O),
        Objs = [O|Rest]
    ; Objs = Rest
    ),
    s1_alternatives(Items, Alt, Ctx, E, Rest).

/* D4: some(Preferred, Alternative | none), none, or err(Why). A preference
   copula be(X, P) (P a preference property) prefers X; any other event with
   one alternative pp prefers its second participant. */
s1_preference_of(T, k(_, _, Prefs, Alt, _, _, _), Scope, Ctx, Pred, Result) :-
    arg(1, Pred, E),
    s1_alternatives(Scope, Alt, Ctx, E, Alts),
    length(Alts, NA),
    ( functor(Pred, predicate, 4),
      arg(2, Pred, Verb),
      Verb == be,
      arg(4, Pred, P0),
      s1_preferred_prop(Scope, Prefs, Ctx, P0) ->
        Copula = true
    ; Copula = false
    ),
    ( NA > 1 ->
        Result = err(alternative_count)
    ; Copula == false, NA =:= 0 ->
        Result = none
    ; Copula == false, functor(Pred, predicate, 3) ->
        Result = err(no_preferred)
    ; ( Copula == true -> arg(3, Pred, P) ; arg(4, Pred, P) ),
      ( NA =:= 1 -> Alts = [A] ; A = none ),
      ( v3_plain(T, P), ( NA =:= 0 ; v3_plain(T, A) ) ->
          Result = some(P, A)
      ; Result = err(endpoint)
      )
    ).

/* D3: the grade of a root-fact copula be(R, L) in context actual whose
   complement's noun is a grade noun. */
s1_grade(T, k(Strengths, Evidences, _, _, _, _, _), Ctx, Pred, Dim, Value) :-
    Ctx == actual,
    functor(Pred, predicate, 4),
    arg(2, Pred, Verb),
    Verb == be,
    arg(3, Pred, Rec),
    v3_plain(T, Rec),
    arg(4, Pred, Label),
    var(Label),
    v3_decl(Label, d(Noun, _)),
    atom(Noun),
    ( memberchk(Noun-Value, Strengths) ->
        Dim = strength
    ; memberchk(Noun-Value, Evidences) ->
        Dim = evidence
    ).

/* D3: one grade per dimension per document; Dims = the earlier sentences'. */
s1_grade_dims([], Dims, Dims).
s1_grade_dims([group(_, _, _, Clauses)|Groups], Dims0, Dims) :-
    s1_clause_dims(Clauses, Dims0, Dims1),
    s1_grade_dims(Groups, Dims1, Dims).

s1_clause_dims([], Dims, Dims).
s1_clause_dims([clause(Head, _)|Clauses], Dims0, Dims) :-
    ( functor(Head, guideline_grade, 5) ->
        arg(3, Head, Dim),
        append(Dims0, [Dim], Dims1)
    ; Dims1 = Dims0
    ),
    s1_clause_dims(Clauses, Dims1, Dims).

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
v3_plain(t(U, _, Sp, W, _, _, _), X) :-
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
    T = t(U, R, _, W, F, _, G),
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
        ; atom(P), memberchk(P, G), v2_qty(U, Scope, Ctx, X, _, _) ->
            v3_range(T, S, Scope, Ctx, PP, Ann)
        ; Ordered \== none, v3_plain(T, X) ->
            Ann = order(E, Ordered, X)
        ; Ann = none
        )
    ; Ordered \== none, v3_plain(T, X) ->
        Ann = order(E, Ordered, X)
    ; Ann = none
    ).

/* D6: the only window pp on E, F frame-shaped, F of L (a time quantity), L of A
   (a plain referent). */
v3_window(T, S, Scope, Ctx, PP, O, window(E, F, L, Unit, A)) :-
    T = t(U, _, _, _, _, _, _),
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
    v3_approx_check(T, S, Scope, Ctx, L, F, LO),
    v2_of_links(Scope, Ctx, L, AL),
    ( AL = [A], v3_plain(T, A) ->
        true
    ; reject(unsupported, temporal_shape(window_shape, S))
    ).

/* D5: W an unanchored exact period; the counted item = E's second participant. */
v3_frequency(T, S, Scope, Ctx, PP, Ann) :-
    T = t(U, _, _, _, _, _, _),
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
    T = t(_, _, _, W, _, _, _),
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

/* ---------- q13 D3/D4: about + ranged minimum ---------- */
v3_approximate(T, Scope, Ctx, R) :-
    T = t(_, _, _, _, _, Ap, _),
    pos_property(Ap, Scope, Ctx, R).

/* D3: a claimed quantity Q (object O, frame F) marked approximate needs eq. */
v3_approx_check(T, S, Scope, Ctx, Q, F, O) :-
    ( ( v3_approximate(T, Scope, Ctx, Q) ; v3_approximate(T, Scope, Ctx, F) ),
      arg(5, O, Op),
      Op \== eq ->
        reject(unsupported, temporal_shape(approximate_bound, S))
    ; true
    ).

/* D4: H an exact, unanchored, unapproximated time quantity. */
v3_range(T, S, Scope, Ctx, PP, range(E, H, Unit)) :-
    T = t(U, _, _, _, _, _, _),
    arg(1, PP, E),
    arg(3, PP, H),
    v2_qty(U, Scope, Ctx, H, Unit, HO),
    ( v2_bound_why(HO, Why) -> reject(unsupported, temporal_shape(Why, S)) ; true ),
    arg(5, HO, Op),
    v2_of_links(Scope, Ctx, H, HL),
    ( Op == eq, HL == [], \+ v3_approximate(T, Scope, Ctx, H) ->
        true
    ; reject(unsupported, temporal_shape(range_shape, S))
    ).

/* D4: the at-least interval quantities on E in Ctx, as L-Unit, in item order. */
v3_partners([], _, _, _, []).
v3_partners([C0-Ann|Anns], Scope, Ctx, E, Ps) :-
    ( C0 == Ctx,
      Ann = interval(E0, _, Q, Unit, _),
      E0 == E,
      v2_obj_of(Scope, Ctx, Q, O),
      arg(5, O, Op),
      Op == geq ->
        Ps = [Q-Unit|Rest]
    ; Ps = Rest
    ),
    v3_partners(Anns, Scope, Ctx, E, Rest).

v3_count_of(Scope, Ctx, V, N) :-
    ( v2_obj_of(Scope, Ctx, V, O), arg(6, O, N0), integer(N0) -> N = N0 ; N = 0 ).

/* D4: each range's Ctx-L: its one partner, of its unit, below its count. */
v3_range_lows([], _, _, _, []).
v3_range_lows([Ctx-Ann|Anns], Scope, All, S, Lows) :-
    ( Ann = range(E, H, Unit) ->
        v3_partners(All, Scope, Ctx, E, Ps),
        ( Ps = [L-Unit0],
          Unit0 == Unit,
          v3_count_of(Scope, Ctx, L, NL),
          v3_count_of(Scope, Ctx, H, NH),
          NL < NH ->
            Lows = [Ctx-L|Rest]
        ; reject(unsupported, temporal_shape(range_shape, S))
        )
    ; Lows = Rest
    ),
    v3_range_lows(Anns, Scope, All, S, Rest).

/* D3: the approximate eq quantities, as Ctx-Q. */
v3_abouts([], _, _, []).
v3_abouts([Ctx-Ann|Anns], T, Scope, Out) :-
    ( v3_about_target(Ann, Q, F),
      v2_obj_of(Scope, Ctx, Q, O),
      arg(5, O, Op),
      Op == eq,
      ( v3_approximate(T, Scope, Ctx, Q) -> true ; v3_approximate(T, Scope, Ctx, F) ) ->
        Out = [Ctx-Q|Rest]
    ; Out = Rest
    ),
    v3_abouts(Anns, T, Scope, Rest).

v3_about_target(interval(_, _, Q, _, _), Q, Q).
v3_about_target(recurrence(_, F, Q, _), Q, F).
v3_about_target(window(_, F, L, _, _), L, F).
v3_about_target(frequency(_, _, W, _), W, W).

/* Every record (document /4, question /5) ends in vocabulary(none) |
   vocabulary(sha256(T)), T = the digest of the raw table bytes (c1 D2/D3). */
vocabulary_record(Record0, Record) :-
    ( nb_current(ace_to_pl_vocabulary, table(_, _, _, _, _, _, _, _, Digest)) ->
        Wrapper = vocabulary(sha256(Digest))
    ; Wrapper = vocabulary(none)
    ),
    Record0 =.. List0,
    append(List0, [Wrapper], List),
    Record =.. List.

/* ---------- schema projection ----------

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
   See docs/REFERENCE.md "Compiled Prolog schema". */

/* Uniform declaration block (F2, c1 D2, s1 D1, s2 D1, nt1 D2, cs1 D2): every
   document declares all 20 indicators regardless of population or table. */
v1_indicators([
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
    guideline_recurrence/4,
    guideline_frequency/5,
    guideline_order/4,
    guideline_recurrence_window/7,
    guideline_range/3,
    guideline_grade/5,
    guideline_preference/4,
    guideline_qualifier/4,
    guideline_unit/3,
    guideline_exclusion/1
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
    b_setval(da2_src, Sentences-Lines),
    v1_ulex_reserved_check,
    v1_collision_scan(Conds),
    v1_tag_conditions(Conds, Tagged),
    group_by_sentence(1, SentenceCount, Tagged, Groups),
    v1_translate_groups(Groups, DocId, [], [], Bundles),
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
v1_tag_condition(Cond, S, boxed(Cond)) :-
    v1_exact_group(Cond),
    !,
    inner_sentence(Cond, S).
v1_tag_condition(Cond, _, _) :-
    reject(unsupported, root_condition(Cond)).

/* Root exact group (contract root-exact D1). APE wraps an upper-bounded
   quantifier's scope in a list. A root list joins its sentence's facts
   when every upper-bounded object among its root-context leaves
   (anchored conditions reached through list spines alone; a box, NAF,
   rule or disjunction keeps its own law in the flatten) states
   `exactly N` with N >= 1, and one does: such a count implies that the
   counted things exist, while `at most`, `less than` and `exactly 0`
   admit zero. Tagged boxed(List), the existing flatten unwraps it
   without an operator edge. */
v1_exact_group(Cond) :-
    nonvar(Cond),
    is_list(Cond),
    Cond \== [],
    v1_group_leaves(Cond, Leaves, []),
    forall(member(Leaf, Leaves),
        ( v1_upper_object(Leaf) -> v1_exact_object(Leaf) ; true )),
    once(( member(Leaf, Leaves), v1_exact_object(Leaf) )).

v1_group_leaves(T, [Inner|L], L) :-
    nonvar(T),
    functor(T, -, 2),
    arg(2, T, Anchor),
    anchor_sentence(Anchor, _),
    !,
    arg(1, T, Inner).
v1_group_leaves(T, L0, L) :-
    nonvar(T),
    functor(T, '[|]', 2),
    !,
    arg(1, T, Head),
    arg(2, T, Tail),
    v1_group_leaves(Head, L0, L1),
    v1_group_leaves(Tail, L1, L).
v1_group_leaves(_, L, L).

v1_upper_object(C) :-
    nonvar(C),
    functor(C, object, 6),
    arg(5, C, Op),
    atom(Op),
    memberchk(Op, [exactly, leq, less]).

v1_exact_object(C) :-
    nonvar(C),
    functor(C, object, 6),
    arg(5, C, Op),
    Op == exactly,
    arg(6, C, N),
    integer(N),
    N >= 1.

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
v1_lemma_slot(Sub, Lemma) :-
    functor(Sub, modifier_adv, 3),
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
v1_translate_groups([], _, _, _, []).
v1_translate_groups([S-Group|Groups], DocId, Map0, Dims0,
        [bundle(S, SGroups)|Bundles]) :-
    b_setval(da2_sentence, S),
    v1_sentence(Group, S, DocId, Map0, Map, SGroups0),
    v1_groups_clause_count(SGroups0, ClauseCount),
    ( ClauseCount =:= 0 ->
        reject(unsupported, sentence_shape(Group))
    ; true
    ),
    s1_grade_dims(SGroups0, Dims0, Dims),
    ( is_set(Dims) ->
        true
    ; reject(unsupported, grade_shape(duplicate_dimension, S))
    ),
    cs1_scope_pass(Group, S, SGroups0, SGroups),
    v1_translate_groups(Groups, DocId, Map, Dims, Bundles).

/* ---------- cs1 scope exclusions (contract cs1 D2-D5: a table with the
   scope + exclusion pair) ----------
   After a sentence's translation and the duplicate-grade check (so every
   earlier reject reports first): an exclusion rule (`If C then it is false
   that a <scope noun> <exclusion verb>s ...`) gains one record clause
   guideline_exclusion(Key) :- Body after its clauses; every
   should|must|may edge clause ends with \+ guideline_exclusion(Key);
   rejects scope_shape(Why, S). DRS read via arg/3 + == only. */
cs1_scope(Noun, Verb, Key) :-
    nb_current(ace_to_pl_vocabulary, table(_, _, _, _, _, _, _,
        k(_, _, _, _, _, [scope(Noun, Key, _)], [exclusion(Verb, _)]), _)).

cs1_scope_pass(Group, S, SGroups0, SGroups) :-
    ( cs1_scope(Noun, Verb, Key) ->
        cs1_class(Group, Noun, Verb, Class),
        ( Class = reject(Why) ->
            reject(unsupported, scope_shape(Why, S))
        ; Class == exclusion ->
            maplist(cs1_group(Key, true), SGroups0, SGroups)
        ; maplist(cs1_group(Key, false), SGroups0, SGroups)
        )
    ; SGroups = SGroups0
    ).

/* D4 order: rules deontic_condition, negated_deontic, subject, consequent;
   roots root_deontic, root. */
cs1_class([rule(Ante, Cons)], Noun, Verb, Class) :-
    !,
    v1_curry(Ante, Cons, Segs, FinalCons),
    ( member(seg(_, Conds), Segs),
      cs1_has_deontic(Conds) ->
        Class = reject(deontic_condition)
    ; cs1_negated_deontic(FinalCons) ->
        Class = reject(negated_deontic)
    ; arg(2, FinalCons, CConds),
      cs1_verb_box(CConds, Verb, BDom, BConds) ->
        cs1_verb_leaf(BConds, Verb, Pred),
        arg(3, Pred, X),
        ( \+ cs1_var_member(X, BDom) ->
            Class = reject(subject)
        ; \+ cs1_noun_in(BConds, X, Noun) ->
            Class = reject(subject)
        ; CConds = [_],
          cs1_exclusion_box(BDom, BConds, Noun, Verb) ->
            Class = exclusion
        ; Class = reject(consequent)
        )
    ; Class = plain
    ).
cs1_class(Group, _, Verb, Class) :-
    ( member(Item, Group),
      cs1_root_deontic(Item) ->
        Class = reject(root_deontic)
    ; member(Item, Group),
      nonvar(Item),
      Item = boxed(C),
      cs1_box_names_verb(C, Verb) ->
        Class = reject(root)
    ; Class = plain
    ).

cs1_deontic_box(T) :-
    compound(T),
    functor(T, Op, 1),
    memberchk(Op, [should, must, may]).

cs1_has_deontic(T) :-
    sub_term(Sub, T),
    nonvar(Sub),
    cs1_deontic_box(Sub),
    !.

cs1_negated_deontic(T) :-
    sub_term(Sub, T),
    nonvar(Sub),
    compound(Sub),
    functor(Sub, -, 1),
    arg(1, Sub, Box),
    cs1_has_deontic(Box),
    !.

cs1_root_deontic(anchored(Inner)) :-
    cs1_has_deontic(Inner).
cs1_root_deontic(boxed(C)) :-
    cs1_has_deontic(C).

cs1_leaf(C, Leaf) :-
    nonvar(C),
    functor(C, -, 2),
    arg(2, C, Anchor),
    nonvar(Anchor),
    anchor_sentence(Anchor, _),
    arg(1, C, Leaf).

cs1_var_member(X, Dom) :-
    member(V, Dom),
    V == X,
    !.

/* A predicate/3..5 leaf of the exclusion verb (a transitive one reaches
   D3, which rejects it as consequent); D3 itself requires arity 3. */
cs1_verb_pred(Leaf, Verb) :-
    nonvar(Leaf),
    functor(Leaf, predicate, Arity),
    Arity >= 3,
    Arity =< 5,
    arg(2, Leaf, V),
    V == Verb.

/* A box's leaves through upper-bound group lists (spec group_leaves:
   anchored conditions via list spines alone), so `exactly|at most|less than
   N <scope noun>s` reaches consequent instead of compiling as plain data. */
cs1_group_leaves(T, Leaves) :-
    ( cs1_leaf(T, Leaf) ->
        Leaves = [Leaf]
    ; compound(T),
      compound_name_arity(T, '[|]', 2) ->
        arg(1, T, H),
        arg(2, T, Tl),
        cs1_group_leaves(H, L1),
        cs1_group_leaves(Tl, L2),
        append(L1, L2, Leaves)
    ; Leaves = []
    ).

cs1_names_verb(BConds, Verb) :-
    cs1_group_leaves(BConds, Leaves),
    member(Leaf, Leaves),
    cs1_verb_pred(Leaf, Verb),
    !.

/* The first top-level classical negation box of the consequent that names
   the exclusion verb. */
cs1_verb_box(CConds, Verb, BDom, BConds) :-
    member(C, CConds),
    cs1_box_names_verb(C, Verb),
    !,
    arg(1, C, Box),
    arg(1, Box, BDom),
    arg(2, Box, BConds).

cs1_box_names_verb(C, Verb) :-
    nonvar(C),
    functor(C, -, 1),
    arg(1, C, Box),
    nonvar(Box),
    functor(Box, drs, 2),
    arg(2, Box, BConds),
    is_list(BConds),
    cs1_names_verb(BConds, Verb).

cs1_verb_leaf(BConds, Verb, Leaf) :-
    cs1_group_leaves(BConds, Leaves),
    member(Leaf, Leaves),
    cs1_verb_pred(Leaf, Verb),
    !.

/* The noun of X's first object leaf is the scope noun. */
cs1_noun_in(BConds, X, Noun) :-
    cs1_group_leaves(BConds, Leaves),
    member(Leaf, Leaves),
    nonvar(Leaf),
    functor(Leaf, object, 6),
    arg(1, Leaf, X1),
    X1 == X,
    !,
    arg(2, Leaf, N),
    N == Noun.

/* D3: object(X, Noun, countable, na, eq, 1), predicate(E, Verb, X), then
   modifier_pp(E, _, Y) leaves with Y outside the box. */
cs1_exclusion_box(BDom, BConds, Noun, Verb) :-
    cs1_leaves(BConds, Leaves),
    Leaves = [O, P|Pps],
    functor(O, object, 6),
    arg(1, O, X),
    arg(2, O, N), N == Noun,
    arg(3, O, Cl), Cl == countable,
    arg(4, O, U), U == na,
    arg(5, O, Cmp), Cmp == eq,
    arg(6, O, Cnt), Cnt == 1,
    functor(P, predicate, 3),
    cs1_verb_pred(P, Verb),
    arg(1, P, E),
    arg(3, P, X2), X2 == X,
    cs1_var_member(X, BDom),
    cs1_var_member(E, BDom),
    forall(member(Pp, Pps),
        ( nonvar(Pp),
          functor(Pp, modifier_pp, 3),
          arg(1, Pp, E1), E1 == E,
          arg(3, Pp, Y),
          \+ cs1_var_member(Y, BDom)
        )).

cs1_leaves([], []).
cs1_leaves([C|Cs], [Leaf|Leaves]) :-
    cs1_leaf(C, Leaf),
    cs1_leaves(Cs, Leaves).

cs1_group(Key, Excl, group(Kind, K, Pairs, Clauses0),
        group(Kind, K, Pairs, Clauses)) :-
    maplist(cs1_guard(Key), Clauses0, Clauses1),
    ( Excl == true,
      Clauses0 = [clause(_, Body)|_] ->
        append(Clauses1, [clause(guideline_exclusion(Key), Body)], Clauses)
    ; Clauses = Clauses1
    ).

cs1_guard(Key, clause(Head, Goals), clause(Head, Goals2)) :-
    ( functor(Head, guideline_operator, 3),
      arg(3, Head, Op),
      atom(Op),
      memberchk(Op, [should, must, may]) ->
        append(Goals, [naf([], [guideline_exclusion(Key)])], Goals2)
    ; Goals2 = Goals
    ).

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
    v2_annotate(S, Items0, Items1),
    s1_strength(S, root, Items1, Items),
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

/* ---------- comparison conditions (contract nt1 D3) ----------
   Rule antecedents (every Horn-split variant) and question goals: an object
   whose comparison bounds its count compares the stated count of a matching
   fact with that bound through guideline_satisfies/4; a time quantity (the
   referent D2 gave a '$guideline_unit' item) leaves its entity literal out and
   compares across units through guideline_satisfies/6, its unit and the unit
   slots of the annotations that name it sharing one variable. Each converted
   referent owns fresh variables v(S, N, V); a NAF payload's join its domain. */
nt1_convertible(Op) :-
    atom(Op),
    memberchk(Op, [exactly, geq, greater, leq, less]).

nt1_object_cmp(Item, Ctx, Inner) :-
    nonvar(Item),
    functor(Item, anchored, 2),
    arg(1, Item, Ctx),
    arg(2, Item, Inner),
    nonvar(Inner),
    functor(Inner, object, 6),
    arg(5, Inner, Op),
    nt1_convertible(Op).

nt1_builtin_goal(Goal) :-
    nonvar(Goal),
    ( functor(Goal, guideline_satisfies, 4)
    ; functor(Goal, guideline_satisfies, 6)
    ; functor(Goal, guideline_product, 6)
    ; functor(Goal, guideline_quotient, 6)
    ),
    !.

/* R-v(S, N, V) per convertible object, preorder, NAF payloads included. */
nt1_vars([], []).
nt1_vars([Item|Items], Vars) :-
    ( nt1_object_cmp(Item, _, Inner) ->
        arg(1, Inner, R),
        Vars = [R-v(_, _, _)|Rest]
    ; nonvar(Item),
      functor(Item, naf, 2) ->
        arg(2, Item, Payload),
        nt1_vars(Payload, PV),
        append(PV, Rest, Vars)
    ; Vars = Rest
    ),
    nt1_vars(Items, Rest).

nt1_unit_of([Item|Items], R, Unit) :-
    ( nonvar(Item),
      functor(Item, anchored, 2),
      arg(2, Item, Inner),
      nonvar(Inner),
      functor(Inner, '$guideline_unit', 2),
      arg(1, Inner, R0),
      R0 == R ->
        arg(2, Inner, Unit)
    ; nt1_unit_of(Items, R, Unit)
    ).

nt1_object_of([Item|Items], R, Obj) :-
    ( nonvar(Item),
      functor(Item, anchored, 2),
      arg(2, Item, Inner),
      nonvar(Inner),
      functor(Inner, object, 6),
      arg(1, Inner, R0),
      R0 == R ->
        Obj = Inner
    ; nt1_object_of(Items, R, Obj)
    ).

/* The converted time quantities: per scope, a convertible object whose scope
   holds its unit item. */
nt1_times(Scope, Times) :-
    nt1_scope_times(Scope, Scope, Here),
    nt1_nested_times(Scope, Nested),
    append(Here, Nested, Times).

nt1_scope_times(_, [], []).
nt1_scope_times(Scope, [Item|Items], Times) :-
    ( nt1_object_cmp(Item, _, Inner),
      arg(1, Inner, R),
      nt1_unit_of(Scope, R, _) ->
        Times = [R|Rest]
    ; Times = Rest
    ),
    nt1_scope_times(Scope, Items, Rest).

nt1_nested_times([], []).
nt1_nested_times([Item|Items], Times) :-
    ( nonvar(Item),
      functor(Item, naf, 2) ->
        arg(2, Item, Payload),
        nt1_times(Payload, Here),
        append(Here, Rest, Times)
    ; Times = Rest
    ),
    nt1_nested_times(Items, Rest).

nt1_compare(Items0, Items) :-
    nt1_vars(Items0, Vars),
    nt1_times(Items0, Times),
    nt1_compare_list(Items0, Vars, Times, Items0, Items).

nt1_compare_list(_, _, _, [], []).
nt1_compare_list(Scope, Vars, Times, [Item|Items], Out) :-
    nt1_compare_item(Scope, Vars, Times, Item, Head),
    nt1_compare_list(Scope, Vars, Times, Items, Tail),
    append(Head, Tail, Out).

nt1_compare_item(Scope, Vars, Times, Item, Out) :-
    ( nt1_object_cmp(Item, Ctx, Inner) ->
        arg(1, Inner, R),
        arg(2, Inner, Noun),
        arg(3, Inner, Class),
        arg(4, Inner, UnitField),
        arg(5, Inner, Op),
        arg(6, Inner, Count),
        v1_lookup(Vars, R, v(S, N, _)),
        ( strict_member(R, Times) ->
            Out = [anchored(Ctx, '$guideline_measure'(R, UnitField, S, N))]
        ; Out = [anchored(Ctx,
              '$guideline_bound'(R, Noun, Class, UnitField, S, N, Op, Count))]
        )
    ; nonvar(Item),
      functor(Item, anchored, 2),
      arg(2, Item, Inner),
      nonvar(Inner),
      functor(Inner, '$guideline_unit', 2),
      arg(1, Inner, R),
      strict_member(R, Times),
      nt1_object_of(Scope, R, Obj) ->
        arg(1, Item, Ctx),
        arg(2, Inner, Unit),
        arg(5, Obj, Op),
        arg(6, Obj, Count),
        v1_lookup(Vars, R, v(S, N, V)),
        Out = [anchored(Ctx, '$guideline_unit'(R, V)),
               anchored(Ctx, '$guideline_scaled'(S, N, V, Op, Count, Unit))]
    ; nonvar(Item),
      functor(Item, anchored, 2) ->
        arg(1, Item, Ctx),
        arg(2, Item, Inner),
        nt1_relink(Vars, Times, Inner, Inner2),
        Out = [anchored(Ctx, Inner2)]
    ; nonvar(Item),
      functor(Item, naf, 2) ->
        arg(1, Item, Dom),
        arg(2, Item, Payload),
        nt1_vars(Payload, PV),
        nt1_fresh(PV, Vars, Fresh),
        append(Dom, Fresh, Dom2),
        nt1_compare_list(Payload, Vars, Times, Payload, Payload2),
        Out = [naf(Dom2, Payload2)]
    ; Out = [Item]
    ).

/* A payload's conversion variables, as the body-wide table names them. */
nt1_fresh([], _, []).
nt1_fresh([R-_|PV], Vars, [S, N, V|Fresh]) :-
    v1_lookup(Vars, R, v(S, N, V)),
    nt1_fresh(PV, Vars, Fresh).

/* An annotation naming a converted time quantity carries its unit variable. */
nt1_relink(Vars, Times, Inner, Out) :-
    ( nonvar(Inner),
      functor(Inner, '$guideline_interval', 5),
      arg(3, Inner, Q),
      strict_member(Q, Times) ->
        Inner = '$guideline_interval'(E, Role, Q, _, A),
        v1_lookup(Vars, Q, v(_, _, V)),
        Out = '$guideline_interval'(E, Role, Q, V, A)
    ; nonvar(Inner),
      functor(Inner, '$guideline_recurrence', 3),
      arg(2, Inner, Q),
      strict_member(Q, Times) ->
        Inner = '$guideline_recurrence'(E, Q, _),
        v1_lookup(Vars, Q, v(_, _, V)),
        Out = '$guideline_recurrence'(E, Q, V)
    ; nonvar(Inner),
      functor(Inner, '$guideline_recurrence_window', 6) ->
        Inner = '$guideline_recurrence_window'(E, Q, GU0, A, L, LU0),
        nt1_unit_slot(Vars, Times, Q, GU0, GU),
        nt1_unit_slot(Vars, Times, L, LU0, LU),
        Out = '$guideline_recurrence_window'(E, Q, GU, A, L, LU)
    ; Out = Inner
    ).

nt1_unit_slot(Vars, Times, Q, U0, U) :-
    ( strict_member(Q, Times) -> v1_lookup(Vars, Q, v(_, _, U)) ; U = U0 ).

/* The per-document replay module + the formula witness state the comparison
   and arithmetic built-ins exactly as guidelines/schema.pl does, clause for
   clause (contracts nt1 D9, D10, da1 D10, da2 D7). */
nt1_schema_clauses([
    (guideline_satisfies(S, N, B, M) :- atom(S), atom(B), guideline_number(N, P, Q), guideline_number(M, R, T), X is P * T, Y is R * Q, guideline_within(S, X, B, Y)),
    (guideline_satisfies(S, N, U, B, M, V) :- atom(S), atom(U), atom(B), atom(V), guideline_number(N, P, Q), guideline_number(M, R, T), guideline_scale(U, G, A), guideline_scale(V, G, C), X is P * A * T, Y is R * C * Q, guideline_within(S, X, B, Y)),
    (guideline_product(S1, N1, S2, N2, S, N) :- guideline_set(S1, N1, A), guideline_set(S2, N2, B), guideline_times(A, B, C), guideline_spell(C, S0, N0), S = S0, N = N0),
    (guideline_quotient(S1, N1, S2, N2, S, N) :- guideline_set(S1, N1, A), guideline_set(S2, N2, B), guideline_inverse(B, D), guideline_times(A, D, C), guideline_spell(C, S0, N0), S = S0, N = N0),
    (guideline_number(N, N, 1) :- integer(N)),
    (guideline_number(N, P, Q) :- compound(N), functor(N, rational, 2), arg(1, N, P), arg(2, N, Q), integer(P), integer(Q), Q >= 2, A is abs(P), guideline_gcd(A, Q, 1)),
    guideline_gcd(A, 0, A),
    (guideline_gcd(A, B, G) :- B > 0, C is A mod B, guideline_gcd(B, C, G)),
    (guideline_set(S, N, V) :- atom(S), guideline_number(N, P, Q), guideline_ray(S, P, Q, V)),
    guideline_ray(exactly, P, Q, point(P, Q)),
    guideline_ray(eq, P, Q, lower(P, Q, true)),
    guideline_ray(geq, P, Q, lower(P, Q, true)),
    guideline_ray(greater, P, Q, lower(P, Q, false)),
    guideline_ray(leq, P, Q, upper(P, Q, true)),
    guideline_ray(less, P, Q, upper(P, Q, false)),
    (guideline_times(point(X, _), _, point(0, 1)) :- X =:= 0),
    (guideline_times(A, point(Y, _), point(0, 1)) :- Y =:= 0, guideline_nonzero(A)),
    (guideline_times(point(X, D), B, C) :- X =\= 0, guideline_nonzero(B), guideline_stretch(X, D, B, C)),
    (guideline_times(A, point(Y, E), C) :- Y =\= 0, guideline_ray_kind(A), guideline_stretch(Y, E, A, C)),
    (guideline_times(lower(X, D, F), lower(Y, E, G), lower(Z, W, K)) :- X >= 0, Y >= 0, Z is X * Y, W is D * E, guideline_att(X, F, Y, G, K)),
    (guideline_times(upper(X, D, F), upper(Y, E, G), lower(Z, W, K)) :- X =< 0, Y =< 0, Z is X * Y, W is D * E, guideline_att(X, F, Y, G, K)),
    (guideline_times(lower(X, D, F), upper(Y, E, G), upper(Z, W, K)) :- X >= 0, Y =< 0, Z is X * Y, W is D * E, guideline_att(X, F, Y, G, K)),
    (guideline_times(upper(X, D, F), lower(Y, E, G), upper(Z, W, K)) :- X =< 0, Y >= 0, Z is X * Y, W is D * E, guideline_att(X, F, Y, G, K)),
    (guideline_nonzero(point(X, _)) :- X =\= 0),
    guideline_nonzero(lower(_, _, _)),
    guideline_nonzero(upper(_, _, _)),
    guideline_ray_kind(lower(_, _, _)),
    guideline_ray_kind(upper(_, _, _)),
    (guideline_stretch(X, D, point(Y, E), point(Z, W)) :- Z is X * Y, W is D * E),
    (guideline_stretch(X, D, lower(Y, E, F), lower(Z, W, F)) :- X > 0, Z is X * Y, W is D * E),
    (guideline_stretch(X, D, lower(Y, E, F), upper(Z, W, F)) :- X < 0, Z is X * Y, W is D * E),
    (guideline_stretch(X, D, upper(Y, E, F), upper(Z, W, F)) :- X > 0, Z is X * Y, W is D * E),
    (guideline_stretch(X, D, upper(Y, E, F), lower(Z, W, F)) :- X < 0, Z is X * Y, W is D * E),
    guideline_att(_, true, _, true, true),
    (guideline_att(X, true, _, false, K) :- guideline_zero(X, K)),
    (guideline_att(_, false, Y, true, K) :- guideline_zero(Y, K)),
    guideline_att(_, false, _, false, false),
    (guideline_zero(X, true) :- X =:= 0),
    (guideline_zero(X, false) :- X =\= 0),
    (guideline_inverse(point(Y, E), point(E, Y)) :- Y > 0),
    (guideline_inverse(point(Y, E), point(F, Z)) :- Y < 0, F is -E, Z is -Y),
    (guideline_spell(point(P, Q), exactly, N) :- guideline_canon(P, Q, N)),
    (guideline_spell(lower(P, Q, true), geq, N) :- guideline_canon(P, Q, N)),
    (guideline_spell(lower(P, Q, false), greater, N) :- guideline_canon(P, Q, N)),
    (guideline_spell(upper(P, Q, true), leq, N) :- guideline_canon(P, Q, N)),
    (guideline_spell(upper(P, Q, false), less, N) :- guideline_canon(P, Q, N)),
    (guideline_canon(P, Q, N) :- A is abs(P), guideline_gcd(A, Q, G), R is P // G, T is Q // G, guideline_make(R, T, N)),
    guideline_make(P, 1, P),
    (guideline_make(P, Q, rational(P, Q)) :- Q > 1),
    (guideline_within(eq, X, geq, Y) :- X >= Y),
    (guideline_within(exactly, X, geq, Y) :- X >= Y),
    (guideline_within(geq, X, geq, Y) :- X >= Y),
    (guideline_within(greater, X, geq, Y) :- X >= Y),
    (guideline_within(eq, X, greater, Y) :- X > Y),
    (guideline_within(exactly, X, greater, Y) :- X > Y),
    (guideline_within(geq, X, greater, Y) :- X > Y),
    (guideline_within(greater, X, greater, Y) :- X >= Y),
    (guideline_within(exactly, X, leq, Y) :- X =< Y),
    (guideline_within(leq, X, leq, Y) :- X =< Y),
    (guideline_within(less, X, leq, Y) :- X =< Y),
    (guideline_within(exactly, X, less, Y) :- X < Y),
    (guideline_within(leq, X, less, Y) :- X < Y),
    (guideline_within(less, X, less, Y) :- X =< Y),
    (guideline_within(exactly, X, exactly, Y) :- X =:= Y),
    guideline_scale(second, time, 1),
    guideline_scale(minute, time, 60),
    guideline_scale(hour, time, 3600),
    guideline_scale(day, time, 86400),
    guideline_scale(week, time, 604800),
    guideline_scale(month, calendar, 1),
    guideline_scale(year, calendar, 12)
]).

nt1_assert_schema :-
    nt1_schema_clauses(Clauses),
    forall(member(C, Clauses), assertz(ace_to_pl_proof_world:C)).

/* ---------- dose arithmetic formulas (contract da2) ----------
   A formula `Y = E` at the top level of a rule's final consequent computes
   its target's count from antecedent quantities and literals through the
   schema's guideline_product/6 + guideline_quotient/6 (contract da1): the
   chain follows every antecedent goal and its result is the target's
   cardinality head. For formula quantities alone, a unit-noun amount stands
   for its count and `some` sets no bound (APE reads `some` + plural as at
   least 2). A literal's value comes from the numeral bytes of the ACE line:
   APE's floats lose the source spelling. Spec = emit.rs § dose arithmetic
   formulas; rejects formula_shape(Why, S); DRS read via arg/3 + == only. */

da2_seg_conds(seg(_, Conds), Conds).

/* Raw = the rule's antecedent + consequent condition terms (anchor search);
   Ordered/Map/DocId/K feed the witness pairs of the converted antecedent. */
da2_formula_pass(Raw, S, AItems, CItems, Ordered, Map, DocId, K,
        AFree, CRest, Chain, Pairs) :-
    ( ( da2_misplaced(AItems, false) ; da2_misplaced(CItems, true) ) ->
        reject(unsupported, formula_shape(position, S))
    ; true
    ),
    da2_top_formulas(CItems, Fs),
    da2_formula_refs(Fs, Ops),
    da2_anchored_list(Raw, [], Rev),
    reverse(Rev, An),
    b_getval(da2_src, Sentences-Lines),
    nth1(S, Sentences, Toks),
    nth1(S, Lines, Line),
    da2_free_items(AItems, Ops, Toks, An, AFree),
    exclude(da2_top_formula, CItems, CItems1),
    v1_witness_pairs(AFree, Ordered, Map, DocId, S, K, Ops, Pairs0),
    da2_steps(Fs, Fs, S, Toks, Line, An, AItems, CItems1, CRest, [], Chain,
        Pairs0, Pairs).

da2_misplaced([Item|Items], TopOk) :-
    ( da2_misplaced_item(Item, TopOk) ->
        true
    ; da2_misplaced(Items, TopOk)
    ).

da2_misplaced_item(anchored(C, Inner), TopOk) :-
    nonvar(Inner),
    functor(Inner, formula, 3),
    \+ ( TopOk == true, C == actual ).
da2_misplaced_item(naf(_, Payload), _) :-
    da2_misplaced(Payload, false).

da2_top_formula(Item) :-
    nonvar(Item),
    functor(Item, anchored, 2),
    arg(1, Item, C),
    C == actual,
    arg(2, Item, Inner),
    nonvar(Inner),
    functor(Inner, formula, 3).

da2_top_formulas([], []).
da2_top_formulas([Item|Items], Fs) :-
    ( da2_top_formula(Item) ->
        arg(2, Item, F),
        Fs = [F|Fs1]
    ; Fs = Fs1
    ),
    da2_top_formulas(Items, Fs1).

% Formula leaves that are referents, across the formulas.
da2_formula_refs([], []).
da2_formula_refs([F|Fs], Ops) :-
    arg(3, F, E),
    da2_leaves(E, Ls),
    include(var, Ls, Vs),
    da2_formula_refs(Fs, Rest),
    append(Vs, Rest, Ops).

da2_leaves(E, Ls) :-
    ( nonvar(E),
      functor(E, expr, 3) ->
        arg(2, E, L),
        arg(3, E, R),
        da2_leaves(L, LL),
        da2_leaves(R, RL),
        append(LL, RL, Ls)
    ; Ls = [E]
    ).

/* Anchored conditions reachable through carriers, each with its token
   index, in preorder (reversed accumulator). */
da2_anchored_list([], Acc, Acc).
da2_anchored_list([T|Ts], Acc0, Acc) :-
    da2_anchored(T, Acc0, Acc1),
    da2_anchored_list(Ts, Acc1, Acc).

da2_anchored(T, Acc0, Acc) :-
    ( var(T) ->
        Acc = Acc0
    ; da2_anchor(T, Inner, K) ->
        da2_anchored(Inner, [Inner-K|Acc0], Acc)
    ; da2_carrier(T) ->
        T =.. [_|Args],
        da2_anchored_list(Args, Acc0, Acc)
    ; Acc = Acc0
    ).

da2_anchor(T, Inner, K) :-
    compound(T),
    functor(T, -, 2),
    arg(2, T, A),
    nonvar(A),
    functor(A, /, 2),
    arg(1, A, Sn),
    integer(Sn),
    arg(2, A, K),
    integer(K),
    arg(1, T, Inner).

da2_carrier(T) :-
    compound(T),
    compound_name_arity(T, N, A),
    da2_carrier(N, A),
    !.

da2_carrier(drs, 2).
da2_carrier(question, 1).
da2_carrier(should, 1).
da2_carrier(must, 1).
da2_carrier(can, 1).
da2_carrier(may, 1).
da2_carrier(-, 1).
da2_carrier(~, 1).
da2_carrier(=>, 2).
da2_carrier(v, 2).
da2_carrier('[|]', 2).

da2_object_token([Inner-K0|An], R, K) :-
    ( nonvar(Inner),
      functor(Inner, object, 6),
      arg(1, Inner, R1),
      R1 == R ->
        K = K0
    ; da2_object_token(An, R, K)
    ).

da2_formula_token([Inner-K0|An], F, K) :-
    ( Inner == F ->
        K = K0
    ; da2_formula_token(An, F, K)
    ).

% The token before R's noun is `some` (no adjective between).
da2_some_before(Toks, An, R) :-
    da2_object_token(An, R, K),
    K >= 1,
    P is K - 1,
    nth0(P, Toks, T),
    T == some.

/* A `some` operand loses its nt1 filter (extension 2). */
da2_free_items([], _, _, _, []).
da2_free_items([Item|Items], Ops, Toks, An, [Item1|Items1]) :-
    ( nonvar(Item),
      functor(Item, anchored, 2),
      arg(1, Item, C),
      C == actual,
      arg(2, Item, Inner),
      nonvar(Inner),
      functor(Inner, '$guideline_bound', 8),
      arg(1, Inner, R),
      strict_member(R, Ops),
      arg(7, Inner, Op),
      Op == geq,
      arg(8, Inner, Count),
      Count == 2,
      da2_some_before(Toks, An, R) ->
        arg(2, Inner, Noun),
        arg(3, Inner, Class),
        arg(4, Inner, Unit),
        arg(5, Inner, SV),
        arg(6, Inner, NV),
        Item1 = anchored(C, '$guideline_amount'(R, Noun, Class, Unit, SV, NV))
    ; Item1 = Item
    ),
    da2_free_items(Items, Ops, Toks, An, Items1).

da2_steps([], _, _, _, _, _, _, CItems, CItems, Chain, Chain, Pairs, Pairs).
da2_steps([F|Fs], All, S, Toks, Line, An, AItems, CItems0, CItems, Chain0,
        Chain, Pairs0, Pairs) :-
    da2_step(F, All, S, Toks, Line, An, AItems, CItems0, CItems1, Goals,
        Pairs0, Pairs1),
    append(Chain0, Goals, Chain1),
    da2_steps(Fs, All, S, Toks, Line, An, AItems, CItems1, CItems, Chain1,
        Chain, Pairs1, Pairs).

/* One formula after the earlier ones: checks in contract order, then its
   chain, its witness, its target rewritten. */
da2_step(F, All, S, Toks, Line, An, AItems, CItems0, CItems, Goals, Pairs0,
        Pairs) :-
    arg(1, F, Y),
    arg(2, F, Rel),
    arg(3, F, E),
    ( Rel == (=) ->
        true
    ; reject(unsupported, formula_shape(relation, S))
    ),
    ( nonvar(E),
      functor(E, expr, 3),
      da2_ops_ok(E) ->
        true
    ; reject(unsupported, formula_shape(operator, S))
    ),
    ( var(Y),
      da2_formula_refs(All, Refs),
      \+ strict_member(Y, Refs),
      da2_targets(All, Y, 1),
      da2_target_ok(CItems0, Toks, An, Y) ->
        true
    ; reject(unsupported, formula_shape(target, S))
    ),
    da2_leaves(E, Leaves),
    include(var, Leaves, Vars),
    da2_operand_pairs(Vars, AItems, S, Ps),
    include(da2_literal, Leaves, Lits),
    ( da2_formula_token(An, F, KF),
      da2_literal_values(Toks, Line, KF, Lits, Vals) ->
        true
    ; reject(unsupported, formula_shape(literal, S))
    ),
    da2_chain(E, Ps, Vals, _, Goals, Out),
    da2_chain_pairs(Goals, Pairs0, S, Pairs),
    da2_rewrite_target(CItems0, Y, Out, CItems).

da2_ops_ok(E) :-
    var(E),
    !.
da2_ops_ok(E) :-
    functor(E, expr, 3),
    !,
    arg(1, E, Op),
    ( Op == (*) -> true ; Op == (/) ),
    arg(2, E, L),
    arg(3, E, R),
    da2_ops_ok(L),
    da2_ops_ok(R).
da2_ops_ok(E) :-
    da2_literal(E).

da2_literal(E) :-
    nonvar(E),
    ( functor(E, int, 1) ; functor(E, real, 1) ),
    !.

da2_targets([], _, 0).
da2_targets([F|Fs], Y, N) :-
    da2_targets(Fs, Y, N0),
    arg(1, F, Y1),
    ( Y1 == Y -> N is N0 + 1 ; N = N0 ).

da2_target_ok(CItems, Toks, An, Y) :-
    da2_top_object(CItems, Y, O),
    arg(3, O, Class),
    Class == countable,
    arg(4, O, Unit),
    Unit == na,
    arg(5, O, Op),
    Op == geq,
    arg(6, O, N),
    N == 2,
    da2_some_before(Toks, An, Y).

da2_top_object([Item|Items], Y, O) :-
    ( nonvar(Item),
      functor(Item, anchored, 2),
      arg(1, Item, C),
      C == actual,
      arg(2, Item, Inner),
      nonvar(Inner),
      functor(Inner, object, 6),
      arg(1, Inner, R),
      R == Y ->
        O = Inner
    ; da2_top_object(Items, Y, O)
    ).

/* The (S, N) pair of each operand leaf, else the reject: every leaf's
   operand law before any leaf's bound. */
da2_operand_pairs(Rs, AItems, S, Ps) :-
    ( da2_operands_ok(Rs, AItems) ->
        true
    ; reject(unsupported, formula_shape(operand, S))
    ),
    da2_bound_pairs(Rs, AItems, S, Ps).

/* A top-level antecedent count (countable, Unit na, so no APE measurement)
   that is no time quantity (a plain-number one keeps its object beside
   '$guideline_unit'). */
da2_operands_ok([], _).
da2_operands_ok([R|Rs], AItems) :-
    da2_operand_item(AItems, R, C),
    arg(3, C, Class),
    Class == countable,
    arg(4, C, Unit),
    Unit == na,
    \+ nt1_unit_of(AItems, R, _),
    da2_operands_ok(Rs, AItems).

da2_bound_pairs([], _, _, []).
da2_bound_pairs([R|Rs], AItems, S, [R-Pair|Ps]) :-
    da2_operand_item(AItems, R, C),
    ( functor(C, '$guideline_bound', 8) ->
        arg(5, C, SV),
        arg(6, C, NV),
        Pair = SV-NV
    ; reject(unsupported, formula_shape(operand_bound, S))
    ),
    da2_bound_pairs(Rs, AItems, S, Ps).

da2_operand_item([Item|Items], R, C) :-
    ( nonvar(Item),
      functor(Item, anchored, 2),
      arg(1, Item, Ctx),
      Ctx == actual,
      arg(2, Item, Inner),
      nonvar(Inner),
      ( functor(Inner, object, 6) ; functor(Inner, '$guideline_bound', 8) ),
      arg(1, Inner, R1),
      R1 == R ->
        C = Inner
    ; da2_operand_item(Items, R, C)
    ).

/* The postorder built-in chain: an operand leaf = its pair, a literal leaf
   = (exactly, value); fresh output variables per node. */
da2_chain(E, Ps, Vals0, Vals, Goals, Out) :-
    ( nonvar(E),
      functor(E, expr, 3) ->
        arg(1, E, Op),
        arg(2, E, L),
        arg(3, E, R),
        da2_chain(L, Ps, Vals0, Vals1, LG, LS-LN),
        da2_chain(R, Ps, Vals1, Vals, RG, RS-RN),
        ( Op == (*) -> Name = guideline_product ; Name = guideline_quotient ),
        G =.. [Name, LS, LN, RS, RN, OS, ON],
        append(LG, RG, G0),
        append(G0, [G], Goals),
        Out = OS-ON
    ; da2_literal(E) ->
        Vals0 = [V|Vals],
        Goals = [],
        Out = exactly-V
    ; da2_pair_of(Ps, E, Out),
      Vals = Vals0,
      Goals = []
    ).

da2_pair_of([R-Pair|Ps], E, Out) :-
    ( R == E -> Out = Pair ; da2_pair_of(Ps, E, Out) ).

/* Witness values of the chain outputs: the chain runs, node by node, on a
   copy whose antecedent variables hold their witness values. */
da2_chain_pairs(Goals, Pairs0, S, Pairs) :-
    copy_term(Goals-Pairs0, GoalsC-PairsC),
    v1_bind_witness(PairsC),
    ( da2_call_all(GoalsC) ->
        da2_out_pairs(Goals, GoalsC, Outs),
        append(Pairs0, Outs, Pairs)
    ; reject(unsupported, formula_shape(witness, S))
    ).

da2_call_all([]).
da2_call_all([G|Gs]) :-
    da2_arith(G),
    da2_call_all(Gs).

da2_out_pairs([], [], []).
da2_out_pairs([G|Gs], [C|Cs], [SV-SVal, NV-NVal|Rest]) :-
    arg(5, G, SV),
    arg(6, G, NV),
    arg(5, C, SVal),
    arg(6, C, NVal),
    da2_out_pairs(Gs, Cs, Rest).

% The schema clauses, loaded once into a private module.
da2_arith(Goal) :-
    ( nb_current(da2_arith_loaded, true) ->
        true
    ; nt1_schema_clauses(Clauses),
      forall(member(C, Clauses), assertz(ace_to_pl_arith:C)),
      nb_setval(da2_arith_loaded, true)
    ),
    call(ace_to_pl_arith:Goal).

da2_rewrite_target([], _, _, []).
da2_rewrite_target([Item|Items], Y, OS-ON, [Item1|Items1]) :-
    ( nonvar(Item),
      functor(Item, anchored, 2),
      arg(1, Item, C),
      C == actual,
      arg(2, Item, Inner),
      nonvar(Inner),
      functor(Inner, object, 6),
      arg(1, Inner, R),
      R == Y ->
        arg(2, Inner, Noun),
        arg(3, Inner, Class),
        arg(4, Inner, Unit),
        Item1 = anchored(C, '$guideline_amount'(Y, Noun, Class, Unit, OS, ON))
    ; Item1 = Item
    ),
    da2_rewrite_target(Items, Y, OS-ON, Items1).

% An operand's witness: an exact nonzero value its condition admits.
da2_operand_witness(Op, Count, W) :-
    ( integer(Count) ->
        ( Op == geq, Count =:= 0 -> W = 1
        ; Op == leq, Count =:= 0 -> W = -1
        ; Op == greater -> W0 is Count + 1, ( W0 =:= 0 -> W is Count + 2 ; W = W0 )
        ; Op == less -> W0 is Count - 1, ( W0 =:= 0 -> W is Count - 2 ; W = W0 )
        ; W = Count
        )
    ; W = Count
    ).

/* Literal values (contract da2 D2): the numeral tokens spell the line's
   lexemes one to one, no sign token precedes a numeral, and the m-th literal
   leaf pairs with the m-th numeral token after the formula's `=` token,
   spells like it, ends its lexeme before a space, `)`, `*`, `/` or the final
   `.`, and takes the lexeme's exact value. */
da2_literal_values(Toks, Line, KF, Lits, Vals) :-
    include(number, Toks, NTs),
    maplist(da2_numeral, NTs, Texts),
    da2_lexemes(Line, Lexs),
    maplist(da2_lexeme_text, Lexs, LTexts),
    Texts == LTexts,
    \+ da2_signed(Toks),
    findall(J, ( nth0(J, Toks, T), J > KF, number(T) ), Js),
    length(Lits, NL),
    length(Js, NJ),
    NJ >= NL,
    da2_lit_values(Lits, Js, Toks, Lexs, Line, Vals).

da2_lexeme_text(Text-_, Text).

da2_lit_values([], _, _, _, _, []).
da2_lit_values([Lit|Lits], [J|Js], Toks, Lexs, Line, [V|Vs]) :-
    nth0(J, Toks, T),
    da2_numeral(T, TT),
    arg(1, Lit, LV),
    da2_numeral(LV, LT),
    LT == TT,
    length(Pre, J),
    append(Pre, _, Toks),
    include(number, Pre, NPre),
    length(NPre, G),
    nth0(G, Lexs, Text-End),
    da2_literal_end_ok(Line, End),
    da2_lexeme_value(Text, V),
    da2_lit_values(Lits, Js, Toks, Lexs, Line, Vs).

% A numeral token's text: an integer's digits (sign included) or the float as
% SWI prints it (the driver's '$decimal' text).
da2_numeral(T, Text) :-
    integer(T),
    !,
    number_codes(T, Text).
da2_numeral(T, Text) :-
    float(T),
    with_output_to(codes(Text), write_term(T, [quoted(true),
        character_escapes(true), ignore_ops(true), numbervars(true)])).

da2_signed(Toks) :-
    append(_, [T, N|_], Toks),
    ( T == (-) ; T == (+) ),
    number(N),
    !.

da2_lexemes(Line, Lexs) :-
    length(Line, Len),
    Max is Len - 1,
    findall(Text-End,
        ( between(0, Max, I),
          da2_lexeme_at(Line, Len, I, End),
          da2_slice(Line, I, End, Text) ),
        Lexs).

da2_lexeme_at(Line, Len, I, End) :-
    nth0(I, Line, C),
    da2_digit(C),
    ( I =:= 0 ->
        true
    ; P is I - 1,
      nth0(P, Line, B),
      \+ da2_wordy(B),
      B =\= 0'.
    ),
    da2_run_end(Line, Len, I, End),
    ( End =:= Len ->
        true
    ; nth0(End, Line, F),
      \+ da2_wordy(F)
    ).

da2_digit(C) :-
    C >= 0'0,
    C =< 0'9.

da2_wordy(C) :-
    ( C >= 0'a, C =< 0'z
    ; C >= 0'A, C =< 0'Z
    ; da2_digit(C)
    ; C =:= 0'_
    ; C =:= 0'-
    ),
    !.

da2_digits_end(Line, Len, I, End) :-
    ( I < Len,
      nth0(I, Line, C),
      da2_digit(C) ->
        I1 is I + 1,
        da2_digits_end(Line, Len, I1, End)
    ; End = I
    ).

da2_run_end(Line, Len, I, End) :-
    da2_digits_end(Line, Len, I, J),
    ( J + 1 < Len,
      nth0(J, Line, 0'.),
      J1 is J + 1,
      nth0(J1, Line, D),
      da2_digit(D) ->
        da2_digits_end(Line, Len, J1, End)
    ; End = J
    ).

da2_slice(Line, I, End, Text) :-
    N is End - I,
    length(Pre, I),
    append(Pre, Rest, Line),
    length(Text, N),
    append(Text, _, Rest).

da2_literal_end_ok(Line, End) :-
    length(Line, Len),
    ( End =:= Len ->
        true
    ; nth0(End, Line, C),
      ( memberchk(C, [0' , 0'), 0'*, 0'/]) ->
          true
      ; C =:= 0'.,
        End + 1 =:= Len
      )
    ).

da2_lexeme_value(Text, V) :-
    ( append(I, [0'.|F], Text) ->
        number_codes(IN, I),
        number_codes(FN, F),
        length(F, L),
        Q is 10^L,
        P is IN * Q + FN,
        da2_arith(guideline_canon(P, Q, V))
    ; number_codes(V, Text)
    ).

v1_plain_rule(Segs, CConds, ADom, S, DocId, Map, SGroups) :-
    v1_seg_items(Segs, S, DocId, 1, N1, AItems0),
    v2_annotate(S, AItems0, AItems1),
    s1_strength(S, antecedent, AItems1, AItems),
    v1_flatten_items(CConds, consequent, S, DocId, ADom, actual, none,
        N1, _, CItems0),
    v2_annotate(S, CItems0, CItems1),
    s1_strength(S, consequent, CItems1, CItems),
    v1_ante_refs(AItems, CItems, Map, Ordered, AnteRefs),
    v1_cons_locals(Ordered, AnteRefs, Map, ConsLocals),
    v1_skolem_map(ConsLocals, Ordered, ADom, S, DocId, Sko),
    nt1_compare(AItems, AItemsC),
    maplist(da2_seg_conds, Segs, SegConds),
    append(SegConds, [CConds], Raw),
    da2_formula_pass(Raw, S, AItemsC, CItems, Ordered, Map, DocId, 1,
        AItemsF, CItemsF, Chain, Pairs),
    v1_variant(AItemsF, CItemsF, Chain, Map, Sko, S, Clauses),
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
    v2_annotate(S, SharedItems0, SharedItems1),
    s1_strength(S, antecedent, SharedItems1, SharedItems),
    v1_flatten_items(Conds1, antecedent, S, DocId, [], actual, none,
        NS, NA1, Arm1Items0),
    v2_annotate(S, Arm1Items0, Arm1Items1),
    s1_strength(S, antecedent, Arm1Items1, Arm1Items),
    v1_flatten_items(Conds2, antecedent, S, DocId, [], actual, none,
        NA1, NA2, Arm2Items0),
    v2_annotate(S, Arm2Items0, Arm2Items1),
    s1_strength(S, antecedent, Arm2Items1, Arm2Items),
    append(ADom, Dom1, Deps1),
    append(ADom, Dom2, Deps2),
    v1_flatten_items(CConds, consequent, S, DocId, Deps1, actual, none,
        NA2, _, CItems10),
    v2_annotate(S, CItems10, CItems11),
    s1_strength(S, consequent, CItems11, CItems1),
    v1_flatten_items(CConds, consequent, S, DocId, Deps2, actual, none,
        NA2, _, CItems20),
    v2_annotate(S, CItems20, CItems21),
    s1_strength(S, consequent, CItems21, CItems2),
    append(Arm1Items, Arm2Items, ArmItems),
    append(SharedItems, ArmItems, AnteAll),
    v1_ante_refs(AnteAll, CItems1, Map, Ordered, AnteRefs),
    v1_cons_locals(Ordered, AnteRefs, Map, ConsLocals),
    v1_skolem_map(ConsLocals, Ordered, Deps1, S, DocId, Sko1),
    v1_skolem_map(ConsLocals, Ordered, Deps2, S, DocId, Sko2),
    append(SharedItems, Arm1Items, Ante10),
    append(SharedItems, Arm2Items, Ante20),
    nt1_compare(Ante10, Ante1),
    nt1_compare(Ante20, Ante2),
    Raw = [Shared, Arm1, Arm2, CConds],
    da2_formula_pass(Raw, S, Ante1, CItems1, Ordered, Map, DocId, 1,
        Ante1F, CItems1F, Chain1, Pairs1),
    v1_variant(Ante1F, CItems1F, Chain1, Map, Sko1, S, Clauses1),
    da2_formula_pass(Raw, S, Ante2, CItems2, Ordered, Map, DocId, 2,
        Ante2F, CItems2F, Chain2, Pairs2),
    v1_variant(Ante2F, CItems2F, Chain2, Map, Sko2, S, Clauses2),
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

v1_variant(AItems, CItems, Chain, Map, Sko, S, Clauses) :-
    v1_expand_items(AItems, Map, Sko, Goals0),
    append(Goals0, Chain, Goals),
    ( Goals0 == [] ->
        reject(unsupported, sentence_shape(rule_without_antecedent(S)))
    ; true
    ),
    v1_expand_items(CItems, Map, Sko, Heads),
    ( Heads == [] ->
        reject(unsupported, sentence_shape(rule_without_consequent(S)))
    ; true
    ),
    v1_rule_clauses(Heads, Goals, Checked),
    v1_naf_safety(Checked),
    maplist(v1_check_safety, Checked),
    bf_order(Goals, Ordered),
    v1_rule_clauses(Heads, Ordered, Clauses).

/* Binding-first body order (contract bf; spec emit.rs bind_first). The checks
   above read the DRS order; the clauses run binding-first: each step takes the
   leftmost ready goal, else the leftmost goal. A positive goal is ready once
   the variables of the arguments it waits on are bound (bf_waited/2); a NAF
   item once its variables that positive goals share are bound. A positive
   goal binds its variables; an open call binds no head variable. */
bf_order(Goals, Ordered) :-
    bf_positive_vars(Goals, [], Shared),
    bf_order(Goals, [], Shared, Ordered).

bf_order([], _, _, []) :- !.
bf_order(Goals, Bound, Shared, [Goal|Ordered]) :-
    ( bf_take_ready(Goals, Bound, Shared, Goal, Rest) ->
        true
    ; Goals = [Goal|Rest]
    ),
    bf_bind(Goal, Bound, Bound1),
    bf_order(Rest, Bound1, Shared, Ordered).

bf_take_ready([Goal|Goals], Bound, Shared, Goal, Goals) :-
    bf_ready(Goal, Bound, Shared),
    !.
bf_take_ready([Goal|Goals], Bound, Shared, Taken, [Goal|Rest]) :-
    bf_take_ready(Goals, Bound, Shared, Taken, Rest).

bf_ready(naf(_, Sub), Bound, Shared) :-
    !,
    term_variables(Sub, Vars),
    forall(( member(V, Vars), v1_var_member(V, Shared) ),
        v1_var_member(V, Bound)).
bf_ready(Goal, Bound, _) :-
    bf_waited(Goal, Args),
    term_variables(Args, Vars),
    forall(member(V, Vars), v1_var_member(V, Bound)).

/* A comparison waits on its count inputs (/4: comparison + count; /6: + the
   unit), an arithmetic built-in on its two operand pairs, a semantic predicate
   other than the entity and event generators on its referent (argument 2). */
bf_waited(guideline_satisfies(S, N, _, _), [S, N]) :- !.
bf_waited(guideline_satisfies(S, N, U, _, _, _), [S, N, U]) :- !.
bf_waited(guideline_product(S1, N1, S2, N2, _, _), [S1, N1, S2, N2]) :- !.
bf_waited(guideline_quotient(S1, N1, S2, N2, _, _), [S1, N1, S2, N2]) :- !.
bf_waited(Goal, [Ref]) :-
    functor(Goal, Name, Arity),
    memberchk(Name/Arity, [guideline_cardinality/5, guideline_arg/4,
        guideline_pp/4, guideline_property/4, guideline_operator/3,
        guideline_interval/6, guideline_recurrence/4, guideline_frequency/5,
        guideline_order/4, guideline_recurrence_window/7, guideline_range/3,
        guideline_grade/5, guideline_preference/4, guideline_qualifier/4,
        guideline_unit/3]),
    !,
    arg(2, Goal, Ref).
bf_waited(_, []).

bf_bind(naf(_, _), Bound, Bound) :- !.
bf_bind(Goal, Bound0, Bound) :-
    term_variables(Goal, Vars),
    append(Bound0, Vars, Bound).

bf_positive_vars([], Vars, Vars).
bf_positive_vars([naf(_, _)|Goals], Vars0, Vars) :-
    !,
    bf_positive_vars(Goals, Vars0, Vars).
bf_positive_vars([Goal|Goals], Vars0, Vars) :-
    term_variables(Goal, GoalVars),
    append(Vars0, GoalVars, Vars1),
    bf_positive_vars(Goals, Vars1, Vars).

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
v1_condition(Context, Inner, Map, Sko, [guideline_range(Context, L, H)]) :-
    nonvar(Inner),
    functor(Inner, '$guideline_range', 2),
    !,
    arg(1, Inner, L0),
    arg(2, Inner, H0),
    v1_ref(L0, Map, Sko, L),
    v1_ref(H0, Map, Sko, H).
v1_condition(Context, Inner, Map, Sko,
        [guideline_grade(Context, Rec, Dim, Value, Label)]) :-
    nonvar(Inner),
    functor(Inner, '$guideline_grade', 4),
    !,
    arg(1, Inner, Rec0),
    arg(2, Inner, Dim),
    arg(3, Inner, Value),
    arg(4, Inner, Label0),
    v1_ref(Rec0, Map, Sko, Rec),
    v1_ref(Label0, Map, Sko, Label).
v1_condition(Context, Inner, Map, Sko,
        [guideline_preference(Context, E, P, A)]) :-
    nonvar(Inner),
    functor(Inner, '$guideline_preference', 3),
    !,
    arg(1, Inner, E0),
    arg(2, Inner, P0),
    arg(3, Inner, A0),
    v1_ref(E0, Map, Sko, E),
    v1_ref(P0, Map, Sko, P),
    ( A0 == none -> A = none ; v1_ref(A0, Map, Sko, A) ).
v1_condition(Context, Inner, Map, Sko,
        [guideline_qualifier(Context, E, Class, Word)]) :-
    nonvar(Inner),
    functor(Inner, '$guideline_qualifier', 3),
    !,
    arg(1, Inner, E0),
    arg(2, Inner, Class),
    arg(3, Inner, Word),
    v1_ref(E0, Map, Sko, E).
v1_condition(Context, Inner, Map, Sko, [guideline_unit(Context, Q, Unit)]) :-
    nonvar(Inner),
    functor(Inner, '$guideline_unit', 2),
    !,
    arg(1, Inner, Q0),
    arg(2, Inner, Unit),
    v1_ref(Q0, Map, Sko, Q).
v1_condition(Context, Inner, Map, Sko,
        [guideline_entity(Context, Ref, Noun, Class),
         guideline_cardinality(Context, Ref, UnitField, S, N),
         guideline_satisfies(S, N, Op, Count)]) :-
    nonvar(Inner),
    functor(Inner, '$guideline_bound', 8),
    !,
    Inner = '$guideline_bound'(Ref0, Noun, Class, UnitField, S, N, Op, Count),
    v1_ref(Ref0, Map, Sko, Ref).
v1_condition(Context, Inner, Map, Sko,
        [guideline_cardinality(Context, Ref, UnitField, S, N)]) :-
    nonvar(Inner),
    functor(Inner, '$guideline_measure', 4),
    !,
    Inner = '$guideline_measure'(Ref0, UnitField, S, N),
    v1_ref(Ref0, Map, Sko, Ref).
v1_condition(_, Inner, _, _, [guideline_satisfies(S, N, V, Op, Count, Unit)]) :-
    nonvar(Inner),
    functor(Inner, '$guideline_scaled', 6),
    !,
    Inner = '$guideline_scaled'(S, N, V, Op, Count, Unit).
v1_condition(Context, Inner, Map, Sko,
        [guideline_entity(Context, Ref, Noun, Class),
         guideline_cardinality(Context, Ref, UnitField, S, N)]) :-
    nonvar(Inner),
    functor(Inner, '$guideline_amount', 6),
    !,
    Inner = '$guideline_amount'(Ref0, Noun, Class, UnitField, S, N),
    v1_ref(Ref0, Map, Sko, Ref).
v1_condition(_, Inner, _, _, _) :-
    nonvar(Inner),
    functor(Inner, formula, 3),
    !,
    b_getval(da2_sentence, S),
    reject(unsupported, formula_shape(position, S)).
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
      memberchk(Op, [eq, about, geq, greater, leq, less, exactly, na]) ->
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
v1_cond_refs(Inner, [R]) :-
    nonvar(Inner),
    ( functor(Inner, '$guideline_bound', 8)
    ; functor(Inner, '$guideline_measure', 4)
    ; functor(Inner, '$guideline_amount', 6)
    ),
    !,
    arg(1, Inner, R).
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
v1_witness_pairs(Items, Ordered, Map, DocId, S, K, Ops, Pairs) :-
    v1_witness_items(Items, Ordered, Map, DocId, S, K, Ops, [], Rev),
    reverse(Rev, Pairs).

v1_witness_items([], _, _, _, _, _, _, Acc, Acc).
v1_witness_items([operator(B, _, Inner, _)|Items], Ordered, Map, DocId, S,
        K, Ops, Acc0, Acc) :-
    !,
    ( var(Inner),
      \+ v1_pair_member(Inner, Acc0) ->
        Acc1 = [Inner-'$guideline_id'(witness, DocId, S, box(B),
            variant(K))|Acc0]
    ; Acc1 = Acc0
    ),
    v1_witness_items(Items, Ordered, Map, DocId, S, K, Ops, Acc1, Acc).
v1_witness_items([naf(_, _)|Items], Ordered, Map, DocId, S, K, Ops, Acc0,
        Acc) :-
    !,
    v1_witness_items(Items, Ordered, Map, DocId, S, K, Ops, Acc0, Acc).
v1_witness_items([anchored(_, Inner)|Items], Ordered, Map, DocId, S, K,
        Ops, Acc0, Acc) :-
    v1_cond_refs(Inner, Refs),
    v1_witness_refs(Refs, Ordered, Map, DocId, S, K, Acc0, Acc1),
    nt1_witness(Inner, Ops, Acc1, Acc2),
    v1_witness_items(Items, Ordered, Map, DocId, S, K, Ops, Acc2, Acc).

/* nt1 D4: a converted condition's witness states its own bound, so its
   comparison holds reflexively; da2 D4: a formula operand's witness is an
   exact nonzero value its condition admits, a `some` operand's exactly 1. */
nt1_witness(Inner, Ops, Acc0, Acc) :-
    ( nonvar(Inner),
      functor(Inner, '$guideline_bound', 8),
      arg(1, Inner, R),
      strict_member(R, Ops) ->
        arg(5, Inner, CV),
        arg(6, Inner, NV),
        arg(7, Inner, Op),
        arg(8, Inner, Count),
        da2_operand_witness(Op, Count, W),
        Acc = [NV-W, CV-exactly|Acc0]
    ; nonvar(Inner),
      functor(Inner, '$guideline_amount', 6) ->
        arg(5, Inner, CV),
        arg(6, Inner, NV),
        Acc = [NV-1, CV-exactly|Acc0]
    ; nonvar(Inner),
      functor(Inner, '$guideline_bound', 8) ->
        arg(5, Inner, CV),
        arg(6, Inner, NV),
        arg(7, Inner, Op),
        arg(8, Inner, Count),
        Acc = [NV-Count, CV-Op|Acc0]
    ; nonvar(Inner),
      functor(Inner, '$guideline_scaled', 6) ->
        Inner = '$guideline_scaled'(CV, NV, UV, Op, Count, Unit),
        Acc = [UV-Unit, NV-Count, CV-Op|Acc0]
    ; Acc = Acc0
    ).

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
    vocabulary_record(Header0, Header),
    v1_indicators(Indicators),
    with_output_to(string(Out),
        ( format('% ~w.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.~n',
              [DocId]),
          v1_render_decls(Indicators),
          render_term_line(guideline_schema_version(1)),
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
    v2_annotate(1, Items0, Items1),
    nt1_compare(Items1, Items),
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
    query_header_term(QId, AceDigest, UlexDigest, Record0),
    vocabulary_record(Record0, Record),
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
    v1_indicators(Indicators),
    v1_declare_proof_world(Indicators),
    nt1_assert_schema,
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
v1_positive_goals([Goal|Goals], Facts) :-
    nt1_builtin_goal(Goal),
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
fallback_error_line(vocabulary_load, "ace_to_pl_error(vocabulary_load,unserializable).\n") :- !.
fallback_error_line(_, "ace_to_pl_error(uncaught,unserializable).\n").

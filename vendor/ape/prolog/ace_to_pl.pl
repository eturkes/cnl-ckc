% ace_to_pl.pl — ACE → plain-Prolog compiler over the APE parser.
% First added to this APE fork 2026-08-06 (not part of upstream APE).
% Copyright 2026 Emir Turkes. Derivative of APE; licensed under
% LGPL-3.0-or-later like the surrounding tree (see ../LICENSE.txt).
%
% Modes (argv after --):
%   <ape-tree-dir> <docid>            compile; ACE bytes on stdin -> Prolog on stdout
%   <ape-tree-dir> <docid> <ulex>     compile with user lexicon file
%   check <file.pl>                   load compiled file into user (quarantined I/O; any load
%                                     diagnostic rejects); every guideline_query goal must succeed
%
% Compile contract: stdin = strict RFC 3629 UTF-8; one ACE sentence per non-empty LF line.
% <docid> = nonempty [a-z0-9-] with no leading dash.
% Success: stdout = compiled Prolog document, stderr = 0 bytes, exit 0.
% Reject: stdout = 0 bytes; stderr = one canonical ace_to_pl_error(Class,Detail) line.
% Exit: 0=compiled; 1=input_utf8|ape_messages|empty_drs|sentence_lines|unsupported|safety|query_failed;
%       2=usage|ape_load|ulex_load|check_load|uncaught.
% Translation is total: every sentence becomes exactly one emitted clause
% (fact | rule | guideline_query(yesno|who(Var), Goal)); any unrecognized
% shape rejects the whole document. Supported DRS shapes:
%   copula fact   object(R,N,countable,na,eq,1) + predicate(E,be,named(P),R)  ->  'N'('P').
%   ground fact   predicate(E,V,Named...)                                     ->  'V'(Names...).
%   rule          =>(drs Ante, drs Cons): Ante = objects/predicates/~(single
%                 predicate); Cons = one predicate                            ->  Head :- Body.
%   question      question(drs): one predicate, optional query(W,who)         ->  guideline_query/2.
% Event referents are erased; each must be box-local with exactly one
% occurrence. Head and NAF variables must be bound by positive body literals.
% NAF-only predicates (no fact or rule head in the document) get a
% ":- dynamic." declaration after the header term, so \+ succeeds by
% absence at load time; undefined positive references stay fail-loud.

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

dispatch([check, File], Input, Output, ErrorStream) :-
    !,
    check_mode(File, Input, Output, ErrorStream).
dispatch([Tree, DocId], Input, Output, ErrorStream) :-
    !,
    validated_docid(DocId, ErrorStream),
    compile_mode(Tree, DocId, none, Input, Output, ErrorStream).
dispatch([Tree, DocId, Ulex], Input, Output, ErrorStream) :-
    !,
    validated_docid(DocId, ErrorStream),
    compile_mode(Tree, DocId, file(Ulex), Input, Output, ErrorStream).
dispatch(Argv, _, _, ErrorStream) :-
    emit_error(ErrorStream, usage, argv(Argv), 2).

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

/* ---------- check mode: consult one compiled document, prove its queries ---------- */

check_mode(File, Input, Output, ErrorStream) :-
    catch(
        ( quarantined_call(Input, Output, ErrorStream,
              check_load_captured(File)) ->
            true
        ; throw(error(check_load_failed(File),
              context(ace_to_pl:check_mode/4, plain_failure)))
        ),
        Error,
        emit_error(ErrorStream, check_load, Error, 2)),
    ( current_predicate(user:guideline_query/2) ->
        findall(query(Kind, Goal), user:guideline_query(Kind, Goal), Queries),
        prove_queries(Queries, Input, Output, ErrorStream)
    ; true
    ),
    halt(0).

/* Load into user with messages captured; any warning or error message
   (syntax errors, failed directives) rejects the whole file. */
check_load_captured(File) :-
    setup_call_cleanup(
        ( nb_setval(ace_to_pl_load_capture, true),
          nb_setval(ace_to_pl_load_failed, false)
        ),
        load_files(user:File, [silent(true)]),
        nb_setval(ace_to_pl_load_capture, false)),
    nb_getval(ace_to_pl_load_failed, Failed),
    ( Failed == false ->
        true
    ; throw(error(check_load_diagnostics(File),
          context(ace_to_pl:check_load_captured/1, File)))
    ).

prove_queries([], _, _, _).
prove_queries([query(Kind, Goal)|Queries], Input, Output, ErrorStream) :-
    catch(
        ( quarantined_call(Input, Output, ErrorStream, user:Goal) ->
            Outcome = proven
        ; Outcome = failed
        ),
        Error,
        Outcome = raised(Error)),
    ( Outcome == proven ->
        prove_queries(Queries, Input, Output, ErrorStream)
    ; Outcome = raised(RaisedError) ->
        emit_error(ErrorStream, query_failed, raised(Kind, RaisedError), 1)
    ; emit_error(ErrorStream, query_failed, Kind, 1)
    ).

/* ---------- compile mode ---------- */

compile_mode(Tree, DocId, Ulex, Input, Output, ErrorStream) :-
    set_stream(Input, type(binary)),
    prompt(_, ''),
    load_ape(Tree, Input, Output, ErrorStream),
    maybe_load_ulex(Ulex, Input, Output, ErrorStream, UlexDigest),
    read_input(Input, ErrorStream, Bytes, Text),
    ( quarantined_call(Input, Output, ErrorStream,
          ace_to_drs:acetext_to_drs(Text, off, off, Sentences, _SyntaxTrees,
              Drs, Messages, _Time)) ->
        accept_or_reject(DocId, Bytes, Text, UlexDigest, Sentences, Drs,
            Messages, Output, ErrorStream)
    ; throw(error(ape_call_failed, context(ace_to_pl:compile_mode/6, Text)))
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

accept_or_reject(_, _, _, _, _, _, Messages, _, ErrorStream) :-
    Messages \== [],
    !,
    emit_error(ErrorStream, ape_messages, Messages, 1).
accept_or_reject(_, _, _, _, _, Drs, [], _, ErrorStream) :-
    Drs == drs([], []),
    !,
    emit_error(ErrorStream, empty_drs, Drs, 1).
accept_or_reject(DocId, Bytes, Text, UlexDigest, Sentences, Drs, [], Output,
        ErrorStream) :-
    catch(
        ( translate_document(DocId, Bytes, Text, UlexDigest, Sentences, Drs,
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
          context(ace_to_pl:accept_or_reject/9, plain_failure)))
    ).

reject(Class, Detail) :-
    throw(ace_to_pl_reject(Class, Detail)).

/* ---------- translation ---------- */

translate_document(DocId, Bytes, Text, UlexDigest, Sentences, Drs, OutCodes) :-
    ( nonvar(Drs),
      Drs = drs(Dom, Conds),
      is_list(Dom),
      is_list(Conds) ->
        true
    ; reject(unsupported, invalid_drs_shape)
    ),
    length(Sentences, SentenceCount),
    input_lines(Text, Lines),
    length(Lines, LineCount),
    ( LineCount =:= SentenceCount ->
        true
    ; reject(sentence_lines,
          counts(lines(LineCount), sentences(SentenceCount)))
    ),
    crypto_data_hash(Bytes, AceDigest, [algorithm(sha256), encoding(octet)]),
    tag_conditions(Conds, Tagged),
    group_by_sentence(1, SentenceCount, Tagged, Groups),
    translate_groups(Groups, Dom, Conds, [], Items),
    naf_dynamic_keys(Items, DynamicKeys),
    render_document(DocId, AceDigest, UlexDigest, Lines, DynamicKeys, Items,
        OutCodes).

input_lines(Text, Lines) :-
    atom_codes(Text, Codes),
    split_lf(Codes, RawLines),
    exclude(==([]), RawLines, Lines).

split_lf(Codes, [Line|Lines]) :-
    append(Line, [0'\n|Rest], Codes),
    !,
    split_lf(Rest, Lines).
split_lf(Codes, [Codes]).

/* Tag every root condition with its sentence id. Anchor extraction
   decomposes with arg/3 and never unifies into the DRS. */
tag_conditions([], []).
tag_conditions([Cond|Conds], [S-Tagged|Rest]) :-
    tag_condition(Cond, S, Tagged),
    tag_conditions(Conds, Rest).

tag_condition(Cond, S, anchored(Inner)) :-
    nonvar(Cond),
    functor(Cond, -, 2),
    arg(1, Cond, Inner),
    arg(2, Cond, Anchor),
    anchor_sentence(Anchor, S),
    nonvar(Inner),
    !.
tag_condition(Cond, S, rule(Ante, Cons)) :-
    nonvar(Cond),
    functor(Cond, =>, 2),
    arg(1, Cond, Ante),
    arg(2, Cond, Cons),
    !,
    inner_sentence(Cond, S).
tag_condition(Cond, S, question(QDrs)) :-
    nonvar(Cond),
    functor(Cond, question, 1),
    arg(1, Cond, QDrs),
    !,
    inner_sentence(Cond, S).
tag_condition(Cond, _, _) :-
    reject(unsupported, root_condition(Cond)).

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

sub_anchor(Term, S) :-
    sub_term(Sub, Term),
    nonvar(Sub),
    functor(Sub, -, 2),
    arg(2, Sub, Anchor),
    nonvar(Anchor),
    anchor_sentence(Anchor, S).

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

take_sentence([], _, [], []).
take_sentence([Sx-Cond|Tagged], S, [Cond|Group], Rest) :-
    Sx =:= S,
    !,
    take_sentence(Tagged, S, Group, Rest).
take_sentence([Sx-Cond|Tagged], S, Group, [Sx-Cond|Rest]) :-
    take_sentence(Tagged, S, Group, Rest).

/* Each sentence yields exactly one emitted item. RootConds is the whole
   root condition list: the scope for root event single-occurrence checks. */
translate_groups([], _, _, _, []).
translate_groups([S-Group|Groups], Dom, RootConds, Bindings0,
        [item(S, Item)|Items]) :-
    translate_sentence(Group, Dom, RootConds, Bindings0, Bindings, Item),
    translate_groups(Groups, Dom, RootConds, Bindings, Items).

translate_sentence(Group, Dom, RootConds, Bindings0, Bindings, fact(Fact)) :-
    copula_fact(Group, Dom, RootConds, Bindings0, Bindings, Fact),
    !.
translate_sentence(Group, Dom, RootConds, Bindings, Bindings, fact(Fact)) :-
    ground_fact(Group, Dom, RootConds, Bindings, Fact),
    !.
translate_sentence([rule(Ante, Cons)], _, _, Bindings, Bindings, Item) :-
    !,
    translate_rule(Ante, Cons, Item).
translate_sentence([question(QDrs)], _, _, Bindings, Bindings, Item) :-
    !,
    translate_question(QDrs, Bindings, Item).
translate_sentence(Group, _, _, _, _, _) :-
    reject(unsupported, sentence_shape(Group)).

/* copula fact: object + be-predicate introducing a named individual */
copula_fact([anchored(Obj), anchored(Pred)], Dom, RootConds, Bindings0,
        Bindings, Fact) :-
    object_shape(Obj, Ref, Noun),
    nonvar(Pred),
    functor(Pred, predicate, 4),
    arg(1, Pred, Event),
    arg(2, Pred, Be),
    Be == be,
    arg(3, Pred, NamedArg),
    named_atom(NamedArg, Name),
    arg(4, Pred, Ref2),
    Ref2 == Ref,
    erasable_event(Event, Dom, RootConds),
    unbound_referent(Ref, Bindings0),
    Bindings = [binding(Ref, Name)|Bindings0],
    Fact =.. [Noun, Name].

/* ground fact: root predicate whose arguments all resolve to names */
ground_fact([anchored(Pred)], Dom, RootConds, Bindings, Fact) :-
    predicate_shape(Pred, Event, Verb, Args0),
    erasable_event(Event, Dom, RootConds),
    resolve_ground_args(Args0, Bindings, Args),
    Fact =.. [Verb|Args].

object_shape(Obj, Ref, Noun) :-
    nonvar(Obj),
    functor(Obj, object, 6),
    arg(1, Obj, Ref),
    var(Ref),
    arg(2, Obj, Noun),
    atom(Noun),
    arg(3, Obj, Countable),
    Countable == countable,
    arg(4, Obj, Unit),
    Unit == na,
    arg(5, Obj, Op),
    Op == eq,
    arg(6, Obj, Count),
    Count == 1.

predicate_shape(Pred, Event, Verb, Args) :-
    nonvar(Pred),
    functor(Pred, predicate, Arity),
    Arity >= 3,
    arg(1, Pred, Event),
    arg(2, Pred, Verb),
    atom(Verb),
    Verb \== be,
    pred_args(3, Arity, Pred, Args).

pred_args(Index, Arity, _, []) :-
    Index > Arity,
    !.
pred_args(Index, Arity, Pred, [Arg|Args]) :-
    arg(Index, Pred, Arg),
    Next is Index + 1,
    pred_args(Next, Arity, Pred, Args).

named_atom(Term, Name) :-
    nonvar(Term),
    functor(Term, named, 1),
    arg(1, Term, Name),
    atom(Name).

unbound_referent(Ref, Bindings) :-
    \+ ( member(binding(Existing, _), Bindings), Existing == Ref ).

resolve_ground_args([], _, []).
resolve_ground_args([Arg|Args0], Bindings, [Name|Args]) :-
    resolve_ground(Bindings, Arg, Name),
    resolve_ground_args(Args0, Bindings, Args).

resolve_ground(_, Arg, Name) :-
    named_atom(Arg, Name),
    !.
resolve_ground(Bindings, Ref, Name) :-
    var(Ref),
    member(binding(ExistingRef, ExistingName), Bindings),
    ExistingRef == Ref,
    !,
    Name = ExistingName.
resolve_ground(_, Arg, _) :-
    reject(unsupported, unresolved_argument(Arg)).

/* Event referents are erased. Requirements: unbound, member of the box
   domain, exactly one occurrence at condition positions inside the scope
   term. Domain lists are binders, not uses: cond_occurrences skips the
   domain argument of every drs/2 box while counting. */
erasable_event(Event, Dom, Scope) :-
    var(Event),
    is_list(Dom),
    ( strict_member(Event, Dom) ->
        true
    ; reject(unsupported, event_not_local)
    ),
    cond_occurrences(Event, Scope, Occurrences),
    ( Occurrences =:= 1 ->
        true
    ; reject(unsupported, event_reused)
    ).

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

cond_occurrences_args(Index, Arity, _, _, Count, Count) :-
    Index > Arity,
    !.
cond_occurrences_args(Index, Arity, Term, Var, Count0, Count) :-
    arg(Index, Term, Arg),
    cond_occurrences(Var, Arg, ArgCount),
    Count1 is Count0 + ArgCount,
    Next is Index + 1,
    cond_occurrences_args(Next, Arity, Term, Var, Count1, Count).

strict_member(X, [Y|Ys]) :-
    ( X == Y ->
        true
    ; strict_member(X, Ys)
    ).

/* ---------- rules ---------- */

translate_rule(Ante, Cons, rule(Head, Body)) :-
    drs_parts(Ante, ADom, AConds),
    drs_parts(Cons, CDom, CConds),
    body_literals(AConds, ADom, Ante, [], Body, HeadVars),
    head_literal(CConds, CDom, =>(Ante, Cons), HeadVars, Head),
    check_naf_safety(Body).

drs_parts(Drs, Dom, Conds) :-
    ( nonvar(Drs),
      functor(Drs, drs, 2),
      arg(1, Drs, Dom),
      arg(2, Drs, Conds),
      is_list(Dom),
      is_list(Conds) ->
        true
    ; reject(unsupported, invalid_drs_shape)
    ).

/* Antecedent conditions in DRS order become body literals. Variables
   accumulate from positive literals only. */
body_literals([], _, _, Vars, [], Vars).
body_literals([Cond|Conds], ADom, Scope, Vars0, [Lit|Lits], Vars) :-
    body_literal(Cond, ADom, Scope, Vars0, Vars1, Lit),
    body_literals(Conds, ADom, Scope, Vars1, Lits, Vars).

body_literal(Cond, ADom, Scope, Vars0, Vars, pos(Lit)) :-
    nonvar(Cond),
    functor(Cond, -, 2),
    arg(1, Cond, Inner),
    arg(2, Cond, Anchor),
    anchor_sentence(Anchor, _),
    !,
    positive_literal(Inner, ADom, Scope, Vars0, Vars, Lit).
body_literal(Cond, _, _, Vars, Vars, naf(Lit)) :-
    nonvar(Cond),
    functor(Cond, ~, 1),
    arg(1, Cond, NDrs),
    !,
    naf_literal(NDrs, Lit).
body_literal(Cond, _, _, _, _, _) :-
    reject(unsupported, antecedent_condition(Cond)).

positive_literal(Inner, _, _, Vars0, Vars, Lit) :-
    object_shape(Inner, Ref, Noun),
    !,
    add_var(Ref, Vars0, Vars),
    Lit =.. [Noun, Ref].
positive_literal(Inner, ADom, Scope, Vars0, Vars, Lit) :-
    predicate_shape(Inner, Event, Verb, Args0),
    !,
    erasable_event(Event, ADom, Scope),
    rule_args(Args0, Vars0, Vars, Args),
    Lit =.. [Verb|Args].
positive_literal(Inner, _, _, _, _, _) :-
    reject(unsupported, antecedent_condition(Inner)).

/* NAF box: exactly one anchored predicate, event local to the box. */
naf_literal(NDrs, Lit) :-
    drs_parts(NDrs, NDom, NConds),
    ( NConds = [NCond],
      nonvar(NCond),
      functor(NCond, -, 2),
      arg(1, NCond, Inner),
      arg(2, NCond, Anchor),
      anchor_sentence(Anchor, _),
      predicate_shape(Inner, Event, Verb, Args0) ->
        true
    ; reject(unsupported, naf_shape(NConds))
    ),
    erasable_event(Event, NDom, NDrs),
    naf_args(Args0, Args),
    Lit =.. [Verb|Args].

naf_args([], []).
naf_args([Arg|Args0], [Out|Args]) :-
    ( named_atom(Arg, Name) ->
        Out = Name
    ; var(Arg) ->
        Out = Arg
    ; reject(unsupported, naf_argument(Arg))
    ),
    naf_args(Args0, Args).

rule_args([], Vars, Vars, []).
rule_args([Arg|Args0], Vars0, Vars, [Out|Args]) :-
    ( named_atom(Arg, Name) ->
        Out = Name,
        Vars1 = Vars0
    ; var(Arg) ->
        Out = Arg,
        add_var(Arg, Vars0, Vars1)
    ; reject(unsupported, rule_argument(Arg))
    ),
    rule_args(Args0, Vars1, Vars, Args).

add_var(Ref, Vars0, Vars) :-
    ( strict_member(Ref, Vars0) ->
        Vars = Vars0
    ; Vars = [Ref|Vars0]
    ).

/* The consequent contributes exactly one positive head literal. */
head_literal(CConds, CDom, Scope, AllowedVars, Head) :-
    ( CConds = [CCond],
      nonvar(CCond),
      functor(CCond, -, 2),
      arg(1, CCond, Inner),
      arg(2, CCond, Anchor),
      anchor_sentence(Anchor, _),
      predicate_shape(Inner, Event, Verb, Args0) ->
        true
    ; reject(unsupported, consequent_shape(CConds))
    ),
    erasable_event(Event, CDom, Scope),
    head_args(Args0, AllowedVars, Args),
    Head =.. [Verb|Args].

head_args([], _, []).
head_args([Arg|Args0], AllowedVars, [Out|Args]) :-
    ( named_atom(Arg, Name) ->
        Out = Name
    ; var(Arg) ->
        ( strict_member(Arg, AllowedVars) ->
            Out = Arg
        ; reject(safety, head_variable_not_bound_in_body)
        )
    ; reject(unsupported, head_argument(Arg))
    ),
    head_args(Args0, AllowedVars, Args).

/* Every NAF literal variable must be bound by an earlier positive literal. */
check_naf_safety(Body) :-
    check_naf_safety(Body, []).

check_naf_safety([], _).
check_naf_safety([pos(Lit)|Lits], Bound0) :-
    term_variables(Lit, Vars),
    merge_vars(Vars, Bound0, Bound),
    check_naf_safety(Lits, Bound).
check_naf_safety([naf(Lit)|Lits], Bound) :-
    term_variables(Lit, Vars),
    ( all_bound(Vars, Bound) ->
        true
    ; reject(safety, naf_variable_not_bound)
    ),
    check_naf_safety(Lits, Bound).

merge_vars([], Vars, Vars).
merge_vars([V|Vs], Vars0, Vars) :-
    add_var(V, Vars0, Vars1),
    merge_vars(Vs, Vars1, Vars).

all_bound([], _).
all_bound([V|Vs], Bound) :-
    strict_member(V, Bound),
    all_bound(Vs, Bound).

/* ---------- questions ---------- */

translate_question(QDrs, Bindings, query(Kind, Goal)) :-
    drs_parts(QDrs, QDom, QConds),
    partition_query_conds(QConds, Markers, Preds),
    ( Preds = [pred(Event, Verb, Args0)] ->
        true
    ; reject(unsupported, question_shape(QConds))
    ),
    erasable_event(Event, QDom, QDrs),
    ( Markers == [] ->
        Kind = yesno,
        resolve_ground_args(Args0, Bindings, Args),
        Goal =.. [Verb|Args]
    ; Markers = [WhRef] ->
        Kind = who(WhRef),
        wh_args(Args0, Bindings, WhRef, Args),
        Goal =.. [Verb|Args],
        ( strict_member(WhRef, Args) ->
            true
        ; reject(unsupported, wh_variable_unused)
        )
    ; reject(unsupported, multiple_wh_markers)
    ).

partition_query_conds([], [], []).
partition_query_conds([Cond|Conds], Markers, Preds) :-
    ( nonvar(Cond),
      functor(Cond, -, 2),
      arg(1, Cond, Inner),
      arg(2, Cond, Anchor),
      anchor_sentence(Anchor, _),
      nonvar(Inner) ->
        true
    ; reject(unsupported, question_condition(Cond))
    ),
    ( functor(Inner, query, 2),
      arg(1, Inner, Ref),
      var(Ref),
      arg(2, Inner, Who),
      Who == who ->
        Markers = [Ref|MarkersRest],
        partition_query_conds(Conds, MarkersRest, Preds)
    ; predicate_shape(Inner, Event, Verb, Args) ->
        Preds = [pred(Event, Verb, Args)|PredsRest],
        partition_query_conds(Conds, Markers, PredsRest)
    ; reject(unsupported, question_condition(Inner))
    ).

wh_args([], _, _, []).
wh_args([Arg|Args0], Bindings, WhRef, [Out|Args]) :-
    ( var(Arg),
      Arg == WhRef ->
        Out = Arg
    ; resolve_ground(Bindings, Arg, Out)
    ),
    wh_args(Args0, Bindings, WhRef, Args).

/* ---------- rendering ---------- */

/* NAF literals over predicates with no clause among the document's facts
   and rule heads get a dynamic declaration in the output, so \+ succeeds
   by absence at load time while undefined positive references stay
   fail-loud under unknown=error. */
naf_dynamic_keys(Items, Keys) :-
    findall(Key, naf_key(Items, Key), NafKeys),
    findall(Key, defined_key(Items, Key), DefinedKeys),
    subtract_keys(NafKeys, DefinedKeys, Missing),
    sort(Missing, Keys).

naf_key(Items, Name/Arity) :-
    member(item(_, rule(_, Body)), Items),
    member(naf(Lit), Body),
    functor(Lit, Name, Arity).

defined_key(Items, Name/Arity) :-
    member(item(_, Item), Items),
    ( Item = fact(Head) ->
        true
    ; Item = rule(Head, _)
    ),
    functor(Head, Name, Arity).

subtract_keys([], _, []).
subtract_keys([Key|Keys0], Removed, Keys) :-
    ( memberchk(Key, Removed) ->
        Keys = Keys1
    ; Keys = [Key|Keys1]
    ),
    subtract_keys(Keys0, Removed, Keys1).

render_document(DocId, AceDigest, UlexDigest, Lines, DynamicKeys, Items,
        OutCodes) :-
    header_term(DocId, AceDigest, UlexDigest, Header),
    with_output_to(string(Out),
        ( format('% ~w.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.~n',
              [DocId]),
          render_term_line(Header),
          render_dynamic_decls(DynamicKeys),
          render_items(Items, Lines)
        )),
    string_codes(Out, OutCodes).

render_dynamic_decls([]).
render_dynamic_decls([Name/Arity|Keys]) :-
    format(':- dynamic(~q/~d).~n', [Name, Arity]),
    render_dynamic_decls(Keys).

header_term(DocId, AceDigest, none,
    guideline_document(DocId, ace_sha256(AceDigest), ulex(none))).
header_term(DocId, AceDigest, sha256(Digest),
    guideline_document(DocId, ace_sha256(AceDigest), ulex(sha256(Digest)))).

render_items([], _).
render_items([item(S, Item)|Items], Lines) :-
    nth1(S, Lines, LineCodes),
    format('% S~w: ~s~n', [S, LineCodes]),
    render_item(Item),
    render_items(Items, Lines).

render_item(fact(Fact)) :-
    render_term_line(Fact).
render_item(rule(Head, Body)) :-
    validate_emittable(rule(Head, Body)),
    copy_term(rule(Head, Body), rule(HeadCopy, BodyCopy)),
    numbervars(rule(HeadCopy, BodyCopy), 0, _),
    write_canonical_part(HeadCopy),
    write(' :- '),
    write_body(BodyCopy),
    write('.'),
    nl.
render_item(query(Kind, Goal)) :-
    render_term_line(guideline_query(Kind, Goal)).

write_body([Lit]) :-
    !,
    write_body_literal(Lit).
write_body([Lit|Lits]) :-
    write_body_literal(Lit),
    write(', '),
    write_body(Lits).

write_body_literal(pos(Lit)) :-
    write_canonical_part(Lit).
write_body_literal(naf(Lit)) :-
    write('\\+ '),
    write_canonical_part(Lit).

/* Validate the original, then number and write a copy: rendering never
   instantiates variables still shared with the DRS or later items. */
render_term_line(Term) :-
    validate_emittable(Term),
    copy_term(Term, Copy),
    numbervars(Copy, 0, _),
    write_canonical_part(Copy),
    write('.'),
    nl.

write_canonical_part(Term) :-
    write_term(Term,
        [ quoted(true),
          numbervars(true),
          character_escapes(true),
          ignore_ops(true)
        ]).

/* Emittable terms: acyclic, no attvars, atom/integer/float atomics,
   no pre-existing '$VAR'/1. */
validate_emittable(Term) :-
    ( acyclic_term(Term),
      term_attvars(Term, []),
      canonical_tree(Term) ->
        true
    ; reject(unsupported, unserializable_term)
    ).

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

/* ---------- canonical error emission ---------- */

emit_error(ErrorStream, Class, Detail, Status) :-
    ( catch(canonical_error_line(ace_to_pl_error(Class, Detail), Line), _,
          fail) ->
        true
    ; fallback_error_line(Class, Line)
    ),
    format(ErrorStream, '~s', [Line]),
    halt(Status).

canonical_error_line(Term, Line) :-
    acyclic_term(Term),
    term_attvars(Term, []),
    canonical_tree(Term),
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
fallback_error_line(query_failed, "ace_to_pl_error(query_failed,unserializable).\n") :- !.
fallback_error_line(usage, "ace_to_pl_error(usage,unserializable).\n") :- !.
fallback_error_line(ape_load, "ace_to_pl_error(ape_load,unserializable).\n") :- !.
fallback_error_line(ulex_load, "ace_to_pl_error(ulex_load,unserializable).\n") :- !.
fallback_error_line(check_load, "ace_to_pl_error(check_load,unserializable).\n") :- !.
fallback_error_line(_, "ace_to_pl_error(uncaught,unserializable).\n").

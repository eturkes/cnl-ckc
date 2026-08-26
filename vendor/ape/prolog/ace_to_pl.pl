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
%   check <file.pl>                   load compiled file into user (quarantined I/O; any load
%                                     diagnostic rejects)
%   aggregate-check <manifest>        load a composition (LF lines "<pl>\t<payload>",
%                                     0-byte file = empty) and replay every payload
%                                     obligation against the loaded whole
%   recursion-check <manifest>        load the same composition (payload column
%                                     ignored) and prove no clause head unifies with
%                                     its own leftmost body goal — the load-order
%                                     divergence shape; rejects
%                                     proof,left_recursive(Site,Name,Arity), where
%                                     Site = sentence(DocId,S) | unattributed
%   answer <manifest> <query-pl>      load the same composition (payload column
%                                     readable, never parsed) and solve one
%                                     compiled query against it under fixed
%                                     bounds (depth 100, inference 100000,
%                                     outer 1000000): stdout = two-line ground
%                                     '$guideline_answers'/4 artifact binding
%                                     query_sha256 over the raw query-file
%                                     bytes; result = solutions(<sorted sol/1
%                                     list>)|indeterminate(limit) for wh,
%                                     yes|no(finite_failure)|indeterminate(limit)
%                                     for yes-no (answers([])); malformed query
%                                     files reject check_load,query_file(<why>);
%                                     nonground solution rejects
%                                     proof,nonground_solution(QId)
%   trace <manifest> <query-pl> <answers-pl>
%                                     load the same composition and re-derive the
%                                     committed answers artifact row by row: one
%                                     directed first proof per positive row via a
%                                     meta-interpreter over the loaded clauses;
%                                     stdout = two-line ground
%                                     '$guideline_traces'/5 artifact mirroring
%                                     the answers result with a proof payload
%                                     per row — proved(Nodes) with
%                                     clause(sentence(DocId,S),
%                                     clause_sha256(<line-digest>), Children)
%                                     positive nodes + naf(Goal) leaves, or
%                                     unproved(finite_failure)|unproved(limit);
%                                     bounds: MI depth 1000, per-row inference
%                                     100000, whole-run 1000000 (a whole-run
%                                     trip emits indeterminate(limit));
%                                     malformed answers files reject
%                                     check_load,answers_file(<why>)
%
% Compile contract: stdin = strict RFC 3629 UTF-8; one ACE sentence per non-empty LF line.
% <docid> = nonempty [a-z0-9-] with no leading dash.
% Success: stdout = compiled Prolog document, stderr = 0 bytes, exit 0.
% Reject: stdout = 0 bytes; stderr = one canonical ace_to_pl_error(Class,Detail) line.
% Exit: 0=compiled; 1=input_utf8|ape_messages|empty_drs|sentence_lines|unsupported|safety|proof;
%       2=usage|ape_load|ulex_load|check_load|uncaught.
% v1 assurance: every v1 compile (product or proof emission) derives per-group
% witness worlds + obligations and replays them against the document's own
% clauses; an underivable obligation rejects the document (class proof).
% v1 is the sole schema: every compile projects onto it, and a future ABI
% extension bumps the version and reintroduces an explicit selector. The
% frozen v1 path lives under "v1 schema projection" near the end of this
% file and is documented in README.md "Compiled Prolog schema (v1)". v1
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

dispatch([check, File], Input, Output, ErrorStream) :-
    !,
    check_mode(File, Input, Output, ErrorStream).
dispatch(['aggregate-check', Manifest], Input, Output, ErrorStream) :-
    !,
    aggregate_check_mode(Manifest, Input, Output, ErrorStream).
dispatch(['recursion-check', Manifest], Input, Output, ErrorStream) :-
    !,
    recursion_check_mode(Manifest, Input, Output, ErrorStream).
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
dispatch([answer, Manifest, QueryPl], Input, Output, ErrorStream) :-
    !,
    answer_mode(Manifest, QueryPl, Input, Output, ErrorStream).
dispatch([answer|Rest], _, _, ErrorStream) :-
    !,
    emit_error(ErrorStream, usage, argv([answer|Rest]), 2).
dispatch([trace, Manifest, QueryPl, AnswersPl], Input, Output, ErrorStream) :-
    !,
    trace_mode(Manifest, QueryPl, AnswersPl, Input, Output, ErrorStream).
dispatch([trace|Rest], _, _, ErrorStream) :-
    !,
    emit_error(ErrorStream, usage, argv([trace|Rest]), 2).
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

/* ---------- check mode: load one compiled document ---------- */

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
    halt(0).

/* Load into user with messages captured; any warning or error message
   (syntax errors, failed directives) rejects the whole file (or, for
   the aggregate list form, the whole set). */
check_load_captured(File) :-
    check_load_captured_list([File], File).

check_load_captured_list(Files, Label) :-
    setup_call_cleanup(
        ( nb_setval(ace_to_pl_load_capture, true),
          nb_setval(ace_to_pl_load_failed, false)
        ),
        load_files(user:Files, [silent(true)]),
        nb_setval(ace_to_pl_load_capture, false)),
    nb_getval(ace_to_pl_load_failed, Failed),
    ( Failed == false ->
        true
    ; throw(error(check_load_diagnostics(Label),
          context(ace_to_pl:check_load_captured_list/2, Label)))
    ).

/* ---------- aggregate-check mode: whole-composition proof replay ---------- */

/* P9: load every manifest pl into user under the capture hook (any
   diagnostic fails), assert the structural invariants, then replay
   every payload obligation against the loaded composition — NAF
   evaluates against the whole batch by design.

   Payloads are evidence, not authority: the obligation set is checked
   against the loaded documents (unique keys, contiguous variants,
   nonempty heads, coverage equality with the sentence identities the
   products themselves carry) before any head is proved, so an emptied,
   truncated, duplicated or misattributed payload fails rather than
   shrinking the replay. Residue: dropping the LAST variant of a
   multi-variant sentence keeps coverage complete and variants
   contiguous; rule-head Deps are variables, so per-sentence variant
   counts are not derivable from the product under the frozen ABI. */
aggregate_check_mode(Manifest, Input, Output, ErrorStream) :-
    catch(
        ( aggregate_read_manifest(Manifest, PlPaths, PayloadPaths) ->
            true
        ; throw(error(aggregate_manifest(Manifest),
              context(ace_to_pl:aggregate_check_mode/4, plain_failure)))
        ),
        Error,
        emit_error(ErrorStream, check_load, Error, 2)),
    ( PlPaths == [] ->
        aggregate_report(Output, 0, 0)
    ; aggregate_declare_indicators,
      aggregate_load(PlPaths, Input, Output, ErrorStream),
      aggregate_assertions(PlPaths, ErrorStream),
      aggregate_groups(PayloadPaths, ErrorStream, Groups),
      aggregate_group_keys(Groups, ErrorStream),
      aggregate_coverage(Groups, ErrorStream),
      aggregate_prove_groups(Groups, ErrorStream),
      length(Groups, Count),
      length(PlPaths, DocCount),
      aggregate_report(Output, DocCount, Count)
    ).

/* The witness replay asserts into user, so the v1 ABI is declared
   dynamic BEFORE loading — loaded clauses then join dynamic predicates
   (a static procedure cannot be modified afterwards). */
aggregate_declare_indicators :-
    v1_indicators(Indicators),
    aggregate_declare(Indicators).

aggregate_declare([]).
aggregate_declare([Name/Arity|Keys]) :-
    dynamic(user:Name/Arity),
    aggregate_declare(Keys).

aggregate_report(Output, DocCount, Count) :-
    format(Output, 'ace_to_pl aggregate ok ~d documents ~d obligations~n',
        [DocCount, Count]),
    halt(0).

/* Manifest grammar (strict): UTF-8, LF-terminated lines
   "<pl-path>\t<payload-path>", nonempty fields, exactly one tab, final
   LF required; a 0-byte file = the empty composition. */
aggregate_read_manifest(File, PlPaths, PayloadPaths) :-
    read_utf8_file(File, Bytes, Text),
    ( Bytes == [] ->
        PlPaths = [],
        PayloadPaths = []
    ; atom_codes(Text, Codes),
      ( append(_, [0'\n], Codes) ->
          true
      ; throw(error(aggregate_manifest(missing_final_newline),
            context(ace_to_pl:aggregate_read_manifest/3, File)))
      ),
      aggregate_manifest_lines(Codes, File, PlPaths, PayloadPaths),
      aggregate_manifest_readable(PlPaths, File),
      aggregate_manifest_readable(PayloadPaths, File)
    ).

aggregate_manifest_readable([], _).
aggregate_manifest_readable([Path|Paths], File) :-
    ( exists_file(Path),
      access_file(Path, read) ->
        true
    ; throw(error(aggregate_manifest(unreadable(Path)),
          context(ace_to_pl:aggregate_read_manifest/3, File)))
    ),
    aggregate_manifest_readable(Paths, File).

aggregate_manifest_lines([], _, [], []).
aggregate_manifest_lines(Codes, File, [Pl|Pls], [Payload|Payloads]) :-
    append(LineCodes, [0'\n|Rest], Codes),
    !,
    aggregate_manifest_entry(LineCodes, File, Pl, Payload),
    aggregate_manifest_lines(Rest, File, Pls, Payloads).

aggregate_manifest_entry(LineCodes, File, Pl, Payload) :-
    ( append(PlCodes, [0'\t|PayloadCodes], LineCodes),
      PlCodes \== [],
      PayloadCodes \== [],
      \+ memberchk(0'\t, PlCodes),
      \+ memberchk(0'\t, PayloadCodes) ->
        atom_codes(Pl, PlCodes),
        atom_codes(Payload, PayloadCodes)
    ; atom_codes(Line, LineCodes),
      throw(error(aggregate_manifest(line(Line)),
          context(ace_to_pl:aggregate_read_manifest/3, File)))
    ).

aggregate_load(PlPaths, Input, Output, ErrorStream) :-
    catch(
        ( quarantined_call(Input, Output, ErrorStream,
              check_load_captured_list(PlPaths, PlPaths)) ->
            true
        ; throw(error(aggregate_load_failed,
              context(ace_to_pl:aggregate_load/4, plain_failure)))
        ),
        Error,
        emit_error(ErrorStream, check_load, Error, 2)).

/* Structural invariants (P9): document records = manifest rows as a
   bijection, and loaded schema-version set = [1]. A sorted-distinct
   count alone let one row carry two same-id records, or two unique
   records while a sibling row carried none. Clause-level census:
   total clause count = rows, distinct docids = rows, then per row
   exactly one attributed full ground fact (clause_property source;
   an unattributed runtime assert lands in the count or starves a
   row). */
aggregate_assertions(PlPaths, ErrorStream) :-
    length(PlPaths, Expected),
    ( current_predicate(user:guideline_document/3) ->
        findall(entry(Source, Body, guideline_document(D, A, U)),
            ( clause(user:guideline_document(D, A, U), Body, Ref),
              ( clause_property(Ref, source(Source)) ->
                  true
              ; Source = none
              ) ),
            Entries)
    ; Entries = []
    ),
    length(Entries, Found),
    ( Found =:= Expected ->
        true
    ; emit_error(ErrorStream, proof,
          document_records(Expected, Found), 1)
    ),
    findall(D1, member(entry(_, _, guideline_document(D1, _, _)), Entries),
        Ids),
    sort(Ids, DistinctIds),
    length(DistinctIds, DistinctFound),
    ( DistinctFound =:= Expected ->
        true
    ; emit_error(ErrorStream, proof,
          document_records(Expected, DistinctFound), 1)
    ),
    aggregate_document_rows(PlPaths, Entries, ErrorStream),
    ( current_predicate(user:guideline_schema_version/1) ->
        findall(V, user:guideline_schema_version(V), Vs)
    ; Vs = []
    ),
    sort(Vs, VersionSet),
    ( VersionSet == [1] ->
        true
    ; emit_error(ErrorStream, proof, schema_versions(VersionSet), 1)
    ).

/* Row bijection walk: rules and nonground facts are not records. */
aggregate_document_rows([], _, _).
aggregate_document_rows([Path|Paths], Entries, ErrorStream) :-
    absolute_file_name(Path, Abs),
    findall(Body-Record, member(entry(Abs, Body, Record), Entries), Row),
    ( Row = [true-Record1] ->
        ( ground(Record1) ->
            true
        ; emit_error(ErrorStream, proof, document_row(Path, nonground), 1)
        )
    ; Row = [_-_] ->
        emit_error(ErrorStream, proof, document_row(Path, rule), 1)
    ; length(Row, N),
      emit_error(ErrorStream, proof, document_row(Path, N), 1)
    ),
    aggregate_document_rows(Paths, Entries, ErrorStream).

/* Payload obligations are evidence, never authority: a payload that
   omits, truncates, forges or repeats groups must fail rather than
   shrink the replay. Groups are therefore read and validated whole,
   their keys proved unique and their sentence coverage proved equal to
   the sentence set the LOADED composition itself carries, before any
   head is proved. Manifest rows bind product to payload for
   provenance; replay soundness rests on that composition-wide
   coverage, so a swapped pair changes nothing the replay claims. */
aggregate_groups([], _, []).
aggregate_groups([Path|Paths], ErrorStream, Groups) :-
    aggregate_payload_terms(Path, ErrorStream, Terms),
    aggregate_path_groups(Terms, Path, ErrorStream, Head),
    aggregate_groups(Paths, ErrorStream, Tail),
    append(Head, Tail, Groups).

aggregate_path_groups([], _, _, []).
aggregate_path_groups([Term|Terms], Path, ErrorStream,
        [group(DocId, S, K, Facts, Heads)|Groups]) :-
    ( Term = '$guideline_proof'(DocId0, S0, variant(K0), witness(Facts0),
          prove(Heads0)),
      is_list(Facts0),
      is_list(Heads0),
      ground(Term) ->
        DocId = DocId0, S = S0, K = K0, Facts = Facts0, Heads = Heads0
    ; emit_error(ErrorStream, check_load, payload_term(Path), 2)
    ),
    ( Heads == [] ->
        emit_error(ErrorStream, proof, empty_obligation(DocId, S,
            variant(K)), 1)
    ; true
    ),
    aggregate_path_groups(Terms, Path, ErrorStream, Groups).

/* One group per (document, sentence, variant), variants numbered 1..N
   per sentence: a dropped or repeated variant is a payload defect. */
aggregate_group_keys(Groups, ErrorStream) :-
    findall(DocId-S-K, member(group(DocId, S, K, _, _), Groups), Keys),
    msort(Keys, Sorted),
    aggregate_duplicate_key(Sorted, ErrorStream),
    sort(Keys, Unique),
    aggregate_variant_runs(Unique, ErrorStream).

aggregate_duplicate_key([Key, Key|_], ErrorStream) :-
    !,
    Key = DocId-S-K,
    emit_error(ErrorStream, proof,
        duplicate_obligation(DocId, S, variant(K)), 1).
aggregate_duplicate_key([_|Keys], ErrorStream) :-
    !,
    aggregate_duplicate_key(Keys, ErrorStream).
aggregate_duplicate_key([], _).

aggregate_variant_runs([], _).
aggregate_variant_runs([DocId-S-K|Keys], ErrorStream) :-
    aggregate_variant_run(Keys, DocId, S, [K], Ks, Rest),
    ( aggregate_sequence(Ks, 1) ->
        true
    ; emit_error(ErrorStream, proof, variant_sequence(DocId, S, Ks), 1)
    ),
    aggregate_variant_runs(Rest, ErrorStream).

aggregate_variant_run([DocId-S-K|Keys], DocId0, S0, Acc, Ks, Rest) :-
    DocId == DocId0,
    S == S0,
    !,
    append(Acc, [K], Acc1),
    aggregate_variant_run(Keys, DocId0, S0, Acc1, Ks, Rest).
aggregate_variant_run(Keys, _, _, Ks, Ks, Keys).

aggregate_sequence([], _).
aggregate_sequence([N|Ns], N) :-
    N1 is N + 1,
    aggregate_sequence(Ns, N1).

/* Coverage oracle: the composition's own clauses carry every sentence
   identity the projection emitted, so the obligation set is checked
   against the products rather than against the payload's own count. */
aggregate_coverage(Groups, ErrorStream) :-
    aggregate_loaded_sentences(Loaded),
    findall(DocId-S, member(group(DocId, S, _, _, _), Groups), Raw),
    sort(Raw, Covered),
    ( aggregate_first_absent(Loaded, Covered, MissingDoc-MissingS) ->
        emit_error(ErrorStream, proof,
            missing_obligation(MissingDoc, MissingS), 1)
    ; aggregate_first_absent(Covered, Loaded, ExtraDoc-ExtraS) ->
        emit_error(ErrorStream, proof,
            extra_obligation(ExtraDoc, ExtraS), 1)
    ; true
    ).

aggregate_first_absent([Pair|Pairs], Others, Absent) :-
    ( memberchk(Pair, Others) ->
        aggregate_first_absent(Pairs, Others, Absent)
    ; Absent = Pair
    ).

aggregate_loaded_sentences(Pairs) :-
    v1_indicators(Indicators),
    findall(DocId-S,
        ( member(Name/Arity, Indicators),
          functor(Head, Name, Arity),
          catch(clause(user:Head, Body), _, fail),
          aggregate_identity((Head :- Body), DocId, S)
        ),
        Raw),
    sort(Raw, Pairs).

aggregate_identity(Term, DocId, S) :-
    aggregate_subterm(Term, Sub),
    nonvar(Sub),
    Sub = '$guideline_id'(_, DocId, S, _, _),
    atom(DocId),
    integer(S).

aggregate_subterm(Term, Term).
aggregate_subterm(Term, Sub) :-
    compound(Term),
    arg(_, Term, Arg),
    aggregate_subterm(Arg, Sub).

aggregate_prove_groups([], _).
aggregate_prove_groups([group(DocId, S, K, Facts, Heads)|Groups],
        ErrorStream) :-
    aggregate_check_one(DocId, S, K, Facts, Heads, ErrorStream),
    aggregate_prove_groups(Groups, ErrorStream).

aggregate_payload_terms(Path, ErrorStream, Terms) :-
    catch(
        ( aggregate_read_terms(Path, Terms) ->
            true
        ; throw(error(payload_read_failed(Path),
              context(ace_to_pl:aggregate_payload_terms/3, plain_failure)))
        ),
        Error,
        emit_error(ErrorStream, check_load, Error, 2)).

aggregate_read_terms(Path, Terms) :-
    read_utf8_file(Path, Bytes, Text),
    aggregate_payload_bytes(Bytes, Path),
    setup_call_cleanup(
        open_string(Text, Stream),
        aggregate_stream_terms(Stream, Terms),
        close(Stream)).

aggregate_stream_terms(Stream, Terms) :-
    read_term(Stream, Term, []),
    ( Term == end_of_file ->
        Terms = []
    ; Terms = [Term|Rest],
      aggregate_stream_terms(Stream, Rest)
    ).

/* Payload bytes are canonical proof output: a nonempty file ends with
   the LF its last term wrote, so a truncated tail is a defect rather
   than a shorter obligation set. */
aggregate_payload_bytes([], _) :- !.
aggregate_payload_bytes(Bytes, Path) :-
    last(Bytes, Last),
    ( Last =:= 0'\n ->
        true
    ; throw(error(payload_bytes(Path),
          context(ace_to_pl:aggregate_read_terms/2, plain_failure)))
    ).

aggregate_check_one(DocId, S, K, Facts, Heads, ErrorStream) :-
    aggregate_assert_witness(Facts, Refs),
    ( aggregate_prove_heads(Heads) ->
        v1_erase_refs(Refs)
    ; v1_erase_refs(Refs),
      emit_error(ErrorStream, proof,
          obligation_failed(DocId, S, variant(K)), 1)
    ).

aggregate_assert_witness([], []).
aggregate_assert_witness([Fact|Facts], [Ref|Refs]) :-
    asserta(user:Fact, Ref),
    aggregate_assert_witness(Facts, Refs).

aggregate_prove_heads([]).
aggregate_prove_heads([Head|Heads]) :-
    v1_bounded_head_call(user:Head),
    aggregate_prove_heads(Heads).

/* ---------- recursion-check mode: composition left-recursion scan ---------- */

/* Termination belongs to the consuming engine, but the one shape that
   makes an open query's termination depend on load order is checkable
   here: naive SLD selects the leftmost body goal, so a clause head that
   unifies with it descends into itself. The scan is structural — no
   payloads, no proving, load-order independent — and reports the rule
   clauses it read, so a composition that loaded no rules is visible in
   the report instead of passing silently. Manifest grammar and loading
   are the aggregate replay's, minus the payload column. */
recursion_check_mode(Manifest, Input, Output, ErrorStream) :-
    catch(
        ( aggregate_read_manifest(Manifest, PlPaths, _) ->
            true
        ; throw(error(aggregate_manifest(Manifest),
              context(ace_to_pl:recursion_check_mode/4, plain_failure)))
        ),
        Error,
        emit_error(ErrorStream, check_load, Error, 2)),
    ( PlPaths == [] ->
        recursion_report(Output, 0, 0)
    ; aggregate_declare_indicators,
      aggregate_load(PlPaths, Input, Output, ErrorStream),
      recursion_rules(Rules),
      recursion_scan(Rules, ErrorStream),
      length(Rules, RuleCount),
      length(PlPaths, DocCount),
      recursion_report(Output, DocCount, RuleCount)
    ).

recursion_report(Output, DocCount, RuleCount) :-
    format(Output, 'ace_to_pl recursion ok ~d documents ~d rule clauses~n',
        [DocCount, RuleCount]),
    halt(0).

recursion_rules(Rules) :-
    v1_indicators(Indicators),
    findall(rule(Name, Arity, Head, Body),
        ( member(Name/Arity, Indicators),
          functor(Head, Name, Arity),
          catch(clause(user:Head, Body), _, fail),
          Body \== true
        ),
        Rules).

/* SLD renames a clause apart before resolving it, so the leftmost goal
   is copied before the unifiability test — a head and goal that unify
   only after renaming (p(X,a) :- p(b,X)) is the same descent. */
recursion_scan([], _).
recursion_scan([rule(Name, Arity, Head, Body)|Rules], ErrorStream) :-
    ( recursion_leftmost(Body, Goal),
      copy_term(Goal, Fresh),
      \+ \+ Head = Fresh ->
        recursion_site(Head, Body, Site),
        emit_error(ErrorStream, proof, left_recursive(Site, Name, Arity), 1)
    ; recursion_scan(Rules, ErrorStream)
    ).

recursion_site(Head, Body, Site) :-
    ( aggregate_identity((Head :- Body), DocId, S) ->
        Site = sentence(DocId, S)
    ; Site = unattributed
    ).

recursion_leftmost(Body, _) :-
    var(Body),
    !,
    fail.
recursion_leftmost((A, _), Goal) :-
    !,
    recursion_leftmost(A, Goal).
recursion_leftmost((A ; _), Goal) :-
    !,
    recursion_leftmost(A, Goal).
recursion_leftmost(\+ A, Goal) :-
    !,
    recursion_leftmost(A, Goal).
recursion_leftmost(user:A, Goal) :-
    !,
    recursion_leftmost(A, Goal).
recursion_leftmost(Goal, Goal).

/* ---------- answer mode: bounded query solve over a composition ---------- */

/* Pipeline order (fixed): argv -> manifest read -> query file read +
   validation -> composition load + structure parity -> solve -> emit.
   Manifest + composition reuse the aggregate machinery verbatim
   (grammar, readability, capture-hook load, document/schema parity),
   so those reject bytes stay canonical; the payload column is checked
   readable and never parsed. Query validation precedes the load so a
   malformed query never executes composition clauses. The empty
   composition still declares the v1 indicators and solves (finite
   emptiness, not a limit). */
answer_mode(Manifest, QueryPl, Input, Output, ErrorStream) :-
    catch(
        ( aggregate_read_manifest(Manifest, PlPaths, _PayloadPaths) ->
            true
        ; throw(error(aggregate_manifest(Manifest),
              context(ace_to_pl:answer_mode/5, plain_failure)))
        ),
        Error,
        emit_error(ErrorStream, check_load, Error, 2)),
    answer_read_query(QueryPl, ErrorStream, QId, Digest, Conj, Answers),
    aggregate_declare_indicators,
    ( PlPaths == [] ->
        true
    ; aggregate_load(PlPaths, Input, Output, ErrorStream),
      aggregate_assertions(PlPaths, ErrorStream)
    ),
    answer_solve(Conj, Answers, QId, ErrorStream, Result),
    answer_emit(Output, QId, Digest, Result),
    halt(0).

/* Solver bounds, fixed by definition: a depth cap per derivation, one
   inner inference budget cumulative across backtracking for the whole
   query (frozen T21 behavior — not per solution), and one outer
   inference budget over the whole enumeration. No in-process time
   limit — wall-clock containment belongs to the caller; these
   constants keep the artifact deterministic. */
answer_depth_limit(100).
answer_inference_limit(100000).
answer_outer_inference_limit(1000000).

/* Query file = data, never consulted: raw bytes read once (strict
   UTF-8) feed the artifact digest verbatim, then the terms are read
   from the decoded text. Open/UTF-8/syntax failures surface verbatim
   under check_load; structural violations reject
   check_load,query_file(<why>). The term law is exactly two terms in
   order — record then projection; comments and layout carry no
   semantics (committed-byte canon belongs to the freshness gate).
   Census ends only at physical EOF (the zero-width read); a literal
   end_of_file. term spans source bytes, so it counts as a data term
   — see answer_stream_terms and the term_count(3) probe. */
answer_read_query(File, ErrorStream, QId, Digest, Conj, Answers) :-
    catch(read_utf8_file(File, Bytes, Text),
        Error,
        emit_error(ErrorStream, check_load, Error, 2)),
    crypto_data_hash(Bytes, Digest, [algorithm(sha256), encoding(octet)]),
    catch(answer_query_terms(File, Text, Terms),
        Error2,
        emit_error(ErrorStream, check_load, Error2, 2)),
    ( Terms = [Record, Projection] ->
        true
    ; length(Terms, N),
      answer_reject(ErrorStream, term_count(N))
    ),
    answer_validate_record(Record, ErrorStream, QId),
    answer_validate_projection(Projection, ErrorStream, Conj, Answers).

answer_reject(ErrorStream, Why) :-
    emit_error(ErrorStream, check_load, query_file(Why), 2).

/* file_name pins the syntax-error context to file(Path,Line,Col,Char);
   an anonymous string stream would embed a process-local pointer and
   push the verbatim error into the unserializable fallback. */
answer_query_terms(File, Text, Terms) :-
    setup_call_cleanup(
        ( open_string(Text, Stream),
          set_stream(Stream, file_name(File))
        ),
        answer_stream_terms(Stream, Terms),
        close(Stream)).

/* Physical EOF is the zero-width read (term start = post-read
   position); a literal end_of_file. term spans source bytes, so the
   census counts it as data instead of stopping early. */
answer_stream_terms(Stream, Terms) :-
    read_term(Stream, Term, [term_position(Position)]),
    stream_property(Stream, position(After)),
    ( Term == end_of_file,
      stream_position_data(byte_count, Position, Offset),
      stream_position_data(byte_count, After, Offset) ->
        Terms = []
    ; Terms = [Term|Rest],
      answer_stream_terms(Stream, Rest)
    ).

/* Record law: ground '$guideline_query'(v1, QId, ace_sha256(H64),
   ulex(none|sha256(H64))); QId rides the docid grammar (it lands in
   the artifact comment, so the grammar keeps it injection-free). Guard
   order shape -> nonground -> version -> qid -> digests; every guard
   inspects without instantiating, and why payloads come only from
   ground-guaranteed positions. Shape spans the outer wrapper functors
   ace_sha256/1 + ulex/1; wrapper-payload defects fall through to
   record_ace_sha256/record_ulex once groundness holds. */
answer_validate_record(Record, ErrorStream, QId) :-
    ( compound(Record),
      Record = '$guideline_query'(Version, QId0, Ace, Ulex),
      compound(Ace), Ace = ace_sha256(_),
      compound(Ulex), Ulex = ulex(_) ->
        true
    ; answer_reject(ErrorStream, record_shape)
    ),
    ( ground(Record) ->
        true
    ; answer_reject(ErrorStream, record_nonground)
    ),
    ( Version == v1 ->
        true
    ; answer_reject(ErrorStream, record_version(Version))
    ),
    ( answer_valid_qid(QId0) ->
        true
    ; answer_reject(ErrorStream, record_qid(QId0))
    ),
    ( Ace = ace_sha256(AceHex),
      answer_hex64(AceHex) ->
        true
    ; answer_reject(ErrorStream, record_ace_sha256)
    ),
    ( ( Ulex == ulex(none)
      ; Ulex = ulex(sha256(UlexHex)),
        answer_hex64(UlexHex)
      ) ->
        true
    ; answer_reject(ErrorStream, record_ulex)
    ),
    QId = QId0.

answer_valid_qid(QId) :-
    atom(QId),
    atom_codes(QId, Codes),
    Codes = [First|_],
    First =\= 0'-,
    docid_codes(Codes).

answer_hex64(Hex) :-
    atom(Hex),
    atom_codes(Hex, Codes),
    length(Codes, 64),
    answer_hex_codes(Codes).

answer_hex_codes([]).
answer_hex_codes([Code|Codes]) :-
    ( Code >= 0'0, Code =< 0'9 ->
        true
    ; Code >= 0'a, Code =< 0'f
    ),
    answer_hex_codes(Codes).

/* Projection law: '$guideline_query_projection'(goal(Conj),
   answers(Rows)). Goal = any proper ','/2 tree (canonical right
   nesting is the freshness gate's law, not this mode's) whose every
   leaf is one of the seven semantic v1 indicators — the walk is
   depth-first left-to-right and the first offender wins, before any
   call. Rows = proper list of answer(Var, Desc): Var a variable
   occurring in Conj (identity, not unifiability), rows
   pairwise-distinct, Desc ground noun(Atom,Atom) | wh(who) |
   wh(what); per-row guard order shape -> nonvar -> absent ->
   duplicate -> desc, first violating index wins. answer_desc stays
   payload-free: a variable descriptor would render nondeterministic
   bytes. */
answer_validate_projection(Projection, ErrorStream, Conj, Answers) :-
    ( compound(Projection),
      Projection = '$guideline_query_projection'(Goal, AnswersWrap),
      compound(Goal),
      Goal = goal(Conj0),
      compound(AnswersWrap),
      AnswersWrap = answers(Answers0) ->
        true
    ; answer_reject(ErrorStream, projection_shape)
    ),
    answer_validate_goal(Conj0, ErrorStream),
    ( is_list(Answers0) ->
        true
    ; answer_reject(ErrorStream, answers_list)
    ),
    term_variables(Conj0, GoalVars),
    answer_validate_rows(Answers0, 1, GoalVars, [], ErrorStream),
    Conj = Conj0,
    Answers = Answers0.

answer_validate_goal(Goal, ErrorStream) :-
    ( var(Goal) ->
        answer_reject(ErrorStream, goal_variable)
    ; Goal = ','(Left, Right) ->
        answer_validate_goal(Left, ErrorStream),
        answer_validate_goal(Right, ErrorStream)
    ; functor(Goal, Name, Arity),
      answer_goal_indicator(Name/Arity) ->
        true
    ; functor(Goal, Name, Arity),
      answer_reject(ErrorStream, goal_foreign(Name, Arity))
    ).

/* The seven semantic indicators: the v1 vocabulary minus the two
   per-document header predicates (schema version, document record),
   which describe provenance rather than content. */
answer_goal_indicator(guideline_entity/4).
answer_goal_indicator(guideline_cardinality/5).
answer_goal_indicator(guideline_event/3).
answer_goal_indicator(guideline_arg/4).
answer_goal_indicator(guideline_pp/4).
answer_goal_indicator(guideline_property/4).
answer_goal_indicator(guideline_operator/3).

answer_validate_rows([], _, _, _, _).
answer_validate_rows([Row|Rows], Index, GoalVars, Seen, ErrorStream) :-
    ( compound(Row),
      Row = answer(Var, Desc) ->
        true
    ; answer_reject(ErrorStream, answer_shape(Index))
    ),
    ( var(Var) ->
        true
    ; answer_reject(ErrorStream, answer_var(Index, nonvar))
    ),
    ( answer_var_member(Var, GoalVars) ->
        true
    ; answer_reject(ErrorStream, answer_var(Index, absent))
    ),
    ( answer_var_member(Var, Seen) ->
        answer_reject(ErrorStream, answer_var(Index, duplicate))
    ; true
    ),
    ( answer_valid_desc(Desc) ->
        true
    ; answer_reject(ErrorStream, answer_desc(Index))
    ),
    NextIndex is Index + 1,
    answer_validate_rows(Rows, NextIndex, GoalVars, [Var|Seen], ErrorStream).

answer_var_member(Var, [Other|Others]) :-
    ( Var == Other ->
        true
    ; answer_var_member(Var, Others)
    ).

answer_valid_desc(Desc) :-
    ground(Desc),
    ( Desc = noun(Noun, Class) ->
        atom(Noun),
        atom(Class)
    ; Desc == wh(who) ->
        true
    ; Desc == wh(what)
    ).

/* Solve. Sentinels are status VALUES (depth_limit_exceeded /
   inference_limit_exceeded); success statuses are true or !. Yes-no
   (answers([])) is bounded-once: the first ordinary proof is
   conclusive and later branches are never searched, while a sentinel
   reached before any proof is indeterminate — never no. Wh collects
   every per-solution status under the outer budget; the canonical
   forward manifest order is the definition of the run, so no
   order-independence is claimed. Any nonground success row is an
   invariant breach that outranks the limit fold (a raised budget must
   surface it, never mask it); sentinel rows carry unbound values and
   stay outside the groundness check. Result algebra:
   solutions(Sorted) via sort/2 (standard order dedup) | yes |
   no(finite_failure) | indeterminate(limit). */
answer_solve(Conj, [], _QId, _ErrorStream, Result) :-
    !,
    answer_depth_limit(Depth),
    answer_inference_limit(Inferences),
    answer_outer_inference_limit(Outer),
    ( call_with_inference_limit(
          once(call_with_inference_limit(
              call_with_depth_limit(user:Conj, Depth, DepthResult),
              Inferences, InferenceResult)),
          Outer, OuterResult) ->
        ( OuterResult \== inference_limit_exceeded,
          InferenceResult \== inference_limit_exceeded,
          integer(DepthResult) ->
            Result = yes
        ; Result = indeterminate(limit)
        )
    ; Result = no(finite_failure)
    ).
answer_solve(Conj, Answers, QId, ErrorStream, Result) :-
    answer_vars(Answers, Vars),
    answer_depth_limit(Depth),
    answer_inference_limit(Inferences),
    answer_outer_inference_limit(Outer),
    call_with_inference_limit(
        findall(row(Vars, InferenceResult, DepthResult),
            ( call_with_inference_limit(
                  call_with_depth_limit(user:Conj, Depth, DepthResult),
                  Inferences, InferenceResult),
              answer_row_invariant(Vars, InferenceResult, DepthResult,
                  QId, ErrorStream) ),
            Rows),
        Outer, OuterResult),
    ( OuterResult == inference_limit_exceeded ->
        Result = indeterminate(limit)
    ; answer_classify_rows(Rows, QId, ErrorStream, Result)
    ).

/* Ordinary success rows are ground-checked at row birth, BEFORE they
   enter the outer-bounded bag: an outer sentinel erases findall's
   partial bag, so a classify-time member check alone masks an earlier
   nonground success as indeterminate(limit) rc0. Sentinel rows carry
   unbound values and stay exempt; the classify-time check remains as
   the law over completed bags. */
answer_row_invariant(Vars, InferenceResult, DepthResult, QId, ErrorStream) :-
    ( InferenceResult \== inference_limit_exceeded,
      integer(DepthResult),
      \+ ground(Vars) ->
        emit_error(ErrorStream, proof, nonground_solution(QId), 1)
    ; true
    ).

answer_vars([], []).
answer_vars([answer(Var, _)|Rows], [Var|Vars]) :-
    answer_vars(Rows, Vars).

answer_classify_rows(Rows, QId, ErrorStream, Result) :-
    ( member(row(Values, InferenceResult, DepthResult), Rows),
      InferenceResult \== inference_limit_exceeded,
      integer(DepthResult),
      \+ ground(Values) ->
        emit_error(ErrorStream, proof, nonground_solution(QId), 1)
    ; member(row(_, InferenceResult, DepthResult), Rows),
      ( InferenceResult == inference_limit_exceeded
      ; DepthResult == depth_limit_exceeded
      ) ->
        Result = indeterminate(limit)
    ; findall(sol(Values), member(row(Values, _, _), Rows), Sols0),
      sort(Sols0, Sols),
      Result = solutions(Sols)
    ).

/* Artifact: comment line + one ground '$guideline_answers'/4 term,
   rendered whole before any byte reaches stdout. The digest binds the
   exact query-file bytes supplied to this run. */
answer_emit(Output, QId, Digest, Result) :-
    with_output_to(string(Out),
        ( format('% ~w answered against the loaded composition by ace_to_pl answer mode; do not edit.~n',
              [QId]),
          render_term_line('$guideline_answers'(v1, QId,
              query_sha256(Digest), result(Result)))
        )),
    string_codes(Out, OutCodes),
    format(Output, '~s', [OutCodes]).

/* ---------- trace mode: directed proof replay of a committed answers artifact ---------- */

/* Pipeline order (fixed): argv -> manifest read -> query file read +
   validation (answer mode's law verbatim) -> answers file read +
   validation -> composition load + structure parity -> per-row
   directed solve -> emit. The trace consumes the COMMITTED answers
   artifact: one directed first proof per positive row, never a second
   enumeration, so consistency with the answers holds by construction
   and drift surfaces as an unproved payload the freshness gate
   rejects. File parsing and final artifact rendering run outside the
   whole-run budget; per-row clause rendering/digesting runs inside it
   but outside the per-row search budget. */
trace_mode(Manifest, QueryPl, AnswersPl, Input, Output, ErrorStream) :-
    catch(
        ( aggregate_read_manifest(Manifest, PlPaths, _PayloadPaths) ->
            true
        ; throw(error(aggregate_manifest(Manifest),
              context(ace_to_pl:trace_mode/6, plain_failure)))
        ),
        Error,
        emit_error(ErrorStream, check_load, Error, 2)),
    answer_read_query(QueryPl, ErrorStream, QId, QueryDigest, Conj, Answers),
    trace_read_answers(AnswersPl, ErrorStream, QId, QueryDigest, Answers,
        AnswersDigest, AnswersResult),
    aggregate_declare_indicators,
    ( PlPaths == [] ->
        true
    ; aggregate_load(PlPaths, Input, Output, ErrorStream),
      aggregate_assertions(PlPaths, ErrorStream)
    ),
    trace_solve(Conj, Answers, AnswersResult, ErrorStream, Result),
    trace_emit(Output, QId, QueryDigest, AnswersDigest, Result, ErrorStream),
    halt(0).

/* MI frame depth bound. A DIFFERENT measure from answer-mode object
   depth (the meta-interpreter spends frames on conjunction walking and
   clause resolution); 1000 = explicit headroom over live proofs, with
   no cross-mode depth-equivalence claim. Per-row and whole-run
   inference budgets reuse the answer-mode constants. */
trace_depth_limit(1000).

/* Answers file = data, never consulted: raw bytes read once (strict
   UTF-8) feed answers_sha256 verbatim, then exactly one term is read
   under the committed census law (physical EOF = zero-width read; a
   literal end_of_file. term is data). Guard order: term_count ->
   record_shape (spans the '$guideline_answers'/4 + query_sha256/1 +
   result/1 wrappers) -> record_nonground -> record_version -> qid
   equality -> digest grammar -> digest custody (against the SHA-256
   this run computed over the supplied query-pl bytes, before any
   composition work) -> result algebra -> mode fit -> per-row solution
   checks in file order. Payloads come only from ground-guaranteed
   positions. */
trace_read_answers(File, ErrorStream, QId, QueryDigest, Answers, Digest,
        Result) :-
    catch(read_utf8_file(File, Bytes, Text),
        Error,
        emit_error(ErrorStream, check_load, Error, 2)),
    crypto_data_hash(Bytes, Digest, [algorithm(sha256), encoding(octet)]),
    catch(answer_query_terms(File, Text, Terms),
        Error2,
        emit_error(ErrorStream, check_load, Error2, 2)),
    ( Terms = [Record] ->
        true
    ; length(Terms, N),
      trace_reject(ErrorStream, term_count(N))
    ),
    ( compound(Record),
      Record = '$guideline_answers'(Version, RecordQId, Wrap, ResultWrap),
      compound(Wrap), Wrap = query_sha256(_),
      compound(ResultWrap), ResultWrap = result(_) ->
        true
    ; trace_reject(ErrorStream, record_shape)
    ),
    ( ground(Record) ->
        true
    ; trace_reject(ErrorStream, record_nonground)
    ),
    ( Version == v1 ->
        true
    ; trace_reject(ErrorStream, record_version(Version))
    ),
    ( RecordQId == QId ->
        true
    ; trace_reject(ErrorStream, qid_mismatch)
    ),
    Wrap = query_sha256(RecordDigest),
    ( answer_hex64(RecordDigest) ->
        true
    ; trace_reject(ErrorStream, record_query_sha256)
    ),
    ( RecordDigest == QueryDigest ->
        true
    ; trace_reject(ErrorStream, query_sha256_mismatch)
    ),
    ResultWrap = result(Result0),
    trace_validate_result(Result0, Answers, ErrorStream),
    Result = Result0.

trace_reject(ErrorStream, Why) :-
    emit_error(ErrorStream, check_load, answers_file(Why), 2).

/* Result algebra: closed shape first, then fit against the query's
   mode (wh = nonempty answers manifest, yes-no = empty), then the
   solutions payload row by row — sol/1 wrapper, proper-list values,
   arity against the manifest — all before any composition work. */
trace_validate_result(Result, Answers, ErrorStream) :-
    ( trace_result_shape(Result) ->
        true
    ; trace_reject(ErrorStream, result_shape)
    ),
    ( Answers == [] ->
        ( trace_yesno_result(Result) ->
            true
        ; trace_reject(ErrorStream, result_mode_mismatch)
        )
    ; ( trace_wh_result(Result) ->
          true
      ; trace_reject(ErrorStream, result_mode_mismatch)
      ),
      ( Result = solutions(Sols) ->
          ( is_list(Sols) ->
              true
          ; trace_reject(ErrorStream, solutions_list)
          ),
          length(Answers, Expected),
          trace_validate_solutions(Sols, 1, Expected, ErrorStream)
      ; true
      )
    ).

trace_result_shape(yes).
trace_result_shape(no(finite_failure)).
trace_result_shape(indeterminate(limit)).
trace_result_shape(solutions(_)).

trace_yesno_result(yes).
trace_yesno_result(no(finite_failure)).
trace_yesno_result(indeterminate(limit)).

trace_wh_result(solutions(_)).
trace_wh_result(indeterminate(limit)).

trace_validate_solutions([], _, _, _).
trace_validate_solutions([Sol|Sols], Index, Expected, ErrorStream) :-
    ( compound(Sol),
      Sol = sol(Values) ->
        true
    ; trace_reject(ErrorStream, solution_shape(Index))
    ),
    ( is_list(Values) ->
        true
    ; trace_reject(ErrorStream, solution_values(Index))
    ),
    length(Values, Actual),
    ( Actual =:= Expected ->
        true
    ; trace_reject(ErrorStream, solution_arity(Index, Expected, Actual))
    ),
    Next is Index + 1,
    trace_validate_solutions(Sols, Next, Expected, ErrorStream).

/* Whole-run budget around every row derivation INCLUDING per-row
   clause materialization; a trip replaces the mirror with
   indeterminate(limit) — mode-level legal, committed-state law
   rejects it at the gate. Bindings are lost on a trip, so the result
   is taken from the sentinel branch alone. */
trace_solve(Conj, Answers, AnswersResult, ErrorStream, Result) :-
    answer_outer_inference_limit(Outer),
    call_with_inference_limit(
        trace_solve_rows(Conj, Answers, AnswersResult, ErrorStream, Result0),
        Outer, OuterResult),
    ( OuterResult == inference_limit_exceeded ->
        Result = indeterminate(limit)
    ; Result = Result0
    ).

/* Mirror law: yes gains a proof payload; no(finite_failure) and
   indeterminate(limit) mirror verbatim with no entries; solutions
   rows keep committed Values, order and count, each gaining a
   payload. Every row proves against a fresh copy of the goal and its
   manifest variables (no cross-row binding leakage). */
trace_solve_rows(Conj, [], AnswersResult, ErrorStream, Result) :-
    !,
    ( AnswersResult == yes ->
        copy_term(Conj, ConjCopy),
        trace_prove_row(ConjCopy, ErrorStream, P),
        Result = yes(P)
    ; Result = AnswersResult
    ).
trace_solve_rows(Conj, Answers, AnswersResult, ErrorStream, Result) :-
    ( AnswersResult = solutions(Sols) ->
        answer_vars(Answers, Vars),
        trace_prove_solutions(Sols, Conj, Vars, ErrorStream, Pairs),
        Result = solutions(Pairs)
    ; Result = AnswersResult
    ).

trace_prove_solutions([], _, _, _, []).
trace_prove_solutions([sol(Values)|Sols], Conj, Vars, ErrorStream,
        [sol(Values, P)|Pairs]) :-
    copy_term(Conj-Vars, ConjCopy-VarsCopy),
    VarsCopy = Values,
    trace_prove_row(ConjCopy, ErrorStream, P),
    trace_prove_solutions(Sols, Conj, Vars, ErrorStream, Pairs).

/* One directed first proof under the frozen per-row wrapper; the
   sentinels are status VALUES. Search runs inside the inference
   budget; the skeleton materializes to nodes only after once/1
   returns, so clause rendering and digesting never charge the search
   budget. A sentinel inside a NAF site check surfaces as a thrown
   marker: that row is unproved(limit), never a naf leaf. Clean
   failure of the whole wrapper (no sentinel) = corpus-relative finite
   failure; a depth cut on the failure path binds the depth sentinel
   (probed on SWI 9.2.9), so failure-after-cut lands in
   unproved(limit). */
trace_prove_row(Goal, ErrorStream, P) :-
    trace_depth_limit(Depth),
    answer_inference_limit(Inferences),
    catch(
        ( call_with_inference_limit(
              call_with_depth_limit(
                  once(trace_mi(Goal, ErrorStream, Skels)),
                  Depth, DepthResult),
              Inferences, InferenceResult) ->
            ( InferenceResult \== inference_limit_exceeded,
              integer(DepthResult) ->
                trace_materialize(Skels, ErrorStream, Nodes),
                P = proved(Nodes)
            ; P = unproved(limit)
            )
        ; P = unproved(finite_failure)
        ),
        ace_to_pl_trace_naf_limit,
        P = unproved(limit)).

/* The meta-interpreter: ','/2 walks, NAF runs the site protocol, the
   seven semantic indicators resolve via clause/3 against user in
   engine clause order (= canonical forward-manifest load order), and
   every other reached goal form rejects fail-closed — in every
   context, positive or inside a NAF check. The skeleton records
   clause references and frozen NAF payloads only; rendering waits for
   materialization. */
trace_mi(Goal, ErrorStream, Skels) :-
    ( var(Goal) ->
        emit_error(ErrorStream, proof, trace_goal_variable, 1)
    ; Goal = (A, B) ->
        trace_mi(A, ErrorStream, SkelsA),
        trace_mi(B, ErrorStream, SkelsB),
        append(SkelsA, SkelsB, Skels)
    ; Goal = (\+ Inner) ->
        trace_mi_naf(Inner, ErrorStream, Skels)
    ; functor(Goal, Name, Arity),
      answer_goal_indicator(Name/Arity) ->
        Skels = [node0(Ref, ChildSkels)],
        clause(user:Goal, Body, Ref),
        trace_mi_body(Body, ErrorStream, ChildSkels)
    ; functor(Goal, Name, Arity),
      emit_error(ErrorStream, proof, trace_goal_foreign(Name, Arity), 1)
    ).

trace_mi_body(Body, ErrorStream, Skels) :-
    ( Body == true ->
        Skels = []
    ; trace_mi(Body, ErrorStream, Skels)
    ).

/* NAF site protocol: the engine never evaluates \+ directly (its
   inner search swallows a success-path depth cut, so a direct leaf
   could certify finite failure while the goal is derivable beyond the
   bound). The site prevalidates the whole box against the goal
   whitelist, then runs a positive MI-recursive derivability check of
   the argument under fresh site-local depth+inference wrappers:
   proved => the naf conjunct fails (cuts inside the successful check
   are harmless — the derivation itself fit the bounds); failed
   sentinel-free => sound naf leaf, payload frozen by copy_term at
   success time so later sibling bindings never rewrite it; any
   sentinel => the containing row is unproved(limit), signalled by a
   private throw the row wrapper catches. */
trace_mi_naf(Inner, ErrorStream, [naf0(Frozen)]) :-
    trace_naf_validate(Inner, ErrorStream),
    trace_depth_limit(Depth),
    answer_inference_limit(Inferences),
    ( call_with_inference_limit(
          call_with_depth_limit(
              once(trace_mi(Inner, ErrorStream, _)),
              Depth, DepthResult),
          Inferences, InferenceResult) ->
        ( InferenceResult \== inference_limit_exceeded,
          integer(DepthResult) ->
            fail
        ; throw(ace_to_pl_trace_naf_limit)
        )
    ; copy_term(Inner, Frozen)
    ).

trace_naf_validate(Goal, ErrorStream) :-
    ( var(Goal) ->
        emit_error(ErrorStream, proof, trace_goal_variable, 1)
    ; Goal = (A, B) ->
        trace_naf_validate(A, ErrorStream),
        trace_naf_validate(B, ErrorStream)
    ; Goal = (\+ Inner) ->
        trace_naf_validate(Inner, ErrorStream)
    ; functor(Goal, Name, Arity),
      answer_goal_indicator(Name/Arity) ->
        true
    ; functor(Goal, Name, Arity),
      emit_error(ErrorStream, proof, trace_goal_foreign(Name, Arity), 1)
    ).

/* Materialization (after once/1, inside the whole-run budget): each
   reference retrieves its PRISTINE renamed clause — never the
   instantiated resolution copy — re-renders it through the document
   clause-line writer, digests the exact line bytes including the LF,
   and extracts its unique sentence identity. NAF skeletons pass their
   frozen payloads through. */
trace_materialize([], _, []).
trace_materialize([node0(Ref, ChildSkels)|Skels], ErrorStream,
        [clause(sentence(DocId, S), clause_sha256(Hex), Children)|Nodes]) :-
    trace_clause_node(Ref, ErrorStream, DocId, S, Hex),
    trace_materialize(ChildSkels, ErrorStream, Children),
    trace_materialize(Skels, ErrorStream, Nodes).
trace_materialize([naf0(Frozen)|Skels], ErrorStream, [naf(Frozen)|Nodes]) :-
    trace_materialize(Skels, ErrorStream, Nodes).

trace_clause_node(Ref, ErrorStream, DocId, S, Hex) :-
    ( clause(user:Head, Body, Ref) ->
        true
    ; throw(error(trace_clause_ref(Ref),
          context(ace_to_pl:trace_clause_node/5, plain_failure)))
    ),
    ( acyclic_term((Head :- Body)),
      term_attvars((Head :- Body), []),
      canonical_tree((Head :- Body)) ->
        true
    ; emit_error(ErrorStream, proof, trace_unserializable, 1)
    ),
    trace_document_line(Head, Body, Line),
    crypto_data_hash(Line, Hex, [algorithm(sha256), encoding(utf8)]),
    trace_identity(Head, Body, ErrorStream, DocId, S).

/* Digest input = the committed DOCUMENT clause line bytes exactly as
   the document emitter wrote them (canonical head/goals, operator
   glue ' :- ', ', ', '\+ ' / '\+ (...)'), reconstructed through the
   emitter's own render_item/render_term_line discipline from the
   decompiled clause. */
trace_document_line(Head, Body, Line) :-
    with_output_to(string(Line),
        ( Body == true ->
            render_term_line(Head)
        ; trace_body_wrapped(Body, Wrapped),
          render_item(rule(Head, Wrapped))
        )).

trace_body_wrapped(Body, Wrapped) :-
    trace_body_list(Body, Wrapped, []).

trace_body_list(Conj, List, Tail) :-
    ( var(Conj) ->
        List = [pos(Conj)|Tail]
    ; Conj = (A, B) ->
        trace_body_list(A, List, Mid),
        trace_body_list(B, Mid, Tail)
    ; Conj = (\+ Inner) ->
        trace_naf_list(Inner, Goals),
        List = [naf_conj(Goals)|Tail]
    ; List = [pos(Conj)|Tail]
    ).

trace_naf_list(Conj, Goals) :-
    ( nonvar(Conj),
      Conj = (A, B) ->
        trace_naf_list(A, GoalsA),
        trace_naf_list(B, GoalsB),
        append(GoalsA, GoalsB, Goals)
    ; Goals = [Conj]
    ).

/* Identity law: count every '$guideline_id'(Role, DocId, S, _, _)
   subterm whose Role is a frozen schema role, DocId an atom and S a
   positive integer; repeated occurrences collapse. Anything other
   than exactly one distinct pair rejects fail-closed. */
trace_identity(Head, Body, ErrorStream, DocId, S) :-
    findall(D-Sx,
        ( aggregate_subterm((Head :- Body), Sub),
          nonvar(Sub),
          Sub = '$guideline_id'(Role, D, Sx, _, _),
          atom(Role),
          trace_identity_role(Role),
          atom(D),
          integer(Sx),
          Sx > 0
        ),
        Raw),
    sort(Raw, Pairs),
    ( Pairs = [DocId0-S0] ->
        DocId = DocId0,
        S = S0
    ; Pairs == [] ->
        emit_error(ErrorStream, proof, clause_identity(none), 1)
    ; length(Pairs, N),
      emit_error(ErrorStream, proof, clause_identity(multiple(N)), 1)
    ).

trace_identity_role(context).
trace_identity_role(product).
trace_identity_role(witness).

/* Artifact: comment line + one '$guideline_traces'/5 term, rendered
   whole before any byte reaches stdout. The trace writer numbers the
   whole term once left-to-right, then writes WITHOUT numbervars
   translation: every variable lands as a literal ground '$VAR'(N)
   subterm, so the committed artifact is ground under strict
   read-back. Unserializable payloads (from hostile inputs) reject
   before any output. */
trace_emit(Output, QId, QueryDigest, AnswersDigest, Result, ErrorStream) :-
    with_output_to(string(Out),
        ( format('% ~w traced against the loaded composition by ace_to_pl trace mode; do not edit.~n',
              [QId]),
          trace_render_term_line('$guideline_traces'(v1, QId,
              query_sha256(QueryDigest), answers_sha256(AnswersDigest),
              result(Result)), ErrorStream)
        )),
    string_codes(Out, OutCodes),
    format(Output, '~s', [OutCodes]).

trace_render_term_line(Term, ErrorStream) :-
    ( acyclic_term(Term),
      term_attvars(Term, []),
      canonical_tree(Term) ->
        true
    ; emit_error(ErrorStream, proof, trace_unserializable, 1)
    ),
    copy_term(Term, Copy),
    numbervars(Copy, 0, _),
    write_term(Copy,
        [ quoted(true),
          character_escapes(true),
          ignore_ops(true)
        ]),
    write('.'),
    nl.

/* ---------- compile mode ---------- */

compile_mode(Mode, Tree, DocId, Ulex, Input, Output, ErrorStream) :-
    set_stream(Input, type(binary)),
    prompt(_, ''),
    load_ape(Tree, Input, Output, ErrorStream),
    maybe_load_ulex(Ulex, Input, Output, ErrorStream, UlexDigest),
    read_input(Input, ErrorStream, Bytes, Text),
    mode_line_check(Mode, Text, ErrorStream),
    ( quarantined_call(Input, Output, ErrorStream,
          ace_to_drs:acetext_to_drs(Text, off, off, Sentences, _SyntaxTrees,
              Drs, Messages, _Time)) ->
        accept_or_reject(Mode, DocId, Bytes, Text, UlexDigest, Sentences, Drs,
            Messages, Output, ErrorStream)
    ; throw(error(ape_call_failed, context(ace_to_pl:compile_mode/7, Text)))
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
        ( mode_translate(Mode, DocId, Bytes, Text, UlexDigest, Sentences, Drs,
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
    v1_flatten_items(Group, root, S, DocId, [], actual, none, 1, _, Items),
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
    v1_seg_items(Segs, S, DocId, 1, N1, AItems),
    v1_flatten_items(CConds, consequent, S, DocId, ADom, actual, none,
        N1, _, CItems),
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
        1, NS, SharedItems),
    v1_flatten_items(Conds1, antecedent, S, DocId, [], actual, none,
        NS, NA1, Arm1Items),
    v1_flatten_items(Conds2, antecedent, S, DocId, [], actual, none,
        NA1, NA2, Arm2Items),
    append(ADom, Dom1, Deps1),
    append(ADom, Dom2, Deps2),
    v1_flatten_items(CConds, consequent, S, DocId, Deps1, actual, none,
        NA2, _, CItems1),
    v1_flatten_items(CConds, consequent, S, DocId, Deps2, actual, none,
        NA2, _, CItems2),
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
    header_term(DocId, AceDigest, UlexDigest, Header),
    v1_indicators(Indicators),
    with_output_to(string(Out),
        ( format('% ~w.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.~n',
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
    reject(unsupported, query_unsupported(leaf(list), 1)).
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
        Items),
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
    query_header_term(QId, AceDigest, UlexDigest, Record),
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

/* Bounded obligation call, shared by per-document and aggregate
   replay: the depth-limited search runs under a global inference
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
fallback_error_line(check_load, "ace_to_pl_error(check_load,unserializable).\n") :- !.
fallback_error_line(_, "ace_to_pl_error(uncaught,unserializable).\n").

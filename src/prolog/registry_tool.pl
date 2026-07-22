:- module(registry_tool, [main/0]).

:- set_prolog_flag(encoding, utf8).

:- use_module(library(readutil), [read_stream_to_codes/2]).
:- use_module(drs_canon, [canonical_line/2]).

/*
Run: swipl -q -f none -F none -s src/prolog/registry_tool.pl -g main
     -t 'halt(9)' -- registry|terminology|mapping|ulex
Input: one strict RFC 3629 UTF-8 canonical registry, terminology, or mapping
term stream. Registry, terminology, and mapping validation write no bytes.
Ulex writes the buffered canonical template stream. Failure writes one canonical
registry_tool_error(Stage,Class,Detail) line and no stdout bytes.
Exit: 0=success; 1=input-content rejection; 2=usage or uncaught internal error.
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

error_stage([registry], registry) :-
    !.
error_stage([terminology], terminology) :-
    !.
error_stage([mapping], mapping) :-
    !.
error_stage([ulex], ulex) :-
    !.
error_stage(_, cli).

run_cli([registry], Input, Output) :-
    !,
    with_output_to(string(Buffer), registry_input(Input)),
    format(Output, '~s', [Buffer]),
    flush_output(Output),
    halt(0).
run_cli([terminology], Input, Output) :-
    !,
    with_output_to(string(Buffer), terminology_input(Input)),
    format(Output, '~s', [Buffer]),
    flush_output(Output),
    halt(0).
run_cli([mapping], Input, Output) :-
    !,
    with_output_to(string(Buffer), mapping_input(Input)),
    format(Output, '~s', [Buffer]),
    flush_output(Output),
    halt(0).
run_cli([ulex], Input, Output) :-
    !,
    with_output_to(string(Buffer), ulex_input(Input)),
    format(Output, '~s', [Buffer]),
    flush_output(Output),
    halt(0).
run_cli(Argv, _, _) :-
    throw(registry_tool_failure(cli, usage, argv(Argv), 2)).

registry_input(Input) :-
    read_canonical_terms(Input, Terms),
    validate_registry(Terms).

terminology_input(Input) :-
    read_canonical_terms(Input, Terms),
    validate_terminology(Terms, _).

mapping_input(Input) :-
    read_canonical_terms(Input, Terms),
    validate_mapping(Terms).

ulex_input(Input) :-
    read_canonical_terms(Input, Terms),
    validate_terminology(Terms, Entries),
    entry_templates(Entries, Templates),
    generated_record_call(ulex_serialization,
        canonical_codes(Templates, 1, Codes)),
    format('~s', [Codes]).

entry_templates([], []).
entry_templates([entry(_, _, _, _, Template)|Entries],
        [Template|Templates]) :-
    entry_templates(Entries, Templates).

generated_record_call(Context, Goal) :-
    catch(Goal,
        registry_reject(Class, Detail),
        throw(error(generated_record_invalid(Class, Detail),
            context(registry_tool, Context)))).

read_canonical_terms(Input, Terms) :-
    read_utf8_input(Input, Text, Codes),
    parse_terms(Text, Terms),
    canonical_fixed_point(Terms, Codes).

read_utf8_input(Input, Text, Codes) :-
    read_stream_to_codes(Input, Bytes),
    decode_utf8(Bytes, Codes, 0),
    string_codes(Text, Codes).

decode_utf8([], [], _).
decode_utf8([Byte|Bytes], [Code|Codes], Offset) :-
    ( decode_utf8_unit(Byte, Bytes, Code, Rest, Width) ->
        Next is Offset + Width,
        decode_utf8(Rest, Codes, Next)
    ; throw(registry_reject(input_utf8, byte_offset(Offset)))
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
              [ module(registry_tool),
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
    throw(registry_reject(syntax, term(Index))).
handle_read_error(_, Error) :-
    throw(Error).

canonical_fixed_point(Terms, InputCodes) :-
    canonical_codes(Terms, 1, CanonicalCodes),
    ( InputCodes == CanonicalCodes ->
        true
    ; first_difference(InputCodes, CanonicalCodes, 0, Offset),
      throw(registry_reject(canonical, codepoint_offset(Offset)))
    ).

canonical_codes([], _, []).
canonical_codes([Term|Terms], Index, Codes) :-
    copy_term(Term, Copy),
    ( catch(canonical_line(Copy, Line), _, fail) ->
        string_codes(Line, Here)
    ; throw(registry_reject(canonical, term(Index, unserializable)))
    ),
    Next is Index + 1,
    canonical_codes(Terms, Next, Rest),
    append(Here, Rest, Codes).

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

validate_registry(Terms) :-
    require_version(Terms, cnl_guideline_registry(1), Rows),
    validate_registry_rows(Rows, 2, Records),
    require_registry_source(Records),
    validate_registry_references(Records),
    validate_registry_duplicates(Records),
    validate_registry_ordering(Records).

require_version([], Expected, _) :-
    throw(registry_reject(version, term(1, missing(Expected)))).
require_version([First|Rows], Expected, Rows) :-
    ( First == Expected ->
        true
    ; throw(registry_reject(version,
          term(1, expected(Expected, found(First)))))
    ).

validate_registry_rows([], _, []).
validate_registry_rows([Term|Terms], Index, [Record|Records]) :-
    registry_row_kind(Term, Index, Kind),
    validate_registry_row(Kind, Term, Index, Record),
    Next is Index + 1,
    validate_registry_rows(Terms, Next, Records).

registry_row_kind(Term, Index, Kind) :-
    term_name_arity(Term, Name, Arity),
    ( Name == guideline_source ->
        require_row_arity(Name, Arity, 11, Index),
        Kind = source
    ; Name == extraction_evidence ->
        require_row_arity(Name, Arity, 5, Index),
        Kind = extraction
    ; Name == guideline_region ->
        require_row_arity(Name, Arity, 6, Index),
        Kind = region
    ; Name == guideline_item_state ->
        require_row_arity(Name, Arity, 3, Index),
        Kind = item_state
    ; Name == guideline_blocked_proposal ->
        require_row_arity(Name, Arity, 4, Index),
        Kind = blocked_proposal
    ; throw(registry_reject(row,
          term(Index, unknown_constructor(Name, Arity))))
    ).

term_name_arity(Term, Name, Arity) :-
    ( compound(Term) ->
        functor(Term, Name, Arity)
    ; atom(Term) ->
        Name = Term,
        Arity = 0
    ; Name = atomic,
      Arity = 0
    ).

require_row_arity(_, Actual, Expected, _) :-
    Actual =:= Expected,
    !.
require_row_arity(Name, Actual, Expected, Index) :-
    throw(registry_reject(row,
        term(Index, constructor(Name, arity(expected(Expected), found(Actual)))))).

validate_registry_row(Kind, Term, Index, Record) :-
    ( Kind == source ->
        validate_source_row(Term, Index, Record)
    ; Kind == extraction ->
        validate_extraction_row(Term, Index, Record)
    ; Kind == region ->
        validate_region_row(Term, Index, Record)
    ; Kind == item_state ->
        validate_item_state_row(Term, Index, Record)
    ; Kind == blocked_proposal ->
        validate_blocked_proposal_row(Term, Index, Record)
    ).

validate_source_row(Term, Index,
        record(source, [GuidelineId], Index,
            source(GuidelineId, ArtifactPath))) :-
    arg(1, Term, GuidelineId),
    expect_stable_id(GuidelineId, Index, guideline_id),
    arg(2, Term, TitleTerm),
    wrapper_value(TitleTerm, title, Index, title, Title),
    expect_nonempty_atom(Title, Index, title),
    arg(3, Term, OrganizationTerm),
    wrapper_value(OrganizationTerm, issuing_organization, Index,
        issuing_organization, Organization),
    expect_nonempty_atom(Organization, Index, issuing_organization),
    arg(4, Term, Urls),
    validate_urls(Urls, Index),
    arg(5, Term, VersionTerm),
    wrapper_value(VersionTerm, version, Index, version, Version),
    expect_nonempty_atom(Version, Index, version),
    arg(6, Term, LanguageTerm),
    wrapper_value(LanguageTerm, language, Index, language, Language),
    expect_nonempty_atom(Language, Index, language),
    arg(7, Term, Artifact),
    validate_artifact(Artifact, Index, ArtifactPath),
    arg(8, Term, Retrieval),
    validate_retrieval(Retrieval, Index, retrieval),
    arg(9, Term, CrossManifestation),
    validate_cross_manifestation(CrossManifestation, Index),
    arg(10, Term, Rights),
    validate_rights(Rights, Index),
    arg(11, Term, PublicationStatus),
    validate_publication_status(PublicationStatus, Index).

validate_urls(Term, Index) :-
    expect_compound(Term, urls, 3, Index, urls),
    arg(1, Term, LandingTerm),
    wrapper_value(LandingTerm, landing_url, Index, landing_url, Landing),
    expect_nonempty_atom(Landing, Index, landing_url),
    arg(2, Term, DoiTerm),
    wrapper_value(DoiTerm, doi, Index, doi, Doi),
    expect_nonempty_atom(Doi, Index, doi),
    arg(3, Term, ArtifactTerm),
    wrapper_value(ArtifactTerm, artifact_url, Index, artifact_url, Url),
    expect_nonempty_atom(Url, Index, artifact_url).

validate_artifact(Term, Index, ArtifactPath) :-
    expect_compound(Term, artifact, 4, Index, artifact),
    arg(1, Term, PathTerm),
    wrapper_value(PathTerm, relpath, Index, artifact_relpath, ArtifactPath),
    expect_relpath(ArtifactPath, Index, artifact_relpath),
    arg(2, Term, HashTerm),
    wrapper_value(HashTerm, artifact_sha256, Index,
        artifact_sha256, Hash),
    validate_digest(Hash, Index, artifact_sha256),
    arg(3, Term, BytesTerm),
    wrapper_value(BytesTerm, byte_length, Index,
        artifact_byte_length, Bytes),
    expect_positive_integer(Bytes, Index, artifact_byte_length),
    arg(4, Term, MediaTerm),
    wrapper_value(MediaTerm, media_type, Index,
        artifact_media_type, MediaType),
    expect_nonempty_atom(MediaType, Index, artifact_media_type).

validate_retrieval(Term, Index, Field) :-
    expect_compound(Term, retrieval, 4, Index, Field),
    arg(1, Term, FirstTerm),
    wrapper_value(FirstTerm, first_retrieved_at, Index,
        first_retrieved_at, First),
    expect_nonempty_atom(First, Index, first_retrieved_at),
    arg(2, Term, SecondTerm),
    wrapper_value(SecondTerm, second_retrieved_at, Index,
        second_retrieved_at, Second),
    expect_nonempty_atom(Second, Index, second_retrieved_at),
    arg(3, Term, CountTerm),
    wrapper_value(CountTerm, fetch_count, Index, fetch_count, Count),
    expect_minimum_integer(Count, 2, Index, fetch_count),
    arg(4, Term, IdenticalTerm),
    wrapper_value(IdenticalTerm, byte_identical, Index,
        byte_identical, Identical),
    expect_boolean(Identical, Index, byte_identical).

validate_cross_manifestation(Term, Index) :-
    expect_compound(Term, cross_manifestation, 9, Index,
        cross_manifestation),
    arg(1, Term, KindTerm),
    wrapper_value(KindTerm, kind, Index, cross_kind, Kind),
    expect_nonempty_atom(Kind, Index, cross_kind),
    arg(2, Term, UrlTerm),
    wrapper_value(UrlTerm, url, Index, cross_url, Url),
    expect_nonempty_atom(Url, Index, cross_url),
    arg(3, Term, HashTerm),
    wrapper_value(HashTerm, evidence_sha256, Index,
        evidence_sha256, Hash),
    validate_digest(Hash, Index, evidence_sha256),
    arg(4, Term, BytesTerm),
    wrapper_value(BytesTerm, byte_length, Index,
        evidence_byte_length, Bytes),
    expect_positive_integer(Bytes, Index, evidence_byte_length),
    arg(5, Term, MediaTerm),
    wrapper_value(MediaTerm, media_type, Index,
        evidence_media_type, MediaType),
    expect_nonempty_atom(MediaType, Index, evidence_media_type),
    arg(6, Term, FirstTerm),
    wrapper_value(FirstTerm, first_retrieved_at, Index,
        evidence_first_retrieved_at, First),
    expect_nonempty_atom(First, Index, evidence_first_retrieved_at),
    arg(7, Term, SecondTerm),
    wrapper_value(SecondTerm, second_retrieved_at, Index,
        evidence_second_retrieved_at, Second),
    expect_nonempty_atom(Second, Index, evidence_second_retrieved_at),
    arg(8, Term, CountTerm),
    wrapper_value(CountTerm, fetch_count, Index,
        evidence_fetch_count, Count),
    expect_minimum_integer(Count, 2, Index, evidence_fetch_count),
    arg(9, Term, IdenticalTerm),
    wrapper_value(IdenticalTerm, byte_identical, Index,
        evidence_byte_identical, Identical),
    expect_boolean(Identical, Index, evidence_byte_identical).

validate_rights(Term, Index) :-
    expect_compound(Term, rights, 8, Index, rights),
    arg(1, Term, CopyrightTerm),
    wrapper_value(CopyrightTerm, copyright_status, Index,
        copyright_status, CopyrightStatus),
    expect_enum(CopyrightStatus,
        [public_domain, licensed, restricted, unknown],
        Index, copyright_status),
    arg(2, Term, LabelTerm),
    wrapper_value(LabelTerm, rights_label, Index, rights_label, Label),
    expect_nonempty_atom(Label, Index, rights_label),
    arg(3, Term, EvidenceTerm),
    validate_rights_evidence(EvidenceTerm, Index),
    arg(4, Term, ObligationsTerm),
    wrapper_value(ObligationsTerm, obligations, Index,
        obligations, Obligations),
    validate_obligations(Obligations, Index),
    arg(5, Term, DerivativeTerm),
    wrapper_value(DerivativeTerm, derivative_mode, Index,
        derivative_mode, DerivativeMode),
    expect_enum(DerivativeMode,
        [project_authored_mapping, source_adaptation, none],
        Index, derivative_mode),
    arg(6, Term, RedistributionTerm),
    wrapper_value(RedistributionTerm, redistribution_status, Index,
        redistribution_status, RedistributionStatus),
    expect_enum(RedistributionStatus,
        [redistributable, reconstructable, restricted_internal_only],
        Index, redistribution_status),
    arg(7, Term, CommitTerm),
    wrapper_value(CommitTerm, may_commit, Index, may_commit, MayCommit),
    expect_enum(MayCommit, [yes, conditional, no], Index, may_commit),
    arg(8, Term, AttributionTerm),
    wrapper_value(AttributionTerm, attribution_text, Index,
        attribution_text, Attribution),
    expect_nonempty_atom(Attribution, Index, attribution_text).

validate_rights_evidence(Term, Index) :-
    expect_compound(Term, rights_evidence, 3, Index, rights_evidence),
    arg(1, Term, QuoteTerm),
    wrapper_value(QuoteTerm, quote, Index, rights_quote, Quote),
    expect_nonempty_atom(Quote, Index, rights_quote),
    arg(2, Term, UrlTerm),
    wrapper_value(UrlTerm, url, Index, rights_url, Url),
    expect_nonempty_atom(Url, Index, rights_url),
    arg(3, Term, RetrievedTerm),
    wrapper_value(RetrievedTerm, retrieved_at, Index,
        rights_retrieved_at, RetrievedAt),
    expect_nonempty_atom(RetrievedAt, Index, rights_retrieved_at).

validate_obligations(Obligations, Index) :-
    ( is_list(Obligations) ->
        true
    ; throw(registry_reject(shape, term(Index, field(obligations))))
    ),
    validate_obligation_atoms(Obligations, Index),
    validate_strict_atom_order(Obligations, Index, obligations).

validate_obligation_atoms([], _).
validate_obligation_atoms([Obligation|Obligations], Index) :-
    expect_enum(Obligation,
        [attribution, no_marks, non_endorsement, source_free_availability],
        Index, obligations),
    validate_obligation_atoms(Obligations, Index).

validate_publication_status(Term, Index) :-
    expect_compound(Term, publication_status, 3, Index,
        publication_status),
    arg(1, Term, Status),
    expect_enum(Status,
        [current_as_observed, superseded, archived, unknown],
        Index, publication_status),
    arg(2, Term, CheckedTerm),
    wrapper_value(CheckedTerm, status_checked_at, Index,
        status_checked_at, CheckedAt),
    expect_nonempty_atom(CheckedAt, Index, status_checked_at),
    arg(3, Term, EvidenceTerm),
    expect_compound(EvidenceTerm, evidence, 2, Index,
        status_evidence),
    arg(1, EvidenceTerm, UrlTerm),
    wrapper_value(UrlTerm, url, Index, status_evidence_url, Url),
    expect_nonempty_atom(Url, Index, status_evidence_url),
    arg(2, EvidenceTerm, UpdatedTerm),
    wrapper_value(UpdatedTerm, updated_at, Index,
        status_evidence_updated_at, UpdatedAt),
    expect_nonempty_atom(UpdatedAt, Index, status_evidence_updated_at).

validate_extraction_row(Term, Index,
        record(extraction, [GuidelineId, ExtractionId], Index,
            extraction(GuidelineId, ExtractionId, ArtifactPath))) :-
    arg(1, Term, GuidelineId),
    expect_stable_id(GuidelineId, Index, guideline_id),
    arg(2, Term, ExtractionId),
    expect_stable_id(ExtractionId, Index, extraction_id),
    arg(3, Term, PathTerm),
    wrapper_value(PathTerm, relpath, Index, extraction_relpath, Path),
    expect_relpath(Path, Index, extraction_relpath),
    arg(4, Term, HashTerm),
    wrapper_value(HashTerm, extraction_sha256, Index,
        extraction_sha256, Hash),
    validate_digest(Hash, Index, extraction_sha256),
    arg(5, Term, ArtifactTerm),
    wrapper_value(ArtifactTerm, artifact_relpath, Index,
        artifact_relpath, ArtifactPath),
    expect_relpath(ArtifactPath, Index, artifact_relpath).

validate_region_row(Term, Index,
        record(region, [GuidelineId, RegionId], Index,
            region(GuidelineId, RegionId, ExtractionId))) :-
    arg(1, Term, GuidelineId),
    expect_stable_id(GuidelineId, Index, guideline_id),
    arg(2, Term, RegionId),
    expect_stable_id(RegionId, Index, region_id),
    arg(3, Term, ExtractionId),
    expect_stable_id(ExtractionId, Index, extraction_id),
    arg(4, Term, PagesTerm),
    validate_pdf_pages(PagesTerm, Index),
    arg(5, Term, RangeTerm),
    validate_byte_range(RangeTerm, Index),
    arg(6, Term, HashTerm),
    wrapper_value(HashTerm, region_sha256, Index, region_sha256, Hash),
    validate_digest(Hash, Index, region_sha256).

validate_pdf_pages(Term, Index) :-
    expect_compound(Term, pdf_pages, 4, Index, pdf_pages),
    arg(1, Term, PhysicalFirst),
    arg(2, Term, PhysicalLast),
    arg(3, Term, PrintedFirst),
    arg(4, Term, PrintedLast),
    expect_positive_integer(PhysicalFirst, Index, pdf_pages),
    expect_positive_integer(PhysicalLast, Index, pdf_pages),
    expect_positive_integer(PrintedFirst, Index, pdf_pages),
    expect_positive_integer(PrintedLast, Index, pdf_pages),
    ( PhysicalFirst =< PhysicalLast,
      PrintedFirst =< PrintedLast ->
        true
    ; throw(registry_reject(range,
          term(Index, pdf_pages(PhysicalFirst, PhysicalLast,
              PrintedFirst, PrintedLast))))
    ).

validate_byte_range(Term, Index) :-
    expect_compound(Term, byte_range, 2, Index, byte_range),
    arg(1, Term, Start),
    arg(2, Term, End),
    ( integer(Start), integer(End) ->
        true
    ; throw(registry_reject(shape, term(Index, field(byte_range))))
    ),
    ( Start >= 0, Start < End ->
        true
    ; throw(registry_reject(range, term(Index, byte_range(Start, End))))
    ).

validate_item_state_row(Term, Index,
        record(item_state, [GuidelineId, ItemId], Index,
            item_state(GuidelineId, ItemId, State))) :-
    arg(1, Term, GuidelineId),
    expect_stable_id(GuidelineId, Index, guideline_id),
    arg(2, Term, ItemId),
    expect_stable_id(ItemId, Index, item_id),
    arg(3, Term, State),
    expect_enum(State, [done, blocked, excluded], Index, item_state).

validate_blocked_proposal_row(Term, Index,
        record(blocked_proposal, [GuidelineId, ItemId, ProposalId], Index,
            blocked_proposal(GuidelineId, ItemId, ProposalId))) :-
    arg(1, Term, GuidelineId),
    expect_stable_id(GuidelineId, Index, guideline_id),
    arg(2, Term, ItemId),
    expect_stable_id(ItemId, Index, item_id),
    arg(3, Term, ProposalId),
    expect_stable_id(ProposalId, Index, proposal_id),
    arg(4, Term, ReasonTerm),
    wrapper_value(ReasonTerm, reason, Index, reason, Reason),
    expect_nonempty_atom(Reason, Index, reason).

require_registry_source(Records) :-
    ( record_kind_present(Records, source) ->
        true
    ; throw(registry_reject(row, required(guideline_source)))
    ).

record_kind_present([record(Kind, _, _, _)|_], Expected) :-
    Kind == Expected,
    !.
record_kind_present([_|Records], Expected) :-
    record_kind_present(Records, Expected).

validate_registry_references([]).
validate_registry_references([Record|Records]) :-
    validate_registry_record_references(Record, [Record|Records]),
    validate_registry_references_with_all(Records, [Record|Records]).

validate_registry_references_with_all([], _).
validate_registry_references_with_all([Record|Records], All) :-
    validate_registry_record_references(Record, All),
    validate_registry_references_with_all(Records, All).

validate_registry_record_references(
        record(source, _, _, _), _).
validate_registry_record_references(
        record(extraction, _, Index,
            extraction(GuidelineId, ExtractionId, ArtifactPath)), Records) :-
    ( find_source(Records, GuidelineId, SourceArtifactPath) ->
        ( ArtifactPath == SourceArtifactPath ->
            true
        ; throw(registry_reject(reference,
              term(Index, artifact_relpath(GuidelineId, ExtractionId))))
        )
    ; throw(registry_reject(reference,
          term(Index, guideline_id(GuidelineId))))
    ).
validate_registry_record_references(
        record(region, _, Index,
            region(GuidelineId, RegionId, ExtractionId)), Records) :-
    ( find_source(Records, GuidelineId, _) ->
        true
    ; throw(registry_reject(reference,
          term(Index, guideline_id(GuidelineId))))
    ),
    ( find_extraction(Records, GuidelineId, ExtractionId) ->
        true
    ; throw(registry_reject(reference,
          term(Index, extraction_id(GuidelineId, RegionId, ExtractionId))))
    ).
validate_registry_record_references(
        record(item_state, _, Index,
            item_state(GuidelineId, ItemId, _)), Records) :-
    ( find_source(Records, GuidelineId, _) ->
        true
    ; throw(registry_reject(reference,
          term(Index, item(GuidelineId, ItemId))))
    ).
validate_registry_record_references(
        record(blocked_proposal, _, Index,
            blocked_proposal(GuidelineId, ItemId, ProposalId)), Records) :-
    ( find_source(Records, GuidelineId, _) ->
        true
    ; throw(registry_reject(reference,
          term(Index, guideline_id(GuidelineId))))
    ),
    ( find_item_state(Records, GuidelineId, ItemId, State) ->
        ( State == blocked ->
            true
        ; throw(registry_reject(reference,
              term(Index, blocked_state(GuidelineId, ItemId, ProposalId))))
        )
    ; throw(registry_reject(reference,
          term(Index, item_state(GuidelineId, ItemId, ProposalId))))
    ).

find_source([record(source, _, _, source(Id, ArtifactPath))|_],
        GuidelineId, ArtifactPath) :-
    Id == GuidelineId,
    !.
find_source([_|Records], GuidelineId, ArtifactPath) :-
    find_source(Records, GuidelineId, ArtifactPath).

find_extraction([record(extraction, _, _,
        extraction(Id, ExtractionId0, _))|_], GuidelineId, ExtractionId) :-
    Id == GuidelineId,
    ExtractionId0 == ExtractionId,
    !.
find_extraction([_|Records], GuidelineId, ExtractionId) :-
    find_extraction(Records, GuidelineId, ExtractionId).

find_item_state([record(item_state, _, _,
        item_state(Id, ItemId0, State))|_], GuidelineId, ItemId, State) :-
    Id == GuidelineId,
    ItemId0 == ItemId,
    !.
find_item_state([_|Records], GuidelineId, ItemId, State) :-
    find_item_state(Records, GuidelineId, ItemId, State).

validate_registry_duplicates(Records) :-
    validate_duplicate_keys(Records, [], [], [], [], []).

validate_duplicate_keys([], _, _, _, _, _).
validate_duplicate_keys([record(Kind, Key, Index, Data)|Records],
        SourceSeen, ExtractionSeen, RegionSeen, StateSeen, ProposalSeen) :-
    ( Kind == source ->
        Data = source(GuidelineId, _),
        check_seen(GuidelineId, Index, guideline_id, SourceSeen),
        NextSource = [seen(GuidelineId, Index)|SourceSeen],
        NextExtraction = ExtractionSeen,
        NextRegion = RegionSeen,
        NextState = StateSeen,
        NextProposal = ProposalSeen
    ; Kind == extraction ->
        check_seen(Key, Index, extraction_key, ExtractionSeen),
        NextSource = SourceSeen,
        NextExtraction = [seen(Key, Index)|ExtractionSeen],
        NextRegion = RegionSeen,
        NextState = StateSeen,
        NextProposal = ProposalSeen
    ; Kind == region ->
        Data = region(_, RegionId, _),
        check_seen(RegionId, Index, region_id, RegionSeen),
        NextSource = SourceSeen,
        NextExtraction = ExtractionSeen,
        NextRegion = [seen(RegionId, Index)|RegionSeen],
        NextState = StateSeen,
        NextProposal = ProposalSeen
    ; Kind == item_state ->
        check_seen(Key, Index, item_state_key, StateSeen),
        NextSource = SourceSeen,
        NextExtraction = ExtractionSeen,
        NextRegion = RegionSeen,
        NextState = [seen(Key, Index)|StateSeen],
        NextProposal = ProposalSeen
    ; check_seen(Key, Index, blocked_proposal_key, ProposalSeen),
      NextSource = SourceSeen,
      NextExtraction = ExtractionSeen,
      NextRegion = RegionSeen,
      NextState = StateSeen,
      NextProposal = [seen(Key, Index)|ProposalSeen]
    ),
    validate_duplicate_keys(Records, NextSource, NextExtraction,
        NextRegion, NextState, NextProposal).

check_seen(Value, Index, Label, Seen) :-
    ( seen_value(Seen, Value, FirstIndex) ->
        throw(registry_reject(duplicate,
            term(Index, duplicate(Label, Value, first_term(FirstIndex)))))
    ; true
    ).

seen_value([seen(SeenValue, Index)|_], Value, Index) :-
    SeenValue == Value,
    !.
seen_value([_|Seen], Value, Index) :-
    seen_value(Seen, Value, Index).

validate_registry_ordering([]).
validate_registry_ordering([Record|Records]) :-
    validate_registry_ordering_(Records, Record).

validate_registry_ordering_([], _).
validate_registry_ordering_([Record|Records], Previous) :-
    record_rank(Previous, PreviousRank),
    record_rank(Record, Rank),
    Record = record(Kind, Key, Index, _),
    Previous = record(PreviousKind, PreviousKey, PreviousIndex, _),
    ( Rank > PreviousRank ->
        true
    ; Rank < PreviousRank ->
        throw(registry_reject(ordering,
            term(Index, section(Kind, after(PreviousKind,
                previous_term(PreviousIndex))))))
    ; compare_atom_key(Order, Key, PreviousKey),
      ( Order == (>) ->
          true
      ; throw(registry_reject(ordering,
            term(Index, key(Kind, Key,
                after(PreviousKey, previous_term(PreviousIndex))))))
      )
    ),
    validate_registry_ordering_(Records, Record).

record_rank(record(Kind, _, _, _), Rank) :-
    ( Kind == source -> Rank = 1
    ; Kind == extraction -> Rank = 2
    ; Kind == region -> Rank = 3
    ; Kind == item_state -> Rank = 4
    ; Rank = 5
    ).

validate_mapping(Terms) :-
    require_version(Terms, cnl_guideline_mapping(1), Rows),
    validate_mapping_rows(Rows, 2, Records),
    require_mapping_region(Records),
    validate_mapping_references(Records),
    validate_mapping_duplicates(Records),
    validate_mapping_ordering(Records),
    validate_mapping_coverage(Records).

validate_mapping_rows([], _, []).
validate_mapping_rows([Term|Terms], Index, [Record|Records]) :-
    mapping_row_kind(Term, Index, Kind),
    validate_mapping_row(Kind, Term, Index, Record),
    Next is Index + 1,
    validate_mapping_rows(Terms, Next, Records).

mapping_row_kind(Term, Index, Kind) :-
    term_name_arity(Term, Name, Arity),
    ( Name == mapping_document ->
        require_row_arity(Name, Arity, 4, Index),
        Kind = document
    ; Name == mapping_region ->
        require_row_arity(Name, Arity, 2, Index),
        Kind = region
    ; Name == mapping_claim ->
        require_row_arity(Name, Arity, 7, Index),
        Kind = claim
    ; Name == mapping_residual ->
        require_row_arity(Name, Arity, 5, Index),
        Kind = residual
    ; throw(registry_reject(row,
          term(Index, unknown_constructor(Name, Arity))))
    ).

validate_mapping_row(Kind, Term, Index, Record) :-
    ( Kind == document ->
        validate_mapping_document_row(Term, Index, Record)
    ; Kind == region ->
        validate_mapping_region_row(Term, Index, Record)
    ; Kind == claim ->
        validate_mapping_claim_row(Term, Index, Record)
    ; validate_mapping_residual_row(Term, Index, Record)
    ).

validate_mapping_document_row(Term, Index,
        mapping_record(document, [GuidelineId, Docid], Index,
            document(GuidelineId, Docid))) :-
    arg(1, Term, GuidelineId),
    expect_stable_id(GuidelineId, Index, guideline_id),
    arg(2, Term, Docid),
    expect_mapping_docid(Docid, Index),
    arg(3, Term, Ace),
    validate_mapping_document_artifact(
        Ace, ace, ace_relpath, ace_sha256, '.ace', Docid, Index),
    arg(4, Term, Ulex),
    validate_mapping_document_artifact(
        Ulex, ulex, ulex_relpath, ulex_sha256, '.ulex', Docid, Index).

validate_mapping_document_artifact(Term, Constructor, PathField,
        DigestField, Suffix, Docid, Index) :-
    expect_compound(Term, Constructor, 2, Index, Constructor),
    arg(1, Term, PathTerm),
    wrapper_value(PathTerm, relpath, Index, PathField, Path),
    expect_relpath(Path, Index, PathField),
    mapping_path_final_segment(Path, FinalSegment),
    atom_concat(Docid, Suffix, ExpectedSegment),
    ( FinalSegment == ExpectedSegment ->
        true
    ; throw(registry_reject(shape, term(Index, field(PathField))))
    ),
    arg(2, Term, DigestTerm),
    wrapper_value(DigestTerm, DigestField, Index,
        DigestField, Digest),
    validate_digest(Digest, Index, DigestField).

mapping_path_final_segment(Path, FinalSegment) :-
    atomic_list_concat(Segments, '/', Path),
    mapping_last_segment(Segments, FinalSegment).

mapping_last_segment([Segment], Segment).
mapping_last_segment([_|Segments], FinalSegment) :-
    mapping_last_segment(Segments, FinalSegment).

expect_mapping_docid(Value, Index) :-
    ( valid_mapping_docid(Value) ->
        true
    ; throw(registry_reject(shape, term(Index, field(docid))))
    ).

valid_mapping_docid(Value) :-
    atom(Value),
    atom_codes(Value, [First|Rest]),
    First =\= 0'-,
    valid_mapping_docid_codes([First|Rest]).

valid_mapping_docid_codes([]).
valid_mapping_docid_codes([Code|Codes]) :-
    ( ascii_lower_or_digit(Code) ; Code =:= 0'- ),
    valid_mapping_docid_codes(Codes).

validate_mapping_region_row(Term, Index,
        mapping_record(region, [GuidelineId, RegionId], Index,
            region(GuidelineId, RegionId))) :-
    arg(1, Term, GuidelineId),
    expect_stable_id(GuidelineId, Index, guideline_id),
    arg(2, Term, RegionId),
    expect_stable_id(RegionId, Index, region_id).

validate_mapping_claim_row(Term, Index,
        mapping_record(claim, [GuidelineId, RegionId, ClaimId], Index,
            claim(GuidelineId, RegionId, ClaimId, Docid))) :-
    arg(1, Term, GuidelineId),
    expect_stable_id(GuidelineId, Index, guideline_id),
    arg(2, Term, RegionId),
    expect_stable_id(RegionId, Index, region_id),
    arg(3, Term, ClaimId),
    expect_stable_id(ClaimId, Index, claim_id),
    arg(4, Term, ProjectionTerm),
    wrapper_value(ProjectionTerm, projection, Index,
        projection, Projection),
    expect_enum(Projection, [applicability, action_kind],
        Index, projection),
    arg(5, Term, DocidTerm),
    wrapper_value(DocidTerm, docid, Index, docid, Docid),
    expect_mapping_docid(Docid, Index),
    arg(6, Term, ItemsTerm),
    wrapper_value(ItemsTerm, items, Index, items, Items),
    validate_mapping_items(Items, Index, Query),
    arg(7, Term, AnswerTerm),
    wrapper_value(AnswerTerm, expected_answer, Index,
        expected_answer, Answer),
    validate_mapping_answer(Answer, Index, AnswerQuery),
    ( AnswerQuery == Query ->
        true
    ; throw(registry_reject(shape,
          term(Index, field(expected_answer_query))))
    ).

validate_mapping_items(Items, Index, Query) :-
    ( is_list(Items) ->
        true
    ; throw(registry_reject(shape, term(Index, field(items))))
    ),
    validate_mapping_items_(Items, Index, 0, 0, none,
        RuleCount, QueryCount, QueryState),
    ( RuleCount >= 1,
      QueryCount =:= 1,
      QueryState = query(Query) ->
        true
    ; throw(registry_reject(shape, term(Index, field(items))))
    ),
    validate_mapping_canonical_order(Items, Index, items).

validate_mapping_items_([], _, RuleCount, QueryCount, QueryState,
        RuleCount, QueryCount, QueryState).
validate_mapping_items_([Item|Items], Index,
        RuleCount0, QueryCount0, QueryState0,
        RuleCount, QueryCount, QueryState) :-
    term_name_arity(Item, Name, Arity),
    ( Name == rule_id, Arity =:= 2 ->
        validate_mapping_item_id(Item, Index),
        RuleCount1 is RuleCount0 + 1,
        QueryCount1 = QueryCount0,
        QueryState1 = QueryState0
    ; Name == query_id, Arity =:= 2 ->
        validate_mapping_item_id(Item, Index),
        RuleCount1 = RuleCount0,
        QueryCount1 is QueryCount0 + 1,
        ( QueryState0 == none ->
            QueryState1 = query(Item)
        ; QueryState1 = QueryState0
        )
    ; throw(registry_reject(shape, term(Index, field(items))))
    ),
    validate_mapping_items_(Items, Index,
        RuleCount1, QueryCount1, QueryState1,
        RuleCount, QueryCount, QueryState).

validate_mapping_item_id(Term, Index) :-
    arg(1, Term, SentenceTerm),
    expect_compound(SentenceTerm, sentence, 1, Index, items),
    arg(1, SentenceTerm, Sentence),
    expect_positive_integer(Sentence, Index, items),
    arg(2, Term, ClauseTerm),
    expect_compound(ClauseTerm, clause, 1, Index, items),
    arg(1, ClauseTerm, Clause),
    expect_positive_integer(Clause, Index, items).

validate_mapping_answer(Answer, Index, Query) :-
    term_name_arity(Answer, Name, Arity),
    ( Name == answer, Arity =:= 3 ->
        validate_mapping_yes_no_answer(Answer, Index, Query)
    ; Name == answer, Arity =:= 4 ->
        validate_mapping_wh_answer(Answer, Index, Query)
    ; throw(registry_reject(shape,
          term(Index, field(expected_answer))))
    ).

validate_mapping_yes_no_answer(Answer, Index, Query) :-
    arg(1, Answer, Query),
    arg(2, Answer, Predicate),
    validate_mapping_ground_predicate(Predicate, Index),
    arg(3, Answer, Outcome),
    ( ( Outcome == proved ; Outcome == not_proved ) ->
        true
    ; throw(registry_reject(shape,
          term(Index, field(expected_answer))))
    ).

validate_mapping_ground_predicate(Term, Index) :-
    expect_compound(Term, pred, 2, Index, expected_answer),
    arg(1, Term, Name),
    expect_nonempty_atom(Name, Index, expected_answer),
    arg(2, Term, Arguments),
    ( is_list(Arguments), Arguments = [_|_] ->
        validate_mapping_named_arguments(Arguments, Index)
    ; throw(registry_reject(shape,
          term(Index, field(expected_answer))))
    ).

validate_mapping_named_arguments([], _).
validate_mapping_named_arguments([Argument|Arguments], Index) :-
    expect_compound(Argument, named, 1, Index, expected_answer),
    arg(1, Argument, Value),
    expect_nonempty_atom(Value, Index, expected_answer),
    validate_mapping_named_arguments(Arguments, Index).

validate_mapping_wh_answer(Answer, Index, Query) :-
    arg(1, Answer, Query),
    arg(2, Answer, Wh),
    expect_compound(Wh, wh, 1, Index, expected_answer),
    arg(1, Wh, Who),
    ( Who == who ->
        true
    ; throw(registry_reject(shape,
          term(Index, field(expected_answer))))
    ),
    arg(3, Answer, Pattern),
    validate_mapping_wh_pattern(Pattern, Index, Name),
    arg(4, Answer, AnswersTerm),
    wrapper_value(AnswersTerm, answers, Index,
        expected_answer, Answers),
    ( is_list(Answers) ->
        true
    ; throw(registry_reject(shape,
          term(Index, field(expected_answer))))
    ),
    validate_mapping_wh_answers(Answers, Name, Index),
    validate_mapping_canonical_order(
        Answers, Index, expected_answer_answers).

validate_mapping_wh_pattern(Term, Index, Name) :-
    expect_compound(Term, pred, 2, Index, expected_answer),
    arg(1, Term, Name),
    expect_nonempty_atom(Name, Index, expected_answer),
    arg(2, Term, Arguments),
    ( is_list(Arguments), Arguments = [Variable] ->
        true
    ; throw(registry_reject(shape,
          term(Index, field(expected_answer))))
    ),
    expect_compound(Variable, var, 1, Index, expected_answer),
    arg(1, Variable, Number),
    ( Number == 1 ->
        true
    ; throw(registry_reject(shape,
          term(Index, field(expected_answer))))
    ).

validate_mapping_wh_answers([], _, _).
validate_mapping_wh_answers([Answer|Answers], Name, Index) :-
    expect_compound(Answer, pred, 2, Index, expected_answer),
    arg(1, Answer, AnswerName),
    expect_nonempty_atom(AnswerName, Index, expected_answer),
    ( AnswerName == Name ->
        true
    ; throw(registry_reject(shape,
          term(Index, field(expected_answer))))
    ),
    arg(2, Answer, Arguments),
    ( is_list(Arguments), Arguments = [Named] ->
        true
    ; throw(registry_reject(shape,
          term(Index, field(expected_answer))))
    ),
    expect_compound(Named, named, 1, Index, expected_answer),
    arg(1, Named, Value),
    expect_nonempty_atom(Value, Index, expected_answer),
    validate_mapping_wh_answers(Answers, Name, Index).

validate_mapping_canonical_order([], _, _).
validate_mapping_canonical_order([_], _, _).
validate_mapping_canonical_order([Left,Right|Rest], Index, Detail) :-
    mapping_canonical_codes(Left, LeftCodes),
    mapping_canonical_codes(Right, RightCodes),
    compare_code_lists(Order, RightCodes, LeftCodes),
    ( Order == (>) ->
        validate_mapping_canonical_order([Right|Rest], Index, Detail)
    ; throw(registry_reject(ordering, term(Index, Detail)))
    ).

mapping_canonical_codes(Term, Codes) :-
    copy_term(Term, Copy),
    canonical_line(Copy, Line),
    string_codes(Line, Codes).

validate_mapping_residual_row(Term, Index,
        mapping_record(residual,
            [GuidelineId, RegionId, ResidualId], Index,
            residual(GuidelineId, RegionId, ResidualId))) :-
    arg(1, Term, GuidelineId),
    expect_stable_id(GuidelineId, Index, guideline_id),
    arg(2, Term, RegionId),
    expect_stable_id(RegionId, Index, region_id),
    arg(3, Term, ResidualId),
    expect_stable_id(ResidualId, Index, residual_id),
    arg(4, Term, ClassTerm),
    wrapper_value(ClassTerm, class, Index, class, Class),
    ( valid_mapping_residual_class(Class) ->
        true
    ; throw(registry_reject(residual,
          term(Index, class(Class))))
    ),
    arg(5, Term, Detail),
    expect_compound(Detail, detail, 2, Index, detail),
    arg(1, Detail, QuoteTerm),
    wrapper_value(QuoteTerm, quote, Index, quote, Quote),
    expect_nonempty_atom(Quote, Index, quote),
    arg(2, Detail, NoteTerm),
    wrapper_value(NoteTerm, note, Index, note, Note),
    expect_nonempty_atom(Note, Index, note).

valid_mapping_residual_class(Class) :-
    member_eq(Class,
        [ copula_head,
          transitive_relation,
          property,
          dose_quantity,
          temporal,
          direction_strength,
          certainty,
          population_threshold,
          labeled_exception,
          multi_entity,
          disjunction,
          scope_deferred
        ]).

require_mapping_region(Records) :-
    ( mapping_has_kind(Records, region) ->
        true
    ; throw(registry_reject(row, required(mapping_region)))
    ).

mapping_has_kind([mapping_record(Kind0, _, _, _)|_], Kind) :-
    Kind0 == Kind,
    !.
mapping_has_kind([_|Records], Kind) :-
    mapping_has_kind(Records, Kind).

validate_mapping_references(Records) :-
    validate_mapping_references_(Records, Records).

validate_mapping_references_([], _).
validate_mapping_references_([Record|Records], All) :-
    validate_mapping_record_references(Record, All),
    validate_mapping_references_(Records, All).

validate_mapping_record_references(
        mapping_record(document, _, _, _), _).
validate_mapping_record_references(
        mapping_record(region, _, _, _), _).
validate_mapping_record_references(
        mapping_record(claim, _, Index,
            claim(GuidelineId, RegionId, _, Docid)), Records) :-
    ( find_mapping_region(Records, GuidelineId, RegionId) ->
        true
    ; throw(registry_reject(reference,
          term(Index, region(GuidelineId, RegionId))))
    ),
    ( find_mapping_document(Records, GuidelineId, Docid) ->
        true
    ; throw(registry_reject(reference,
          term(Index, docid(GuidelineId, Docid))))
    ).
validate_mapping_record_references(
        mapping_record(residual, _, Index,
            residual(GuidelineId, RegionId, _)), Records) :-
    ( find_mapping_region(Records, GuidelineId, RegionId) ->
        true
    ; throw(registry_reject(reference,
          term(Index, region(GuidelineId, RegionId))))
    ).

find_mapping_document(
        [mapping_record(document, _, _, document(GuidelineId0, Docid0))|_],
        GuidelineId, Docid) :-
    GuidelineId0 == GuidelineId,
    Docid0 == Docid,
    !.
find_mapping_document([_|Records], GuidelineId, Docid) :-
    find_mapping_document(Records, GuidelineId, Docid).

find_mapping_region(
        [mapping_record(region, _, _, region(GuidelineId0, RegionId0))|_],
        GuidelineId, RegionId) :-
    GuidelineId0 == GuidelineId,
    RegionId0 == RegionId,
    !.
find_mapping_region([_|Records], GuidelineId, RegionId) :-
    find_mapping_region(Records, GuidelineId, RegionId).

validate_mapping_duplicates(Records) :-
    validate_mapping_duplicate_records(Records,
        [], [], [], [], [], [], []).

validate_mapping_duplicate_records([], _, _, _, _, _, _, _).
validate_mapping_duplicate_records(
        [mapping_record(Kind, Key, Index, Data)|Records],
        DocumentKeySeen, DocidSeen, RegionKeySeen,
        ClaimKeySeen, ClaimIdSeen, ResidualKeySeen, ResidualIdSeen) :-
    ( Kind == document ->
        Data = document(_, Docid),
        check_seen(Key, Index, document_key, DocumentKeySeen),
        check_seen(Docid, Index, docid, DocidSeen),
        NextDocumentKeySeen = [seen(Key, Index)|DocumentKeySeen],
        NextDocidSeen = [seen(Docid, Index)|DocidSeen],
        NextRegionKeySeen = RegionKeySeen,
        NextClaimKeySeen = ClaimKeySeen,
        NextClaimIdSeen = ClaimIdSeen,
        NextResidualKeySeen = ResidualKeySeen,
        NextResidualIdSeen = ResidualIdSeen
    ; Kind == region ->
        check_seen(Key, Index, region_key, RegionKeySeen),
        NextDocumentKeySeen = DocumentKeySeen,
        NextDocidSeen = DocidSeen,
        NextRegionKeySeen = [seen(Key, Index)|RegionKeySeen],
        NextClaimKeySeen = ClaimKeySeen,
        NextClaimIdSeen = ClaimIdSeen,
        NextResidualKeySeen = ResidualKeySeen,
        NextResidualIdSeen = ResidualIdSeen
    ; Kind == claim ->
        Data = claim(_, _, ClaimId, _),
        check_seen(Key, Index, claim_key, ClaimKeySeen),
        check_seen(ClaimId, Index, claim_id, ClaimIdSeen),
        NextDocumentKeySeen = DocumentKeySeen,
        NextDocidSeen = DocidSeen,
        NextRegionKeySeen = RegionKeySeen,
        NextClaimKeySeen = [seen(Key, Index)|ClaimKeySeen],
        NextClaimIdSeen = [seen(ClaimId, Index)|ClaimIdSeen],
        NextResidualKeySeen = ResidualKeySeen,
        NextResidualIdSeen = ResidualIdSeen
    ; Data = residual(_, _, ResidualId),
      check_seen(Key, Index, residual_key, ResidualKeySeen),
      check_seen(ResidualId, Index, residual_id, ResidualIdSeen),
      NextDocumentKeySeen = DocumentKeySeen,
      NextDocidSeen = DocidSeen,
      NextRegionKeySeen = RegionKeySeen,
      NextClaimKeySeen = ClaimKeySeen,
      NextClaimIdSeen = ClaimIdSeen,
      NextResidualKeySeen = [seen(Key, Index)|ResidualKeySeen],
      NextResidualIdSeen = [seen(ResidualId, Index)|ResidualIdSeen]
    ),
    validate_mapping_duplicate_records(Records,
        NextDocumentKeySeen, NextDocidSeen, NextRegionKeySeen,
        NextClaimKeySeen, NextClaimIdSeen,
        NextResidualKeySeen, NextResidualIdSeen).

validate_mapping_ordering([]).
validate_mapping_ordering([Record|Records]) :-
    validate_mapping_ordering_(Records, Record).

validate_mapping_ordering_([], _).
validate_mapping_ordering_([Record|Records], Previous) :-
    mapping_record_rank(Previous, PreviousRank),
    mapping_record_rank(Record, Rank),
    Record = mapping_record(Kind, Key, Index, _),
    Previous = mapping_record(PreviousKind, PreviousKey, PreviousIndex, _),
    ( Rank > PreviousRank ->
        true
    ; Rank < PreviousRank ->
        throw(registry_reject(ordering,
          term(Index, section(Kind, after(PreviousKind,
              previous_term(PreviousIndex))))))
    ; compare_atom_key(Order, Key, PreviousKey),
      ( Order == (>) ->
          true
      ; throw(registry_reject(ordering,
            term(Index, key(Kind, Key,
                after(PreviousKey, previous_term(PreviousIndex))))))
      )
    ),
    validate_mapping_ordering_(Records, Record).

mapping_record_rank(mapping_record(Kind, _, _, _), Rank) :-
    ( Kind == document ->
        Rank = 1
    ; Kind == region ->
        Rank = 2
    ; Kind == claim ->
        Rank = 3
    ; Rank = 4
    ).

validate_mapping_coverage(Records) :-
    validate_mapping_region_coverage(Records, Records),
    validate_mapping_document_coverage(Records, Records).

validate_mapping_region_coverage([], _).
validate_mapping_region_coverage(
        [mapping_record(Kind, _, Index, Data)|Records], All) :-
    ( Kind == region ->
        Data = region(GuidelineId, RegionId),
        ( mapping_has_claim(All, GuidelineId, RegionId) ->
            ( mapping_has_residual(All, GuidelineId, RegionId) ->
                true
            ; throw(registry_reject(coverage,
                  term(Index, claim_without_residual(
                      GuidelineId, RegionId))))
            )
        ; mapping_has_residual(All, GuidelineId, RegionId) ->
            true
        ; throw(registry_reject(coverage,
              term(Index, region_uncovered(GuidelineId, RegionId))))
        )
    ; true
    ),
    validate_mapping_region_coverage(Records, All).

validate_mapping_document_coverage([], _).
validate_mapping_document_coverage(
        [mapping_record(Kind, _, Index, Data)|Records], All) :-
    ( Kind == document ->
        Data = document(GuidelineId, Docid),
        ( mapping_document_has_claim(All, GuidelineId, Docid) ->
            true
        ; throw(registry_reject(coverage,
              term(Index, document_unreferenced(Docid))))
        )
    ; true
    ),
    validate_mapping_document_coverage(Records, All).

mapping_has_claim(
        [mapping_record(claim, _, _,
            claim(GuidelineId0, RegionId0, _, _))|_],
        GuidelineId, RegionId) :-
    GuidelineId0 == GuidelineId,
    RegionId0 == RegionId,
    !.
mapping_has_claim([_|Records], GuidelineId, RegionId) :-
    mapping_has_claim(Records, GuidelineId, RegionId).

mapping_has_residual(
        [mapping_record(residual, _, _,
            residual(GuidelineId0, RegionId0, _))|_],
        GuidelineId, RegionId) :-
    GuidelineId0 == GuidelineId,
    RegionId0 == RegionId,
    !.
mapping_has_residual([_|Records], GuidelineId, RegionId) :-
    mapping_has_residual(Records, GuidelineId, RegionId).

mapping_document_has_claim(
        [mapping_record(claim, _, _,
            claim(GuidelineId0, _, _, Docid0))|_],
        GuidelineId, Docid) :-
    GuidelineId0 == GuidelineId,
    Docid0 == Docid,
    !.
mapping_document_has_claim([_|Records], GuidelineId, Docid) :-
    mapping_document_has_claim(Records, GuidelineId, Docid).


validate_terminology(Terms, Entries) :-
    require_version(Terms, cnl_guideline_terminology(1), Rows),
    validate_terminology_rows(Rows, 2, Entries),
    require_terminology_entry(Entries),
    validate_terminology_duplicates(Entries),
    validate_terminology_ordering(Entries).

validate_terminology_rows([], _, []).
validate_terminology_rows([Term|Terms], Index,
        [Entry|Entries]) :-
    terminology_row_shape(Term, Index),
    validate_terminology_row(Term, Index, Entry),
    Next is Index + 1,
    validate_terminology_rows(Terms, Next, Entries).

terminology_row_shape(Term, Index) :-
    term_name_arity(Term, Name, Arity),
    ( Name == terminology_entry ->
        require_row_arity(Name, Arity, 3, Index)
    ; throw(registry_reject(row,
          term(Index, unknown_constructor(Name, Arity))))
    ).

validate_terminology_row(Term, Index,
        entry(Index, EntryId, Kind, WordForm, Template)) :-
    arg(1, Term, EntryId),
    expect_stable_id(EntryId, Index, entry_id),
    arg(2, Term, Template),
    validate_template(Template, Index, Kind, WordForm),
    arg(3, Term, SurfaceTerm),
    wrapper_value(SurfaceTerm, english_surface, Index,
        english_surface, Surface),
    expect_nonempty_atom(Surface, Index, english_surface).

validate_template(Template, Index, Kind, WordForm) :-
    ( compound(Template) ->
        functor(Template, Kind, Arity)
    ; term_name_arity(Template, Kind, Arity)
    ),
    template_signature(Kind, ExpectedArity, GenderPosition),
    ( Arity =:= ExpectedArity ->
        true
    ; throw(registry_reject(template,
          term(Index, kind(Kind,
              arity(expected(ExpectedArity), found(Arity))))))
    ),
    validate_template_atoms(Template, 1, Arity, Index, Kind),
    arg(1, Template, WordForm),
    validate_template_gender(Template, GenderPosition, Index, Kind).

template_signature(Kind, Arity, GenderPosition) :-
    ( Kind == adv -> Arity = 2, GenderPosition = none
    ; Kind == adv_comp -> Arity = 2, GenderPosition = none
    ; Kind == adv_sup -> Arity = 2, GenderPosition = none
    ; Kind == adj_itr -> Arity = 2, GenderPosition = none
    ; Kind == adj_itr_comp -> Arity = 2, GenderPosition = none
    ; Kind == adj_itr_sup -> Arity = 2, GenderPosition = none
    ; Kind == adj_tr -> Arity = 3, GenderPosition = none
    ; Kind == adj_tr_comp -> Arity = 3, GenderPosition = none
    ; Kind == adj_tr_sup -> Arity = 3, GenderPosition = none
    ; Kind == noun_sg -> Arity = 3, GenderPosition = 3
    ; Kind == noun_pl -> Arity = 3, GenderPosition = 3
    ; Kind == noun_mass -> Arity = 3, GenderPosition = 3
    ; Kind == mn_sg -> Arity = 2, GenderPosition = none
    ; Kind == mn_pl -> Arity = 2, GenderPosition = none
    ; Kind == pn_sg -> Arity = 3, GenderPosition = 3
    ; Kind == pn_pl -> Arity = 3, GenderPosition = 3
    ; Kind == pndef_sg -> Arity = 3, GenderPosition = 3
    ; Kind == pndef_pl -> Arity = 3, GenderPosition = 3
    ; Kind == iv_finsg -> Arity = 2, GenderPosition = none
    ; Kind == iv_infpl -> Arity = 2, GenderPosition = none
    ; Kind == tv_finsg -> Arity = 2, GenderPosition = none
    ; Kind == tv_infpl -> Arity = 2, GenderPosition = none
    ; Kind == tv_pp -> Arity = 2, GenderPosition = none
    ; Kind == dv_finsg -> Arity = 3, GenderPosition = none
    ; Kind == dv_infpl -> Arity = 3, GenderPosition = none
    ; Kind == dv_pp -> Arity = 3, GenderPosition = none
    ; Kind == prep -> Arity = 2, GenderPosition = none
    ; throw(registry_reject(template, kind(Kind)))
    ).

validate_template_atoms(_, Position, Arity, _, _) :-
    Position > Arity,
    !.
validate_template_atoms(Template, Position, Arity, Index, Kind) :-
    arg(Position, Template, Value),
    ( atom(Value), Value \== '' ->
        true
    ; throw(registry_reject(shape,
          term(Index, template_field(Kind, Position))))
    ),
    Next is Position + 1,
    validate_template_atoms(Template, Next, Arity, Index, Kind).

validate_template_gender(_, GenderPosition, _, _) :-
    GenderPosition == none,
    !.
validate_template_gender(Template, GenderPosition, Index, Kind) :-
    arg(GenderPosition, Template, Gender),
    ( valid_gender(Gender) ->
        true
    ; throw(registry_reject(gender,
          term(Index, kind(Kind, value(Gender)))))
    ).

valid_gender(Gender) :-
    Gender == undef
    ; Gender == neutr
    ; Gender == human
    ; Gender == masc
    ; Gender == fem.

require_terminology_entry([]) :-
    throw(registry_reject(row, required(terminology_entry))).
require_terminology_entry([_|_]).

validate_terminology_duplicates(Entries) :-
    validate_entry_duplicates(Entries, [], []).

validate_entry_duplicates([], _, _).
validate_entry_duplicates([entry(Index, EntryId, Kind, WordForm, _)|Entries],
        IdSeen, KeySeen) :-
    check_seen(EntryId, Index, entry_id, IdSeen),
    Key = [Kind, WordForm],
    check_seen(Key, Index, terminology_key, KeySeen),
    validate_entry_duplicates(Entries,
        [seen(EntryId, Index)|IdSeen], [seen(Key, Index)|KeySeen]).

validate_terminology_ordering([]).
validate_terminology_ordering([Entry|Entries]) :-
    validate_terminology_ordering_(Entries, Entry).

validate_terminology_ordering_([], _).
validate_terminology_ordering_(
        [Entry|Entries], Previous) :-
    Entry = entry(Index, _, Kind, WordForm, _),
    Previous = entry(PreviousIndex, _, PreviousKind, PreviousWordForm, _),
    Key = [Kind, WordForm],
    PreviousKey = [PreviousKind, PreviousWordForm],
    compare_atom_key(Order, Key, PreviousKey),
    ( Order == (>) ->
        true
    ; throw(registry_reject(ordering,
          term(Index, terminology_key(Kind, WordForm,
              after(PreviousKind, PreviousWordForm,
                  previous_term(PreviousIndex))))))
    ),
    validate_terminology_ordering_(Entries, Entry).

expect_compound(Term, ExpectedName, ExpectedArity, Index, Field) :-
    ( compound(Term) ->
        functor(Term, Name, Arity),
        ( Name == ExpectedName, Arity =:= ExpectedArity ->
            true
        ; throw(registry_reject(shape, term(Index, field(Field))))
        )
    ; throw(registry_reject(shape, term(Index, field(Field))))
    ).

wrapper_value(Term, Name, Index, Field, Value) :-
    expect_compound(Term, Name, 1, Index, Field),
    arg(1, Term, Value).

expect_nonempty_atom(Value, Index, Field) :-
    ( atom(Value), Value \== '' ->
        true
    ; throw(registry_reject(shape, term(Index, field(Field))))
    ).

expect_stable_id(Value, Index, Field) :-
    ( valid_stable_id(Value) ->
        true
    ; throw(registry_reject(shape, term(Index, field(Field))))
    ).

valid_stable_id(Value) :-
    atom(Value),
    atom_codes(Value, [First|Rest]),
    ascii_lower(First),
    valid_id_rest(Rest, false).

valid_id_rest([], LastSeparator) :-
    LastSeparator == false.
valid_id_rest([Code|Codes], _) :-
    ascii_lower_or_digit(Code),
    !,
    valid_id_rest(Codes, false).
valid_id_rest([Code|Codes], LastSeparator) :-
    ( Code =:= 0'- ; Code =:= 0'. ),
    LastSeparator == false,
    Codes = [_|_],
    valid_id_rest(Codes, true).

ascii_lower(Code) :-
    Code >= 0'a,
    Code =< 0'z.

ascii_lower_or_digit(Code) :-
    ascii_lower(Code)
    ; Code >= 0'0, Code =< 0'9.

expect_relpath(Value, Index, Field) :-
    ( valid_relpath(Value) ->
        true
    ; throw(registry_reject(shape, term(Index, field(Field))))
    ).

valid_relpath(Value) :-
    atom(Value),
    Value \== '',
    atom_codes(Value, [First|_]),
    First =\= 0'/,
    atomic_list_concat(Segments, '/', Value),
    Segments = [_|_],
    valid_path_segments(Segments).

valid_path_segments([]).
valid_path_segments([Segment|Segments]) :-
    Segment \== '',
    Segment \== '.',
    Segment \== '..',
    valid_path_segments(Segments).

expect_positive_integer(Value, Index, Field) :-
    ( integer(Value), Value > 0 ->
        true
    ; throw(registry_reject(shape, term(Index, field(Field))))
    ).

expect_minimum_integer(Value, Minimum, Index, Field) :-
    ( integer(Value), Value >= Minimum ->
        true
    ; throw(registry_reject(shape, term(Index, field(Field))))
    ).

expect_boolean(Value, _Index, _Field) :-
    ( Value == true ; Value == false ),
    !.
expect_boolean(_, Index, Field) :-
    throw(registry_reject(shape, term(Index, field(Field)))).

expect_enum(Value, Allowed, Index, Field) :-
    ( member_eq(Value, Allowed) ->
        true
    ; throw(registry_reject(shape, term(Index, field(Field))))
    ).

member_eq(Value, [Allowed|_]) :-
    Value == Allowed,
    !.
member_eq(Value, [_|Allowed]) :-
    member_eq(Value, Allowed).

validate_digest(Hash, Index, Field) :-
    ( atom(Hash) ->
        atom_codes(Hash, Codes),
        length(Codes, Length),
        ( Length =:= 64 ->
            ( lower_hex_codes(Codes) ->
                true
            ; throw(registry_reject(digest,
                  term(Index, digest(Field, lower_hex))))
            )
        ; throw(registry_reject(digest,
              term(Index, digest(Field, length(expected(64), found(Length))))))
        )
    ; throw(registry_reject(shape, term(Index, field(Field))))
    ).

lower_hex_codes([]).
lower_hex_codes([Code|Codes]) :-
    ( Code >= 0'0, Code =< 0'9
    ; Code >= 0'a, Code =< 0'f
    ),
    lower_hex_codes(Codes).

validate_strict_atom_order([], _, _).
validate_strict_atom_order([_], _, _).
validate_strict_atom_order([Left,Right|Rest], Index, Field) :-
    compare_atoms_codepoint(Order, Right, Left),
    ( Order == (>) ->
        validate_strict_atom_order([Right|Rest], Index, Field)
    ; throw(registry_reject(shape, term(Index, field(Field))))
    ).

compare_atom_key(=, [], []).
compare_atom_key(Order, [Left|Lefts], [Right|Rights]) :-
    compare_atoms_codepoint(HeadOrder, Left, Right),
    ( HeadOrder == (=) ->
        compare_atom_key(Order, Lefts, Rights)
    ; Order = HeadOrder
    ).

compare_atoms_codepoint(Order, Left, Right) :-
    atom_codes(Left, LeftCodes),
    atom_codes(Right, RightCodes),
    compare_code_lists(Order, LeftCodes, RightCodes).

compare_code_lists(=, [], []).
compare_code_lists(<, [], [_|_]).
compare_code_lists(>, [_|_], []).
compare_code_lists(Order, [Left|Lefts], [Right|Rights]) :-
    ( Left < Right ->
        Order = (<)
    ; Left > Right ->
        Order = (>)
    ; compare_code_lists(Order, Lefts, Rights)
    ).

handle_error(_, ErrorStream,
        registry_tool_failure(Stage, Class, Detail, Status)) :-
    !,
    emit_error(ErrorStream, Stage, Class, Detail, Status).
handle_error(Stage, ErrorStream, registry_reject(Class, Detail)) :-
    !,
    emit_error(ErrorStream, Stage, Class, Detail, 1).
handle_error(Stage, ErrorStream, Error) :-
    emit_error(ErrorStream, Stage, uncaught, Error, 2).

emit_error(ErrorStream, Stage, Class, Detail, Status) :-
    ( catch(canonical_line(
              registry_tool_error(Stage, Class, Detail), Line), _, fail) ->
        true
    ; fallback_error_line(Stage, Class, Line)
    ),
    format(ErrorStream, '~s', [Line]),
    flush_output(ErrorStream),
    halt(Status).

fallback_error_line(Stage, Class, Line) :-
    ( catch(canonical_line(
              registry_tool_error(Stage, Class, unserializable), Line), _, fail) ->
        true
    ; Line = "registry_tool_error(cli,uncaught,unserializable).\n"
    ).

use vstd::prelude::*;
use vstd::slice::slice_to_vec;

verus! {

pub enum Sym {
    DollarGuideline,
    DollarGuidelineId,
    DollarGuidelineProof,
    Comma,
    Minus,
    Slash,
    Implies,
    Cons,
    AceSha256,
    Actual,
    Answer,
    ApeMessages,
    Box,
    BundleCount,
    Can,
    Clauses,
    ConditionOutsideSentenceRange,
    ConditionShape,
    Context,
    DeferredOperator,
    Disjunction,
    DisjunctiveAntecedent,
    Docid,
    Drs,
    DumpNoncanonical,
    Eq,
    Exactly,
    Geq,
    Greater,
    Guideline,
    GuidelineArg,
    GuidelineCardinality,
    GuidelineEntity,
    GuidelineEvent,
    GuidelineOperator,
    GuidelinePp,
    GuidelineProperty,
    HeadVariableNotBoundInBody,
    InvalidDrsShape,
    Leq,
    Less,
    May,
    Messages,
    MixedOrMissingSentenceAnchors,
    ModifierPp,
    Must,
    Na,
    NafPlacement,
    NafSafety,
    NafShape,
    Noncanonical,
    NongroundObligation,
    Noun,
    Object,
    ObjectOperator,
    OperatorScopedRule,
    Pos,
    Predicate,
    Product,
    Projection,
    Property,
    PropertyPolarity,
    Qid,
    Query,
    QueryMarker,
    QueryMarkerUnbound,
    QueryRoot,
    QuerySentences,
    QueryText,
    QueryUnsupported,
    Question,
    RecordShape,
    Ref,
    ReservedNameCollision,
    RootCondition,
    RuleWithoutAntecedent,
    RuleWithoutConsequent,
    Sentence,
    SentenceLines,
    SentenceShape,
    Sentences,
    Should,
    Ulex,
    UnresolvedArgument,
    Unsupported,
    V,
    Variant,
    Wh,
    What,
    Which,
    Who,
    Witness,
    Naf,
}

pub open spec fn symbol(s: &Sym) -> Seq<u8> {
    match s {
        Sym::DollarGuideline => ckc_spec::v1text::ascii("$guideline_"@),
        Sym::DollarGuidelineId => ckc_spec::v1text::ascii("$guideline_id"@),
        Sym::DollarGuidelineProof => ckc_spec::v1text::ascii("$guideline_proof"@),
        Sym::Comma => ckc_spec::v1text::ascii(","@),
        Sym::Minus => ckc_spec::v1text::ascii("-"@),
        Sym::Slash => ckc_spec::v1text::ascii("/"@),
        Sym::Implies => ckc_spec::v1text::ascii("=>"@),
        Sym::Cons => ckc_spec::v1text::ascii("[|]"@),
        Sym::AceSha256 => ckc_spec::v1text::ascii("ace_sha256"@),
        Sym::Actual => ckc_spec::v1text::ascii("actual"@),
        Sym::Answer => ckc_spec::v1text::ascii("answer"@),
        Sym::ApeMessages => ckc_spec::v1text::ascii("ape_messages"@),
        Sym::Box => ckc_spec::v1text::ascii("box"@),
        Sym::BundleCount => ckc_spec::v1text::ascii("bundle_count"@),
        Sym::Can => ckc_spec::v1text::ascii("can"@),
        Sym::Clauses => ckc_spec::v1text::ascii("clauses"@),
        Sym::ConditionOutsideSentenceRange => ckc_spec::v1text::ascii(
            "condition_outside_sentence_range"@,
        ),
        Sym::ConditionShape => ckc_spec::v1text::ascii("condition_shape"@),
        Sym::Context => ckc_spec::v1text::ascii("context"@),
        Sym::DeferredOperator => ckc_spec::v1text::ascii("deferred_operator"@),
        Sym::Disjunction => ckc_spec::v1text::ascii("disjunction"@),
        Sym::DisjunctiveAntecedent => ckc_spec::v1text::ascii("disjunctive_antecedent"@),
        Sym::Docid => ckc_spec::v1text::ascii("docid"@),
        Sym::Drs => ckc_spec::v1text::ascii("drs"@),
        Sym::DumpNoncanonical => ckc_spec::v1text::ascii("dump_noncanonical"@),
        Sym::Eq => ckc_spec::v1text::ascii("eq"@),
        Sym::Exactly => ckc_spec::v1text::ascii("exactly"@),
        Sym::Geq => ckc_spec::v1text::ascii("geq"@),
        Sym::Greater => ckc_spec::v1text::ascii("greater"@),
        Sym::Guideline => ckc_spec::v1text::ascii("guideline_"@),
        Sym::GuidelineArg => ckc_spec::v1text::ascii("guideline_arg"@),
        Sym::GuidelineCardinality => ckc_spec::v1text::ascii("guideline_cardinality"@),
        Sym::GuidelineEntity => ckc_spec::v1text::ascii("guideline_entity"@),
        Sym::GuidelineEvent => ckc_spec::v1text::ascii("guideline_event"@),
        Sym::GuidelineOperator => ckc_spec::v1text::ascii("guideline_operator"@),
        Sym::GuidelinePp => ckc_spec::v1text::ascii("guideline_pp"@),
        Sym::GuidelineProperty => ckc_spec::v1text::ascii("guideline_property"@),
        Sym::HeadVariableNotBoundInBody => ckc_spec::v1text::ascii(
            "head_variable_not_bound_in_body"@,
        ),
        Sym::InvalidDrsShape => ckc_spec::v1text::ascii("invalid_drs_shape"@),
        Sym::Leq => ckc_spec::v1text::ascii("leq"@),
        Sym::Less => ckc_spec::v1text::ascii("less"@),
        Sym::May => ckc_spec::v1text::ascii("may"@),
        Sym::Messages => ckc_spec::v1text::ascii("messages"@),
        Sym::MixedOrMissingSentenceAnchors => ckc_spec::v1text::ascii(
            "mixed_or_missing_sentence_anchors"@,
        ),
        Sym::ModifierPp => ckc_spec::v1text::ascii("modifier_pp"@),
        Sym::Must => ckc_spec::v1text::ascii("must"@),
        Sym::Na => ckc_spec::v1text::ascii("na"@),
        Sym::NafPlacement => ckc_spec::v1text::ascii("naf_placement"@),
        Sym::NafSafety => ckc_spec::v1text::ascii("naf_safety"@),
        Sym::NafShape => ckc_spec::v1text::ascii("naf_shape"@),
        Sym::Noncanonical => ckc_spec::v1text::ascii("noncanonical"@),
        Sym::NongroundObligation => ckc_spec::v1text::ascii("nonground_obligation"@),
        Sym::Noun => ckc_spec::v1text::ascii("noun"@),
        Sym::Object => ckc_spec::v1text::ascii("object"@),
        Sym::ObjectOperator => ckc_spec::v1text::ascii("object_operator"@),
        Sym::OperatorScopedRule => ckc_spec::v1text::ascii("operator_scoped_rule"@),
        Sym::Pos => ckc_spec::v1text::ascii("pos"@),
        Sym::Predicate => ckc_spec::v1text::ascii("predicate"@),
        Sym::Product => ckc_spec::v1text::ascii("product"@),
        Sym::Projection => ckc_spec::v1text::ascii("projection"@),
        Sym::Property => ckc_spec::v1text::ascii("property"@),
        Sym::PropertyPolarity => ckc_spec::v1text::ascii("property_polarity"@),
        Sym::Qid => ckc_spec::v1text::ascii("qid"@),
        Sym::Query => ckc_spec::v1text::ascii("query"@),
        Sym::QueryMarker => ckc_spec::v1text::ascii("query_marker"@),
        Sym::QueryMarkerUnbound => ckc_spec::v1text::ascii("query_marker_unbound"@),
        Sym::QueryRoot => ckc_spec::v1text::ascii("query_root"@),
        Sym::QuerySentences => ckc_spec::v1text::ascii("query_sentences"@),
        Sym::QueryText => ckc_spec::v1text::ascii("query_text"@),
        Sym::QueryUnsupported => ckc_spec::v1text::ascii("query_unsupported"@),
        Sym::Question => ckc_spec::v1text::ascii("question"@),
        Sym::RecordShape => ckc_spec::v1text::ascii("record_shape"@),
        Sym::Ref => ckc_spec::v1text::ascii("ref"@),
        Sym::ReservedNameCollision => ckc_spec::v1text::ascii("reserved_name_collision"@),
        Sym::RootCondition => ckc_spec::v1text::ascii("root_condition"@),
        Sym::RuleWithoutAntecedent => ckc_spec::v1text::ascii("rule_without_antecedent"@),
        Sym::RuleWithoutConsequent => ckc_spec::v1text::ascii("rule_without_consequent"@),
        Sym::Sentence => ckc_spec::v1text::ascii("sentence"@),
        Sym::SentenceLines => ckc_spec::v1text::ascii("sentence_lines"@),
        Sym::SentenceShape => ckc_spec::v1text::ascii("sentence_shape"@),
        Sym::Sentences => ckc_spec::v1text::ascii("sentences"@),
        Sym::Should => ckc_spec::v1text::ascii("should"@),
        Sym::Ulex => ckc_spec::v1text::ascii("ulex"@),
        Sym::UnresolvedArgument => ckc_spec::v1text::ascii("unresolved_argument"@),
        Sym::Unsupported => ckc_spec::v1text::ascii("unsupported"@),
        Sym::V => ckc_spec::v1text::ascii("v"@),
        Sym::Variant => ckc_spec::v1text::ascii("variant"@),
        Sym::Wh => ckc_spec::v1text::ascii("wh"@),
        Sym::What => ckc_spec::v1text::ascii("what"@),
        Sym::Which => ckc_spec::v1text::ascii("which"@),
        Sym::Who => ckc_spec::v1text::ascii("who"@),
        Sym::Witness => ckc_spec::v1text::ascii("witness"@),
        Sym::Naf => ckc_spec::v1text::ascii("~"@),
    }
}

pub fn symbol_bytes(s: &Sym) -> (out: Vec<u8>)
    ensures
        out@ == symbol(s),
{
    match s {
        Sym::DollarGuideline => bytes_dollarguideline(),
        Sym::DollarGuidelineId => bytes_dollarguidelineid(),
        Sym::DollarGuidelineProof => bytes_dollarguidelineproof(),
        Sym::Comma => bytes_comma(),
        Sym::Minus => bytes_minus(),
        Sym::Slash => bytes_slash(),
        Sym::Implies => bytes_implies(),
        Sym::Cons => bytes_cons(),
        Sym::AceSha256 => bytes_acesha256(),
        Sym::Actual => bytes_actual(),
        Sym::Answer => bytes_answer(),
        Sym::ApeMessages => bytes_apemessages(),
        Sym::Box => bytes_box(),
        Sym::BundleCount => bytes_bundlecount(),
        Sym::Can => bytes_can(),
        Sym::Clauses => bytes_clauses(),
        Sym::ConditionOutsideSentenceRange => bytes_conditionoutsidesentencerange(),
        Sym::ConditionShape => bytes_conditionshape(),
        Sym::Context => bytes_context(),
        Sym::DeferredOperator => bytes_deferredoperator(),
        Sym::Disjunction => bytes_disjunction(),
        Sym::DisjunctiveAntecedent => bytes_disjunctiveantecedent(),
        Sym::Docid => bytes_docid(),
        Sym::Drs => bytes_drs(),
        Sym::DumpNoncanonical => bytes_dumpnoncanonical(),
        Sym::Eq => bytes_eq(),
        Sym::Exactly => bytes_exactly(),
        Sym::Geq => bytes_geq(),
        Sym::Greater => bytes_greater(),
        Sym::Guideline => bytes_guideline(),
        Sym::GuidelineArg => bytes_guidelinearg(),
        Sym::GuidelineCardinality => bytes_guidelinecardinality(),
        Sym::GuidelineEntity => bytes_guidelineentity(),
        Sym::GuidelineEvent => bytes_guidelineevent(),
        Sym::GuidelineOperator => bytes_guidelineoperator(),
        Sym::GuidelinePp => bytes_guidelinepp(),
        Sym::GuidelineProperty => bytes_guidelineproperty(),
        Sym::HeadVariableNotBoundInBody => bytes_headvariablenotboundinbody(),
        Sym::InvalidDrsShape => bytes_invaliddrsshape(),
        Sym::Leq => bytes_leq(),
        Sym::Less => bytes_less(),
        Sym::May => bytes_may(),
        Sym::Messages => bytes_messages(),
        Sym::MixedOrMissingSentenceAnchors => bytes_mixedormissingsentenceanchors(),
        Sym::ModifierPp => bytes_modifierpp(),
        Sym::Must => bytes_must(),
        Sym::Na => bytes_na(),
        Sym::NafPlacement => bytes_nafplacement(),
        Sym::NafSafety => bytes_nafsafety(),
        Sym::NafShape => bytes_nafshape(),
        Sym::Noncanonical => bytes_noncanonical(),
        Sym::NongroundObligation => bytes_nongroundobligation(),
        Sym::Noun => bytes_noun(),
        Sym::Object => bytes_object(),
        Sym::ObjectOperator => bytes_objectoperator(),
        Sym::OperatorScopedRule => bytes_operatorscopedrule(),
        Sym::Pos => bytes_pos(),
        Sym::Predicate => bytes_predicate(),
        Sym::Product => bytes_product(),
        Sym::Projection => bytes_projection(),
        Sym::Property => bytes_property(),
        Sym::PropertyPolarity => bytes_propertypolarity(),
        Sym::Qid => bytes_qid(),
        Sym::Query => bytes_query(),
        Sym::QueryMarker => bytes_querymarker(),
        Sym::QueryMarkerUnbound => bytes_querymarkerunbound(),
        Sym::QueryRoot => bytes_queryroot(),
        Sym::QuerySentences => bytes_querysentences(),
        Sym::QueryText => bytes_querytext(),
        Sym::QueryUnsupported => bytes_queryunsupported(),
        Sym::Question => bytes_question(),
        Sym::RecordShape => bytes_recordshape(),
        Sym::Ref => bytes_ref(),
        Sym::ReservedNameCollision => bytes_reservednamecollision(),
        Sym::RootCondition => bytes_rootcondition(),
        Sym::RuleWithoutAntecedent => bytes_rulewithoutantecedent(),
        Sym::RuleWithoutConsequent => bytes_rulewithoutconsequent(),
        Sym::Sentence => bytes_sentence(),
        Sym::SentenceLines => bytes_sentencelines(),
        Sym::SentenceShape => bytes_sentenceshape(),
        Sym::Sentences => bytes_sentences(),
        Sym::Should => bytes_should(),
        Sym::Ulex => bytes_ulex(),
        Sym::UnresolvedArgument => bytes_unresolvedargument(),
        Sym::Unsupported => bytes_unsupported(),
        Sym::V => bytes_v(),
        Sym::Variant => bytes_variant(),
        Sym::Wh => bytes_wh(),
        Sym::What => bytes_what(),
        Sym::Which => bytes_which(),
        Sym::Who => bytes_who(),
        Sym::Witness => bytes_witness(),
        Sym::Naf => bytes_naf(),
    }
}

fn bytes_dollarguideline() -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::v1text::ascii("$guideline_"@),
{
    let b: &[u8] = b"$guideline_";
    proof {
        reveal_byteslit(b"$guideline_");
        reveal_strlit("$guideline_");
        reveal(ckc_spec::v1text::ascii);
        vstd::assert_seqs_equal!(b@ == ckc_spec::v1text::ascii("$guideline_"@));
    }
    slice_to_vec(b)
}

fn bytes_dollarguidelineid() -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::v1text::ascii("$guideline_id"@),
{
    let b: &[u8] = b"$guideline_id";
    proof {
        reveal_byteslit(b"$guideline_id");
        reveal_strlit("$guideline_id");
        reveal(ckc_spec::v1text::ascii);
        vstd::assert_seqs_equal!(b@ == ckc_spec::v1text::ascii("$guideline_id"@));
    }
    slice_to_vec(b)
}

fn bytes_dollarguidelineproof() -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::v1text::ascii("$guideline_proof"@),
{
    let b: &[u8] = b"$guideline_proof";
    proof {
        reveal_byteslit(b"$guideline_proof");
        reveal_strlit("$guideline_proof");
        reveal(ckc_spec::v1text::ascii);
        vstd::assert_seqs_equal!(b@ == ckc_spec::v1text::ascii("$guideline_proof"@));
    }
    slice_to_vec(b)
}

fn bytes_comma() -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::v1text::ascii(","@),
{
    let b: &[u8] = b",";
    proof {
        reveal_byteslit(b",");
        reveal_strlit(",");
        reveal(ckc_spec::v1text::ascii);
        vstd::assert_seqs_equal!(b@ == ckc_spec::v1text::ascii(","@));
    }
    slice_to_vec(b)
}

fn bytes_minus() -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::v1text::ascii("-"@),
{
    let b: &[u8] = b"-";
    proof {
        reveal_byteslit(b"-");
        reveal_strlit("-");
        reveal(ckc_spec::v1text::ascii);
        vstd::assert_seqs_equal!(b@ == ckc_spec::v1text::ascii("-"@));
    }
    slice_to_vec(b)
}

fn bytes_slash() -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::v1text::ascii("/"@),
{
    let b: &[u8] = b"/";
    proof {
        reveal_byteslit(b"/");
        reveal_strlit("/");
        reveal(ckc_spec::v1text::ascii);
        vstd::assert_seqs_equal!(b@ == ckc_spec::v1text::ascii("/"@));
    }
    slice_to_vec(b)
}

fn bytes_implies() -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::v1text::ascii("=>"@),
{
    let b: &[u8] = b"=>";
    proof {
        reveal_byteslit(b"=>");
        reveal_strlit("=>");
        reveal(ckc_spec::v1text::ascii);
        vstd::assert_seqs_equal!(b@ == ckc_spec::v1text::ascii("=>"@));
    }
    slice_to_vec(b)
}

fn bytes_cons() -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::v1text::ascii("[|]"@),
{
    let b: &[u8] = b"[|]";
    proof {
        reveal_byteslit(b"[|]");
        reveal_strlit("[|]");
        reveal(ckc_spec::v1text::ascii);
        vstd::assert_seqs_equal!(b@ == ckc_spec::v1text::ascii("[|]"@));
    }
    slice_to_vec(b)
}

fn bytes_acesha256() -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::v1text::ascii("ace_sha256"@),
{
    let b: &[u8] = b"ace_sha256";
    proof {
        reveal_byteslit(b"ace_sha256");
        reveal_strlit("ace_sha256");
        reveal(ckc_spec::v1text::ascii);
        vstd::assert_seqs_equal!(b@ == ckc_spec::v1text::ascii("ace_sha256"@));
    }
    slice_to_vec(b)
}

fn bytes_actual() -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::v1text::ascii("actual"@),
{
    let b: &[u8] = b"actual";
    proof {
        reveal_byteslit(b"actual");
        reveal_strlit("actual");
        reveal(ckc_spec::v1text::ascii);
        vstd::assert_seqs_equal!(b@ == ckc_spec::v1text::ascii("actual"@));
    }
    slice_to_vec(b)
}

fn bytes_answer() -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::v1text::ascii("answer"@),
{
    let b: &[u8] = b"answer";
    proof {
        reveal_byteslit(b"answer");
        reveal_strlit("answer");
        reveal(ckc_spec::v1text::ascii);
        vstd::assert_seqs_equal!(b@ == ckc_spec::v1text::ascii("answer"@));
    }
    slice_to_vec(b)
}

fn bytes_apemessages() -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::v1text::ascii("ape_messages"@),
{
    let b: &[u8] = b"ape_messages";
    proof {
        reveal_byteslit(b"ape_messages");
        reveal_strlit("ape_messages");
        reveal(ckc_spec::v1text::ascii);
        vstd::assert_seqs_equal!(b@ == ckc_spec::v1text::ascii("ape_messages"@));
    }
    slice_to_vec(b)
}

fn bytes_box() -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::v1text::ascii("box"@),
{
    let b: &[u8] = b"box";
    proof {
        reveal_byteslit(b"box");
        reveal_strlit("box");
        reveal(ckc_spec::v1text::ascii);
        vstd::assert_seqs_equal!(b@ == ckc_spec::v1text::ascii("box"@));
    }
    slice_to_vec(b)
}

fn bytes_bundlecount() -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::v1text::ascii("bundle_count"@),
{
    let b: &[u8] = b"bundle_count";
    proof {
        reveal_byteslit(b"bundle_count");
        reveal_strlit("bundle_count");
        reveal(ckc_spec::v1text::ascii);
        vstd::assert_seqs_equal!(b@ == ckc_spec::v1text::ascii("bundle_count"@));
    }
    slice_to_vec(b)
}

fn bytes_can() -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::v1text::ascii("can"@),
{
    let b: &[u8] = b"can";
    proof {
        reveal_byteslit(b"can");
        reveal_strlit("can");
        reveal(ckc_spec::v1text::ascii);
        vstd::assert_seqs_equal!(b@ == ckc_spec::v1text::ascii("can"@));
    }
    slice_to_vec(b)
}

fn bytes_clauses() -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::v1text::ascii("clauses"@),
{
    let b: &[u8] = b"clauses";
    proof {
        reveal_byteslit(b"clauses");
        reveal_strlit("clauses");
        reveal(ckc_spec::v1text::ascii);
        vstd::assert_seqs_equal!(b@ == ckc_spec::v1text::ascii("clauses"@));
    }
    slice_to_vec(b)
}

fn bytes_conditionoutsidesentencerange() -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::v1text::ascii("condition_outside_sentence_range"@),
{
    let b: &[u8] = b"condition_outside_sentence_range";
    proof {
        reveal_byteslit(b"condition_outside_sentence_range");
        reveal_strlit("condition_outside_sentence_range");
        reveal(ckc_spec::v1text::ascii);
        vstd::assert_seqs_equal!(b@ == ckc_spec::v1text::ascii("condition_outside_sentence_range"@));
    }
    slice_to_vec(b)
}

fn bytes_conditionshape() -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::v1text::ascii("condition_shape"@),
{
    let b: &[u8] = b"condition_shape";
    proof {
        reveal_byteslit(b"condition_shape");
        reveal_strlit("condition_shape");
        reveal(ckc_spec::v1text::ascii);
        vstd::assert_seqs_equal!(b@ == ckc_spec::v1text::ascii("condition_shape"@));
    }
    slice_to_vec(b)
}

fn bytes_context() -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::v1text::ascii("context"@),
{
    let b: &[u8] = b"context";
    proof {
        reveal_byteslit(b"context");
        reveal_strlit("context");
        reveal(ckc_spec::v1text::ascii);
        vstd::assert_seqs_equal!(b@ == ckc_spec::v1text::ascii("context"@));
    }
    slice_to_vec(b)
}

fn bytes_deferredoperator() -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::v1text::ascii("deferred_operator"@),
{
    let b: &[u8] = b"deferred_operator";
    proof {
        reveal_byteslit(b"deferred_operator");
        reveal_strlit("deferred_operator");
        reveal(ckc_spec::v1text::ascii);
        vstd::assert_seqs_equal!(b@ == ckc_spec::v1text::ascii("deferred_operator"@));
    }
    slice_to_vec(b)
}

fn bytes_disjunction() -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::v1text::ascii("disjunction"@),
{
    let b: &[u8] = b"disjunction";
    proof {
        reveal_byteslit(b"disjunction");
        reveal_strlit("disjunction");
        reveal(ckc_spec::v1text::ascii);
        vstd::assert_seqs_equal!(b@ == ckc_spec::v1text::ascii("disjunction"@));
    }
    slice_to_vec(b)
}

fn bytes_disjunctiveantecedent() -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::v1text::ascii("disjunctive_antecedent"@),
{
    let b: &[u8] = b"disjunctive_antecedent";
    proof {
        reveal_byteslit(b"disjunctive_antecedent");
        reveal_strlit("disjunctive_antecedent");
        reveal(ckc_spec::v1text::ascii);
        vstd::assert_seqs_equal!(b@ == ckc_spec::v1text::ascii("disjunctive_antecedent"@));
    }
    slice_to_vec(b)
}

fn bytes_docid() -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::v1text::ascii("docid"@),
{
    let b: &[u8] = b"docid";
    proof {
        reveal_byteslit(b"docid");
        reveal_strlit("docid");
        reveal(ckc_spec::v1text::ascii);
        vstd::assert_seqs_equal!(b@ == ckc_spec::v1text::ascii("docid"@));
    }
    slice_to_vec(b)
}

fn bytes_drs() -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::v1text::ascii("drs"@),
{
    let b: &[u8] = b"drs";
    proof {
        reveal_byteslit(b"drs");
        reveal_strlit("drs");
        reveal(ckc_spec::v1text::ascii);
        vstd::assert_seqs_equal!(b@ == ckc_spec::v1text::ascii("drs"@));
    }
    slice_to_vec(b)
}

fn bytes_dumpnoncanonical() -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::v1text::ascii("dump_noncanonical"@),
{
    let b: &[u8] = b"dump_noncanonical";
    proof {
        reveal_byteslit(b"dump_noncanonical");
        reveal_strlit("dump_noncanonical");
        reveal(ckc_spec::v1text::ascii);
        vstd::assert_seqs_equal!(b@ == ckc_spec::v1text::ascii("dump_noncanonical"@));
    }
    slice_to_vec(b)
}

fn bytes_eq() -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::v1text::ascii("eq"@),
{
    let b: &[u8] = b"eq";
    proof {
        reveal_byteslit(b"eq");
        reveal_strlit("eq");
        reveal(ckc_spec::v1text::ascii);
        vstd::assert_seqs_equal!(b@ == ckc_spec::v1text::ascii("eq"@));
    }
    slice_to_vec(b)
}

fn bytes_exactly() -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::v1text::ascii("exactly"@),
{
    let b: &[u8] = b"exactly";
    proof {
        reveal_byteslit(b"exactly");
        reveal_strlit("exactly");
        reveal(ckc_spec::v1text::ascii);
        vstd::assert_seqs_equal!(b@ == ckc_spec::v1text::ascii("exactly"@));
    }
    slice_to_vec(b)
}

fn bytes_geq() -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::v1text::ascii("geq"@),
{
    let b: &[u8] = b"geq";
    proof {
        reveal_byteslit(b"geq");
        reveal_strlit("geq");
        reveal(ckc_spec::v1text::ascii);
        vstd::assert_seqs_equal!(b@ == ckc_spec::v1text::ascii("geq"@));
    }
    slice_to_vec(b)
}

fn bytes_greater() -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::v1text::ascii("greater"@),
{
    let b: &[u8] = b"greater";
    proof {
        reveal_byteslit(b"greater");
        reveal_strlit("greater");
        reveal(ckc_spec::v1text::ascii);
        vstd::assert_seqs_equal!(b@ == ckc_spec::v1text::ascii("greater"@));
    }
    slice_to_vec(b)
}

fn bytes_guideline() -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::v1text::ascii("guideline_"@),
{
    let b: &[u8] = b"guideline_";
    proof {
        reveal_byteslit(b"guideline_");
        reveal_strlit("guideline_");
        reveal(ckc_spec::v1text::ascii);
        vstd::assert_seqs_equal!(b@ == ckc_spec::v1text::ascii("guideline_"@));
    }
    slice_to_vec(b)
}

fn bytes_guidelinearg() -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::v1text::ascii("guideline_arg"@),
{
    let b: &[u8] = b"guideline_arg";
    proof {
        reveal_byteslit(b"guideline_arg");
        reveal_strlit("guideline_arg");
        reveal(ckc_spec::v1text::ascii);
        vstd::assert_seqs_equal!(b@ == ckc_spec::v1text::ascii("guideline_arg"@));
    }
    slice_to_vec(b)
}

fn bytes_guidelinecardinality() -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::v1text::ascii("guideline_cardinality"@),
{
    let b: &[u8] = b"guideline_cardinality";
    proof {
        reveal_byteslit(b"guideline_cardinality");
        reveal_strlit("guideline_cardinality");
        reveal(ckc_spec::v1text::ascii);
        vstd::assert_seqs_equal!(b@ == ckc_spec::v1text::ascii("guideline_cardinality"@));
    }
    slice_to_vec(b)
}

fn bytes_guidelineentity() -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::v1text::ascii("guideline_entity"@),
{
    let b: &[u8] = b"guideline_entity";
    proof {
        reveal_byteslit(b"guideline_entity");
        reveal_strlit("guideline_entity");
        reveal(ckc_spec::v1text::ascii);
        vstd::assert_seqs_equal!(b@ == ckc_spec::v1text::ascii("guideline_entity"@));
    }
    slice_to_vec(b)
}

fn bytes_guidelineevent() -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::v1text::ascii("guideline_event"@),
{
    let b: &[u8] = b"guideline_event";
    proof {
        reveal_byteslit(b"guideline_event");
        reveal_strlit("guideline_event");
        reveal(ckc_spec::v1text::ascii);
        vstd::assert_seqs_equal!(b@ == ckc_spec::v1text::ascii("guideline_event"@));
    }
    slice_to_vec(b)
}

fn bytes_guidelineoperator() -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::v1text::ascii("guideline_operator"@),
{
    let b: &[u8] = b"guideline_operator";
    proof {
        reveal_byteslit(b"guideline_operator");
        reveal_strlit("guideline_operator");
        reveal(ckc_spec::v1text::ascii);
        vstd::assert_seqs_equal!(b@ == ckc_spec::v1text::ascii("guideline_operator"@));
    }
    slice_to_vec(b)
}

fn bytes_guidelinepp() -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::v1text::ascii("guideline_pp"@),
{
    let b: &[u8] = b"guideline_pp";
    proof {
        reveal_byteslit(b"guideline_pp");
        reveal_strlit("guideline_pp");
        reveal(ckc_spec::v1text::ascii);
        vstd::assert_seqs_equal!(b@ == ckc_spec::v1text::ascii("guideline_pp"@));
    }
    slice_to_vec(b)
}

fn bytes_guidelineproperty() -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::v1text::ascii("guideline_property"@),
{
    let b: &[u8] = b"guideline_property";
    proof {
        reveal_byteslit(b"guideline_property");
        reveal_strlit("guideline_property");
        reveal(ckc_spec::v1text::ascii);
        vstd::assert_seqs_equal!(b@ == ckc_spec::v1text::ascii("guideline_property"@));
    }
    slice_to_vec(b)
}

fn bytes_headvariablenotboundinbody() -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::v1text::ascii("head_variable_not_bound_in_body"@),
{
    let b: &[u8] = b"head_variable_not_bound_in_body";
    proof {
        reveal_byteslit(b"head_variable_not_bound_in_body");
        reveal_strlit("head_variable_not_bound_in_body");
        reveal(ckc_spec::v1text::ascii);
        vstd::assert_seqs_equal!(b@ == ckc_spec::v1text::ascii("head_variable_not_bound_in_body"@));
    }
    slice_to_vec(b)
}

fn bytes_invaliddrsshape() -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::v1text::ascii("invalid_drs_shape"@),
{
    let b: &[u8] = b"invalid_drs_shape";
    proof {
        reveal_byteslit(b"invalid_drs_shape");
        reveal_strlit("invalid_drs_shape");
        reveal(ckc_spec::v1text::ascii);
        vstd::assert_seqs_equal!(b@ == ckc_spec::v1text::ascii("invalid_drs_shape"@));
    }
    slice_to_vec(b)
}

fn bytes_leq() -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::v1text::ascii("leq"@),
{
    let b: &[u8] = b"leq";
    proof {
        reveal_byteslit(b"leq");
        reveal_strlit("leq");
        reveal(ckc_spec::v1text::ascii);
        vstd::assert_seqs_equal!(b@ == ckc_spec::v1text::ascii("leq"@));
    }
    slice_to_vec(b)
}

fn bytes_less() -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::v1text::ascii("less"@),
{
    let b: &[u8] = b"less";
    proof {
        reveal_byteslit(b"less");
        reveal_strlit("less");
        reveal(ckc_spec::v1text::ascii);
        vstd::assert_seqs_equal!(b@ == ckc_spec::v1text::ascii("less"@));
    }
    slice_to_vec(b)
}

fn bytes_may() -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::v1text::ascii("may"@),
{
    let b: &[u8] = b"may";
    proof {
        reveal_byteslit(b"may");
        reveal_strlit("may");
        reveal(ckc_spec::v1text::ascii);
        vstd::assert_seqs_equal!(b@ == ckc_spec::v1text::ascii("may"@));
    }
    slice_to_vec(b)
}

fn bytes_messages() -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::v1text::ascii("messages"@),
{
    let b: &[u8] = b"messages";
    proof {
        reveal_byteslit(b"messages");
        reveal_strlit("messages");
        reveal(ckc_spec::v1text::ascii);
        vstd::assert_seqs_equal!(b@ == ckc_spec::v1text::ascii("messages"@));
    }
    slice_to_vec(b)
}

fn bytes_mixedormissingsentenceanchors() -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::v1text::ascii("mixed_or_missing_sentence_anchors"@),
{
    let b: &[u8] = b"mixed_or_missing_sentence_anchors";
    proof {
        reveal_byteslit(b"mixed_or_missing_sentence_anchors");
        reveal_strlit("mixed_or_missing_sentence_anchors");
        reveal(ckc_spec::v1text::ascii);
        vstd::assert_seqs_equal!(b@ == ckc_spec::v1text::ascii("mixed_or_missing_sentence_anchors"@));
    }
    slice_to_vec(b)
}

fn bytes_modifierpp() -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::v1text::ascii("modifier_pp"@),
{
    let b: &[u8] = b"modifier_pp";
    proof {
        reveal_byteslit(b"modifier_pp");
        reveal_strlit("modifier_pp");
        reveal(ckc_spec::v1text::ascii);
        vstd::assert_seqs_equal!(b@ == ckc_spec::v1text::ascii("modifier_pp"@));
    }
    slice_to_vec(b)
}

fn bytes_must() -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::v1text::ascii("must"@),
{
    let b: &[u8] = b"must";
    proof {
        reveal_byteslit(b"must");
        reveal_strlit("must");
        reveal(ckc_spec::v1text::ascii);
        vstd::assert_seqs_equal!(b@ == ckc_spec::v1text::ascii("must"@));
    }
    slice_to_vec(b)
}

fn bytes_na() -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::v1text::ascii("na"@),
{
    let b: &[u8] = b"na";
    proof {
        reveal_byteslit(b"na");
        reveal_strlit("na");
        reveal(ckc_spec::v1text::ascii);
        vstd::assert_seqs_equal!(b@ == ckc_spec::v1text::ascii("na"@));
    }
    slice_to_vec(b)
}

fn bytes_nafplacement() -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::v1text::ascii("naf_placement"@),
{
    let b: &[u8] = b"naf_placement";
    proof {
        reveal_byteslit(b"naf_placement");
        reveal_strlit("naf_placement");
        reveal(ckc_spec::v1text::ascii);
        vstd::assert_seqs_equal!(b@ == ckc_spec::v1text::ascii("naf_placement"@));
    }
    slice_to_vec(b)
}

fn bytes_nafsafety() -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::v1text::ascii("naf_safety"@),
{
    let b: &[u8] = b"naf_safety";
    proof {
        reveal_byteslit(b"naf_safety");
        reveal_strlit("naf_safety");
        reveal(ckc_spec::v1text::ascii);
        vstd::assert_seqs_equal!(b@ == ckc_spec::v1text::ascii("naf_safety"@));
    }
    slice_to_vec(b)
}

fn bytes_nafshape() -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::v1text::ascii("naf_shape"@),
{
    let b: &[u8] = b"naf_shape";
    proof {
        reveal_byteslit(b"naf_shape");
        reveal_strlit("naf_shape");
        reveal(ckc_spec::v1text::ascii);
        vstd::assert_seqs_equal!(b@ == ckc_spec::v1text::ascii("naf_shape"@));
    }
    slice_to_vec(b)
}

fn bytes_noncanonical() -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::v1text::ascii("noncanonical"@),
{
    let b: &[u8] = b"noncanonical";
    proof {
        reveal_byteslit(b"noncanonical");
        reveal_strlit("noncanonical");
        reveal(ckc_spec::v1text::ascii);
        vstd::assert_seqs_equal!(b@ == ckc_spec::v1text::ascii("noncanonical"@));
    }
    slice_to_vec(b)
}

fn bytes_nongroundobligation() -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::v1text::ascii("nonground_obligation"@),
{
    let b: &[u8] = b"nonground_obligation";
    proof {
        reveal_byteslit(b"nonground_obligation");
        reveal_strlit("nonground_obligation");
        reveal(ckc_spec::v1text::ascii);
        vstd::assert_seqs_equal!(b@ == ckc_spec::v1text::ascii("nonground_obligation"@));
    }
    slice_to_vec(b)
}

fn bytes_noun() -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::v1text::ascii("noun"@),
{
    let b: &[u8] = b"noun";
    proof {
        reveal_byteslit(b"noun");
        reveal_strlit("noun");
        reveal(ckc_spec::v1text::ascii);
        vstd::assert_seqs_equal!(b@ == ckc_spec::v1text::ascii("noun"@));
    }
    slice_to_vec(b)
}

fn bytes_object() -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::v1text::ascii("object"@),
{
    let b: &[u8] = b"object";
    proof {
        reveal_byteslit(b"object");
        reveal_strlit("object");
        reveal(ckc_spec::v1text::ascii);
        vstd::assert_seqs_equal!(b@ == ckc_spec::v1text::ascii("object"@));
    }
    slice_to_vec(b)
}

fn bytes_objectoperator() -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::v1text::ascii("object_operator"@),
{
    let b: &[u8] = b"object_operator";
    proof {
        reveal_byteslit(b"object_operator");
        reveal_strlit("object_operator");
        reveal(ckc_spec::v1text::ascii);
        vstd::assert_seqs_equal!(b@ == ckc_spec::v1text::ascii("object_operator"@));
    }
    slice_to_vec(b)
}

fn bytes_operatorscopedrule() -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::v1text::ascii("operator_scoped_rule"@),
{
    let b: &[u8] = b"operator_scoped_rule";
    proof {
        reveal_byteslit(b"operator_scoped_rule");
        reveal_strlit("operator_scoped_rule");
        reveal(ckc_spec::v1text::ascii);
        vstd::assert_seqs_equal!(b@ == ckc_spec::v1text::ascii("operator_scoped_rule"@));
    }
    slice_to_vec(b)
}

fn bytes_pos() -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::v1text::ascii("pos"@),
{
    let b: &[u8] = b"pos";
    proof {
        reveal_byteslit(b"pos");
        reveal_strlit("pos");
        reveal(ckc_spec::v1text::ascii);
        vstd::assert_seqs_equal!(b@ == ckc_spec::v1text::ascii("pos"@));
    }
    slice_to_vec(b)
}

fn bytes_predicate() -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::v1text::ascii("predicate"@),
{
    let b: &[u8] = b"predicate";
    proof {
        reveal_byteslit(b"predicate");
        reveal_strlit("predicate");
        reveal(ckc_spec::v1text::ascii);
        vstd::assert_seqs_equal!(b@ == ckc_spec::v1text::ascii("predicate"@));
    }
    slice_to_vec(b)
}

fn bytes_product() -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::v1text::ascii("product"@),
{
    let b: &[u8] = b"product";
    proof {
        reveal_byteslit(b"product");
        reveal_strlit("product");
        reveal(ckc_spec::v1text::ascii);
        vstd::assert_seqs_equal!(b@ == ckc_spec::v1text::ascii("product"@));
    }
    slice_to_vec(b)
}

fn bytes_projection() -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::v1text::ascii("projection"@),
{
    let b: &[u8] = b"projection";
    proof {
        reveal_byteslit(b"projection");
        reveal_strlit("projection");
        reveal(ckc_spec::v1text::ascii);
        vstd::assert_seqs_equal!(b@ == ckc_spec::v1text::ascii("projection"@));
    }
    slice_to_vec(b)
}

fn bytes_property() -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::v1text::ascii("property"@),
{
    let b: &[u8] = b"property";
    proof {
        reveal_byteslit(b"property");
        reveal_strlit("property");
        reveal(ckc_spec::v1text::ascii);
        vstd::assert_seqs_equal!(b@ == ckc_spec::v1text::ascii("property"@));
    }
    slice_to_vec(b)
}

fn bytes_propertypolarity() -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::v1text::ascii("property_polarity"@),
{
    let b: &[u8] = b"property_polarity";
    proof {
        reveal_byteslit(b"property_polarity");
        reveal_strlit("property_polarity");
        reveal(ckc_spec::v1text::ascii);
        vstd::assert_seqs_equal!(b@ == ckc_spec::v1text::ascii("property_polarity"@));
    }
    slice_to_vec(b)
}

fn bytes_qid() -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::v1text::ascii("qid"@),
{
    let b: &[u8] = b"qid";
    proof {
        reveal_byteslit(b"qid");
        reveal_strlit("qid");
        reveal(ckc_spec::v1text::ascii);
        vstd::assert_seqs_equal!(b@ == ckc_spec::v1text::ascii("qid"@));
    }
    slice_to_vec(b)
}

fn bytes_query() -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::v1text::ascii("query"@),
{
    let b: &[u8] = b"query";
    proof {
        reveal_byteslit(b"query");
        reveal_strlit("query");
        reveal(ckc_spec::v1text::ascii);
        vstd::assert_seqs_equal!(b@ == ckc_spec::v1text::ascii("query"@));
    }
    slice_to_vec(b)
}

fn bytes_querymarker() -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::v1text::ascii("query_marker"@),
{
    let b: &[u8] = b"query_marker";
    proof {
        reveal_byteslit(b"query_marker");
        reveal_strlit("query_marker");
        reveal(ckc_spec::v1text::ascii);
        vstd::assert_seqs_equal!(b@ == ckc_spec::v1text::ascii("query_marker"@));
    }
    slice_to_vec(b)
}

fn bytes_querymarkerunbound() -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::v1text::ascii("query_marker_unbound"@),
{
    let b: &[u8] = b"query_marker_unbound";
    proof {
        reveal_byteslit(b"query_marker_unbound");
        reveal_strlit("query_marker_unbound");
        reveal(ckc_spec::v1text::ascii);
        vstd::assert_seqs_equal!(b@ == ckc_spec::v1text::ascii("query_marker_unbound"@));
    }
    slice_to_vec(b)
}

fn bytes_queryroot() -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::v1text::ascii("query_root"@),
{
    let b: &[u8] = b"query_root";
    proof {
        reveal_byteslit(b"query_root");
        reveal_strlit("query_root");
        reveal(ckc_spec::v1text::ascii);
        vstd::assert_seqs_equal!(b@ == ckc_spec::v1text::ascii("query_root"@));
    }
    slice_to_vec(b)
}

fn bytes_querysentences() -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::v1text::ascii("query_sentences"@),
{
    let b: &[u8] = b"query_sentences";
    proof {
        reveal_byteslit(b"query_sentences");
        reveal_strlit("query_sentences");
        reveal(ckc_spec::v1text::ascii);
        vstd::assert_seqs_equal!(b@ == ckc_spec::v1text::ascii("query_sentences"@));
    }
    slice_to_vec(b)
}

fn bytes_querytext() -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::v1text::ascii("query_text"@),
{
    let b: &[u8] = b"query_text";
    proof {
        reveal_byteslit(b"query_text");
        reveal_strlit("query_text");
        reveal(ckc_spec::v1text::ascii);
        vstd::assert_seqs_equal!(b@ == ckc_spec::v1text::ascii("query_text"@));
    }
    slice_to_vec(b)
}

fn bytes_queryunsupported() -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::v1text::ascii("query_unsupported"@),
{
    let b: &[u8] = b"query_unsupported";
    proof {
        reveal_byteslit(b"query_unsupported");
        reveal_strlit("query_unsupported");
        reveal(ckc_spec::v1text::ascii);
        vstd::assert_seqs_equal!(b@ == ckc_spec::v1text::ascii("query_unsupported"@));
    }
    slice_to_vec(b)
}

fn bytes_question() -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::v1text::ascii("question"@),
{
    let b: &[u8] = b"question";
    proof {
        reveal_byteslit(b"question");
        reveal_strlit("question");
        reveal(ckc_spec::v1text::ascii);
        vstd::assert_seqs_equal!(b@ == ckc_spec::v1text::ascii("question"@));
    }
    slice_to_vec(b)
}

fn bytes_recordshape() -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::v1text::ascii("record_shape"@),
{
    let b: &[u8] = b"record_shape";
    proof {
        reveal_byteslit(b"record_shape");
        reveal_strlit("record_shape");
        reveal(ckc_spec::v1text::ascii);
        vstd::assert_seqs_equal!(b@ == ckc_spec::v1text::ascii("record_shape"@));
    }
    slice_to_vec(b)
}

fn bytes_ref() -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::v1text::ascii("ref"@),
{
    let b: &[u8] = b"ref";
    proof {
        reveal_byteslit(b"ref");
        reveal_strlit("ref");
        reveal(ckc_spec::v1text::ascii);
        vstd::assert_seqs_equal!(b@ == ckc_spec::v1text::ascii("ref"@));
    }
    slice_to_vec(b)
}

fn bytes_reservednamecollision() -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::v1text::ascii("reserved_name_collision"@),
{
    let b: &[u8] = b"reserved_name_collision";
    proof {
        reveal_byteslit(b"reserved_name_collision");
        reveal_strlit("reserved_name_collision");
        reveal(ckc_spec::v1text::ascii);
        vstd::assert_seqs_equal!(b@ == ckc_spec::v1text::ascii("reserved_name_collision"@));
    }
    slice_to_vec(b)
}

fn bytes_rootcondition() -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::v1text::ascii("root_condition"@),
{
    let b: &[u8] = b"root_condition";
    proof {
        reveal_byteslit(b"root_condition");
        reveal_strlit("root_condition");
        reveal(ckc_spec::v1text::ascii);
        vstd::assert_seqs_equal!(b@ == ckc_spec::v1text::ascii("root_condition"@));
    }
    slice_to_vec(b)
}

fn bytes_rulewithoutantecedent() -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::v1text::ascii("rule_without_antecedent"@),
{
    let b: &[u8] = b"rule_without_antecedent";
    proof {
        reveal_byteslit(b"rule_without_antecedent");
        reveal_strlit("rule_without_antecedent");
        reveal(ckc_spec::v1text::ascii);
        vstd::assert_seqs_equal!(b@ == ckc_spec::v1text::ascii("rule_without_antecedent"@));
    }
    slice_to_vec(b)
}

fn bytes_rulewithoutconsequent() -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::v1text::ascii("rule_without_consequent"@),
{
    let b: &[u8] = b"rule_without_consequent";
    proof {
        reveal_byteslit(b"rule_without_consequent");
        reveal_strlit("rule_without_consequent");
        reveal(ckc_spec::v1text::ascii);
        vstd::assert_seqs_equal!(b@ == ckc_spec::v1text::ascii("rule_without_consequent"@));
    }
    slice_to_vec(b)
}

fn bytes_sentence() -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::v1text::ascii("sentence"@),
{
    let b: &[u8] = b"sentence";
    proof {
        reveal_byteslit(b"sentence");
        reveal_strlit("sentence");
        reveal(ckc_spec::v1text::ascii);
        vstd::assert_seqs_equal!(b@ == ckc_spec::v1text::ascii("sentence"@));
    }
    slice_to_vec(b)
}

fn bytes_sentencelines() -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::v1text::ascii("sentence_lines"@),
{
    let b: &[u8] = b"sentence_lines";
    proof {
        reveal_byteslit(b"sentence_lines");
        reveal_strlit("sentence_lines");
        reveal(ckc_spec::v1text::ascii);
        vstd::assert_seqs_equal!(b@ == ckc_spec::v1text::ascii("sentence_lines"@));
    }
    slice_to_vec(b)
}

fn bytes_sentenceshape() -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::v1text::ascii("sentence_shape"@),
{
    let b: &[u8] = b"sentence_shape";
    proof {
        reveal_byteslit(b"sentence_shape");
        reveal_strlit("sentence_shape");
        reveal(ckc_spec::v1text::ascii);
        vstd::assert_seqs_equal!(b@ == ckc_spec::v1text::ascii("sentence_shape"@));
    }
    slice_to_vec(b)
}

fn bytes_sentences() -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::v1text::ascii("sentences"@),
{
    let b: &[u8] = b"sentences";
    proof {
        reveal_byteslit(b"sentences");
        reveal_strlit("sentences");
        reveal(ckc_spec::v1text::ascii);
        vstd::assert_seqs_equal!(b@ == ckc_spec::v1text::ascii("sentences"@));
    }
    slice_to_vec(b)
}

fn bytes_should() -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::v1text::ascii("should"@),
{
    let b: &[u8] = b"should";
    proof {
        reveal_byteslit(b"should");
        reveal_strlit("should");
        reveal(ckc_spec::v1text::ascii);
        vstd::assert_seqs_equal!(b@ == ckc_spec::v1text::ascii("should"@));
    }
    slice_to_vec(b)
}

fn bytes_ulex() -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::v1text::ascii("ulex"@),
{
    let b: &[u8] = b"ulex";
    proof {
        reveal_byteslit(b"ulex");
        reveal_strlit("ulex");
        reveal(ckc_spec::v1text::ascii);
        vstd::assert_seqs_equal!(b@ == ckc_spec::v1text::ascii("ulex"@));
    }
    slice_to_vec(b)
}

fn bytes_unresolvedargument() -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::v1text::ascii("unresolved_argument"@),
{
    let b: &[u8] = b"unresolved_argument";
    proof {
        reveal_byteslit(b"unresolved_argument");
        reveal_strlit("unresolved_argument");
        reveal(ckc_spec::v1text::ascii);
        vstd::assert_seqs_equal!(b@ == ckc_spec::v1text::ascii("unresolved_argument"@));
    }
    slice_to_vec(b)
}

fn bytes_unsupported() -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::v1text::ascii("unsupported"@),
{
    let b: &[u8] = b"unsupported";
    proof {
        reveal_byteslit(b"unsupported");
        reveal_strlit("unsupported");
        reveal(ckc_spec::v1text::ascii);
        vstd::assert_seqs_equal!(b@ == ckc_spec::v1text::ascii("unsupported"@));
    }
    slice_to_vec(b)
}

fn bytes_v() -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::v1text::ascii("v"@),
{
    let b: &[u8] = b"v";
    proof {
        reveal_byteslit(b"v");
        reveal_strlit("v");
        reveal(ckc_spec::v1text::ascii);
        vstd::assert_seqs_equal!(b@ == ckc_spec::v1text::ascii("v"@));
    }
    slice_to_vec(b)
}

fn bytes_variant() -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::v1text::ascii("variant"@),
{
    let b: &[u8] = b"variant";
    proof {
        reveal_byteslit(b"variant");
        reveal_strlit("variant");
        reveal(ckc_spec::v1text::ascii);
        vstd::assert_seqs_equal!(b@ == ckc_spec::v1text::ascii("variant"@));
    }
    slice_to_vec(b)
}

fn bytes_wh() -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::v1text::ascii("wh"@),
{
    let b: &[u8] = b"wh";
    proof {
        reveal_byteslit(b"wh");
        reveal_strlit("wh");
        reveal(ckc_spec::v1text::ascii);
        vstd::assert_seqs_equal!(b@ == ckc_spec::v1text::ascii("wh"@));
    }
    slice_to_vec(b)
}

fn bytes_what() -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::v1text::ascii("what"@),
{
    let b: &[u8] = b"what";
    proof {
        reveal_byteslit(b"what");
        reveal_strlit("what");
        reveal(ckc_spec::v1text::ascii);
        vstd::assert_seqs_equal!(b@ == ckc_spec::v1text::ascii("what"@));
    }
    slice_to_vec(b)
}

fn bytes_which() -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::v1text::ascii("which"@),
{
    let b: &[u8] = b"which";
    proof {
        reveal_byteslit(b"which");
        reveal_strlit("which");
        reveal(ckc_spec::v1text::ascii);
        vstd::assert_seqs_equal!(b@ == ckc_spec::v1text::ascii("which"@));
    }
    slice_to_vec(b)
}

fn bytes_who() -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::v1text::ascii("who"@),
{
    let b: &[u8] = b"who";
    proof {
        reveal_byteslit(b"who");
        reveal_strlit("who");
        reveal(ckc_spec::v1text::ascii);
        vstd::assert_seqs_equal!(b@ == ckc_spec::v1text::ascii("who"@));
    }
    slice_to_vec(b)
}

fn bytes_witness() -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::v1text::ascii("witness"@),
{
    let b: &[u8] = b"witness";
    proof {
        reveal_byteslit(b"witness");
        reveal_strlit("witness");
        reveal(ckc_spec::v1text::ascii);
        vstd::assert_seqs_equal!(b@ == ckc_spec::v1text::ascii("witness"@));
    }
    slice_to_vec(b)
}

fn bytes_naf() -> (out: Vec<u8>)
    ensures
        out@ == ckc_spec::v1text::ascii("~"@),
{
    let b: &[u8] = b"~";
    proof {
        reveal_byteslit(b"~");
        reveal_strlit("~");
        reveal(ckc_spec::v1text::ascii);
        vstd::assert_seqs_equal!(b@ == ckc_spec::v1text::ascii("~"@));
    }
    slice_to_vec(b)
}

} // verus!

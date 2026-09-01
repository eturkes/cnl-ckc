use vstd::prelude::*;
use crate::term::*;

verus! {

// Trusted spec: canonical v1 clause-file grammar, printer-first (contract
// m5u2 R4/R9/R10/R11). `print_v1` is the byte law; a file is accepted iff
// some wellformed model prints to exactly its bytes (`accepts`). The
// grammar's authority = the legacy emitters' writer range (write_term
// [quoted,numbervars,character_escapes,ignore_ops] + the rule/NAF glue in
// ace_to_pl.pl v1 rendering) over the four artifact classes, narrowed by
// ruling R21 (no float literals) and ruling R22 (atoms holding any byte
// >= 0x80 spell quoted, the bytes raw inside the quotes; SWI's own
// unicode bare/escape classes stay outside the verified surface).
// Escape spellings are live-probe pins from swipl 9.2.9
// (.scratch/m5u2/spec-probes/): named \a..\r for 0x07-0x0D, \uXXXX
// (4 uppercase hex) for other ASCII control bytes + 0x7F, backslash and
// quote escaped, every other byte raw. UTF-8 validity of the whole file
// is checked at the shell boundary, before the kernel sees bytes.

// --- byte classes (ASCII-exact) ---

pub open spec fn is_lower_b(b: u8) -> bool {
    0x61 <= b && b <= 0x7A
}

pub open spec fn is_digit_b(b: u8) -> bool {
    0x30 <= b && b <= 0x39
}

// Identifier continuation: a-z A-Z 0-9 _
pub open spec fn is_alnum_b(b: u8) -> bool {
    is_lower_b(b) || (0x41 <= b && b <= 0x5A) || is_digit_b(b) || b == 0x5F
}

// SWI symbol chars: # $ & * + - . / : < = > ? @ \ ^ ~
pub open spec fn is_graphic_b(b: u8) -> bool {
    b == 0x23 || b == 0x24 || b == 0x26 || b == 0x2A || b == 0x2B || b == 0x2D
        || b == 0x2E || b == 0x2F || b == 0x3A || b == 0x3C || b == 0x3D
        || b == 0x3E || b == 0x3F || b == 0x40 || b == 0x5C || b == 0x5E
        || b == 0x7E
}

pub open spec fn is_hex_lower_b(b: u8) -> bool {
    is_digit_b(b) || (0x61 <= b && b <= 0x66)
}

pub open spec fn all_in(s: Seq<u8>, pred: spec_fn(u8) -> bool) -> bool {
    forall|i: int| #![auto] 0 <= i < s.len() ==> pred(s[i])
}

// ASCII template helper: string literals become byte sequences.
pub open spec fn ascii(s: Seq<char>) -> Seq<u8> {
    Seq::new(s.len(), |i: int| (s[i] as int) as u8)
}

// --- canonical decimals ---

pub open spec fn digit_byte(d: int) -> u8 {
    (0x30 + d) as u8
}

pub open spec fn udec_bytes(n: nat) -> Seq<u8>
    decreases n,
{
    if n < 10 {
        seq![digit_byte(n as int)]
    } else {
        udec_bytes(n / 10) + seq![digit_byte((n % 10) as int)]
    }
}

pub open spec fn dec_bytes(n: int) -> Seq<u8> {
    if n < 0 {
        seq![0x2Du8] + udec_bytes((-n) as nat)
    } else {
        udec_bytes(n as nat)
    }
}

// --- atom spelling ---

pub open spec fn alpha_bare(name: Seq<u8>) -> bool {
    name.len() > 0 && is_lower_b(name[0]) && all_in(name, |b: u8| is_alnum_b(b))
}

// Graphic atoms print bare unless the name is exactly "." or opens a
// block comment (probe: '/*=' quoted, '=/*=' bare).
pub open spec fn graphic_bare(name: Seq<u8>) -> bool {
    &&& name.len() > 0
    &&& all_in(name, |b: u8| is_graphic_b(b))
    &&& name != seq![0x2Eu8]
    &&& !(name.len() >= 2 && name[0] == 0x2F && name[1] == 0x2A)
}

// Solo atoms that print bare: ";" "!" "{}". The atom named "[]" prints
// quoted (SWI keeps it distinct from nil); "," and "|" print quoted.
pub open spec fn solo_bare(name: Seq<u8>) -> bool {
    name == seq![0x3Bu8] || name == seq![0x21u8] || name == seq![0x7Bu8, 0x7Du8]
}

pub open spec fn atom_bare(name: Seq<u8>) -> bool {
    alpha_bare(name) || graphic_bare(name) || solo_bare(name)
}

pub open spec fn uhex_digit(d: int) -> u8 {
    if d < 10 {
        digit_byte(d)
    } else {
        (0x41 + (d - 10)) as u8
    }
}

pub open spec fn uhex4(v: int) -> Seq<u8> {
    seq![
        uhex_digit(v / 4096 % 16),
        uhex_digit(v / 256 % 16),
        uhex_digit(v / 16 % 16),
        uhex_digit(v % 16),
    ]
}

// Quoted-content escape law (probe-pinned).
pub open spec fn esc_byte(b: u8) -> Seq<u8> {
    if b == 0x5C {
        ascii("\\\\"@)
    } else if b == 0x27 {
        ascii("\\'"@)
    } else if b == 0x07 {
        ascii("\\a"@)
    } else if b == 0x08 {
        ascii("\\b"@)
    } else if b == 0x09 {
        ascii("\\t"@)
    } else if b == 0x0A {
        ascii("\\n"@)
    } else if b == 0x0B {
        ascii("\\v"@)
    } else if b == 0x0C {
        ascii("\\f"@)
    } else if b == 0x0D {
        ascii("\\r"@)
    } else if b < 0x20 || b == 0x7F {
        ascii("\\u"@) + uhex4(b as int)
    } else {
        seq![b]
    }
}

pub open spec fn esc_all(name: Seq<u8>) -> Seq<u8>
    decreases name.len(),
{
    if name.len() == 0 {
        Seq::empty()
    } else {
        esc_byte(name[0]) + esc_all(name.drop_first())
    }
}

pub open spec fn atom_bytes(name: Seq<u8>) -> Seq<u8> {
    if atom_bare(name) {
        name
    } else {
        seq![0x27u8] + esc_all(name) + seq![0x27u8]
    }
}

// --- numbervar rendering: 0..25 -> A..Z, then A1..Z1, A2.. ---

pub open spec fn var_bytes(k: nat) -> Seq<u8> {
    let letter = (0x41 + (k % 26)) as u8;
    if k / 26 == 0 {
        seq![letter]
    } else {
        seq![letter] + udec_bytes(k / 26)
    }
}

// --- term rendering ---

pub open spec fn cons_name() -> Seq<u8> {
    seq![0x5Bu8, 0x7Cu8, 0x5Du8]  // "[|]"
}

pub open spec fn curly_name() -> Seq<u8> {
    seq![0x7Bu8, 0x7Du8]  // "{}"
}

pub open spec fn term_bytes(t: Term) -> Seq<u8>
    decreases t, 0int,
{
    match t {
        Term::Var(k) => var_bytes(k),
        Term::Int(n) => dec_bytes(n),
        Term::Nil => ascii("[]"@),
        Term::Atom(name) => atom_bytes(name),
        Term::Comp(name, args) => if name == cons_name() && args.len() == 2 {
            seq![0x5Bu8] + term_bytes(args[0]) + tail_bytes(args[1]) + seq![0x5Du8]
        } else if name == curly_name() && args.len() == 1 {
            seq![0x7Bu8] + term_bytes(args[0]) + seq![0x7Du8]
        } else {
            atom_bytes(name) + seq![0x28u8] + args_bytes(args) + seq![0x29u8]
        },
    }
}

pub open spec fn args_bytes(ts: Seq<Term>) -> Seq<u8>
    decreases ts, 0int,
{
    if ts.len() == 0 {
        Seq::empty()
    } else if ts.len() == 1 {
        term_bytes(ts[0])
    } else {
        term_bytes(ts[0]) + seq![0x2Cu8] + args_bytes(ts.drop_first())
    }
}

pub open spec fn tail_bytes(t: Term) -> Seq<u8>
    decreases t, 1int,
{
    match t {
        Term::Nil => Seq::empty(),
        Term::Comp(name, args) => if name == cons_name() && args.len() == 2 {
            seq![0x2Cu8] + term_bytes(args[0]) + tail_bytes(args[1])
        } else {
            seq![0x7Cu8] + term_bytes(t)
        },
        _ => seq![0x7Cu8] + term_bytes(t),
    }
}

// --- v1 ABI indicators (frozen order = the emitted declaration block) ---

pub open spec fn indicator(i: int) -> (Seq<u8>, nat) {
    if i == 0 {
        (ascii("guideline_schema_version"@), 1)
    } else if i == 1 {
        (ascii("guideline_document"@), 3)
    } else if i == 2 {
        (ascii("guideline_entity"@), 4)
    } else if i == 3 {
        (ascii("guideline_cardinality"@), 5)
    } else if i == 4 {
        (ascii("guideline_event"@), 3)
    } else if i == 5 {
        (ascii("guideline_arg"@), 4)
    } else if i == 6 {
        (ascii("guideline_pp"@), 4)
    } else if i == 7 {
        (ascii("guideline_property"@), 4)
    } else {
        (ascii("guideline_operator"@), 3)
    }
}

// The seven semantic indicators (positions 2..8) are the only head/body
// predicates of bundle clauses (contract R14: foreign or variable body
// goals are grammar-unrepresentable).
pub open spec fn is_semantic_pred(name: Seq<u8>, arity: nat) -> bool {
    exists|i: int| 2 <= i < 9 && #[trigger] indicator(i) == (name, arity)
}

// --- clause lines ---

pub ghost enum BodyItem {
    Pos(Term),
    Naf(Seq<Term>),
}

pub ghost struct DocClause {
    pub head: Term,
    pub body: Seq<BodyItem>,  // empty = fact
}

pub open spec fn lit_list_bytes(gs: Seq<Term>) -> Seq<u8>
    decreases gs,
{
    if gs.len() == 0 {
        Seq::empty()
    } else if gs.len() == 1 {
        term_bytes(gs[0])
    } else {
        term_bytes(gs[0]) + ascii(", "@) + lit_list_bytes(gs.drop_first())
    }
}

// NAF glue: singleton "\+ G"; conjunction "\+ (G1, G2)".
pub open spec fn body_item_bytes(it: BodyItem) -> Seq<u8> {
    match it {
        BodyItem::Pos(l) => term_bytes(l),
        BodyItem::Naf(gs) => if gs.len() == 1 {
            ascii("\\+ "@) + term_bytes(gs[0])
        } else {
            ascii("\\+ ("@) + lit_list_bytes(gs) + seq![0x29u8]
        },
    }
}

pub open spec fn body_bytes(items: Seq<BodyItem>) -> Seq<u8>
    decreases items,
{
    if items.len() == 0 {
        Seq::empty()
    } else if items.len() == 1 {
        body_item_bytes(items[0])
    } else {
        body_item_bytes(items[0]) + ascii(", "@) + body_bytes(items.drop_first())
    }
}

pub open spec fn term_line(t: Term) -> Seq<u8> {
    term_bytes(t) + ascii(".\n"@)
}

pub open spec fn clause_line(c: DocClause) -> Seq<u8> {
    if c.body.len() == 0 {
        term_line(c.head)
    } else {
        term_bytes(c.head) + ascii(" :- "@) + body_bytes(c.body) + ascii(".\n"@)
    }
}

// --- clause wellformedness ---

pub open spec fn wf_literal(t: Term) -> bool {
    match t {
        Term::Comp(name, args) => is_semantic_pred(name, args.len())
            && wf_terms(args) && no_dollar_var_all(args),
        _ => false,
    }
}

pub open spec fn wf_body_item(it: BodyItem) -> bool {
    match it {
        BodyItem::Pos(l) => wf_literal(l),
        BodyItem::Naf(gs) => gs.len() >= 1
            && forall|i: int| #![auto] 0 <= i < gs.len() ==> wf_literal(gs[i]),
    }
}

pub open spec fn item_var_stream(it: BodyItem) -> Seq<nat> {
    match it {
        BodyItem::Pos(l) => var_stream(l),
        BodyItem::Naf(gs) => var_stream_all(gs),
    }
}

pub open spec fn body_var_stream(items: Seq<BodyItem>) -> Seq<nat>
    decreases items,
{
    if items.len() == 0 {
        Seq::empty()
    } else {
        item_var_stream(items[0]) + body_var_stream(items.drop_first())
    }
}

// One clause line = one numbervars pass: head then body, firsts 0,1,2,..
pub open spec fn wf_clause(c: DocClause) -> bool {
    &&& wf_literal(c.head)
    &&& forall|i: int| #![auto] 0 <= i < c.body.len() ==> wf_body_item(c.body[i])
    &&& var_canonical(var_stream(c.head) + body_var_stream(c.body))
}

// --- file models ---

pub ghost struct Bundle {
    pub s: nat,           // sentence ordinal, marker "% S<s>: "
    pub text: Seq<u8>,    // sentence bytes, opaque payload (R11)
    pub clauses: Seq<DocClause>,
}

pub ghost struct DocFile {
    pub docid: Seq<u8>,
    pub ace: Seq<u8>,              // 64 lowercase hex
    pub ulex: Option<Seq<u8>>,     // None = ulex(none)
    pub bundles: Seq<Bundle>,
}

pub ghost struct QueryFile {
    pub qid: Seq<u8>,
    pub ace: Seq<u8>,
    pub ulex: Option<Seq<u8>>,
    pub qtext: Seq<u8>,   // "% Q1: " payload
    pub goal: Term,
    pub answers: Term,
}

pub ghost struct AnswersFile {
    pub qid: Seq<u8>,
    pub qsha: Seq<u8>,
    pub result: Term,
}

pub ghost struct TracesFile {
    pub qid: Seq<u8>,
    pub qsha: Seq<u8>,
    pub asha: Seq<u8>,
    pub result: Term,
}

pub ghost enum V1File {
    Doc(DocFile),
    Query(QueryFile),
    Answers(AnswersFile),
    Traces(TracesFile),
}

// --- envelope rendering ---

pub open spec fn doc_line1(docid: Seq<u8>) -> Seq<u8> {
    ascii("% "@) + docid
        + ascii(".pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.\n"@)
}

pub open spec fn query_line1(qid: Seq<u8>) -> Seq<u8> {
    ascii("% "@) + qid
        + ascii(" compiled from ACE question by ace_to_pl question mode; do not edit.\n"@)
}

pub open spec fn answers_line1(qid: Seq<u8>) -> Seq<u8> {
    ascii("% "@) + qid
        + ascii(" answered against the loaded composition by ace_to_pl answer mode; do not edit.\n"@)
}

pub open spec fn traces_line1(qid: Seq<u8>) -> Seq<u8> {
    ascii("% "@) + qid
        + ascii(" traced against the loaded composition by ace_to_pl trace mode; do not edit.\n"@)
}

pub open spec fn decl_pair(i: int) -> Seq<u8> {
    ascii(":- multifile("@) + indicator(i).0 + seq![0x2Fu8] + udec_bytes(indicator(i).1)
        + ascii(").\n"@) + ascii(":- discontiguous("@) + indicator(i).0 + seq![0x2Fu8]
        + udec_bytes(indicator(i).1) + ascii(").\n"@)
}

pub open spec fn decls_from(i: int) -> Seq<u8>
    decreases 9 - i,
{
    if i < 0 || i >= 9 {
        Seq::empty()
    } else {
        decl_pair(i) + decls_from(i + 1)
    }
}

pub open spec fn marker_line(s: nat, text: Seq<u8>) -> Seq<u8> {
    ascii("% S"@) + udec_bytes(s) + ascii(": "@) + text + seq![0x0Au8]
}

pub open spec fn ulex_term(u: Option<Seq<u8>>) -> Term {
    match u {
        Option::None => Term::Atom(ascii("none"@)),
        Option::Some(h) => Term::Comp(ascii("sha256"@), seq![Term::Atom(h)]),
    }
}

pub open spec fn schema_version_term() -> Term {
    Term::Comp(ascii("guideline_schema_version"@), seq![Term::Int(1)])
}

pub open spec fn doc_record_term(d: DocFile) -> Term {
    Term::Comp(
        ascii("guideline_document"@),
        seq![
            Term::Atom(d.docid),
            Term::Comp(ascii("ace_sha256"@), seq![Term::Atom(d.ace)]),
            Term::Comp(ascii("ulex"@), seq![ulex_term(d.ulex)]),
        ],
    )
}

pub open spec fn query_record_term(q: QueryFile) -> Term {
    Term::Comp(
        ascii("$guideline_query"@),
        seq![
            Term::Atom(ascii("v1"@)),
            Term::Atom(q.qid),
            Term::Comp(ascii("ace_sha256"@), seq![Term::Atom(q.ace)]),
            Term::Comp(ascii("ulex"@), seq![ulex_term(q.ulex)]),
        ],
    )
}

pub open spec fn projection_term(q: QueryFile) -> Term {
    Term::Comp(
        ascii("$guideline_query_projection"@),
        seq![
            Term::Comp(ascii("goal"@), seq![q.goal]),
            Term::Comp(ascii("answers"@), seq![q.answers]),
        ],
    )
}

pub open spec fn answers_record_term(a: AnswersFile) -> Term {
    Term::Comp(
        ascii("$guideline_answers"@),
        seq![
            Term::Atom(ascii("v1"@)),
            Term::Atom(a.qid),
            Term::Comp(ascii("query_sha256"@), seq![Term::Atom(a.qsha)]),
            Term::Comp(ascii("result"@), seq![a.result]),
        ],
    )
}

pub open spec fn traces_record_term(t: TracesFile) -> Term {
    Term::Comp(
        ascii("$guideline_traces"@),
        seq![
            Term::Atom(ascii("v1"@)),
            Term::Atom(t.qid),
            Term::Comp(ascii("query_sha256"@), seq![Term::Atom(t.qsha)]),
            Term::Comp(ascii("answers_sha256"@), seq![Term::Atom(t.asha)]),
            Term::Comp(ascii("result"@), seq![t.result]),
        ],
    )
}

pub open spec fn clauses_bytes(cs: Seq<DocClause>) -> Seq<u8>
    decreases cs,
{
    if cs.len() == 0 {
        Seq::empty()
    } else {
        clause_line(cs[0]) + clauses_bytes(cs.drop_first())
    }
}

pub open spec fn bundles_bytes(bs: Seq<Bundle>) -> Seq<u8>
    decreases bs,
{
    if bs.len() == 0 {
        Seq::empty()
    } else {
        marker_line(bs[0].s, bs[0].text) + clauses_bytes(bs[0].clauses)
            + bundles_bytes(bs.drop_first())
    }
}

pub open spec fn print_doc(d: DocFile) -> Seq<u8> {
    doc_line1(d.docid) + decls_from(0) + term_line(schema_version_term())
        + term_line(doc_record_term(d)) + bundles_bytes(d.bundles)
}

pub open spec fn print_query(q: QueryFile) -> Seq<u8> {
    query_line1(q.qid) + term_line(query_record_term(q)) + ascii("% Q1: "@) + q.qtext
        + seq![0x0Au8] + term_line(projection_term(q))
}

pub open spec fn print_answers(a: AnswersFile) -> Seq<u8> {
    answers_line1(a.qid) + term_line(answers_record_term(a))
}

pub open spec fn print_traces(t: TracesFile) -> Seq<u8> {
    traces_line1(t.qid) + term_line(traces_record_term(t))
}

pub open spec fn print_v1(f: V1File) -> Seq<u8> {
    match f {
        V1File::Doc(d) => print_doc(d),
        V1File::Query(q) => print_query(q),
        V1File::Answers(a) => print_answers(a),
        V1File::Traces(t) => print_traces(t),
    }
}

// --- wellformedness ---

// docid/qid law: nonempty [a-z0-9-], no leading dash.
pub open spec fn name_ok(id: Seq<u8>) -> bool {
    &&& id.len() > 0
    &&& all_in(id, |b: u8| is_lower_b(b) || is_digit_b(b) || b == 0x2D)
    &&& id[0] != 0x2D
}

pub open spec fn hex64(h: Seq<u8>) -> bool {
    h.len() == 64 && all_in(h, |b: u8| is_hex_lower_b(b))
}

pub open spec fn ulex_ok(u: Option<Seq<u8>>) -> bool {
    match u {
        Option::None => true,
        Option::Some(h) => hex64(h),
    }
}

// Comment payloads: nonempty, LF-free, otherwise opaque (R11).
pub open spec fn text_ok(text: Seq<u8>) -> bool {
    text.len() > 0 && all_in(text, |b: u8| b != 0x0A)
}

pub open spec fn wf_bundle(b: Bundle) -> bool {
    &&& b.s >= 1
    &&& text_ok(b.text)
    &&& b.clauses.len() >= 1
    &&& forall|i: int| #![auto] 0 <= i < b.clauses.len() ==> wf_clause(b.clauses[i])
}

pub open spec fn wf_doc(d: DocFile) -> bool {
    &&& name_ok(d.docid)
    &&& hex64(d.ace)
    &&& ulex_ok(d.ulex)
    &&& d.bundles.len() >= 1
    &&& forall|i: int| #![auto] 0 <= i < d.bundles.len() ==> wf_bundle(d.bundles[i])
    &&& forall|i: int|
        0 <= i < d.bundles.len() - 1 ==> #[trigger] d.bundles[i].s < d.bundles[i + 1].s
}

pub open spec fn wf_query(q: QueryFile) -> bool {
    &&& name_ok(q.qid)
    &&& hex64(q.ace)
    &&& ulex_ok(q.ulex)
    &&& text_ok(q.qtext)
    &&& wf_term(q.goal) && no_dollar_var(q.goal)
    &&& wf_term(q.answers) && no_dollar_var(q.answers)
    &&& var_canonical(var_stream(q.goal) + var_stream(q.answers))
}

pub open spec fn wf_answers(a: AnswersFile) -> bool {
    &&& name_ok(a.qid)
    &&& hex64(a.qsha)
    &&& wf_term(a.result) && ground(a.result) && no_dollar_var(a.result)
}

// Traces admit '$VAR'/1 as an ordinary compound: their writer runs
// without the numbervars option, so it prints structurally.
pub open spec fn wf_traces(t: TracesFile) -> bool {
    &&& name_ok(t.qid)
    &&& hex64(t.qsha)
    &&& hex64(t.asha)
    &&& wf_term(t.result) && ground(t.result)
}

pub open spec fn wf_v1(f: V1File) -> bool {
    match f {
        V1File::Doc(d) => wf_doc(d),
        V1File::Query(q) => wf_query(q),
        V1File::Answers(a) => wf_answers(a),
        V1File::Traces(t) => wf_traces(t),
    }
}

// --- the acceptance law ---

pub open spec fn accepts(bytes: Seq<u8>) -> bool {
    exists|f: V1File| #[trigger] wf_v1(f) && print_v1(f) == bytes
}

// --- exec-facing verdict (R9: the position payload is a deterministic
// diagnostic pinned by fixtures, outside the theorems) ---

pub enum EV1Verdict {
    Ok,
    Reject { line: u64, col: u64 },
}

impl EV1Verdict {
    pub open spec fn is_accept(&self) -> bool {
        match self {
            EV1Verdict::Ok => true,
            EV1Verdict::Reject { .. } => false,
        }
    }
}

} // verus!

use vstd::prelude::*;

verus! {

// Trusted spec: render-side align-TSV validator (contract m5u1 R3; legacy
// identity = tools/ui.py hl_parse_align, byte-exact on error details).
// Inputs are code-point sequences (UTF-8 decoding = shell boundary).
// Artifact rows: group<TAB>side<TAB>start<TAB>span; offsets/lengths count
// code points. `align_outcome` is the one function the kernel binding
// quotes; everything else defines it.

// --- primitive text helpers ---

pub open spec fn is_ascii_digit(c: char) -> bool {
    '0' <= c && c <= '9'
}

pub open spec fn all_ascii_digits(s: Seq<char>) -> bool {
    forall|i: int| #![auto] 0 <= i < s.len() ==> is_ascii_digit(s[i])
}

// Legacy rule: Python int(x) parses and str(int(x)) round-trips = exactly
// ASCII digits, nonempty, no leading zero except "0" itself. Unbounded.
pub open spec fn is_canonical_decimal(s: Seq<char>) -> bool {
    &&& s.len() > 0
    &&& all_ascii_digits(s)
    &&& (s.len() == 1 || s[0] != '0')
}

pub open spec fn digit_value(c: char) -> int {
    c as int - '0' as int
}

pub open spec fn dec_value(s: Seq<char>) -> int
    decreases s.len(),
{
    if s.len() == 0 {
        0
    } else {
        dec_value(s.drop_last()) * 10 + digit_value(s.last())
    }
}

pub open spec fn digit_char(d: int) -> char {
    if d == 0 { '0' } else if d == 1 { '1' } else if d == 2 { '2' }
    else if d == 3 { '3' } else if d == 4 { '4' } else if d == 5 { '5' }
    else if d == 6 { '6' } else if d == 7 { '7' } else if d == 8 { '8' }
    else { '9' }
}

// Canonical decimal rendering for row ordinals (n >= 1 in every use).
pub open spec fn dec_str(n: int) -> Seq<char>
    recommends n >= 0,
    decreases n,
{
    if n < 10 {
        seq![digit_char(n)]
    } else {
        dec_str(n / 10) + seq![digit_char(n % 10)]
    }
}

// Python str.split semantics: "" -> [""], trailing separator yields a
// trailing empty piece. Total, structural.
pub open spec fn split_at_seps(s: Seq<char>, sep: char) -> Seq<Seq<char>>
    decreases s.len(),
{
    if s.len() == 0 {
        seq![Seq::<char>::empty()]
    } else if s[0] == sep {
        seq![Seq::<char>::empty()] + split_at_seps(s.drop_first(), sep)
    } else {
        let rest = split_at_seps(s.drop_first(), sep);
        seq![seq![s[0]] + rest[0]] + rest.drop_first()
    }
}

// --- parsed rows ---

pub ghost struct RawSpan {
    pub start: int,
    pub end: int,
    pub group: int,
}

// Row-order spans of one side (side_name = "src" or "ace"); meaningful on
// row-check-clean files, total everywhere.
pub open spec fn side_spans(rows: Seq<Seq<char>>, side_name: Seq<char>) -> Seq<RawSpan>
    decreases rows.len(),
{
    if rows.len() == 0 {
        Seq::empty()
    } else {
        let fields = split_at_seps(rows[0], '\t');
        let rest = side_spans(rows.drop_first(), side_name);
        if fields.len() == 4 && fields[1] == side_name {
            seq![
                RawSpan {
                    start: dec_value(fields[2]),
                    end: dec_value(fields[2]) + fields[3].len(),
                    group: dec_value(fields[0]),
                },
            ] + rest
        } else {
            rest
        }
    }
}

pub open spec fn groups_of(spans: Seq<RawSpan>) -> Set<int>
    decreases spans.len(),
{
    if spans.len() == 0 {
        Set::empty()
    } else {
        groups_of(spans.drop_first()).insert(spans[0].group)
    }
}

// Two distinct span instances of one side intersect (spans are nonempty
// intervals, so touching ends never intersect; duplicates always do).
pub open spec fn has_overlap(spans: Seq<RawSpan>) -> bool {
    exists|i: int, j: int|
        #![auto]
        0 <= i < spans.len() && 0 <= j < spans.len() && i != j
            && spans[i].start < spans[j].end && spans[j].start < spans[i].end
}

// --- violations, legacy-ordered ---

pub ghost enum Violation {
    MissingTrailingNewline,
    EmptyFile,
    FieldCount { row: int },
    GroupCanonical { row: int },
    StartCanonical { row: int },
    EmptySpan { row: int },
    SideVocab { row: int },
    OutOfRange { row: int },
    SpanMismatch { row: int },
    NotBothSided,
    OverlapSrc,
    OverlapAce,
}

// In-row check order (legacy): field count -> group canonical -> start
// canonical -> empty span -> side vocabulary -> range -> text match.
pub open spec fn row_violation(row: Seq<char>, n: int, src: Seq<char>, ace: Seq<char>) -> Option<
    Violation,
> {
    let fields = split_at_seps(row, '\t');
    if fields.len() != 4 {
        Some(Violation::FieldCount { row: n })
    } else if !is_canonical_decimal(fields[0]) {
        Some(Violation::GroupCanonical { row: n })
    } else if !is_canonical_decimal(fields[2]) {
        Some(Violation::StartCanonical { row: n })
    } else if fields[3].len() == 0 {
        Some(Violation::EmptySpan { row: n })
    } else if fields[1] != "src"@ && fields[1] != "ace"@ {
        Some(Violation::SideVocab { row: n })
    } else {
        let text = if fields[1] == "src"@ {
            src
        } else {
            ace
        };
        let start = dec_value(fields[2]);
        let end = start + fields[3].len();
        if end > text.len() {
            Some(Violation::OutOfRange { row: n })
        } else if text.subrange(start, end) != fields[3] {
            Some(Violation::SpanMismatch { row: n })
        } else {
            None
        }
    }
}

pub open spec fn rows_violation(
    rows: Seq<Seq<char>>,
    n: int,
    src: Seq<char>,
    ace: Seq<char>,
) -> Option<Violation>
    decreases rows.len(),
{
    if rows.len() == 0 {
        None
    } else {
        match row_violation(rows[0], n, src, ace) {
            Some(v) => Some(v),
            None => rows_violation(rows.drop_first(), n + 1, src, ace),
        }
    }
}

// Whole-file first violation (legacy order): trailing newline -> empty file
// -> rows in physical order (1-based) -> groups both-sided -> src overlap
// -> ace overlap.
pub open spec fn first_violation(align: Seq<char>, src: Seq<char>, ace: Seq<char>) -> Option<
    Violation,
> {
    if align.len() == 0 || align.last() != '\n' {
        Some(Violation::MissingTrailingNewline)
    } else {
        let body = align.drop_last();
        if body.len() == 0 {
            Some(Violation::EmptyFile)
        } else {
            let rows = split_at_seps(body, '\n');
            match rows_violation(rows, 1, src, ace) {
                Some(v) => Some(v),
                None => {
                    let srcs = side_spans(rows, "src"@);
                    let aces = side_spans(rows, "ace"@);
                    if groups_of(srcs) != groups_of(aces) {
                        Some(Violation::NotBothSided)
                    } else if has_overlap(srcs) {
                        Some(Violation::OverlapSrc)
                    } else if has_overlap(aces) {
                        Some(Violation::OverlapAce)
                    } else {
                        None
                    }
                },
            }
        }
    }
}

pub open spec fn wellformed(align: Seq<char>, src: Seq<char>, ace: Seq<char>) -> bool {
    first_violation(align, src, ace) is None
}

// --- error rendering (byte-exact legacy details) ---

pub open spec fn row_prefix(n: int) -> Seq<char> {
    "row "@ + dec_str(n) + ": "@
}

pub open spec fn render(v: Violation) -> Seq<char> {
    match v {
        Violation::MissingTrailingNewline => "missing trailing newline"@,
        Violation::EmptyFile => "empty file"@,
        Violation::FieldCount { row } => row_prefix(row) + "expected 4 tab-separated fields"@,
        Violation::GroupCanonical { row } => row_prefix(row)
            + "group must be a canonical decimal"@,
        Violation::StartCanonical { row } => row_prefix(row)
            + "start must be a canonical decimal"@,
        Violation::EmptySpan { row } => row_prefix(row) + "empty span"@,
        Violation::SideVocab { row } => row_prefix(row) + "side must be src or ace"@,
        Violation::OutOfRange { row } => row_prefix(row) + "span out of range"@,
        Violation::SpanMismatch { row } => row_prefix(row)
            + "span does not match the text at start"@,
        Violation::NotBothSided => "every group needs both a src span and an ace span"@,
        Violation::OverlapSrc => "overlapping src spans"@,
        Violation::OverlapAce => "overlapping ace spans"@,
    }
}

// --- success model (legacy: dense display indexes by first ACE span in
// sorted (start,end,group) order; each side start-sorted) ---

pub ghost struct OutSpan {
    pub start: int,
    pub end: int,
    pub index: int,
}

pub ghost struct AlignModel {
    pub src: Seq<OutSpan>,
    pub ace: Seq<OutSpan>,
    pub count: int,
}

pub open spec fn raw_le(a: RawSpan, b: RawSpan) -> bool {
    ||| a.start < b.start
    ||| (a.start == b.start && a.end < b.end)
    ||| (a.start == b.start && a.end == b.end && a.group <= b.group)
}

pub open spec fn insert_raw(x: RawSpan, s: Seq<RawSpan>) -> Seq<RawSpan>
    decreases s.len(),
{
    if s.len() == 0 {
        seq![x]
    } else if raw_le(x, s[0]) {
        seq![x] + s
    } else {
        seq![s[0]] + insert_raw(x, s.drop_first())
    }
}

pub open spec fn sort_raw(s: Seq<RawSpan>) -> Seq<RawSpan>
    decreases s.len(),
{
    if s.len() == 0 {
        s
    } else {
        insert_raw(s[0], sort_raw(s.drop_first()))
    }
}

pub open spec fn out_le(a: OutSpan, b: OutSpan) -> bool {
    ||| a.start < b.start
    ||| (a.start == b.start && a.end < b.end)
    ||| (a.start == b.start && a.end == b.end && a.index <= b.index)
}

pub open spec fn insert_out(x: OutSpan, s: Seq<OutSpan>) -> Seq<OutSpan>
    decreases s.len(),
{
    if s.len() == 0 {
        seq![x]
    } else if out_le(x, s[0]) {
        seq![x] + s
    } else {
        seq![s[0]] + insert_out(x, s.drop_first())
    }
}

pub open spec fn sort_out(s: Seq<OutSpan>) -> Seq<OutSpan>
    decreases s.len(),
{
    if s.len() == 0 {
        s
    } else {
        insert_out(s[0], sort_out(s.drop_first()))
    }
}

// First-occurrence group order over a span sequence.
pub open spec fn groups_in_order(spans: Seq<RawSpan>, seen: Set<int>) -> Seq<int>
    decreases spans.len(),
{
    if spans.len() == 0 {
        Seq::empty()
    } else if seen.contains(spans[0].group) {
        groups_in_order(spans.drop_first(), seen)
    } else {
        seq![spans[0].group] + groups_in_order(spans.drop_first(), seen.insert(spans[0].group))
    }
}

pub open spec fn index_in(order: Seq<int>, g: int) -> int
    decreases order.len(),
{
    if order.len() == 0 {
        0
    } else if order[0] == g {
        0
    } else {
        1 + index_in(order.drop_first(), g)
    }
}

pub open spec fn to_out(spans: Seq<RawSpan>, order: Seq<int>) -> Seq<OutSpan>
    decreases spans.len(),
{
    if spans.len() == 0 {
        Seq::empty()
    } else {
        seq![
            OutSpan {
                start: spans[0].start,
                end: spans[0].end,
                index: index_in(order, spans[0].group),
            },
        ] + to_out(spans.drop_first(), order)
    }
}

// Defined for wellformed inputs (total everywhere).
pub open spec fn model_of(align: Seq<char>, src: Seq<char>, ace: Seq<char>) -> AlignModel {
    let rows = split_at_seps(align.drop_last(), '\n');
    let srcs = side_spans(rows, "src"@);
    let aces = side_spans(rows, "ace"@);
    let order = groups_in_order(sort_raw(aces), Set::empty());
    AlignModel {
        src: sort_out(to_out(srcs, order)),
        ace: sort_out(to_out(aces, order)),
        count: order.len() as int,
    }
}

// --- the one bound outcome ---

pub ghost enum AlignOutcome {
    Ok(AlignModel),
    Err(Seq<char>),
}

pub open spec fn align_outcome(align: Seq<char>, src: Seq<char>, ace: Seq<char>) -> AlignOutcome {
    match first_violation(align, src, ace) {
        Some(v) => AlignOutcome::Err(render(v)),
        None => AlignOutcome::Ok(model_of(align, src, ace)),
    }
}

// --- exec-facing result types (views bind exec results to the spec) ---

pub struct ESpan {
    pub start: u64,
    pub end: u64,
    pub index: u64,
}

impl View for ESpan {
    type V = OutSpan;

    open spec fn view(&self) -> OutSpan {
        OutSpan { start: self.start as int, end: self.end as int, index: self.index as int }
    }
}

pub struct EModel {
    pub src: Vec<ESpan>,
    pub ace: Vec<ESpan>,
    pub count: u64,
}

impl View for EModel {
    type V = AlignModel;

    open spec fn view(&self) -> AlignModel {
        AlignModel {
            src: self.src@.map_values(|e: ESpan| e@),
            ace: self.ace@.map_values(|e: ESpan| e@),
            count: self.count as int,
        }
    }
}

pub enum ECheck {
    Ok(EModel),
    Err(Vec<char>),
}

impl View for ECheck {
    type V = AlignOutcome;

    open spec fn view(&self) -> AlignOutcome {
        match self {
            ECheck::Ok(m) => AlignOutcome::Ok(m@),
            ECheck::Err(e) => AlignOutcome::Err(e@),
        }
    }
}

} // verus!

use vstd::prelude::*;
use crate::term::*;
use crate::v1text::*;
use crate::engine::*;

verus! {

// Trusted spec: aggregate obligation replay + recursion scan (contract
// m5u2 K2; legacy law ace_to_pl.pl aggregate_check_mode /
// recursion_check_mode). Every mode = one spec fn from inputs to the exact
// (rc, stdout, stderr) triple; stage order mirrors the legacy pipeline so
// the first reject wins identically. The shell owns file I/O: it hands the
// kernel one Src per file it read (manifest, every manifest cell, query).

// --- outputs ---

pub ghost struct Out { pub rc: int, pub out: Seq<u8>, pub err: Seq<u8> }

pub open spec fn atom(s: Seq<char>) -> Term { Term::Atom(ascii(s)) }

pub open spec fn error_line(class: Seq<char>, detail: Term) -> Seq<u8> {
    term_line(Term::Comp(ascii("ace_to_pl_error"@), seq![atom(class), detail]))
}

pub open spec fn reject(rc: int, class: Seq<char>, detail: Term) -> Out {
    Out { rc, out: Seq::empty(), err: error_line(class, detail) }
}

pub open spec fn check_load(detail: Term) -> Out { reject(2, "check_load"@, detail) }
pub open spec fn proof_fail(detail: Term) -> Out { reject(1, "proof"@, detail) }
pub open spec fn ok(out: Seq<u8>) -> Out { Out { rc: 0, out, err: Seq::empty() } }

pub open spec fn ctx(pred: Seq<char>, arity: int, arg: Term) -> Term {
    Term::Comp(ascii("context"@), seq![
        Term::Comp(ascii(":"@), seq![atom("ace_to_pl"@), Term::Comp(ascii("/"@), seq![atom(pred), Term::Int(arity)])]),
        arg,
    ])
}

// Manifest defects keep SWI's error/context wrapper verbatim (R18).
pub open spec fn manifest_reject(mpath: Seq<u8>, d: Term) -> Out {
    check_load(Term::Comp(ascii("error"@), seq![
        Term::Comp(ascii("aggregate_manifest"@), seq![d]),
        ctx("aggregate_read_manifest"@, 3, Term::Atom(mpath)),
    ]))
}

pub open spec fn utf8_reject(off: nat) -> Out {
    check_load(Term::Comp(ascii("error"@), seq![
        Term::Comp(ascii("syntax_error"@), seq![atom("invalid_utf8"@)]),
        ctx("read_utf8_input"@, 3, Term::Comp(ascii("byte_offset"@), seq![Term::Int(off as int)])),
    ]))
}

// --- manifest grammar: `<pl>TAB<payload>LF` rows, both fields nonempty, no other TAB ---

pub ghost struct MRow { pub pl: Seq<u8>, pub payload: Seq<u8> }

pub ghost enum MOut { Rows(Seq<MRow>), MissingLf, BadLine(Seq<u8>) }

pub open spec fn first_byte(bytes: Seq<u8>, b: u8, i: nat) -> nat
    decreases bytes.len() - i,
{
    if i >= bytes.len() { bytes.len() } else if bytes[i as int] == b { i } else { first_byte(bytes, b, i + 1) }
}

pub open spec fn lines_of(bytes: Seq<u8>) -> Seq<Seq<u8>>
    decreases bytes.len(),
{
    if bytes.len() == 0 { Seq::empty() } else {
        let n = first_byte(bytes, 0x0A, 0);
        if n >= bytes.len() { seq![bytes] } else { seq![bytes.take(n as int)] + lines_of(bytes.skip(n as int + 1)) }
    }
}

pub open spec fn entry(line: Seq<u8>) -> Option<MRow> {
    let t = first_byte(line, 0x09, 0);
    if 0 < t && t + 1 < line.len() && first_byte(line, 0x09, t + 1) >= line.len() {
        Option::Some(MRow { pl: line.take(t as int), payload: line.skip(t as int + 1) })
    } else { Option::None }
}

pub open spec fn rows_of(lines: Seq<Seq<u8>>) -> MOut
    decreases lines.len(),
{
    if lines.len() == 0 { MOut::Rows(Seq::empty()) } else {
        match entry(lines[0]) {
            Option::None => MOut::BadLine(lines[0]),
            Option::Some(r) => match rows_of(lines.drop_first()) {
                MOut::Rows(rs) => MOut::Rows(seq![r] + rs),
                other => other,
            },
        }
    }
}

pub open spec fn parse_manifest(bytes: Seq<u8>) -> MOut {
    if bytes.len() == 0 { MOut::Rows(Seq::empty()) }
    else if bytes.last() != 0x0A { MOut::MissingLf }
    else { rows_of(lines_of(bytes)) }
}

// --- sources: what the shell read for one manifest cell ---

pub ghost enum Src { Missing, Bad(nat), Bytes(Seq<u8>) }   // Bad = first invalid UTF-8 byte offset

pub open spec fn src_bytes(s: Src) -> Seq<u8> { match s { Src::Bytes(b) => b, _ => Seq::empty() } }

// Readability sweep over one manifest column, row order.
pub open spec fn first_missing(paths: Seq<Seq<u8>>, srcs: Seq<Src>, i: nat) -> Option<Seq<u8>>
    decreases paths.len() - i,
{
    if i >= paths.len() { Option::None }
    else if srcs[i as int] is Missing { Option::Some(paths[i as int]) }
    else { first_missing(paths, srcs, i + 1) }
}

// --- load model: consult reloads a repeated path, so each distinct pl path loads once ---

pub open spec fn unique_from(rows: Seq<MRow>, i: nat, acc: Seq<Seq<u8>>) -> Seq<Seq<u8>>
    decreases rows.len() - i,
{
    if i >= rows.len() { acc }
    else if acc.contains(rows[i as int].pl) { unique_from(rows, i + 1, acc) }
    else { unique_from(rows, i + 1, acc.push(rows[i as int].pl)) }
}

pub open spec fn unique_paths(rows: Seq<MRow>) -> Seq<Seq<u8>> { unique_from(rows, 0, Seq::empty()) }

pub open spec fn row_index(rows: Seq<MRow>, p: Seq<u8>, i: nat) -> nat
    decreases rows.len() - i,
{
    if i >= rows.len() || rows[i as int].pl == p { i } else { row_index(rows, p, i + 1) }
}

pub open spec fn src_of(rows: Seq<MRow>, pls: Seq<Src>, p: Seq<u8>) -> Src { pls[row_index(rows, p, 0) as int] }

pub open spec fn the_v1(bytes: Seq<u8>) -> V1File {
    choose|f: V1File| wf_v1(f) && print_v1(f) == bytes
}

// First distinct path whose bytes are not a canonical v1 file.
pub open spec fn first_bad_member(rows: Seq<MRow>, pls: Seq<Src>, ups: Seq<Seq<u8>>, i: nat) -> Option<Seq<u8>>
    decreases ups.len() - i,
{
    if i >= ups.len() { Option::None } else {
        let s = src_of(rows, pls, ups[i as int]);
        if s is Bytes && accepts(src_bytes(s)) { first_bad_member(rows, pls, ups, i + 1) } else { Option::Some(ups[i as int]) }
    }
}

pub open spec fn members(rows: Seq<MRow>, pls: Seq<Src>) -> Seq<V1File> {
    unique_paths(rows).map_values(|p: Seq<u8>| the_v1(src_bytes(src_of(rows, pls, p))))
}

pub open spec fn docs_of(ms: Seq<V1File>) -> Seq<DocFile>
    decreases ms.len(),
{
    if ms.len() == 0 { Seq::empty() } else {
        match ms[0] { V1File::Doc(d) => seq![d] + docs_of(ms.drop_first()), _ => docs_of(ms.drop_first()) }
    }
}

// The loaded program: per document its two records then every clause, in file order.
pub open spec fn doc_db(d: DocFile) -> Seq<DocClause> {
    seq![fact_clause(schema_version_term()), fact_clause(doc_record_term(d))]
        + d.bundles.map_values(|b: Bundle| b.clauses).flatten()
}

pub open spec fn db_of(docs: Seq<DocFile>) -> Seq<DocClause> { docs.map_values(|d: DocFile| doc_db(d)).flatten() }

// --- payload grammar (printer-first): '$guideline_proof'(D, S, variant(K), witness([..]), prove([..])) lines ---

pub ghost struct Ob { pub docid: Term, pub s: Term, pub k: Term, pub facts: Seq<Term>, pub heads: Seq<Term> }

pub open spec fn ob_term(o: Ob) -> Term {
    Term::Comp(ascii("$guideline_proof"@), seq![
        o.docid, o.s,
        Term::Comp(ascii("variant"@), seq![o.k]),
        Term::Comp(ascii("witness"@), seq![list_term(o.facts)]),
        Term::Comp(ascii("prove"@), seq![list_term(o.heads)]),
    ])
}

pub open spec fn wf_obs(obs: Seq<Ob>) -> bool {
    forall|i: int| 0 <= i < obs.len() ==> wf_term(ob_term(#[trigger] obs[i])) && ground(ob_term(obs[i])) && no_dollar_var(ob_term(obs[i]))
}

pub open spec fn print_payload(obs: Seq<Ob>) -> Seq<u8> {
    obs.map_values(|o: Ob| term_line(ob_term(o))).flatten()
}

pub open spec fn payload_accepts(bytes: Seq<u8>) -> bool {
    exists|obs: Seq<Ob>| #[trigger] wf_obs(obs) && print_payload(obs) == bytes
}

pub open spec fn the_payload(bytes: Seq<u8>) -> Seq<Ob> {
    choose|obs: Seq<Ob>| wf_obs(obs) && print_payload(obs) == bytes
}

pub open spec fn variant(o: Ob) -> Term { Term::Comp(ascii("variant"@), seq![o.k]) }

pub open spec fn first_empty(obs: Seq<Ob>) -> Option<Ob>
    decreases obs.len(),
{
    if obs.len() == 0 { Option::None }
    else if obs[0].heads.len() == 0 { Option::Some(obs[0]) }
    else { first_empty(obs.drop_first()) }
}

pub ghost enum Stage { Fail(Out), Obs(Seq<Ob>) }

// Payload rows in order: UTF-8, grammar, then empty obligations of that row.
pub open spec fn payload_stage(rows: Seq<MRow>, pys: Seq<Src>, i: nat, acc: Seq<Ob>) -> Stage
    decreases rows.len() - i,
{
    if i >= rows.len() { Stage::Obs(acc) } else {
        match pys[i as int] {
            Src::Bad(off) => Stage::Fail(utf8_reject(off)),
            s => if !payload_accepts(src_bytes(s)) {   // Missing cells never reach here (manifest_stage)
                Stage::Fail(check_load(Term::Comp(ascii("payload_term"@), seq![Term::Atom(rows[i as int].payload)])))
            } else {
                let obs = the_payload(src_bytes(s));
                match first_empty(obs) {
                    Option::Some(o) => Stage::Fail(proof_fail(Term::Comp(ascii("empty_obligation"@), seq![o.docid, o.s, variant(o)]))),
                    Option::None => payload_stage(rows, pys, i + 1, acc + obs),
                }
            },
        }
    }
}

// --- key laws over D-S-K in standard order ---

pub open spec fn pair(a: Term, b: Term) -> Term { Term::Comp(ascii("-"@), seq![a, b]) }
pub open spec fn key(o: Ob) -> Term { pair(pair(o.docid, o.s), o.k) }
pub open spec fn ds(o: Ob) -> Term { pair(o.docid, o.s) }

pub open spec fn first_dup(s: Seq<Term>) -> Option<Term>
    decreases s.len(),
{
    if s.len() < 2 { Option::None } else if s[0] == s[1] { Option::Some(s[0]) } else { first_dup(s.drop_first()) }
}

pub open spec fn arg(t: Term, i: int) -> Term {
    match t { Term::Comp(_, a) => if 0 <= i < a.len() { a[i] } else { Term::Nil }, _ => Term::Nil }
}

pub open spec fn is_sequence(ks: Seq<Term>, n: int) -> bool
    decreases ks.len(),
{
    ks.len() == 0 || (ks[0] == Term::Int(n) && is_sequence(ks.drop_first(), n + 1))
}

pub open spec fn variant_sequence(d: Term, ks: Seq<Term>) -> Term {
    Term::Comp(ascii("variant_sequence"@), seq![arg(d, 0), arg(d, 1), list_term(ks)])
}

// Walk sorted-unique keys; (d, ks) = the open D-S run with its variants so far.
pub open spec fn bad_run(keys: Seq<Term>, d: Term, ks: Seq<Term>) -> Option<Term>
    decreases keys.len(),
{
    if keys.len() == 0 || arg(keys[0], 0) != d {
        if !is_sequence(ks, 1) { Option::Some(variant_sequence(d, ks)) }
        else if keys.len() == 0 { Option::None }
        else { bad_run(keys.drop_first(), arg(keys[0], 0), seq![arg(keys[0], 1)]) }
    } else { bad_run(keys.drop_first(), d, ks.push(arg(keys[0], 1))) }
}

pub open spec fn first_bad_run(keys: Seq<Term>) -> Option<Term> {
    if keys.len() == 0 { Option::None } else { bad_run(keys.drop_first(), arg(keys[0], 0), seq![arg(keys[0], 1)]) }
}

pub open spec fn key_check(obs: Seq<Ob>) -> Option<Out> {
    let keys = obs.map_values(|o: Ob| key(o));
    match first_dup(msort(keys)) {
        Option::Some(k) => Option::Some(proof_fail(Term::Comp(ascii("duplicate_obligation"@),
            seq![arg(arg(k, 0), 0), arg(arg(k, 0), 1), Term::Comp(ascii("variant"@), seq![arg(k, 1)])]))),
        Option::None => match first_bad_run(sort_unique(keys)) {
            Option::Some(d) => Option::Some(proof_fail(d)),
            Option::None => Option::None,
        },
    }
}

// --- coverage: every '$guideline_id'(_, D, S, _, _) subterm of every loaded clause ---

pub open spec fn gid_name() -> Seq<u8> { ascii("$guideline_id"@) }

pub open spec fn gid_pairs(t: Term) -> Seq<Term>
    decreases t,
{
    match t {
        Term::Comp(name, args) =>
            (if name == gid_name() && args.len() == 5 && args[1] is Atom && args[2] is Int { seq![pair(args[1], args[2])] } else { Seq::empty() })
                + gid_pairs_all(args),
        _ => Seq::empty(),
    }
}

pub open spec fn gid_pairs_all(ts: Seq<Term>) -> Seq<Term>
    decreases ts,
{
    if ts.len() == 0 { Seq::empty() } else { gid_pairs(ts[0]) + gid_pairs_all(ts.drop_first()) }
}

pub open spec fn item_gids(it: BodyItem) -> Seq<Term> {
    match it { BodyItem::Pos(l) => gid_pairs(l), BodyItem::Naf(gs) => gid_pairs_all(gs) }
}

pub open spec fn clause_gids(c: DocClause) -> Seq<Term> {
    gid_pairs(c.head) + c.body.map_values(|it: BodyItem| item_gids(it)).flatten()
}

pub open spec fn first_absent(xs: Seq<Term>, ys: Seq<Term>) -> Option<Term>
    decreases xs.len(),
{
    if xs.len() == 0 { Option::None } else if !ys.contains(xs[0]) { Option::Some(xs[0]) } else { first_absent(xs.drop_first(), ys) }
}

pub open spec fn coverage_check(db: Seq<DocClause>, obs: Seq<Ob>) -> Option<Out> {
    let loaded = sort_unique(db.map_values(|c: DocClause| clause_gids(c)).flatten());
    let covered = sort_unique(obs.map_values(|o: Ob| ds(o)));
    match first_absent(loaded, covered) {
        Option::Some(p) => Option::Some(proof_fail(Term::Comp(ascii("missing_obligation"@), args_of(p)))),
        Option::None => match first_absent(covered, loaded) {
            Option::Some(p) => Option::Some(proof_fail(Term::Comp(ascii("extra_obligation"@), args_of(p)))),
            Option::None => Option::None,
        },
    }
}

// --- proof: each obligation's heads under its asserted witness facts ---

pub open spec fn first_unproved(db: Seq<DocClause>, obs: Seq<Ob>) -> Option<Ob>
    decreases obs.len(),
{
    if obs.len() == 0 { Option::None }
    else if !heads_proved(witness_db(obs[0].facts, db), obs[0].heads) { Option::Some(obs[0]) }
    else { first_unproved(db, obs.drop_first()) }
}

pub open spec fn agg_meter(rows: nat, obs: nat) -> Seq<u8> {
    ascii("ace_to_pl aggregate ok "@) + udec_bytes(rows) + ascii(" documents "@) + udec_bytes(obs) + ascii(" obligations\n"@)
}

pub open spec fn unreadable(mpath: Seq<u8>, p: Seq<u8>) -> Out {
    manifest_reject(mpath, Term::Comp(ascii("unreadable"@), seq![Term::Atom(p)]))
}

// Stage 1a: the manifest file itself.
pub open spec fn manifest_rows(mpath: Seq<u8>, manifest: Src) -> Result<Seq<MRow>, Out> {
    match manifest {
        Src::Missing => Result::Err(check_load(atom("unreadable"@))),
        Src::Bad(off) => Result::Err(utf8_reject(off)),
        Src::Bytes(b) => match parse_manifest(b) {
            MOut::MissingLf => Result::Err(manifest_reject(mpath, atom("missing_final_newline"@))),
            MOut::BadLine(l) => Result::Err(manifest_reject(mpath, Term::Comp(ascii("line"@), seq![Term::Atom(l)]))),
            MOut::Rows(rows) => Result::Ok(rows),
        },
    }
}

// Stage 1b: readability of every cell, row order, pl column first.
pub open spec fn manifest_stage(mpath: Seq<u8>, manifest: Src, pls: Seq<Src>, pys: Seq<Src>) -> Result<Seq<MRow>, Out> {
    match manifest_rows(mpath, manifest) {
        Result::Err(o) => Result::Err(o),
        Result::Ok(rows) => match first_missing(rows.map_values(|r: MRow| r.pl), pls, 0) {
            Option::Some(p) => Result::Err(unreadable(mpath, p)),
            Option::None => match first_missing(rows.map_values(|r: MRow| r.payload), pys, 0) {
                Option::Some(p) => Result::Err(unreadable(mpath, p)),
                Option::None => Result::Ok(rows),
            },
        },
    }
}

// Stage 2: consult every distinct pl path once; the first non-canonical member rejects.
pub open spec fn load_stage(rows: Seq<MRow>, pls: Seq<Src>) -> Result<Seq<DocFile>, Out> {
    match first_bad_member(rows, pls, unique_paths(rows), 0) {
        Option::Some(p) => Result::Err(check_load(Term::Comp(ascii("noncanonical"@), seq![Term::Atom(p)]))),
        Option::None => Result::Ok(docs_of(members(rows, pls))),
    }
}

// Stage 3 (aggregate + answer modes): document records = manifest rows as a
// bijection, counted in total and then by distinct docid.
pub open spec fn assertions(rows: Seq<MRow>, docs: Seq<DocFile>) -> Option<Out> {
    let expected = rows.len();
    let distinct = sort_unique(docs.map_values(|d: DocFile| Term::Atom(d.docid))).len();
    if docs.len() != expected {
        Option::Some(proof_fail(Term::Comp(ascii("document_records"@), seq![Term::Int(expected as int), Term::Int(docs.len() as int)])))
    } else if distinct != expected {
        Option::Some(proof_fail(Term::Comp(ascii("document_records"@), seq![Term::Int(expected as int), Term::Int(distinct as int)])))
    } else { Option::None }
}

pub open spec fn agg_body(rows: Seq<MRow>, docs: Seq<DocFile>, pys: Seq<Src>) -> Out {
    match payload_stage(rows, pys, 0, Seq::empty()) {
        Stage::Fail(o) => o,
        Stage::Obs(obs) => {
            let db = db_of(docs);
            match key_check(obs) {
                Option::Some(o) => o,
                Option::None => match coverage_check(db, obs) {
                    Option::Some(o) => o,
                    Option::None => match first_unproved(db, obs) {
                        Option::Some(ob) => proof_fail(Term::Comp(ascii("obligation_failed"@), seq![ob.docid, ob.s, variant(ob)])),
                        Option::None => ok(agg_meter(rows.len(), obs.len())),
                    },
                },
            }
        },
    }
}

pub open spec fn agg_output(mpath: Seq<u8>, manifest: Src, pls: Seq<Src>, pys: Seq<Src>) -> Out {
    match manifest_stage(mpath, manifest, pls, pys) {
        Result::Err(o) => o,
        Result::Ok(rows) => if rows.len() == 0 { ok(agg_meter(0, 0)) } else {
            match load_stage(rows, pls) {
                Result::Err(o) => o,
                Result::Ok(docs) => match assertions(rows, docs) {
                    Option::Some(o) => o,
                    Option::None => agg_body(rows, docs, pys),
                },
            }
        },
    }
}

// --- recursion scan: rule clauses indicator-major, then clause order ---

pub open spec fn indicator_rules(cs: Seq<DocClause>, i: int) -> Seq<DocClause> {
    cs.filter(|c: DocClause| lit_fa(c.head) == Option::Some(indicator(i)) && c.body.len() > 0)
}

pub open spec fn rules(cs: Seq<DocClause>) -> Seq<DocClause> {
    Seq::new(9, |i: int| indicator_rules(cs, i)).flatten()
}

pub open spec fn leftmost(c: DocClause) -> Term {
    match c.body[0] { BodyItem::Pos(l) => l, BodyItem::Naf(gs) => gs[0] }
}

pub open spec fn site(c: DocClause) -> Term {
    let gs = clause_gids(c);
    if gs.len() == 0 { atom("unattributed"@) } else { Term::Comp(ascii("sentence"@), args_of(gs[0])) }
}

pub open spec fn first_left_recursive(rs: Seq<DocClause>) -> Option<DocClause>
    decreases rs.len(),
{
    if rs.len() == 0 { Option::None }
    else if unifiable_apart(leftmost(rs[0]), rs[0].head) { Option::Some(rs[0]) }
    else { first_left_recursive(rs.drop_first()) }
}

pub open spec fn rec_meter(rows: nat, rules: nat) -> Seq<u8> {
    ascii("ace_to_pl recursion ok "@) + udec_bytes(rows) + ascii(" documents "@) + udec_bytes(rules) + ascii(" rule clauses\n"@)
}

pub open spec fn recursion_output(mpath: Seq<u8>, manifest: Src, pls: Seq<Src>, pys: Seq<Src>) -> Out {
    match manifest_stage(mpath, manifest, pls, pys) {
        Result::Err(o) => o,
        Result::Ok(rows) => if rows.len() == 0 { ok(rec_meter(0, 0)) } else {
            match load_stage(rows, pls) {
                Result::Err(o) => o,
                Result::Ok(docs) => {
                    let rs = rules(db_of(docs));
                    match first_left_recursive(rs) {
                        Option::Some(c) => match c.head {   // heads are compounds (wf_literal)
                            Term::Comp(name, args) => proof_fail(Term::Comp(ascii("left_recursive"@), seq![site(c), Term::Atom(name), Term::Int(args.len() as int)])),
                            _ => ok(rec_meter(rows.len(), rs.len())),
                        },
                        Option::None => ok(rec_meter(rows.len(), rs.len())),
                    }
                },
            }
        },
    }
}

// --- exec-facing mirrors (shell/kernel protocol) ---

pub enum ESrc { Missing, Bad(usize), Bytes(Vec<u8>) }

impl View for ESrc {
    type V = Src;
    open spec fn view(&self) -> Src {
        match self { ESrc::Missing => Src::Missing, ESrc::Bad(o) => Src::Bad(*o as nat), ESrc::Bytes(b) => Src::Bytes(b@) }
    }
}

pub struct ERow { pub pl: Vec<u8>, pub payload: Vec<u8> }

impl View for ERow {
    type V = MRow;
    open spec fn view(&self) -> MRow { MRow { pl: self.pl@, payload: self.payload@ } }
}

pub struct EOut { pub rc: u8, pub out: Vec<u8>, pub err: Vec<u8> }

impl View for EOut {
    type V = Out;
    open spec fn view(&self) -> Out { Out { rc: self.rc as int, out: self.out@, err: self.err@ } }
}

pub open spec fn srcs(v: Seq<ESrc>) -> Seq<Src> { v.map_values(|s: ESrc| s@) }

pub open spec fn rows_view(r: Result<Vec<ERow>, EOut>) -> Result<Seq<MRow>, Out> {
    match r { Result::Ok(rows) => Result::Ok(rows@.map_values(|e: ERow| e@)), Result::Err(o) => Result::Err(o@) }
}

// The shell reads one cell per manifest row and column before calling a mode.
pub open spec fn cells_ok(manifest: Src, pls: Seq<Src>, pys: Seq<Src>) -> bool {
    match parse_manifest(src_bytes(manifest)) { MOut::Rows(rows) => pls.len() == rows.len() && pys.len() == rows.len(), _ => true }
}

} // verus!

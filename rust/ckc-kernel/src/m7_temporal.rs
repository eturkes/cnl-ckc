use crate::k4_bytes::{append, copy, eq, split, starts_with};
#[cfg(verus_keep_ghost)]
use crate::m7_supersession::sup_rows;
use crate::m7_supersession::{ESuperseding, clone_rows};
use ckc_spec::check::*;
#[cfg(verus_keep_ghost)]
use ckc_spec::v1text::{all_in, ascii, udec_bytes};
use ckc_spec::vocabulary::*;
use vstd::prelude::*;
use vstd::slice::slice_subrange;

verus! {

// m7t D1 + q12 D1 + c1 D4: the vocabulary.tsv grammar, parsed once for
// `ckc check` and for the annotating projection.
pub struct EVocabulary {
    pub units: Vec<(Vec<u8>, Vec<u8>)>,
    pub relations: Vec<(Vec<u8>, Vec<u8>)>,
    pub spacings: Vec<(Vec<u8>, Vec<u8>)>,
    pub windows: Vec<(Vec<u8>, Vec<u8>)>,
    pub frequencies: Vec<Vec<u8>>,
    pub approximations: Vec<Vec<u8>>,
    pub ranges: Vec<Vec<u8>>,
    pub strengths: Vec<(Vec<u8>, Vec<u8>)>,
    pub evidences: Vec<(Vec<u8>, Vec<u8>)>,
    pub preferences: Vec<Vec<u8>>,
    pub alternatives: Vec<Vec<u8>>,
    pub qualifiers: Vec<(Vec<u8>, Vec<u8>)>,
    pub scopes: Vec<(Vec<u8>, Vec<u8>)>,
    pub exclusions: Vec<Vec<u8>>,
    pub supersessions: Vec<(Vec<u8>, ESuperseding)>,
    pub superseded: Vec<(Vec<u8>, ESuperseding)>,
}

pub open spec fn pairs_view(v: Seq<(Vec<u8>, Vec<u8>)>) -> Seq<(Seq<u8>, Seq<u8>)> {
    v.map_values(|p: (Vec<u8>, Vec<u8>)| (p.0@, p.1@))
}

pub open spec fn lemmas_view(v: Seq<Vec<u8>>) -> Seq<Seq<u8>> {
    v.map_values(|x: Vec<u8>| x@)
}

impl View for EVocabulary {
    type V = Vocabulary;

    open spec fn view(&self) -> Vocabulary {
        Vocabulary {
            units: pairs_view(self.units@),
            relations: pairs_view(self.relations@),
            spacings: pairs_view(self.spacings@),
            windows: pairs_view(self.windows@),
            frequencies: lemmas_view(self.frequencies@),
            approximations: lemmas_view(self.approximations@),
            ranges: lemmas_view(self.ranges@),
            strengths: pairs_view(self.strengths@),
            evidences: pairs_view(self.evidences@),
            preferences: lemmas_view(self.preferences@),
            alternatives: lemmas_view(self.alternatives@),
            qualifiers: pairs_view(self.qualifiers@),
            scopes: pairs_view(self.scopes@),
            exclusions: lemmas_view(self.exclusions@),
            supersessions: sup_rows(self.supersessions@),
            superseded: sup_rows(self.superseded@),
        }
    }
}

// Every literal this module emits equals its spec spelling.
proof fn literals()
    ensures
        b"field count"@ == ascii("field count"@),
        b"lemma"@ == ascii("lemma"@),
        b"unit"@ == ascii("unit"@),
        b"unit id"@ == ascii("unit id"@),
        b"duplicate noun"@ == ascii("duplicate noun"@),
        b"relation"@ == ascii("relation"@),
        b"role id"@ == ascii("role id"@),
        b"duplicate preposition"@ == ascii("duplicate preposition"@),
        b"spacing"@ == ascii("spacing"@),
        b"frame lemma"@ == ascii("frame lemma"@),
        b"kind"@ == ascii("kind"@),
        b"header"@ == ascii("header"@),
        b"no rows"@ == ascii("no rows"@),
        b"final newline"@ == ascii("final newline"@),
        b" at row "@ == ascii(" at row "@),
        b"ckc: vocabulary ok "@ == ascii("ckc: vocabulary ok "@),
        b"comment"@ == ascii("comment"@),
        b" "@ == ascii(" "@),
        b" rows\n"@ == ascii(" rows\n"@),
        b": "@ == ascii(": "@),
        b"vocabulary"@ == ascii("vocabulary"@),
        b"window"@ == ascii("window"@),
        b"frequency"@ == ascii("frequency"@),
        b"frequency value"@ == ascii("frequency value"@),
        b"period"@ == ascii("period"@),
        b"approximation"@ == ascii("approximation"@),
        b"approximation value"@ == ascii("approximation value"@),
        b"about"@ == ascii("about"@),
        b"duplicate adjective"@ == ascii("duplicate adjective"@),
        b"range"@ == ascii("range"@),
        b"range value"@ == ascii("range value"@),
        b"minimum"@ == ascii("minimum"@),
        b"strength"@ == ascii("strength"@),
        b"strength value"@ == ascii("strength value"@),
        b"evidence"@ == ascii("evidence"@),
        b"evidence value"@ == ascii("evidence value"@),
        b"preference"@ == ascii("preference"@),
        b"preference value"@ == ascii("preference value"@),
        b"preferred"@ == ascii("preferred"@),
        b"alternative"@ == ascii("alternative"@),
        b"alternative value"@ == ascii("alternative value"@),
        b"qualifier"@ == ascii("qualifier"@),
        b"qualifier value"@ == ascii("qualifier value"@),
        b"duplicate qualifier"@ == ascii("duplicate qualifier"@),
{
    reveal(ascii);
    reveal_byteslit(b"qualifier");
    reveal_strlit("qualifier");
    assert(b"qualifier"@ =~= ascii("qualifier"@));
    reveal_byteslit(b"qualifier value");
    reveal_strlit("qualifier value");
    assert(b"qualifier value"@ =~= ascii("qualifier value"@));
    reveal_byteslit(b"duplicate qualifier");
    reveal_strlit("duplicate qualifier");
    assert(b"duplicate qualifier"@ =~= ascii("duplicate qualifier"@));
    reveal_byteslit(b"strength");
    reveal_strlit("strength");
    reveal_byteslit(b"strength value");
    reveal_strlit("strength value");
    reveal_byteslit(b"evidence");
    reveal_strlit("evidence");
    reveal_byteslit(b"evidence value");
    reveal_strlit("evidence value");
    reveal_byteslit(b"preference");
    reveal_strlit("preference");
    reveal_byteslit(b"preference value");
    reveal_strlit("preference value");
    reveal_byteslit(b"preferred");
    reveal_strlit("preferred");
    reveal_byteslit(b"alternative");
    reveal_strlit("alternative");
    reveal_byteslit(b"alternative value");
    reveal_strlit("alternative value");
    reveal(ascii);
    assert(b"strength"@ =~= ascii("strength"@));
    assert(b"strength value"@ =~= ascii("strength value"@));
    assert(b"evidence"@ =~= ascii("evidence"@));
    assert(b"evidence value"@ =~= ascii("evidence value"@));
    assert(b"preference"@ =~= ascii("preference"@));
    assert(b"preference value"@ =~= ascii("preference value"@));
    assert(b"preferred"@ =~= ascii("preferred"@));
    assert(b"alternative"@ =~= ascii("alternative"@));
    assert(b"alternative value"@ =~= ascii("alternative value"@));
    reveal_byteslit(b"approximation");
    reveal_strlit("approximation");
    reveal_byteslit(b"approximation value");
    reveal_strlit("approximation value");
    reveal_byteslit(b"about");
    reveal_strlit("about");
    reveal_byteslit(b"duplicate adjective");
    reveal_strlit("duplicate adjective");
    reveal_byteslit(b"range");
    reveal_strlit("range");
    reveal_byteslit(b"range value");
    reveal_strlit("range value");
    reveal_byteslit(b"minimum");
    reveal_strlit("minimum");
    reveal(ascii);
    assert(b"approximation"@ =~= ascii("approximation"@));
    assert(b"approximation value"@ =~= ascii("approximation value"@));
    assert(b"about"@ =~= ascii("about"@));
    assert(b"duplicate adjective"@ =~= ascii("duplicate adjective"@));
    assert(b"range"@ =~= ascii("range"@));
    assert(b"range value"@ =~= ascii("range value"@));
    assert(b"minimum"@ =~= ascii("minimum"@));
    reveal_byteslit(b"window");
    reveal_strlit("window");
    reveal_byteslit(b"frequency");
    reveal_strlit("frequency");
    reveal_byteslit(b"frequency value");
    reveal_strlit("frequency value");
    reveal_byteslit(b"period");
    reveal_strlit("period");
    reveal(ascii);
    assert(b"window"@ =~= ascii("window"@));
    assert(b"frequency"@ =~= ascii("frequency"@));
    assert(b"frequency value"@ =~= ascii("frequency value"@));
    assert(b"period"@ =~= ascii("period"@));
    reveal(ascii);
    reveal_byteslit(b"field count");
    reveal_strlit("field count");
    assert(b"field count"@ =~= ascii("field count"@));
    reveal_byteslit(b"lemma");
    reveal_strlit("lemma");
    assert(b"lemma"@ =~= ascii("lemma"@));
    reveal_byteslit(b"unit");
    reveal_strlit("unit");
    assert(b"unit"@ =~= ascii("unit"@));
    reveal_byteslit(b"unit id");
    reveal_strlit("unit id");
    assert(b"unit id"@ =~= ascii("unit id"@));
    reveal_byteslit(b"duplicate noun");
    reveal_strlit("duplicate noun");
    assert(b"duplicate noun"@ =~= ascii("duplicate noun"@));
    reveal_byteslit(b"relation");
    reveal_strlit("relation");
    assert(b"relation"@ =~= ascii("relation"@));
    reveal_byteslit(b"role id");
    reveal_strlit("role id");
    assert(b"role id"@ =~= ascii("role id"@));
    reveal_byteslit(b"duplicate preposition");
    reveal_strlit("duplicate preposition");
    assert(b"duplicate preposition"@ =~= ascii("duplicate preposition"@));
    reveal_byteslit(b"spacing");
    reveal_strlit("spacing");
    assert(b"spacing"@ =~= ascii("spacing"@));
    reveal_byteslit(b"frame lemma");
    reveal_strlit("frame lemma");
    assert(b"frame lemma"@ =~= ascii("frame lemma"@));
    reveal_byteslit(b"kind");
    reveal_strlit("kind");
    assert(b"kind"@ =~= ascii("kind"@));
    reveal_byteslit(b"header");
    reveal_strlit("header");
    assert(b"header"@ =~= ascii("header"@));
    reveal_byteslit(b"no rows");
    reveal_strlit("no rows");
    assert(b"no rows"@ =~= ascii("no rows"@));
    reveal_byteslit(b"final newline");
    reveal_strlit("final newline");
    assert(b"final newline"@ =~= ascii("final newline"@));
    reveal_byteslit(b" at row ");
    reveal_strlit(" at row ");
    assert(b" at row "@ =~= ascii(" at row "@));
    reveal_byteslit(b"ckc: vocabulary ok ");
    reveal_strlit("ckc: vocabulary ok ");
    assert(b"ckc: vocabulary ok "@ =~= ascii("ckc: vocabulary ok "@));
    reveal_byteslit(b"comment");
    reveal_strlit("comment");
    assert(b"comment"@ =~= ascii("comment"@));
    reveal_byteslit(b" ");
    reveal_strlit(" ");
    assert(b" "@ =~= ascii(" "@));
    reveal_byteslit(b" rows\n");
    reveal_strlit(" rows\n");
    assert(b" rows\n"@ =~= ascii(" rows\n"@));
    reveal_byteslit(b": ");
    reveal_strlit(": ");
    assert(b": "@ =~= ascii(": "@));
    reveal_byteslit(b"vocabulary");
    reveal_strlit("vocabulary");
    assert(b"vocabulary"@ =~= ascii("vocabulary"@));
}

proof fn scope_literals()
    ensures
        b"scope"@ == ascii("scope"@),
        b"scope value"@ == ascii("scope value"@),
        b"duplicate scope"@ == ascii("duplicate scope"@),
        b"exclusion"@ == ascii("exclusion"@),
        b"exclusion value"@ == ascii("exclusion value"@),
        b"duplicate exclusion"@ == ascii("duplicate exclusion"@),
        b"scope without exclusion at row "@ == ascii("scope without exclusion at row "@),
        b"exclusion without scope at row "@ == ascii("exclusion without scope at row "@),
        b"scope key at row "@ == ascii("scope key at row "@),
        b"guidelines/"@ == ascii("guidelines/"@),
        b"/vocabulary.tsv"@ == ascii("/vocabulary.tsv"@),
{
    reveal(ascii);
    reveal_byteslit(b"scope");
    reveal_strlit("scope");
    assert(b"scope"@ =~= ascii("scope"@));
    reveal_byteslit(b"scope value");
    reveal_strlit("scope value");
    assert(b"scope value"@ =~= ascii("scope value"@));
    reveal_byteslit(b"duplicate scope");
    reveal_strlit("duplicate scope");
    assert(b"duplicate scope"@ =~= ascii("duplicate scope"@));
    reveal_byteslit(b"exclusion");
    reveal_strlit("exclusion");
    assert(b"exclusion"@ =~= ascii("exclusion"@));
    reveal_byteslit(b"exclusion value");
    reveal_strlit("exclusion value");
    assert(b"exclusion value"@ =~= ascii("exclusion value"@));
    reveal_byteslit(b"duplicate exclusion");
    reveal_strlit("duplicate exclusion");
    assert(b"duplicate exclusion"@ =~= ascii("duplicate exclusion"@));
    reveal_byteslit(b"scope without exclusion at row ");
    reveal_strlit("scope without exclusion at row ");
    assert(b"scope without exclusion at row "@ =~= ascii("scope without exclusion at row "@));
    reveal_byteslit(b"exclusion without scope at row ");
    reveal_strlit("exclusion without scope at row ");
    assert(b"exclusion without scope at row "@ =~= ascii("exclusion without scope at row "@));
    reveal_byteslit(b"scope key at row ");
    reveal_strlit("scope key at row ");
    assert(b"scope key at row "@ =~= ascii("scope key at row "@));
    reveal_byteslit(b"guidelines/");
    reveal_strlit("guidelines/");
    assert(b"guidelines/"@ =~= ascii("guidelines/"@));
    reveal_byteslit(b"/vocabulary.tsv");
    reveal_strlit("/vocabulary.tsv");
    assert(b"/vocabulary.tsv"@ =~= ascii("/vocabulary.tsv"@));
}

pub fn lemma_ok_exec(l: &[u8]) -> (r: bool)
    ensures
        r == lemma_ok(l@),
{
    if l.len() == 0 {
        return false;
    }
    let mut i = 0usize;
    while i < l.len()
        invariant
            i <= l@.len(),
            forall|k: int| 0 <= k < i ==> #[trigger] l@[k] > 0x20 && l@[k] != 0x7F,
        decreases l@.len() - i,
    {
        if l[i] <= 0x20 || l[i] == 0x7F {
            proof {
                if all_in(l@, |b: u8| b > 0x20 && b != 0x7F) {
                    assert(l@[i as int] > 0x20 && l@[i as int] != 0x7F);
                }
            }
            return false;
        }
        i += 1;
    }
    proof {
        assert(all_in(l@, |b: u8| b > 0x20 && b != 0x7F));
    }
    crate::k5_utf8::valid(l)
}

pub fn clone_pairs(v: &Vec<(Vec<u8>, Vec<u8>)>) -> (r: Vec<(Vec<u8>, Vec<u8>)>)
    ensures
        pairs_view(r@) == pairs_view(v@),
{
    let mut r = Vec::new();
    let mut i = 0usize;
    while i < v.len()
        invariant
            i <= v@.len(),
            pairs_view(r@) == pairs_view(v@).take(i as int),
        decreases v@.len() - i,
    {
        let ghost before = r@;
        let a = copy(&v[i].0);
        let b = copy(&v[i].1);
        r.push((a, b));
        proof {
            assert(pairs_view(before).len() == before.len());
            assert(before.len() == i);
            assert(r@ == before.push((a, b)));
            assert forall|j: int| 0 <= j < i + 1 implies #[trigger] pairs_view(r@)[j] == pairs_view(
                v@,
            ).take(i + 1)[j] by {
                if j < i {
                    assert(r@[j] == before[j]);
                    assert(pairs_view(before)[j] == pairs_view(v@).take(i as int)[j]);
                }
            }
            assert(pairs_view(r@) =~= pairs_view(v@).take(i + 1));
        }
        i += 1;
    }
    proof {
        assert(pairs_view(v@).take(v@.len() as int) =~= pairs_view(v@));
    }
    r
}

// keys(rows).contains(k)
pub fn key_in(rows: &Vec<(Vec<u8>, Vec<u8>)>, k: &[u8]) -> (r: bool)
    ensures
        r == keys(pairs_view(rows@)).contains(k@),
{
    let mut i = 0usize;
    while i < rows.len()
        invariant
            i <= rows@.len(),
            forall|j: int| 0 <= j < i ==> #[trigger] rows@[j].0@ != k@,
        decreases rows@.len() - i,
    {
        if eq(&rows[i].0, k) {
            proof {
                assert(keys(pairs_view(rows@))[i as int] == k@);
            }
            return true;
        }
        i += 1;
    }
    proof {
        if keys(pairs_view(rows@)).contains(k@) {
            let j = choose|j: int|
                0 <= j < keys(pairs_view(rows@)).len() && keys(pairs_view(rows@))[j] == k@;
            assert(rows@[j].0@ != k@);
        }
    }
    false
}

// frames: the second components contain k
pub fn snd_in(rows: &Vec<(Vec<u8>, Vec<u8>)>, k: &[u8]) -> (r: bool)
    ensures
        r == pairs_view(rows@).map_values(|p: (Seq<u8>, Seq<u8>)| p.1).contains(k@),
{
    let ghost s = pairs_view(rows@).map_values(|p: (Seq<u8>, Seq<u8>)| p.1);
    let mut i = 0usize;
    while i < rows.len()
        invariant
            i <= rows@.len(),
            s == pairs_view(rows@).map_values(|p: (Seq<u8>, Seq<u8>)| p.1),
            forall|j: int| 0 <= j < i ==> #[trigger] rows@[j].1@ != k@,
        decreases rows@.len() - i,
    {
        if eq(&rows[i].1, k) {
            proof {
                assert(s[i as int] == k@);
            }
            return true;
        }
        i += 1;
    }
    proof {
        if s.contains(k@) {
            let j = choose|j: int| 0 <= j < s.len() && s[j] == k@;
            assert(rows@[j].1@ != k@);
        }
    }
    false
}

pub fn pair_in(rows: &Vec<(Vec<u8>, Vec<u8>)>, a: &[u8], b: &[u8]) -> (r: bool)
    ensures
        r == pairs_view(rows@).contains((a@, b@)),
{
    let mut i = 0usize;
    while i < rows.len()
        invariant
            i <= rows@.len(),
            forall|j: int| 0 <= j < i ==> #[trigger] pairs_view(rows@)[j] != (a@, b@),
        decreases rows@.len() - i,
    {
        if eq(&rows[i].0, a) && eq(&rows[i].1, b) {
            proof {
                assert(pairs_view(rows@)[i as int] == (a@, b@));
            }
            return true;
        }
        i += 1;
    }
    proof {
        if pairs_view(rows@).contains((a@, b@)) {
            let j = choose|j: int|
                0 <= j < pairs_view(rows@).len() && pairs_view(rows@)[j] == (a@, b@);
            assert(pairs_view(rows@)[j] != (a@, b@));
        }
    }
    false
}

pub fn unit_id_ok(v: &[u8]) -> (r: bool)
    ensures
        r == unit_ids().contains(v@),
{
    let r = eq(v, b"second") || eq(v, b"minute") || eq(v, b"hour") || eq(v, b"day") || eq(
        v,
        b"week",
    ) || eq(v, b"month") || eq(v, b"year");
    proof {
        reveal_byteslit(b"second");
        reveal_byteslit(b"minute");
        reveal_byteslit(b"hour");
        reveal_byteslit(b"day");
        reveal_byteslit(b"week");
        reveal_byteslit(b"month");
        reveal_byteslit(b"year");
        reveal_strlit("second");
        reveal_strlit("minute");
        reveal_strlit("hour");
        reveal_strlit("day");
        reveal_strlit("week");
        reveal_strlit("month");
        reveal_strlit("year");
        reveal(ascii);
        let u = unit_ids();
        assert(u.len() == 7);
        assert(u[0] =~= b"second"@);
        assert(u[1] =~= b"minute"@);
        assert(u[2] =~= b"hour"@);
        assert(u[3] =~= b"day"@);
        assert(u[4] =~= b"week"@);
        assert(u[5] =~= b"month"@);
        assert(u[6] =~= b"year"@);
        if r {
            if v@ == u[0] {
                assert(u.contains(v@));
            } else if v@ == u[1] {
                assert(u.contains(v@));
            } else if v@ == u[2] {
                assert(u.contains(v@));
            } else if v@ == u[3] {
                assert(u.contains(v@));
            } else if v@ == u[4] {
                assert(u.contains(v@));
            } else if v@ == u[5] {
                assert(u.contains(v@));
            } else {
                assert(u.contains(v@));
            }
        }
    }
    r
}

pub fn qualifier_id_ok(v: &[u8]) -> (r: bool)
    ensures
        r == qualifier_ids().contains(v@),
{
    let r = eq(v, b"usuality") || eq(v, b"ideality") || eq(v, b"focus") || eq(v, b"manner") || eq(
        v,
        b"force",
    ) || eq(v, b"possibility") || eq(v, b"applicability");
    proof {
        reveal_byteslit(b"usuality");
        reveal_byteslit(b"ideality");
        reveal_byteslit(b"focus");
        reveal_byteslit(b"manner");
        reveal_byteslit(b"force");
        reveal_byteslit(b"possibility");
        reveal_byteslit(b"applicability");
        reveal_strlit("usuality");
        reveal_strlit("ideality");
        reveal_strlit("focus");
        reveal_strlit("manner");
        reveal_strlit("force");
        reveal_strlit("possibility");
        reveal_strlit("applicability");
        reveal(ascii);
        let u = qualifier_ids();
        assert(u.len() == 7);
        assert(u[0] =~= b"usuality"@);
        assert(u[1] =~= b"ideality"@);
        assert(u[2] =~= b"focus"@);
        assert(u[3] =~= b"manner"@);
        assert(u[4] =~= b"force"@);
        assert(u[5] =~= b"possibility"@);
        assert(u[6] =~= b"applicability"@);
        if r {
            if v@ == u[0] {
                assert(u.contains(v@));
            } else if v@ == u[1] {
                assert(u.contains(v@));
            } else if v@ == u[2] {
                assert(u.contains(v@));
            } else if v@ == u[3] {
                assert(u.contains(v@));
            } else if v@ == u[4] {
                assert(u.contains(v@));
            } else if v@ == u[5] {
                assert(u.contains(v@));
            } else {
                assert(u.contains(v@));
            }
        }
    }
    r
}

pub fn role_id_ok(v: &[u8]) -> (r: bool)
    ensures
        r == role_ids().contains(v@),
{
    let r = eq(v, b"duration") || eq(v, b"within") || eq(v, b"after") || eq(v, b"before");
    proof {
        reveal_byteslit(b"duration");
        reveal_byteslit(b"within");
        reveal_byteslit(b"after");
        reveal_byteslit(b"before");
        reveal_strlit("duration");
        reveal_strlit("within");
        reveal_strlit("after");
        reveal_strlit("before");
        reveal(ascii);
        let u = role_ids();
        assert(u.len() == 4);
        assert(u[0] =~= b"duration"@);
        assert(u[1] =~= b"within"@);
        assert(u[2] =~= b"after"@);
        assert(u[3] =~= b"before"@);
        if r {
            if v@ == u[0] {
                assert(u.contains(v@));
            } else if v@ == u[1] {
                assert(u.contains(v@));
            } else if v@ == u[2] {
                assert(u.contains(v@));
            } else {
                assert(u.contains(v@));
            }
        }
    }
    r
}

pub fn strength_id_ok(v: &[u8]) -> (r: bool)
    ensures
        r == strength_ids().contains(v@),
{
    let r = eq(v, b"strong") || eq(v, b"conditional");
    proof {
        reveal_byteslit(b"strong");
        reveal_byteslit(b"conditional");
        reveal_strlit("strong");
        reveal_strlit("conditional");
        reveal(ascii);
        let u = strength_ids();
        assert(u.len() == 2);
        assert(u[0] =~= b"strong"@);
        assert(u[1] =~= b"conditional"@);
        if r {
            if v@ == u[0] {
                assert(u.contains(v@));
            } else {
                assert(u.contains(v@));
            }
        }
    }
    r
}

pub fn evidence_id_ok(v: &[u8]) -> (r: bool)
    ensures
        r == evidence_ids().contains(v@),
{
    let r = eq(v, b"high") || eq(v, b"moderate") || eq(v, b"low") || eq(v, b"very_low");
    proof {
        reveal_byteslit(b"high");
        reveal_byteslit(b"moderate");
        reveal_byteslit(b"low");
        reveal_byteslit(b"very_low");
        reveal_strlit("high");
        reveal_strlit("moderate");
        reveal_strlit("low");
        reveal_strlit("very_low");
        reveal(ascii);
        let u = evidence_ids();
        assert(u.len() == 4);
        assert(u[0] =~= b"high"@);
        assert(u[1] =~= b"moderate"@);
        assert(u[2] =~= b"low"@);
        assert(u[3] =~= b"very_low"@);
        if r {
            if v@ == u[0] {
                assert(u.contains(v@));
            } else if v@ == u[1] {
                assert(u.contains(v@));
            } else if v@ == u[2] {
                assert(u.contains(v@));
            } else {
                assert(u.contains(v@));
            }
        }
    }
    r
}

pub fn why(s: &[u8]) -> (r: Result<EVocabulary, Vec<u8>>)
    ensures
        r matches Err(e) && e@ == s@,
{
    Err(copy(s))
}

pub open spec fn tview(r: Result<EVocabulary, Vec<u8>>) -> Result<Vocabulary, Seq<u8>> {
    match r {
        Ok(t) => Ok(t@),
        Err(e) => Err(e@),
    }
}

pub fn clone_lemmas(v: &Vec<Vec<u8>>) -> (r: Vec<Vec<u8>>)
    ensures
        lemmas_view(r@) == lemmas_view(v@),
{
    let mut r: Vec<Vec<u8>> = Vec::new();
    let mut i = 0usize;
    while i < v.len()
        invariant
            i <= v@.len(),
            lemmas_view(r@) == lemmas_view(v@).take(i as int),
        decreases v@.len() - i,
    {
        let ghost before = r@;
        r.push(copy(&v[i]));
        proof {
            assert(lemmas_view(r@) =~= lemmas_view(before).push(v@[i as int]@));
            assert(lemmas_view(v@).take(i + 1) =~= lemmas_view(v@).take(i as int).push(
                v@[i as int]@,
            ));
        }
        i += 1;
    }
    proof {
        assert(lemmas_view(v@).take(v@.len() as int) =~= lemmas_view(v@));
    }
    r
}

pub fn seq_in(v: &Vec<Vec<u8>>, k: &[u8]) -> (r: bool)
    ensures
        r == lemmas_view(v@).contains(k@),
{
    let mut i = 0usize;
    while i < v.len()
        invariant
            i <= v@.len(),
            forall|j: int| 0 <= j < i ==> #[trigger] v@[j]@ != k@,
        decreases v@.len() - i,
    {
        if eq(&v[i], k) {
            proof {
                assert(lemmas_view(v@)[i as int] == k@);
            }
            return true;
        }
        i += 1;
    }
    proof {
        if lemmas_view(v@).contains(k@) {
            let j = choose|j: int| 0 <= j < lemmas_view(v@).len() && lemmas_view(v@)[j] == k@;
            assert(v@[j]@ != k@);
        }
    }
    false
}

proof fn concat_contains<A>(a: Seq<A>, b: Seq<A>, x: A)
    ensures
        (a + b).contains(x) == (a.contains(x) || b.contains(x)),
{
    if a.contains(x) {
        let i = choose|i: int| 0 <= i < a.len() && a[i] == x;
        assert((a + b)[i] == x);
    }
    if b.contains(x) {
        let i = choose|i: int| 0 <= i < b.len() && b[i] == x;
        assert((a + b)[a.len() + i] == x);
    }
    if (a + b).contains(x) {
        let i = choose|i: int| 0 <= i < (a + b).len() && (a + b)[i] == x;
        if i < a.len() {
            assert(a[i] == x);
        } else {
            assert(b[i - a.len()] == x);
        }
    }
}

// frames(t).contains(k): a spacing or window frame noun.
pub fn frame_in(t: &EVocabulary, k: &[u8]) -> (r: bool)
    ensures
        r == frames(t@).contains(k@),
{
    let a = snd_in(&t.spacings, k);
    let b = snd_in(&t.windows, k);
    proof {
        concat_contains(
            pairs_view(t.spacings@).map_values(|p: (Seq<u8>, Seq<u8>)| p.1),
            pairs_view(t.windows@).map_values(|p: (Seq<u8>, Seq<u8>)| p.1),
            k@,
        );
    }
    a || b
}

// grade_nouns(t).contains(k): a strength or evidence noun.
pub fn grade_noun_in(t: &EVocabulary, k: &[u8]) -> (r: bool)
    ensures
        r == grade_nouns(t@).contains(k@),
{
    let a = key_in(&t.strengths, k);
    let b = key_in(&t.evidences, k);
    proof {
        concat_contains(keys(pairs_view(t.strengths@)), keys(pairs_view(t.evidences@)), k@);
    }
    a || b
}

pub fn typed_noun_exec(t: &EVocabulary, k: &[u8]) -> (r: bool)
    ensures
        r == typed_noun(t@, k@),
{
    key_in(&t.units, k) || frame_in(t, k) || grade_noun_in(t, k) || key_in(&t.scopes, k)
}

// time_prepositions(t).contains(k): a relation, spacing, window, frequency or range preposition.
pub fn time_prep_in(t: &EVocabulary, k: &[u8]) -> (r: bool)
    ensures
        r == time_prepositions(t@).contains(k@),
{
    let a = key_in(&t.relations, k);
    let b = key_in(&t.spacings, k);
    let c = key_in(&t.windows, k);
    let d = seq_in(&t.frequencies, k);
    let e = seq_in(&t.ranges, k);
    proof {
        let (ra, sa, wa, fa, ga) = (
            keys(pairs_view(t.relations@)),
            keys(pairs_view(t.spacings@)),
            keys(pairs_view(t.windows@)),
            lemmas_view(t.frequencies@),
            lemmas_view(t.ranges@),
        );
        concat_contains(ra, sa, k@);
        concat_contains(ra + sa, wa, k@);
        concat_contains(ra + sa + wa, fa, k@);
        concat_contains(ra + sa + wa + fa, ga, k@);
    }
    a || b || c || d || e
}

pub fn clone_tab(t: &EVocabulary) -> (r: EVocabulary)
    ensures
        r@ == t@,
{
    EVocabulary {
        units: clone_pairs(&t.units),
        relations: clone_pairs(&t.relations),
        spacings: clone_pairs(&t.spacings),
        windows: clone_pairs(&t.windows),
        frequencies: clone_lemmas(&t.frequencies),
        approximations: clone_lemmas(&t.approximations),
        ranges: clone_lemmas(&t.ranges),
        strengths: clone_pairs(&t.strengths),
        evidences: clone_pairs(&t.evidences),
        preferences: clone_lemmas(&t.preferences),
        alternatives: clone_lemmas(&t.alternatives),
        qualifiers: clone_pairs(&t.qualifiers),
        scopes: clone_pairs(&t.scopes),
        exclusions: clone_lemmas(&t.exclusions),
        supersessions: clone_rows(&t.supersessions),
        superseded: clone_rows(&t.superseded),
    }
}

pub fn add_row_exec(t: &EVocabulary, line: &[u8]) -> (r: Result<EVocabulary, Vec<u8>>)
    ensures
        tview(r) == add_row(t@, line@),
{
    let f = split(line, 0x09);
    proof {
        literals();
        scope_literals();
        crate::m7_supersession::literals();
        assert(byte_rows(f@).len() == f@.len());
    }
    if line.len() > 0 && line[0] == 0x23 {
        return why(b"comment");
    }
    if f.len() != 3 {
        return why(b"field count");
    }
    proof {
        assert(f@[0]@ == split_on(line@, 0x09)[0]);
        assert(f@[1]@ == split_on(line@, 0x09)[1]);
        assert(f@[2]@ == split_on(line@, 0x09)[2]);
    }
    if !lemma_ok_exec(&f[1]) {
        return why(b"lemma");
    }
    let ghost fs = split_on(line@, 0x09);
    if eq(&f[0], b"unit") {
        if !unit_id_ok(&f[2]) {
            return why(b"unit id");
        }
        if typed_noun_exec(t, &f[1]) {
            return why(b"duplicate noun");
        }
        let mut r = clone_tab(t);
        r.units.push((copy(&f[1]), copy(&f[2])));
        proof {
            assert(pairs_view(r.units@) =~= pairs_view(t.units@).push((f@[1]@, f@[2]@)));
        }
        Ok(r)
    } else if eq(&f[0], b"relation") {
        if !role_id_ok(&f[2]) {
            return why(b"role id");
        }
        if key_in(&t.relations, &f[1]) || seq_in(&t.frequencies, &f[1]) || seq_in(&t.ranges, &f[1])
            || seq_in(&t.alternatives, &f[1]) {
            return why(b"duplicate preposition");
        }
        let mut r = clone_tab(t);
        r.relations.push((copy(&f[1]), copy(&f[2])));
        proof {
            assert(pairs_view(r.relations@) =~= pairs_view(t.relations@).push((f@[1]@, f@[2]@)));
        }
        Ok(r)
    } else if eq(&f[0], b"spacing") || eq(&f[0], b"window") {
        let spacing = eq(&f[0], b"spacing");
        if !lemma_ok_exec(&f[2]) {
            return why(b"frame lemma");
        }
        if pair_in(&t.spacings, &f[1], &f[2]) || pair_in(&t.windows, &f[1], &f[2]) || key_in(
            &t.units,
            &f[2],
        ) || grade_noun_in(t, &f[2]) || key_in(&t.scopes, &f[2]) {
            return why(b"duplicate noun");
        }
        if seq_in(&t.alternatives, &f[1]) {
            return why(b"duplicate preposition");
        }
        let mut r = clone_tab(t);
        if spacing {
            r.spacings.push((copy(&f[1]), copy(&f[2])));
            proof {
                assert(pairs_view(r.spacings@) =~= pairs_view(t.spacings@).push((f@[1]@, f@[2]@)));
            }
        } else {
            r.windows.push((copy(&f[1]), copy(&f[2])));
            proof {
                assert(pairs_view(r.windows@) =~= pairs_view(t.windows@).push((f@[1]@, f@[2]@)));
            }
        }
        Ok(r)
    } else if eq(&f[0], b"frequency") {
        if !eq(&f[2], b"period") {
            return why(b"frequency value");
        }
        if seq_in(&t.frequencies, &f[1]) || key_in(&t.relations, &f[1]) || seq_in(&t.ranges, &f[1])
            || seq_in(&t.alternatives, &f[1]) {
            return why(b"duplicate preposition");
        }
        let mut r = clone_tab(t);
        r.frequencies.push(copy(&f[1]));
        proof {
            assert(lemmas_view(r.frequencies@) =~= lemmas_view(t.frequencies@).push(f@[1]@));
        }
        Ok(r)
    } else if eq(&f[0], b"approximation") {
        if !eq(&f[2], b"about") {
            return why(b"approximation value");
        }
        if seq_in(&t.approximations, &f[1]) || seq_in(&t.preferences, &f[1]) || key_in(
            &t.qualifiers,
            &f[1],
        ) {
            return why(b"duplicate adjective");
        }
        let mut r = clone_tab(t);
        r.approximations.push(copy(&f[1]));
        proof {
            assert(lemmas_view(r.approximations@) =~= lemmas_view(t.approximations@).push(f@[1]@));
        }
        Ok(r)
    } else if eq(&f[0], b"range") {
        if !eq(&f[2], b"minimum") {
            return why(b"range value");
        }
        if seq_in(&t.ranges, &f[1]) || key_in(&t.relations, &f[1]) || seq_in(&t.frequencies, &f[1])
            || seq_in(&t.alternatives, &f[1]) {
            return why(b"duplicate preposition");
        }
        let mut r = clone_tab(t);
        r.ranges.push(copy(&f[1]));
        proof {
            assert(lemmas_view(r.ranges@) =~= lemmas_view(t.ranges@).push(f@[1]@));
        }
        Ok(r)
    } else if eq(&f[0], b"strength") || eq(&f[0], b"evidence") {
        let strength = eq(&f[0], b"strength");
        if strength && !strength_id_ok(&f[2]) {
            return why(b"strength value");
        }
        if !strength && !evidence_id_ok(&f[2]) {
            return why(b"evidence value");
        }
        if typed_noun_exec(t, &f[1]) {
            return why(b"duplicate noun");
        }
        let mut r = clone_tab(t);
        if strength {
            r.strengths.push((copy(&f[1]), copy(&f[2])));
            proof {
                assert(pairs_view(r.strengths@) =~= pairs_view(t.strengths@).push(
                    (f@[1]@, f@[2]@),
                ));
            }
        } else {
            r.evidences.push((copy(&f[1]), copy(&f[2])));
            proof {
                assert(pairs_view(r.evidences@) =~= pairs_view(t.evidences@).push(
                    (f@[1]@, f@[2]@),
                ));
            }
        }
        Ok(r)
    } else if eq(&f[0], b"preference") {
        if !eq(&f[2], b"preferred") {
            return why(b"preference value");
        }
        if seq_in(&t.preferences, &f[1]) || seq_in(&t.approximations, &f[1]) || key_in(
            &t.qualifiers,
            &f[1],
        ) {
            return why(b"duplicate adjective");
        }
        let mut r = clone_tab(t);
        r.preferences.push(copy(&f[1]));
        proof {
            assert(lemmas_view(r.preferences@) =~= lemmas_view(t.preferences@).push(f@[1]@));
        }
        Ok(r)
    } else if eq(&f[0], b"alternative") {
        if !eq(&f[2], b"alternative") {
            return why(b"alternative value");
        }
        if seq_in(&t.alternatives, &f[1]) || time_prep_in(t, &f[1]) {
            return why(b"duplicate preposition");
        }
        let mut r = clone_tab(t);
        r.alternatives.push(copy(&f[1]));
        proof {
            assert(lemmas_view(r.alternatives@) =~= lemmas_view(t.alternatives@).push(f@[1]@));
        }
        Ok(r)
    } else if eq(&f[0], b"qualifier") {
        if !qualifier_id_ok(&f[2]) {
            return why(b"qualifier value");
        }
        if key_in(&t.qualifiers, &f[1]) {
            return why(b"duplicate qualifier");
        }
        if seq_in(&t.approximations, &f[1]) || seq_in(&t.preferences, &f[1]) {
            return why(b"duplicate adjective");
        }
        let mut r = clone_tab(t);
        r.qualifiers.push((copy(&f[1]), copy(&f[2])));
        proof {
            assert(pairs_view(r.qualifiers@) =~= pairs_view(t.qualifiers@).push((f@[1]@, f@[2]@)));
        }
        Ok(r)
    } else if eq(&f[0], b"scope") {
        if !crate::v1_term_impl::name_ok_exec(&f[2]) {
            return why(b"scope value");
        }
        if t.scopes.len() > 0 {
            return why(b"duplicate scope");
        }
        if typed_noun_exec(t, &f[1]) {
            return why(b"duplicate noun");
        }
        let mut r = clone_tab(t);
        r.scopes.push((copy(&f[1]), copy(&f[2])));
        proof {
            assert(pairs_view(r.scopes@) =~= pairs_view(t.scopes@).push((f@[1]@, f@[2]@)));
        }
        Ok(r)
    } else if eq(&f[0], b"exclusion") {
        if !eq(&f[2], b"scope") {
            return why(b"exclusion value");
        }
        if t.exclusions.len() > 0 {
            return why(b"duplicate exclusion");
        }
        let mut r = clone_tab(t);
        r.exclusions.push(copy(&f[1]));
        proof {
            assert(lemmas_view(r.exclusions@) =~= lemmas_view(t.exclusions@).push(f@[1]@));
        }
        Ok(r)
    } else if eq(&f[0], b"supersession") || eq(&f[0], b"superseded") {
        let document = eq(&f[0], b"supersession");
        if document && !crate::v1_term_impl::name_ok_exec(&f[1]) {
            return why(b"supersession document");
        }
        let sup = match crate::m7_supersession::superseding_exec(&f[2]) {
            Some(s) => s,
            None => return why(b"supersession value"),
        };
        if document && crate::m7_supersession::key_in(&t.supersessions, &f[1]) {
            return why(b"duplicate supersession");
        }
        if !document && crate::m7_supersession::key_in(&t.superseded, &f[1]) {
            return why(b"duplicate superseded");
        }
        let mut r = clone_tab(t);
        let ghost s = sup@;
        if document {
            r.supersessions.push((copy(&f[1]), sup));
            proof {
                assert(sup_rows(r.supersessions@) =~= sup_rows(t.supersessions@).push((f@[1]@, s)));
            }
        } else {
            r.superseded.push((copy(&f[1]), sup));
            proof {
                assert(sup_rows(r.superseded@) =~= sup_rows(t.superseded@).push((f@[1]@, s)));
            }
        }
        Ok(r)
    } else {
        why(b"kind")
    }
}

pub fn kind_row_exec(rows: &Vec<Vec<u8>>, n: usize, kind: &[u8]) -> (r: usize)
    requires
        n <= rows.len(),
    ensures
        r as nat == kind_row(byte_rows(rows@).take(n as int), kind@, 1),
{
    hide(kind_row);
    let ghost rs = byte_rows(rows@).take(n as int);
    let mut i = 0usize;
    proof {
        assert(rs.skip(0) =~= rs);
    }
    while i < n
        invariant
            i <= n <= rows.len(),
            rs == byte_rows(rows@).take(n as int),
            kind_row(rs, kind@, 1) == kind_row(rs.skip(i as int), kind@, (i + 1) as nat),
        decreases n - i,
    {
        let fields = split(&rows[i], 0x09);
        proof {
            assert(rs.skip(i as int)[0] == rows@[i as int]@);
            assert(rs.skip(i as int).drop_first() =~= rs.skip(i + 1));
            assert(fields@[0]@ == split_on(rows@[i as int]@, 0x09)[0]);
            reveal_with_fuel(kind_row, 1);
        }
        if eq(&fields[0], kind) {
            return i + 1;
        }
        i += 1;
    }
    proof {
        assert(rs.skip(n as int).len() == 0);
        reveal_with_fuel(kind_row, 1);
    }
    0
}

fn scope_row_why(prefix: &[u8], rows: &Vec<Vec<u8>>, n: usize, kind: &[u8]) -> (r: Vec<u8>)
    requires
        n <= rows.len(),
    ensures
        r@ == prefix@ + udec_bytes(kind_row(byte_rows(rows@).take(n as int), kind@, 1)),
{
    let row = kind_row_exec(rows, n, kind);
    let mut out = copy(prefix);
    let count = crate::k2_manifest::udec_vec(row);
    append(&mut out, &count);
    out
}

fn pairing_exec(t: &EVocabulary, rows: &Vec<Vec<u8>>, n: usize) -> (r: Option<Vec<u8>>)
    requires
        n <= rows.len(),
    ensures
        (match r {
            Some(why) => Some(why@),
            None => None,
        }) == pairing(t@, byte_rows(rows@).take(n as int)),
{
    proof {
        scope_literals();
    }
    if t.scopes.len() > 0 && t.exclusions.len() == 0 {
        Some(scope_row_why(b"scope without exclusion at row ", rows, n, b"scope"))
    } else if t.exclusions.len() > 0 && t.scopes.len() == 0 {
        Some(scope_row_why(b"exclusion without scope at row ", rows, n, b"exclusion"))
    } else {
        None
    }
}

pub fn guideline_of_exec(path: &[u8]) -> (r: Option<Vec<u8>>)
    ensures
        (match r {
            Some(id) => Some(id@),
            None => None,
        }) == guideline_of(path@),
{
    let pre: &[u8] = b"guidelines/";
    let post: &[u8] = b"/vocabulary.tsv";
    proof {
        scope_literals();
        reveal_byteslit(b"guidelines/");
        reveal_byteslit(b"/vocabulary.tsv");
    }
    if path.len() <= pre.len() + post.len() || !starts_with(path, pre)
        || !crate::k4_bytes::ends_with(path, post) {
        return None;
    }
    let id = slice_subrange(path, pre.len(), path.len() - post.len());
    if crate::k4_bytes::has(id, 0x2F) {
        proof {
            assert(!all_in(id@, |b: u8| b != 0x2F));
        }
        None
    } else {
        proof {
            assert(all_in(id@, |b: u8| b != 0x2F));
        }
        Some(copy(id))
    }
}

fn key_law_exec(path: &[u8], t: &EVocabulary, bytes: &[u8]) -> (r: Option<Vec<u8>>)
    requires
        bytes.len() >= vocabulary_header().len(),
    ensures
        (match r {
            Some(why) => Some(why@),
            None => None,
        }) == key_law(path@, t@, bytes@),
{
    let h: &[u8] = b"# format: kind\tlemma\tvalue\n";
    proof {
        scope_literals();
        reveal_byteslit(b"# format: kind\tlemma\tvalue\n");
        reveal_strlit("# format: kind\tlemma\tvalue\n");
        reveal(ascii);
        assert(h@ =~= vocabulary_header());
    }
    match guideline_of_exec(path) {
        Some(id) => {
            if t.scopes.len() > 0 && !eq(&t.scopes[0].1, &id) {
                let body = slice_subrange(bytes, h.len(), bytes.len());
                let rows = split(body, 0x0A);
                let n = rows.len() - 1;
                proof {
                    assert(byte_rows(rows@).take(n as int) =~= body_lines(body@));
                    assert(body@ =~= bytes@.skip(vocabulary_header().len() as int));
                }
                Some(scope_row_why(b"scope key at row ", &rows, n, b"scope"))
            } else {
                None
            }
        },
        None => None,
    }
}

proof fn add_row_count(t: Vocabulary, line: Seq<u8>)
    ensures
        add_row(t, line) matches Ok(t2) ==> row_count(t2) == row_count(t) + 1,
{
}

pub open spec fn counted_view(r: Result<(EVocabulary, usize), Vec<u8>>) -> Result<
    Vocabulary,
    Seq<u8>,
> {
    match r {
        Ok((t, _)) => Ok(t@),
        Err(e) => Err(e@),
    }
}

// The table + its row count (the count rides the parse: the Vec lengths need no sum).
pub fn parse_vocabulary_exec(bytes: &[u8]) -> (r: Result<(EVocabulary, usize), Vec<u8>>)
    ensures
        counted_view(r) == parse_vocabulary(bytes@),
        r matches Ok((t, n)) ==> n == row_count(t@),
{
    let h: &[u8] = b"# format: kind\tlemma\tvalue\n";
    proof {
        reveal_byteslit(b"# format: kind\tlemma\tvalue\n");
        reveal_strlit("# format: kind\tlemma\tvalue\n");
        reveal(ascii);
        assert(h@ =~= vocabulary_header());
        literals();
    }
    if !starts_with(bytes, h) {
        return Err(copy(b"header"));
    }
    let hl = h.len();
    let body = slice_subrange(bytes, hl, bytes.len());
    proof {
        assert(body@ =~= bytes@.skip(vocabulary_header().len() as int));
    }
    if body.len() == 0 {
        return Err(copy(b"no rows"));
    }
    if body[body.len() - 1] != 0x0A {
        return Err(copy(b"final newline"));
    }
    let lines = split(body, 0x0A);
    let n = lines.len() - 1;
    let ghost rows = body_lines(body@);
    let ghost e0 = empty_table();
    proof {
        assert(rows =~= byte_rows(lines@).drop_last());
        assert(body@.last() == 0x0A);
        assert(parse_vocabulary(bytes@) == match add_rows(e0, rows, 1) {
            Err(why) => Err(why),
            Ok(t) => match pairing(t, rows) {
                Some(why) => Err(why),
                None => Ok(t),
            },
        });
    }
    let mut t = EVocabulary {
        units: Vec::new(),
        relations: Vec::new(),
        spacings: Vec::new(),
        windows: Vec::new(),
        frequencies: Vec::new(),
        approximations: Vec::new(),
        ranges: Vec::new(),
        strengths: Vec::new(),
        evidences: Vec::new(),
        preferences: Vec::new(),
        alternatives: Vec::new(),
        qualifiers: Vec::new(),
        scopes: Vec::new(),
        exclusions: Vec::new(),
        supersessions: Vec::new(),
        superseded: Vec::new(),
    };
    let mut i = 0usize;
    proof {
        assert(t@.units =~= Seq::<(Seq<u8>, Seq<u8>)>::empty());
        assert(t@.relations =~= Seq::<(Seq<u8>, Seq<u8>)>::empty());
        assert(t@.spacings =~= Seq::<(Seq<u8>, Seq<u8>)>::empty());
        assert(t@.windows =~= Seq::<(Seq<u8>, Seq<u8>)>::empty());
        assert(t@.frequencies =~= Seq::<Seq<u8>>::empty());
        assert(t@.approximations =~= Seq::<Seq<u8>>::empty());
        assert(t@.ranges =~= Seq::<Seq<u8>>::empty());
        assert(t@.strengths =~= Seq::<(Seq<u8>, Seq<u8>)>::empty());
        assert(t@.evidences =~= Seq::<(Seq<u8>, Seq<u8>)>::empty());
        assert(t@.preferences =~= Seq::<Seq<u8>>::empty());
        assert(t@.alternatives =~= Seq::<Seq<u8>>::empty());
        assert(t@.qualifiers =~= Seq::<(Seq<u8>, Seq<u8>)>::empty());
        assert(t@.scopes =~= Seq::<(Seq<u8>, Seq<u8>)>::empty());
        assert(t@.exclusions =~= Seq::<Seq<u8>>::empty());
        assert(t@.supersessions =~= Seq::<(Seq<u8>, Superseding)>::empty());
        assert(t@.superseded =~= Seq::<(Seq<u8>, Superseding)>::empty());
        assert(t@ == e0);
        assert(rows.skip(0) =~= rows);
    }
    while i < n
        invariant
            n == lines@.len() - 1,
            i <= n,
            rows == byte_rows(lines@).drop_last(),
            add_rows(e0, rows, 1) == add_rows(t@, rows.skip(i as int), (i + 1) as nat),
            row_count(t@) == i,
            parse_vocabulary(bytes@) == match add_rows(e0, rows, 1) {
                Err(why) => Err(why),
                Ok(t) => match pairing(t, rows) {
                    Some(why) => Err(why),
                    None => Ok(t),
                },
            },
        decreases n - i,
    {
        proof {
            assert(rows.skip(i as int)[0] == lines@[i as int]@);
            assert(rows.skip(i as int).drop_first() =~= rows.skip(i + 1));
        }
        match add_row_exec(&t, &lines[i]) {
            Err(e) => {
                let ghost ev = e@;
                let mut out = e;
                append(&mut out, b" at row ");
                let num = crate::k2_manifest::udec_vec(i + 1);
                append(&mut out, &num);
                proof {
                    literals();
                    let rest = rows.skip(i as int);
                    assert(rest.len() > 0);
                    assert(rest[0] == lines@[i as int]@);
                    assert(add_row(t@, rest[0]) == Err::<Vocabulary, Seq<u8>>(ev));
                    assert(add_rows(t@, rest, (i + 1) as nat) == Err::<Vocabulary, Seq<u8>>(
                        ev + ascii(" at row "@) + udec_bytes((i + 1) as nat),
                    ));
                    assert(out@ =~= ev + ascii(" at row "@) + udec_bytes((i + 1) as nat));
                }
                return Err(out);
            },
            Ok(t2) => {
                proof {
                    add_row_count(t@, lines@[i as int]@);
                }
                t = t2;
            },
        }
        i += 1;
    }
    proof {
        assert(rows.skip(n as int).len() == 0);
    }
    proof {
        assert(byte_rows(lines@).take(n as int) =~= rows);
    }
    match pairing_exec(&t, &lines, n) {
        Some(why) => Err(why),
        None => Ok((t, n)),
    }
}

pub fn check_supersession_impl(path: &[u8], bytes: &[u8], c: &ckc_spec::check::ECoverage) -> (r:
    ckc_spec::check::EVerdict)
    ensures
        r@ == ckc_spec::vocabulary::supersession_check(path@, bytes@, c@.rows),
{
    crate::m7_supersession::check(path, bytes, c)
}

pub fn check_vocabulary_impl(path: &[u8], bytes: &[u8]) -> (r: ckc_spec::check::EVerdict)
    ensures
        r@ == vocabulary_check(path@, bytes@),
{
    proof {
        literals();
    }
    match parse_vocabulary_exec(bytes) {
        Ok((t, count)) => {
            if let Some(why) = key_law_exec(path, &t, bytes) {
                let mut d = copy(path);
                append(&mut d, b": ");
                append(&mut d, &why);
                return ckc_spec::check::EVerdict::Fail(copy(b"vocabulary"), d);
            }
            let mut m = copy(b"ckc: vocabulary ok ");
            append(&mut m, path);
            append(&mut m, b" ");
            let num = crate::k2_manifest::udec_vec(count);
            append(&mut m, &num);
            append(&mut m, b" rows\n");
            ckc_spec::check::EVerdict::Ok(m)
        },
        Err(why) => {
            let mut d = copy(path);
            append(&mut d, b": ");
            append(&mut d, &why);
            ckc_spec::check::EVerdict::Fail(copy(b"vocabulary"), d)
        },
    }
}

} // verus!

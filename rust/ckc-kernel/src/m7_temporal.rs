use crate::k4_bytes::{append, copy, eq, split, starts_with};
use ckc_spec::check::*;
use ckc_spec::temporal::*;
#[cfg(verus_keep_ghost)]
use ckc_spec::v1text::{all_in, ascii, udec_bytes};
use vstd::prelude::*;
use vstd::slice::slice_subrange;

verus! {

// m7t D1 + q12 D1: the temporal.tsv grammar (v2 | v3 header), parsed once for
// `ckc check` and for the v2/v3 projection.
pub struct ETemporal {
    pub version: u8,
    pub units: Vec<(Vec<u8>, Vec<u8>)>,
    pub relations: Vec<(Vec<u8>, Vec<u8>)>,
    pub spacings: Vec<(Vec<u8>, Vec<u8>)>,
    pub windows: Vec<(Vec<u8>, Vec<u8>)>,
    pub frequencies: Vec<Vec<u8>>,
}

pub open spec fn pairs_view(v: Seq<(Vec<u8>, Vec<u8>)>) -> Seq<(Seq<u8>, Seq<u8>)> {
    v.map_values(|p: (Vec<u8>, Vec<u8>)| (p.0@, p.1@))
}

pub open spec fn lemmas_view(v: Seq<Vec<u8>>) -> Seq<Seq<u8>> {
    v.map_values(|x: Vec<u8>| x@)
}

impl View for ETemporal {
    type V = Temporal;

    open spec fn view(&self) -> Temporal {
        Temporal {
            version: self.version as nat,
            units: pairs_view(self.units@),
            relations: pairs_view(self.relations@),
            spacings: pairs_view(self.spacings@),
            windows: pairs_view(self.windows@),
            frequencies: lemmas_view(self.frequencies@),
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
        b"ckc: temporal ok "@ == ascii("ckc: temporal ok "@),
        b" "@ == ascii(" "@),
        b" rows\n"@ == ascii(" rows\n"@),
        b": "@ == ascii(": "@),
        b"temporal"@ == ascii("temporal"@),
        b"window"@ == ascii("window"@),
        b"frequency"@ == ascii("frequency"@),
        b"frequency value"@ == ascii("frequency value"@),
        b"period"@ == ascii("period"@),
{
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
    reveal_byteslit(b"ckc: temporal ok ");
    reveal_strlit("ckc: temporal ok ");
    assert(b"ckc: temporal ok "@ =~= ascii("ckc: temporal ok "@));
    reveal_byteslit(b" ");
    reveal_strlit(" ");
    assert(b" "@ =~= ascii(" "@));
    reveal_byteslit(b" rows\n");
    reveal_strlit(" rows\n");
    assert(b" rows\n"@ =~= ascii(" rows\n"@));
    reveal_byteslit(b": ");
    reveal_strlit(": ");
    assert(b": "@ =~= ascii(": "@));
    reveal_byteslit(b"temporal");
    reveal_strlit("temporal");
    assert(b"temporal"@ =~= ascii("temporal"@));
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

pub fn why(s: &[u8]) -> (r: Result<ETemporal, Vec<u8>>)
    ensures
        r matches Err(e) && e@ == s@,
{
    Err(copy(s))
}

pub open spec fn tview(r: Result<ETemporal, Vec<u8>>) -> Result<Temporal, Seq<u8>> {
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
pub fn frame_in(t: &ETemporal, k: &[u8]) -> (r: bool)
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

pub fn clone_tab(t: &ETemporal) -> (r: ETemporal)
    ensures
        r@ == t@,
{
    ETemporal {
        version: t.version,
        units: clone_pairs(&t.units),
        relations: clone_pairs(&t.relations),
        spacings: clone_pairs(&t.spacings),
        windows: clone_pairs(&t.windows),
        frequencies: clone_lemmas(&t.frequencies),
    }
}

pub fn add_row_exec(t: &ETemporal, line: &[u8]) -> (r: Result<ETemporal, Vec<u8>>)
    ensures
        tview(r) == add_row(t@, line@),
{
    let f = split(line, 0x09);
    proof {
        literals();
        assert(byte_rows(f@).len() == f@.len());
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
        if key_in(&t.units, &f[1]) || frame_in(t, &f[1]) {
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
        if key_in(&t.relations, &f[1]) || seq_in(&t.frequencies, &f[1]) {
            return why(b"duplicate preposition");
        }
        let mut r = clone_tab(t);
        r.relations.push((copy(&f[1]), copy(&f[2])));
        proof {
            assert(pairs_view(r.relations@) =~= pairs_view(t.relations@).push((f@[1]@, f@[2]@)));
        }
        Ok(r)
    } else if eq(&f[0], b"spacing") || (t.version == 3 && eq(&f[0], b"window")) {
        let spacing = eq(&f[0], b"spacing");
        if !lemma_ok_exec(&f[2]) {
            return why(b"frame lemma");
        }
        if pair_in(&t.spacings, &f[1], &f[2]) || pair_in(&t.windows, &f[1], &f[2]) || key_in(
            &t.units,
            &f[2],
        ) {
            return why(b"duplicate noun");
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
    } else if t.version == 3 && eq(&f[0], b"frequency") {
        if !eq(&f[2], b"period") {
            return why(b"frequency value");
        }
        if seq_in(&t.frequencies, &f[1]) || key_in(&t.relations, &f[1]) {
            return why(b"duplicate preposition");
        }
        let mut r = clone_tab(t);
        r.frequencies.push(copy(&f[1]));
        proof {
            assert(lemmas_view(r.frequencies@) =~= lemmas_view(t.frequencies@).push(f@[1]@));
        }
        Ok(r)
    } else {
        why(b"kind")
    }
}

proof fn add_row_count(t: Temporal, line: Seq<u8>)
    ensures
        add_row(t, line) matches Ok(t2) ==> row_count(t2) == row_count(t) + 1,
{
}

pub open spec fn counted_view(r: Result<(ETemporal, usize), Vec<u8>>) -> Result<Temporal, Seq<u8>> {
    match r {
        Ok((t, _)) => Ok(t@),
        Err(e) => Err(e@),
    }
}

// The table + its row count (the count rides the parse: the Vec lengths need no sum).
pub fn parse_temporal_exec(bytes: &[u8]) -> (r: Result<(ETemporal, usize), Vec<u8>>)
    ensures
        counted_view(r) == parse_temporal(bytes@),
        r matches Ok((t, n)) ==> n == row_count(t@),
{
    let h2: &[u8] =
        b"# format: kind\tlemma\tvalue\n# kind: unit (value: second|minute|hour|day|week|month|year) | relation (value: duration|within|after|before) | spacing (value: frame noun lemma)\n";
    let h3: &[u8] =
        b"# format: kind\tlemma\tvalue\n# kind: unit (value: second|minute|hour|day|week|month|year) | relation (value: duration|within|after|before) | spacing (value: frame noun lemma) | window (value: frame noun lemma) | frequency (value: period) | approximation (value: about) | range (value: minimum)\n";
    proof {
        reveal_byteslit(
            b"# format: kind\tlemma\tvalue\n# kind: unit (value: second|minute|hour|day|week|month|year) | relation (value: duration|within|after|before) | spacing (value: frame noun lemma)\n",
        );
        reveal_strlit(
            "# format: kind\tlemma\tvalue\n# kind: unit (value: second|minute|hour|day|week|month|year) | relation (value: duration|within|after|before) | spacing (value: frame noun lemma)\n",
        );
        reveal_byteslit(
            b"# format: kind\tlemma\tvalue\n# kind: unit (value: second|minute|hour|day|week|month|year) | relation (value: duration|within|after|before) | spacing (value: frame noun lemma) | window (value: frame noun lemma) | frequency (value: period) | approximation (value: about) | range (value: minimum)\n",
        );
        reveal_strlit(
            "# format: kind\tlemma\tvalue\n# kind: unit (value: second|minute|hour|day|week|month|year) | relation (value: duration|within|after|before) | spacing (value: frame noun lemma) | window (value: frame noun lemma) | frequency (value: period) | approximation (value: about) | range (value: minimum)\n",
        );
        reveal(ascii);
        assert(h2@ =~= temporal_header());
        assert(h3@ =~= temporal_header_v3());
        literals();
    }
    let v2 = starts_with(bytes, h2);
    let v3 = !v2 && starts_with(bytes, h3);
    if !v2 && !v3 {
        return Err(copy(b"header"));
    }
    let version: u8 = if v2 {
        2
    } else {
        3
    };
    let hl = if v2 {
        h2.len()
    } else {
        h3.len()
    };
    let ghost h = if v2 {
        temporal_header()
    } else {
        temporal_header_v3()
    };
    let body = slice_subrange(bytes, hl, bytes.len());
    proof {
        assert(hl == h.len());
        assert(body@ =~= bytes@.skip(h.len() as int));
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
    let ghost e0 = empty_table(version as nat);
    proof {
        assert(rows =~= byte_rows(lines@).drop_last());
        assert(body@.last() == 0x0A);
        assert(parse_temporal(bytes@) == add_rows(e0, rows, 1));
    }
    let mut t = ETemporal {
        version,
        units: Vec::new(),
        relations: Vec::new(),
        spacings: Vec::new(),
        windows: Vec::new(),
        frequencies: Vec::new(),
    };
    let mut i = 0usize;
    proof {
        assert(t@.units =~= Seq::<(Seq<u8>, Seq<u8>)>::empty());
        assert(t@.relations =~= Seq::<(Seq<u8>, Seq<u8>)>::empty());
        assert(t@.spacings =~= Seq::<(Seq<u8>, Seq<u8>)>::empty());
        assert(t@.windows =~= Seq::<(Seq<u8>, Seq<u8>)>::empty());
        assert(t@.frequencies =~= Seq::<Seq<u8>>::empty());
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
            parse_temporal(bytes@) == add_rows(e0, rows, 1),
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
                    assert(add_row(t@, rest[0]) == Err::<Temporal, Seq<u8>>(ev));
                    assert(add_rows(t@, rest, (i + 1) as nat) == Err::<Temporal, Seq<u8>>(
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
    Ok((t, n))
}

pub fn check_temporal_impl(path: &[u8], bytes: &[u8]) -> (r: ckc_spec::check::EVerdict)
    ensures
        r@ == temporal_check(path@, bytes@),
{
    proof {
        literals();
    }
    match parse_temporal_exec(bytes) {
        Ok((_, count)) => {
            let mut m = copy(b"ckc: temporal ok ");
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
            ckc_spec::check::EVerdict::Fail(copy(b"temporal"), d)
        },
    }
}

} // verus!

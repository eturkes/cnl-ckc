use crate::check::*;
use crate::v1text::*;
use vstd::prelude::*;
use vstd::utf8::*;

verus! {

// Trusted spec: the per-guideline vocabulary table `vocabulary.tsv` (contracts
// m7t D1, q12 D1, q13 D1, c1 D4, s1 D2, s2 D2). Corpus data maps source lemmas onto closed
// schema ids, so the compiler names no source word: unit rows map a noun lemma
// to a unit id, relation rows map a preposition lemma to an interval role,
// spacing rows pair a preposition with the frame noun of a recurrence
// (`at an interval of …`), window rows the frame of a scoped recurrence
// (`during a window of …`), frequency rows the preposition of a count per
// period (`per 1 day`), approximation rows an adjective that marks a time
// quantity approximate (`2 approximate hours`), range rows the preposition of a
// ranged minimum's upper end (`at least 8 hours to 12 hours`); strength and
// evidence rows map a grade noun to its common value, preference rows an
// adjective that marks the preferred option on a copula, alternative rows the
// preposition that joins the option a preference names as its alternative;
// qualifier rows map an adverb, or the adjective of a copula, to the function
// class of the qualifier it carries; the scope row names the noun that stands
// for the guideline's recommendations as a whole + the guideline's key, the
// exclusion row the verb an exclusion denies of that noun (contract cs1 D1).
pub ghost struct Vocabulary {
    pub units: Seq<(Seq<u8>, Seq<u8>)>,
    pub relations: Seq<(Seq<u8>, Seq<u8>)>,
    pub spacings: Seq<(Seq<u8>, Seq<u8>)>,
    pub windows: Seq<(Seq<u8>, Seq<u8>)>,
    pub frequencies: Seq<Seq<u8>>,  // preposition lemmas
    pub approximations: Seq<Seq<u8>>,  // adjective lemmas
    pub ranges: Seq<Seq<u8>>,  // preposition lemmas
    pub strengths: Seq<(Seq<u8>, Seq<u8>)>,  // grade noun → strength value
    pub evidences: Seq<(Seq<u8>, Seq<u8>)>,  // grade noun → evidence value
    pub preferences: Seq<Seq<u8>>,  // adjective lemmas
    pub alternatives: Seq<Seq<u8>>,  // preposition lemmas
    pub qualifiers: Seq<(Seq<u8>, Seq<u8>)>,  // adverb | copula adjective → class
    pub scopes: Seq<(Seq<u8>, Seq<u8>)>,  // scope noun → guideline key (at most one)
    pub exclusions: Seq<Seq<u8>>,  // exclusion verb lemmas (at most one)
}

pub open spec fn vocabulary_header() -> Seq<u8> {
    ascii("# format: kind\tlemma\tvalue\n"@)
}

pub open spec fn unit_ids() -> Seq<Seq<u8>> {
    seq![
        ascii("second"@),
        ascii("minute"@),
        ascii("hour"@),
        ascii("day"@),
        ascii("week"@),
        ascii("month"@),
        ascii("year"@),
    ]
}

pub open spec fn role_ids() -> Seq<Seq<u8>> {
    seq![ascii("duration"@), ascii("within"@), ascii("after"@), ascii("before"@)]
}

pub open spec fn strength_ids() -> Seq<Seq<u8>> {
    seq![ascii("strong"@), ascii("conditional"@)]
}

pub open spec fn evidence_ids() -> Seq<Seq<u8>> {
    seq![ascii("high"@), ascii("moderate"@), ascii("low"@), ascii("very_low"@)]
}

pub open spec fn qualifier_ids() -> Seq<Seq<u8>> {
    seq![
        ascii("usuality"@),
        ascii("ideality"@),
        ascii("focus"@),
        ascii("manner"@),
        ascii("force"@),
        ascii("possibility"@),
        ascii("applicability"@),
    ]
}

// A lemma field: nonempty UTF-8 with no space, control or DEL byte (the
// compiler matches it against the parser's decoded lemmas).
pub open spec fn lemma_ok(l: Seq<u8>) -> bool {
    l.len() > 0 && all_in(l, |b: u8| b > 0x20 && b != 0x7F) && valid_utf8(l)
}

pub open spec fn keys(rows: Seq<(Seq<u8>, Seq<u8>)>) -> Seq<Seq<u8>> {
    rows.map_values(|r: (Seq<u8>, Seq<u8>)| r.0)
}

// Frame nouns: spacing frames, then window frames.
pub open spec fn frames(t: Vocabulary) -> Seq<Seq<u8>> {
    t.spacings.map_values(|r: (Seq<u8>, Seq<u8>)| r.1) + t.windows.map_values(
        |r: (Seq<u8>, Seq<u8>)| r.1,
    )
}

// Grade nouns: strength rows, then evidence rows.
pub open spec fn grade_nouns(t: Vocabulary) -> Seq<Seq<u8>> {
    keys(t.strengths) + keys(t.evidences)
}

// A noun in a row that types it: a unit, frame, grade or scope noun.
pub open spec fn typed_noun(t: Vocabulary, n: Seq<u8>) -> bool {
    keys(t.units).contains(n) || frames(t).contains(n) || grade_nouns(t).contains(n) || keys(
        t.scopes,
    ).contains(n)
}

// Every preposition a row names outside the alternative kind.
pub open spec fn time_prepositions(t: Vocabulary) -> Seq<Seq<u8>> {
    keys(t.relations) + keys(t.spacings) + keys(t.windows) + t.frequencies + t.ranges
}

pub open spec fn row_count(t: Vocabulary) -> nat {
    t.units.len() + t.relations.len() + t.spacings.len() + t.windows.len() + t.frequencies.len()
        + t.approximations.len() + t.ranges.len() + t.strengths.len() + t.evidences.len()
        + t.preferences.len() + t.alternatives.len() + t.qualifiers.len() + t.scopes.len()
        + t.exclusions.len()
}

pub open spec fn empty_table() -> Vocabulary {
    Vocabulary {
        units: Seq::empty(),
        relations: Seq::empty(),
        spacings: Seq::empty(),
        windows: Seq::empty(),
        frequencies: Seq::empty(),
        approximations: Seq::empty(),
        ranges: Seq::empty(),
        strengths: Seq::empty(),
        evidences: Seq::empty(),
        preferences: Seq::empty(),
        alternatives: Seq::empty(),
        qualifiers: Seq::empty(),
        scopes: Seq::empty(),
        exclusions: Seq::empty(),
    }
}

// A frame row (spacing | window): (preposition, frame noun) once across both
// kinds; a frame noun is never a unit or grade noun; its preposition is never
// an alternative.
pub open spec fn frame_row(t: Vocabulary, f: Seq<Seq<u8>>) -> Option<Seq<u8>> {
    if !lemma_ok(f[2]) {
        Option::Some(ascii("frame lemma"@))
    } else if t.spacings.contains((f[1], f[2])) || t.windows.contains((f[1], f[2])) || keys(
        t.units,
    ).contains(f[2]) || grade_nouns(t).contains(f[2]) || keys(t.scopes).contains(f[2]) {
        Option::Some(ascii("duplicate noun"@))
    } else if t.alternatives.contains(f[1]) {
        Option::Some(ascii("duplicate preposition"@))
    } else {
        Option::None
    }
}

// A grade row (strength | evidence): the value is the kind's closed id; a noun
// grades once across both kinds and is never a unit or frame noun.
pub open spec fn grade_row(
    t: Vocabulary,
    f: Seq<Seq<u8>>,
    ids: Seq<Seq<u8>>,
    value_why: Seq<u8>,
) -> Option<Seq<u8>> {
    if !ids.contains(f[2]) {
        Option::Some(value_why)
    } else if typed_noun(t, f[1]) {
        Option::Some(ascii("duplicate noun"@))
    } else {
        Option::None
    }
}

// One body row added to the table, or the grammar it breaks. A row starting
// with `#` is a comment, never a row (a pre-c1 two-line header fails at row 1).
// A noun is one of unit, frame and grade noun; each lemma maps once per kind; a
// preposition maps to at most one of relation, frequency and range, and an
// alternative preposition to no other kind; a lemma is at most one of an
// approximation, a preference and a qualifier (the three read `property`
// conditions), each once. The later row of a conflict reports it.
pub open spec fn add_row(t: Vocabulary, line: Seq<u8>) -> Result<Vocabulary, Seq<u8>> {
    let f = split_on(line, 0x09);
    if line.len() > 0 && line[0] == 0x23 {
        Result::Err(ascii("comment"@))
    } else if f.len() != 3 {
        Result::Err(ascii("field count"@))
    } else if !lemma_ok(f[1]) {
        Result::Err(ascii("lemma"@))
    } else if f[0] == ascii("unit"@) {
        if !unit_ids().contains(f[2]) {
            Result::Err(ascii("unit id"@))
        } else if typed_noun(t, f[1]) {
            Result::Err(ascii("duplicate noun"@))
        } else {
            Result::Ok(Vocabulary { units: t.units.push((f[1], f[2])), ..t })
        }
    } else if f[0] == ascii("relation"@) {
        if !role_ids().contains(f[2]) {
            Result::Err(ascii("role id"@))
        } else if keys(t.relations).contains(f[1]) || t.frequencies.contains(f[1])
            || t.ranges.contains(f[1]) || t.alternatives.contains(f[1]) {
            Result::Err(ascii("duplicate preposition"@))
        } else {
            Result::Ok(Vocabulary { relations: t.relations.push((f[1], f[2])), ..t })
        }
    } else if f[0] == ascii("spacing"@) {
        match frame_row(t, f) {
            Option::Some(why) => Result::Err(why),
            Option::None => Result::Ok(Vocabulary { spacings: t.spacings.push((f[1], f[2])), ..t }),
        }
    } else if f[0] == ascii("window"@) {
        match frame_row(t, f) {
            Option::Some(why) => Result::Err(why),
            Option::None => Result::Ok(Vocabulary { windows: t.windows.push((f[1], f[2])), ..t }),
        }
    } else if f[0] == ascii("frequency"@) {
        if f[2] != ascii("period"@) {
            Result::Err(ascii("frequency value"@))
        } else if t.frequencies.contains(f[1]) || keys(t.relations).contains(f[1])
            || t.ranges.contains(f[1]) || t.alternatives.contains(f[1]) {
            Result::Err(ascii("duplicate preposition"@))
        } else {
            Result::Ok(Vocabulary { frequencies: t.frequencies.push(f[1]), ..t })
        }
    } else if f[0] == ascii("approximation"@) {
        if f[2] != ascii("about"@) {
            Result::Err(ascii("approximation value"@))
        } else if t.approximations.contains(f[1]) || t.preferences.contains(f[1]) || keys(
            t.qualifiers,
        ).contains(f[1]) {
            Result::Err(ascii("duplicate adjective"@))
        } else {
            Result::Ok(Vocabulary { approximations: t.approximations.push(f[1]), ..t })
        }
    } else if f[0] == ascii("range"@) {
        if f[2] != ascii("minimum"@) {
            Result::Err(ascii("range value"@))
        } else if t.ranges.contains(f[1]) || keys(t.relations).contains(f[1])
            || t.frequencies.contains(f[1]) || t.alternatives.contains(f[1]) {
            Result::Err(ascii("duplicate preposition"@))
        } else {
            Result::Ok(Vocabulary { ranges: t.ranges.push(f[1]), ..t })
        }
    } else if f[0] == ascii("strength"@) {
        match grade_row(t, f, strength_ids(), ascii("strength value"@)) {
            Option::Some(why) => Result::Err(why),
            Option::None => Result::Ok(
                Vocabulary { strengths: t.strengths.push((f[1], f[2])), ..t },
            ),
        }
    } else if f[0] == ascii("evidence"@) {
        match grade_row(t, f, evidence_ids(), ascii("evidence value"@)) {
            Option::Some(why) => Result::Err(why),
            Option::None => Result::Ok(
                Vocabulary { evidences: t.evidences.push((f[1], f[2])), ..t },
            ),
        }
    } else if f[0] == ascii("preference"@) {
        if f[2] != ascii("preferred"@) {
            Result::Err(ascii("preference value"@))
        } else if t.preferences.contains(f[1]) || t.approximations.contains(f[1]) || keys(
            t.qualifiers,
        ).contains(f[1]) {
            Result::Err(ascii("duplicate adjective"@))
        } else {
            Result::Ok(Vocabulary { preferences: t.preferences.push(f[1]), ..t })
        }
    } else if f[0] == ascii("alternative"@) {
        if f[2] != ascii("alternative"@) {
            Result::Err(ascii("alternative value"@))
        } else if t.alternatives.contains(f[1]) || time_prepositions(t).contains(f[1]) {
            Result::Err(ascii("duplicate preposition"@))
        } else {
            Result::Ok(Vocabulary { alternatives: t.alternatives.push(f[1]), ..t })
        }
    } else if f[0] == ascii("qualifier"@) {
        if !qualifier_ids().contains(f[2]) {
            Result::Err(ascii("qualifier value"@))
        } else if keys(t.qualifiers).contains(f[1]) {
            Result::Err(ascii("duplicate qualifier"@))
        } else if t.approximations.contains(f[1]) || t.preferences.contains(f[1]) {
            Result::Err(ascii("duplicate adjective"@))
        } else {
            Result::Ok(Vocabulary { qualifiers: t.qualifiers.push((f[1], f[2])), ..t })
        }
    } else if f[0] == ascii("scope"@) {
        if !name_ok(f[2]) {
            Result::Err(ascii("scope value"@))
        } else if t.scopes.len() > 0 {
            Result::Err(ascii("duplicate scope"@))
        } else if typed_noun(t, f[1]) {
            Result::Err(ascii("duplicate noun"@))
        } else {
            Result::Ok(Vocabulary { scopes: t.scopes.push((f[1], f[2])), ..t })
        }
    } else if f[0] == ascii("exclusion"@) {
        if f[2] != ascii("scope"@) {
            Result::Err(ascii("exclusion value"@))
        } else if t.exclusions.len() > 0 {
            Result::Err(ascii("duplicate exclusion"@))
        } else {
            Result::Ok(Vocabulary { exclusions: t.exclusions.push(f[1]), ..t })
        }
    } else {
        Result::Err(ascii("kind"@))
    }
}

// The 1-based row of the first body line of kind `kind` (0 = none).
pub open spec fn kind_row(rows: Seq<Seq<u8>>, kind: Seq<u8>, n: nat) -> nat
    decreases rows.len(),
{
    if rows.len() == 0 {
        0
    } else if split_on(rows[0], 0x09)[0] == kind {
        n
    } else {
        kind_row(rows.drop_first(), kind, n + 1)
    }
}

// cs1 D1: a scope row and an exclusion row come as a pair; a lone row reports.
pub open spec fn pairing(t: Vocabulary, rows: Seq<Seq<u8>>) -> Option<Seq<u8>> {
    if t.scopes.len() > 0 && t.exclusions.len() == 0 {
        Option::Some(
            ascii("scope without exclusion at row "@) + udec_bytes(
                kind_row(rows, ascii("scope"@), 1),
            ),
        )
    } else if t.exclusions.len() > 0 && t.scopes.len() == 0 {
        Option::Some(
            ascii("exclusion without scope at row "@) + udec_bytes(
                kind_row(rows, ascii("exclusion"@), 1),
            ),
        )
    } else {
        Option::None
    }
}

// cs1 D1: a scoped table holds the scope + exclusion pair.
pub open spec fn scoped(t: Vocabulary) -> bool {
    t.scopes.len() > 0 && t.exclusions.len() > 0
}

pub open spec fn add_rows(t: Vocabulary, rows: Seq<Seq<u8>>, n: nat) -> Result<Vocabulary, Seq<u8>>
    decreases rows.len(),
{
    if rows.len() == 0 {
        Result::Ok(t)
    } else {
        match add_row(t, rows[0]) {
            Result::Err(why) => Result::Err(why + ascii(" at row "@) + udec_bytes(n)),
            Result::Ok(t2) => add_rows(t2, rows.drop_first(), n + 1),
        }
    }
}

// The table, or the first violation: header, ≥1 row, final newline, rows in order.
pub open spec fn parse_vocabulary(bytes: Seq<u8>) -> Result<Vocabulary, Seq<u8>> {
    if !starts(bytes, vocabulary_header()) {
        Result::Err(ascii("header"@))
    } else {
        let body = bytes.skip(vocabulary_header().len() as int);
        if body.len() == 0 {
            Result::Err(ascii("no rows"@))
        } else if body.last() != 0x0A {
            Result::Err(ascii("final newline"@))
        } else {
            match add_rows(empty_table(), body_lines(body), 1) {
                Result::Err(why) => Result::Err(why),
                Result::Ok(t) => match pairing(t, body_lines(body)) {
                    Option::Some(why) => Result::Err(why),
                    Option::None => Result::Ok(t),
                },
            }
        }
    }
}

// cs1 D1: the guideline id a corpus table path names (`guidelines/<id>/vocabulary.tsv`).
pub open spec fn guideline_of(path: Seq<u8>) -> Option<Seq<u8>> {
    let pre = ascii("guidelines/"@);
    let post = ascii("/vocabulary.tsv"@);
    if path.len() > pre.len() + post.len() && path.take(pre.len() as int) == pre && path.skip(
        path.len() - post.len(),
    ) == post {
        let id = path.subrange(pre.len() as int, path.len() - post.len());
        if all_in(id, |b: u8| b != 0x2F) {
            Option::Some(id)
        } else {
            Option::None
        }
    } else {
        Option::None
    }
}

// The key law (cs1 D1): a corpus table's scope key names its own guideline.
pub open spec fn key_law(path: Seq<u8>, t: Vocabulary, bytes: Seq<u8>) -> Option<Seq<u8>> {
    match guideline_of(path) {
        Option::Some(id) => if t.scopes.len() > 0 && t.scopes[0].1 != id {
            Option::Some(
                ascii("scope key at row "@) + udec_bytes(
                    kind_row(
                        body_lines(bytes.skip(vocabulary_header().len() as int)),
                        ascii("scope"@),
                        1,
                    ),
                ),
            )
        } else {
            Option::None
        },
        Option::None => Option::None,
    }
}

// `ckc check` section: path = the repo-relative table path.
pub open spec fn vocabulary_check(path: Seq<u8>, bytes: Seq<u8>) -> Verdict {
    match parse_vocabulary(bytes) {
        Result::Ok(t) => match key_law(path, t, bytes) {
            Option::Some(why) => fail("vocabulary"@, path + ascii(": "@) + why),
            Option::None => Verdict::Ok(
                ascii("ckc: vocabulary ok "@) + path + ascii(" "@) + udec_bytes(row_count(t))
                    + ascii(" rows\n"@),
            ),
        },
        Result::Err(why) => fail("vocabulary"@, path + ascii(": "@) + why),
    }
}

} // verus!

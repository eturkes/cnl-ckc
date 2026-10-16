use crate::check::*;
use crate::v1text::*;
use vstd::prelude::*;
use vstd::utf8::*;

verus! {

// Trusted spec: the per-guideline vocabulary table `vocabulary.tsv` (contracts
// m7t D1, q12 D1, q13 D1, c1 D4, s1 D2). Corpus data maps source lemmas onto closed
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
// preposition that joins the option a preference names as its alternative.
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

// Every preposition a row names outside the alternative kind.
pub open spec fn time_prepositions(t: Vocabulary) -> Seq<Seq<u8>> {
    keys(t.relations) + keys(t.spacings) + keys(t.windows) + t.frequencies + t.ranges
}

pub open spec fn row_count(t: Vocabulary) -> nat {
    t.units.len() + t.relations.len() + t.spacings.len() + t.windows.len() + t.frequencies.len()
        + t.approximations.len() + t.ranges.len() + t.strengths.len() + t.evidences.len()
        + t.preferences.len() + t.alternatives.len()
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
    ).contains(f[2]) || grade_nouns(t).contains(f[2]) {
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
    } else if grade_nouns(t).contains(f[1]) || keys(t.units).contains(f[1]) || frames(t).contains(
        f[1],
    ) {
        Option::Some(ascii("duplicate noun"@))
    } else {
        Option::None
    }
}

// One body row added to the table, or the grammar it breaks. A row starting
// with `#` is a comment, never a row (a pre-c1 two-line header fails at row 1).
// A noun is one of unit, frame and grade noun; each lemma maps once per kind; a
// preposition maps to at most one of relation, frequency and range, and an
// alternative preposition to no other kind; an adjective is an approximation
// or a preference, never both. The later row of a conflict reports it.
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
        } else if keys(t.units).contains(f[1]) || frames(t).contains(f[1]) || grade_nouns(
            t,
        ).contains(f[1]) {
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
        } else if t.approximations.contains(f[1]) || t.preferences.contains(f[1]) {
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
        } else if t.preferences.contains(f[1]) || t.approximations.contains(f[1]) {
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
    } else {
        Result::Err(ascii("kind"@))
    }
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
            add_rows(empty_table(), body_lines(body), 1)
        }
    }
}

// `ckc check` section: path = the repo-relative table path.
pub open spec fn vocabulary_check(path: Seq<u8>, bytes: Seq<u8>) -> Verdict {
    match parse_vocabulary(bytes) {
        Result::Ok(t) => Verdict::Ok(
            ascii("ckc: vocabulary ok "@) + path + ascii(" "@) + udec_bytes(row_count(t)) + ascii(
                " rows\n"@,
            ),
        ),
        Result::Err(why) => fail("vocabulary"@, path + ascii(": "@) + why),
    }
}

} // verus!

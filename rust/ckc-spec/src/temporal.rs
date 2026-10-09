use crate::check::*;
use crate::v1text::*;
use vstd::prelude::*;
use vstd::utf8::*;

verus! {

// Trusted spec: the per-guideline temporal vocabulary `temporal.tsv` (contracts
// m7t D1, q12 D1). Corpus data maps source lemmas onto closed schema ids, so the
// compiler names no source word: unit rows map a noun lemma to a unit id,
// relation rows map a preposition lemma to an interval role, spacing rows pair
// a preposition with the frame noun of a recurrence (`at an interval of …`).
// The header selects the schema version: v2 = those three kinds; v3 adds window
// rows (the frame of a scoped recurrence, `during a window of …`) and frequency
// rows (the preposition of a count per period, `per 1 day`).
pub ghost struct Temporal {
    pub version: nat,  // 2 | 3
    pub units: Seq<(Seq<u8>, Seq<u8>)>,
    pub relations: Seq<(Seq<u8>, Seq<u8>)>,
    pub spacings: Seq<(Seq<u8>, Seq<u8>)>,
    pub windows: Seq<(Seq<u8>, Seq<u8>)>,  // v3
    pub frequencies: Seq<Seq<u8>>,  // v3: preposition lemmas
}

pub open spec fn temporal_header() -> Seq<u8> {
    ascii(
        "# format: kind\tlemma\tvalue\n# kind: unit (value: second|minute|hour|day|week|month|year) | relation (value: duration|within|after|before) | spacing (value: frame noun lemma)\n"@,
    )
}

// The v3 header names every v3 kind; approximation + range rows parse from q13.
pub open spec fn temporal_header_v3() -> Seq<u8> {
    ascii(
        "# format: kind\tlemma\tvalue\n# kind: unit (value: second|minute|hour|day|week|month|year) | relation (value: duration|within|after|before) | spacing (value: frame noun lemma) | window (value: frame noun lemma) | frequency (value: period) | approximation (value: about) | range (value: minimum)\n"@,
    )
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

// A lemma field: nonempty UTF-8 with no space, control or DEL byte (the
// compiler matches it against the parser's decoded lemmas).
pub open spec fn lemma_ok(l: Seq<u8>) -> bool {
    l.len() > 0 && all_in(l, |b: u8| b > 0x20 && b != 0x7F) && valid_utf8(l)
}

pub open spec fn keys(rows: Seq<(Seq<u8>, Seq<u8>)>) -> Seq<Seq<u8>> {
    rows.map_values(|r: (Seq<u8>, Seq<u8>)| r.0)
}

// Frame nouns: spacing frames, then window frames.
pub open spec fn frames(t: Temporal) -> Seq<Seq<u8>> {
    t.spacings.map_values(|r: (Seq<u8>, Seq<u8>)| r.1) + t.windows.map_values(
        |r: (Seq<u8>, Seq<u8>)| r.1,
    )
}

pub open spec fn row_count(t: Temporal) -> nat {
    t.units.len() + t.relations.len() + t.spacings.len() + t.windows.len() + t.frequencies.len()
}

pub open spec fn empty_table(version: nat) -> Temporal {
    Temporal {
        version,
        units: Seq::empty(),
        relations: Seq::empty(),
        spacings: Seq::empty(),
        windows: Seq::empty(),
        frequencies: Seq::empty(),
    }
}

// A frame row (spacing | window): (preposition, frame noun) once across both
// kinds; a frame noun is never a unit lemma.
pub open spec fn frame_row(t: Temporal, f: Seq<Seq<u8>>) -> Option<Seq<u8>> {
    if !lemma_ok(f[2]) {
        Option::Some(ascii("frame lemma"@))
    } else if t.spacings.contains((f[1], f[2])) || t.windows.contains((f[1], f[2])) || keys(
        t.units,
    ).contains(f[2]) {
        Option::Some(ascii("duplicate noun"@))
    } else {
        Option::None
    }
}

// One body row added to the table, or the grammar it breaks. A noun is a unit
// or a frame, never both; each lemma maps once per kind; a v3 frequency
// preposition is never a relation preposition.
pub open spec fn add_row(t: Temporal, line: Seq<u8>) -> Result<Temporal, Seq<u8>> {
    let f = split_on(line, 0x09);
    if f.len() != 3 {
        Result::Err(ascii("field count"@))
    } else if !lemma_ok(f[1]) {
        Result::Err(ascii("lemma"@))
    } else if f[0] == ascii("unit"@) {
        if !unit_ids().contains(f[2]) {
            Result::Err(ascii("unit id"@))
        } else if keys(t.units).contains(f[1]) || frames(t).contains(f[1]) {
            Result::Err(ascii("duplicate noun"@))
        } else {
            Result::Ok(Temporal { units: t.units.push((f[1], f[2])), ..t })
        }
    } else if f[0] == ascii("relation"@) {
        if !role_ids().contains(f[2]) {
            Result::Err(ascii("role id"@))
        } else if keys(t.relations).contains(f[1]) || t.frequencies.contains(f[1]) {
            Result::Err(ascii("duplicate preposition"@))
        } else {
            Result::Ok(Temporal { relations: t.relations.push((f[1], f[2])), ..t })
        }
    } else if f[0] == ascii("spacing"@) {
        match frame_row(t, f) {
            Option::Some(why) => Result::Err(why),
            Option::None => Result::Ok(Temporal { spacings: t.spacings.push((f[1], f[2])), ..t }),
        }
    } else if t.version == 3 && f[0] == ascii("window"@) {
        match frame_row(t, f) {
            Option::Some(why) => Result::Err(why),
            Option::None => Result::Ok(Temporal { windows: t.windows.push((f[1], f[2])), ..t }),
        }
    } else if t.version == 3 && f[0] == ascii("frequency"@) {
        if f[2] != ascii("period"@) {
            Result::Err(ascii("frequency value"@))
        } else if t.frequencies.contains(f[1]) || keys(t.relations).contains(f[1]) {
            Result::Err(ascii("duplicate preposition"@))
        } else {
            Result::Ok(Temporal { frequencies: t.frequencies.push(f[1]), ..t })
        }
    } else {
        Result::Err(ascii("kind"@))
    }
}

pub open spec fn add_rows(t: Temporal, rows: Seq<Seq<u8>>, n: nat) -> Result<Temporal, Seq<u8>>
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

// The table, or the first violation: header (v2 or v3), final newline, rows in
// order, ≥1 row.
pub open spec fn parse_temporal(bytes: Seq<u8>) -> Result<Temporal, Seq<u8>> {
    let v = if starts(bytes, temporal_header()) {
        2nat
    } else if starts(bytes, temporal_header_v3()) {
        3nat
    } else {
        0nat
    };
    if v == 0 {
        Result::Err(ascii("header"@))
    } else {
        let h = if v == 2 {
            temporal_header()
        } else {
            temporal_header_v3()
        };
        let body = bytes.skip(h.len() as int);
        if body.len() == 0 {
            Result::Err(ascii("no rows"@))
        } else if body.last() != 0x0A {
            Result::Err(ascii("final newline"@))
        } else {
            add_rows(empty_table(v), body_lines(body), 1)
        }
    }
}

// `ckc check` section: path = the repo-relative table path.
pub open spec fn temporal_check(path: Seq<u8>, bytes: Seq<u8>) -> Verdict {
    match parse_temporal(bytes) {
        Result::Ok(t) => Verdict::Ok(
            ascii("ckc: temporal ok "@) + path + ascii(" "@) + udec_bytes(row_count(t)) + ascii(
                " rows\n"@,
            ),
        ),
        Result::Err(why) => fail("temporal"@, path + ascii(": "@) + why),
    }
}

} // verus!

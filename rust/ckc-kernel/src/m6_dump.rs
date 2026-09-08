use crate::k2_engine::{args_roots, literal_matches};
use crate::k2_term::{ENode, ETermArena};
#[cfg(verus_keep_ghost)]
use crate::k2_term::{arena_ok, root_ok};
#[cfg(verus_keep_ghost)]
use crate::v1_term_impl::*;
use crate::v1_term_impl::{
    ESpannedTerm, EVarTracker, GTermExpected, new_var_tracker, parse_term, track_parsed_term,
};
use ckc_spec::emit::Dump;
use ckc_spec::term::Term;
#[cfg(verus_keep_ghost)]
use ckc_spec::{emit::*, term::*, v1text::*};
use vstd::assert_seqs_equal;
use vstd::prelude::*;
use vstd::slice::slice_to_vec;

verus! {

pub struct EDump {
    pub sentences: usize,
    pub drs: usize,
    pub messages: usize,
    pub model: Ghost<Dump>,
}

impl View for EDump {
    type V = Dump;

    open spec fn view(&self) -> Dump {
        self.model@
    }
}

pub open spec fn dump_roots(nodes: Seq<ENode>, d: &EDump) -> bool {
    &&& d.sentences < nodes.len() && d.drs < nodes.len() && d.messages < nodes.len()
    &&& nodes[d.sentences as int].term@ == d@.sentences
    &&& nodes[d.drs as int].term@ == d@.drs
    &&& nodes[d.messages as int].term@ == d@.messages
}

pub proof fn dump_prefix(before: Seq<ENode>, after: Seq<ENode>, d: &EDump)
    requires
        before.is_prefix_of(after),
        dump_roots(before, d),
    ensures
        dump_roots(after, d),
{
}

pub open spec fn dump_term(d: Dump, i: int) -> Term {
    if i == 0 {
        Term::Comp(ascii("sentences"@), seq![d.sentences])
    } else if i == 1 {
        Term::Comp(ascii("drs"@), seq![d.drs])
    } else {
        Term::Comp(ascii("messages"@), seq![d.messages])
    }
}

proof fn wrapper_laws(name: Seq<u8>, t: Term)
    ensures
        wf_term(Term::Comp(name, seq![t])) == wf_term(t),
        no_dollar_var(Term::Comp(name, seq![t])) == (name != dollar_var_name() && no_dollar_var(t)),
        var_stream(Term::Comp(name, seq![t])) == var_stream(t),
{
    assert(seq![t].drop_first() == Seq::<Term>::empty());
    assert(wf_terms(seq![t]) == wf_term(t)) by {
        reveal_with_fuel(wf_terms, 2);
    }
    assert(no_dollar_var_all(seq![t]) == no_dollar_var(t)) by {
        reveal_with_fuel(no_dollar_var_all, 2);
    }
    assert(var_stream_all(seq![t]) == var_stream(t)) by {
        reveal_with_fuel(var_stream_all, 2);
    }
    assert(wf_term(Term::Comp(name, seq![t])) == wf_terms(seq![t])) by {
        reveal(wf_term);
    }
    assert(no_dollar_var(Term::Comp(name, seq![t])) == (name != dollar_var_name()
        && no_dollar_var_all(seq![t]))) by {
        reveal(no_dollar_var);
    }
    assert(var_stream(Term::Comp(name, seq![t])) == var_stream_all(seq![t])) by {
        reveal(var_stream);
    }
}

proof fn dump_guides(bytes: Seq<u8>, d: Dump)
    requires
        wf_dump(d),
        print_dump(d) == bytes,
        bytes.len() <= usize::MAX,
    ensures
        wf_term(dump_term(d, 0)),
        wf_term(dump_term(d, 1)),
        wf_term(dump_term(d, 2)),
        term_keys_fit(dump_term(d, 0)),
        term_keys_fit(dump_term(d, 1)),
        term_keys_fit(dump_term(d, 2)),
        no_dollar_var(dump_term(d, 0)),
        no_dollar_var(dump_term(d, 1)),
        no_dollar_var(dump_term(d, 2)),
{
    wrapper_laws(ascii("sentences"@), d.sentences);
    wrapper_laws(ascii("drs"@), d.drs);
    wrapper_laws(ascii("messages"@), d.messages);
    reveal_strlit("sentences");
    reveal_strlit("drs");
    reveal_strlit("messages");
    reveal(ascii);
    reveal(dollar_var_name);
    term_var_lengths(dump_term(d, 0));
    term_var_lengths(dump_term(d, 1));
    term_var_lengths(dump_term(d, 2));
    let a = var_stream(dump_term(d, 0));
    let b = var_stream(dump_term(d, 1));
    let c = var_stream(dump_term(d, 2));
    assert(a + b + c == var_stream(d.sentences) + var_stream(d.drs) + var_stream(d.messages));
    assert((a + b + c).len() <= bytes.len());
    canonical_stream_keys_fit(a + b + c);
    stream_keys_fit_split(a + b, c);
    stream_keys_fit_split(a, b);
    term_keys_from_stream(dump_term(d, 0));
    term_keys_from_stream(dump_term(d, 1));
    term_keys_from_stream(dump_term(d, 2));
}

pub open spec fn line_at(bytes: Seq<u8>, start: int, t: Term) -> bool {
    &&& 0 <= start
    &&& start + term_line(t).len() <= bytes.len()
    &&& bytes.subrange(start, start + term_line(t).len()) == term_line(t)
    &&& wf_term(t)
    &&& term_keys_fit(t)
}

proof fn line_term_at(bytes: Seq<u8>, start: int, t: Term)
    requires
        line_at(bytes, start, t),
    ensures
        term_at(bytes, start, start + term_bytes(t).len(), t),
        start + term_bytes(t).len() + 2 <= bytes.len(),
        bytes[start + term_bytes(t).len()] == 0x2eu8,
        bytes[start + term_bytes(t).len() + 1] == 0x0au8,
        term_line(t).len() == term_bytes(t).len() + 2,
{
    reveal(line_at);
    reveal(term_line);
    reveal_strlit(".\n");
    reveal(ascii);
    term_bytes_nonempty(t);
    let end = start + term_bytes(t).len();
    assert(bytes.subrange(start, end + 2)[term_bytes(t).len() as int] == 0x2eu8);
    assert(bytes.subrange(start, end + 2)[term_bytes(t).len() as int + 1] == 0x0au8);
    assert(bytes[end] == 0x2eu8);
    assert(bytes[end + 1] == 0x0au8);
    assert_seqs_equal!(bytes.subrange(start, end) == term_bytes(t));
    reveal(term_boundary);
}

fn parse_line(
    bytes: &[u8],
    start: usize,
    arena: &mut ETermArena,
    Ghost(expected): Ghost<Option<Term>>,
    tracker: &mut EVarTracker,
) -> (r: Option<(ESpannedTerm, usize)>)
    requires
        start <= bytes.len(),
        arena_ok(old(arena)),
        expected matches Some(t) ==> line_at(bytes@, start as int, t),
        old(tracker).valid ==> tracker_state_ok(old(tracker).next, old(tracker).stream@),
        tracker_complete(old(tracker).valid, old(tracker).stream@),
        old(tracker).stream@.len() <= start,
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        final(tracker).valid ==> tracker_state_ok(final(tracker).next, final(tracker).stream@),
        tracker_complete(final(tracker).valid, final(tracker).stream@),
        r matches Some((t, next)) ==> {
            &&& spanned_term_ok(bytes@, &t) && spanned_root_ok(final(arena), &t)
            &&& t.start == start && t.end + 2 == next && next <= bytes.len()
            &&& bytes@.subrange(start as int, next as int) == term_line(t@)
            &&& final(tracker).stream@ == old(tracker).stream@ + var_stream(t@)
            &&& final(tracker).stream@.len() <= next
        },
        expected matches Some(t) ==> r matches Some((parsed, next)) && parsed@ == t && next == start
            + term_line(t).len(),
{
    proof {
        reveal_strlit(".\n");
        reveal(ascii);
        reveal(term_line);
    }
    let ghost guide = match expected {
        Some(t) => Some(GTermExpected { term: t, end: (start + term_bytes(t).len()) as usize }),
        None => None,
    };
    proof {
        if let Some(t) = expected {
            line_term_at(bytes@, start as int, t);
        }
    }
    if start == bytes.len() {
        return None;
    }
    let mut at = 0usize;
    let mut scratch_tracker = new_var_tracker();
    let parsed = match parse_term(
        bytes,
        start,
        arena,
        Ghost(guide),
        Ghost(GTermExpected { term: Term::Nil, end: 0 }),
        Ghost(Seq::empty()),
        false,
        &mut scratch_tracker,
        &mut at,
    ) {
        Some(t) => t,
        None => return None,
    };
    if bytes.len() - parsed.end < 2 || bytes[parsed.end] != 0x2e || bytes[parsed.end + 1] != 0x0a {
        return None;
    }
    let next = parsed.end + 2;
    proof {
        assert_seqs_equal!(bytes@.subrange(start as int, next as int) == term_line(parsed@));
        reveal(term_boundary);
        assert(term_at(bytes@, start as int, parsed.end as int, parsed@));
    }
    track_parsed_term(bytes, &parsed, tracker, &mut at);
    Some((parsed, next))
}

fn unwrap_line(arena: &ETermArena, line: &ESpannedTerm, name: &[u8]) -> (r: Option<usize>)
    requires
        arena_ok(arena),
        spanned_root_ok(arena, line),
    ensures
        r matches Some(root) ==> root < arena.nodes.len() && line@ == Term::Comp(
            name@,
            seq![arena@[root as int]],
        ),
        (exists|t: Term| line@ == Term::Comp(name@, seq![t])) ==> r is Some,
{
    let label = slice_to_vec(name);
    if !literal_matches(arena, line.root, &label, 1) {
        return None;
    }
    let args = args_roots(arena, line.root);
    proof {
        assert(args.len() == 1);
        let term = arena@[line.root as int];
        assert(term is Comp);
        assert_seqs_equal!(ckc_spec::engine::args_of(term) == seq![arena@[args@[0] as int]]);
    }
    Some(args[0])
}

pub fn parse_dump(bytes: &[u8], arena: &mut ETermArena) -> (r: Option<EDump>)
    requires
        arena_ok(old(arena)),
    ensures
        arena_ok(final(arena)),
        old(arena).nodes@.is_prefix_of(final(arena).nodes@),
        r.is_some() == dump_accepts(bytes@),
        r matches Some(d) ==> dump_roots(final(arena).nodes@, &d) && d@ == the_dump(bytes@)
            && wf_dump(d@) && print_dump(d@) == bytes@,
{
    let ghost origin = arena.nodes@;
    let ghost expected = if dump_accepts(bytes@) {
        Some(the_dump(bytes@))
    } else {
        None
    };
    proof {
        if let Some(d) = expected {
            reveal(dump_accepts);
            reveal(the_dump);
            assert(wf_dump(d) && print_dump(d) == bytes@);
            dump_guides(bytes@, d);
            assert(line_at(bytes@, 0, dump_term(d, 0))) by {
                assert_seqs_equal!(bytes@.subrange(0, term_line(dump_term(d, 0)).len() as int) == term_line(dump_term(d, 0)));
            }
        }
    }
    let mut tracker = new_var_tracker();
    let ghost e0 = match expected {
        Some(d) => Some(dump_term(d, 0)),
        None => None,
    };
    let (l0, p1) = match parse_line(bytes, 0, arena, Ghost(e0), &mut tracker) {
        Some(v) => v,
        None => return None,
    };
    let ghost n0 = arena.nodes@;
    let label0: &[u8] = b"sentences";
    proof {
        reveal_byteslit(b"sentences");
        reveal_strlit("sentences");
        reveal(ascii);
        assert_seqs_equal!(label0@ == ascii("sentences"@));
        if let Some(e) = expected {
            assert(l0@ == Term::Comp(label0@, seq![e.sentences]));
            assert(exists|v: Term| l0@ == Term::Comp(label0@, seq![v])) by {
                assert(l0@ == Term::Comp(label0@, seq![e.sentences]));
            }
        }
    }
    let sentences = match unwrap_line(arena, &l0, label0) {
        Some(root) => root,
        None => return None,
    };
    if !l0.parsed.no_dollar {
        return None;
    }
    proof {
        if let Some(d) = expected {
            assert(line_at(bytes@, p1 as int, dump_term(d, 1))) by {
                assert_seqs_equal!(bytes@.subrange(p1 as int, p1 + term_line(dump_term(d, 1)).len()) == term_line(dump_term(d, 1)));
            }
        }
    }
    let ghost e1 = match expected {
        Some(d) => Some(dump_term(d, 1)),
        None => None,
    };
    let (l1, p2) = match parse_line(bytes, p1, arena, Ghost(e1), &mut tracker) {
        Some(v) => v,
        None => return None,
    };
    proof {
        crate::k2_load::prefix_chain(origin, n0, arena.nodes@);
    }
    let ghost n1 = arena.nodes@;
    let label1: &[u8] = b"drs";
    proof {
        reveal_byteslit(b"drs");
        reveal_strlit("drs");
        reveal(ascii);
        assert_seqs_equal!(label1@ == ascii("drs"@));
        if let Some(e) = expected {
            assert(l1@ == Term::Comp(label1@, seq![e.drs]));
            assert(exists|v: Term| l1@ == Term::Comp(label1@, seq![v])) by {
                assert(l1@ == Term::Comp(label1@, seq![e.drs]));
            }
        }
    }
    let drs = match unwrap_line(arena, &l1, label1) {
        Some(root) => root,
        None => return None,
    };
    if !l1.parsed.no_dollar {
        return None;
    }
    proof {
        if let Some(d) = expected {
            assert(line_at(bytes@, p2 as int, dump_term(d, 2))) by {
                assert_seqs_equal!(bytes@.subrange(p2 as int, p2 + term_line(dump_term(d, 2)).len()) == term_line(dump_term(d, 2)));
            }
        }
    }
    let ghost e2 = match expected {
        Some(d) => Some(dump_term(d, 2)),
        None => None,
    };
    let (l2, p3) = match parse_line(bytes, p2, arena, Ghost(e2), &mut tracker) {
        Some(v) => v,
        None => return None,
    };
    proof {
        crate::k2_load::prefix_chain(origin, n1, arena.nodes@);
        crate::k2_load::prefix_chain(n0, n1, arena.nodes@);
    }
    let label2: &[u8] = b"messages";
    proof {
        reveal_byteslit(b"messages");
        reveal_strlit("messages");
        reveal(ascii);
        assert_seqs_equal!(label2@ == ascii("messages"@));
        if let Some(e) = expected {
            assert(l2@ == Term::Comp(label2@, seq![e.messages]));
            assert(exists|v: Term| l2@ == Term::Comp(label2@, seq![v])) by {
                assert(l2@ == Term::Comp(label2@, seq![e.messages]));
            }
        }
    }
    let messages = match unwrap_line(arena, &l2, label2) {
        Some(root) => root,
        None => return None,
    };
    if !l2.parsed.no_dollar || p3 != bytes.len() {
        return None;
    }
    proof {
        assert(n0[sentences as int] == arena.nodes@[sentences as int]);
        assert(n1[drs as int] == arena.nodes@[drs as int]);
    }
    let ghost d = Dump {
        sentences: arena@[sentences as int],
        drs: arena@[drs as int],
        messages: arena@[messages as int],
    };
    proof {
        wrapper_laws(ascii("sentences"@), d.sentences);
        wrapper_laws(ascii("drs"@), d.drs);
        wrapper_laws(ascii("messages"@), d.messages);
        assert(l0@ == dump_term(d, 0));
        assert(l1@ == dump_term(d, 1));
        assert(l2@ == dump_term(d, 2));
        assert(tracker.stream@ == var_stream(d.sentences) + var_stream(d.drs) + var_stream(
            d.messages,
        ));
        assert_seqs_equal!(print_dump(d) == bytes@);
        if let Some(e) = expected {
            assert(d == e);
        }
    }
    if !tracker.valid {
        proof {
            if let Some(e) = expected {
                assert(false);
            }
        }
        return None;
    }
    proof {
        tracker_state_canonical(tracker.next, tracker.stream@);
        assert(wf_dump(d));
        assert(dump_accepts(bytes@)) by {
            assert(wf_dump(d) && print_dump(d) == bytes@);
        }
        assert(expected is Some);
        assert(d == the_dump(bytes@));
    }
    Some(EDump { sentences, drs, messages, model: Ghost(d) })
}

} // verus!

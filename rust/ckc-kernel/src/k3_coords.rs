#[cfg(verus_keep_ghost)]
use crate::k2_term::root_ok;
use crate::k2_term::{ENodeKind, EOrder, ETermArena};
use crate::k3_front::{bytes_equal, is_comp};
use crate::v1_term_impl::EParsedV1;
#[cfg(verus_keep_ghost)]
use ckc_spec::term::Term;
use ckc_spec::trace::*;
use ckc_spec::v1text::*;
use vstd::assert_seqs_equal;
use vstd::prelude::*;
use vstd::slice::slice_to_vec;

verus! {

pub struct ECoord {
    pub docid: Vec<u8>,
    pub ordinal: Vec<u8>,
    pub model: Ghost<Coord>,
}

pub open spec fn coord_ok(c: &ECoord) -> bool {
    c.docid@ == c.model@.docid && c.ordinal@ == udec_bytes(c.model@.s)
}

pub open spec fn coords_view(cs: Seq<ECoord>) -> Seq<Coord> {
    cs.map_values(|c: ECoord| c.model@)
}

pub open spec fn coords_ok(cs: Seq<ECoord>) -> bool {
    forall|i: int| 0 <= i < cs.len() ==> #[trigger] coord_ok(&cs[i])
}

pub open spec fn coord_option(out: Option<ECoord>) -> Option<Coord> {
    match out {
        Some(c) => Some(c.model@),
        None => None,
    }
}

pub fn make_coord(docid: &[u8], ordinal: &[u8], Ghost(s): Ghost<nat>) -> (out: ECoord)
    requires
        ordinal@ == udec_bytes(s),
    ensures
        coord_ok(&out),
        out.model@ == (Coord { docid: docid@, s }),
{
    ECoord {
        docid: slice_to_vec(docid),
        ordinal: slice_to_vec(ordinal),
        model: Ghost(Coord { docid: docid@, s }),
    }
}

fn repeated_coords(docid: &[u8], ordinal: &[u8], count: usize, Ghost(s): Ghost<nat>) -> (out: Vec<
    ECoord,
>)
    requires
        ordinal@ == udec_bytes(s),
    ensures
        coords_ok(out@),
        coords_view(out@) == Seq::new(count as nat, |i: int| Coord { docid: docid@, s }),
{
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < count
        invariant
            ordinal@ == udec_bytes(s),
            i <= count,
            out.len() == i,
            coords_ok(out@),
            coords_view(out@) == Seq::new(i as nat, |j: int| Coord { docid: docid@, s }),
        decreases count - i,
    {
        let c = make_coord(docid, ordinal, Ghost(s));
        let ghost previous = out@;
        out.push(c);
        proof {
            assert forall|j: int| 0 <= j < out.len() implies #[trigger] coord_ok(&out@[j]) by {
                if j < previous.len() {
                    assert(out@[j] == previous[j]);
                }
            }
            assert_seqs_equal!(coords_view(out@) == Seq::new(i as nat + 1, |j: int| Coord { docid: docid@, s }), j => {
                if j < previous.len() {
                    assert(out@[j] == previous[j]);
                    assert(coords_view(previous)[j] == Coord { docid: docid@, s });
                } else { assert(out@[j] == c); }
            });
        }
        i += 1;
    }
    out
}

pub proof fn coords_concat(left: Seq<ECoord>, right: Seq<ECoord>)
    requires
        coords_ok(left),
        coords_ok(right),
    ensures
        coords_ok(left + right),
        coords_view(left + right) == coords_view(left) + coords_view(right),
{
    assert forall|i: int| 0 <= i < (left + right).len() implies #[trigger] coord_ok(
        &(left + right)[i],
    ) by {
        if i < left.len() {
            assert((left + right)[i] == left[i]);
        } else {
            assert((left + right)[i] == right[i - left.len()]);
        }
    }
    assert_seqs_equal!(coords_view(left + right) == coords_view(left) + coords_view(right));
}

pub fn document_coords(parsed: &EParsedV1, Ghost(doc): Ghost<DocFile>) -> (out: Vec<ECoord>)
    requires
        crate::v1_term_impl::parsed_metadata_ok(parsed),
        parsed@ == V1File::Doc(doc),
    ensures
        coords_ok(out@),
        coords_view(out@) == doc_coords(doc),
{
    let zero: &[u8] = b"0";
    proof {
        reveal_byteslit(b"0");
        reveal_with_fuel(udec_bytes, 1);
        assert(zero@ == udec_bytes(0));
    }
    let mut out = repeated_coords(&parsed.docid, zero, 2, Ghost(0nat));
    let ghost prefix = coords_view(out@);
    let ghost parts = doc.bundles.map_values(|b: Bundle| bundle_coords(doc.docid, b));
    let mut i = 0usize;
    proof {
        assert_seqs_equal!(prefix == seq![Coord { docid: doc.docid, s: 0 }, Coord { docid: doc.docid, s: 0 }]);
        assert_seqs_equal!(parts.take(0) == Seq::empty());
        reveal_with_fuel(Seq::<_>::flatten, 1);
    }
    while i < parsed.bundle_meta.len()
        invariant
            crate::v1_term_impl::parsed_metadata_ok(parsed),
            parsed@ == V1File::Doc(doc),
            parts == doc.bundles.map_values(|b: Bundle| bundle_coords(doc.docid, b)),
            prefix == seq![Coord { docid: doc.docid, s: 0 }, Coord { docid: doc.docid, s: 0 }],
            i <= parsed.bundle_meta.len(),
            i <= parts.len(),
            coords_ok(out@),
            coords_view(out@) == prefix + parts.take(i as int).flatten(),
        decreases parsed.bundle_meta.len() - i,
    {
        let meta = &parsed.bundle_meta[i];
        proof {
            assert(crate::v1_term_impl::bundle_metadata_ok(parsed.bundle_meta@, doc.bundles));
            assert(meta.model@ == doc.bundles[i as int]);
        }
        let mut next = repeated_coords(
            &parsed.docid,
            &meta.ordinal,
            meta.count,
            Ghost(meta.model@.s),
        );
        let ghost left = out@;
        let ghost right = next@;
        out.append(&mut next);
        proof {
            coords_concat(left, right);
            assert(coords_view(right) == parts[i as int]);
            assert_seqs_equal!(parts.take(i as int + 1) == parts.take(i as int).push(parts[i as int]));
            parts.take(i as int).lemma_flatten_push(parts[i as int]);
        }
        i += 1;
    }
    proof {
        assert_seqs_equal!(parts.take(i as int) == parts);
    }
    out
}

pub fn coord_equal(left: &ECoord, right: &ECoord) -> (out: bool)
    requires
        coord_ok(left),
        coord_ok(right),
    ensures
        out == (left.model@ == right.model@),
{
    if !crate::k2_engine::vec_equal(&left.docid, &right.docid) {
        return false;
    }
    let same = crate::k2_engine::vec_equal(&left.ordinal, &right.ordinal);
    proof {
        if same {
            crate::v1_term_impl::udec_bytes_injective(left.model@.s, right.model@.s);
        }
    }
    same
}

pub fn coord_of_exec(arena: &ETermArena, root: usize) -> (out: Option<ECoord>)
    requires
        root_ok(arena, root),
    ensures
        out matches Some(c) ==> coord_ok(&c),
        coord_option(out) == coord_of(arena@[root as int]),
{
    let sentence: &[u8] = b"sentence";
    proof {
        reveal_byteslit(b"sentence");
        reveal_strlit("sentence");
        reveal(ascii);
        assert(sentence@ == ascii("sentence"@));
    }
    if !is_comp(arena, root, sentence, 2) {
        return None;
    }
    let args = crate::k2_engine::args_roots(arena, root);
    proof {
        assert(args.len() == 2);
        assert(crate::k2_term::node_ok(arena.nodes@, args@[0] as int));
        assert(crate::k2_term::node_ok(arena.nodes@, args@[1] as int));
        reveal(crate::k2_term::node_ok);
        assert_seqs_equal!(ckc_spec::engine::args_of(arena@[root as int]) == seq![arena@[args@[0] as int], arena@[args@[1] as int]]);
        assert(arena@[root as int] == Term::Comp(
            sentence@,
            ckc_spec::engine::args_of(arena@[root as int]),
        ));
    }
    let docid = match &arena.nodes[args[0]].kind {
        ENodeKind::Atom { name } => name,
        _ => return None,
    };
    if !crate::v1_term_impl::name_ok_exec(docid) {
        return None;
    }
    match &arena.nodes[args[1]].kind {
        ENodeKind::Int { magnitude, negative, value, .. } => {
            let zero = slice_to_vec(b"0");
            let bound = slice_to_vec(b"1000000000");
            proof {
                reveal_byteslit(b"0");
                reveal_byteslit(b"1000000000");
                reveal_with_fuel(udec_bytes, 11);
                assert(zero@ == udec_bytes(0));
                assert(bound@ == udec_bytes(1000000000));
            }
            let lower = crate::k2_term::int_order(
                magnitude,
                *negative,
                *value,
                &zero,
                false,
                Ghost(0int),
            );
            if !matches!(lower, EOrder::Greater) {
                return None;
            }
            let upper = crate::k2_term::int_order(
                magnitude,
                *negative,
                *value,
                &bound,
                false,
                Ghost(1000000000int),
            );
            if !matches!(upper, EOrder::Less) {
                return None;
            }
            Some(make_coord(docid, magnitude, Ghost(value@ as nat)))
        },
        _ => None,
    }
}

pub open spec fn hex_option(out: Option<Vec<u8>>) -> Option<Seq<u8>> {
    match out {
        Some(h) => Some(h@),
        None => None,
    }
}

pub fn hex_of_exec(arena: &ETermArena, root: usize) -> (out: Option<Vec<u8>>)
    requires
        root_ok(arena, root),
    ensures
        hex_option(out) == hex_of(arena@[root as int]),
{
    let name: &[u8] = b"clause_sha256";
    proof {
        reveal_byteslit(b"clause_sha256");
        reveal_strlit("clause_sha256");
        reveal(ascii);
        assert(name@ == ascii("clause_sha256"@));
    }
    if !is_comp(arena, root, name, 1) {
        return None;
    }
    let args = crate::k2_engine::args_roots(arena, root);
    proof {
        assert(args.len() == 1);
        assert(crate::k2_term::node_ok(arena.nodes@, args@[0] as int));
        reveal(crate::k2_term::node_ok);
        assert_seqs_equal!(ckc_spec::engine::args_of(arena@[root as int]) == seq![arena@[args@[0] as int]]);
    }
    match &arena.nodes[args[0]].kind {
        ENodeKind::Atom { name: hash } => if crate::v1_term_impl::hex64_exec(hash) {
            Some(hash.clone())
        } else {
            None
        },
        _ => None,
    }
}

pub fn digest_equal(digests: &Vec<Vec<u8>>, i: usize, value: &[u8]) -> (out: bool)
    ensures
        out == (digest_at(digests_view(digests@), i as nat) == value@),
{
    if i < digests.len() {
        bytes_equal(&digests[i], value)
    } else {
        let empty = value.len() == 0;
        proof {
            if empty {
                assert_seqs_equal!(value@ == Seq::empty());
            }
        }
        empty
    }
}

pub fn join_count_exec(
    coords: &Vec<ECoord>,
    digests: &Vec<Vec<u8>>,
    coord: &ECoord,
    hash: &[u8],
) -> (out: usize)
    requires
        coords_ok(coords@),
        coord_ok(coord),
    ensures
        out as nat == join_count(
            coords_view(coords@),
            digests_view(digests@),
            coord.model@,
            hash@,
            0,
        ),
        out <= coords.len(),
{
    let ghost cs = coords_view(coords@);
    let ghost ds = digests_view(digests@);
    let mut i = 0usize;
    let mut count = 0usize;
    while i < coords.len()
        invariant
            coords_ok(coords@),
            coord_ok(coord),
            cs == coords_view(coords@),
            ds == digests_view(digests@),
            i <= coords.len(),
            count <= i,
            count as nat + join_count(cs, ds, coord.model@, hash@, i as nat) == join_count(
                cs,
                ds,
                coord.model@,
                hash@,
                0,
            ),
        decreases coords.len() - i,
    {
        if coord_equal(&coords[i], coord) && digest_equal(digests, i, hash) {
            count += 1;
        }
        proof {
            reveal_with_fuel(join_count, 1);
        }
        i += 1;
    }
    proof {
        reveal_with_fuel(join_count, 1);
    }
    count
}

} // verus!

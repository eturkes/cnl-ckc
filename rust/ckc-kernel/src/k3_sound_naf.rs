use ckc_spec::answers::*;
use ckc_spec::engine::*;
use ckc_spec::term::*;
use ckc_spec::trace::*;
use ckc_spec::v1text::*;
use vstd::prelude::*;

verus! {

pub proof fn trun_mono(db: Seq<DocClause>, c: TCfg, small: nat, large: nat)
    requires
        small <= large,
        !(trun(db, c, small).0 is Limit),
    ensures
        trun(db, c, large).0 == trun(db, c, small).0,
        trun(db, c, large).1 == trun(db, c, small).1 + large - small,
    decreases small,
{
    assert(small > 0);
    match tstep(db, c) {
        TStep::Next(c2) => trun_mono(db, c2, (small - 1) as nat, (large - 1) as nat),
        _ => {},
    }
}

pub proof fn unify_n_mono(u: UState, small: nat, large: nat)
    requires
        small <= large,
        !(unify_n(u, small) is Out),
    ensures
        unify_n(u, large) == unify_n(u, small),
    decreases small,
{
    assert(small > 0);
    if u.pairs.len() > 0 {
        let a = u.pairs[0].0;
        let b = u.pairs[0].1;
        let rest = u.pairs.drop_first();
        let sf = (small - 1) as nat;
        let lf = (large - 1) as nat;
        match (a, b) {
            (Term::Var(x), _) => {
                if a == b {
                    unify_n_mono(UState { pairs: rest, ..u }, sf, lf);
                } else if !occurs(x, b) {
                    unify_n_mono(u_bind(u, rest, x, b), sf, lf);
                }
            },
            (_, Term::Var(y)) => {
                if !occurs(y, a) {
                    unify_n_mono(u_bind(u, rest, y, a), sf, lf);
                }
            },
            (Term::Comp(n, xs), Term::Comp(m, ys)) => {
                if n == m && xs.len() == ys.len() {
                    unify_n_mono(UState { pairs: zip(xs, ys) + rest, ..u }, sf, lf);
                }
            },
            _ => {
                if a == b {
                    unify_n_mono(UState { pairs: rest, ..u }, sf, lf);
                }
            },
        }
    }
}

pub proof fn unify_n_agree(u: UState, f: nat, g: nat)
    requires
        !(unify_n(u, f) is Out),
        !(unify_n(u, g) is Out),
    ensures
        unify_n(u, f) == unify_n(u, g),
{
    if f <= g {
        unify_n_mono(u, f, g);
    } else {
        unify_n_mono(u, g, f);
    }
}

pub proof fn unify_at(u: UState, fuel: nat)
    requires
        !(unify_n(u, fuel) is Out),
    ensures
        unify(u) == unify_n(u, fuel),
{
    assert(exists|f: nat| !(unify_n(u, f) is Out));
    let g = choose|f: nat| !(unify_n(u, f) is Out);
    unify_n_agree(u, fuel, g);
}

pub open spec fn lift_var(x: nat, base: nat, delta: nat) -> nat {
    if x < base {
        x
    } else {
        x + delta
    }
}

pub open spec fn rename(t: Term, base: nat, delta: nat) -> Term
    decreases t,
{
    match t {
        Term::Var(x) => Term::Var(lift_var(x, base, delta)),
        Term::Comp(n, ts) => Term::Comp(n, rename_all(ts, base, delta)),
        _ => t,
    }
}

pub open spec fn rename_all(ts: Seq<Term>, base: nat, delta: nat) -> Seq<Term>
    decreases ts,
{
    if ts.len() == 0 {
        Seq::empty()
    } else {
        seq![rename(ts[0], base, delta)] + rename_all(ts.drop_first(), base, delta)
    }
}

pub proof fn lift_var_inj(x: nat, y: nat, base: nat, delta: nat)
    ensures
        (lift_var(x, base, delta) == lift_var(y, base, delta)) == (x == y),
{
}

pub proof fn rename_all_index(ts: Seq<Term>, base: nat, delta: nat)
    ensures
        rename_all(ts, base, delta).len() == ts.len(),
        forall|i: int|
            0 <= i < ts.len() ==> #[trigger] rename_all(ts, base, delta)[i] == rename(
                ts[i],
                base,
                delta,
            ),
    decreases ts,
{
    if ts.len() > 0 {
        rename_all_index(ts.drop_first(), base, delta);
        assert forall|i: int| 0 <= i < ts.len() implies #[trigger] rename_all(ts, base, delta)[i]
            == rename(ts[i], base, delta) by {
            if i > 0 {
                assert(ts.drop_first()[i - 1] == ts[i]);
            }
        }
    }
}

pub proof fn rename_inj(a: Term, b: Term, base: nat, delta: nat)
    ensures
        (rename(a, base, delta) == rename(b, base, delta)) == (a == b),
    decreases a,
{
    match (a, b) {
        (Term::Var(x), Term::Var(y)) => lift_var_inj(x, y, base, delta),
        (Term::Comp(n, xs), Term::Comp(m, ys)) => {
            rename_all_inj(xs, ys, base, delta);
        },
        _ => {},
    }
}

pub proof fn rename_all_inj(xs: Seq<Term>, ys: Seq<Term>, base: nat, delta: nat)
    ensures
        (rename_all(xs, base, delta) == rename_all(ys, base, delta)) == (xs == ys),
    decreases xs,
{
    rename_all_index(xs, base, delta);
    rename_all_index(ys, base, delta);
    if rename_all(xs, base, delta) == rename_all(ys, base, delta) {
        assert(xs.len() == ys.len());
        assert forall|i: int| 0 <= i < xs.len() implies xs[i] == ys[i] by {
            assert(rename_all(xs, base, delta)[i] == rename_all(ys, base, delta)[i]);
            rename_inj(xs[i], ys[i], base, delta);
        }
        assert(xs =~= ys);
    }
}

pub proof fn rename_occurs(x: nat, t: Term, base: nat, delta: nat)
    ensures
        occurs(lift_var(x, base, delta), rename(t, base, delta)) == occurs(x, t),
    decreases t,
{
    match t {
        Term::Var(y) => lift_var_inj(x, y, base, delta),
        Term::Comp(_, ts) => rename_occurs_all(x, ts, base, delta),
        _ => {},
    }
}

pub proof fn rename_occurs_all(x: nat, ts: Seq<Term>, base: nat, delta: nat)
    ensures
        occurs_all(lift_var(x, base, delta), rename_all(ts, base, delta)) == occurs_all(x, ts),
    decreases ts,
{
    rename_all_index(ts, base, delta);
    if ts.len() > 0 {
        rename_occurs(x, ts[0], base, delta);
        rename_occurs_all(x, ts.drop_first(), base, delta);
        assert(rename_all(ts, base, delta).drop_first() =~= rename_all(
            ts.drop_first(),
            base,
            delta,
        ));
    }
}

pub proof fn rename_subst(t: Term, x: nat, v: Term, base: nat, delta: nat)
    ensures
        rename(subst(t, x, v), base, delta) == subst(
            rename(t, base, delta),
            lift_var(x, base, delta),
            rename(v, base, delta),
        ),
    decreases t,
{
    match t {
        Term::Var(y) => lift_var_inj(x, y, base, delta),
        Term::Comp(_, ts) => rename_subst_all(ts, x, v, base, delta),
        _ => {},
    }
}

pub proof fn rename_subst_all(ts: Seq<Term>, x: nat, v: Term, base: nat, delta: nat)
    ensures
        rename_all(subst_all(ts, x, v), base, delta) == subst_all(
            rename_all(ts, base, delta),
            lift_var(x, base, delta),
            rename(v, base, delta),
        ),
    decreases ts,
{
    rename_all_index(ts, base, delta);
    if ts.len() > 0 {
        rename_subst(ts[0], x, v, base, delta);
        rename_subst_all(ts.drop_first(), x, v, base, delta);
        assert(subst_all(ts, x, v).drop_first() =~= subst_all(ts.drop_first(), x, v));
        assert(rename_all(ts, base, delta).drop_first() =~= rename_all(
            ts.drop_first(),
            base,
            delta,
        ));
    }
}

pub proof fn subst_all_index(ts: Seq<Term>, x: nat, v: Term)
    ensures
        subst_all(ts, x, v).len() == ts.len(),
        forall|i: int|
            0 <= i < ts.len() ==> #[trigger] subst_all(ts, x, v)[i] == subst(ts[i], x, v),
    decreases ts,
{
    if ts.len() > 0 {
        subst_all_index(ts.drop_first(), x, v);
        assert forall|i: int| 0 <= i < ts.len() implies #[trigger] subst_all(ts, x, v)[i] == subst(
            ts[i],
            x,
            v,
        ) by {
            if i > 0 {
                assert(ts.drop_first()[i - 1] == ts[i]);
            }
        }
    }
}

pub open spec fn terms_rel(xs: Seq<Term>, ys: Seq<Term>, base: nat, delta: nat) -> bool {
    xs.len() <= ys.len() && forall|i: int|
        0 <= i < xs.len() ==> #[trigger] ys[i] == rename(xs[i], base, delta)
}

pub open spec fn pairs_rel(
    xs: Seq<(Term, Term)>,
    ys: Seq<(Term, Term)>,
    base: nat,
    delta: nat,
) -> bool {
    xs.len() == ys.len() && forall|i: int|
        0 <= i < xs.len() ==> #[trigger] ys[i] == (
            rename(xs[i].0, base, delta),
            rename(xs[i].1, base, delta),
        )
}

pub open spec fn unify_rel(u: UState, v: UState, base: nat, delta: nat) -> bool {
    u.stack.len() == 0 && v.stack.len() == 0 && pairs_rel(u.pairs, v.pairs, base, delta)
        && terms_rel(u.sol, v.sol, base, delta)
}

pub open spec fn uout_rel(a: UOut, b: UOut, base: nat, delta: nat) -> bool {
    match (a, b) {
        (UOut::Ok(sa, ta), UOut::Ok(sb, tb)) => sa.len() == 0 && sb.len() == 0 && terms_rel(
            ta,
            tb,
            base,
            delta,
        ),
        (UOut::Fail, UOut::Fail) | (UOut::Out, UOut::Out) => true,
        _ => false,
    }
}

pub proof fn terms_subst_rel(xs: Seq<Term>, ys: Seq<Term>, x: nat, v: Term, base: nat, delta: nat)
    requires
        terms_rel(xs, ys, base, delta),
    ensures
        terms_rel(
            subst_all(xs, x, v),
            subst_all(ys, lift_var(x, base, delta), rename(v, base, delta)),
            base,
            delta,
        ),
{
    subst_all_index(xs, x, v);
    subst_all_index(ys, lift_var(x, base, delta), rename(v, base, delta));
    assert forall|i: int| 0 <= i < subst_all(xs, x, v).len() implies #[trigger] subst_all(
        ys,
        lift_var(x, base, delta),
        rename(v, base, delta),
    )[i] == rename(subst_all(xs, x, v)[i], base, delta) by {
        assert(ys[i] == rename(xs[i], base, delta));
        rename_subst(xs[i], x, v, base, delta);
    }
}

pub proof fn pairs_tail_rel(xs: Seq<(Term, Term)>, ys: Seq<(Term, Term)>, base: nat, delta: nat)
    requires
        pairs_rel(xs, ys, base, delta),
        xs.len() > 0,
    ensures
        pairs_rel(xs.drop_first(), ys.drop_first(), base, delta),
{
    assert forall|i: int| 0 <= i < xs.drop_first().len() implies #[trigger] ys.drop_first()[i] == (
        rename(xs.drop_first()[i].0, base, delta),
        rename(xs.drop_first()[i].1, base, delta),
    ) by {
        assert(ys[i + 1] == (rename(xs[i + 1].0, base, delta), rename(xs[i + 1].1, base, delta)));
    }
}

pub proof fn pairs_zip_rel(
    xs: Seq<Term>,
    ys: Seq<Term>,
    rest: Seq<(Term, Term)>,
    other: Seq<(Term, Term)>,
    base: nat,
    delta: nat,
)
    requires
        xs.len() == ys.len(),
        pairs_rel(rest, other, base, delta),
    ensures
        pairs_rel(
            zip(xs, ys) + rest,
            zip(rename_all(xs, base, delta), rename_all(ys, base, delta)) + other,
            base,
            delta,
        ),
{
    rename_all_index(xs, base, delta);
    rename_all_index(ys, base, delta);
    let a = zip(xs, ys) + rest;
    let b = zip(rename_all(xs, base, delta), rename_all(ys, base, delta)) + other;
    assert forall|i: int| 0 <= i < a.len() implies #[trigger] b[i] == (
        rename(a[i].0, base, delta),
        rename(a[i].1, base, delta),
    ) by {
        if i < xs.len() {
            assert(rename_all(xs, base, delta)[i] == rename(xs[i], base, delta));
            assert(rename_all(ys, base, delta)[i] == rename(ys[i], base, delta));
        } else {
            assert(other[i - xs.len()] == (
                rename(rest[i - xs.len()].0, base, delta),
                rename(rest[i - xs.len()].1, base, delta),
            ));
        }
    }
}

pub proof fn bind_rel(u: UState, v: UState, x: nat, t: Term, base: nat, delta: nat)
    requires
        unify_rel(u, v, base, delta),
        u.pairs.len() > 0,
    ensures
        unify_rel(
            u_bind(u, u.pairs.drop_first(), x, t),
            u_bind(v, v.pairs.drop_first(), lift_var(x, base, delta), rename(t, base, delta)),
            base,
            delta,
        ),
{
    terms_subst_rel(u.sol, v.sol, x, t, base, delta);
    let a = u_bind(u, u.pairs.drop_first(), x, t);
    let b = u_bind(v, v.pairs.drop_first(), lift_var(x, base, delta), rename(t, base, delta));
    assert forall|i: int| 0 <= i < a.pairs.len() implies #[trigger] b.pairs[i] == (
        rename(a.pairs[i].0, base, delta),
        rename(a.pairs[i].1, base, delta),
    ) by {
        assert(v.pairs[i + 1] == (
            rename(u.pairs[i + 1].0, base, delta),
            rename(u.pairs[i + 1].1, base, delta),
        ));
        rename_subst(u.pairs[i + 1].0, x, t, base, delta);
        rename_subst(u.pairs[i + 1].1, x, t, base, delta);
    }
}

pub proof fn unify_n_related(u: UState, v: UState, fuel: nat, base: nat, delta: nat)
    requires
        unify_rel(u, v, base, delta),
    ensures
        uout_rel(unify_n(u, fuel), unify_n(v, fuel), base, delta),
    decreases fuel,
{
    if fuel > 0 && u.pairs.len() > 0 {
        let a = u.pairs[0].0;
        let b = u.pairs[0].1;
        let rest = u.pairs.drop_first();
        let other = v.pairs.drop_first();
        let f = (fuel - 1) as nat;
        assert(v.pairs[0] == (rename(a, base, delta), rename(b, base, delta)));
        pairs_tail_rel(u.pairs, v.pairs, base, delta);
        rename_inj(a, b, base, delta);
        match (a, b) {
            (Term::Var(x), _) => {
                rename_occurs(x, b, base, delta);
                if a == b {
                    unify_n_related(
                        UState { pairs: rest, ..u },
                        UState { pairs: other, ..v },
                        f,
                        base,
                        delta,
                    );
                } else if !occurs(x, b) {
                    bind_rel(u, v, x, b, base, delta);
                    unify_n_related(
                        u_bind(u, rest, x, b),
                        u_bind(v, other, lift_var(x, base, delta), rename(b, base, delta)),
                        f,
                        base,
                        delta,
                    );
                }
            },
            (_, Term::Var(y)) => {
                rename_occurs(y, a, base, delta);
                if !occurs(y, a) {
                    bind_rel(u, v, y, a, base, delta);
                    unify_n_related(
                        u_bind(u, rest, y, a),
                        u_bind(v, other, lift_var(y, base, delta), rename(a, base, delta)),
                        f,
                        base,
                        delta,
                    );
                }
            },
            (Term::Comp(n, xs), Term::Comp(m, ys)) => {
                rename_all_index(xs, base, delta);
                rename_all_index(ys, base, delta);
                if n == m && xs.len() == ys.len() {
                    pairs_zip_rel(xs, ys, rest, other, base, delta);
                    unify_n_related(
                        UState { pairs: zip(xs, ys) + rest, ..u },
                        UState {
                            pairs: zip(rename_all(xs, base, delta), rename_all(ys, base, delta))
                                + other,
                            ..v
                        },
                        f,
                        base,
                        delta,
                    );
                }
            },
            _ => {
                if a == b {
                    unify_n_related(
                        UState { pairs: rest, ..u },
                        UState { pairs: other, ..v },
                        f,
                        base,
                        delta,
                    );
                }
            },
        }
    }
}

pub proof fn unify_related(u: UState, v: UState, base: nat, delta: nat)
    requires
        unify_rel(u, v, base, delta),
    ensures
        uout_rel(unify(u), unify(v), base, delta),
{
    if exists|f: nat| !(unify_n(u, f) is Out) {
        let f = choose|f: nat| !(unify_n(u, f) is Out);
        unify_n_related(u, v, f, base, delta);
        unify_at(u, f);
        unify_at(v, f);
    } else {
        assert forall|f: nat| unify_n(v, f) is Out by {
            unify_n_related(u, v, f, base, delta);
        }
    }
}

pub proof fn nvars_all_index(ts: Seq<Term>, i: int)
    requires
        0 <= i < ts.len(),
    ensures
        nvars(ts[i]) <= nvars_all(ts),
    decreases ts,
{
    if i > 0 {
        nvars_all_index(ts.drop_first(), i - 1);
    }
}

pub proof fn nvars_all_upper(ts: Seq<Term>, bound: nat)
    requires
        forall|i: int| 0 <= i < ts.len() ==> nvars(#[trigger] ts[i]) <= bound,
    ensures
        nvars_all(ts) <= bound,
    decreases ts,
{
    if ts.len() > 0 {
        assert forall|i: int| 0 <= i < ts.drop_first().len() implies nvars(
            #[trigger] ts.drop_first()[i],
        ) <= bound by {
            assert(nvars(ts[i + 1]) <= bound);
        }
        nvars_all_upper(ts.drop_first(), bound);
    }
}

pub proof fn subst_bound(t: Term, x: nat, v: Term, bound: nat)
    requires
        nvars(t) <= bound,
        nvars(v) <= bound,
    ensures
        nvars(subst(t, x, v)) <= bound,
    decreases t,
{
    match t {
        Term::Comp(_, ts) => subst_all_bound(ts, x, v, bound),
        _ => {},
    }
}

pub proof fn subst_all_bound(ts: Seq<Term>, x: nat, v: Term, bound: nat)
    requires
        nvars_all(ts) <= bound,
        nvars(v) <= bound,
    ensures
        nvars_all(subst_all(ts, x, v)) <= bound,
    decreases ts,
{
    if ts.len() > 0 {
        subst_bound(ts[0], x, v, bound);
        subst_all_bound(ts.drop_first(), x, v, bound);
        assert(subst_all(ts, x, v).drop_first() =~= subst_all(ts.drop_first(), x, v));
    }
}

pub proof fn shift_bound(t: Term, off: nat)
    ensures
        nvars(shift(t, off)) <= nvars(t) + off,
    decreases t,
{
    match t {
        Term::Comp(_, ts) => shift_all_bound(ts, off),
        _ => {},
    }
}

pub proof fn shift_all_bound(ts: Seq<Term>, off: nat)
    ensures
        nvars_all(shift_all(ts, off)) <= nvars_all(ts) + off,
    decreases ts,
{
    if ts.len() > 0 {
        shift_bound(ts[0], off);
        shift_all_bound(ts.drop_first(), off);
        assert(shift_all(ts, off).drop_first() =~= shift_all(ts.drop_first(), off));
    }
}

pub proof fn conj_bound(ts: Seq<Term>)
    requires
        ts.len() > 0,
    ensures
        nvars(conj_term(ts)) <= nvars_all(ts),
    decreases ts,
{
    reveal_with_fuel(nvars, 4);
    if ts.len() > 1 {
        conj_bound(ts.drop_first());
        reveal_with_fuel(nvars_all, 3);
    }
}

pub proof fn shift_all_len(ts: Seq<Term>, off: nat)
    ensures
        shift_all(ts, off).len() == ts.len(),
    decreases ts,
{
    if ts.len() > 0 {
        shift_all_len(ts.drop_first(), off);
    }
}

pub proof fn item_bound(items: Seq<BodyItem>, i: int, off: nat)
    requires
        0 <= i < items.len(),
        wf_body_item(items[i]),
    ensures
        nvars(item_term(items[i], off)) <= items_nvars(items) + off,
    decreases items,
{
    if i > 0 {
        item_bound(items.drop_first(), i - 1, off);
    } else {
        match items[0] {
            BodyItem::Pos(t) => shift_bound(t, off),
            BodyItem::Naf(ts) => {
                shift_all_len(ts, off);
                shift_all_bound(ts, off);
                conj_bound(shift_all(ts, off));
                reveal_with_fuel(nvars_all, 2);
                reveal_with_fuel(nvars, 4);
            },
        }
    }
}

pub proof fn rename_fixed(t: Term, base: nat, delta: nat)
    requires
        nvars(t) <= base,
    ensures
        rename(t, base, delta) == t,
    decreases t,
{
    match t {
        Term::Comp(_, ts) => rename_all_fixed(ts, base, delta),
        _ => {},
    }
}

pub proof fn rename_all_fixed(ts: Seq<Term>, base: nat, delta: nat)
    requires
        nvars_all(ts) <= base,
    ensures
        rename_all(ts, base, delta) == ts,
    decreases ts,
{
    if ts.len() > 0 {
        rename_fixed(ts[0], base, delta);
        rename_all_fixed(ts.drop_first(), base, delta);
        assert(seq![ts[0]] + ts.drop_first() =~= ts);
    }
}

pub proof fn rename_shift(t: Term, off: nat, base: nat, delta: nat)
    requires
        off >= base,
    ensures
        rename(shift(t, off), base, delta) == shift(t, off + delta),
    decreases t,
{
    match t {
        Term::Comp(_, ts) => rename_shift_all(ts, off, base, delta),
        _ => {},
    }
}

pub proof fn rename_shift_all(ts: Seq<Term>, off: nat, base: nat, delta: nat)
    requires
        off >= base,
    ensures
        rename_all(shift_all(ts, off), base, delta) == shift_all(ts, off + delta),
    decreases ts,
{
    if ts.len() > 0 {
        rename_shift(ts[0], off, base, delta);
        rename_shift_all(ts.drop_first(), off, base, delta);
        assert(shift_all(ts, off).drop_first() =~= shift_all(ts.drop_first(), off));
    }
}

pub proof fn rename_conj(ts: Seq<Term>, base: nat, delta: nat)
    requires
        ts.len() > 0,
    ensures
        rename(conj_term(ts), base, delta) == conj_term(rename_all(ts, base, delta)),
    decreases ts,
{
    rename_all_index(ts, base, delta);
    assert(rename_all(ts, base, delta)[0] == rename(ts[0], base, delta));
    if ts.len() > 1 {
        rename_conj(ts.drop_first(), base, delta);
        assert(rename_all(ts, base, delta).drop_first() =~= rename_all(
            ts.drop_first(),
            base,
            delta,
        ));
        reveal_with_fuel(rename_all, 3);
        reveal_with_fuel(rename, 4);
        let tail = conj_term(ts.drop_first());
        assert(rename_all(seq![ts[0], tail], base, delta) =~= seq![
            rename(ts[0], base, delta),
            rename(tail, base, delta),
        ]);
        assert(rename(conj_term(ts), base, delta) == Term::Comp(
            comma_name(),
            seq![rename(ts[0], base, delta), rename(tail, base, delta)],
        ));
        assert(conj_term(rename_all(ts, base, delta)) == Term::Comp(
            comma_name(),
            seq![rename(ts[0], base, delta), rename(tail, base, delta)],
        ));
    } else {
        assert(conj_term(ts) == ts[0]);
        assert(conj_term(rename_all(ts, base, delta)) == rename(ts[0], base, delta));
    }
}

pub proof fn rename_item(it: BodyItem, off: nat, base: nat, delta: nat)
    requires
        off >= base,
        wf_body_item(it),
    ensures
        rename(item_term(it, off), base, delta) == item_term(it, off + delta),
{
    match it {
        BodyItem::Pos(t) => rename_shift(t, off, base, delta),
        BodyItem::Naf(ts) => {
            shift_all_len(ts, off);
            rename_shift_all(ts, off, base, delta);
            rename_conj(shift_all(ts, off), base, delta);
            reveal_with_fuel(rename_all, 2);
            reveal_with_fuel(rename, 4);
        },
    }
}

pub open spec fn u_bounded(u: UState, bound: nat) -> bool {
    u.stack.len() == 0 && nvars_all(u.sol) <= bound && forall|i: int|
        0 <= i < u.pairs.len() ==> #[trigger] nvars(u.pairs[i].0) <= bound && nvars(u.pairs[i].1)
            <= bound
}

pub proof fn u_bind_bounded(u: UState, x: nat, v: Term, bound: nat)
    requires
        u_bounded(u, bound),
        u.pairs.len() > 0,
        nvars(v) <= bound,
    ensures
        u_bounded(u_bind(u, u.pairs.drop_first(), x, v), bound),
        u_bind(u, u.pairs.drop_first(), x, v).sol.len() == u.sol.len(),
{
    subst_all_bound(u.sol, x, v, bound);
    subst_all_index(u.sol, x, v);
    let b = u_bind(u, u.pairs.drop_first(), x, v);
    assert forall|i: int| 0 <= i < b.pairs.len() implies #[trigger] nvars(b.pairs[i].0) <= bound
        && nvars(b.pairs[i].1) <= bound by {
        subst_bound(u.pairs[i + 1].0, x, v, bound);
        subst_bound(u.pairs[i + 1].1, x, v, bound);
    }
}

pub proof fn unify_n_bound(u: UState, fuel: nat, bound: nat)
    requires
        u_bounded(u, bound),
    ensures
        match unify_n(u, fuel) {
            UOut::Ok(gs, ts) => gs.len() == 0 && ts.len() == u.sol.len() && nvars_all(ts) <= bound,
            _ => true,
        },
    decreases fuel,
{
    if fuel > 0 && u.pairs.len() > 0 {
        let a = u.pairs[0].0;
        let b = u.pairs[0].1;
        let rest = u.pairs.drop_first();
        let f = (fuel - 1) as nat;
        assert(nvars(a) <= bound && nvars(b) <= bound);
        assert forall|i: int| 0 <= i < rest.len() implies #[trigger] nvars(rest[i].0) <= bound
            && nvars(rest[i].1) <= bound by {
            assert(nvars(u.pairs[i + 1].0) <= bound);
            assert(nvars(u.pairs[i + 1].1) <= bound);
        }
        match (a, b) {
            (Term::Var(x), _) => {
                if a == b {
                    unify_n_bound(UState { pairs: rest, ..u }, f, bound);
                } else if !occurs(x, b) {
                    u_bind_bounded(u, x, b, bound);
                    unify_n_bound(u_bind(u, rest, x, b), f, bound);
                }
            },
            (_, Term::Var(y)) => {
                if !occurs(y, a) {
                    u_bind_bounded(u, y, a, bound);
                    unify_n_bound(u_bind(u, rest, y, a), f, bound);
                }
            },
            (Term::Comp(n, xs), Term::Comp(m, ys)) => {
                if n == m && xs.len() == ys.len() {
                    let pairs = zip(xs, ys) + rest;
                    assert forall|i: int| 0 <= i < pairs.len() implies #[trigger] nvars(pairs[i].0)
                        <= bound && nvars(pairs[i].1) <= bound by {
                        if i < xs.len() {
                            nvars_all_index(xs, i);
                            nvars_all_index(ys, i);
                        } else {
                            assert(nvars(rest[i - xs.len()].0) <= bound);
                            assert(nvars(rest[i - xs.len()].1) <= bound);
                        }
                    }
                    unify_n_bound(UState { pairs, ..u }, f, bound);
                }
            },
            _ => {
                if a == b {
                    unify_n_bound(UState { pairs: rest, ..u }, f, bound);
                }
            },
        }
    }
}

pub proof fn unify_bound(u: UState, bound: nat)
    requires
        u_bounded(u, bound),
    ensures
        match unify(u) {
            UOut::Ok(gs, ts) => gs.len() == 0 && ts.len() == u.sol.len() && nvars_all(ts) <= bound,
            _ => true,
        },
{
    if exists|f: nat| !(unify_n(u, f) is Out) {
        let f = choose|f: nat| !(unify_n(u, f) is Out);
        unify_n_bound(u, f, bound);
    }
}

pub open spec fn stack_bound(stack: Seq<TGoal>, bound: nat) -> bool {
    forall|i: int| 0 <= i < stack.len() ==> nvars(tgoal_term(#[trigger] stack[i])) <= bound
}

pub open spec fn stack_cuts(stack: Seq<TGoal>, alts: Seq<TAlt>, limit: nat) -> bool {
    limit <= alts.len() && forall|i: int|
        0 <= i < stack.len() ==> (#[trigger] stack[i] matches TGoal::NafCut(l) ==> l < limit
            && alts[l as int] is Naf)
}

pub open spec fn call_stack(stack: Seq<TGoal>) -> bool {
    stack.len() > 0 && match stack[0] {
        TGoal::Lit(Term::Comp(n, ts), d, _) => d > 0 && !(n == comma_name() && ts.len() == 2) && !(n
            == naf_name() && ts.len() == 1),
        _ => false,
    }
}

pub open spec fn alt_wf(a: TAlt, alts: Seq<TAlt>, i: nat) -> bool {
    match a {
        TAlt::Cl { stack, fresh, .. } => stack_bound(stack, fresh) && stack_cuts(stack, alts, i)
            && call_stack(stack),
        TAlt::Naf { stack, fresh, inner, .. } => stack_bound(stack, fresh) && stack_cuts(
            stack,
            alts,
            i,
        ) && nvars(inner) <= fresh,
    }
}

pub open spec fn alts_wf(alts: Seq<TAlt>) -> bool {
    forall|i: int| 0 <= i < alts.len() ==> alt_wf(#[trigger] alts[i], alts, i as nat)
}

pub open spec fn cfg_wf(c: TCfg) -> bool {
    stack_bound(c.stack, c.fresh) && stack_cuts(c.stack, c.alts, c.alts.len()) && (c.ci == 0
        || call_stack(c.stack)) && alts_wf(c.alts)
}

pub proof fn stack_cuts_prefix(stack: Seq<TGoal>, xs: Seq<TAlt>, ys: Seq<TAlt>, limit: nat)
    requires
        stack_cuts(stack, xs, limit),
        limit <= ys.len(),
        xs.take(limit as int) == ys.take(limit as int),
    ensures
        stack_cuts(stack, ys, limit),
{
    assert forall|i: int| 0 <= i < stack.len() implies (#[trigger] stack[i] matches TGoal::NafCut(l)
        ==> l < limit && ys[l as int] is Naf) by {
        if let TGoal::NafCut(l) = stack[i] {
            assert(l < limit);
            assert(limit <= xs.len() && limit <= ys.len());
            assert(xs.take(limit as int)[l as int] == xs[l as int]);
            assert(ys.take(limit as int)[l as int] == ys[l as int]);
            assert(xs.take(limit as int)[l as int] == ys.take(limit as int)[l as int]);
            assert(xs[l as int] == ys[l as int]);
        }
    }
}

pub proof fn alt_wf_prefix(a: TAlt, xs: Seq<TAlt>, ys: Seq<TAlt>, i: nat)
    requires
        alt_wf(a, xs, i),
        i <= ys.len(),
        xs.take(i as int) == ys.take(i as int),
    ensures
        alt_wf(a, ys, i),
{
    match a {
        TAlt::Cl { stack, .. } | TAlt::Naf { stack, .. } => stack_cuts_prefix(stack, xs, ys, i),
    }
}

pub proof fn alts_wf_take(alts: Seq<TAlt>, n: nat)
    requires
        alts_wf(alts),
        n <= alts.len(),
    ensures
        alts_wf(alts.take(n as int)),
{
    assert forall|i: int| 0 <= i < alts.take(n as int).len() implies alt_wf(
        #[trigger] alts.take(n as int)[i],
        alts.take(n as int),
        i as nat,
    ) by {
        assert(alts.take(n as int).take(i) =~= alts.take(i));
        alt_wf_prefix(alts[i], alts, alts.take(n as int), i as nat);
    }
}

pub proof fn alts_wf_push(alts: Seq<TAlt>, a: TAlt)
    requires
        alts_wf(alts),
        alt_wf(a, alts.push(a), alts.len()),
    ensures
        alts_wf(alts.push(a)),
{
    assert forall|i: int| 0 <= i < alts.push(a).len() implies alt_wf(
        #[trigger] alts.push(a)[i],
        alts.push(a),
        i as nat,
    ) by {
        if i < alts.len() {
            assert(alts.push(a).take(i) =~= alts.take(i));
            alt_wf_prefix(alts[i], alts, alts.push(a), i as nat);
        }
    }
}

pub proof fn roots_wf(goals: Seq<Term>)
    ensures
        cfg_wf(roots_cfg(goals)),
{
    let c = roots_cfg(goals);
    assert forall|i: int| 0 <= i < c.stack.len() implies nvars(tgoal_term(#[trigger] c.stack[i]))
        <= c.fresh by {
        nvars_all_index(goals, i);
    }
}

pub open spec fn goal_meta(a: TGoal, b: TGoal) -> bool {
    match (a, b) {
        (TGoal::Lit(_, da, pa), TGoal::Lit(_, db, pb)) => da == db && pa == pb,
        (TGoal::NafCut(a), TGoal::NafCut(b)) => a == b,
        _ => false,
    }
}

pub proof fn tunify_metadata(pairs: Seq<(Term, Term)>, stack: Seq<TGoal>)
    ensures
        match tunify(pairs, stack) {
            TUni::Ok(out) => out.len() == stack.len() && forall|i: int|
                0 <= i < stack.len() ==> goal_meta(#[trigger] stack[i], out[i]),
            _ => true,
        },
{
    if let UOut::Ok(_, ts) = unify(
        UState { pairs, stack: Seq::empty(), sol: stack.map_values(|g: TGoal| tgoal_term(g)) },
    ) {
        assert forall|i: int| 0 <= i < stack.len() implies goal_meta(
            #[trigger] stack[i],
            tgoal_with(stack[i], ts[i]),
        ) by {
            match stack[i] {
                TGoal::Lit(_, _, _) => {},
                TGoal::NafCut(_) => {},
            }
        }
    }
}

pub proof fn tunify_bounded(pairs: Seq<(Term, Term)>, stack: Seq<TGoal>, bound: nat)
    requires
        stack_bound(stack, bound),
        forall|i: int|
            0 <= i < pairs.len() ==> #[trigger] nvars(pairs[i].0) <= bound && nvars(pairs[i].1)
                <= bound,
    ensures
        match tunify(pairs, stack) {
            TUni::Ok(out) => stack_bound(out, bound),
            _ => true,
        },
{
    let ts = stack.map_values(|g: TGoal| tgoal_term(g));
    assert forall|i: int| 0 <= i < ts.len() implies nvars(#[trigger] ts[i]) <= bound by {
        assert(nvars(tgoal_term(stack[i])) <= bound);
    }
    nvars_all_upper(ts, bound);
    let u = UState { pairs, stack: Seq::empty(), sol: ts };
    unify_bound(u, bound);
    if let UOut::Ok(_, out) = unify(u) {
        let gs = Seq::new(stack.len(), |i: int| tgoal_with(stack[i], out[i]));
        assert forall|i: int| 0 <= i < gs.len() implies nvars(tgoal_term(#[trigger] gs[i]))
            <= bound by {
            nvars_all_index(out, i);
            match stack[i] {
                TGoal::Lit(_, _, _) => {},
                TGoal::NafCut(_) => {},
            }
        }
    }
}

pub proof fn tunify_cuts(pairs: Seq<(Term, Term)>, stack: Seq<TGoal>, alts: Seq<TAlt>, limit: nat)
    requires
        stack_cuts(stack, alts, limit),
    ensures
        match tunify(pairs, stack) {
            TUni::Ok(out) => stack_cuts(out, alts, limit),
            _ => true,
        },
{
    tunify_metadata(pairs, stack);
    if let TUni::Ok(out) = tunify(pairs, stack) {
        assert forall|i: int| 0 <= i < out.len() implies (#[trigger] out[i] matches TGoal::NafCut(l)
            ==> l < limit && alts[l as int] is Naf) by {
            assert(goal_meta(stack[i], out[i]));
            assert(stack[i] matches TGoal::NafCut(l) ==> l < limit && alts[l as int] is Naf);
        }
    }
}

pub proof fn next_match_shape(db: Seq<DocClause>, name: Seq<u8>, arity: nat, from: nat)
    ensures
        next_match(db, name, arity, from) matches Option::Some(m) ==> lit_fa(db[m as int].head)
            == Option::Some((name, arity)),
    decreases db.len() - from,
{
    if from < db.len() && lit_fa(db[from as int].head) != Option::Some((name, arity)) {
        next_match_shape(db, name, arity, from + 1);
    }
}

pub proof fn tbody_bounded(items: Seq<BodyItem>, off: nat, d: nat, path: Seq<nat>)
    requires
        forall|i: int| 0 <= i < items.len() ==> wf_body_item(#[trigger] items[i]),
    ensures
        stack_bound(tbody_goals(items, off, d, path), items_nvars(items) + off),
{
    let gs = tbody_goals(items, off, d, path);
    assert forall|i: int| 0 <= i < gs.len() implies nvars(tgoal_term(#[trigger] gs[i]))
        <= items_nvars(items) + off by {
        item_bound(items, i, off);
    }
}

pub proof fn stack_bound_join(a: Seq<TGoal>, b: Seq<TGoal>, bound: nat)
    requires
        stack_bound(a, bound),
        stack_bound(b, bound),
    ensures
        stack_bound(a + b, bound),
{
    assert forall|i: int| 0 <= i < (a + b).len() implies nvars(tgoal_term(#[trigger] (a + b)[i]))
        <= bound by {
        if i < a.len() {
            assert(nvars(tgoal_term(a[i])) <= bound);
        } else {
            assert(nvars(tgoal_term(b[i - a.len()])) <= bound);
        }
    }
}

pub proof fn stack_cuts_join(a: Seq<TGoal>, b: Seq<TGoal>, alts: Seq<TAlt>, limit: nat)
    requires
        stack_cuts(a, alts, limit),
        stack_cuts(b, alts, limit),
    ensures
        stack_cuts(a + b, alts, limit),
{
    assert forall|i: int| 0 <= i < (a + b).len() implies (#[trigger] (a
        + b)[i] matches TGoal::NafCut(l) ==> l < limit && alts[l as int] is Naf) by {
        if i < a.len() {
            assert(a[i] matches TGoal::NafCut(l) ==> l < limit && alts[l as int] is Naf);
        } else {
            assert(b[i - a.len()] matches TGoal::NafCut(l) ==> l < limit && alts[l as int] is Naf);
        }
    }
}

pub proof fn stack_tail_wf(stack: Seq<TGoal>, bound: nat, alts: Seq<TAlt>, limit: nat)
    requires
        stack.len() > 0,
        stack_bound(stack, bound),
        stack_cuts(stack, alts, limit),
    ensures
        stack_bound(stack.drop_first(), bound),
        stack_cuts(stack.drop_first(), alts, limit),
{
    assert forall|i: int| 0 <= i < stack.drop_first().len() implies nvars(
        tgoal_term(#[trigger] stack.drop_first()[i]),
    ) <= bound by {
        assert(nvars(tgoal_term(stack[i + 1])) <= bound);
    }
    assert forall|i: int| 0 <= i < stack.drop_first().len() implies (
    #[trigger] stack.drop_first()[i] matches TGoal::NafCut(l) ==> l < limit
        && alts[l as int] is Naf) by {
        assert(stack[i + 1] matches TGoal::NafCut(l) ==> l < limit && alts[l as int] is Naf);
    }
}

pub proof fn tfail_wf(c: TCfg)
    requires
        alts_wf(c.alts),
    ensures
        tfail(c) matches TStep::Next(c2) ==> cfg_wf(c2),
{
    if c.alts.len() > 0 {
        let i = (c.alts.len() - 1) as nat;
        let rest = c.alts.drop_last();
        assert(rest =~= c.alts.take(i as int));
        alts_wf_take(c.alts, i);
        assert(alt_wf(c.alts[i as int], c.alts, i));
        assert(c.alts.take(i as int) =~= rest.take(i as int));
        alt_wf_prefix(c.alts[i as int], c.alts, rest, i);
        match c.alts.last() {
            TAlt::Cl { .. } | TAlt::Naf { .. } => {},
        }
    }
}

pub proof fn shifted_args_len(t: Term, off: nat)
    ensures
        args_of(shift(t, off)).len() == args_of(t).len(),
{
    match t {
        Term::Comp(_, ts) => shift_all_len(ts, off),
        _ => {},
    }
}

pub proof fn head_pairs_bounded(args: Seq<Term>, cl: DocClause, name: Seq<u8>, off: nat)
    requires
        nvars_all(args) <= off,
        lit_fa(cl.head) == Option::Some((name, args.len())),
    ensures
        forall|i: int|
            0 <= i < zip(args, args_of(shift(cl.head, off))).len() ==> #[trigger] nvars(
                zip(args, args_of(shift(cl.head, off)))[i].0,
            ) <= off + clause_nvars(cl) && nvars(zip(args, args_of(shift(cl.head, off)))[i].1)
                <= off + clause_nvars(cl),
{
    shifted_args_len(cl.head, off);
    shift_bound(cl.head, off);
    assert(nvars_all(args_of(shift(cl.head, off))) <= nvars(shift(cl.head, off)));
    let pairs = zip(args, args_of(shift(cl.head, off)));
    assert forall|i: int| 0 <= i < pairs.len() implies #[trigger] nvars(pairs[i].0) <= off
        + clause_nvars(cl) && nvars(pairs[i].1) <= off + clause_nvars(cl) by {
        nvars_all_index(args, i);
        nvars_all_index(args_of(shift(cl.head, off)), i);
    }
}

pub proof fn tcall_wf(
    db: Seq<DocClause>,
    c: TCfg,
    name: Seq<u8>,
    args: Seq<Term>,
    d: nat,
    rest: Seq<TGoal>,
    ci: nat,
    path: Seq<nat>,
)
    requires
        bodies_wf(db),
        cfg_wf(c),
        call_stack(c.stack),
        c.stack == seq![TGoal::Lit(Term::Comp(name, args), d, path)] + rest,
    ensures
        tcall(db, c, name, args, d, rest, ci, path) matches TStep::Next(c2) ==> cfg_wf(c2),
    decreases db.len() - ci,
{
    match next_match(db, name, args.len(), ci) {
        Option::None => tfail_wf(c),
        Option::Some(m) => {
            next_match_bound(db, name, args.len(), ci);
            next_match_shape(db, name, args.len(), ci);
            let cl = db[m as int];
            let fresh = c.fresh + clause_nvars(cl);
            let body = tbody_goals(cl.body, c.fresh, (d - 1) as nat, path);
            let stack2 = body + rest;
            let pairs = zip(args, args_of(shift(cl.head, c.fresh)));
            let alt = TAlt::Cl { stack: c.stack, fresh: c.fresh, ci: m + 1, log: c.log };
            let alts = c.alts.push(alt);
            assert(nvars(tgoal_term(c.stack[0])) <= c.fresh);
            assert forall|i: int| 0 <= i < cl.body.len() implies wf_body_item(
                #[trigger] cl.body[i],
            ) by {
                assert(wf_body_item(db[m as int].body[i]));
            }
            tbody_bounded(cl.body, c.fresh, (d - 1) as nat, path);
            assert(rest =~= c.stack.drop_first());
            stack_tail_wf(c.stack, c.fresh, c.alts, c.alts.len());
            assert(alts.take(c.alts.len() as int) =~= c.alts);
            assert(c.alts.take(c.alts.len() as int) =~= c.alts);
            stack_cuts_prefix(c.stack, c.alts, alts, c.alts.len());
            stack_cuts_prefix(rest, c.alts, alts, c.alts.len());
            alts_wf_push(c.alts, alt);
            assert(stack_bound(body, fresh));
            assert(stack_bound(rest, fresh));
            stack_bound_join(body, rest, fresh);
            assert(stack_cuts(body, alts, alts.len()));
            assert(stack_cuts(rest, alts, alts.len()));
            stack_cuts_join(body, rest, alts, alts.len());
            head_pairs_bounded(args, cl, name, c.fresh);
            tunify_bounded(pairs, stack2, fresh);
            tunify_cuts(pairs, stack2, alts, alts.len());
            match tunify(pairs, stack2) {
                TUni::Ok(_) => {},
                TUni::Fail => tcall_wf(db, c, name, args, d, rest, m + 1, path),
                TUni::Out => {},
            }
        },
    }
}

pub proof fn tstep_wf(db: Seq<DocClause>, c: TCfg)
    requires
        bodies_wf(db),
        cfg_wf(c),
    ensures
        tstep(db, c) matches TStep::Next(c2) ==> cfg_wf(c2),
{
    if c.stack.len() > 0 {
        let rest = c.stack.drop_first();
        stack_tail_wf(c.stack, c.fresh, c.alts, c.alts.len());
        match c.stack[0] {
            TGoal::NafCut(lvl) => {
                assert(lvl < c.alts.len() && c.alts[lvl as int] is Naf);
                alts_wf_take(c.alts, lvl);
                if let TAlt::Naf { pruned, .. } = c.alts[lvl as int] {
                    tfail_wf(TCfg { alts: c.alts.take(lvl as int), pruned, ..c });
                }
            },
            TGoal::Lit(g, d, path) => {
                assert(nvars(g) <= c.fresh);
                if d == 0 {
                    tfail_wf(TCfg { pruned: true, ..c });
                } else {
                    match g {
                        Term::Comp(name, args) => {
                            if name == comma_name() && args.len() == 2 {
                                assert(c.ci == 0);
                                nvars_all_index(args, 0);
                                nvars_all_index(args, 1);
                                let front = seq![
                                    TGoal::Lit(args[0], d, path),
                                    TGoal::Lit(args[1], d, path),
                                ];
                                assert(stack_bound(front, c.fresh));
                                assert(stack_cuts(front, c.alts, c.alts.len()));
                                stack_bound_join(front, rest, c.fresh);
                                stack_cuts_join(front, rest, c.alts, c.alts.len());
                            } else if name == naf_name() && args.len() == 1 {
                                assert(c.ci == 0);
                                nvars_all_index(args, 0);
                                let alt = TAlt::Naf {
                                    stack: rest,
                                    fresh: c.fresh,
                                    log: c.log,
                                    path,
                                    inner: args[0],
                                    pruned: c.pruned,
                                };
                                let alts = c.alts.push(alt);
                                assert(c.alts.take(c.alts.len() as int) =~= c.alts);
                                assert(alts.take(c.alts.len() as int) =~= c.alts);
                                stack_cuts_prefix(rest, c.alts, alts, c.alts.len());
                                alts_wf_push(c.alts, alt);
                                let front = seq![
                                    TGoal::Lit(args[0], trace_depth(), path),
                                    TGoal::NafCut(c.alts.len()),
                                ];
                                assert(stack_bound(front, c.fresh));
                                assert(stack_cuts(front, alts, alts.len()));
                                assert(stack_cuts(rest, alts, alts.len()));
                                stack_bound_join(front, rest, c.fresh);
                                stack_cuts_join(front, rest, alts, alts.len());
                            } else {
                                assert(seq![c.stack[0]] + rest =~= c.stack);
                                tcall_wf(db, c, name, args, d, rest, c.ci, path);
                            }
                        },
                        _ => tfail_wf(c),
                    }
                }
            },
        }
    }
}

pub open spec fn proj_goal(a: TGoal, b: TGoal, base: nat, delta: nat, off: nat) -> bool {
    match (a, b) {
        (TGoal::Lit(t, d, _), TGoal::Lit(u, e, _)) => d == e && u == rename(t, base, delta),
        (TGoal::NafCut(l), TGoal::NafCut(k)) => k == l + off,
        _ => false,
    }
}

pub open spec fn proj_stack(a: Seq<TGoal>, b: Seq<TGoal>, base: nat, delta: nat, off: nat) -> bool {
    off > 0 && a.len() < b.len() && b[a.len() as int] == TGoal::NafCut((off - 1) as nat) && forall|
        i: int,
    |
        0 <= i < a.len() ==> proj_goal(#[trigger] a[i], b[i], base, delta, off)
}

pub open spec fn proj_alt(a: TAlt, b: TAlt, base: nat, delta: nat, off: nat) -> bool {
    match (a, b) {
        (
            TAlt::Cl { stack: s, fresh: f, ci: i, .. },
            TAlt::Cl { stack: t, fresh: g, ci: j, .. },
        ) => f >= base && g == f + delta && i == j && proj_stack(s, t, base, delta, off),
        (
            TAlt::Naf { stack: s, fresh: f, inner: u, pruned: p, .. },
            TAlt::Naf { stack: t, fresh: g, inner: v, pruned: q, .. },
        ) => f >= base && g == f + delta && v == rename(u, base, delta) && p == q && proj_stack(
            s,
            t,
            base,
            delta,
            off,
        ),
        _ => false,
    }
}

pub open spec fn proj_alts(a: Seq<TAlt>, b: Seq<TAlt>, base: nat, delta: nat, off: nat) -> bool {
    off > 0 && a.len() + off == b.len() && b[(off - 1) as int] is Naf && forall|i: int|
        0 <= i < a.len() ==> proj_alt(#[trigger] a[i], b[i + off], base, delta, off)
}

pub open spec fn proj_cfg(a: TCfg, b: TCfg, base: nat, delta: nat, off: nat) -> bool {
    proj_stack(a.stack, b.stack, base, delta, off) && proj_alts(a.alts, b.alts, base, delta, off)
        && a.fresh >= base && b.fresh == a.fresh + delta && a.ci == b.ci && a.pruned == b.pruned
}

pub open spec fn proj_step(a: TStep, b: TStep, base: nat, delta: nat, off: nat) -> bool {
    match b {
        TStep::Next(c) => if c.alts.len() >= off {
            a matches TStep::Next(d) && proj_cfg(d, c, base, delta, off)
        } else if c.alts.len() + 1 == off {
            a == TStep::Done(true)
        } else {
            true
        },
        _ => true,
    }
}

pub proof fn proj_stack_tail(a: Seq<TGoal>, b: Seq<TGoal>, base: nat, delta: nat, off: nat)
    requires
        proj_stack(a, b, base, delta, off),
        a.len() > 0,
    ensures
        proj_stack(a.drop_first(), b.drop_first(), base, delta, off),
{
    assert forall|i: int| 0 <= i < a.drop_first().len() implies proj_goal(
        #[trigger] a.drop_first()[i],
        b.drop_first()[i],
        base,
        delta,
        off,
    ) by {
        assert(proj_goal(a[i + 1], b[i + 1], base, delta, off));
    }
}

pub proof fn proj_stack_join(
    ah: Seq<TGoal>,
    bh: Seq<TGoal>,
    a: Seq<TGoal>,
    b: Seq<TGoal>,
    base: nat,
    delta: nat,
    off: nat,
)
    requires
        ah.len() == bh.len(),
        proj_stack(a, b, base, delta, off),
        forall|i: int| 0 <= i < ah.len() ==> proj_goal(#[trigger] ah[i], bh[i], base, delta, off),
    ensures
        proj_stack(ah + a, bh + b, base, delta, off),
{
    assert forall|i: int| 0 <= i < (ah + a).len() implies proj_goal(
        #[trigger] (ah + a)[i],
        (bh + b)[i],
        base,
        delta,
        off,
    ) by {
        if i < ah.len() {
            assert(proj_goal(ah[i], bh[i], base, delta, off));
        } else {
            assert(proj_goal(a[i - ah.len()], b[i - bh.len()], base, delta, off));
        }
    }
}

pub proof fn proj_alts_take(a: Seq<TAlt>, b: Seq<TAlt>, n: nat, base: nat, delta: nat, off: nat)
    requires
        proj_alts(a, b, base, delta, off),
        n <= a.len(),
    ensures
        proj_alts(a.take(n as int), b.take((n + off) as int), base, delta, off),
{
    assert forall|i: int| 0 <= i < a.take(n as int).len() implies proj_alt(
        #[trigger] a.take(n as int)[i],
        b.take((n + off) as int)[i + off],
        base,
        delta,
        off,
    ) by {
        assert(proj_alt(a[i], b[i + off], base, delta, off));
    }
}

pub proof fn proj_alts_push(
    a: Seq<TAlt>,
    b: Seq<TAlt>,
    x: TAlt,
    y: TAlt,
    base: nat,
    delta: nat,
    off: nat,
)
    requires
        proj_alts(a, b, base, delta, off),
        proj_alt(x, y, base, delta, off),
    ensures
        proj_alts(a.push(x), b.push(y), base, delta, off),
{
    assert forall|i: int| 0 <= i < a.push(x).len() implies proj_alt(
        #[trigger] a.push(x)[i],
        b.push(y)[i + off],
        base,
        delta,
        off,
    ) by {
        if i < a.len() {
            assert(proj_alt(a[i], b[i + off], base, delta, off));
        }
    }
}

pub proof fn tfail_size(c: TCfg)
    ensures
        tfail(c) matches TStep::Next(d) ==> d.alts.len() + 1 == c.alts.len(),
{
}

pub proof fn project_fail(a: TCfg, b: TCfg, base: nat, delta: nat, off: nat)
    requires
        proj_cfg(a, b, base, delta, off),
    ensures
        proj_step(tfail(a), tfail(b), base, delta, off),
{
    if a.alts.len() > 0 {
        let n = (a.alts.len() - 1) as nat;
        assert(proj_alt(a.alts[n as int], b.alts[(n + off) as int], base, delta, off));
        proj_alts_take(a.alts, b.alts, n, base, delta, off);
        assert(a.alts.drop_last() =~= a.alts.take(n as int));
        assert(b.alts.drop_last() =~= b.alts.take((n + off) as int));
        match (a.alts.last(), b.alts.last()) {
            (TAlt::Cl { .. }, TAlt::Cl { .. }) => {},
            (TAlt::Naf { .. }, TAlt::Naf { .. }) => {},
            _ => {},
        }
    }
}

pub proof fn unify_output_len(u: UState)
    requires
        u.stack.len() == 0,
    ensures
        unify(u) matches UOut::Ok(_, out) ==> out.len() == u.sol.len(),
{
    let xs = u.pairs.map_values(|p: (Term, Term)| p.0);
    let ys = u.pairs.map_values(|p: (Term, Term)| p.1);
    let bound = nvars_all(u.sol) + nvars_all(xs) + nvars_all(ys);
    assert forall|i: int| 0 <= i < u.pairs.len() implies #[trigger] nvars(u.pairs[i].0) <= bound
        && nvars(u.pairs[i].1) <= bound by {
        nvars_all_index(xs, i);
        nvars_all_index(ys, i);
    }
    unify_bound(u, bound);
}

pub open spec fn proj_uni(a: TUni, b: TUni, base: nat, delta: nat, off: nat) -> bool {
    match (a, b) {
        (TUni::Ok(s), TUni::Ok(t)) => proj_stack(s, t, base, delta, off),
        (TUni::Fail, TUni::Fail) | (TUni::Out, TUni::Out) => true,
        _ => false,
    }
}

pub proof fn project_unify(
    ap: Seq<(Term, Term)>,
    bp: Seq<(Term, Term)>,
    a: Seq<TGoal>,
    b: Seq<TGoal>,
    base: nat,
    delta: nat,
    off: nat,
)
    requires
        proj_stack(a, b, base, delta, off),
        pairs_rel(ap, bp, base, delta),
    ensures
        proj_uni(tunify(ap, a), tunify(bp, b), base, delta, off),
{
    let u = UState { pairs: ap, stack: Seq::empty(), sol: a.map_values(|g: TGoal| tgoal_term(g)) };
    let v = UState { pairs: bp, stack: Seq::empty(), sol: b.map_values(|g: TGoal| tgoal_term(g)) };
    assert forall|i: int| 0 <= i < u.sol.len() implies #[trigger] v.sol[i] == rename(
        u.sol[i],
        base,
        delta,
    ) by {
        assert(proj_goal(a[i], b[i], base, delta, off));
        match (a[i], b[i]) {
            (TGoal::Lit(_, _, _), TGoal::Lit(_, _, _)) => {},
            (TGoal::NafCut(_), TGoal::NafCut(_)) => {},
            _ => {},
        }
    }
    unify_related(u, v, base, delta);
    unify_output_len(u);
    unify_output_len(v);
    match (unify(u), unify(v)) {
        (UOut::Ok(_, xs), UOut::Ok(_, ys)) => {
            let s = Seq::new(a.len(), |i: int| tgoal_with(a[i], xs[i]));
            let t = Seq::new(b.len(), |i: int| tgoal_with(b[i], ys[i]));
            assert forall|i: int| 0 <= i < s.len() implies proj_goal(
                #[trigger] s[i],
                t[i],
                base,
                delta,
                off,
            ) by {
                assert(proj_goal(a[i], b[i], base, delta, off));
                assert(ys[i] == rename(xs[i], base, delta));
                match (a[i], b[i]) {
                    (TGoal::Lit(_, _, _), TGoal::Lit(_, _, _)) => {},
                    (TGoal::NafCut(_), TGoal::NafCut(_)) => {},
                    _ => {},
                }
            }
        },
        _ => {},
    }
}

pub proof fn rename_args(t: Term, base: nat, delta: nat)
    ensures
        args_of(rename(t, base, delta)) == rename_all(args_of(t), base, delta),
{
    match t {
        Term::Comp(_, _) => {},
        _ => {},
    }
}

pub proof fn tbody_project(
    items: Seq<BodyItem>,
    fresh: nat,
    d: nat,
    pa: Seq<nat>,
    pb: Seq<nat>,
    base: nat,
    delta: nat,
    off: nat,
)
    requires
        fresh >= base,
        forall|i: int| 0 <= i < items.len() ==> wf_body_item(#[trigger] items[i]),
    ensures
        forall|i: int|
            0 <= i < items.len() ==> proj_goal(
                #[trigger] tbody_goals(items, fresh, d, pa)[i],
                tbody_goals(items, fresh + delta, d, pb)[i],
                base,
                delta,
                off,
            ),
{
    assert forall|i: int| 0 <= i < items.len() implies proj_goal(
        #[trigger] tbody_goals(items, fresh, d, pa)[i],
        tbody_goals(items, fresh + delta, d, pb)[i],
        base,
        delta,
        off,
    ) by {
        rename_item(items[i], fresh, base, delta);
    }
}

pub proof fn project_call(
    db: Seq<DocClause>,
    a: TCfg,
    b: TCfg,
    name: Seq<u8>,
    args: Seq<Term>,
    d: nat,
    ra: Seq<TGoal>,
    rb: Seq<TGoal>,
    ci: nat,
    pa: Seq<nat>,
    pb: Seq<nat>,
    base: nat,
    delta: nat,
    off: nat,
)
    requires
        bodies_wf(db),
        proj_cfg(a, b, base, delta, off),
        proj_stack(ra, rb, base, delta, off),
    ensures
        proj_step(
            tcall(db, a, name, args, d, ra, ci, pa),
            tcall(db, b, name, rename_all(args, base, delta), d, rb, ci, pb),
            base,
            delta,
            off,
        ),
    decreases db.len() - ci,
{
    rename_all_index(args, base, delta);
    match next_match(db, name, args.len(), ci) {
        Option::None => project_fail(a, b, base, delta, off),
        Option::Some(m) => {
            next_match_bound(db, name, args.len(), ci);
            next_match_shape(db, name, args.len(), ci);
            let cl = db[m as int];
            let ah = tbody_goals(cl.body, a.fresh, (d - 1) as nat, pa);
            let bh = tbody_goals(cl.body, b.fresh, (d - 1) as nat, pb);
            assert forall|i: int| 0 <= i < cl.body.len() implies wf_body_item(
                #[trigger] cl.body[i],
            ) by {
                assert(wf_body_item(db[m as int].body[i]));
            }
            tbody_project(cl.body, a.fresh, (d - 1) as nat, pa, pb, base, delta, off);
            proj_stack_join(ah, bh, ra, rb, base, delta, off);
            let ap = zip(args, args_of(shift(cl.head, a.fresh)));
            let bp = zip(rename_all(args, base, delta), args_of(shift(cl.head, b.fresh)));
            shifted_args_len(cl.head, a.fresh);
            rename_args(shift(cl.head, a.fresh), base, delta);
            rename_shift(cl.head, a.fresh, base, delta);
            pairs_zip_rel(
                args,
                args_of(shift(cl.head, a.fresh)),
                Seq::empty(),
                Seq::empty(),
                base,
                delta,
            );
            assert(zip(args, args_of(shift(cl.head, a.fresh))) + Seq::empty() =~= ap);
            assert(zip(
                rename_all(args, base, delta),
                rename_all(args_of(shift(cl.head, a.fresh)), base, delta),
            ) + Seq::empty() =~= bp);
            project_unify(ap, bp, ah + ra, bh + rb, base, delta, off);
            let aa = TAlt::Cl { stack: a.stack, fresh: a.fresh, ci: m + 1, log: a.log };
            let ba = TAlt::Cl { stack: b.stack, fresh: b.fresh, ci: m + 1, log: b.log };
            proj_alts_push(a.alts, b.alts, aa, ba, base, delta, off);
            match (tunify(ap, ah + ra), tunify(bp, bh + rb)) {
                (TUni::Ok(_), TUni::Ok(_)) => {},
                (TUni::Fail, TUni::Fail) => project_call(
                    db,
                    a,
                    b,
                    name,
                    args,
                    d,
                    ra,
                    rb,
                    m + 1,
                    pa,
                    pb,
                    base,
                    delta,
                    off,
                ),
                _ => {},
            }
        },
    }
}

pub proof fn project_transition(
    db: Seq<DocClause>,
    a: TCfg,
    b: TCfg,
    base: nat,
    delta: nat,
    off: nat,
)
    requires
        bodies_wf(db),
        cfg_wf(b),
        proj_cfg(a, b, base, delta, off),
    ensures
        proj_step(tstep(db, a), tstep(db, b), base, delta, off),
{
    if a.stack.len() == 0 {
        assert(b.stack[0] == TGoal::NafCut((off - 1) as nat));
        if let TAlt::Naf { pruned, .. } = b.alts[(off - 1) as int] {
            let c = TCfg { alts: b.alts.take((off - 1) as int), pruned, ..b };
            tfail_size(c);
        }
    } else {
        assert(proj_goal(a.stack[0], b.stack[0], base, delta, off));
        let ra = a.stack.drop_first();
        let rb = b.stack.drop_first();
        proj_stack_tail(a.stack, b.stack, base, delta, off);
        match (a.stack[0], b.stack[0]) {
            (TGoal::NafCut(l), TGoal::NafCut(k)) => {
                assert(k < b.alts.len() && b.alts[k as int] is Naf);
                assert(l < a.alts.len());
                assert(proj_alt(a.alts[l as int], b.alts[(l + off) as int], base, delta, off));
                proj_alts_take(a.alts, b.alts, l, base, delta, off);
                if let (TAlt::Naf { pruned: p, .. }, TAlt::Naf { pruned: q, .. }) = (
                    a.alts[l as int],
                    b.alts[k as int],
                ) {
                    project_fail(
                        TCfg { alts: a.alts.take(l as int), pruned: p, ..a },
                        TCfg { alts: b.alts.take(k as int), pruned: q, ..b },
                        base,
                        delta,
                        off,
                    );
                }
            },
            (TGoal::Lit(g, d, pa), TGoal::Lit(h, e, pb)) => {
                if d == 0 {
                    project_fail(
                        TCfg { pruned: true, ..a },
                        TCfg { pruned: true, ..b },
                        base,
                        delta,
                        off,
                    );
                } else {
                    match g {
                        Term::Comp(name, args) => {
                            rename_all_index(args, base, delta);
                            let other = rename_all(args, base, delta);
                            if name == comma_name() && args.len() == 2 {
                                assert(other[0] == rename(args[0], base, delta));
                                assert(other[1] == rename(args[1], base, delta));
                                let ah = seq![
                                    TGoal::Lit(args[0], d, pa),
                                    TGoal::Lit(args[1], d, pa),
                                ];
                                let bh = seq![
                                    TGoal::Lit(other[0], d, pb),
                                    TGoal::Lit(other[1], d, pb),
                                ];
                                assert forall|i: int| 0 <= i < ah.len() implies proj_goal(
                                    #[trigger] ah[i],
                                    bh[i],
                                    base,
                                    delta,
                                    off,
                                ) by {};
                                proj_stack_join(ah, bh, ra, rb, base, delta, off);
                            } else if name == naf_name() && args.len() == 1 {
                                assert(other[0] == rename(args[0], base, delta));
                                let aa = TAlt::Naf {
                                    stack: ra,
                                    fresh: a.fresh,
                                    log: a.log,
                                    path: pa,
                                    inner: args[0],
                                    pruned: a.pruned,
                                };
                                let ba = TAlt::Naf {
                                    stack: rb,
                                    fresh: b.fresh,
                                    log: b.log,
                                    path: pb,
                                    inner: other[0],
                                    pruned: b.pruned,
                                };
                                proj_alts_push(a.alts, b.alts, aa, ba, base, delta, off);
                                let ah = seq![
                                    TGoal::Lit(args[0], trace_depth(), pa),
                                    TGoal::NafCut(a.alts.len()),
                                ];
                                let bh = seq![
                                    TGoal::Lit(other[0], trace_depth(), pb),
                                    TGoal::NafCut(b.alts.len()),
                                ];
                                assert forall|i: int| 0 <= i < ah.len() implies proj_goal(
                                    #[trigger] ah[i],
                                    bh[i],
                                    base,
                                    delta,
                                    off,
                                ) by {};
                                proj_stack_join(ah, bh, ra, rb, base, delta, off);
                            } else {
                                project_call(
                                    db,
                                    a,
                                    b,
                                    name,
                                    args,
                                    d,
                                    ra,
                                    rb,
                                    a.ci,
                                    pa,
                                    pb,
                                    base,
                                    delta,
                                    off,
                                );
                            }
                        },
                        _ => project_fail(a, b, base, delta, off),
                    }
                }
            },
            _ => {},
        }
    }
}

pub ghost struct Shadow {
    pub cfg: TCfg,
    pub fuel: nat,
    pub base: nat,
    pub delta: nat,
}

pub open spec fn shadow_valid(db: Seq<DocClause>, c: TCfg, fuel: nat, i: nat, s: Shadow) -> bool {
    i < c.alts.len() && s.fuel >= fuel && proj_cfg(s.cfg, c, s.base, s.delta, i + 1)
        && match c.alts[i as int] {
        TAlt::Naf { inner, .. } => trun(db, roots_cfg(seq![inner]), trace_inf()).0 == trun(
            db,
            s.cfg,
            s.fuel,
        ).0,
        _ => false,
    }
}

pub open spec fn frame_valid(db: Seq<DocClause>, c: TCfg, fuel: nat, i: nat) -> bool {
    i < c.alts.len() && (c.alts[i as int] is Naf ==> exists|s: Shadow|
        shadow_valid(db, c, fuel, i, s))
}

pub open spec fn frames_safe(db: Seq<DocClause>, c: TCfg, fuel: nat) -> bool {
    forall|i: int| 0 <= i < c.alts.len() ==> #[trigger] frame_valid(db, c, fuel, i as nat)
}

pub proof fn frame_return(db: Seq<DocClause>, c: TCfg, d: TCfg, fuel: nat, i: nat)
    requires
        bodies_wf(db),
        cfg_wf(c),
        fuel > 0,
        frame_valid(db, c, fuel, i),
        tstep(db, c) == TStep::Next(d),
        d.alts.len() == i,
    ensures
        c.alts[i as int] matches TAlt::Naf { inner, .. } ==> naf_fails(db, inner),
{
    if let TAlt::Naf { inner, .. } = c.alts[i as int] {
        let s = choose|s: Shadow| shadow_valid(db, c, fuel, i, s);
        project_transition(db, s.cfg, c, s.base, s.delta, i + 1);
        assert(tstep(db, s.cfg) == TStep::Done(true));
    }
}

pub proof fn frame_advance(db: Seq<DocClause>, c: TCfg, d: TCfg, fuel: nat, i: nat)
    requires
        bodies_wf(db),
        cfg_wf(c),
        fuel > 0,
        frame_valid(db, c, fuel, i),
        tstep(db, c) == TStep::Next(d),
        i < d.alts.len(),
        c.alts[i as int] == d.alts[i as int],
    ensures
        frame_valid(db, d, (fuel - 1) as nat, i),
{
    if c.alts[i as int] is Naf {
        let s = choose|s: Shadow| shadow_valid(db, c, fuel, i, s);
        project_transition(db, s.cfg, c, s.base, s.delta, i + 1);
        if let TStep::Next(s2) = tstep(db, s.cfg) {
            let next = Shadow { cfg: s2, fuel: (s.fuel - 1) as nat, ..s };
            assert(shadow_valid(db, d, (fuel - 1) as nat, i, next));
        }
    }
}

pub open spec fn alts_prefix(a: Seq<TAlt>, b: Seq<TAlt>) -> bool {
    b.len() <= a.len() + 1 && forall|i: int|
        0 <= i < a.len() && i < b.len() ==> #[trigger] b[i] == a[i]
}

pub proof fn tfail_prefix(c: TCfg)
    ensures
        tfail(c) matches TStep::Next(d) ==> alts_prefix(c.alts, d.alts),
{
}

pub proof fn tcall_prefix(
    db: Seq<DocClause>,
    c: TCfg,
    name: Seq<u8>,
    args: Seq<Term>,
    depth: nat,
    rest: Seq<TGoal>,
    ci: nat,
    path: Seq<nat>,
)
    ensures
        tcall(db, c, name, args, depth, rest, ci, path) matches TStep::Next(d) ==> alts_prefix(
            c.alts,
            d.alts,
        ) && (d.alts.len() > c.alts.len() ==> d.alts.last() is Cl),
    decreases db.len() - ci,
{
    match next_match(db, name, args.len(), ci) {
        Option::None => {
            tfail_prefix(c);
            tfail_size(c);
        },
        Option::Some(m) => {
            next_match_bound(db, name, args.len(), ci);
            let cl = db[m as int];
            match tunify(
                zip(args, args_of(shift(cl.head, c.fresh))),
                tbody_goals(cl.body, c.fresh, (depth - 1) as nat, path) + rest,
            ) {
                TUni::Fail => tcall_prefix(db, c, name, args, depth, rest, m + 1, path),
                _ => {},
            }
        },
    }
}

pub proof fn tstep_prefix(db: Seq<DocClause>, c: TCfg)
    requires
        cfg_wf(c),
    ensures
        tstep(db, c) matches TStep::Next(d) ==> alts_prefix(c.alts, d.alts),
{
    if c.stack.len() > 0 {
        let rest = c.stack.drop_first();
        match c.stack[0] {
            TGoal::NafCut(l) => {
                assert(l < c.alts.len());
                if let TAlt::Naf { pruned, .. } = c.alts[l as int] {
                    let c2 = TCfg { alts: c.alts.take(l as int), pruned, ..c };
                    tfail_prefix(c2);
                    tfail_size(c2);
                }
            },
            TGoal::Lit(g, depth, path) => {
                if depth == 0 {
                    tfail_prefix(TCfg { pruned: true, ..c });
                } else {
                    match g {
                        Term::Comp(name, args) => {
                            if !(name == comma_name() && args.len() == 2) && !(name == naf_name()
                                && args.len() == 1) {
                                tcall_prefix(db, c, name, args, depth, rest, c.ci, path);
                            }
                        },
                        _ => tfail_prefix(c),
                    }
                }
            },
        }
    }
}

pub open spec fn enter_site(c: TCfg, inner: Term, path: Seq<nat>) -> TCfg {
    TCfg {
        alts: c.alts.push(
            TAlt::Naf {
                stack: c.stack.drop_first(),
                fresh: c.fresh,
                log: c.log,
                path,
                inner,
                pruned: c.pruned,
            },
        ),
        stack: seq![TGoal::Lit(inner, trace_depth(), path), TGoal::NafCut(c.alts.len())]
            + c.stack.drop_first(),
        pruned: false,
        ..c
    }
}

pub proof fn initialize_shadow(db: Seq<DocClause>, c: TCfg, inner: Term, path: Seq<nat>, fuel: nat)
    requires
        c.ci == 0,
        nvars(inner) <= c.fresh,
        fuel <= trace_inf(),
    ensures
        frame_valid(db, enter_site(c, inner, path), fuel, c.alts.len()),
{
    let base = nvars(inner);
    let delta = (c.fresh - base) as nat;
    let root = roots_cfg(seq![inner]);
    let s = Shadow { cfg: root, fuel: trace_inf(), base, delta };
    let d = enter_site(c, inner, path);
    reveal_with_fuel(nvars_all, 2);
    rename_fixed(inner, base, delta);
    assert(root.fresh == base);
    assert forall|i: int| 0 <= i < root.stack.len() implies proj_goal(
        #[trigger] root.stack[i],
        d.stack[i],
        base,
        delta,
        c.alts.len() + 1,
    ) by {};
    assert(shadow_valid(db, d, fuel, c.alts.len(), s));
}

pub proof fn new_frame_valid(db: Seq<DocClause>, c: TCfg, d: TCfg, fuel: nat)
    requires
        cfg_wf(c),
        fuel <= trace_inf(),
        tstep(db, c) == TStep::Next(d),
        d.alts.len() > c.alts.len(),
    ensures
        frame_valid(db, d, fuel, c.alts.len()),
{
    if c.stack.len() > 0 {
        let rest = c.stack.drop_first();
        match c.stack[0] {
            TGoal::NafCut(l) => {
                assert(l < c.alts.len());
                if let TAlt::Naf { pruned, .. } = c.alts[l as int] {
                    tfail_size(TCfg { alts: c.alts.take(l as int), pruned, ..c });
                }
            },
            TGoal::Lit(g, depth, path) => {
                if depth == 0 {
                    tfail_size(TCfg { pruned: true, ..c });
                } else {
                    match g {
                        Term::Comp(name, args) => {
                            if name == comma_name() && args.len() == 2 {
                            } else if name == naf_name() && args.len() == 1 {
                                assert(c.ci == 0);
                                assert(nvars(g) <= c.fresh);
                                nvars_all_index(args, 0);
                                initialize_shadow(db, c, args[0], path, fuel);
                            } else {
                                tcall_prefix(db, c, name, args, depth, rest, c.ci, path);
                                assert(d.alts.len() == c.alts.len() + 1);
                                assert(d.alts[c.alts.len() as int] is Cl);
                            }
                        },
                        _ => tfail_size(c),
                    }
                }
            },
        }
    }
}

pub proof fn frames_advance(db: Seq<DocClause>, c: TCfg, d: TCfg, fuel: nat)
    requires
        bodies_wf(db),
        cfg_wf(c),
        frames_safe(db, c, fuel),
        0 < fuel <= trace_inf(),
        tstep(db, c) == TStep::Next(d),
    ensures
        frames_safe(db, d, (fuel - 1) as nat),
{
    tstep_prefix(db, c);
    assert forall|i: int| 0 <= i < d.alts.len() implies #[trigger] frame_valid(
        db,
        d,
        (fuel - 1) as nat,
        i as nat,
    ) by {
        if i < c.alts.len() {
            assert(d.alts[i] == c.alts[i]);
            frame_advance(db, c, d, fuel, i as nat);
        } else {
            assert(i == c.alts.len());
            new_frame_valid(db, c, d, (fuel - 1) as nat);
        }
    }
}

pub open spec fn event_safe(db: Seq<DocClause>, event: TEv) -> bool {
    match event {
        TEv::Clause(_) => true,
        TEv::Naf(t) => naf_fails(db, t),
    }
}

pub open spec fn log_safe(db: Seq<DocClause>, log: Seq<(Seq<nat>, TEv)>) -> bool {
    forall|i: int| 0 <= i < log.len() ==> event_safe(db, #[trigger] log[i].1)
}

pub open spec fn alt_log(a: TAlt) -> Seq<(Seq<nat>, TEv)> {
    match a {
        TAlt::Cl { log, .. } | TAlt::Naf { log, .. } => log,
    }
}

pub open spec fn alts_logs_safe(db: Seq<DocClause>, alts: Seq<TAlt>) -> bool {
    forall|i: int| 0 <= i < alts.len() ==> log_safe(db, alt_log(#[trigger] alts[i]))
}

pub open spec fn logs_safe(db: Seq<DocClause>, c: TCfg) -> bool {
    log_safe(db, c.log) && alts_logs_safe(db, c.alts)
}

pub proof fn log_safe_push(
    db: Seq<DocClause>,
    log: Seq<(Seq<nat>, TEv)>,
    path: Seq<nat>,
    event: TEv,
)
    requires
        log_safe(db, log),
        event_safe(db, event),
    ensures
        log_safe(db, log.push((path, event))),
{
    assert forall|i: int| 0 <= i < log.push((path, event)).len() implies event_safe(
        db,
        #[trigger] log.push((path, event))[i].1,
    ) by {
        if i < log.len() {
            assert(event_safe(db, log[i].1));
        }
    }
}

pub proof fn alts_logs_take(db: Seq<DocClause>, alts: Seq<TAlt>, n: nat)
    requires
        alts_logs_safe(db, alts),
        n <= alts.len(),
    ensures
        alts_logs_safe(db, alts.take(n as int)),
{
    assert forall|i: int| 0 <= i < alts.take(n as int).len() implies log_safe(
        db,
        alt_log(#[trigger] alts.take(n as int)[i]),
    ) by {
        assert(log_safe(db, alt_log(alts[i])));
    }
}

pub proof fn alts_logs_push(db: Seq<DocClause>, alts: Seq<TAlt>, a: TAlt)
    requires
        alts_logs_safe(db, alts),
        log_safe(db, alt_log(a)),
    ensures
        alts_logs_safe(db, alts.push(a)),
{
    assert forall|i: int| 0 <= i < alts.push(a).len() implies log_safe(
        db,
        alt_log(#[trigger] alts.push(a)[i]),
    ) by {
        if i < alts.len() {
            assert(log_safe(db, alt_log(alts[i])));
        }
    }
}

pub open spec fn returns_ok(db: Seq<DocClause>, alts: Seq<TAlt>, result: TStep) -> bool {
    match result {
        TStep::Next(c) => c.alts.len() < alts.len() ==> match alts[c.alts.len() as int] {
            TAlt::Naf { inner, .. } => naf_fails(db, inner),
            _ => true,
        },
        _ => true,
    }
}

pub proof fn returns_ok_take(db: Seq<DocClause>, alts: Seq<TAlt>, n: nat, result: TStep)
    requires
        returns_ok(db, alts, result),
        n <= alts.len(),
    ensures
        returns_ok(db, alts.take(n as int), result),
{
}

pub proof fn tfail_logs(db: Seq<DocClause>, c: TCfg)
    requires
        alts_logs_safe(db, c.alts),
        returns_ok(db, c.alts, tfail(c)),
    ensures
        tfail(c) matches TStep::Next(d) ==> logs_safe(db, d),
        !(tfail(c) is Sol),
{
    if c.alts.len() > 0 {
        let i = (c.alts.len() - 1) as nat;
        let rest = c.alts.drop_last();
        assert(rest =~= c.alts.take(i as int));
        alts_logs_take(db, c.alts, i);
        assert(log_safe(db, alt_log(c.alts[i as int])));
        match c.alts.last() {
            TAlt::Cl { .. } => {},
            TAlt::Naf { log, path, inner, .. } => {
                if !c.pruned {
                    assert(naf_fails(db, inner));
                    log_safe_push(db, log, path, TEv::Naf(inner));
                }
            },
        }
    }
}

pub proof fn tcall_logs(
    db: Seq<DocClause>,
    c: TCfg,
    name: Seq<u8>,
    args: Seq<Term>,
    depth: nat,
    rest: Seq<TGoal>,
    ci: nat,
    path: Seq<nat>,
)
    requires
        logs_safe(db, c),
        returns_ok(db, c.alts, tcall(db, c, name, args, depth, rest, ci, path)),
    ensures
        tcall(db, c, name, args, depth, rest, ci, path) matches TStep::Next(d) ==> logs_safe(db, d),
        !(tcall(db, c, name, args, depth, rest, ci, path) is Sol),
    decreases db.len() - ci,
{
    match next_match(db, name, args.len(), ci) {
        Option::None => tfail_logs(db, c),
        Option::Some(m) => {
            next_match_bound(db, name, args.len(), ci);
            let cl = db[m as int];
            match tunify(
                zip(args, args_of(shift(cl.head, c.fresh))),
                tbody_goals(cl.body, c.fresh, (depth - 1) as nat, path) + rest,
            ) {
                TUni::Ok(_) => {
                    log_safe_push(db, c.log, path, TEv::Clause(m));
                    alts_logs_push(
                        db,
                        c.alts,
                        TAlt::Cl { stack: c.stack, fresh: c.fresh, ci: m + 1, log: c.log },
                    );
                },
                TUni::Fail => tcall_logs(db, c, name, args, depth, rest, m + 1, path),
                TUni::Out => {},
            }
        },
    }
}

pub proof fn tstep_logs(db: Seq<DocClause>, c: TCfg)
    requires
        cfg_wf(c),
        logs_safe(db, c),
        returns_ok(db, c.alts, tstep(db, c)),
    ensures
        tstep(db, c) matches TStep::Next(d) ==> logs_safe(db, d),
        tstep(db, c) matches TStep::Sol(log) ==> log_safe(db, log),
{
    if c.stack.len() > 0 {
        let rest = c.stack.drop_first();
        match c.stack[0] {
            TGoal::NafCut(l) => {
                assert(l < c.alts.len());
                if let TAlt::Naf { pruned, .. } = c.alts[l as int] {
                    let d = TCfg { alts: c.alts.take(l as int), pruned, ..c };
                    alts_logs_take(db, c.alts, l);
                    returns_ok_take(db, c.alts, l, tstep(db, c));
                    tfail_logs(db, d);
                }
            },
            TGoal::Lit(g, depth, path) => {
                if depth == 0 {
                    tfail_logs(db, TCfg { pruned: true, ..c });
                } else {
                    match g {
                        Term::Comp(name, args) => {
                            if name == comma_name() && args.len() == 2 {
                            } else if name == naf_name() && args.len() == 1 {
                                alts_logs_push(
                                    db,
                                    c.alts,
                                    TAlt::Naf {
                                        stack: rest,
                                        fresh: c.fresh,
                                        log: c.log,
                                        path,
                                        inner: args[0],
                                        pruned: c.pruned,
                                    },
                                );
                            } else {
                                tcall_logs(db, c, name, args, depth, rest, c.ci, path);
                            }
                        },
                        _ => tfail_logs(db, c),
                    }
                }
            },
        }
    }
}

pub proof fn step_returns_ok(db: Seq<DocClause>, c: TCfg, fuel: nat)
    requires
        bodies_wf(db),
        cfg_wf(c),
        frames_safe(db, c, fuel),
        fuel > 0,
    ensures
        returns_ok(db, c.alts, tstep(db, c)),
{
    if let TStep::Next(d) = tstep(db, c) {
        if d.alts.len() < c.alts.len() {
            frame_return(db, c, d, fuel, d.alts.len());
        }
    }
}

pub proof fn run_logs_safe(db: Seq<DocClause>, c: TCfg, fuel: nat)
    requires
        bodies_wf(db),
        cfg_wf(c),
        logs_safe(db, c),
        frames_safe(db, c, fuel),
        fuel <= trace_inf(),
    ensures
        trun(db, c, fuel).0 matches TOut::Proved(log) ==> log_safe(db, log),
    decreases fuel,
{
    if fuel > 0 {
        step_returns_ok(db, c, fuel);
        tstep_logs(db, c);
        if let TStep::Next(d) = tstep(db, c) {
            tstep_wf(db, c);
            frames_advance(db, c, d, fuel);
            run_logs_safe(db, d, (fuel - 1) as nat);
        }
    }
}

pub proof fn roots_recorded_naf_fails(
    db: Seq<DocClause>,
    goals: Seq<Term>,
    fuel: nat,
    log: Seq<(Seq<nat>, TEv)>,
    path: Seq<nat>,
    t: Term,
)
    requires
        bodies_wf(db),
        fuel <= trace_inf(),
        trun(db, roots_cfg(goals), fuel).0 == TOut::Proved(log),
        log.contains((path, TEv::Naf(t))),
    ensures
        naf_fails(db, t),
{
    let c = roots_cfg(goals);
    roots_wf(goals);
    assert(logs_safe(db, c));
    assert(frames_safe(db, c, fuel));
    run_logs_safe(db, c, fuel);
    let i = choose|i: int| 0 <= i < log.len() && log[i] == (path, TEv::Naf(t));
    assert(event_safe(db, log[i].1));
}

pub proof fn recorded_naf_fails(
    db: Seq<DocClause>,
    goal: Term,
    log: Seq<(Seq<nat>, TEv)>,
    path: Seq<nat>,
    t: Term,
)
    requires
        bodies_wf(db),
        goal_walk(goal) is None,
        trun(db, roots_cfg(conj_leaves(goal)), trace_inf()).0 == TOut::Proved(log),
        log.contains((path, TEv::Naf(t))),
    ensures
        naf_fails(db, t),
{
    roots_recorded_naf_fails(db, conj_leaves(goal), trace_inf(), log, path, t);
}

} // verus!

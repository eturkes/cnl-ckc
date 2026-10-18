use super::naf::*;
use super::*;

verus! {

broadcast use {vstd::seq::group_seq_axioms, vstd::seq_lib::group_seq_properties};
// Selection facts over the trace stack (contract eso D2, D4).

pub open spec fn erased(stack: Seq<TGoal>) -> Seq<Goal> {
    stack.map_values(|g: TGoal| erase(g))
}

pub proof fn tsel_facts(stack: Seq<TGoal>)
    requires
        stack.len() > 0,
    ensures
        tselected(stack) < stack.len(),
        tselected(stack) > 0 ==> match stack[tselected(stack) as int] {
            TGoal::Lit(Term::Comp(n, ts), _, _) => !(n == naf_name() && ts.len() == 1),
            _ => false,
        },
{
    let e = erased(stack);
    crate::k2_select::selected_facts(e);
    let k = selected(e);
    if k > 0 {
        assert(e[k as int] == erase(stack[k as int]));
    }
}

pub proof fn split_at(stack: Seq<TGoal>, k: int)
    requires
        0 <= k < stack.len(),
    ensures
        stack == stack.take(k) + seq![stack[k]] + stack.skip(k + 1),
{
    assert(stack =~= stack.take(k) + seq![stack[k]] + stack.skip(k + 1));
}

// --- injective renaming keeps the selection (the naf shadow's projection) ---
pub proof fn binds_rename(t: Term, x: Term, base: nat, delta: nat)
    requires
        x is Var,
    ensures
        binds(rename(t, base, delta), rename(x, base, delta)) == binds(t, x),
    decreases t,
{
    crate::k2_select::binds_unfold(t, x);
    crate::k2_select::binds_unfold(rename(t, base, delta), rename(x, base, delta));
    match t {
        Term::Comp(n, ts) => {
            rename_all_index(ts, base, delta);
            if n == comma_name() && ts.len() == 2 {
                binds_rename(ts[0], x, base, delta);
                binds_rename(ts[1], x, base, delta);
            } else if ts.len() > 0 {
                rename_inj(ts[0], x, base, delta);
            }
        },
        _ => {},
    }
}

pub proof fn run_binds_proj(
    a: Seq<TGoal>,
    b: Seq<TGoal>,
    j: int,
    x: Term,
    base: nat,
    delta: nat,
    off: nat,
)
    requires
        proj_stack(a, b, base, delta, off),
        0 <= j <= a.len(),
        x is Var,
    ensures
        run_binds(erased(b).skip(j), rename(x, base, delta)) == run_binds(erased(a).skip(j), x),
    decreases a.len() - j,
{
    let ea = erased(a).skip(j);
    let eb = erased(b).skip(j);
    let rx = rename(x, base, delta);
    if j == a.len() {
        crate::k2_select::run_binds_empty(ea, x);
        assert(eb[0] == erase(b[a.len() as int]));
        crate::k2_select::run_binds_cut(eb, rx, (off - 1) as nat);
    } else {
        assert(proj_goal(a[j], b[j], base, delta, off));
        run_binds_proj(a, b, j + 1, x, base, delta, off);
        assert(ea.drop_first() =~= erased(a).skip(j + 1));
        assert(eb.drop_first() =~= erased(b).skip(j + 1));
        assert(ea[0] == erase(a[j]));
        assert(eb[0] == erase(b[j]));
        match (a[j], b[j]) {
            (TGoal::Lit(t, d, _), TGoal::Lit(u, e, _)) => {
                crate::k2_select::run_binds_lit(ea, x, t, d);
                crate::k2_select::run_binds_lit(eb, rx, u, e);
                binds_rename(t, x, base, delta);
                rename_ground(t, base, delta);
                match t {
                    Term::Comp(n, ts) => {
                        rename_all_index(ts, base, delta);
                    },
                    _ => {},
                }
            },
            (TGoal::NafCut(l), TGoal::NafCut(k)) => {
                crate::k2_select::run_binds_cut(ea, x, l);
                crate::k2_select::run_binds_cut(eb, rx, k);
            },
            _ => {},
        }
    }
}

pub proof fn waits_proj(a: Seq<TGoal>, b: Seq<TGoal>, i: int, base: nat, delta: nat, off: nat)
    requires
        proj_stack(a, b, base, delta, off),
        0 <= i < a.len(),
    ensures
        waits(erase(b[i]), erased(b).skip(i + 1)) == waits(erase(a[i]), erased(a).skip(i + 1)),
{
    assert(proj_goal(a[i], b[i], base, delta, off));
    match (a[i], b[i]) {
        (TGoal::Lit(t, d, _), TGoal::Lit(u, e, _)) => {
            match t {
                Term::Comp(n, ts) => {
                    rename_all_index(ts, base, delta);
                    if ts.len() == 3 {
                        match ts[1] {
                            Term::Var(_) => {
                                run_binds_proj(a, b, i + 1, ts[1], base, delta, off);
                            },
                            _ => {},
                        }
                    }
                },
                _ => {},
            }
        },
        _ => {},
    }
}

pub proof fn selected_proj(a: Seq<TGoal>, b: Seq<TGoal>, i: int, base: nat, delta: nat, off: nat)
    requires
        proj_stack(a, b, base, delta, off),
        0 <= i < a.len(),
    ensures
        selected(erased(b).skip(i)) == selected(erased(a).skip(i)),
    decreases a.len() - i,
{
    let ea = erased(a).skip(i);
    let eb = erased(b).skip(i);
    crate::k2_select::selected_unfold(ea);
    crate::k2_select::selected_unfold(eb);
    assert(ea[0] == erase(a[i]));
    assert(eb[0] == erase(b[i]));
    assert(ea.drop_first() =~= erased(a).skip(i + 1));
    assert(eb.drop_first() =~= erased(b).skip(i + 1));
    waits_proj(a, b, i, base, delta, off);
    if i + 1 < a.len() {
        selected_proj(a, b, i + 1, base, delta, off);
    } else {
        // the last goal of the projected level waits on nothing: its run is empty
        match a[i] {
            TGoal::Lit(Term::Comp(_, ts), _, _) => {
                if ts.len() == 3 {
                    crate::k2_select::run_binds_empty(erased(a).skip(i + 1), ts[1]);
                }
            },
            _ => {},
        }
    }
}

pub proof fn tselected_proj(a: Seq<TGoal>, b: Seq<TGoal>, base: nat, delta: nat, off: nat)
    requires
        proj_stack(a, b, base, delta, off),
        a.len() > 0,
    ensures
        tselected(b) == tselected(a),
{
    selected_proj(a, b, 0, base, delta, off);
    assert(erased(a).skip(0) =~= erased(a));
    assert(erased(b).skip(0) =~= erased(b));
}

// --- the parts around a selected goal ---
pub proof fn cuts_parts(stack: Seq<TGoal>, k: int, n: nat)
    requires
        super::goals::cuts_below(stack, n),
        0 <= k < stack.len(),
    ensures
        super::goals::cuts_below(stack.take(k), n),
        super::goals::cuts_below(stack.skip(k + 1), n),
{
    assert forall|i: int|
        0 <= i < stack.take(k).len() && (#[trigger] super::goals::goal_frame(
            stack.take(k)[i],
        )).0 implies super::goals::goal_frame(stack.take(k)[i]).1 < n by {
        assert(stack.take(k)[i] == stack[i]);
    }
    assert forall|i: int|
        0 <= i < stack.skip(k + 1).len() && (#[trigger] super::goals::goal_frame(
            stack.skip(k + 1)[i],
        )).0 implies super::goals::goal_frame(stack.skip(k + 1)[i]).1 < n by {
        assert(stack.skip(k + 1)[i] == stack[k + 1 + i]);
    }
}

// A naf cut of the stack outside the selected literal survives replacing that literal.
pub proof fn in_naf_splice(pre: Seq<TGoal>, g: TGoal, rest: Seq<TGoal>, mid: Seq<TGoal>)
    requires
        in_naf(pre + seq![g] + rest),
        g is Lit,
    ensures
        in_naf(pre + mid + rest),
{
    let old = pre + seq![g] + rest;
    let new = pre + mid + rest;
    let i = choose|i: int| 0 <= i < old.len() && old[i] is NafCut;
    if i < pre.len() {
        assert(old[i] == pre[i]);
        assert(new[i] == pre[i]);
    } else {
        assert(i > pre.len());
        assert(old[i] == rest[i - pre.len() - 1]);
        assert(new[pre.len() + mid.len() + i - pre.len() - 1] == rest[i - pre.len() - 1]);
    }
}

pub proof fn tsel_front_cut(stack: Seq<TGoal>)
    requires
        stack.len() > 0,
        stack[0] is NafCut,
    ensures
        tselected(stack) == 0,
{
    let e = erased(stack);
    crate::k2_select::selected_unfold(e);
    assert(e[0] == erase(stack[0]));
}

// A projected stack splits around a selected index into projected parts.
pub proof fn proj_split(a: Seq<TGoal>, b: Seq<TGoal>, k: int, base: nat, delta: nat, off: nat)
    requires
        proj_stack(a, b, base, delta, off),
        0 <= k < a.len(),
    ensures
        proj_prefix(a.take(k), b.take(k), base, delta, off),
        proj_stack(a.skip(k + 1), b.skip(k + 1), base, delta, off),
        proj_goal(a[k], b[k], base, delta, off),
{
    assert forall|i: int| 0 <= i < a.take(k).len() implies proj_goal(
        #[trigger] a.take(k)[i],
        b.take(k)[i],
        base,
        delta,
        off,
    ) by {
        assert(a.take(k)[i] == a[i]);
        assert(b.take(k)[i] == b[i]);
    }
    let ra = a.skip(k + 1);
    let rb = b.skip(k + 1);
    assert(rb[ra.len() as int] == b[a.len() as int]);
    assert forall|i: int| 0 <= i < ra.len() implies proj_goal(
        #[trigger] ra[i],
        rb[i],
        base,
        delta,
        off,
    ) by {
        assert(ra[i] == a[k + 1 + i]);
        assert(rb[i] == b[k + 1 + i]);
    }
}

} // verus!

//! Executable premise of envelope::naf_extension_wfm (TAS's DC bridge).
use super::refinement::same;
use super::{Program, Result, bad};
use sb1_checker::program::{Clause, Goal, parse_clauses};
use sb1_checker::term::Arena;

pub struct Original<'a> {
    pub document: usize,
    pub ordinal: usize,
    pub text: &'a str,
}
fn goals(a: &Arena, xs: &[Goal], b: &Arena, ys: &[Goal]) -> bool {
    xs.len() == ys.len()
        && xs.iter().zip(ys).all(|(x, y)| match (x, y) {
            (Goal::Positive(x), Goal::Positive(y)) | (Goal::Builtin(x), Goal::Builtin(y)) => {
                same(a, *x, b, *y)
            }
            (Goal::Not(x), Goal::Not(y)) => goals(a, x, b, y),
            _ => false,
        })
}
pub fn check(original: &[Original<'_>], shaped: &Program) -> Result<()> {
    check_ir(original, &shaped.arena, &shaped.clauses, &shaped.patient)
}
pub fn check_ir(
    original: &[Original<'_>],
    shaped: &Arena,
    clauses: &[Clause],
    patient: &[bool],
) -> Result<()> {
    if patient.len() != clauses.len()
        || original.len() != patient.iter().filter(|patient| !**patient).count()
    {
        return Err(bad("NAF-extension clause count"));
    }
    let expected = clauses.iter().enumerate().filter(|(i, _)| !patient[*i]);
    for (actual, (_, c)) in original.iter().zip(expected) {
        if actual.document != c.document || actual.ordinal != c.ordinal {
            return Err(bad("NAF-extension original clause ids/order"));
        }
        // A clause-local arena keeps this binding's transient memory independent
        // of KB size; semantic term equality is across the two arenas.
        let mut arena = Arena::default();
        let clauses = parse_clauses(&mut arena, &format!("{}.\n", actual.text), actual.document)
            .map_err(bad)?;
        let [source] = clauses.as_slice() else {
            return Err(bad("NAF-extension source clause count"));
        };
        if source.names.len() != c.names.len()
            || !same(&arena, source.head, shaped, c.head)
            || source.body.len() > c.body.len()
            || !goals(&arena, &source.body, shaped, &c.body[..source.body.len()])
        {
            return Err(bad(format!(
                "{}:{} DC shape changes head/positive/built-in/original NAF",
                c.document, c.ordinal
            )));
        }
        for added in &c.body[source.body.len()..] {
            let valid = matches!(added, Goal::Not(inner) if matches!(inner.as_slice(), [Goal::Positive(t)] if shaped.named(*t, "guideline_exclusion", 1).is_some_and(|a| shaped.is_ground(a[0]))));
            if !valid {
                return Err(bad(format!(
                    "{}:{} DC addition is not a ground scope-exclusion NAF",
                    c.document, c.ordinal
                )));
            }
        }
    }
    Ok(())
}

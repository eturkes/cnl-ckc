#!/usr/bin/env python3
"""CS2b: projection-notes edits for the condition-scope re-authoring (ruling 4 + census rulings).
usage: notes-update.py <guideline-dir> — exact (cell, old, new) edits per document row; idempotent."""
import sys
from pathlib import Path

D = Path(sys.argv[1])
K, X = 2, 3  # kept, dropped cells
NAF = '(no record: does not provably)'
EDITS = {
    'rec02-imp09': [(K, 'patient-specific classical no-contraindication conditions', f'patient-specific no-contraindication conditions {NAF}')],
    'rec02-imp18': [(X, 'without recast as classical-negation antecedent', f'without recast as a does-not-provably antecedent {NAF}')],
    'rec02-imp21': [(K, 'three classical-negation gates', f'three does-not-provably gates {NAF}')],
    'rec03-imp07': [(K, 'electrocardiographic-monitoring consideration independently implies', f'electrocardiographic-monitoring consideration {NAF} independently implies')],
    'rec03-imp08': [(K, 'or absent patient-education preparation each implies', f'or absent patient-education preparation {NAF} each implies')],
    'rec07-imp15': [(K, 'high-risk regimen without benefit evidence;', f'high-risk regimen without benefit evidence {NAF};')],
    's20-12': [(K, 'no-NSAID-contraindication guard (classical negation)', f'no-NSAID-contraindication guard {NAF}')],
    's21-02': [(K, 'no-contraindication guard;', f'no-contraindication guard {NAF};')],
    's25-08': [(X, 'two classical-negation gates', f'two does-not-provably gates {NAF}')],
    's27-05': [(K, 'no current/previous gastrointestinal bleeding', f'no current/previous gastrointestinal bleeding {NAF}')],
    's28-05': [(X, 'a negated-consideration antecedent', f'a does-not-provably consideration antecedent {NAF}')],
    's31-04': [(K, 'clinician should reserve ER/LA opioid for severe continuous pain (as S30-11);', 'clinician should reserve ER/LA opioid for severe continuous pain (as S30-11) and is recommended not to prescribe one for a pain that is not severe and continuous (classical: an observed classification);'),
               (K, 'during at least 1 week (typed duration), otherwise', f'during at least 1 week (typed duration; {NAF}), otherwise'),
               (X, 'rendered by a classically negated antecedent (fires where non-receipt is stated)', 'rendered by a does-not-provably antecedent (fires where no receipt is recorded)')],
    's32-06': [(K, 'or patient-education preparation (the gates of rec03-imp08)', f'or patient-education preparation {NAF} (the gates of rec03-imp08)')],
    's48-12': [(K, 'medication-specific no-contraindication guards', 'medication-specific no-contraindication guards (no record: is not provably contraindicated)')],
    's54-08': [(K, 'an unexpected screening result that does not have an other-explanation', f'an unexpected screening result that does not have an other-explanation {NAF}')],
    's59-12': [(K, 'while interval absent (SHOULD over not)', f'while interval absent {NAF} (SHOULD over not)')],
    's59-14': [(K, 'unless a very limited circumstance holds (SHOULD over not)', f'unless a very limited circumstance holds {NAF} (SHOULD over not)')],
    's60-10': [(K, 'no-opioid-use-disorder-diagnosis conditions', f'no-opioid-use-disorder-diagnosis {NAF} conditions')],
    's23-06': [(K, 'clinician should avoid stool-softener alone and fiber-laxative alone', f'clinician should avoid a stool softener or fiber laxative that the patient uses without an additional laxative {NAF}'),
               (X, '; without-another-laxative conditions folded into prohibited-treatment compounds', '')],
    's29-02': [(K, 'pain-treatment initiation, and pain-treatment continuation', 'pain-treatment initiation, and pain-treatment continuation; clinical-oversight rationale typed: every opioid initiation and continuation and every pain-treatment initiation and continuation needs a clinical oversight'),
               (X, 'clinical-oversight rationale omitted; ', ''),
               (X, 'because-of clinical-oversight rationale folded into temporal review contexts', 'because-of link between the review duties and the oversight need not typed')],
    'rec03-imp02': [(K, 'clinician should reserve ER/LA opioid for severe continuous pain; for-PP retained', 'clinician should reserve ER/LA opioid for severe continuous pain; for-PP retained; recommended not to prescribe an ER/LA opioid for a pain that is not severe and continuous (classical: an observed classification)')],
    's34-07': [(K, '`closely` typed on monitoring (manner)', f'`closely` typed on monitoring (manner); no dosage increase without close monitoring typed: close monitoring absent {NAF} → recommended not to increase the dosage, within the same few-day acute-pain treatment'),
               (X, 'should-not-attempt-without transformed into positive monitoring duty;', 'should-not-attempt-without typed as a prohibition beside the positive monitoring duty;'),
               (X, 'outpatient scope omitted;', 'outpatient scope carried guideline-wide by the S7-02 facility exclusions;')],
    's39-16': [(K, 'clinician should ensure patient mild-to-moderate withdrawal from full-agonist opioids before first buprenorphine dose', 'clinician should ensure patient mild-to-moderate withdrawal from full-agonist opioids before first buprenorphine dose; a patient taking a full agonist opioid without mild-to-moderate withdrawal (classical: an observed finding) → recommended not to conduct a standard buprenorphine initiation (low-dose initiation, S59-05, stays outside the prohibition)')],
    's40-14': [(K, 'prevent an unintentional long-term transition', f'prevent an unintentional long-term transition; continuation without reassessment typed: no reassessment {NAF} → recommended not to continue the opioid therapy'),
               (X, 'simply-because continued-without-reassessment causal frame compressed into reassess+prevent consequents', 'simply-because causal frame compressed into reassess+prevent consequents; continued-without-reassessment also typed as the continuation prohibition')],
    'rec01-imp04': [(K, 'baseline restoration;', 'baseline restoration; continuation limited to severe pain typed: an additional opioid that the patient does not need for severe acute pain (classical: an observed need) → recommended not to continue it;'),
                    (K, 'long-term opioid-use patient condition on all three rules', 'long-term opioid-use patient condition on all four rules'),
                    (X, '“only for” and pain-severe-enough-to-require qualifier collapsed into limit + duration noun', '“only for” and pain-severe-enough-to-require qualifier collapsed into limit + duration noun, and typed as the continuation prohibition')],
    'rec12': [(K, 'medication-free detoxification not recommended for the disorder', 'medication-free detoxification for a patient with the disorder recommended against (Prohibitions ruling: "not recommended" = recommends against)')],
    'rec12-imp08': [(K, 'general duty retained', f'general duty retained; waiver prerequisite typed: no waiver {NAF} → recommended not to prescribe buprenorphine for opioid use disorder')],
    'rec05-imp26': [(K, 'remain relevant during shorter-duration opioid discontinuation', 'remain relevant during shorter-duration opioid discontinuation; Recommendation 5 scope typed as data: it applies during long-term opioid therapy for subacute or chronic pain'),
                    (X, 'Recommendation 5 long-term subacute/chronic preface and Recommendations 6/7 cross-references omitted', 'Recommendations 6/7 cross-references omitted; Recommendation 5 applicability stays data (no rule consults it)')],
}
p = D / 'audit' / 'projection-notes.tsv'
rows = p.read_text(encoding='utf-8').split('\n')
done = 0
for doc, edits in EDITS.items():
    key = f'cdc2022-opioid-{doc}\t'
    idx = [i for i, r in enumerate(rows) if r.startswith(key)]
    assert len(idx) == 1, doc
    f = rows[idx[0]].split('\t')
    assert len(f) == 4, doc
    for cell, old, new in edits:
        if new and new in f[cell]:
            continue
        if not new and old not in f[cell]:
            continue
        assert f[cell].count(old) == 1, (doc, old)
        f[cell] = f[cell].replace(old, new)
        done += 1
    rows[idx[0]] = '\t'.join(f)
p.write_text('\n'.join(rows), encoding='utf-8')
print(f'notes-update: {done} edits ({len(EDITS)} documents)')

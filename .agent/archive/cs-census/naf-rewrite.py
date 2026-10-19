#!/usr/bin/env python3
"""CS2b ruling 4: rewrite the absence-of-record negations of rule antecedents to `does not provably`.
usage: naf-rewrite.py <guideline-dir> — rewrites ace/<doc>.ace in place; one exact antecedent edit per row; idempotent."""
import sys
from pathlib import Path

D = Path(sys.argv[1])
P = 'does not provably '
EDITS = [  # (doc, sentence, old, new) — old must occur once in that sentence's antecedent
    ('rec02-imp09', 1, 'the patient does not have a contraindication', 'the patient ' + P + 'have a contraindication'),
    ('rec02-imp18', 1, 'If a clinician does not consider', 'If a clinician ' + P + 'consider'),
    ('rec02-imp21', 2, 'If a clinician does not make', 'If a clinician ' + P + 'make'),
    ('rec02-imp21', 3, 'If a clinician does not conduct', 'If a clinician ' + P + 'conduct'),
    ('rec02-imp21', 4, 'If a clinician does not use', 'If a clinician ' + P + 'use'),
    ('rec03-imp07', 1, 'If a clinician does not understand', 'If a clinician ' + P + 'understand'),
    ('rec03-imp07', 2, 'If a clinician does not prepare', 'If a clinician ' + P + 'prepare'),
    ('rec03-imp07', 3, 'If a clinician does not prepare', 'If a clinician ' + P + 'prepare'),
    ('rec03-imp07', 4, 'If a clinician does not assess', 'If a clinician ' + P + 'assess'),
    ('rec03-imp07', 5, 'If a clinician does not consider', 'If a clinician ' + P + 'consider'),
    ('rec03-imp08', 1, 'If a clinician does not understand', 'If a clinician ' + P + 'understand'),
    ('rec03-imp08', 2, 'If a clinician does not understand', 'If a clinician ' + P + 'understand'),
    ('rec03-imp08', 3, 'If a clinician does not prepare', 'If a clinician ' + P + 'prepare'),
    ('rec07-imp15', 3, 'the patient does not have an evidence-of-opioid-benefit', 'the patient ' + P + 'have an evidence-of-opioid-benefit'),
    ('rec07-imp15', 4, 'the patient does not have an evidence-of-opioid-benefit', 'the patient ' + P + 'have an evidence-of-opioid-benefit'),
    ('s20-12', 1, 'the patient does not have an NSAID-contraindication', 'the patient ' + P + 'have an NSAID-contraindication'),
] + [('s21-02', n, 'the patient does not have a contraindication', 'the patient ' + P + 'have a contraindication') for n in range(1, 8)] + [
    ('s25-08', 1, 'the clinician does not conduct', 'the clinician ' + P + 'conduct'),
    ('s25-08', 2, 'the clinician does not make', 'the clinician ' + P + 'make'),
    ('s27-05', 1, 'and does not have a current-gastrointestinal-bleeding and does not have a previous-gastrointestinal-bleeding', 'and ' + P + 'have a current-gastrointestinal-bleeding and ' + P + 'have a previous-gastrointestinal-bleeding'),
    ('s27-05', 2, 'and does not have a current-gastrointestinal-bleeding and does not have a previous-gastrointestinal-bleeding', 'and ' + P + 'have a current-gastrointestinal-bleeding and ' + P + 'have a previous-gastrointestinal-bleeding'),
    ('s28-05', 1, 'If a clinician does not consider', 'If a clinician ' + P + 'consider'),
    ('s31-04', 2, 'If a patient does not receive', 'If a patient ' + P + 'receive'),
    ('s32-06', 3, 'If a clinician does not understand', 'If a clinician ' + P + 'understand'),
    ('s32-06', 4, 'If a clinician does not understand', 'If a clinician ' + P + 'understand'),
    ('s32-06', 5, 'If a clinician does not prepare', 'If a clinician ' + P + 'prepare'),
    ('s48-12', 1, 'a tricyclic-antidepressant is not contraindicated', 'a tricyclic-antidepressant is not provably contraindicated'),
    ('s48-12', 2, 'an SNRI-antidepressant is not contraindicated', 'an SNRI-antidepressant is not provably contraindicated'),
    ('s54-08', 6, 'does not have an other-explanation', P + 'have an other-explanation'),
    ('s59-12', 2, 'If a patient does not have', 'If a patient ' + P + 'have'),
    ('s59-14', 1, 'If a patient does not have', 'If a patient ' + P + 'have'),
    ('s60-10', 1, 'and does not have an opioid-use-disorder-diagnosis', 'and ' + P + 'have an opioid-use-disorder-diagnosis'),
]
done = 0
for doc, n, old, new in EDITS:
    f = D / 'ace' / f'cdc2022-opioid-{doc}.ace'
    lines = f.read_text(encoding='utf-8').split('\n')
    idx = [i for i, l in enumerate(lines) if l]
    i = idx[n - 1]
    ante = lines[i].split(' then ', 1)[0]
    if new in ante:
        continue
    assert ante.count(old) == 1, (doc, n, old)
    lines[i] = lines[i].replace(old, new, 1)
    f.write_text('\n'.join(lines), encoding='utf-8')
    done += 1
print(f'naf-rewrite: {done} edits ({len(EDITS)} rows)')

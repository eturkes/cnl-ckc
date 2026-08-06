% cdc2022-opioid-rec04.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
guideline_document('cdc2022-opioid-rec04',ace_sha256('58e8de0553d2860167522a090632fe75556004b6777df591aedb80683451da78'),ulex(sha256('82f2a92d7aab48ce88f0c83434cf9c92e9f416cfb2d06b7685499dbaf95dc4d2'))).
% S1: Rec04-clinician is an opioid-dosage-clinician.
'opioid-dosage-clinician'('Rec04-clinician').
% S2: Rec04 is a category-a-recommendation.
'category-a-recommendation'('Rec04').
% S3: Rec04 is an evidence-type-3-recommendation.
'evidence-type-3-recommendation'('Rec04').
% S4: Every opioid-dosage-clinician prescribes-lowest-effective-dosage.
'prescribe-lowest-effective-dosage'(A) :- 'opioid-dosage-clinician'(A).
% S5: Does Rec04-clinician prescribe-lowest-effective-dosage?
guideline_query(yesno,'prescribe-lowest-effective-dosage'('Rec04-clinician')).
% S6: Is Rec04 a category-a-recommendation?
guideline_query(yesno,'category-a-recommendation'('Rec04')).
% S7: Is Rec04 an evidence-type-3-recommendation?
guideline_query(yesno,'evidence-type-3-recommendation'('Rec04')).

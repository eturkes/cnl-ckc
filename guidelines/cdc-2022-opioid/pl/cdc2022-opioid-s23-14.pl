% cdc2022-opioid-s23-14.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
:- multifile(guideline_schema_version/1).
:- discontiguous(guideline_schema_version/1).
:- multifile(guideline_document/4).
:- discontiguous(guideline_document/4).
:- multifile(guideline_entity/4).
:- discontiguous(guideline_entity/4).
:- multifile(guideline_cardinality/5).
:- discontiguous(guideline_cardinality/5).
:- multifile(guideline_event/3).
:- discontiguous(guideline_event/3).
:- multifile(guideline_arg/4).
:- discontiguous(guideline_arg/4).
:- multifile(guideline_pp/4).
:- discontiguous(guideline_pp/4).
:- multifile(guideline_property/4).
:- discontiguous(guideline_property/4).
:- multifile(guideline_operator/3).
:- discontiguous(guideline_operator/3).
:- multifile(guideline_interval/6).
:- discontiguous(guideline_interval/6).
:- multifile(guideline_recurrence/4).
:- discontiguous(guideline_recurrence/4).
:- multifile(guideline_frequency/5).
:- discontiguous(guideline_frequency/5).
:- multifile(guideline_order/4).
:- discontiguous(guideline_order/4).
:- multifile(guideline_recurrence_window/7).
:- discontiguous(guideline_recurrence_window/7).
:- multifile(guideline_range/3).
:- discontiguous(guideline_range/3).
:- multifile(guideline_grade/5).
:- discontiguous(guideline_grade/5).
:- multifile(guideline_preference/4).
:- discontiguous(guideline_preference/4).
:- multifile(guideline_qualifier/4).
:- discontiguous(guideline_qualifier/4).
:- multifile(guideline_unit/3).
:- discontiguous(guideline_unit/3).
:- multifile(guideline_exclusion/1).
:- discontiguous(guideline_exclusion/1).
:- multifile(guideline_supersession/4).
:- discontiguous(guideline_supersession/4).
:- multifile(guideline_evaluation_date/1).
:- discontiguous(guideline_evaluation_date/1).
guideline_schema_version(1).
guideline_document('cdc2022-opioid-s23-14',ace_sha256(eb9fe506a169978b38639606c457de41ffddfe91c1819981e62ef1eb582803c1),ulex(sha256(fba48e003101de64df33400ab2595fa7589360ac19f4c77ac3d17bd085f5b8c0)),vocabulary(sha256('335c132fd0e3d3163ba40c255e7db41eecc36492d8098263c505b17f44bbef5a'))).
% S1: Every clinician should discuss an intentional-or-unintentional-unprescribed-person-opioid-sharing-risk-for-household-or-other-person with a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s23-14',1,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-14',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-14',1,ref(2),[A]),'intentional-or-unintentional-unprescribed-person-opioid-sharing-risk-for-household-or-other-person',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-14',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-14',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-14',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-14',1,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-14',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-14',1,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s23-14',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-14',1,ref(4),[A]),discuss) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-14',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-14',1,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-14',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-14',1,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s23-14',1,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s23-14',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-14',1,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-s23-14',1,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S2: Every clinician should discuss a shared-opioid-overdose-at-same-or-lower-dosage-risk with a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s23-14',2,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-14',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-14',2,ref(2),[A]),'shared-opioid-overdose-at-same-or-lower-dosage-risk',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-14',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-14',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-14',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-14',2,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-14',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-14',2,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s23-14',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-14',2,ref(4),[A]),discuss) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-14',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-14',2,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-14',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-14',2,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s23-14',2,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s23-14',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-14',2,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-s23-14',2,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S3: Every clinician should discuss a young-child-or-pet-unintentional-opioid-ingestion-risk with a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s23-14',3,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-14',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-14',3,ref(2),[A]),'young-child-or-pet-unintentional-opioid-ingestion-risk',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-14',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-14',3,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-14',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-14',3,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-14',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-14',3,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s23-14',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-14',3,ref(4),[A]),discuss) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-14',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-14',3,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-14',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-14',3,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s23-14',3,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s23-14',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-14',3,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-s23-14',3,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S4: Every unprescribed-shared-opioid-recipient can possibly experience an overdose at a same-or-lower-dosage.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s23-14',4,box(1),[A]),can) :- guideline_entity(actual,A,'unprescribed-shared-opioid-recipient',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-14',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-14',4,ref(2),[A]),overdose,countable) :- guideline_entity(actual,A,'unprescribed-shared-opioid-recipient',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-14',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-14',4,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'unprescribed-shared-opioid-recipient',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-14',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-14',4,ref(3),[A]),'same-or-lower-dosage',countable) :- guideline_entity(actual,A,'unprescribed-shared-opioid-recipient',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-14',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-14',4,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,'unprescribed-shared-opioid-recipient',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-s23-14',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-14',4,ref(4),[A]),possibility,possibly) :- guideline_entity(actual,A,'unprescribed-shared-opioid-recipient',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s23-14',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-14',4,ref(4),[A]),experience) :- guideline_entity(actual,A,'unprescribed-shared-opioid-recipient',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-14',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-14',4,ref(4),[A]),1,A) :- guideline_entity(actual,A,'unprescribed-shared-opioid-recipient',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-14',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-14',4,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s23-14',4,ref(2),[A])) :- guideline_entity(actual,A,'unprescribed-shared-opioid-recipient',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s23-14',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-14',4,ref(4),[A]),at,'$guideline_id'(product,'cdc2022-opioid-s23-14',4,ref(3),[A])) :- guideline_entity(actual,A,'unprescribed-shared-opioid-recipient',countable), guideline_cardinality(actual,A,na,eq,1).
% S5: Every young-child can possibly experience an unintentional-opioid-ingestion.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s23-14',5,box(1),[A]),can) :- guideline_entity(actual,A,'young-child',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-14',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-14',5,ref(2),[A]),'unintentional-opioid-ingestion',countable) :- guideline_entity(actual,A,'young-child',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-14',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-14',5,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'young-child',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-s23-14',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-14',5,ref(3),[A]),possibility,possibly) :- guideline_entity(actual,A,'young-child',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s23-14',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-14',5,ref(3),[A]),experience) :- guideline_entity(actual,A,'young-child',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-14',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-14',5,ref(3),[A]),1,A) :- guideline_entity(actual,A,'young-child',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-14',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-14',5,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s23-14',5,ref(2),[A])) :- guideline_entity(actual,A,'young-child',countable), guideline_cardinality(actual,A,na,eq,1).
% S6: Every pet can possibly experience an unintentional-opioid-ingestion.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s23-14',6,box(1),[A]),can) :- guideline_entity(actual,A,pet,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-14',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-14',6,ref(2),[A]),'unintentional-opioid-ingestion',countable) :- guideline_entity(actual,A,pet,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-14',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-14',6,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,pet,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-s23-14',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-14',6,ref(3),[A]),possibility,possibly) :- guideline_entity(actual,A,pet,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s23-14',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-14',6,ref(3),[A]),experience) :- guideline_entity(actual,A,pet,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-14',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-14',6,ref(3),[A]),1,A) :- guideline_entity(actual,A,pet,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-14',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-14',6,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s23-14',6,ref(2),[A])) :- guideline_entity(actual,A,pet,countable), guideline_cardinality(actual,A,na,eq,1).

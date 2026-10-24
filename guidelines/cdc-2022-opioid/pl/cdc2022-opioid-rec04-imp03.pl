% cdc2022-opioid-rec04-imp03.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_schema_version(1).
guideline_document('cdc2022-opioid-rec04-imp03',ace_sha256('5fdd549cbdae5ecd0399bfa22c7d29b17160dcd97470f4fdc3f5c62ab8d00a08'),ulex(sha256('64c086862c2f48d491a9fde0ba9f0fcf090e25e84456b9749332ec3ace211ce4')),vocabulary(sha256(c36289aa40f75805a83501ca057b29dd1ba7292b025716853f654890b77ec50c))).
% S1: An opioid-dosage-recommendation applies during an opioid-initiation.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec04-imp03',1,ref(1),[]),'opioid-dosage-recommendation',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec04-imp03',1,ref(1),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec04-imp03',1,ref(2),[]),'opioid-initiation',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec04-imp03',1,ref(2),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-rec04-imp03',1,ref(3),[]),apply).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec04-imp03',1,ref(3),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec04-imp03',1,ref(1),[])).
guideline_pp(actual,'$guideline_id'(product,'cdc2022-opioid-rec04-imp03',1,ref(3),[]),during,'$guideline_id'(product,'cdc2022-opioid-rec04-imp03',1,ref(2),[])).
% S2: An opioid-dosage-recommendation applies during a dosage-increase.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec04-imp03',2,ref(1),[]),'opioid-dosage-recommendation',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec04-imp03',2,ref(1),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec04-imp03',2,ref(2),[]),'dosage-increase',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec04-imp03',2,ref(2),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-rec04-imp03',2,ref(3),[]),apply).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec04-imp03',2,ref(3),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec04-imp03',2,ref(1),[])).
guideline_pp(actual,'$guideline_id'(product,'cdc2022-opioid-rec04-imp03',2,ref(3),[]),during,'$guideline_id'(product,'cdc2022-opioid-rec04-imp03',2,ref(2),[])).
% S3: A dosage-reduction has a different-benefit.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec04-imp03',3,ref(1),[]),'dosage-reduction',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec04-imp03',3,ref(1),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec04-imp03',3,ref(2),[]),'different-benefit',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec04-imp03',3,ref(2),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-rec04-imp03',3,ref(3),[]),have).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec04-imp03',3,ref(3),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec04-imp03',3,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec04-imp03',3,ref(3),[]),2,'$guideline_id'(product,'cdc2022-opioid-rec04-imp03',3,ref(2),[])).
% S4: A dosage-reduction has a different-risk.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec04-imp03',4,ref(1),[]),'dosage-reduction',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec04-imp03',4,ref(1),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec04-imp03',4,ref(2),[]),'different-risk',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec04-imp03',4,ref(2),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-rec04-imp03',4,ref(3),[]),have).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec04-imp03',4,ref(3),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec04-imp03',4,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec04-imp03',4,ref(3),[]),2,'$guideline_id'(product,'cdc2022-opioid-rec04-imp03',4,ref(2),[])).

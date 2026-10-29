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
:- multifile(guideline_supersession/4).
:- discontiguous(guideline_supersession/4).
:- multifile(guideline_evaluation_date/1).
:- discontiguous(guideline_evaluation_date/1).
guideline_schema_version(1).
guideline_document('cdc2022-opioid-rec04-imp03',ace_sha256('5273bd3466a95fa4bebb44de2ec7cc032a02511643f1e5d6e3b2c0a3a9a23b3c'),ulex(sha256(fba48e003101de64df33400ab2595fa7589360ac19f4c77ac3d17bd085f5b8c0)),vocabulary(sha256('335c132fd0e3d3163ba40c255e7db41eecc36492d8098263c505b17f44bbef5a'))).
% S1: If there is an opioid-dosage-recommendation and there is an opioid-initiation then the opioid-dosage-recommendation applies during the opioid-initiation.
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-rec04-imp03',1,ref(3),[A,B]),apply) :- guideline_entity(actual,A,'opioid-dosage-recommendation',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-initiation',countable), guideline_cardinality(actual,B,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec04-imp03',1,ref(3),[A,B]),1,A) :- guideline_entity(actual,A,'opioid-dosage-recommendation',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-initiation',countable), guideline_cardinality(actual,B,na,eq,1).
guideline_pp(actual,'$guideline_id'(product,'cdc2022-opioid-rec04-imp03',1,ref(3),[A,B]),during,B) :- guideline_entity(actual,A,'opioid-dosage-recommendation',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-initiation',countable), guideline_cardinality(actual,B,na,eq,1).
% S2: If there is an opioid-dosage-recommendation and there is a dosage-increase then the opioid-dosage-recommendation applies during the dosage-increase.
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-rec04-imp03',2,ref(3),[A,B]),apply) :- guideline_entity(actual,A,'opioid-dosage-recommendation',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'dosage-increase',countable), guideline_cardinality(actual,B,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec04-imp03',2,ref(3),[A,B]),1,A) :- guideline_entity(actual,A,'opioid-dosage-recommendation',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'dosage-increase',countable), guideline_cardinality(actual,B,na,eq,1).
guideline_pp(actual,'$guideline_id'(product,'cdc2022-opioid-rec04-imp03',2,ref(3),[A,B]),during,B) :- guideline_entity(actual,A,'opioid-dosage-recommendation',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'dosage-increase',countable), guideline_cardinality(actual,B,na,eq,1).
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

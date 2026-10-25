% cdc2022-opioid-s26-14.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s26-14',ace_sha256(b13182eace55aa54c34c6e99ee5df5601a16ad89e6fe48050aead632f1448c03),ulex(sha256('64c086862c2f48d491a9fde0ba9f0fcf090e25e84456b9749332ec3ace211ce4')),vocabulary(sha256(c36289aa40f75805a83501ca057b29dd1ba7292b025716853f654890b77ec50c))).
% S1: Every physical-activity can provide an additional-health-benefit.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s26-14',1,box(1),[A]),can) :- guideline_entity(actual,A,'physical-activity',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s26-14',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-14',1,ref(2),[A]),'additional-health-benefit',countable) :- guideline_entity(actual,A,'physical-activity',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s26-14',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-14',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'physical-activity',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s26-14',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-14',1,ref(3),[A]),provide) :- guideline_entity(actual,A,'physical-activity',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s26-14',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-14',1,ref(3),[A]),1,A) :- guideline_entity(actual,A,'physical-activity',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s26-14',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-14',1,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s26-14',1,ref(2),[A])) :- guideline_entity(actual,A,'physical-activity',countable), guideline_cardinality(actual,A,na,eq,1).
% S2: An additional-health-benefit-example is a depression-symptom-prevention-or-reduction-example.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s26-14',2,ref(1),[]),'additional-health-benefit-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s26-14',2,ref(1),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s26-14',2,ref(2),[]),'depression-symptom-prevention-or-reduction-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s26-14',2,ref(2),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s26-14',2,ref(3),[]),be).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s26-14',2,ref(3),[]),1,'$guideline_id'(product,'cdc2022-opioid-s26-14',2,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s26-14',2,ref(3),[]),2,'$guideline_id'(product,'cdc2022-opioid-s26-14',2,ref(2),[])).

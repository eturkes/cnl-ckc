% cdc2022-opioid-s32-02.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s32-02',ace_sha256(d7c9409a38c189642b3733b9a3988068cc18804fecc3c1cd18a9fe98b3f3d164),ulex(sha256('2733db8c734f4672e1d598cc0911538cb926c6072c8cc42898ec8aa2d7176379')),vocabulary(sha256('335c132fd0e3d3163ba40c255e7db41eecc36492d8098263c505b17f44bbef5a'))).
% S1: Every clinician can possibly need a combined-immediate-release-and-ER-LA-opioid-prescription.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s32-02',1,box(1),[A]),can) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s32-02',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s32-02',1,ref(2),[A]),'combined-immediate-release-and-ER-LA-opioid-prescription',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s32-02',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s32-02',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-s32-02',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s32-02',1,ref(3),[A]),possibility,possibly) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s32-02',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s32-02',1,ref(3),[A]),need) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s32-02',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s32-02',1,ref(3),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s32-02',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s32-02',1,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s32-02',1,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S2: If a clinician prescribes an immediate-release-opioid with an ER-LA-opioid then the clinician should consider an increased-combination-overdose-risk and should prescribe the immediate-release-opioid with the ER-LA-opioid cautiously.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s32-02',2,box(1),[A,B,C,D]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'immediate-release-opioid',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'ER-LA-opioid',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,prescribe), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,with,C), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s32-02',2,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s32-02',2,ref(5),[A,B,C,D]),'increased-combination-overdose-risk',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'immediate-release-opioid',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'ER-LA-opioid',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,prescribe), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,with,C).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s32-02',2,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s32-02',2,ref(5),[A,B,C,D]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'immediate-release-opioid',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'ER-LA-opioid',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,prescribe), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,with,C).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s32-02',2,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s32-02',2,ref(6),[A,B,C,D]),consider) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'immediate-release-opioid',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'ER-LA-opioid',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,prescribe), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,with,C).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s32-02',2,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s32-02',2,ref(6),[A,B,C,D]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'immediate-release-opioid',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'ER-LA-opioid',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,prescribe), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,with,C).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s32-02',2,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s32-02',2,ref(6),[A,B,C,D]),2,'$guideline_id'(product,'cdc2022-opioid-s32-02',2,ref(5),[A,B,C,D])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'immediate-release-opioid',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'ER-LA-opioid',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,prescribe), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,with,C).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s32-02',2,box(2),[A,B,C,D]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'immediate-release-opioid',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'ER-LA-opioid',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,prescribe), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,with,C), \+ guideline_exclusion('cdc-2022-opioid').
guideline_event('$guideline_id'(context,'cdc2022-opioid-s32-02',2,box(2),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s32-02',2,ref(7),[A,B,C,D]),prescribe) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'immediate-release-opioid',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'ER-LA-opioid',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,prescribe), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,with,C).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s32-02',2,box(2),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s32-02',2,ref(7),[A,B,C,D]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'immediate-release-opioid',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'ER-LA-opioid',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,prescribe), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,with,C).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s32-02',2,box(2),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s32-02',2,ref(7),[A,B,C,D]),2,B) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'immediate-release-opioid',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'ER-LA-opioid',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,prescribe), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,with,C).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-s32-02',2,box(2),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s32-02',2,ref(7),[A,B,C,D]),manner,cautiously) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'immediate-release-opioid',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'ER-LA-opioid',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,prescribe), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,with,C).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s32-02',2,box(2),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s32-02',2,ref(7),[A,B,C,D]),with,C) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'immediate-release-opioid',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'ER-LA-opioid',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,prescribe), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,with,C).
% S3: An immediate-release-and-ER-LA-opioid-combination-example includes an ER-LA-to-immediate-release-transition-example and includes a temporary-postoperative-short-term-opioid-use-example and includes a methadone-stabilized-patient-acute-pain-example.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s32-02',3,ref(1),[]),'immediate-release-and-ER-LA-opioid-combination-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s32-02',3,ref(1),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s32-02',3,ref(2),[]),'ER-LA-to-immediate-release-transition-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s32-02',3,ref(2),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s32-02',3,ref(3),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s32-02',3,ref(3),[]),1,'$guideline_id'(product,'cdc2022-opioid-s32-02',3,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s32-02',3,ref(3),[]),2,'$guideline_id'(product,'cdc2022-opioid-s32-02',3,ref(2),[])).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s32-02',3,ref(4),[]),'temporary-postoperative-short-term-opioid-use-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s32-02',3,ref(4),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s32-02',3,ref(5),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s32-02',3,ref(5),[]),1,'$guideline_id'(product,'cdc2022-opioid-s32-02',3,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s32-02',3,ref(5),[]),2,'$guideline_id'(product,'cdc2022-opioid-s32-02',3,ref(4),[])).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s32-02',3,ref(6),[]),'methadone-stabilized-patient-acute-pain-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s32-02',3,ref(6),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s32-02',3,ref(7),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s32-02',3,ref(7),[]),1,'$guideline_id'(product,'cdc2022-opioid-s32-02',3,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s32-02',3,ref(7),[]),2,'$guideline_id'(product,'cdc2022-opioid-s32-02',3,ref(6),[])).
% S4: Every combined-immediate-release-and-ER-LA-opioid-prescription can possibly increase an overdose-risk.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s32-02',4,box(1),[A]),can) :- guideline_entity(actual,A,'combined-immediate-release-and-ER-LA-opioid-prescription',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s32-02',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s32-02',4,ref(2),[A]),'overdose-risk',countable) :- guideline_entity(actual,A,'combined-immediate-release-and-ER-LA-opioid-prescription',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s32-02',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s32-02',4,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'combined-immediate-release-and-ER-LA-opioid-prescription',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-s32-02',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s32-02',4,ref(3),[A]),possibility,possibly) :- guideline_entity(actual,A,'combined-immediate-release-and-ER-LA-opioid-prescription',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s32-02',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s32-02',4,ref(3),[A]),increase) :- guideline_entity(actual,A,'combined-immediate-release-and-ER-LA-opioid-prescription',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s32-02',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s32-02',4,ref(3),[A]),1,A) :- guideline_entity(actual,A,'combined-immediate-release-and-ER-LA-opioid-prescription',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s32-02',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s32-02',4,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s32-02',4,ref(2),[A])) :- guideline_entity(actual,A,'combined-immediate-release-and-ER-LA-opioid-prescription',countable), guideline_cardinality(actual,A,na,eq,1).

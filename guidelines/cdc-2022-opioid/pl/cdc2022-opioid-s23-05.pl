% cdc2022-opioid-s23-05.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s23-05',ace_sha256('637f19717d06818d8ae061be6799e8aa3557df23cc5d6b84025f3f4a90bce180'),ulex(sha256('2733db8c734f4672e1d598cc0911538cb926c6072c8cc42898ec8aa2d7176379')),vocabulary(sha256('335c132fd0e3d3163ba40c255e7db41eecc36492d8098263c505b17f44bbef5a'))).
% S1: If a patient has a prolonged-opioid-use for a few-day-period then the patient can possibly need a prophylactic-pharmacologic-bowel-therapy for a regular-bowel-movement.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s23-05',1,box(1),[A,B,C,D]),can) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'prolonged-opioid-use',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'few-day-period',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,have), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-05',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s23-05',1,ref(5),[A,B,C,D]),'prophylactic-pharmacologic-bowel-therapy',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'prolonged-opioid-use',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'few-day-period',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,have), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-05',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s23-05',1,ref(5),[A,B,C,D]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'prolonged-opioid-use',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'few-day-period',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,have), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-05',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s23-05',1,ref(6),[A,B,C,D]),'regular-bowel-movement',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'prolonged-opioid-use',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'few-day-period',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,have), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-05',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s23-05',1,ref(6),[A,B,C,D]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'prolonged-opioid-use',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'few-day-period',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,have), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-s23-05',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s23-05',1,ref(7),[A,B,C,D]),possibility,possibly) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'prolonged-opioid-use',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'few-day-period',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,have), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s23-05',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s23-05',1,ref(7),[A,B,C,D]),need) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'prolonged-opioid-use',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'few-day-period',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,have), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-05',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s23-05',1,ref(7),[A,B,C,D]),1,A) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'prolonged-opioid-use',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'few-day-period',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,have), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-05',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s23-05',1,ref(7),[A,B,C,D]),2,'$guideline_id'(product,'cdc2022-opioid-s23-05',1,ref(5),[A,B,C,D])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'prolonged-opioid-use',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'few-day-period',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,have), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s23-05',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s23-05',1,ref(7),[A,B,C,D]),for,'$guideline_id'(product,'cdc2022-opioid-s23-05',1,ref(6),[A,B,C,D])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'prolonged-opioid-use',countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'few-day-period',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,have), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,for,C).
% S2: A prophylactic-pharmacologic-bowel-therapy-example includes a stimulant-laxative-example.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s23-05',2,ref(1),[]),'prophylactic-pharmacologic-bowel-therapy-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s23-05',2,ref(1),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s23-05',2,ref(2),[]),'stimulant-laxative-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s23-05',2,ref(2),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s23-05',2,ref(3),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s23-05',2,ref(3),[]),1,'$guideline_id'(product,'cdc2022-opioid-s23-05',2,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s23-05',2,ref(3),[]),2,'$guideline_id'(product,'cdc2022-opioid-s23-05',2,ref(2),[])).
% S3: A stimulant-laxative-example is a senna-example.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s23-05',3,ref(1),[]),'stimulant-laxative-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s23-05',3,ref(1),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s23-05',3,ref(2),[]),'senna-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s23-05',3,ref(2),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s23-05',3,ref(3),[]),be).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s23-05',3,ref(3),[]),1,'$guideline_id'(product,'cdc2022-opioid-s23-05',3,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s23-05',3,ref(3),[]),2,'$guideline_id'(product,'cdc2022-opioid-s23-05',3,ref(2),[])).
% S4: It is possible that a prophylactic-pharmacologic-bowel-therapy-example includes a stool-softener-example.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s23-05',4,box(1),[]),can).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-05',4,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-s23-05',4,ref(1),[]),'prophylactic-pharmacologic-bowel-therapy-example',countable).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-05',4,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-s23-05',4,ref(1),[]),na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-05',4,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-s23-05',4,ref(2),[]),'stool-softener-example',countable).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-05',4,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-s23-05',4,ref(2),[]),na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s23-05',4,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-s23-05',4,ref(3),[]),include).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-05',4,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-s23-05',4,ref(3),[]),1,'$guideline_id'(product,'cdc2022-opioid-s23-05',4,ref(1),[])).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-05',4,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-s23-05',4,ref(3),[]),2,'$guideline_id'(product,'cdc2022-opioid-s23-05',4,ref(2),[])).

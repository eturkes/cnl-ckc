% cdc2022-opioid-s59-03.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s59-03',ace_sha256(ce2ac71cf077eca72160cca182569d893a635f63f8b1a2491cf5a459d6c8b97a),ulex(sha256('2733db8c734f4672e1d598cc0911538cb926c6072c8cc42898ec8aa2d7176379')),vocabulary(sha256('335c132fd0e3d3163ba40c255e7db41eecc36492d8098263c505b17f44bbef5a'))).
% S1: If a patient does not have an objective-opioid-withdrawal-sign and there is a clinician then it is recommended that the clinician does not generally conduct a standard-buprenorphine-initiation for the patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s59-03',1,box(2),[A,B]),should) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(C,D,'objective-opioid-withdrawal-sign',countable), guideline_operator(actual,C,-), guideline_cardinality(C,D,na,eq,1), guideline_event(C,E,have), guideline_arg(C,E,1,A), guideline_arg(C,E,2,D), guideline_entity(actual,B,clinician,countable), guideline_cardinality(actual,B,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_operator('$guideline_id'(context,'cdc2022-opioid-s59-03',1,box(2),[A,B]),'$guideline_id'(context,'cdc2022-opioid-s59-03',1,box(3),[A,B]),-) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(C,D,'objective-opioid-withdrawal-sign',countable), guideline_operator(actual,C,-), guideline_cardinality(C,D,na,eq,1), guideline_event(C,E,have), guideline_arg(C,E,1,A), guideline_arg(C,E,2,D), guideline_entity(actual,B,clinician,countable), guideline_cardinality(actual,B,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s59-03',1,box(3),[A,B]),'$guideline_id'(product,'cdc2022-opioid-s59-03',1,ref(5),[A,B]),'standard-buprenorphine-initiation',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(C,D,'objective-opioid-withdrawal-sign',countable), guideline_operator(actual,C,-), guideline_cardinality(C,D,na,eq,1), guideline_event(C,E,have), guideline_arg(C,E,1,A), guideline_arg(C,E,2,D), guideline_entity(actual,B,clinician,countable), guideline_cardinality(actual,B,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s59-03',1,box(3),[A,B]),'$guideline_id'(product,'cdc2022-opioid-s59-03',1,ref(5),[A,B]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(C,D,'objective-opioid-withdrawal-sign',countable), guideline_operator(actual,C,-), guideline_cardinality(C,D,na,eq,1), guideline_event(C,E,have), guideline_arg(C,E,1,A), guideline_arg(C,E,2,D), guideline_entity(actual,B,clinician,countable), guideline_cardinality(actual,B,na,eq,1).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-s59-03',1,box(3),[A,B]),'$guideline_id'(product,'cdc2022-opioid-s59-03',1,ref(6),[A,B]),usuality,generally) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(C,D,'objective-opioid-withdrawal-sign',countable), guideline_operator(actual,C,-), guideline_cardinality(C,D,na,eq,1), guideline_event(C,E,have), guideline_arg(C,E,1,A), guideline_arg(C,E,2,D), guideline_entity(actual,B,clinician,countable), guideline_cardinality(actual,B,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s59-03',1,box(3),[A,B]),'$guideline_id'(product,'cdc2022-opioid-s59-03',1,ref(6),[A,B]),conduct) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(C,D,'objective-opioid-withdrawal-sign',countable), guideline_operator(actual,C,-), guideline_cardinality(C,D,na,eq,1), guideline_event(C,E,have), guideline_arg(C,E,1,A), guideline_arg(C,E,2,D), guideline_entity(actual,B,clinician,countable), guideline_cardinality(actual,B,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s59-03',1,box(3),[A,B]),'$guideline_id'(product,'cdc2022-opioid-s59-03',1,ref(6),[A,B]),1,B) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(C,D,'objective-opioid-withdrawal-sign',countable), guideline_operator(actual,C,-), guideline_cardinality(C,D,na,eq,1), guideline_event(C,E,have), guideline_arg(C,E,1,A), guideline_arg(C,E,2,D), guideline_entity(actual,B,clinician,countable), guideline_cardinality(actual,B,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s59-03',1,box(3),[A,B]),'$guideline_id'(product,'cdc2022-opioid-s59-03',1,ref(6),[A,B]),2,'$guideline_id'(product,'cdc2022-opioid-s59-03',1,ref(5),[A,B])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(C,D,'objective-opioid-withdrawal-sign',countable), guideline_operator(actual,C,-), guideline_cardinality(C,D,na,eq,1), guideline_event(C,E,have), guideline_arg(C,E,1,A), guideline_arg(C,E,2,D), guideline_entity(actual,B,clinician,countable), guideline_cardinality(actual,B,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s59-03',1,box(3),[A,B]),'$guideline_id'(product,'cdc2022-opioid-s59-03',1,ref(6),[A,B]),for,A) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(C,D,'objective-opioid-withdrawal-sign',countable), guideline_operator(actual,C,-), guideline_cardinality(C,D,na,eq,1), guideline_event(C,E,have), guideline_arg(C,E,1,A), guideline_arg(C,E,2,D), guideline_entity(actual,B,clinician,countable), guideline_cardinality(actual,B,na,eq,1).
% S2: Every buprenorphine is a partial-agonist-opioid.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s59-03',2,ref(2),[A]),'partial-agonist-opioid',countable) :- guideline_entity(actual,A,buprenorphine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s59-03',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,buprenorphine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s59-03',2,ref(3),[A]),be) :- guideline_entity(actual,A,buprenorphine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s59-03',2,ref(3),[A]),1,A) :- guideline_entity(actual,A,buprenorphine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s59-03',2,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s59-03',2,ref(2),[A])) :- guideline_entity(actual,A,buprenorphine,countable), guideline_cardinality(actual,A,na,eq,1).

% cdc2022-opioid-s23-02.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s23-02',ace_sha256('421456e4453b1330de83d59beecf2da946a60c19ce1ae546cc5055b8638c75f9'),ulex(sha256(fba48e003101de64df33400ab2595fa7589360ac19f4c77ac3d17bd085f5b8c0)),vocabulary(sha256('335c132fd0e3d3163ba40c255e7db41eecc36492d8098263c505b17f44bbef5a'))).
% S1: Every clinician should advise a potentially-fatal-respiratory-depression-risk with a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s23-02',1,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-02',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-02',1,ref(2),[A]),'potentially-fatal-respiratory-depression-risk',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-02',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-02',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-02',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-02',1,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-02',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-02',1,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s23-02',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-02',1,ref(4),[A]),advise) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-02',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-02',1,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-02',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-02',1,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s23-02',1,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s23-02',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-02',1,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-s23-02',1,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S2: Every clinician should advise a serious-opioid-use-disorder-risk with a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s23-02',2,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-02',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-02',2,ref(2),[A]),'serious-opioid-use-disorder-risk',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-02',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-02',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-02',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-02',2,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-02',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-02',2,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s23-02',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-02',2,ref(4),[A]),advise) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-02',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-02',2,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-02',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-02',2,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s23-02',2,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s23-02',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-02',2,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-s23-02',2,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S3: Every clinician should advise an opioid-use-disorder-distress-risk with a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s23-02',3,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-02',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-02',3,ref(2),[A]),'opioid-use-disorder-distress-risk',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-02',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-02',3,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-02',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-02',3,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-02',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-02',3,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s23-02',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-02',3,ref(4),[A]),advise) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-02',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-02',3,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-02',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-02',3,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s23-02',3,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s23-02',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-02',3,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-s23-02',3,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S4: Every clinician should advise a work-school-or-home-obligation-impairment-risk with a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s23-02',4,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-02',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-02',4,ref(2),[A]),'work-school-or-home-obligation-impairment-risk',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-02',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-02',4,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-02',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-02',4,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-02',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-02',4,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s23-02',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-02',4,ref(4),[A]),advise) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-02',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-02',4,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-02',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-02',4,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s23-02',4,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s23-02',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-02',4,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-s23-02',4,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S5: Every opioid-use-disorder can cause an opioid-use-disorder-related-distress and can cause a work-school-or-home-obligation-fulfillment-inability.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s23-02',5,box(1),[A]),can) :- guideline_entity(actual,A,'opioid-use-disorder',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-02',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-02',5,ref(2),[A]),'opioid-use-disorder-related-distress',countable) :- guideline_entity(actual,A,'opioid-use-disorder',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-02',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-02',5,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-use-disorder',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s23-02',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-02',5,ref(3),[A]),cause) :- guideline_entity(actual,A,'opioid-use-disorder',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-02',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-02',5,ref(3),[A]),1,A) :- guideline_entity(actual,A,'opioid-use-disorder',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-02',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-02',5,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s23-02',5,ref(2),[A])) :- guideline_entity(actual,A,'opioid-use-disorder',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s23-02',5,box(2),[A]),can) :- guideline_entity(actual,A,'opioid-use-disorder',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s23-02',5,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-02',5,ref(4),[A]),'work-school-or-home-obligation-fulfillment-inability',countable) :- guideline_entity(actual,A,'opioid-use-disorder',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s23-02',5,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-02',5,ref(4),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-use-disorder',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s23-02',5,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-02',5,ref(5),[A]),cause) :- guideline_entity(actual,A,'opioid-use-disorder',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-02',5,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-02',5,ref(5),[A]),1,A) :- guideline_entity(actual,A,'opioid-use-disorder',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s23-02',5,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s23-02',5,ref(5),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s23-02',5,ref(4),[A])) :- guideline_entity(actual,A,'opioid-use-disorder',countable), guideline_cardinality(actual,A,na,eq,1).

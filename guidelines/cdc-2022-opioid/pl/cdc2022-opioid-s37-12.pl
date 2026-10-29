% cdc2022-opioid-s37-12.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s37-12',ace_sha256('5a8bad5ce3fce04abe9180dbb1c97441e9f80fb8ee1bf0c0dda94d652257fd6e'),ulex(sha256(fba48e003101de64df33400ab2595fa7589360ac19f4c77ac3d17bd085f5b8c0)),vocabulary(sha256('335c132fd0e3d3163ba40c255e7db41eecc36492d8098263c505b17f44bbef5a'))).
% S1: If a clinician tapers a benzodiazepine then the clinician should taper the benzodiazepine gradually.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s37-12',1,box(1),[A,B,C]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,benzodiazepine,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,taper), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), \+ guideline_exclusion('cdc-2022-opioid').
guideline_event('$guideline_id'(context,'cdc2022-opioid-s37-12',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s37-12',1,ref(4),[A,B,C]),taper) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,benzodiazepine,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,taper), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s37-12',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s37-12',1,ref(4),[A,B,C]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,benzodiazepine,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,taper), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s37-12',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s37-12',1,ref(4),[A,B,C]),2,B) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,benzodiazepine,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,taper), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-s37-12',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s37-12',1,ref(4),[A,B,C]),manner,gradually) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,benzodiazepine,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,taper), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
% S2: Every benzodiazepine-withdrawal can cause an anxiety and can cause a hallucination and can cause a seizure and can cause some delirium-tremens and can rarely cause a death.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s37-12',2,box(1),[A]),can) :- guideline_entity(actual,A,'benzodiazepine-withdrawal',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s37-12',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-12',2,ref(2),[A]),anxiety,countable) :- guideline_entity(actual,A,'benzodiazepine-withdrawal',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s37-12',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-12',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'benzodiazepine-withdrawal',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s37-12',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-12',2,ref(3),[A]),cause) :- guideline_entity(actual,A,'benzodiazepine-withdrawal',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s37-12',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-12',2,ref(3),[A]),1,A) :- guideline_entity(actual,A,'benzodiazepine-withdrawal',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s37-12',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-12',2,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s37-12',2,ref(2),[A])) :- guideline_entity(actual,A,'benzodiazepine-withdrawal',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s37-12',2,box(2),[A]),can) :- guideline_entity(actual,A,'benzodiazepine-withdrawal',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s37-12',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-12',2,ref(4),[A]),hallucination,countable) :- guideline_entity(actual,A,'benzodiazepine-withdrawal',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s37-12',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-12',2,ref(4),[A]),na,eq,1) :- guideline_entity(actual,A,'benzodiazepine-withdrawal',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s37-12',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-12',2,ref(5),[A]),cause) :- guideline_entity(actual,A,'benzodiazepine-withdrawal',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s37-12',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-12',2,ref(5),[A]),1,A) :- guideline_entity(actual,A,'benzodiazepine-withdrawal',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s37-12',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-12',2,ref(5),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s37-12',2,ref(4),[A])) :- guideline_entity(actual,A,'benzodiazepine-withdrawal',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s37-12',2,box(3),[A]),can) :- guideline_entity(actual,A,'benzodiazepine-withdrawal',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s37-12',2,box(3),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-12',2,ref(6),[A]),seizure,countable) :- guideline_entity(actual,A,'benzodiazepine-withdrawal',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s37-12',2,box(3),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-12',2,ref(6),[A]),na,eq,1) :- guideline_entity(actual,A,'benzodiazepine-withdrawal',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s37-12',2,box(3),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-12',2,ref(7),[A]),cause) :- guideline_entity(actual,A,'benzodiazepine-withdrawal',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s37-12',2,box(3),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-12',2,ref(7),[A]),1,A) :- guideline_entity(actual,A,'benzodiazepine-withdrawal',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s37-12',2,box(3),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-12',2,ref(7),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s37-12',2,ref(6),[A])) :- guideline_entity(actual,A,'benzodiazepine-withdrawal',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s37-12',2,box(4),[A]),can) :- guideline_entity(actual,A,'benzodiazepine-withdrawal',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s37-12',2,box(4),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-12',2,ref(8),[A]),'delirium-tremens',mass) :- guideline_entity(actual,A,'benzodiazepine-withdrawal',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s37-12',2,box(4),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-12',2,ref(8),[A]),na,na,na) :- guideline_entity(actual,A,'benzodiazepine-withdrawal',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s37-12',2,box(4),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-12',2,ref(9),[A]),cause) :- guideline_entity(actual,A,'benzodiazepine-withdrawal',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s37-12',2,box(4),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-12',2,ref(9),[A]),1,A) :- guideline_entity(actual,A,'benzodiazepine-withdrawal',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s37-12',2,box(4),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-12',2,ref(9),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s37-12',2,ref(8),[A])) :- guideline_entity(actual,A,'benzodiazepine-withdrawal',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s37-12',2,box(5),[A]),can) :- guideline_entity(actual,A,'benzodiazepine-withdrawal',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s37-12',2,box(5),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-12',2,ref(10),[A]),death,countable) :- guideline_entity(actual,A,'benzodiazepine-withdrawal',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s37-12',2,box(5),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-12',2,ref(10),[A]),na,eq,1) :- guideline_entity(actual,A,'benzodiazepine-withdrawal',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-s37-12',2,box(5),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-12',2,ref(11),[A]),usuality,rarely) :- guideline_entity(actual,A,'benzodiazepine-withdrawal',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s37-12',2,box(5),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-12',2,ref(11),[A]),cause) :- guideline_entity(actual,A,'benzodiazepine-withdrawal',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s37-12',2,box(5),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-12',2,ref(11),[A]),1,A) :- guideline_entity(actual,A,'benzodiazepine-withdrawal',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s37-12',2,box(5),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-12',2,ref(11),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s37-12',2,ref(10),[A])) :- guideline_entity(actual,A,'benzodiazepine-withdrawal',countable), guideline_cardinality(actual,A,na,eq,1).

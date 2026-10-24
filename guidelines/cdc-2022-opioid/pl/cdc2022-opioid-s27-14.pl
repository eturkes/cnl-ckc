% cdc2022-opioid-s27-14.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s27-14',ace_sha256('89d25f3025693a50032a38ea3bbc36112468ecd8c904ab900215b09646bcfa82'),ulex(sha256('45608095e3f77853fdabcce148a9eb7633a8ae776353b8f008c753b846886588')),vocabulary(sha256(c36289aa40f75805a83501ca057b29dd1ba7292b025716853f654890b77ec50c))).
% S1: If a patient has an age and the age includes at least 65 years then it is possible that a tricyclic-antidepressant is inappropriate for the patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s27-14',1,box(1),[A,B,C,D,E]),can) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,age,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_event(actual,E,include), guideline_arg(actual,E,1,B), guideline_arg(actual,E,2,D), guideline_cardinality(actual,D,na,F,G), guideline_unit(actual,D,H), guideline_satisfies(F,G,H,geq,65,year).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s27-14',1,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-s27-14',1,ref(6),[A,B,C,D,E]),'tricyclic-antidepressant',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,age,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_event(actual,E,include), guideline_arg(actual,E,1,B), guideline_arg(actual,E,2,D), guideline_cardinality(actual,D,na,F,G), guideline_unit(actual,D,H), guideline_satisfies(F,G,H,geq,65,year).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s27-14',1,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-s27-14',1,ref(6),[A,B,C,D,E]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,age,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_event(actual,E,include), guideline_arg(actual,E,1,B), guideline_arg(actual,E,2,D), guideline_cardinality(actual,D,na,F,G), guideline_unit(actual,D,H), guideline_satisfies(F,G,H,geq,65,year).
guideline_property('$guideline_id'(context,'cdc2022-opioid-s27-14',1,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-s27-14',1,ref(7),[A,B,C,D,E]),inappropriate,pos) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,age,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_event(actual,E,include), guideline_arg(actual,E,1,B), guideline_arg(actual,E,2,D), guideline_cardinality(actual,D,na,F,G), guideline_unit(actual,D,H), guideline_satisfies(F,G,H,geq,65,year).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s27-14',1,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-s27-14',1,ref(8),[A,B,C,D,E]),be) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,age,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_event(actual,E,include), guideline_arg(actual,E,1,B), guideline_arg(actual,E,2,D), guideline_cardinality(actual,D,na,F,G), guideline_unit(actual,D,H), guideline_satisfies(F,G,H,geq,65,year).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s27-14',1,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-s27-14',1,ref(8),[A,B,C,D,E]),1,'$guideline_id'(product,'cdc2022-opioid-s27-14',1,ref(6),[A,B,C,D,E])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,age,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_event(actual,E,include), guideline_arg(actual,E,1,B), guideline_arg(actual,E,2,D), guideline_cardinality(actual,D,na,F,G), guideline_unit(actual,D,H), guideline_satisfies(F,G,H,geq,65,year).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s27-14',1,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-s27-14',1,ref(8),[A,B,C,D,E]),2,'$guideline_id'(product,'cdc2022-opioid-s27-14',1,ref(7),[A,B,C,D,E])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,age,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_event(actual,E,include), guideline_arg(actual,E,1,B), guideline_arg(actual,E,2,D), guideline_cardinality(actual,D,na,F,G), guideline_unit(actual,D,H), guideline_satisfies(F,G,H,geq,65,year).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s27-14',1,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-s27-14',1,ref(8),[A,B,C,D,E]),for,A) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,age,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_event(actual,E,include), guideline_arg(actual,E,1,B), guideline_arg(actual,E,2,D), guideline_cardinality(actual,D,na,F,G), guideline_unit(actual,D,H), guideline_satisfies(F,G,H,geq,65,year).
% S2: Every tricyclic-antidepressant causes an anticholinergic-effect.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s27-14',2,ref(2),[A]),'anticholinergic-effect',countable) :- guideline_entity(actual,A,'tricyclic-antidepressant',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s27-14',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'tricyclic-antidepressant',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s27-14',2,ref(3),[A]),cause) :- guideline_entity(actual,A,'tricyclic-antidepressant',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s27-14',2,ref(3),[A]),1,A) :- guideline_entity(actual,A,'tricyclic-antidepressant',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s27-14',2,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s27-14',2,ref(2),[A])) :- guideline_entity(actual,A,'tricyclic-antidepressant',countable), guideline_cardinality(actual,A,na,eq,1).

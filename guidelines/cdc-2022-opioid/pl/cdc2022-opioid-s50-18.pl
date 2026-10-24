% cdc2022-opioid-s50-18.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s50-18',ace_sha256('04027fedd66f5e6978dacd426b2a5d2733d969309942e3ac4065dac6c4e16db9'),ulex(sha256('45608095e3f77853fdabcce148a9eb7633a8ae776353b8f008c753b846886588')),vocabulary(sha256(c36289aa40f75805a83501ca057b29dd1ba7292b025716853f654890b77ec50c))).
% S1: If a clinician dismisses a patient on a PDMP-information-basis then the clinician can adversely affect a patient-safety and can cause a missed-lifesaving-information-opportunity and can cause a missed-lifesaving-intervention-opportunity.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s50-18',1,box(1),[A,B,C,D]),can) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,patient,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'PDMP-information-basis',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,dismiss), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,on,C).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s50-18',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s50-18',1,ref(5),[A,B,C,D]),'patient-safety',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,patient,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'PDMP-information-basis',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,dismiss), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,on,C).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s50-18',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s50-18',1,ref(5),[A,B,C,D]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,patient,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'PDMP-information-basis',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,dismiss), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,on,C).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-s50-18',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s50-18',1,ref(6),[A,B,C,D]),manner,adversely) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,patient,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'PDMP-information-basis',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,dismiss), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,on,C).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s50-18',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s50-18',1,ref(6),[A,B,C,D]),affect) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,patient,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'PDMP-information-basis',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,dismiss), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,on,C).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s50-18',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s50-18',1,ref(6),[A,B,C,D]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,patient,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'PDMP-information-basis',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,dismiss), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,on,C).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s50-18',1,box(1),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s50-18',1,ref(6),[A,B,C,D]),2,'$guideline_id'(product,'cdc2022-opioid-s50-18',1,ref(5),[A,B,C,D])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,patient,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'PDMP-information-basis',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,dismiss), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,on,C).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s50-18',1,box(2),[A,B,C,D]),can) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,patient,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'PDMP-information-basis',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,dismiss), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,on,C).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s50-18',1,box(2),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s50-18',1,ref(7),[A,B,C,D]),'missed-lifesaving-information-opportunity',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,patient,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'PDMP-information-basis',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,dismiss), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,on,C).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s50-18',1,box(2),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s50-18',1,ref(7),[A,B,C,D]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,patient,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'PDMP-information-basis',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,dismiss), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,on,C).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s50-18',1,box(2),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s50-18',1,ref(8),[A,B,C,D]),cause) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,patient,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'PDMP-information-basis',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,dismiss), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,on,C).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s50-18',1,box(2),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s50-18',1,ref(8),[A,B,C,D]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,patient,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'PDMP-information-basis',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,dismiss), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,on,C).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s50-18',1,box(2),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s50-18',1,ref(8),[A,B,C,D]),2,'$guideline_id'(product,'cdc2022-opioid-s50-18',1,ref(7),[A,B,C,D])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,patient,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'PDMP-information-basis',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,dismiss), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,on,C).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s50-18',1,box(3),[A,B,C,D]),can) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,patient,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'PDMP-information-basis',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,dismiss), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,on,C).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s50-18',1,box(3),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s50-18',1,ref(9),[A,B,C,D]),'missed-lifesaving-intervention-opportunity',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,patient,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'PDMP-information-basis',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,dismiss), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,on,C).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s50-18',1,box(3),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s50-18',1,ref(9),[A,B,C,D]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,patient,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'PDMP-information-basis',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,dismiss), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,on,C).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s50-18',1,box(3),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s50-18',1,ref(10),[A,B,C,D]),cause) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,patient,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'PDMP-information-basis',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,dismiss), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,on,C).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s50-18',1,box(3),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s50-18',1,ref(10),[A,B,C,D]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,patient,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'PDMP-information-basis',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,dismiss), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,on,C).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s50-18',1,box(3),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s50-18',1,ref(10),[A,B,C,D]),2,'$guideline_id'(product,'cdc2022-opioid-s50-18',1,ref(9),[A,B,C,D])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,patient,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'PDMP-information-basis',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,dismiss), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,on,C).
% S2: A lifesaving-information-example includes a prescription-opioid-risk-information-example and includes an overdose-prevention-information-example.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s50-18',2,ref(1),[]),'lifesaving-information-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s50-18',2,ref(1),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s50-18',2,ref(2),[]),'prescription-opioid-risk-information-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s50-18',2,ref(2),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s50-18',2,ref(3),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s50-18',2,ref(3),[]),1,'$guideline_id'(product,'cdc2022-opioid-s50-18',2,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s50-18',2,ref(3),[]),2,'$guideline_id'(product,'cdc2022-opioid-s50-18',2,ref(2),[])).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s50-18',2,ref(4),[]),'overdose-prevention-information-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s50-18',2,ref(4),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s50-18',2,ref(5),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s50-18',2,ref(5),[]),1,'$guideline_id'(product,'cdc2022-opioid-s50-18',2,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s50-18',2,ref(5),[]),2,'$guideline_id'(product,'cdc2022-opioid-s50-18',2,ref(4),[])).
% S3: A lifesaving-intervention-example includes a safer-prescription-example and includes a nonopioid-pain-treatment-example and includes a naloxone-example and includes an effective-substance-use-disorder-treatment-example.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s50-18',3,ref(1),[]),'lifesaving-intervention-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s50-18',3,ref(1),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s50-18',3,ref(2),[]),'safer-prescription-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s50-18',3,ref(2),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s50-18',3,ref(3),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s50-18',3,ref(3),[]),1,'$guideline_id'(product,'cdc2022-opioid-s50-18',3,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s50-18',3,ref(3),[]),2,'$guideline_id'(product,'cdc2022-opioid-s50-18',3,ref(2),[])).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s50-18',3,ref(4),[]),'nonopioid-pain-treatment-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s50-18',3,ref(4),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s50-18',3,ref(5),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s50-18',3,ref(5),[]),1,'$guideline_id'(product,'cdc2022-opioid-s50-18',3,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s50-18',3,ref(5),[]),2,'$guideline_id'(product,'cdc2022-opioid-s50-18',3,ref(4),[])).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s50-18',3,ref(6),[]),'naloxone-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s50-18',3,ref(6),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s50-18',3,ref(7),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s50-18',3,ref(7),[]),1,'$guideline_id'(product,'cdc2022-opioid-s50-18',3,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s50-18',3,ref(7),[]),2,'$guideline_id'(product,'cdc2022-opioid-s50-18',3,ref(6),[])).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s50-18',3,ref(8),[]),'effective-substance-use-disorder-treatment-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s50-18',3,ref(8),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s50-18',3,ref(9),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s50-18',3,ref(9),[]),1,'$guideline_id'(product,'cdc2022-opioid-s50-18',3,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s50-18',3,ref(9),[]),2,'$guideline_id'(product,'cdc2022-opioid-s50-18',3,ref(8),[])).

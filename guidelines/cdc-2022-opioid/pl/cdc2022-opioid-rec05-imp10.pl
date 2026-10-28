% cdc2022-opioid-rec05-imp10.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-rec05-imp10',ace_sha256(d3c2d26408afe01c0cb54fb03ada9a8fee4829f01fc36b6a54c9ad655a46af6f),ulex(sha256('2733db8c734f4672e1d598cc0911538cb926c6072c8cc42898ec8aa2d7176379')),vocabulary(sha256('335c132fd0e3d3163ba40c255e7db41eecc36492d8098263c505b17f44bbef5a'))).
% S1: If a clinician reduces an opioid then the clinician should slowly taper the opioid for a withdrawal-minimization.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec05-imp10',1,box(1),[A,B,C]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,reduce), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp10',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',1,ref(4),[A,B,C]),'withdrawal-minimization',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,reduce), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp10',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',1,ref(4),[A,B,C]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,reduce), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-rec05-imp10',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',1,ref(5),[A,B,C]),manner,slowly) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,reduce), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec05-imp10',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',1,ref(5),[A,B,C]),taper) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,reduce), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp10',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',1,ref(5),[A,B,C]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,reduce), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp10',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',1,ref(5),[A,B,C]),2,B) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,reduce), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec05-imp10',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',1,ref(5),[A,B,C]),for,'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',1,ref(4),[A,B,C])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,reduce), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
% S2: If a clinician discontinues an opioid then the clinician should slowly taper the opioid for a withdrawal-minimization.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec05-imp10',2,box(1),[A,B,C]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,discontinue), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp10',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',2,ref(4),[A,B,C]),'withdrawal-minimization',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,discontinue), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp10',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',2,ref(4),[A,B,C]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,discontinue), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-rec05-imp10',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',2,ref(5),[A,B,C]),manner,slowly) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,discontinue), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec05-imp10',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',2,ref(5),[A,B,C]),taper) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,discontinue), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp10',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',2,ref(5),[A,B,C]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,discontinue), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp10',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',2,ref(5),[A,B,C]),2,B) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,discontinue), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec05-imp10',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',2,ref(5),[A,B,C]),for,'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',2,ref(4),[A,B,C])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,discontinue), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
% S3: An opioid-withdrawal-symptom-and-sign-example includes an anxiety-example and includes an insomnia-example and includes an abdominal-pain-example and includes a vomiting-example and includes a diarrhea-example and includes a diaphoresis-example and includes a mydriasis-example and includes a tremor-example and includes a tachycardia-example and includes a piloerection-example.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',3,ref(1),[]),'opioid-withdrawal-symptom-and-sign-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',3,ref(1),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',3,ref(2),[]),'anxiety-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',3,ref(2),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',3,ref(3),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',3,ref(3),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',3,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',3,ref(3),[]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',3,ref(2),[])).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',3,ref(4),[]),'insomnia-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',3,ref(4),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',3,ref(5),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',3,ref(5),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',3,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',3,ref(5),[]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',3,ref(4),[])).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',3,ref(6),[]),'abdominal-pain-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',3,ref(6),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',3,ref(7),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',3,ref(7),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',3,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',3,ref(7),[]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',3,ref(6),[])).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',3,ref(8),[]),'vomiting-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',3,ref(8),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',3,ref(9),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',3,ref(9),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',3,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',3,ref(9),[]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',3,ref(8),[])).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',3,ref(10),[]),'diarrhea-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',3,ref(10),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',3,ref(11),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',3,ref(11),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',3,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',3,ref(11),[]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',3,ref(10),[])).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',3,ref(12),[]),'diaphoresis-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',3,ref(12),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',3,ref(13),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',3,ref(13),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',3,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',3,ref(13),[]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',3,ref(12),[])).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',3,ref(14),[]),'mydriasis-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',3,ref(14),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',3,ref(15),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',3,ref(15),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',3,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',3,ref(15),[]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',3,ref(14),[])).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',3,ref(16),[]),'tremor-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',3,ref(16),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',3,ref(17),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',3,ref(17),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',3,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',3,ref(17),[]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',3,ref(16),[])).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',3,ref(18),[]),'tachycardia-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',3,ref(18),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',3,ref(19),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',3,ref(19),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',3,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',3,ref(19),[]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',3,ref(18),[])).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',3,ref(20),[]),'piloerection-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',3,ref(20),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',3,ref(21),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',3,ref(21),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',3,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',3,ref(21),[]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp10',3,ref(20),[])).

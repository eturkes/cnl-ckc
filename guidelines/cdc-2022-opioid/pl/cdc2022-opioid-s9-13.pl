% cdc2022-opioid-s9-13.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s9-13',ace_sha256('621ae0d0022f7829c2318d0109c50a780a24907f15e9ca491f633cbf7f9bac66'),ulex(sha256('2733db8c734f4672e1d598cc0911538cb926c6072c8cc42898ec8aa2d7176379')),vocabulary(sha256('335c132fd0e3d3163ba40c255e7db41eecc36492d8098263c505b17f44bbef5a'))).
% S1: Every opioid-pain-clinical-practice-guideline updates a chronic-pain-management-recommendation on a new-evidence-basis.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s9-13',1,ref(2),[A]),'chronic-pain-management-recommendation',countable) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s9-13',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s9-13',1,ref(3),[A]),'new-evidence-basis',countable) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s9-13',1,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s9-13',1,ref(4),[A]),update) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s9-13',1,ref(4),[A]),1,A) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s9-13',1,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s9-13',1,ref(2),[A])) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp(actual,'$guideline_id'(product,'cdc2022-opioid-s9-13',1,ref(4),[A]),on,'$guideline_id'(product,'cdc2022-opioid-s9-13',1,ref(3),[A])) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
% S2: Every opioid-pain-clinical-practice-guideline should assist a clinician in an opioid-pain-medication-benefit-and-risk-weighing for a painful-acute-condition.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s9-13',2,box(1),[A]),should) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s9-13',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-13',2,ref(2),[A]),clinician,countable) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s9-13',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-13',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s9-13',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-13',2,ref(3),[A]),'opioid-pain-medication-benefit-and-risk-weighing',countable) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s9-13',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-13',2,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s9-13',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-13',2,ref(4),[A]),'painful-acute-condition',countable) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s9-13',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-13',2,ref(4),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s9-13',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-13',2,ref(5),[A]),assist) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s9-13',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-13',2,ref(5),[A]),1,A) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s9-13',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-13',2,ref(5),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s9-13',2,ref(2),[A])) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s9-13',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-13',2,ref(5),[A]),for,'$guideline_id'(product,'cdc2022-opioid-s9-13',2,ref(4),[A])) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s9-13',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-13',2,ref(5),[A]),in,'$guideline_id'(product,'cdc2022-opioid-s9-13',2,ref(3),[A])) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
% S3: Every opioid-pain-clinical-practice-guideline should assist a clinician in an opioid-pain-medication-benefit-and-risk-weighing for a procedure-related-pain.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s9-13',3,box(1),[A]),should) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s9-13',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-13',3,ref(2),[A]),clinician,countable) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s9-13',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-13',3,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s9-13',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-13',3,ref(3),[A]),'opioid-pain-medication-benefit-and-risk-weighing',countable) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s9-13',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-13',3,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s9-13',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-13',3,ref(4),[A]),'procedure-related-pain',countable) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s9-13',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-13',3,ref(4),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s9-13',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-13',3,ref(5),[A]),assist) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s9-13',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-13',3,ref(5),[A]),1,A) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s9-13',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-13',3,ref(5),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s9-13',3,ref(2),[A])) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s9-13',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-13',3,ref(5),[A]),for,'$guideline_id'(product,'cdc2022-opioid-s9-13',3,ref(4),[A])) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s9-13',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s9-13',3,ref(5),[A]),in,'$guideline_id'(product,'cdc2022-opioid-s9-13',3,ref(3),[A])) :- guideline_entity(actual,A,'opioid-pain-clinical-practice-guideline',countable), guideline_cardinality(actual,A,na,eq,1).
% S4: A painful-acute-condition-example includes a low-back-pain-example and includes a neck-pain-example and includes an other-musculoskeletal-pain-example and includes a neuropathic-pain-example and includes a dental-pain-example and includes a kidney-stone-pain-example and includes an acute-episodic-migraine-example.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s9-13',4,ref(1),[]),'painful-acute-condition-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s9-13',4,ref(1),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s9-13',4,ref(2),[]),'low-back-pain-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s9-13',4,ref(2),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s9-13',4,ref(3),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s9-13',4,ref(3),[]),1,'$guideline_id'(product,'cdc2022-opioid-s9-13',4,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s9-13',4,ref(3),[]),2,'$guideline_id'(product,'cdc2022-opioid-s9-13',4,ref(2),[])).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s9-13',4,ref(4),[]),'neck-pain-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s9-13',4,ref(4),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s9-13',4,ref(5),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s9-13',4,ref(5),[]),1,'$guideline_id'(product,'cdc2022-opioid-s9-13',4,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s9-13',4,ref(5),[]),2,'$guideline_id'(product,'cdc2022-opioid-s9-13',4,ref(4),[])).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s9-13',4,ref(6),[]),'other-musculoskeletal-pain-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s9-13',4,ref(6),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s9-13',4,ref(7),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s9-13',4,ref(7),[]),1,'$guideline_id'(product,'cdc2022-opioid-s9-13',4,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s9-13',4,ref(7),[]),2,'$guideline_id'(product,'cdc2022-opioid-s9-13',4,ref(6),[])).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s9-13',4,ref(8),[]),'neuropathic-pain-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s9-13',4,ref(8),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s9-13',4,ref(9),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s9-13',4,ref(9),[]),1,'$guideline_id'(product,'cdc2022-opioid-s9-13',4,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s9-13',4,ref(9),[]),2,'$guideline_id'(product,'cdc2022-opioid-s9-13',4,ref(8),[])).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s9-13',4,ref(10),[]),'dental-pain-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s9-13',4,ref(10),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s9-13',4,ref(11),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s9-13',4,ref(11),[]),1,'$guideline_id'(product,'cdc2022-opioid-s9-13',4,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s9-13',4,ref(11),[]),2,'$guideline_id'(product,'cdc2022-opioid-s9-13',4,ref(10),[])).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s9-13',4,ref(12),[]),'kidney-stone-pain-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s9-13',4,ref(12),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s9-13',4,ref(13),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s9-13',4,ref(13),[]),1,'$guideline_id'(product,'cdc2022-opioid-s9-13',4,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s9-13',4,ref(13),[]),2,'$guideline_id'(product,'cdc2022-opioid-s9-13',4,ref(12),[])).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s9-13',4,ref(14),[]),'acute-episodic-migraine-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s9-13',4,ref(14),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s9-13',4,ref(15),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s9-13',4,ref(15),[]),1,'$guideline_id'(product,'cdc2022-opioid-s9-13',4,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s9-13',4,ref(15),[]),2,'$guideline_id'(product,'cdc2022-opioid-s9-13',4,ref(14),[])).
% S5: A procedure-related-pain-example includes a postoperative-pain-example and includes an oral-surgery-pain-example.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s9-13',5,ref(1),[]),'procedure-related-pain-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s9-13',5,ref(1),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s9-13',5,ref(2),[]),'postoperative-pain-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s9-13',5,ref(2),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s9-13',5,ref(3),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s9-13',5,ref(3),[]),1,'$guideline_id'(product,'cdc2022-opioid-s9-13',5,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s9-13',5,ref(3),[]),2,'$guideline_id'(product,'cdc2022-opioid-s9-13',5,ref(2),[])).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s9-13',5,ref(4),[]),'oral-surgery-pain-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s9-13',5,ref(4),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s9-13',5,ref(5),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s9-13',5,ref(5),[]),1,'$guideline_id'(product,'cdc2022-opioid-s9-13',5,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s9-13',5,ref(5),[]),2,'$guideline_id'(product,'cdc2022-opioid-s9-13',5,ref(4),[])).

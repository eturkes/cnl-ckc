% cdc2022-opioid-s52-18.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s52-18',ace_sha256(c2cc0525d621e3170d267a596a8e1d172fc687e3cc582512712fb25876e1303d),ulex(sha256('2733db8c734f4672e1d598cc0911538cb926c6072c8cc42898ec8aa2d7176379')),vocabulary(sha256('335c132fd0e3d3163ba40c255e7db41eecc36492d8098263c505b17f44bbef5a'))).
% S1: Every PDMP-review-recommendation-category-B-rating acknowledges a PDMP-availability-variation and acknowledges a circumstance-variation.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s52-18',1,ref(2),[A]),'PDMP-availability-variation',countable) :- guideline_entity(actual,A,'PDMP-review-recommendation-category-B-rating',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s52-18',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'PDMP-review-recommendation-category-B-rating',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s52-18',1,ref(3),[A]),acknowledge) :- guideline_entity(actual,A,'PDMP-review-recommendation-category-B-rating',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s52-18',1,ref(3),[A]),1,A) :- guideline_entity(actual,A,'PDMP-review-recommendation-category-B-rating',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s52-18',1,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s52-18',1,ref(2),[A])) :- guideline_entity(actual,A,'PDMP-review-recommendation-category-B-rating',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s52-18',1,ref(4),[A]),'circumstance-variation',countable) :- guideline_entity(actual,A,'PDMP-review-recommendation-category-B-rating',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s52-18',1,ref(4),[A]),na,eq,1) :- guideline_entity(actual,A,'PDMP-review-recommendation-category-B-rating',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s52-18',1,ref(5),[A]),acknowledge) :- guideline_entity(actual,A,'PDMP-review-recommendation-category-B-rating',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s52-18',1,ref(5),[A]),1,A) :- guideline_entity(actual,A,'PDMP-review-recommendation-category-B-rating',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s52-18',1,ref(5),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s52-18',1,ref(4),[A])) :- guideline_entity(actual,A,'PDMP-review-recommendation-category-B-rating',countable), guideline_cardinality(actual,A,na,eq,1).
% S2: A PDMP-access-failure-exception-example involves a severe-acute-pain-emergency-department-patient-example and involves a PDMP-system-access-failure-example and involves an hours-long-prescription-wait-adverse-effect-example and involves a reasonable-clinician-adverse-effect-determination-example.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s52-18',2,ref(1),[]),'PDMP-access-failure-exception-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s52-18',2,ref(1),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s52-18',2,ref(2),[]),'severe-acute-pain-emergency-department-patient-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s52-18',2,ref(2),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s52-18',2,ref(3),[]),involve).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s52-18',2,ref(3),[]),1,'$guideline_id'(product,'cdc2022-opioid-s52-18',2,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s52-18',2,ref(3),[]),2,'$guideline_id'(product,'cdc2022-opioid-s52-18',2,ref(2),[])).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s52-18',2,ref(4),[]),'PDMP-system-access-failure-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s52-18',2,ref(4),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s52-18',2,ref(5),[]),involve).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s52-18',2,ref(5),[]),1,'$guideline_id'(product,'cdc2022-opioid-s52-18',2,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s52-18',2,ref(5),[]),2,'$guideline_id'(product,'cdc2022-opioid-s52-18',2,ref(4),[])).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s52-18',2,ref(6),[]),'hours-long-prescription-wait-adverse-effect-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s52-18',2,ref(6),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s52-18',2,ref(7),[]),involve).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s52-18',2,ref(7),[]),1,'$guideline_id'(product,'cdc2022-opioid-s52-18',2,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s52-18',2,ref(7),[]),2,'$guideline_id'(product,'cdc2022-opioid-s52-18',2,ref(6),[])).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s52-18',2,ref(8),[]),'reasonable-clinician-adverse-effect-determination-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s52-18',2,ref(8),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s52-18',2,ref(9),[]),involve).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s52-18',2,ref(9),[]),1,'$guideline_id'(product,'cdc2022-opioid-s52-18',2,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s52-18',2,ref(9),[]),2,'$guideline_id'(product,'cdc2022-opioid-s52-18',2,ref(8),[])).

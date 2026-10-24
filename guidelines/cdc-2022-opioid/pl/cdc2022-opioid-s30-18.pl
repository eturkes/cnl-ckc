% cdc2022-opioid-s30-18.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s30-18',ace_sha256('9a1f2a1a709dcef99720d2b98aa10a649390b11338a7eef5c4e6e5a39d8d4c8d'),ulex(sha256('45608095e3f77853fdabcce148a9eb7633a8ae776353b8f008c753b846886588')),vocabulary(sha256(c36289aa40f75805a83501ca057b29dd1ba7292b025716853f654890b77ec50c))).
% S1: Every multimodal-therapy can outperform a usual-care in a long-term-pain-and-disability-reduction.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s30-18',1,box(1),[A]),can) :- guideline_entity(actual,A,'multimodal-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s30-18',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-18',1,ref(2),[A]),'usual-care',countable) :- guideline_entity(actual,A,'multimodal-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s30-18',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-18',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'multimodal-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s30-18',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-18',1,ref(3),[A]),'long-term-pain-and-disability-reduction',countable) :- guideline_entity(actual,A,'multimodal-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s30-18',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-18',1,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,'multimodal-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s30-18',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-18',1,ref(4),[A]),outperform) :- guideline_entity(actual,A,'multimodal-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s30-18',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-18',1,ref(4),[A]),1,A) :- guideline_entity(actual,A,'multimodal-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s30-18',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-18',1,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s30-18',1,ref(2),[A])) :- guideline_entity(actual,A,'multimodal-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s30-18',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-18',1,ref(4),[A]),in,'$guideline_id'(product,'cdc2022-opioid-s30-18',1,ref(3),[A])) :- guideline_entity(actual,A,'multimodal-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
% S2: Every multimodal-therapy can outperform a physical-treatment-alone in a long-term-pain-and-disability-reduction.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s30-18',2,box(1),[A]),can) :- guideline_entity(actual,A,'multimodal-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s30-18',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-18',2,ref(2),[A]),'physical-treatment-alone',countable) :- guideline_entity(actual,A,'multimodal-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s30-18',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-18',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'multimodal-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s30-18',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-18',2,ref(3),[A]),'long-term-pain-and-disability-reduction',countable) :- guideline_entity(actual,A,'multimodal-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s30-18',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-18',2,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,'multimodal-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s30-18',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-18',2,ref(4),[A]),outperform) :- guideline_entity(actual,A,'multimodal-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s30-18',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-18',2,ref(4),[A]),1,A) :- guideline_entity(actual,A,'multimodal-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s30-18',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-18',2,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s30-18',2,ref(2),[A])) :- guideline_entity(actual,A,'multimodal-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s30-18',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-18',2,ref(4),[A]),in,'$guideline_id'(product,'cdc2022-opioid-s30-18',2,ref(3),[A])) :- guideline_entity(actual,A,'multimodal-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
% S3: Every multidisciplinary-biopsychosocial-rehabilitation can outperform a usual-care in a long-term-pain-and-disability-reduction.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s30-18',3,box(1),[A]),can) :- guideline_entity(actual,A,'multidisciplinary-biopsychosocial-rehabilitation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s30-18',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-18',3,ref(2),[A]),'usual-care',countable) :- guideline_entity(actual,A,'multidisciplinary-biopsychosocial-rehabilitation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s30-18',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-18',3,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'multidisciplinary-biopsychosocial-rehabilitation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s30-18',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-18',3,ref(3),[A]),'long-term-pain-and-disability-reduction',countable) :- guideline_entity(actual,A,'multidisciplinary-biopsychosocial-rehabilitation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s30-18',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-18',3,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,'multidisciplinary-biopsychosocial-rehabilitation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s30-18',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-18',3,ref(4),[A]),outperform) :- guideline_entity(actual,A,'multidisciplinary-biopsychosocial-rehabilitation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s30-18',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-18',3,ref(4),[A]),1,A) :- guideline_entity(actual,A,'multidisciplinary-biopsychosocial-rehabilitation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s30-18',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-18',3,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s30-18',3,ref(2),[A])) :- guideline_entity(actual,A,'multidisciplinary-biopsychosocial-rehabilitation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s30-18',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-18',3,ref(4),[A]),in,'$guideline_id'(product,'cdc2022-opioid-s30-18',3,ref(3),[A])) :- guideline_entity(actual,A,'multidisciplinary-biopsychosocial-rehabilitation',countable), guideline_cardinality(actual,A,na,eq,1).
% S4: Every multidisciplinary-biopsychosocial-rehabilitation can outperform a physical-treatment-alone in a long-term-pain-and-disability-reduction.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s30-18',4,box(1),[A]),can) :- guideline_entity(actual,A,'multidisciplinary-biopsychosocial-rehabilitation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s30-18',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-18',4,ref(2),[A]),'physical-treatment-alone',countable) :- guideline_entity(actual,A,'multidisciplinary-biopsychosocial-rehabilitation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s30-18',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-18',4,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'multidisciplinary-biopsychosocial-rehabilitation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s30-18',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-18',4,ref(3),[A]),'long-term-pain-and-disability-reduction',countable) :- guideline_entity(actual,A,'multidisciplinary-biopsychosocial-rehabilitation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s30-18',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-18',4,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,'multidisciplinary-biopsychosocial-rehabilitation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s30-18',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-18',4,ref(4),[A]),outperform) :- guideline_entity(actual,A,'multidisciplinary-biopsychosocial-rehabilitation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s30-18',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-18',4,ref(4),[A]),1,A) :- guideline_entity(actual,A,'multidisciplinary-biopsychosocial-rehabilitation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s30-18',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-18',4,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s30-18',4,ref(2),[A])) :- guideline_entity(actual,A,'multidisciplinary-biopsychosocial-rehabilitation',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s30-18',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s30-18',4,ref(4),[A]),in,'$guideline_id'(product,'cdc2022-opioid-s30-18',4,ref(3),[A])) :- guideline_entity(actual,A,'multidisciplinary-biopsychosocial-rehabilitation',countable), guideline_cardinality(actual,A,na,eq,1).
% S5: A multidisciplinary-biopsychosocial-rehabilitation-example is a psychological-therapy-and-exercise-combination-example.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s30-18',5,ref(1),[]),'multidisciplinary-biopsychosocial-rehabilitation-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s30-18',5,ref(1),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s30-18',5,ref(2),[]),'psychological-therapy-and-exercise-combination-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s30-18',5,ref(2),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s30-18',5,ref(3),[]),be).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s30-18',5,ref(3),[]),1,'$guideline_id'(product,'cdc2022-opioid-s30-18',5,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s30-18',5,ref(3),[]),2,'$guideline_id'(product,'cdc2022-opioid-s30-18',5,ref(2),[])).
% S6: A physical-treatment-example is an exercise-example.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s30-18',6,ref(1),[]),'physical-treatment-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s30-18',6,ref(1),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s30-18',6,ref(2),[]),'exercise-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s30-18',6,ref(2),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s30-18',6,ref(3),[]),be).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s30-18',6,ref(3),[]),1,'$guideline_id'(product,'cdc2022-opioid-s30-18',6,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s30-18',6,ref(3),[]),2,'$guideline_id'(product,'cdc2022-opioid-s30-18',6,ref(2),[])).

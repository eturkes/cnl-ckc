% cdc2022-opioid-s26-13.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s26-13',ace_sha256('6efda4ee5f29f2f0b8155ebbc9e334e71c38e48ddef37bb68e723a4e253c2b31'),ulex(sha256(fba48e003101de64df33400ab2595fa7589360ac19f4c77ac3d17bd085f5b8c0)),vocabulary(sha256('335c132fd0e3d3163ba40c255e7db41eecc36492d8098263c505b17f44bbef5a'))).
% S1: Every exercise can encourage an active-care-plan-participation and can help a life-pain-effect-addressing.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s26-13',1,box(1),[A]),can) :- guideline_entity(actual,A,exercise,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s26-13',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-13',1,ref(2),[A]),'active-care-plan-participation',countable) :- guideline_entity(actual,A,exercise,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s26-13',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-13',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,exercise,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s26-13',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-13',1,ref(3),[A]),encourage) :- guideline_entity(actual,A,exercise,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s26-13',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-13',1,ref(3),[A]),1,A) :- guideline_entity(actual,A,exercise,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s26-13',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-13',1,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s26-13',1,ref(2),[A])) :- guideline_entity(actual,A,exercise,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s26-13',1,box(2),[A]),can) :- guideline_entity(actual,A,exercise,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s26-13',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-13',1,ref(4),[A]),'life-pain-effect-addressing',countable) :- guideline_entity(actual,A,exercise,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s26-13',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-13',1,ref(4),[A]),na,eq,1) :- guideline_entity(actual,A,exercise,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s26-13',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-13',1,ref(5),[A]),help) :- guideline_entity(actual,A,exercise,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s26-13',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-13',1,ref(5),[A]),1,A) :- guideline_entity(actual,A,exercise,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s26-13',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-13',1,ref(5),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s26-13',1,ref(4),[A])) :- guideline_entity(actual,A,exercise,countable), guideline_cardinality(actual,A,na,eq,1).
% S2: Every mind-body-intervention can encourage an active-care-plan-participation and can help a life-pain-effect-addressing.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s26-13',2,box(1),[A]),can) :- guideline_entity(actual,A,'mind-body-intervention',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s26-13',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-13',2,ref(2),[A]),'active-care-plan-participation',countable) :- guideline_entity(actual,A,'mind-body-intervention',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s26-13',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-13',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'mind-body-intervention',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s26-13',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-13',2,ref(3),[A]),encourage) :- guideline_entity(actual,A,'mind-body-intervention',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s26-13',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-13',2,ref(3),[A]),1,A) :- guideline_entity(actual,A,'mind-body-intervention',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s26-13',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-13',2,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s26-13',2,ref(2),[A])) :- guideline_entity(actual,A,'mind-body-intervention',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s26-13',2,box(2),[A]),can) :- guideline_entity(actual,A,'mind-body-intervention',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s26-13',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-13',2,ref(4),[A]),'life-pain-effect-addressing',countable) :- guideline_entity(actual,A,'mind-body-intervention',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s26-13',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-13',2,ref(4),[A]),na,eq,1) :- guideline_entity(actual,A,'mind-body-intervention',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s26-13',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-13',2,ref(5),[A]),help) :- guideline_entity(actual,A,'mind-body-intervention',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s26-13',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-13',2,ref(5),[A]),1,A) :- guideline_entity(actual,A,'mind-body-intervention',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s26-13',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-13',2,ref(5),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s26-13',2,ref(4),[A])) :- guideline_entity(actual,A,'mind-body-intervention',countable), guideline_cardinality(actual,A,na,eq,1).
% S3: Every behavioral-treatment can encourage an active-care-plan-participation and can help a life-pain-effect-addressing.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s26-13',3,box(1),[A]),can) :- guideline_entity(actual,A,'behavioral-treatment',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s26-13',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-13',3,ref(2),[A]),'active-care-plan-participation',countable) :- guideline_entity(actual,A,'behavioral-treatment',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s26-13',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-13',3,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'behavioral-treatment',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s26-13',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-13',3,ref(3),[A]),encourage) :- guideline_entity(actual,A,'behavioral-treatment',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s26-13',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-13',3,ref(3),[A]),1,A) :- guideline_entity(actual,A,'behavioral-treatment',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s26-13',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-13',3,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s26-13',3,ref(2),[A])) :- guideline_entity(actual,A,'behavioral-treatment',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s26-13',3,box(2),[A]),can) :- guideline_entity(actual,A,'behavioral-treatment',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s26-13',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-13',3,ref(4),[A]),'life-pain-effect-addressing',countable) :- guideline_entity(actual,A,'behavioral-treatment',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s26-13',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-13',3,ref(4),[A]),na,eq,1) :- guideline_entity(actual,A,'behavioral-treatment',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s26-13',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-13',3,ref(5),[A]),help) :- guideline_entity(actual,A,'behavioral-treatment',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s26-13',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-13',3,ref(5),[A]),1,A) :- guideline_entity(actual,A,'behavioral-treatment',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s26-13',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s26-13',3,ref(5),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s26-13',3,ref(4),[A])) :- guideline_entity(actual,A,'behavioral-treatment',countable), guideline_cardinality(actual,A,na,eq,1).
% S4: A behavioral-treatment-example includes a cognitive-behavioral-therapy-example and includes a mindfulness-practice-example.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s26-13',4,ref(1),[]),'behavioral-treatment-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s26-13',4,ref(1),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s26-13',4,ref(2),[]),'cognitive-behavioral-therapy-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s26-13',4,ref(2),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s26-13',4,ref(3),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s26-13',4,ref(3),[]),1,'$guideline_id'(product,'cdc2022-opioid-s26-13',4,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s26-13',4,ref(3),[]),2,'$guideline_id'(product,'cdc2022-opioid-s26-13',4,ref(2),[])).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s26-13',4,ref(4),[]),'mindfulness-practice-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s26-13',4,ref(4),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s26-13',4,ref(5),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s26-13',4,ref(5),[]),1,'$guideline_id'(product,'cdc2022-opioid-s26-13',4,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s26-13',4,ref(5),[]),2,'$guideline_id'(product,'cdc2022-opioid-s26-13',4,ref(4),[])).
% S5: Every active-therapy particularly outperforms a more-passive-treatment in a somewhat-more-robust-sustained-pain-and-function-improvement-evidence at a longer-term-follow-up.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s26-13',5,ref(2),[A]),'more-passive-treatment',countable) :- guideline_entity(actual,A,'active-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s26-13',5,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'active-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s26-13',5,ref(3),[A]),'somewhat-more-robust-sustained-pain-and-function-improvement-evidence',countable) :- guideline_entity(actual,A,'active-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s26-13',5,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,'active-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s26-13',5,ref(4),[A]),'longer-term-follow-up',countable) :- guideline_entity(actual,A,'active-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s26-13',5,ref(4),[A]),na,eq,1) :- guideline_entity(actual,A,'active-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_qualifier(actual,'$guideline_id'(product,'cdc2022-opioid-s26-13',5,ref(5),[A]),focus,particularly) :- guideline_entity(actual,A,'active-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s26-13',5,ref(5),[A]),outperform) :- guideline_entity(actual,A,'active-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s26-13',5,ref(5),[A]),1,A) :- guideline_entity(actual,A,'active-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s26-13',5,ref(5),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s26-13',5,ref(2),[A])) :- guideline_entity(actual,A,'active-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp(actual,'$guideline_id'(product,'cdc2022-opioid-s26-13',5,ref(5),[A]),at,'$guideline_id'(product,'cdc2022-opioid-s26-13',5,ref(4),[A])) :- guideline_entity(actual,A,'active-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp(actual,'$guideline_id'(product,'cdc2022-opioid-s26-13',5,ref(5),[A]),in,'$guideline_id'(product,'cdc2022-opioid-s26-13',5,ref(3),[A])) :- guideline_entity(actual,A,'active-therapy',countable), guideline_cardinality(actual,A,na,eq,1).
% S6: A more-passive-treatment-example is a massage-example.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s26-13',6,ref(1),[]),'more-passive-treatment-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s26-13',6,ref(1),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s26-13',6,ref(2),[]),'massage-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s26-13',6,ref(2),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s26-13',6,ref(3),[]),be).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s26-13',6,ref(3),[]),1,'$guideline_id'(product,'cdc2022-opioid-s26-13',6,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s26-13',6,ref(3),[]),2,'$guideline_id'(product,'cdc2022-opioid-s26-13',6,ref(2),[])).

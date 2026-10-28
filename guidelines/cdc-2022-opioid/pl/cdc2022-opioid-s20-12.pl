% cdc2022-opioid-s20-12.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s20-12',ace_sha256('94cc5829df88806d294de75fb4f969e0628d98132c4eed54fe133cccf225e028'),ulex(sha256('2733db8c734f4672e1d598cc0911538cb926c6072c8cc42898ec8aa2d7176379')),vocabulary(sha256('335c132fd0e3d3163ba40c255e7db41eecc36492d8098263c505b17f44bbef5a'))).
% S1: If a patient has a kidney-stone-pain and the patient does not provably have an NSAID-contraindication then every NSAID is preferred for the kidney-stone-pain.
guideline_property(actual,'$guideline_id'(product,'cdc2022-opioid-s20-12',1,ref(7),[A,B,C,D]),preferred,pos) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'kidney-stone-pain',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), \+ (guideline_entity(actual,E,'NSAID-contraindication',countable), guideline_cardinality(actual,E,na,eq,1), guideline_event(actual,F,have), guideline_arg(actual,F,1,A), guideline_arg(actual,F,2,E)), guideline_entity(actual,D,'NSAID',countable), guideline_cardinality(actual,D,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s20-12',1,ref(8),[A,B,C,D]),be) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'kidney-stone-pain',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), \+ (guideline_entity(actual,E,'NSAID-contraindication',countable), guideline_cardinality(actual,E,na,eq,1), guideline_event(actual,F,have), guideline_arg(actual,F,1,A), guideline_arg(actual,F,2,E)), guideline_entity(actual,D,'NSAID',countable), guideline_cardinality(actual,D,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s20-12',1,ref(8),[A,B,C,D]),1,D) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'kidney-stone-pain',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), \+ (guideline_entity(actual,E,'NSAID-contraindication',countable), guideline_cardinality(actual,E,na,eq,1), guideline_event(actual,F,have), guideline_arg(actual,F,1,A), guideline_arg(actual,F,2,E)), guideline_entity(actual,D,'NSAID',countable), guideline_cardinality(actual,D,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s20-12',1,ref(8),[A,B,C,D]),2,'$guideline_id'(product,'cdc2022-opioid-s20-12',1,ref(7),[A,B,C,D])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'kidney-stone-pain',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), \+ (guideline_entity(actual,E,'NSAID-contraindication',countable), guideline_cardinality(actual,E,na,eq,1), guideline_event(actual,F,have), guideline_arg(actual,F,1,A), guideline_arg(actual,F,2,E)), guideline_entity(actual,D,'NSAID',countable), guideline_cardinality(actual,D,na,eq,1).
guideline_preference(actual,'$guideline_id'(product,'cdc2022-opioid-s20-12',1,ref(8),[A,B,C,D]),D,none) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'kidney-stone-pain',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), \+ (guideline_entity(actual,E,'NSAID-contraindication',countable), guideline_cardinality(actual,E,na,eq,1), guideline_event(actual,F,have), guideline_arg(actual,F,1,A), guideline_arg(actual,F,2,E)), guideline_entity(actual,D,'NSAID',countable), guideline_cardinality(actual,D,na,eq,1).
guideline_pp(actual,'$guideline_id'(product,'cdc2022-opioid-s20-12',1,ref(8),[A,B,C,D]),for,B) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'kidney-stone-pain',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), \+ (guideline_entity(actual,E,'NSAID-contraindication',countable), guideline_cardinality(actual,E,na,eq,1), guideline_event(actual,F,have), guideline_arg(actual,F,1,A), guideline_arg(actual,F,2,E)), guideline_entity(actual,D,'NSAID',countable), guideline_cardinality(actual,D,na,eq,1).
% S2: Every NSAID shows an at-least-opioid-equal-acute-kidney-stone-pain-effectiveness.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s20-12',2,ref(2),[A]),'at-least-opioid-equal-acute-kidney-stone-pain-effectiveness',countable) :- guideline_entity(actual,A,'NSAID',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s20-12',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'NSAID',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s20-12',2,ref(3),[A]),show) :- guideline_entity(actual,A,'NSAID',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s20-12',2,ref(3),[A]),1,A) :- guideline_entity(actual,A,'NSAID',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s20-12',2,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s20-12',2,ref(2),[A])) :- guideline_entity(actual,A,'NSAID',countable), guideline_cardinality(actual,A,na,eq,1).
% S3: Every NSAID can decrease a ureteral-smooth-muscle-tone and can decrease a ureteral-spasm.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s20-12',3,box(1),[A]),can) :- guideline_entity(actual,A,'NSAID',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s20-12',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s20-12',3,ref(2),[A]),'ureteral-smooth-muscle-tone',countable) :- guideline_entity(actual,A,'NSAID',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s20-12',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s20-12',3,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'NSAID',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s20-12',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s20-12',3,ref(3),[A]),decrease) :- guideline_entity(actual,A,'NSAID',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s20-12',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s20-12',3,ref(3),[A]),1,A) :- guideline_entity(actual,A,'NSAID',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s20-12',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s20-12',3,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s20-12',3,ref(2),[A])) :- guideline_entity(actual,A,'NSAID',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s20-12',3,box(2),[A]),can) :- guideline_entity(actual,A,'NSAID',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s20-12',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s20-12',3,ref(4),[A]),'ureteral-spasm',countable) :- guideline_entity(actual,A,'NSAID',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s20-12',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s20-12',3,ref(4),[A]),na,eq,1) :- guideline_entity(actual,A,'NSAID',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s20-12',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s20-12',3,ref(5),[A]),decrease) :- guideline_entity(actual,A,'NSAID',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s20-12',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s20-12',3,ref(5),[A]),1,A) :- guideline_entity(actual,A,'NSAID',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s20-12',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s20-12',3,ref(5),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s20-12',3,ref(4),[A])) :- guideline_entity(actual,A,'NSAID',countable), guideline_cardinality(actual,A,na,eq,1).
% S4: Every ureteral-spasm can cause a kidney-stone-pain.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s20-12',4,box(1),[A]),can) :- guideline_entity(actual,A,'ureteral-spasm',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s20-12',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s20-12',4,ref(2),[A]),'kidney-stone-pain',countable) :- guideline_entity(actual,A,'ureteral-spasm',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s20-12',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s20-12',4,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'ureteral-spasm',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s20-12',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s20-12',4,ref(3),[A]),cause) :- guideline_entity(actual,A,'ureteral-spasm',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s20-12',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s20-12',4,ref(3),[A]),1,A) :- guideline_entity(actual,A,'ureteral-spasm',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s20-12',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s20-12',4,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s20-12',4,ref(2),[A])) :- guideline_entity(actual,A,'ureteral-spasm',countable), guideline_cardinality(actual,A,na,eq,1).

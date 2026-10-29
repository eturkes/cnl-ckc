% cdc2022-opioid-s34-06.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s34-06',ace_sha256('5b7d0fee3ac82be88f3f98b54e848d3ac88bd353f7026dd6231582d400a5ac31'),ulex(sha256(fba48e003101de64df33400ab2595fa7589360ac19f4c77ac3d17bd085f5b8c0)),vocabulary(sha256('335c132fd0e3d3163ba40c255e7db41eecc36492d8098263c505b17f44bbef5a'))).
% S1: If a dosage-increase is unnecessary for an opioid-dosage and there is a clinician then it is recommended that the clinician does not generally increase the opioid-dosage.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s34-06',1,box(1),[A,B,C,D,E]),should) :- guideline_entity(actual,A,'dosage-increase',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-dosage',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,D,be), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,C), guideline_property(actual,C,unnecessary,pos), guideline_pp(actual,D,for,B), guideline_entity(actual,E,clinician,countable), guideline_cardinality(actual,E,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_operator('$guideline_id'(context,'cdc2022-opioid-s34-06',1,box(1),[A,B,C,D,E]),'$guideline_id'(context,'cdc2022-opioid-s34-06',1,box(2),[A,B,C,D,E]),-) :- guideline_entity(actual,A,'dosage-increase',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-dosage',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,D,be), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,C), guideline_property(actual,C,unnecessary,pos), guideline_pp(actual,D,for,B), guideline_entity(actual,E,clinician,countable), guideline_cardinality(actual,E,na,eq,1).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-s34-06',1,box(2),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-s34-06',1,ref(6),[A,B,C,D,E]),usuality,generally) :- guideline_entity(actual,A,'dosage-increase',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-dosage',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,D,be), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,C), guideline_property(actual,C,unnecessary,pos), guideline_pp(actual,D,for,B), guideline_entity(actual,E,clinician,countable), guideline_cardinality(actual,E,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s34-06',1,box(2),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-s34-06',1,ref(6),[A,B,C,D,E]),increase) :- guideline_entity(actual,A,'dosage-increase',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-dosage',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,D,be), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,C), guideline_property(actual,C,unnecessary,pos), guideline_pp(actual,D,for,B), guideline_entity(actual,E,clinician,countable), guideline_cardinality(actual,E,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s34-06',1,box(2),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-s34-06',1,ref(6),[A,B,C,D,E]),1,E) :- guideline_entity(actual,A,'dosage-increase',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-dosage',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,D,be), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,C), guideline_property(actual,C,unnecessary,pos), guideline_pp(actual,D,for,B), guideline_entity(actual,E,clinician,countable), guideline_cardinality(actual,E,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s34-06',1,box(2),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-s34-06',1,ref(6),[A,B,C,D,E]),2,B) :- guideline_entity(actual,A,'dosage-increase',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-dosage',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,D,be), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,C), guideline_property(actual,C,unnecessary,pos), guideline_pp(actual,D,for,B), guideline_entity(actual,E,clinician,countable), guideline_cardinality(actual,E,na,eq,1).
% S2: If a clinician increases an opioid-dosage then the clinician should cautiously increase the opioid-dosage by a smallest-practical-amount.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s34-06',2,box(1),[A,B,C]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-dosage',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,increase), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s34-06',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s34-06',2,ref(4),[A,B,C]),'smallest-practical-amount',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-dosage',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,increase), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s34-06',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s34-06',2,ref(4),[A,B,C]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-dosage',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,increase), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-s34-06',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s34-06',2,ref(5),[A,B,C]),manner,cautiously) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-dosage',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,increase), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s34-06',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s34-06',2,ref(5),[A,B,C]),increase) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-dosage',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,increase), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s34-06',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s34-06',2,ref(5),[A,B,C]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-dosage',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,increase), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s34-06',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s34-06',2,ref(5),[A,B,C]),2,B) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-dosage',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,increase), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s34-06',2,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s34-06',2,ref(5),[A,B,C]),by,'$guideline_id'(product,'cdc2022-opioid-s34-06',2,ref(4),[A,B,C])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-dosage',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,increase), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
% S3: Every overdose-risk increases with a rising-opioid-dosage.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s34-06',3,ref(2),[A]),'rising-opioid-dosage',countable) :- guideline_entity(actual,A,'overdose-risk',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s34-06',3,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'overdose-risk',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s34-06',3,ref(3),[A]),increase) :- guideline_entity(actual,A,'overdose-risk',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s34-06',3,ref(3),[A]),1,A) :- guideline_entity(actual,A,'overdose-risk',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp(actual,'$guideline_id'(product,'cdc2022-opioid-s34-06',3,ref(3),[A]),with,'$guideline_id'(product,'cdc2022-opioid-s34-06',3,ref(2),[A])) :- guideline_entity(actual,A,'overdose-risk',countable), guideline_cardinality(actual,A,na,eq,1).

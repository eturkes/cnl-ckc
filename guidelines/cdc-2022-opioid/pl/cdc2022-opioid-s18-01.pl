% cdc2022-opioid-s18-01.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s18-01',ace_sha256('5942f06651b30c5e4c7b13d035c493fe039a19ae7fc97c612f249cc725d95cd5'),ulex(sha256(fba48e003101de64df33400ab2595fa7589360ac19f4c77ac3d17bd085f5b8c0)),vocabulary(sha256('335c132fd0e3d3163ba40c255e7db41eecc36492d8098263c505b17f44bbef5a'))).
% S1: If an implementation-consideration is for a recommendation then the implementation-consideration should inform a clinician-patient-decision for the recommendation.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s18-01',1,box(1),[A,B,C]),should) :- guideline_entity(actual,A,'implementation-consideration',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,recommendation,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_pp(actual,C,for,B), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s18-01',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s18-01',1,ref(4),[A,B,C]),'clinician-patient-decision',countable) :- guideline_entity(actual,A,'implementation-consideration',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,recommendation,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_pp(actual,C,for,B).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s18-01',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s18-01',1,ref(4),[A,B,C]),na,eq,1) :- guideline_entity(actual,A,'implementation-consideration',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,recommendation,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_pp(actual,C,for,B).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s18-01',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s18-01',1,ref(5),[A,B,C]),inform) :- guideline_entity(actual,A,'implementation-consideration',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,recommendation,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_pp(actual,C,for,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s18-01',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s18-01',1,ref(5),[A,B,C]),1,A) :- guideline_entity(actual,A,'implementation-consideration',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,recommendation,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_pp(actual,C,for,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s18-01',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s18-01',1,ref(5),[A,B,C]),2,'$guideline_id'(product,'cdc2022-opioid-s18-01',1,ref(4),[A,B,C])) :- guideline_entity(actual,A,'implementation-consideration',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,recommendation,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_pp(actual,C,for,B).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s18-01',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-s18-01',1,ref(5),[A,B,C]),for,B) :- guideline_entity(actual,A,'implementation-consideration',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,recommendation,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_pp(actual,C,for,B).
% S2: If there is a clinician then it is recommended that the clinician does not rigidly follow an implementation-consideration.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s18-01',2,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_operator('$guideline_id'(context,'cdc2022-opioid-s18-01',2,box(1),[A]),'$guideline_id'(context,'cdc2022-opioid-s18-01',2,box(2),[A]),-) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s18-01',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s18-01',2,ref(2),[A]),'implementation-consideration',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s18-01',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s18-01',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-s18-01',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s18-01',2,ref(3),[A]),manner,rigidly) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s18-01',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s18-01',2,ref(3),[A]),follow) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s18-01',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s18-01',2,ref(3),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s18-01',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s18-01',2,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s18-01',2,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S3: If there is a clinician then it is recommended that the clinician does not inflexibly follow an implementation-consideration.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s18-01',3,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_operator('$guideline_id'(context,'cdc2022-opioid-s18-01',3,box(1),[A]),'$guideline_id'(context,'cdc2022-opioid-s18-01',3,box(2),[A]),-) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s18-01',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s18-01',3,ref(2),[A]),'implementation-consideration',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s18-01',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s18-01',3,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-s18-01',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s18-01',3,ref(3),[A]),manner,inflexibly) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s18-01',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s18-01',3,ref(3),[A]),follow) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s18-01',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s18-01',3,ref(3),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s18-01',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s18-01',3,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s18-01',3,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S4: Every implementation-consideration offers a practical-insight and offers a recommendation-context and offers a specific-example.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s18-01',4,ref(2),[A]),'practical-insight',countable) :- guideline_entity(actual,A,'implementation-consideration',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s18-01',4,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'implementation-consideration',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s18-01',4,ref(3),[A]),offer) :- guideline_entity(actual,A,'implementation-consideration',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s18-01',4,ref(3),[A]),1,A) :- guideline_entity(actual,A,'implementation-consideration',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s18-01',4,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s18-01',4,ref(2),[A])) :- guideline_entity(actual,A,'implementation-consideration',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s18-01',4,ref(4),[A]),'recommendation-context',countable) :- guideline_entity(actual,A,'implementation-consideration',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s18-01',4,ref(4),[A]),na,eq,1) :- guideline_entity(actual,A,'implementation-consideration',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s18-01',4,ref(5),[A]),offer) :- guideline_entity(actual,A,'implementation-consideration',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s18-01',4,ref(5),[A]),1,A) :- guideline_entity(actual,A,'implementation-consideration',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s18-01',4,ref(5),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s18-01',4,ref(4),[A])) :- guideline_entity(actual,A,'implementation-consideration',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s18-01',4,ref(6),[A]),'specific-example',countable) :- guideline_entity(actual,A,'implementation-consideration',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s18-01',4,ref(6),[A]),na,eq,1) :- guideline_entity(actual,A,'implementation-consideration',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s18-01',4,ref(7),[A]),offer) :- guideline_entity(actual,A,'implementation-consideration',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s18-01',4,ref(7),[A]),1,A) :- guideline_entity(actual,A,'implementation-consideration',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s18-01',4,ref(7),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s18-01',4,ref(6),[A])) :- guideline_entity(actual,A,'implementation-consideration',countable), guideline_cardinality(actual,A,na,eq,1).

% cdc2022-opioid-rec10-imp12.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-rec10-imp12',ace_sha256('7a9e86c1838503a5b76f393f6684f67e402d746e2cb7a08bd1b0cb656f475709'),ulex(sha256(fba48e003101de64df33400ab2595fa7589360ac19f4c77ac3d17bd085f5b8c0)),vocabulary(sha256('335c132fd0e3d3163ba40c255e7db41eecc36492d8098263c505b17f44bbef5a'))).
% S1: Every clinician should nonjudgmentally discuss an unexpected-toxicology-result with a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec10-imp12',1,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec10-imp12',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp12',1,ref(2),[A]),'unexpected-toxicology-result',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec10-imp12',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp12',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec10-imp12',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp12',1,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec10-imp12',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp12',1,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-rec10-imp12',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp12',1,ref(4),[A]),manner,nonjudgmentally) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec10-imp12',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp12',1,ref(4),[A]),discuss) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec10-imp12',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp12',1,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec10-imp12',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp12',1,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec10-imp12',1,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec10-imp12',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp12',1,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-rec10-imp12',1,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S2: If there is a clinician then it is recommended that the clinician does not use a potentially-stigmatizing-language during a nonjudgmental-unexpected-result-discussion.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec10-imp12',2,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_operator('$guideline_id'(context,'cdc2022-opioid-rec10-imp12',2,box(1),[A]),'$guideline_id'(context,'cdc2022-opioid-rec10-imp12',2,box(2),[A]),-) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec10-imp12',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp12',2,ref(2),[A]),'potentially-stigmatizing-language',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec10-imp12',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp12',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec10-imp12',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp12',2,ref(3),[A]),'nonjudgmental-unexpected-result-discussion',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec10-imp12',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp12',2,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec10-imp12',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp12',2,ref(4),[A]),use) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec10-imp12',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp12',2,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec10-imp12',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp12',2,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec10-imp12',2,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec10-imp12',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec10-imp12',2,ref(4),[A]),during,'$guideline_id'(product,'cdc2022-opioid-rec10-imp12',2,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S3: A potentially-stigmatizing-language-example includes a clean-specimen-description-example and includes a dirty-specimen-description-example.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec10-imp12',3,ref(1),[]),'potentially-stigmatizing-language-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec10-imp12',3,ref(1),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec10-imp12',3,ref(2),[]),'clean-specimen-description-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec10-imp12',3,ref(2),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-rec10-imp12',3,ref(3),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec10-imp12',3,ref(3),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec10-imp12',3,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec10-imp12',3,ref(3),[]),2,'$guideline_id'(product,'cdc2022-opioid-rec10-imp12',3,ref(2),[])).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec10-imp12',3,ref(4),[]),'dirty-specimen-description-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec10-imp12',3,ref(4),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-rec10-imp12',3,ref(5),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec10-imp12',3,ref(5),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec10-imp12',3,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec10-imp12',3,ref(5),[]),2,'$guideline_id'(product,'cdc2022-opioid-rec10-imp12',3,ref(4),[])).

% cdc2022-opioid-s37-15.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
:- multifile(guideline_schema_version/1).
:- discontiguous(guideline_schema_version/1).
:- multifile(guideline_document/3).
:- discontiguous(guideline_document/3).
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
guideline_schema_version(1).
guideline_document('cdc2022-opioid-s37-15',ace_sha256(ae70c7cdc676ee26df8d9041885914632dfb31a86d484a7e4da8b81ab5df2856),ulex(sha256(ef57ba7180864095bab88b95f11ddb1a2c858593eba715dbfbe874db09506c5b))).
% S1: Every clinician should offer a continued-opioid-use-reevaluation-opportunity to a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s37-15',1,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s37-15',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-15',1,ref(2),[A]),'continued-opioid-use-reevaluation-opportunity',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s37-15',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-15',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s37-15',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-15',1,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s37-15',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-15',1,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s37-15',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-15',1,ref(4),[A]),offer) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s37-15',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-15',1,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s37-15',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-15',1,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s37-15',1,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s37-15',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s37-15',1,ref(4),[A]),3,'$guideline_id'(product,'cdc2022-opioid-s37-15',1,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).

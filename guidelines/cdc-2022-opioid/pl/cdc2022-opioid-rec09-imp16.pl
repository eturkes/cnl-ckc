% cdc2022-opioid-rec09-imp16.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
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
guideline_document('cdc2022-opioid-rec09-imp16',ace_sha256(e309907420fb0ec0e138ee5dd7ce6ba37037df098d0f56cc4ef8644d292a2186),ulex(sha256('4c55bed7525c11e80d870254a4f876c23b6aa3c2a077242833ec32997784b01c'))).
% S1: If a clinician conducts an MME-dosage-determination then the clinician should not include a buprenorphine in the MME-dosage-determination.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec09-imp16',1,box(1),[A,B,C]),-) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'MME-dosage-determination',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,conduct), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_operator('$guideline_id'(context,'cdc2022-opioid-rec09-imp16',1,box(1),[A,B,C]),'$guideline_id'(context,'cdc2022-opioid-rec09-imp16',1,box(2),[A,B,C]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'MME-dosage-determination',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,conduct), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec09-imp16',1,box(2),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec09-imp16',1,ref(4),[A,B,C]),buprenorphine,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'MME-dosage-determination',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,conduct), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec09-imp16',1,box(2),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec09-imp16',1,ref(4),[A,B,C]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'MME-dosage-determination',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,conduct), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec09-imp16',1,box(2),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec09-imp16',1,ref(5),[A,B,C]),include) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'MME-dosage-determination',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,conduct), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec09-imp16',1,box(2),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec09-imp16',1,ref(5),[A,B,C]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'MME-dosage-determination',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,conduct), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec09-imp16',1,box(2),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec09-imp16',1,ref(5),[A,B,C]),2,'$guideline_id'(product,'cdc2022-opioid-rec09-imp16',1,ref(4),[A,B,C])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'MME-dosage-determination',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,conduct), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec09-imp16',1,box(2),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec09-imp16',1,ref(5),[A,B,C]),in,B) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'MME-dosage-determination',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,conduct), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
% S2: Every buprenorphine has a partial-opioid-receptor-agonist-property.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec09-imp16',2,ref(2),[A]),'partial-opioid-receptor-agonist-property',countable) :- guideline_entity(actual,A,buprenorphine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec09-imp16',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,buprenorphine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-rec09-imp16',2,ref(3),[A]),have) :- guideline_entity(actual,A,buprenorphine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec09-imp16',2,ref(3),[A]),1,A) :- guideline_entity(actual,A,buprenorphine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec09-imp16',2,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec09-imp16',2,ref(2),[A])) :- guideline_entity(actual,A,buprenorphine,countable), guideline_cardinality(actual,A,na,eq,1).
% S3: Every buprenorphine has a respiratory-depression-ceiling-effect.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec09-imp16',3,ref(2),[A]),'respiratory-depression-ceiling-effect',countable) :- guideline_entity(actual,A,buprenorphine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec09-imp16',3,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,buprenorphine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-rec09-imp16',3,ref(3),[A]),have) :- guideline_entity(actual,A,buprenorphine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec09-imp16',3,ref(3),[A]),1,A) :- guideline_entity(actual,A,buprenorphine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec09-imp16',3,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec09-imp16',3,ref(2),[A])) :- guideline_entity(actual,A,buprenorphine,countable), guideline_cardinality(actual,A,na,eq,1).

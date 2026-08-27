% cdc2022-opioid-s60-05.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
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
guideline_document('cdc2022-opioid-s60-05',ace_sha256(db125d9fe86b874b5d28cf45948c654b3dd01a5dffa5a2ec3ad64cc7d9bb53df),ulex(sha256('4a882a1996ea910df0fbf91fea897c0805e67d48f9e1ce2fcdbbbd9250c193ae'))).
% S1: Every clinician should provide an opioid-use-disorder-specialist-search-assistance to a patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s60-05',1,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s60-05',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s60-05',1,ref(2),[A]),'opioid-use-disorder-specialist-search-assistance',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s60-05',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s60-05',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s60-05',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s60-05',1,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s60-05',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s60-05',1,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s60-05',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s60-05',1,ref(4),[A]),provide) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s60-05',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s60-05',1,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s60-05',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s60-05',1,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s60-05',1,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s60-05',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s60-05',1,ref(4),[A]),3,'$guideline_id'(product,'cdc2022-opioid-s60-05',1,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S2: Every clinician should arrange a specialist-follow-up for a patient with a qualified-opioid-use-disorder-treatment-specialist.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s60-05',2,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s60-05',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s60-05',2,ref(2),[A]),'specialist-follow-up',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s60-05',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s60-05',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s60-05',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s60-05',2,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s60-05',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s60-05',2,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s60-05',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s60-05',2,ref(4),[A]),'qualified-opioid-use-disorder-treatment-specialist',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s60-05',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s60-05',2,ref(4),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s60-05',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s60-05',2,ref(5),[A]),arrange) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s60-05',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s60-05',2,ref(5),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s60-05',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s60-05',2,ref(5),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s60-05',2,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s60-05',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s60-05',2,ref(5),[A]),with,'$guideline_id'(product,'cdc2022-opioid-s60-05',2,ref(4),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s60-05',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s60-05',2,ref(5),[A]),for,'$guideline_id'(product,'cdc2022-opioid-s60-05',2,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S3: Every clinician should coordinate a continuing-care for a patient with a qualified-opioid-use-disorder-treatment-specialist.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s60-05',3,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s60-05',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s60-05',3,ref(2),[A]),'continuing-care',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s60-05',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s60-05',3,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s60-05',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s60-05',3,ref(3),[A]),patient,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s60-05',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s60-05',3,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s60-05',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s60-05',3,ref(4),[A]),'qualified-opioid-use-disorder-treatment-specialist',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s60-05',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s60-05',3,ref(4),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s60-05',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s60-05',3,ref(5),[A]),coordinate) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s60-05',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s60-05',3,ref(5),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s60-05',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s60-05',3,ref(5),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s60-05',3,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s60-05',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s60-05',3,ref(5),[A]),with,'$guideline_id'(product,'cdc2022-opioid-s60-05',3,ref(4),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s60-05',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s60-05',3,ref(5),[A]),for,'$guideline_id'(product,'cdc2022-opioid-s60-05',3,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).

% cdc2022-opioid-s29-02.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
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
guideline_document('cdc2022-opioid-s29-02',ace_sha256(c811ddd72ff48ca4073935f06bbee4feb60c3a659aa4b1d3232c33bde8399ff2),ulex(sha256('2a4a3e829c919b3dd3b20356e7a69585cfb305c3c57dfb531e3f2c1eb97637b6'))).
% S1: Every clinician should review a care-access-consideration before an opioid-initiation.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s29-02',1,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s29-02',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-02',1,ref(2),[A]),'care-access-consideration',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s29-02',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-02',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s29-02',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-02',1,ref(3),[A]),'opioid-initiation',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s29-02',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-02',1,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s29-02',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-02',1,ref(4),[A]),review) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s29-02',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-02',1,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s29-02',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-02',1,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s29-02',1,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s29-02',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-02',1,ref(4),[A]),before,'$guideline_id'(product,'cdc2022-opioid-s29-02',1,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S2: Every clinician should review a care-access-consideration before an opioid-continuation.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s29-02',2,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s29-02',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-02',2,ref(2),[A]),'care-access-consideration',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s29-02',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-02',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s29-02',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-02',2,ref(3),[A]),'opioid-continuation',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s29-02',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-02',2,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s29-02',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-02',2,ref(4),[A]),review) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s29-02',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-02',2,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s29-02',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-02',2,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s29-02',2,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s29-02',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-02',2,ref(4),[A]),before,'$guideline_id'(product,'cdc2022-opioid-s29-02',2,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S3: Every clinician should review a care-access-consideration before a pain-treatment-initiation.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s29-02',3,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s29-02',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-02',3,ref(2),[A]),'care-access-consideration',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s29-02',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-02',3,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s29-02',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-02',3,ref(3),[A]),'pain-treatment-initiation',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s29-02',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-02',3,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s29-02',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-02',3,ref(4),[A]),review) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s29-02',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-02',3,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s29-02',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-02',3,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s29-02',3,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s29-02',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-02',3,ref(4),[A]),before,'$guideline_id'(product,'cdc2022-opioid-s29-02',3,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S4: Every clinician should review a care-access-consideration before a pain-treatment-continuation.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s29-02',4,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s29-02',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-02',4,ref(2),[A]),'care-access-consideration',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s29-02',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-02',4,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s29-02',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-02',4,ref(3),[A]),'pain-treatment-continuation',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s29-02',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-02',4,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s29-02',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-02',4,ref(4),[A]),review) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s29-02',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-02',4,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s29-02',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-02',4,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s29-02',4,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s29-02',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s29-02',4,ref(4),[A]),before,'$guideline_id'(product,'cdc2022-opioid-s29-02',4,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).

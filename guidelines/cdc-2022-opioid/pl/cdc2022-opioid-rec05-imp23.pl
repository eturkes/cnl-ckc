% cdc2022-opioid-rec05-imp23.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
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
guideline_document('cdc2022-opioid-rec05-imp23',ace_sha256('4ee680699637e85007499508ec9181afcb9d44cf68aaeeb2345974d0b5cb73cc'),ulex(sha256('2a4a3e829c919b3dd3b20356e7a69585cfb305c3c57dfb531e3f2c1eb97637b6'))).
% S1: Every clinician can use a periodic-and-strategic-motivational-question and can use a periodic-and-strategic-motivational-statement.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec05-imp23',1,box(1),[A]),can) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp23',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp23',1,ref(2),[A]),'periodic-and-strategic-motivational-question',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp23',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp23',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec05-imp23',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp23',1,ref(3),[A]),use) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp23',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp23',1,ref(3),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp23',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp23',1,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp23',1,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec05-imp23',1,box(2),[A]),can) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp23',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp23',1,ref(4),[A]),'periodic-and-strategic-motivational-statement',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp23',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp23',1,ref(4),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec05-imp23',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp23',1,ref(5),[A]),use) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp23',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp23',1,ref(5),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp23',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp23',1,ref(5),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp23',1,ref(4),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S2: A periodic-and-strategic-motivational-question can encourage an appropriate-therapeutic-change and can encourage a functional-goal.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp23',2,ref(1),[]),'periodic-and-strategic-motivational-question',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp23',2,ref(1),[]),na,eq,1).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec05-imp23',2,box(1),[]),can).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp23',2,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp23',2,ref(2),[]),'appropriate-therapeutic-change',countable).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp23',2,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp23',2,ref(2),[]),na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec05-imp23',2,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp23',2,ref(3),[]),encourage).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp23',2,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp23',2,ref(3),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec05-imp23',2,ref(1),[])).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp23',2,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp23',2,ref(3),[]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp23',2,ref(2),[])).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec05-imp23',2,box(2),[]),can).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp23',2,box(2),[]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp23',2,ref(4),[]),'functional-goal',countable).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp23',2,box(2),[]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp23',2,ref(4),[]),na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec05-imp23',2,box(2),[]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp23',2,ref(5),[]),encourage).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp23',2,box(2),[]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp23',2,ref(5),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec05-imp23',2,ref(1),[])).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp23',2,box(2),[]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp23',2,ref(5),[]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp23',2,ref(4),[])).
% S3: A periodic-and-strategic-motivational-statement can encourage an appropriate-therapeutic-change and can encourage a functional-goal.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp23',3,ref(1),[]),'periodic-and-strategic-motivational-statement',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp23',3,ref(1),[]),na,eq,1).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec05-imp23',3,box(1),[]),can).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp23',3,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp23',3,ref(2),[]),'appropriate-therapeutic-change',countable).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp23',3,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp23',3,ref(2),[]),na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec05-imp23',3,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp23',3,ref(3),[]),encourage).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp23',3,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp23',3,ref(3),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec05-imp23',3,ref(1),[])).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp23',3,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp23',3,ref(3),[]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp23',3,ref(2),[])).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec05-imp23',3,box(2),[]),can).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec05-imp23',3,box(2),[]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp23',3,ref(4),[]),'functional-goal',countable).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec05-imp23',3,box(2),[]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp23',3,ref(4),[]),na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec05-imp23',3,box(2),[]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp23',3,ref(5),[]),encourage).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp23',3,box(2),[]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp23',3,ref(5),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec05-imp23',3,ref(1),[])).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec05-imp23',3,box(2),[]),'$guideline_id'(product,'cdc2022-opioid-rec05-imp23',3,ref(5),[]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp23',3,ref(4),[])).

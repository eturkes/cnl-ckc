% cdc2022-opioid-rec03-imp01.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
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
guideline_document('cdc2022-opioid-rec03-imp01',ace_sha256(fc567a2593eb1936b662fd028bf11e1f4e173351c408461e49e62b5ae704c9c1),ulex(sha256('4c55bed7525c11e80d870254a4f876c23b6aa3c2a077242833ec32997784b01c'))).
% S1: Every clinician should not treat an acute-pain with an ER-LA-opioid.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec03-imp01',1,box(1),[A]),-) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator('$guideline_id'(context,'cdc2022-opioid-rec03-imp01',1,box(1),[A]),'$guideline_id'(context,'cdc2022-opioid-rec03-imp01',1,box(2),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec03-imp01',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp01',1,ref(2),[A]),'acute-pain',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec03-imp01',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp01',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec03-imp01',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp01',1,ref(3),[A]),'ER-LA-opioid',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec03-imp01',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp01',1,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec03-imp01',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp01',1,ref(4),[A]),treat) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec03-imp01',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp01',1,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec03-imp01',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp01',1,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec03-imp01',1,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec03-imp01',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp01',1,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-rec03-imp01',1,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S2: Every clinician should not initiate an ER-LA-opioid for a subacute-pain.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec03-imp01',2,box(1),[A]),-) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator('$guideline_id'(context,'cdc2022-opioid-rec03-imp01',2,box(1),[A]),'$guideline_id'(context,'cdc2022-opioid-rec03-imp01',2,box(2),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec03-imp01',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp01',2,ref(2),[A]),'ER-LA-opioid',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec03-imp01',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp01',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec03-imp01',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp01',2,ref(3),[A]),'subacute-pain',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec03-imp01',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp01',2,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec03-imp01',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp01',2,ref(4),[A]),initiate) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec03-imp01',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp01',2,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec03-imp01',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp01',2,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec03-imp01',2,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec03-imp01',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp01',2,ref(4),[A]),for,'$guideline_id'(product,'cdc2022-opioid-rec03-imp01',2,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S3: Every clinician should not initiate an ER-LA-opioid for a chronic-pain.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec03-imp01',3,box(1),[A]),-) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator('$guideline_id'(context,'cdc2022-opioid-rec03-imp01',3,box(1),[A]),'$guideline_id'(context,'cdc2022-opioid-rec03-imp01',3,box(2),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec03-imp01',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp01',3,ref(2),[A]),'ER-LA-opioid',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec03-imp01',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp01',3,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec03-imp01',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp01',3,ref(3),[A]),'chronic-pain',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec03-imp01',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp01',3,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec03-imp01',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp01',3,ref(4),[A]),initiate) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec03-imp01',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp01',3,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec03-imp01',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp01',3,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec03-imp01',3,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec03-imp01',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp01',3,ref(4),[A]),for,'$guideline_id'(product,'cdc2022-opioid-rec03-imp01',3,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S4: Every clinician should not prescribe an ER-LA-opioid for an intermittent-or-as-needed-use.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec03-imp01',4,box(1),[A]),-) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator('$guideline_id'(context,'cdc2022-opioid-rec03-imp01',4,box(1),[A]),'$guideline_id'(context,'cdc2022-opioid-rec03-imp01',4,box(2),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec03-imp01',4,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp01',4,ref(2),[A]),'ER-LA-opioid',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec03-imp01',4,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp01',4,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec03-imp01',4,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp01',4,ref(3),[A]),'intermittent-or-as-needed-use',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec03-imp01',4,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp01',4,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec03-imp01',4,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp01',4,ref(4),[A]),prescribe) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec03-imp01',4,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp01',4,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec03-imp01',4,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp01',4,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec03-imp01',4,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec03-imp01',4,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp01',4,ref(4),[A]),for,'$guideline_id'(product,'cdc2022-opioid-rec03-imp01',4,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).

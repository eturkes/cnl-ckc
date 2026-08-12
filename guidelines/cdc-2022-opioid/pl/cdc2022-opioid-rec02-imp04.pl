% cdc2022-opioid-rec02-imp04.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
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
guideline_document('cdc2022-opioid-rec02-imp04',ace_sha256(b1dcac5936c91963eb5d0eee6083bc55ecd456c51aadf8392b896a2b6ae7db40),ulex(sha256('9d39595833bb1358c343511806091b34e2442561f073b432f531f68ac385a15e'))).
% S1: Every health-insurer can increase a therapy-reimbursement and can increase an effective-nonpharmacologic-therapy-access.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec02-imp04',1,box(1),[A]),can) :- guideline_entity(actual,A,'health-insurer',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec02-imp04',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp04',1,ref(2),[A]),'therapy-reimbursement',countable) :- guideline_entity(actual,A,'health-insurer',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec02-imp04',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp04',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'health-insurer',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec02-imp04',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp04',1,ref(3),[A]),increase) :- guideline_entity(actual,A,'health-insurer',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec02-imp04',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp04',1,ref(3),[A]),1,A) :- guideline_entity(actual,A,'health-insurer',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec02-imp04',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp04',1,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec02-imp04',1,ref(2),[A])) :- guideline_entity(actual,A,'health-insurer',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec02-imp04',1,box(2),[A]),can) :- guideline_entity(actual,A,'health-insurer',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec02-imp04',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp04',1,ref(4),[A]),'effective-nonpharmacologic-therapy-access',countable) :- guideline_entity(actual,A,'health-insurer',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec02-imp04',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp04',1,ref(4),[A]),na,eq,1) :- guideline_entity(actual,A,'health-insurer',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec02-imp04',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp04',1,ref(5),[A]),increase) :- guideline_entity(actual,A,'health-insurer',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec02-imp04',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp04',1,ref(5),[A]),1,A) :- guideline_entity(actual,A,'health-insurer',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec02-imp04',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp04',1,ref(5),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec02-imp04',1,ref(4),[A])) :- guideline_entity(actual,A,'health-insurer',countable), guideline_cardinality(actual,A,na,eq,1).
% S2: Every health-system can increase a therapy-reimbursement and can increase an effective-nonpharmacologic-therapy-access.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec02-imp04',2,box(1),[A]),can) :- guideline_entity(actual,A,'health-system',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec02-imp04',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp04',2,ref(2),[A]),'therapy-reimbursement',countable) :- guideline_entity(actual,A,'health-system',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec02-imp04',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp04',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'health-system',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec02-imp04',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp04',2,ref(3),[A]),increase) :- guideline_entity(actual,A,'health-system',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec02-imp04',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp04',2,ref(3),[A]),1,A) :- guideline_entity(actual,A,'health-system',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec02-imp04',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp04',2,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec02-imp04',2,ref(2),[A])) :- guideline_entity(actual,A,'health-system',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec02-imp04',2,box(2),[A]),can) :- guideline_entity(actual,A,'health-system',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec02-imp04',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp04',2,ref(4),[A]),'effective-nonpharmacologic-therapy-access',countable) :- guideline_entity(actual,A,'health-system',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec02-imp04',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp04',2,ref(4),[A]),na,eq,1) :- guideline_entity(actual,A,'health-system',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec02-imp04',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp04',2,ref(5),[A]),increase) :- guideline_entity(actual,A,'health-system',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec02-imp04',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp04',2,ref(5),[A]),1,A) :- guideline_entity(actual,A,'health-system',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec02-imp04',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp04',2,ref(5),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec02-imp04',2,ref(4),[A])) :- guideline_entity(actual,A,'health-system',countable), guideline_cardinality(actual,A,na,eq,1).
% S3: An increased-therapy-access can improve a pain-management and can reduce a medication-use and can reduce an associated-risk.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec02-imp04',3,ref(1),[]),'increased-therapy-access',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec02-imp04',3,ref(1),[]),na,eq,1).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec02-imp04',3,box(1),[]),can).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec02-imp04',3,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp04',3,ref(2),[]),'pain-management',countable).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec02-imp04',3,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp04',3,ref(2),[]),na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec02-imp04',3,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp04',3,ref(3),[]),improve).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec02-imp04',3,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp04',3,ref(3),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec02-imp04',3,ref(1),[])).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec02-imp04',3,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp04',3,ref(3),[]),2,'$guideline_id'(product,'cdc2022-opioid-rec02-imp04',3,ref(2),[])).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec02-imp04',3,box(2),[]),can).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec02-imp04',3,box(2),[]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp04',3,ref(4),[]),'medication-use',countable).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec02-imp04',3,box(2),[]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp04',3,ref(4),[]),na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec02-imp04',3,box(2),[]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp04',3,ref(5),[]),reduce).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec02-imp04',3,box(2),[]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp04',3,ref(5),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec02-imp04',3,ref(1),[])).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec02-imp04',3,box(2),[]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp04',3,ref(5),[]),2,'$guideline_id'(product,'cdc2022-opioid-rec02-imp04',3,ref(4),[])).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec02-imp04',3,box(3),[]),can).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec02-imp04',3,box(3),[]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp04',3,ref(6),[]),'associated-risk',countable).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec02-imp04',3,box(3),[]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp04',3,ref(6),[]),na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec02-imp04',3,box(3),[]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp04',3,ref(7),[]),reduce).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec02-imp04',3,box(3),[]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp04',3,ref(7),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec02-imp04',3,ref(1),[])).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec02-imp04',3,box(3),[]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp04',3,ref(7),[]),2,'$guideline_id'(product,'cdc2022-opioid-rec02-imp04',3,ref(6),[])).

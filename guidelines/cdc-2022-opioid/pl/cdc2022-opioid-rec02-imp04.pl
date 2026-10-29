% cdc2022-opioid-rec02-imp04.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-rec02-imp04',ace_sha256('6c532e029794ff6ed6551e4fec94b173c2c56da2c4449f9912bba545a5536598'),ulex(sha256(fba48e003101de64df33400ab2595fa7589360ac19f4c77ac3d17bd085f5b8c0)),vocabulary(sha256('335c132fd0e3d3163ba40c255e7db41eecc36492d8098263c505b17f44bbef5a'))).
% S1: Every health-insurer can increase a therapy-reimbursement and can increase an evidence-based-noninvasive-nonpharmacologic-therapy-access.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec02-imp04',1,box(1),[A]),can) :- guideline_entity(actual,A,'health-insurer',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec02-imp04',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp04',1,ref(2),[A]),'therapy-reimbursement',countable) :- guideline_entity(actual,A,'health-insurer',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec02-imp04',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp04',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'health-insurer',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec02-imp04',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp04',1,ref(3),[A]),increase) :- guideline_entity(actual,A,'health-insurer',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec02-imp04',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp04',1,ref(3),[A]),1,A) :- guideline_entity(actual,A,'health-insurer',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec02-imp04',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp04',1,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec02-imp04',1,ref(2),[A])) :- guideline_entity(actual,A,'health-insurer',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec02-imp04',1,box(2),[A]),can) :- guideline_entity(actual,A,'health-insurer',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec02-imp04',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp04',1,ref(4),[A]),'evidence-based-noninvasive-nonpharmacologic-therapy-access',countable) :- guideline_entity(actual,A,'health-insurer',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec02-imp04',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp04',1,ref(4),[A]),na,eq,1) :- guideline_entity(actual,A,'health-insurer',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec02-imp04',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp04',1,ref(5),[A]),increase) :- guideline_entity(actual,A,'health-insurer',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec02-imp04',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp04',1,ref(5),[A]),1,A) :- guideline_entity(actual,A,'health-insurer',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec02-imp04',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp04',1,ref(5),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec02-imp04',1,ref(4),[A])) :- guideline_entity(actual,A,'health-insurer',countable), guideline_cardinality(actual,A,na,eq,1).
% S2: Every health-system can increase a therapy-reimbursement and can increase an evidence-based-noninvasive-nonpharmacologic-therapy-access.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec02-imp04',2,box(1),[A]),can) :- guideline_entity(actual,A,'health-system',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec02-imp04',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp04',2,ref(2),[A]),'therapy-reimbursement',countable) :- guideline_entity(actual,A,'health-system',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec02-imp04',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp04',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'health-system',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec02-imp04',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp04',2,ref(3),[A]),increase) :- guideline_entity(actual,A,'health-system',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec02-imp04',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp04',2,ref(3),[A]),1,A) :- guideline_entity(actual,A,'health-system',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec02-imp04',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp04',2,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec02-imp04',2,ref(2),[A])) :- guideline_entity(actual,A,'health-system',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec02-imp04',2,box(2),[A]),can) :- guideline_entity(actual,A,'health-system',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec02-imp04',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec02-imp04',2,ref(4),[A]),'evidence-based-noninvasive-nonpharmacologic-therapy-access',countable) :- guideline_entity(actual,A,'health-system',countable), guideline_cardinality(actual,A,na,eq,1).
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

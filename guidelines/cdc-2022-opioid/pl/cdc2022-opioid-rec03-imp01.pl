% cdc2022-opioid-rec03-imp01.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_schema_version(1).
guideline_document('cdc2022-opioid-rec03-imp01',ace_sha256(d72671ffd1d673109e8abd15467cf5175fc9ed63f23a59dea6baa079f4826f08),ulex(sha256(ebe2d3db8559958f9ed44ec7a76cb2fb6ea75017f9f9437c3ea03b07092b0e75)),vocabulary(sha256('9d7632637554f8cbfe72e7d34a9b9d14c29eb8593aebb1c128a870c5dcf77ead'))).
% S1: If there is a clinician then it is recommended that the clinician does not treat an acute-pain with an ER-LA-opioid.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec03-imp01',1,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator('$guideline_id'(context,'cdc2022-opioid-rec03-imp01',1,box(1),[A]),'$guideline_id'(context,'cdc2022-opioid-rec03-imp01',1,box(2),[A]),-) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec03-imp01',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp01',1,ref(2),[A]),'acute-pain',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec03-imp01',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp01',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec03-imp01',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp01',1,ref(3),[A]),'ER-LA-opioid',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec03-imp01',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp01',1,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec03-imp01',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp01',1,ref(4),[A]),treat) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec03-imp01',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp01',1,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec03-imp01',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp01',1,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec03-imp01',1,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec03-imp01',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp01',1,ref(4),[A]),with,'$guideline_id'(product,'cdc2022-opioid-rec03-imp01',1,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S2: If there is a clinician then it is recommended that the clinician does not initiate an ER-LA-opioid for a subacute-pain.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec03-imp01',2,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator('$guideline_id'(context,'cdc2022-opioid-rec03-imp01',2,box(1),[A]),'$guideline_id'(context,'cdc2022-opioid-rec03-imp01',2,box(2),[A]),-) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec03-imp01',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp01',2,ref(2),[A]),'ER-LA-opioid',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec03-imp01',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp01',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec03-imp01',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp01',2,ref(3),[A]),'subacute-pain',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec03-imp01',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp01',2,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec03-imp01',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp01',2,ref(4),[A]),initiate) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec03-imp01',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp01',2,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec03-imp01',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp01',2,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec03-imp01',2,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec03-imp01',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp01',2,ref(4),[A]),for,'$guideline_id'(product,'cdc2022-opioid-rec03-imp01',2,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S3: If there is a clinician then it is recommended that the clinician does not initiate an ER-LA-opioid for a chronic-pain.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec03-imp01',3,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator('$guideline_id'(context,'cdc2022-opioid-rec03-imp01',3,box(1),[A]),'$guideline_id'(context,'cdc2022-opioid-rec03-imp01',3,box(2),[A]),-) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec03-imp01',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp01',3,ref(2),[A]),'ER-LA-opioid',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec03-imp01',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp01',3,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec03-imp01',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp01',3,ref(3),[A]),'chronic-pain',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec03-imp01',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp01',3,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec03-imp01',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp01',3,ref(4),[A]),initiate) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec03-imp01',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp01',3,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec03-imp01',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp01',3,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec03-imp01',3,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec03-imp01',3,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp01',3,ref(4),[A]),for,'$guideline_id'(product,'cdc2022-opioid-rec03-imp01',3,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
% S4: If there is a clinician then it is recommended that the clinician does not prescribe an ER-LA-opioid for an intermittent-or-as-needed-use.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec03-imp01',4,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator('$guideline_id'(context,'cdc2022-opioid-rec03-imp01',4,box(1),[A]),'$guideline_id'(context,'cdc2022-opioid-rec03-imp01',4,box(2),[A]),-) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec03-imp01',4,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp01',4,ref(2),[A]),'ER-LA-opioid',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec03-imp01',4,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp01',4,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec03-imp01',4,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp01',4,ref(3),[A]),'intermittent-or-as-needed-use',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec03-imp01',4,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp01',4,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec03-imp01',4,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp01',4,ref(4),[A]),prescribe) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec03-imp01',4,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp01',4,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec03-imp01',4,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp01',4,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-rec03-imp01',4,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec03-imp01',4,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec03-imp01',4,ref(4),[A]),for,'$guideline_id'(product,'cdc2022-opioid-rec03-imp01',4,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).

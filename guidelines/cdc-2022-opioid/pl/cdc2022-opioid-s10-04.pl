% cdc2022-opioid-s10-04.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s10-04',ace_sha256('8c201ba7fe3cbe80c881dd4ed490d0f1991ebf42690b91ba44344210d67286f1'),ulex(sha256('3e37dcfbb36dbdd0830d644235c88914b28def633a7ac43d2b5bb53faf8ba507')),vocabulary(sha256('9d7632637554f8cbfe72e7d34a9b9d14c29eb8593aebb1c128a870c5dcf77ead'))).
% S1: If a patient has an age and the age includes less than 18 years and the patient has an opioid-pain-medication-use then every recommendation does not address the opioid-pain-medication-use.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s10-04',1,box(1),[A,B,C,D,E,F,G,H]),-) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,age,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_event(actual,E,include), guideline_arg(actual,E,1,B), guideline_arg(actual,E,2,D), guideline_cardinality(actual,D,na,I,J), guideline_unit(actual,D,K), guideline_satisfies(I,J,K,less,18,year), guideline_entity(actual,F,'opioid-pain-medication-use',countable), guideline_cardinality(actual,F,na,eq,1), guideline_event(actual,G,have), guideline_arg(actual,G,1,A), guideline_arg(actual,G,2,F), guideline_entity(actual,H,recommendation,countable), guideline_cardinality(actual,H,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s10-04',1,box(1),[A,B,C,D,E,F,G,H]),'$guideline_id'(product,'cdc2022-opioid-s10-04',1,ref(9),[A,B,C,D,E,F,G,H]),address) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,age,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_event(actual,E,include), guideline_arg(actual,E,1,B), guideline_arg(actual,E,2,D), guideline_cardinality(actual,D,na,I,J), guideline_unit(actual,D,K), guideline_satisfies(I,J,K,less,18,year), guideline_entity(actual,F,'opioid-pain-medication-use',countable), guideline_cardinality(actual,F,na,eq,1), guideline_event(actual,G,have), guideline_arg(actual,G,1,A), guideline_arg(actual,G,2,F), guideline_entity(actual,H,recommendation,countable), guideline_cardinality(actual,H,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s10-04',1,box(1),[A,B,C,D,E,F,G,H]),'$guideline_id'(product,'cdc2022-opioid-s10-04',1,ref(9),[A,B,C,D,E,F,G,H]),1,H) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,age,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_event(actual,E,include), guideline_arg(actual,E,1,B), guideline_arg(actual,E,2,D), guideline_cardinality(actual,D,na,I,J), guideline_unit(actual,D,K), guideline_satisfies(I,J,K,less,18,year), guideline_entity(actual,F,'opioid-pain-medication-use',countable), guideline_cardinality(actual,F,na,eq,1), guideline_event(actual,G,have), guideline_arg(actual,G,1,A), guideline_arg(actual,G,2,F), guideline_entity(actual,H,recommendation,countable), guideline_cardinality(actual,H,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s10-04',1,box(1),[A,B,C,D,E,F,G,H]),'$guideline_id'(product,'cdc2022-opioid-s10-04',1,ref(9),[A,B,C,D,E,F,G,H]),2,F) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,age,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_event(actual,E,include), guideline_arg(actual,E,1,B), guideline_arg(actual,E,2,D), guideline_cardinality(actual,D,na,I,J), guideline_unit(actual,D,K), guideline_satisfies(I,J,K,less,18,year), guideline_entity(actual,F,'opioid-pain-medication-use',countable), guideline_cardinality(actual,F,na,eq,1), guideline_event(actual,G,have), guideline_arg(actual,G,1,A), guideline_arg(actual,G,2,F), guideline_entity(actual,H,recommendation,countable), guideline_cardinality(actual,H,na,eq,1).

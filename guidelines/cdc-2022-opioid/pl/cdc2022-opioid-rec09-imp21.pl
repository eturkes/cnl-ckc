% cdc2022-opioid-rec09-imp21.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-rec09-imp21',ace_sha256(ce6767de5711e74a3dc9c076ffd96cd77c5c876719016058bd1e2140d9c355b4),ulex(sha256('2733db8c734f4672e1d598cc0911538cb926c6072c8cc42898ec8aa2d7176379')),vocabulary(sha256('335c132fd0e3d3163ba40c255e7db41eecc36492d8098263c505b17f44bbef5a'))).
% S1: If a patient has an opioid-diversion-risk and the opioid-diversion-risk possibly is likely then every clinician may consider a toxicology-testing for the patient for a withdrawal-free-opioid-discontinuation-determination.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec09-imp21',1,box(1),[A,B,C,D,E,F]),may) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-diversion-risk',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_event(actual,E,be), guideline_qualifier(actual,E,possibility,possibly), guideline_arg(actual,E,1,B), guideline_arg(actual,E,2,D), guideline_property(actual,D,likely,pos), guideline_qualifier(actual,E,possibility,likely), guideline_entity(actual,F,clinician,countable), guideline_cardinality(actual,F,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec09-imp21',1,box(1),[A,B,C,D,E,F]),'$guideline_id'(product,'cdc2022-opioid-rec09-imp21',1,ref(7),[A,B,C,D,E,F]),'toxicology-testing',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-diversion-risk',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_event(actual,E,be), guideline_qualifier(actual,E,possibility,possibly), guideline_arg(actual,E,1,B), guideline_arg(actual,E,2,D), guideline_property(actual,D,likely,pos), guideline_qualifier(actual,E,possibility,likely), guideline_entity(actual,F,clinician,countable), guideline_cardinality(actual,F,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec09-imp21',1,box(1),[A,B,C,D,E,F]),'$guideline_id'(product,'cdc2022-opioid-rec09-imp21',1,ref(7),[A,B,C,D,E,F]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-diversion-risk',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_event(actual,E,be), guideline_qualifier(actual,E,possibility,possibly), guideline_arg(actual,E,1,B), guideline_arg(actual,E,2,D), guideline_property(actual,D,likely,pos), guideline_qualifier(actual,E,possibility,likely), guideline_entity(actual,F,clinician,countable), guideline_cardinality(actual,F,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec09-imp21',1,box(1),[A,B,C,D,E,F]),'$guideline_id'(product,'cdc2022-opioid-rec09-imp21',1,ref(8),[A,B,C,D,E,F]),'withdrawal-free-opioid-discontinuation-determination',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-diversion-risk',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_event(actual,E,be), guideline_qualifier(actual,E,possibility,possibly), guideline_arg(actual,E,1,B), guideline_arg(actual,E,2,D), guideline_property(actual,D,likely,pos), guideline_qualifier(actual,E,possibility,likely), guideline_entity(actual,F,clinician,countable), guideline_cardinality(actual,F,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec09-imp21',1,box(1),[A,B,C,D,E,F]),'$guideline_id'(product,'cdc2022-opioid-rec09-imp21',1,ref(8),[A,B,C,D,E,F]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-diversion-risk',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_event(actual,E,be), guideline_qualifier(actual,E,possibility,possibly), guideline_arg(actual,E,1,B), guideline_arg(actual,E,2,D), guideline_property(actual,D,likely,pos), guideline_qualifier(actual,E,possibility,likely), guideline_entity(actual,F,clinician,countable), guideline_cardinality(actual,F,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec09-imp21',1,box(1),[A,B,C,D,E,F]),'$guideline_id'(product,'cdc2022-opioid-rec09-imp21',1,ref(9),[A,B,C,D,E,F]),consider) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-diversion-risk',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_event(actual,E,be), guideline_qualifier(actual,E,possibility,possibly), guideline_arg(actual,E,1,B), guideline_arg(actual,E,2,D), guideline_property(actual,D,likely,pos), guideline_qualifier(actual,E,possibility,likely), guideline_entity(actual,F,clinician,countable), guideline_cardinality(actual,F,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec09-imp21',1,box(1),[A,B,C,D,E,F]),'$guideline_id'(product,'cdc2022-opioid-rec09-imp21',1,ref(9),[A,B,C,D,E,F]),1,F) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-diversion-risk',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_event(actual,E,be), guideline_qualifier(actual,E,possibility,possibly), guideline_arg(actual,E,1,B), guideline_arg(actual,E,2,D), guideline_property(actual,D,likely,pos), guideline_qualifier(actual,E,possibility,likely), guideline_entity(actual,F,clinician,countable), guideline_cardinality(actual,F,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec09-imp21',1,box(1),[A,B,C,D,E,F]),'$guideline_id'(product,'cdc2022-opioid-rec09-imp21',1,ref(9),[A,B,C,D,E,F]),2,'$guideline_id'(product,'cdc2022-opioid-rec09-imp21',1,ref(7),[A,B,C,D,E,F])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-diversion-risk',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_event(actual,E,be), guideline_qualifier(actual,E,possibility,possibly), guideline_arg(actual,E,1,B), guideline_arg(actual,E,2,D), guideline_property(actual,D,likely,pos), guideline_qualifier(actual,E,possibility,likely), guideline_entity(actual,F,clinician,countable), guideline_cardinality(actual,F,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec09-imp21',1,box(1),[A,B,C,D,E,F]),'$guideline_id'(product,'cdc2022-opioid-rec09-imp21',1,ref(9),[A,B,C,D,E,F]),for,'$guideline_id'(product,'cdc2022-opioid-rec09-imp21',1,ref(8),[A,B,C,D,E,F])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-diversion-risk',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_event(actual,E,be), guideline_qualifier(actual,E,possibility,possibly), guideline_arg(actual,E,1,B), guideline_arg(actual,E,2,D), guideline_property(actual,D,likely,pos), guideline_qualifier(actual,E,possibility,likely), guideline_entity(actual,F,clinician,countable), guideline_cardinality(actual,F,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec09-imp21',1,box(1),[A,B,C,D,E,F]),'$guideline_id'(product,'cdc2022-opioid-rec09-imp21',1,ref(9),[A,B,C,D,E,F]),for,A) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'opioid-diversion-risk',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,have), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_event(actual,E,be), guideline_qualifier(actual,E,possibility,possibly), guideline_arg(actual,E,1,B), guideline_arg(actual,E,2,D), guideline_property(actual,D,likely,pos), guideline_qualifier(actual,E,possibility,likely), guideline_entity(actual,F,clinician,countable), guideline_cardinality(actual,F,na,eq,1).
% S2: An opioid-diversion-risk-example includes a prescription-opioid-sharing-without-taking-example and includes a prescription-opioid-selling-without-taking-example.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec09-imp21',2,ref(1),[]),'opioid-diversion-risk-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec09-imp21',2,ref(1),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec09-imp21',2,ref(2),[]),'prescription-opioid-sharing-without-taking-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec09-imp21',2,ref(2),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-rec09-imp21',2,ref(3),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec09-imp21',2,ref(3),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec09-imp21',2,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec09-imp21',2,ref(3),[]),2,'$guideline_id'(product,'cdc2022-opioid-rec09-imp21',2,ref(2),[])).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec09-imp21',2,ref(4),[]),'prescription-opioid-selling-without-taking-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec09-imp21',2,ref(4),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-rec09-imp21',2,ref(5),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec09-imp21',2,ref(5),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec09-imp21',2,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec09-imp21',2,ref(5),[]),2,'$guideline_id'(product,'cdc2022-opioid-rec09-imp21',2,ref(4),[])).

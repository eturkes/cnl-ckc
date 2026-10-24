% cdc2022-opioid-rec11-imp06.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_schema_version(1).
guideline_document('cdc2022-opioid-rec11-imp06',ace_sha256(f733caebee4061a6bfcb5f40e66771ccdb9737be684f0ea4d0ee71befccf188c),ulex(sha256('45608095e3f77853fdabcce148a9eb7633a8ae776353b8f008c753b846886588')),vocabulary(sha256(c36289aa40f75805a83501ca057b29dd1ba7292b025716853f654890b77ec50c))).
% S1: If a clinician plans a benzodiazepine-discontinuation then the clinician should gradually taper a benzodiazepine before the benzodiazepine-discontinuation.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec11-imp06',1,box(1),[A,B,C]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'benzodiazepine-discontinuation',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,plan), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec11-imp06',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec11-imp06',1,ref(4),[A,B,C]),benzodiazepine,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'benzodiazepine-discontinuation',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,plan), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec11-imp06',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec11-imp06',1,ref(4),[A,B,C]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'benzodiazepine-discontinuation',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,plan), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-rec11-imp06',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec11-imp06',1,ref(5),[A,B,C]),manner,gradually) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'benzodiazepine-discontinuation',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,plan), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec11-imp06',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec11-imp06',1,ref(5),[A,B,C]),taper) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'benzodiazepine-discontinuation',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,plan), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec11-imp06',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec11-imp06',1,ref(5),[A,B,C]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'benzodiazepine-discontinuation',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,plan), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec11-imp06',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec11-imp06',1,ref(5),[A,B,C]),2,'$guideline_id'(product,'cdc2022-opioid-rec11-imp06',1,ref(4),[A,B,C])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'benzodiazepine-discontinuation',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,plan), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec11-imp06',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec11-imp06',1,ref(5),[A,B,C]),before,B) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'benzodiazepine-discontinuation',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,plan), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_order('$guideline_id'(context,'cdc2022-opioid-rec11-imp06',1,box(1),[A,B,C]),'$guideline_id'(product,'cdc2022-opioid-rec11-imp06',1,ref(5),[A,B,C]),before,B) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'benzodiazepine-discontinuation',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,plan), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
% S2: An abrupt-benzodiazepine-withdrawal can relate to a rebound-anxiety and can relate to a hallucination and can relate to a seizure and can relate to some delirium-tremens and can rarely relate to a death.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec11-imp06',2,ref(1),[]),'abrupt-benzodiazepine-withdrawal',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec11-imp06',2,ref(1),[]),na,eq,1).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec11-imp06',2,box(1),[]),can).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec11-imp06',2,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-rec11-imp06',2,ref(2),[]),'rebound-anxiety',countable).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec11-imp06',2,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-rec11-imp06',2,ref(2),[]),na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec11-imp06',2,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-rec11-imp06',2,ref(3),[]),relate).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec11-imp06',2,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-rec11-imp06',2,ref(3),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec11-imp06',2,ref(1),[])).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec11-imp06',2,box(1),[]),'$guideline_id'(product,'cdc2022-opioid-rec11-imp06',2,ref(3),[]),to,'$guideline_id'(product,'cdc2022-opioid-rec11-imp06',2,ref(2),[])).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec11-imp06',2,box(2),[]),can).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec11-imp06',2,box(2),[]),'$guideline_id'(product,'cdc2022-opioid-rec11-imp06',2,ref(4),[]),hallucination,countable).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec11-imp06',2,box(2),[]),'$guideline_id'(product,'cdc2022-opioid-rec11-imp06',2,ref(4),[]),na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec11-imp06',2,box(2),[]),'$guideline_id'(product,'cdc2022-opioid-rec11-imp06',2,ref(5),[]),relate).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec11-imp06',2,box(2),[]),'$guideline_id'(product,'cdc2022-opioid-rec11-imp06',2,ref(5),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec11-imp06',2,ref(1),[])).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec11-imp06',2,box(2),[]),'$guideline_id'(product,'cdc2022-opioid-rec11-imp06',2,ref(5),[]),to,'$guideline_id'(product,'cdc2022-opioid-rec11-imp06',2,ref(4),[])).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec11-imp06',2,box(3),[]),can).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec11-imp06',2,box(3),[]),'$guideline_id'(product,'cdc2022-opioid-rec11-imp06',2,ref(6),[]),seizure,countable).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec11-imp06',2,box(3),[]),'$guideline_id'(product,'cdc2022-opioid-rec11-imp06',2,ref(6),[]),na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec11-imp06',2,box(3),[]),'$guideline_id'(product,'cdc2022-opioid-rec11-imp06',2,ref(7),[]),relate).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec11-imp06',2,box(3),[]),'$guideline_id'(product,'cdc2022-opioid-rec11-imp06',2,ref(7),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec11-imp06',2,ref(1),[])).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec11-imp06',2,box(3),[]),'$guideline_id'(product,'cdc2022-opioid-rec11-imp06',2,ref(7),[]),to,'$guideline_id'(product,'cdc2022-opioid-rec11-imp06',2,ref(6),[])).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec11-imp06',2,box(4),[]),can).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec11-imp06',2,box(4),[]),'$guideline_id'(product,'cdc2022-opioid-rec11-imp06',2,ref(8),[]),'delirium-tremens',mass).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec11-imp06',2,box(4),[]),'$guideline_id'(product,'cdc2022-opioid-rec11-imp06',2,ref(8),[]),na,na,na).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec11-imp06',2,box(4),[]),'$guideline_id'(product,'cdc2022-opioid-rec11-imp06',2,ref(9),[]),relate).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec11-imp06',2,box(4),[]),'$guideline_id'(product,'cdc2022-opioid-rec11-imp06',2,ref(9),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec11-imp06',2,ref(1),[])).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec11-imp06',2,box(4),[]),'$guideline_id'(product,'cdc2022-opioid-rec11-imp06',2,ref(9),[]),to,'$guideline_id'(product,'cdc2022-opioid-rec11-imp06',2,ref(8),[])).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec11-imp06',2,box(5),[]),can).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec11-imp06',2,box(5),[]),'$guideline_id'(product,'cdc2022-opioid-rec11-imp06',2,ref(10),[]),death,countable).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec11-imp06',2,box(5),[]),'$guideline_id'(product,'cdc2022-opioid-rec11-imp06',2,ref(10),[]),na,eq,1).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-rec11-imp06',2,box(5),[]),'$guideline_id'(product,'cdc2022-opioid-rec11-imp06',2,ref(11),[]),usuality,rarely).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec11-imp06',2,box(5),[]),'$guideline_id'(product,'cdc2022-opioid-rec11-imp06',2,ref(11),[]),relate).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec11-imp06',2,box(5),[]),'$guideline_id'(product,'cdc2022-opioid-rec11-imp06',2,ref(11),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec11-imp06',2,ref(1),[])).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec11-imp06',2,box(5),[]),'$guideline_id'(product,'cdc2022-opioid-rec11-imp06',2,ref(11),[]),to,'$guideline_id'(product,'cdc2022-opioid-rec11-imp06',2,ref(10),[])).

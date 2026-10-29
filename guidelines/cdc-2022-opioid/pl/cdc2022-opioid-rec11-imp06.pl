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
:- multifile(guideline_supersession/4).
:- discontiguous(guideline_supersession/4).
:- multifile(guideline_evaluation_date/1).
:- discontiguous(guideline_evaluation_date/1).
guideline_schema_version(1).
guideline_document('cdc2022-opioid-rec11-imp06',ace_sha256('6f63d3e18c3cd5b983f8129f6338d4a1bea5bc92f1f7104f5b5fa0bb5cbc2ff6'),ulex(sha256(fba48e003101de64df33400ab2595fa7589360ac19f4c77ac3d17bd085f5b8c0)),vocabulary(sha256('335c132fd0e3d3163ba40c255e7db41eecc36492d8098263c505b17f44bbef5a'))).
% S1: If a patient receives a benzodiazepine and a clinician plans a benzodiazepine-discontinuation that stops the benzodiazepine for the patient then the clinician should gradually taper the benzodiazepine before the benzodiazepine-discontinuation.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec11-imp06',1,box(1),[A,B,C,D,E,F,G]),should) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,benzodiazepine,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,receive), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1), guideline_entity(actual,E,'benzodiazepine-discontinuation',countable), guideline_cardinality(actual,E,na,eq,1), guideline_event(actual,F,stop), guideline_arg(actual,F,1,E), guideline_arg(actual,F,2,B), guideline_pp(actual,F,for,A), guideline_event(actual,G,plan), guideline_arg(actual,G,1,D), guideline_arg(actual,G,2,E), \+ guideline_exclusion('cdc-2022-opioid').
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-rec11-imp06',1,box(1),[A,B,C,D,E,F,G]),'$guideline_id'(product,'cdc2022-opioid-rec11-imp06',1,ref(8),[A,B,C,D,E,F,G]),manner,gradually) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,benzodiazepine,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,receive), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1), guideline_entity(actual,E,'benzodiazepine-discontinuation',countable), guideline_cardinality(actual,E,na,eq,1), guideline_event(actual,F,stop), guideline_arg(actual,F,1,E), guideline_arg(actual,F,2,B), guideline_pp(actual,F,for,A), guideline_event(actual,G,plan), guideline_arg(actual,G,1,D), guideline_arg(actual,G,2,E).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec11-imp06',1,box(1),[A,B,C,D,E,F,G]),'$guideline_id'(product,'cdc2022-opioid-rec11-imp06',1,ref(8),[A,B,C,D,E,F,G]),taper) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,benzodiazepine,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,receive), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1), guideline_entity(actual,E,'benzodiazepine-discontinuation',countable), guideline_cardinality(actual,E,na,eq,1), guideline_event(actual,F,stop), guideline_arg(actual,F,1,E), guideline_arg(actual,F,2,B), guideline_pp(actual,F,for,A), guideline_event(actual,G,plan), guideline_arg(actual,G,1,D), guideline_arg(actual,G,2,E).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec11-imp06',1,box(1),[A,B,C,D,E,F,G]),'$guideline_id'(product,'cdc2022-opioid-rec11-imp06',1,ref(8),[A,B,C,D,E,F,G]),1,D) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,benzodiazepine,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,receive), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1), guideline_entity(actual,E,'benzodiazepine-discontinuation',countable), guideline_cardinality(actual,E,na,eq,1), guideline_event(actual,F,stop), guideline_arg(actual,F,1,E), guideline_arg(actual,F,2,B), guideline_pp(actual,F,for,A), guideline_event(actual,G,plan), guideline_arg(actual,G,1,D), guideline_arg(actual,G,2,E).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec11-imp06',1,box(1),[A,B,C,D,E,F,G]),'$guideline_id'(product,'cdc2022-opioid-rec11-imp06',1,ref(8),[A,B,C,D,E,F,G]),2,B) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,benzodiazepine,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,receive), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1), guideline_entity(actual,E,'benzodiazepine-discontinuation',countable), guideline_cardinality(actual,E,na,eq,1), guideline_event(actual,F,stop), guideline_arg(actual,F,1,E), guideline_arg(actual,F,2,B), guideline_pp(actual,F,for,A), guideline_event(actual,G,plan), guideline_arg(actual,G,1,D), guideline_arg(actual,G,2,E).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec11-imp06',1,box(1),[A,B,C,D,E,F,G]),'$guideline_id'(product,'cdc2022-opioid-rec11-imp06',1,ref(8),[A,B,C,D,E,F,G]),before,E) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,benzodiazepine,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,receive), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1), guideline_entity(actual,E,'benzodiazepine-discontinuation',countable), guideline_cardinality(actual,E,na,eq,1), guideline_event(actual,F,stop), guideline_arg(actual,F,1,E), guideline_arg(actual,F,2,B), guideline_pp(actual,F,for,A), guideline_event(actual,G,plan), guideline_arg(actual,G,1,D), guideline_arg(actual,G,2,E).
guideline_order('$guideline_id'(context,'cdc2022-opioid-rec11-imp06',1,box(1),[A,B,C,D,E,F,G]),'$guideline_id'(product,'cdc2022-opioid-rec11-imp06',1,ref(8),[A,B,C,D,E,F,G]),before,E) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,benzodiazepine,countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,receive), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1), guideline_entity(actual,E,'benzodiazepine-discontinuation',countable), guideline_cardinality(actual,E,na,eq,1), guideline_event(actual,F,stop), guideline_arg(actual,F,1,E), guideline_arg(actual,F,2,B), guideline_pp(actual,F,for,A), guideline_event(actual,G,plan), guideline_arg(actual,G,1,D), guideline_arg(actual,G,2,E).
% S2: Every abrupt-benzodiazepine-withdrawal can relate to a rebound-anxiety and can relate to a hallucination and can relate to a seizure and can relate to some delirium-tremens and can rarely relate to a death.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec11-imp06',2,box(1),[A]),can) :- guideline_entity(actual,A,'abrupt-benzodiazepine-withdrawal',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec11-imp06',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec11-imp06',2,ref(2),[A]),'rebound-anxiety',countable) :- guideline_entity(actual,A,'abrupt-benzodiazepine-withdrawal',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec11-imp06',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec11-imp06',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'abrupt-benzodiazepine-withdrawal',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec11-imp06',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec11-imp06',2,ref(3),[A]),relate) :- guideline_entity(actual,A,'abrupt-benzodiazepine-withdrawal',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec11-imp06',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec11-imp06',2,ref(3),[A]),1,A) :- guideline_entity(actual,A,'abrupt-benzodiazepine-withdrawal',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec11-imp06',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-rec11-imp06',2,ref(3),[A]),to,'$guideline_id'(product,'cdc2022-opioid-rec11-imp06',2,ref(2),[A])) :- guideline_entity(actual,A,'abrupt-benzodiazepine-withdrawal',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec11-imp06',2,box(2),[A]),can) :- guideline_entity(actual,A,'abrupt-benzodiazepine-withdrawal',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec11-imp06',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec11-imp06',2,ref(4),[A]),hallucination,countable) :- guideline_entity(actual,A,'abrupt-benzodiazepine-withdrawal',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec11-imp06',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec11-imp06',2,ref(4),[A]),na,eq,1) :- guideline_entity(actual,A,'abrupt-benzodiazepine-withdrawal',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec11-imp06',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec11-imp06',2,ref(5),[A]),relate) :- guideline_entity(actual,A,'abrupt-benzodiazepine-withdrawal',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec11-imp06',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec11-imp06',2,ref(5),[A]),1,A) :- guideline_entity(actual,A,'abrupt-benzodiazepine-withdrawal',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec11-imp06',2,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-rec11-imp06',2,ref(5),[A]),to,'$guideline_id'(product,'cdc2022-opioid-rec11-imp06',2,ref(4),[A])) :- guideline_entity(actual,A,'abrupt-benzodiazepine-withdrawal',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec11-imp06',2,box(3),[A]),can) :- guideline_entity(actual,A,'abrupt-benzodiazepine-withdrawal',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec11-imp06',2,box(3),[A]),'$guideline_id'(product,'cdc2022-opioid-rec11-imp06',2,ref(6),[A]),seizure,countable) :- guideline_entity(actual,A,'abrupt-benzodiazepine-withdrawal',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec11-imp06',2,box(3),[A]),'$guideline_id'(product,'cdc2022-opioid-rec11-imp06',2,ref(6),[A]),na,eq,1) :- guideline_entity(actual,A,'abrupt-benzodiazepine-withdrawal',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec11-imp06',2,box(3),[A]),'$guideline_id'(product,'cdc2022-opioid-rec11-imp06',2,ref(7),[A]),relate) :- guideline_entity(actual,A,'abrupt-benzodiazepine-withdrawal',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec11-imp06',2,box(3),[A]),'$guideline_id'(product,'cdc2022-opioid-rec11-imp06',2,ref(7),[A]),1,A) :- guideline_entity(actual,A,'abrupt-benzodiazepine-withdrawal',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec11-imp06',2,box(3),[A]),'$guideline_id'(product,'cdc2022-opioid-rec11-imp06',2,ref(7),[A]),to,'$guideline_id'(product,'cdc2022-opioid-rec11-imp06',2,ref(6),[A])) :- guideline_entity(actual,A,'abrupt-benzodiazepine-withdrawal',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec11-imp06',2,box(4),[A]),can) :- guideline_entity(actual,A,'abrupt-benzodiazepine-withdrawal',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec11-imp06',2,box(4),[A]),'$guideline_id'(product,'cdc2022-opioid-rec11-imp06',2,ref(8),[A]),'delirium-tremens',mass) :- guideline_entity(actual,A,'abrupt-benzodiazepine-withdrawal',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec11-imp06',2,box(4),[A]),'$guideline_id'(product,'cdc2022-opioid-rec11-imp06',2,ref(8),[A]),na,na,na) :- guideline_entity(actual,A,'abrupt-benzodiazepine-withdrawal',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec11-imp06',2,box(4),[A]),'$guideline_id'(product,'cdc2022-opioid-rec11-imp06',2,ref(9),[A]),relate) :- guideline_entity(actual,A,'abrupt-benzodiazepine-withdrawal',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec11-imp06',2,box(4),[A]),'$guideline_id'(product,'cdc2022-opioid-rec11-imp06',2,ref(9),[A]),1,A) :- guideline_entity(actual,A,'abrupt-benzodiazepine-withdrawal',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec11-imp06',2,box(4),[A]),'$guideline_id'(product,'cdc2022-opioid-rec11-imp06',2,ref(9),[A]),to,'$guideline_id'(product,'cdc2022-opioid-rec11-imp06',2,ref(8),[A])) :- guideline_entity(actual,A,'abrupt-benzodiazepine-withdrawal',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec11-imp06',2,box(5),[A]),can) :- guideline_entity(actual,A,'abrupt-benzodiazepine-withdrawal',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec11-imp06',2,box(5),[A]),'$guideline_id'(product,'cdc2022-opioid-rec11-imp06',2,ref(10),[A]),death,countable) :- guideline_entity(actual,A,'abrupt-benzodiazepine-withdrawal',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec11-imp06',2,box(5),[A]),'$guideline_id'(product,'cdc2022-opioid-rec11-imp06',2,ref(10),[A]),na,eq,1) :- guideline_entity(actual,A,'abrupt-benzodiazepine-withdrawal',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-rec11-imp06',2,box(5),[A]),'$guideline_id'(product,'cdc2022-opioid-rec11-imp06',2,ref(11),[A]),usuality,rarely) :- guideline_entity(actual,A,'abrupt-benzodiazepine-withdrawal',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec11-imp06',2,box(5),[A]),'$guideline_id'(product,'cdc2022-opioid-rec11-imp06',2,ref(11),[A]),relate) :- guideline_entity(actual,A,'abrupt-benzodiazepine-withdrawal',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec11-imp06',2,box(5),[A]),'$guideline_id'(product,'cdc2022-opioid-rec11-imp06',2,ref(11),[A]),1,A) :- guideline_entity(actual,A,'abrupt-benzodiazepine-withdrawal',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec11-imp06',2,box(5),[A]),'$guideline_id'(product,'cdc2022-opioid-rec11-imp06',2,ref(11),[A]),to,'$guideline_id'(product,'cdc2022-opioid-rec11-imp06',2,ref(10),[A])) :- guideline_entity(actual,A,'abrupt-benzodiazepine-withdrawal',countable), guideline_cardinality(actual,A,na,eq,1).

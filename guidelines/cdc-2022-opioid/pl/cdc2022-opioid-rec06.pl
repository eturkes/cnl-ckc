% cdc2022-opioid-rec06.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
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
guideline_document('cdc2022-opioid-rec06',ace_sha256('3fb9daa7136b6c988cce4863837cae390c6d263828ab9046d3cb92a4943687fd'),ulex(sha256(ef57ba7180864095bab88b95f11ddb1a2c858593eba715dbfbe874db09506c5b))).
% S1: A recommendation is a category-A-recommendation and is an evidence-type-4-recommendation.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec06',1,ref(1),[]),recommendation,countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec06',1,ref(1),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec06',1,ref(2),[]),'category-A-recommendation',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec06',1,ref(2),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-rec06',1,ref(3),[]),be).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec06',1,ref(3),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec06',1,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec06',1,ref(3),[]),2,'$guideline_id'(product,'cdc2022-opioid-rec06',1,ref(2),[])).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec06',1,ref(4),[]),'evidence-type-4-recommendation',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec06',1,ref(4),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-rec06',1,ref(5),[]),be).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec06',1,ref(5),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec06',1,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec06',1,ref(5),[]),2,'$guideline_id'(product,'cdc2022-opioid-rec06',1,ref(4),[])).
% S2: If an opioid is needed for an acute-pain then every clinician should limit an opioid-quantity for an expected-severe-pain-duration.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec06',2,box(1),[A,B,C,D,E]),should) :- guideline_entity(actual,A,opioid,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'acute-pain',countable), guideline_cardinality(actual,B,na,eq,1), guideline_property(actual,C,needed,pos), guideline_event(actual,D,be), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,C), guideline_pp(actual,D,for,B), guideline_entity(actual,E,clinician,countable), guideline_cardinality(actual,E,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec06',2,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-rec06',2,ref(6),[A,B,C,D,E]),'opioid-quantity',countable) :- guideline_entity(actual,A,opioid,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'acute-pain',countable), guideline_cardinality(actual,B,na,eq,1), guideline_property(actual,C,needed,pos), guideline_event(actual,D,be), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,C), guideline_pp(actual,D,for,B), guideline_entity(actual,E,clinician,countable), guideline_cardinality(actual,E,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec06',2,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-rec06',2,ref(6),[A,B,C,D,E]),na,eq,1) :- guideline_entity(actual,A,opioid,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'acute-pain',countable), guideline_cardinality(actual,B,na,eq,1), guideline_property(actual,C,needed,pos), guideline_event(actual,D,be), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,C), guideline_pp(actual,D,for,B), guideline_entity(actual,E,clinician,countable), guideline_cardinality(actual,E,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-rec06',2,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-rec06',2,ref(7),[A,B,C,D,E]),'expected-severe-pain-duration',countable) :- guideline_entity(actual,A,opioid,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'acute-pain',countable), guideline_cardinality(actual,B,na,eq,1), guideline_property(actual,C,needed,pos), guideline_event(actual,D,be), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,C), guideline_pp(actual,D,for,B), guideline_entity(actual,E,clinician,countable), guideline_cardinality(actual,E,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-rec06',2,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-rec06',2,ref(7),[A,B,C,D,E]),na,eq,1) :- guideline_entity(actual,A,opioid,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'acute-pain',countable), guideline_cardinality(actual,B,na,eq,1), guideline_property(actual,C,needed,pos), guideline_event(actual,D,be), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,C), guideline_pp(actual,D,for,B), guideline_entity(actual,E,clinician,countable), guideline_cardinality(actual,E,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec06',2,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-rec06',2,ref(8),[A,B,C,D,E]),limit) :- guideline_entity(actual,A,opioid,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'acute-pain',countable), guideline_cardinality(actual,B,na,eq,1), guideline_property(actual,C,needed,pos), guideline_event(actual,D,be), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,C), guideline_pp(actual,D,for,B), guideline_entity(actual,E,clinician,countable), guideline_cardinality(actual,E,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec06',2,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-rec06',2,ref(8),[A,B,C,D,E]),1,E) :- guideline_entity(actual,A,opioid,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'acute-pain',countable), guideline_cardinality(actual,B,na,eq,1), guideline_property(actual,C,needed,pos), guideline_event(actual,D,be), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,C), guideline_pp(actual,D,for,B), guideline_entity(actual,E,clinician,countable), guideline_cardinality(actual,E,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec06',2,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-rec06',2,ref(8),[A,B,C,D,E]),2,'$guideline_id'(product,'cdc2022-opioid-rec06',2,ref(6),[A,B,C,D,E])) :- guideline_entity(actual,A,opioid,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'acute-pain',countable), guideline_cardinality(actual,B,na,eq,1), guideline_property(actual,C,needed,pos), guideline_event(actual,D,be), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,C), guideline_pp(actual,D,for,B), guideline_entity(actual,E,clinician,countable), guideline_cardinality(actual,E,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec06',2,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-rec06',2,ref(8),[A,B,C,D,E]),for,'$guideline_id'(product,'cdc2022-opioid-rec06',2,ref(7),[A,B,C,D,E])) :- guideline_entity(actual,A,opioid,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'acute-pain',countable), guideline_cardinality(actual,B,na,eq,1), guideline_property(actual,C,needed,pos), guideline_event(actual,D,be), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,C), guideline_pp(actual,D,for,B), guideline_entity(actual,E,clinician,countable), guideline_cardinality(actual,E,na,eq,1).

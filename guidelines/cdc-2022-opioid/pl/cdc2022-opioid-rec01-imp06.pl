% cdc2022-opioid-rec01-imp06.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
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
guideline_document('cdc2022-opioid-rec01-imp06',ace_sha256(fe179b2c63e598fc107cea0f518fc6326b4a75cd516c4c623104ad5b745d3351),ulex(sha256('06c1c26d6b6abe1436b8c94a276937ad072806062117417de2ab8086fdf9a3e9'))).
% S1: An opioid-therapy has an important-role for an acute-pain that relates to a severe-traumatic-injury.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',1,ref(1),[]),'opioid-therapy',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',1,ref(1),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',1,ref(2),[]),'important-role',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',1,ref(2),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',1,ref(3),[]),'acute-pain',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',1,ref(3),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',1,ref(4),[]),'severe-traumatic-injury',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',1,ref(4),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',1,ref(5),[]),relate).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',1,ref(5),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',1,ref(3),[])).
guideline_pp(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',1,ref(5),[]),to,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',1,ref(4),[])).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',1,ref(6),[]),have).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',1,ref(6),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',1,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',1,ref(6),[]),2,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',1,ref(2),[])).
guideline_pp(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',1,ref(6),[]),for,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',1,ref(3),[])).
% S2: An opioid-therapy has an important-role for an acute-pain that relates to a crush-injury.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',2,ref(1),[]),'opioid-therapy',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',2,ref(1),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',2,ref(2),[]),'important-role',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',2,ref(2),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',2,ref(3),[]),'acute-pain',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',2,ref(3),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',2,ref(4),[]),'crush-injury',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',2,ref(4),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',2,ref(5),[]),relate).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',2,ref(5),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',2,ref(3),[])).
guideline_pp(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',2,ref(5),[]),to,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',2,ref(4),[])).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',2,ref(6),[]),have).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',2,ref(6),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',2,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',2,ref(6),[]),2,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',2,ref(2),[])).
guideline_pp(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',2,ref(6),[]),for,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',2,ref(3),[])).
% S3: An opioid-therapy has an important-role for an acute-pain that relates to a burn.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',3,ref(1),[]),'opioid-therapy',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',3,ref(1),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',3,ref(2),[]),'important-role',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',3,ref(2),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',3,ref(3),[]),'acute-pain',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',3,ref(3),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',3,ref(4),[]),burn,countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',3,ref(4),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',3,ref(5),[]),relate).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',3,ref(5),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',3,ref(3),[])).
guideline_pp(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',3,ref(5),[]),to,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',3,ref(4),[])).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',3,ref(6),[]),have).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',3,ref(6),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',3,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',3,ref(6),[]),2,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',3,ref(2),[])).
guideline_pp(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',3,ref(6),[]),for,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',3,ref(3),[])).
% S4: An opioid-therapy has an important-role for a moderate-to-severe-postoperative-pain that relates to an invasive-surgery.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',4,ref(1),[]),'opioid-therapy',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',4,ref(1),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',4,ref(2),[]),'important-role',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',4,ref(2),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',4,ref(3),[]),'moderate-to-severe-postoperative-pain',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',4,ref(3),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',4,ref(4),[]),'invasive-surgery',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',4,ref(4),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',4,ref(5),[]),relate).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',4,ref(5),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',4,ref(3),[])).
guideline_pp(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',4,ref(5),[]),to,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',4,ref(4),[])).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',4,ref(6),[]),have).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',4,ref(6),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',4,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',4,ref(6),[]),2,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',4,ref(2),[])).
guideline_pp(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',4,ref(6),[]),for,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',4,ref(3),[])).
% S5: If an NSAID is contraindicated and an other-therapy is contraindicated or an NSAID is ineffective and an other-therapy is ineffective then an opioid-therapy has an important-role for an other-severe-acute-pain.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',5,ref(13),[A,B,C,D,E,F]),'opioid-therapy',countable) :- guideline_entity(actual,A,'NSAID',countable), guideline_cardinality(actual,A,na,eq,1), guideline_property(actual,B,contraindicated,pos), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,'other-therapy',countable), guideline_cardinality(actual,D,na,eq,1), guideline_property(actual,E,contraindicated,pos), guideline_event(actual,F,be), guideline_arg(actual,F,1,D), guideline_arg(actual,F,2,E).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',5,ref(13),[A,B,C,D,E,F]),na,eq,1) :- guideline_entity(actual,A,'NSAID',countable), guideline_cardinality(actual,A,na,eq,1), guideline_property(actual,B,contraindicated,pos), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,'other-therapy',countable), guideline_cardinality(actual,D,na,eq,1), guideline_property(actual,E,contraindicated,pos), guideline_event(actual,F,be), guideline_arg(actual,F,1,D), guideline_arg(actual,F,2,E).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',5,ref(14),[A,B,C,D,E,F]),'important-role',countable) :- guideline_entity(actual,A,'NSAID',countable), guideline_cardinality(actual,A,na,eq,1), guideline_property(actual,B,contraindicated,pos), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,'other-therapy',countable), guideline_cardinality(actual,D,na,eq,1), guideline_property(actual,E,contraindicated,pos), guideline_event(actual,F,be), guideline_arg(actual,F,1,D), guideline_arg(actual,F,2,E).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',5,ref(14),[A,B,C,D,E,F]),na,eq,1) :- guideline_entity(actual,A,'NSAID',countable), guideline_cardinality(actual,A,na,eq,1), guideline_property(actual,B,contraindicated,pos), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,'other-therapy',countable), guideline_cardinality(actual,D,na,eq,1), guideline_property(actual,E,contraindicated,pos), guideline_event(actual,F,be), guideline_arg(actual,F,1,D), guideline_arg(actual,F,2,E).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',5,ref(15),[A,B,C,D,E,F]),'other-severe-acute-pain',countable) :- guideline_entity(actual,A,'NSAID',countable), guideline_cardinality(actual,A,na,eq,1), guideline_property(actual,B,contraindicated,pos), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,'other-therapy',countable), guideline_cardinality(actual,D,na,eq,1), guideline_property(actual,E,contraindicated,pos), guideline_event(actual,F,be), guideline_arg(actual,F,1,D), guideline_arg(actual,F,2,E).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',5,ref(15),[A,B,C,D,E,F]),na,eq,1) :- guideline_entity(actual,A,'NSAID',countable), guideline_cardinality(actual,A,na,eq,1), guideline_property(actual,B,contraindicated,pos), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,'other-therapy',countable), guideline_cardinality(actual,D,na,eq,1), guideline_property(actual,E,contraindicated,pos), guideline_event(actual,F,be), guideline_arg(actual,F,1,D), guideline_arg(actual,F,2,E).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',5,ref(16),[A,B,C,D,E,F]),have) :- guideline_entity(actual,A,'NSAID',countable), guideline_cardinality(actual,A,na,eq,1), guideline_property(actual,B,contraindicated,pos), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,'other-therapy',countable), guideline_cardinality(actual,D,na,eq,1), guideline_property(actual,E,contraindicated,pos), guideline_event(actual,F,be), guideline_arg(actual,F,1,D), guideline_arg(actual,F,2,E).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',5,ref(16),[A,B,C,D,E,F]),1,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',5,ref(13),[A,B,C,D,E,F])) :- guideline_entity(actual,A,'NSAID',countable), guideline_cardinality(actual,A,na,eq,1), guideline_property(actual,B,contraindicated,pos), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,'other-therapy',countable), guideline_cardinality(actual,D,na,eq,1), guideline_property(actual,E,contraindicated,pos), guideline_event(actual,F,be), guideline_arg(actual,F,1,D), guideline_arg(actual,F,2,E).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',5,ref(16),[A,B,C,D,E,F]),2,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',5,ref(14),[A,B,C,D,E,F])) :- guideline_entity(actual,A,'NSAID',countable), guideline_cardinality(actual,A,na,eq,1), guideline_property(actual,B,contraindicated,pos), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,'other-therapy',countable), guideline_cardinality(actual,D,na,eq,1), guideline_property(actual,E,contraindicated,pos), guideline_event(actual,F,be), guideline_arg(actual,F,1,D), guideline_arg(actual,F,2,E).
guideline_pp(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',5,ref(16),[A,B,C,D,E,F]),for,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',5,ref(15),[A,B,C,D,E,F])) :- guideline_entity(actual,A,'NSAID',countable), guideline_cardinality(actual,A,na,eq,1), guideline_property(actual,B,contraindicated,pos), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,'other-therapy',countable), guideline_cardinality(actual,D,na,eq,1), guideline_property(actual,E,contraindicated,pos), guideline_event(actual,F,be), guideline_arg(actual,F,1,D), guideline_arg(actual,F,2,E).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',5,ref(13),[A,B,C,D,E,F]),'opioid-therapy',countable) :- guideline_entity(actual,A,'NSAID',countable), guideline_cardinality(actual,A,na,eq,1), guideline_property(actual,B,ineffective,pos), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,'other-therapy',countable), guideline_cardinality(actual,D,na,eq,1), guideline_property(actual,E,ineffective,pos), guideline_event(actual,F,be), guideline_arg(actual,F,1,D), guideline_arg(actual,F,2,E).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',5,ref(13),[A,B,C,D,E,F]),na,eq,1) :- guideline_entity(actual,A,'NSAID',countable), guideline_cardinality(actual,A,na,eq,1), guideline_property(actual,B,ineffective,pos), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,'other-therapy',countable), guideline_cardinality(actual,D,na,eq,1), guideline_property(actual,E,ineffective,pos), guideline_event(actual,F,be), guideline_arg(actual,F,1,D), guideline_arg(actual,F,2,E).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',5,ref(14),[A,B,C,D,E,F]),'important-role',countable) :- guideline_entity(actual,A,'NSAID',countable), guideline_cardinality(actual,A,na,eq,1), guideline_property(actual,B,ineffective,pos), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,'other-therapy',countable), guideline_cardinality(actual,D,na,eq,1), guideline_property(actual,E,ineffective,pos), guideline_event(actual,F,be), guideline_arg(actual,F,1,D), guideline_arg(actual,F,2,E).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',5,ref(14),[A,B,C,D,E,F]),na,eq,1) :- guideline_entity(actual,A,'NSAID',countable), guideline_cardinality(actual,A,na,eq,1), guideline_property(actual,B,ineffective,pos), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,'other-therapy',countable), guideline_cardinality(actual,D,na,eq,1), guideline_property(actual,E,ineffective,pos), guideline_event(actual,F,be), guideline_arg(actual,F,1,D), guideline_arg(actual,F,2,E).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',5,ref(15),[A,B,C,D,E,F]),'other-severe-acute-pain',countable) :- guideline_entity(actual,A,'NSAID',countable), guideline_cardinality(actual,A,na,eq,1), guideline_property(actual,B,ineffective,pos), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,'other-therapy',countable), guideline_cardinality(actual,D,na,eq,1), guideline_property(actual,E,ineffective,pos), guideline_event(actual,F,be), guideline_arg(actual,F,1,D), guideline_arg(actual,F,2,E).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',5,ref(15),[A,B,C,D,E,F]),na,eq,1) :- guideline_entity(actual,A,'NSAID',countable), guideline_cardinality(actual,A,na,eq,1), guideline_property(actual,B,ineffective,pos), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,'other-therapy',countable), guideline_cardinality(actual,D,na,eq,1), guideline_property(actual,E,ineffective,pos), guideline_event(actual,F,be), guideline_arg(actual,F,1,D), guideline_arg(actual,F,2,E).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',5,ref(16),[A,B,C,D,E,F]),have) :- guideline_entity(actual,A,'NSAID',countable), guideline_cardinality(actual,A,na,eq,1), guideline_property(actual,B,ineffective,pos), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,'other-therapy',countable), guideline_cardinality(actual,D,na,eq,1), guideline_property(actual,E,ineffective,pos), guideline_event(actual,F,be), guideline_arg(actual,F,1,D), guideline_arg(actual,F,2,E).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',5,ref(16),[A,B,C,D,E,F]),1,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',5,ref(13),[A,B,C,D,E,F])) :- guideline_entity(actual,A,'NSAID',countable), guideline_cardinality(actual,A,na,eq,1), guideline_property(actual,B,ineffective,pos), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,'other-therapy',countable), guideline_cardinality(actual,D,na,eq,1), guideline_property(actual,E,ineffective,pos), guideline_event(actual,F,be), guideline_arg(actual,F,1,D), guideline_arg(actual,F,2,E).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',5,ref(16),[A,B,C,D,E,F]),2,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',5,ref(14),[A,B,C,D,E,F])) :- guideline_entity(actual,A,'NSAID',countable), guideline_cardinality(actual,A,na,eq,1), guideline_property(actual,B,ineffective,pos), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,'other-therapy',countable), guideline_cardinality(actual,D,na,eq,1), guideline_property(actual,E,ineffective,pos), guideline_event(actual,F,be), guideline_arg(actual,F,1,D), guideline_arg(actual,F,2,E).
guideline_pp(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',5,ref(16),[A,B,C,D,E,F]),for,'$guideline_id'(product,'cdc2022-opioid-rec01-imp06',5,ref(15),[A,B,C,D,E,F])) :- guideline_entity(actual,A,'NSAID',countable), guideline_cardinality(actual,A,na,eq,1), guideline_property(actual,B,ineffective,pos), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,'other-therapy',countable), guideline_cardinality(actual,D,na,eq,1), guideline_property(actual,E,ineffective,pos), guideline_event(actual,F,be), guideline_arg(actual,F,1,D), guideline_arg(actual,F,2,E).

% cdc2022-opioid-rec05-imp26.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
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
guideline_document('cdc2022-opioid-rec05-imp26',ace_sha256('9f3fb807e00941484f647cbbbb1986b61ea4188c3340fe600b62cefb694a3c64'),ulex(sha256('4a882a1996ea910df0fbf91fea897c0805e67d48f9e1ce2fcdbbbd9250c193ae'))).
% S1: A communication-principle is relevant during a shorter-duration-opioid-discontinuation.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',1,ref(1),[]),'communication-principle',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',1,ref(1),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',1,ref(2),[]),'shorter-duration-opioid-discontinuation',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',1,ref(2),[]),na,eq,1).
guideline_property(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',1,ref(3),[]),relevant,pos).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',1,ref(4),[]),be).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',1,ref(4),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',1,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',1,ref(4),[]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',1,ref(3),[])).
guideline_pp(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',1,ref(4),[]),during,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',1,ref(2),[])).
% S2: A pain-management-principle is relevant during a shorter-duration-opioid-discontinuation.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',2,ref(1),[]),'pain-management-principle',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',2,ref(1),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',2,ref(2),[]),'shorter-duration-opioid-discontinuation',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',2,ref(2),[]),na,eq,1).
guideline_property(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',2,ref(3),[]),relevant,pos).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',2,ref(4),[]),be).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',2,ref(4),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',2,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',2,ref(4),[]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',2,ref(3),[])).
guideline_pp(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',2,ref(4),[]),during,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',2,ref(2),[])).
% S3: A behavioral-support-principle is relevant during a shorter-duration-opioid-discontinuation.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',3,ref(1),[]),'behavioral-support-principle',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',3,ref(1),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',3,ref(2),[]),'shorter-duration-opioid-discontinuation',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',3,ref(2),[]),na,eq,1).
guideline_property(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',3,ref(3),[]),relevant,pos).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',3,ref(4),[]),be).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',3,ref(4),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',3,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',3,ref(4),[]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',3,ref(3),[])).
guideline_pp(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',3,ref(4),[]),during,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',3,ref(2),[])).
% S4: A slower-taper-principle is relevant during a shorter-duration-opioid-discontinuation.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',4,ref(1),[]),'slower-taper-principle',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',4,ref(1),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',4,ref(2),[]),'shorter-duration-opioid-discontinuation',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',4,ref(2),[]),na,eq,1).
guideline_property(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',4,ref(3),[]),relevant,pos).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',4,ref(4),[]),be).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',4,ref(4),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',4,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',4,ref(4),[]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',4,ref(3),[])).
guideline_pp(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',4,ref(4),[]),during,'$guideline_id'(product,'cdc2022-opioid-rec05-imp26',4,ref(2),[])).

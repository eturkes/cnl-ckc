% cdc2022-opioid-rec01-imp01.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-rec01-imp01',ace_sha256(f4387293da2e2da28fb16b535e771b839d1a13bc0ff79887e3582aa546af635b),ulex(sha256(fba48e003101de64df33400ab2595fa7589360ac19f4c77ac3d17bd085f5b8c0)),vocabulary(sha256('335c132fd0e3d3163ba40c255e7db41eecc36492d8098263c505b17f44bbef5a'))).
% S1: If a nonopioid-pharmacologic-therapy is appropriate for a specific-condition then every clinician should maximize the nonopioid-pharmacologic-therapy for the specific-condition.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec01-imp01',1,box(1),[A,B,C,D,E]),should) :- guideline_entity(actual,A,'nonopioid-pharmacologic-therapy',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'specific-condition',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,D,be), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,C), guideline_property(actual,C,appropriate,pos), guideline_qualifier(actual,D,applicability,appropriate), guideline_pp(actual,D,for,B), guideline_entity(actual,E,clinician,countable), guideline_cardinality(actual,E,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec01-imp01',1,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',1,ref(6),[A,B,C,D,E]),maximize) :- guideline_entity(actual,A,'nonopioid-pharmacologic-therapy',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'specific-condition',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,D,be), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,C), guideline_property(actual,C,appropriate,pos), guideline_qualifier(actual,D,applicability,appropriate), guideline_pp(actual,D,for,B), guideline_entity(actual,E,clinician,countable), guideline_cardinality(actual,E,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec01-imp01',1,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',1,ref(6),[A,B,C,D,E]),1,E) :- guideline_entity(actual,A,'nonopioid-pharmacologic-therapy',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'specific-condition',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,D,be), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,C), guideline_property(actual,C,appropriate,pos), guideline_qualifier(actual,D,applicability,appropriate), guideline_pp(actual,D,for,B), guideline_entity(actual,E,clinician,countable), guideline_cardinality(actual,E,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec01-imp01',1,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',1,ref(6),[A,B,C,D,E]),2,A) :- guideline_entity(actual,A,'nonopioid-pharmacologic-therapy',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'specific-condition',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,D,be), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,C), guideline_property(actual,C,appropriate,pos), guideline_qualifier(actual,D,applicability,appropriate), guideline_pp(actual,D,for,B), guideline_entity(actual,E,clinician,countable), guideline_cardinality(actual,E,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec01-imp01',1,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',1,ref(6),[A,B,C,D,E]),for,B) :- guideline_entity(actual,A,'nonopioid-pharmacologic-therapy',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'specific-condition',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,D,be), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,C), guideline_property(actual,C,appropriate,pos), guideline_qualifier(actual,D,applicability,appropriate), guideline_pp(actual,D,for,B), guideline_entity(actual,E,clinician,countable), guideline_cardinality(actual,E,na,eq,1).
% S2: If a nonpharmacologic-therapy is appropriate for a specific-condition then every clinician should maximize the nonpharmacologic-therapy for the specific-condition.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-rec01-imp01',2,box(1),[A,B,C,D,E]),should) :- guideline_entity(actual,A,'nonpharmacologic-therapy',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'specific-condition',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,D,be), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,C), guideline_property(actual,C,appropriate,pos), guideline_qualifier(actual,D,applicability,appropriate), guideline_pp(actual,D,for,B), guideline_entity(actual,E,clinician,countable), guideline_cardinality(actual,E,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_event('$guideline_id'(context,'cdc2022-opioid-rec01-imp01',2,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',2,ref(6),[A,B,C,D,E]),maximize) :- guideline_entity(actual,A,'nonpharmacologic-therapy',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'specific-condition',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,D,be), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,C), guideline_property(actual,C,appropriate,pos), guideline_qualifier(actual,D,applicability,appropriate), guideline_pp(actual,D,for,B), guideline_entity(actual,E,clinician,countable), guideline_cardinality(actual,E,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec01-imp01',2,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',2,ref(6),[A,B,C,D,E]),1,E) :- guideline_entity(actual,A,'nonpharmacologic-therapy',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'specific-condition',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,D,be), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,C), guideline_property(actual,C,appropriate,pos), guideline_qualifier(actual,D,applicability,appropriate), guideline_pp(actual,D,for,B), guideline_entity(actual,E,clinician,countable), guideline_cardinality(actual,E,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-rec01-imp01',2,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',2,ref(6),[A,B,C,D,E]),2,A) :- guideline_entity(actual,A,'nonpharmacologic-therapy',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'specific-condition',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,D,be), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,C), guideline_property(actual,C,appropriate,pos), guideline_qualifier(actual,D,applicability,appropriate), guideline_pp(actual,D,for,B), guideline_entity(actual,E,clinician,countable), guideline_cardinality(actual,E,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-rec01-imp01',2,box(1),[A,B,C,D,E]),'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',2,ref(6),[A,B,C,D,E]),for,B) :- guideline_entity(actual,A,'nonpharmacologic-therapy',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'specific-condition',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,D,be), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,C), guideline_property(actual,C,appropriate,pos), guideline_qualifier(actual,D,applicability,appropriate), guideline_pp(actual,D,for,B), guideline_entity(actual,E,clinician,countable), guideline_cardinality(actual,E,na,eq,1).
% S3: A nonopioid-pharmacologic-therapy-example includes a topical-NSAID-example and includes an oral-NSAID-example and includes an acetaminophen-example.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',3,ref(1),[]),'nonopioid-pharmacologic-therapy-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',3,ref(1),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',3,ref(2),[]),'topical-NSAID-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',3,ref(2),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',3,ref(3),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',3,ref(3),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',3,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',3,ref(3),[]),2,'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',3,ref(2),[])).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',3,ref(4),[]),'oral-NSAID-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',3,ref(4),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',3,ref(5),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',3,ref(5),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',3,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',3,ref(5),[]),2,'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',3,ref(4),[])).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',3,ref(6),[]),'acetaminophen-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',3,ref(6),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',3,ref(7),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',3,ref(7),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',3,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',3,ref(7),[]),2,'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',3,ref(6),[])).
% S4: A nonpharmacologic-therapy-example includes an ice-example and includes a heat-example and includes an elevation-example and includes a rest-example and includes an immobilization-example and includes an exercise-example.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',4,ref(1),[]),'nonpharmacologic-therapy-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',4,ref(1),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',4,ref(2),[]),'ice-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',4,ref(2),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',4,ref(3),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',4,ref(3),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',4,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',4,ref(3),[]),2,'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',4,ref(2),[])).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',4,ref(4),[]),'heat-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',4,ref(4),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',4,ref(5),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',4,ref(5),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',4,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',4,ref(5),[]),2,'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',4,ref(4),[])).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',4,ref(6),[]),'elevation-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',4,ref(6),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',4,ref(7),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',4,ref(7),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',4,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',4,ref(7),[]),2,'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',4,ref(6),[])).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',4,ref(8),[]),'rest-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',4,ref(8),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',4,ref(9),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',4,ref(9),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',4,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',4,ref(9),[]),2,'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',4,ref(8),[])).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',4,ref(10),[]),'immobilization-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',4,ref(10),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',4,ref(11),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',4,ref(11),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',4,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',4,ref(11),[]),2,'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',4,ref(10),[])).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',4,ref(12),[]),'exercise-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',4,ref(12),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',4,ref(13),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',4,ref(13),[]),1,'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',4,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',4,ref(13),[]),2,'$guideline_id'(product,'cdc2022-opioid-rec01-imp01',4,ref(12),[])).

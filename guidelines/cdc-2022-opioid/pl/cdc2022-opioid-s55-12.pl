% cdc2022-opioid-s55-12.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s55-12',ace_sha256(e7936ea634b3d60a9bc99783f1e636359a442fbb3228438702a9d11af01cbaa4),ulex(sha256('2733db8c734f4672e1d598cc0911538cb926c6072c8cc42898ec8aa2d7176379')),vocabulary(sha256('335c132fd0e3d3163ba40c255e7db41eecc36492d8098263c505b17f44bbef5a'))).
% S1: Every central-nervous-system-depressant can potentiate an opioid-associated-respiratory-depression.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s55-12',1,box(1),[A]),can) :- guideline_entity(actual,A,'central-nervous-system-depressant',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s55-12',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s55-12',1,ref(2),[A]),'opioid-associated-respiratory-depression',countable) :- guideline_entity(actual,A,'central-nervous-system-depressant',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s55-12',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s55-12',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'central-nervous-system-depressant',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s55-12',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s55-12',1,ref(3),[A]),potentiate) :- guideline_entity(actual,A,'central-nervous-system-depressant',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s55-12',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s55-12',1,ref(3),[A]),1,A) :- guideline_entity(actual,A,'central-nervous-system-depressant',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s55-12',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s55-12',1,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s55-12',1,ref(2),[A])) :- guideline_entity(actual,A,'central-nervous-system-depressant',countable), guideline_cardinality(actual,A,na,eq,1).
% S2: If a clinician prescribes an opioid with a central-nervous-system-depressant and the central-nervous-system-depressant is not a benzodiazepine then the clinician should weigh a benefit against a risk.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s55-12',2,box(2),[A,B,C,D]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'central-nervous-system-depressant',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,prescribe), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,with,C), guideline_entity(E,F,benzodiazepine,countable), guideline_operator(actual,E,-), guideline_cardinality(E,F,na,eq,1), guideline_event(E,G,be), guideline_arg(E,G,1,C), guideline_arg(E,G,2,F), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s55-12',2,box(2),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s55-12',2,ref(7),[A,B,C,D]),benefit,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'central-nervous-system-depressant',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,prescribe), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,with,C), guideline_entity(E,F,benzodiazepine,countable), guideline_operator(actual,E,-), guideline_cardinality(E,F,na,eq,1), guideline_event(E,G,be), guideline_arg(E,G,1,C), guideline_arg(E,G,2,F).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s55-12',2,box(2),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s55-12',2,ref(7),[A,B,C,D]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'central-nervous-system-depressant',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,prescribe), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,with,C), guideline_entity(E,F,benzodiazepine,countable), guideline_operator(actual,E,-), guideline_cardinality(E,F,na,eq,1), guideline_event(E,G,be), guideline_arg(E,G,1,C), guideline_arg(E,G,2,F).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s55-12',2,box(2),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s55-12',2,ref(8),[A,B,C,D]),risk,countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'central-nervous-system-depressant',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,prescribe), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,with,C), guideline_entity(E,F,benzodiazepine,countable), guideline_operator(actual,E,-), guideline_cardinality(E,F,na,eq,1), guideline_event(E,G,be), guideline_arg(E,G,1,C), guideline_arg(E,G,2,F).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s55-12',2,box(2),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s55-12',2,ref(8),[A,B,C,D]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'central-nervous-system-depressant',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,prescribe), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,with,C), guideline_entity(E,F,benzodiazepine,countable), guideline_operator(actual,E,-), guideline_cardinality(E,F,na,eq,1), guideline_event(E,G,be), guideline_arg(E,G,1,C), guideline_arg(E,G,2,F).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s55-12',2,box(2),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s55-12',2,ref(9),[A,B,C,D]),weigh) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'central-nervous-system-depressant',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,prescribe), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,with,C), guideline_entity(E,F,benzodiazepine,countable), guideline_operator(actual,E,-), guideline_cardinality(E,F,na,eq,1), guideline_event(E,G,be), guideline_arg(E,G,1,C), guideline_arg(E,G,2,F).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s55-12',2,box(2),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s55-12',2,ref(9),[A,B,C,D]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'central-nervous-system-depressant',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,prescribe), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,with,C), guideline_entity(E,F,benzodiazepine,countable), guideline_operator(actual,E,-), guideline_cardinality(E,F,na,eq,1), guideline_event(E,G,be), guideline_arg(E,G,1,C), guideline_arg(E,G,2,F).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s55-12',2,box(2),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s55-12',2,ref(9),[A,B,C,D]),2,'$guideline_id'(product,'cdc2022-opioid-s55-12',2,ref(7),[A,B,C,D])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'central-nervous-system-depressant',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,prescribe), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,with,C), guideline_entity(E,F,benzodiazepine,countable), guideline_operator(actual,E,-), guideline_cardinality(E,F,na,eq,1), guideline_event(E,G,be), guideline_arg(E,G,1,C), guideline_arg(E,G,2,F).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s55-12',2,box(2),[A,B,C,D]),'$guideline_id'(product,'cdc2022-opioid-s55-12',2,ref(9),[A,B,C,D]),against,'$guideline_id'(product,'cdc2022-opioid-s55-12',2,ref(8),[A,B,C,D])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,opioid,countable), guideline_cardinality(actual,B,na,eq,1), guideline_entity(actual,C,'central-nervous-system-depressant',countable), guideline_cardinality(actual,C,na,eq,1), guideline_event(actual,D,prescribe), guideline_arg(actual,D,1,A), guideline_arg(actual,D,2,B), guideline_pp(actual,D,with,C), guideline_entity(E,F,benzodiazepine,countable), guideline_operator(actual,E,-), guideline_cardinality(E,F,na,eq,1), guideline_event(E,G,be), guideline_arg(E,G,1,C), guideline_arg(E,G,2,F).
% S3: A central-nervous-system-depressant-example includes a muscle-relaxant-example and includes a nonbenzodiazepine-sedative-hypnotic-example and includes a potentially-sedating-anticonvulsant-example.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s55-12',3,ref(1),[]),'central-nervous-system-depressant-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s55-12',3,ref(1),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s55-12',3,ref(2),[]),'muscle-relaxant-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s55-12',3,ref(2),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s55-12',3,ref(3),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s55-12',3,ref(3),[]),1,'$guideline_id'(product,'cdc2022-opioid-s55-12',3,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s55-12',3,ref(3),[]),2,'$guideline_id'(product,'cdc2022-opioid-s55-12',3,ref(2),[])).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s55-12',3,ref(4),[]),'nonbenzodiazepine-sedative-hypnotic-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s55-12',3,ref(4),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s55-12',3,ref(5),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s55-12',3,ref(5),[]),1,'$guideline_id'(product,'cdc2022-opioid-s55-12',3,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s55-12',3,ref(5),[]),2,'$guideline_id'(product,'cdc2022-opioid-s55-12',3,ref(4),[])).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s55-12',3,ref(6),[]),'potentially-sedating-anticonvulsant-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s55-12',3,ref(6),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s55-12',3,ref(7),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s55-12',3,ref(7),[]),1,'$guideline_id'(product,'cdc2022-opioid-s55-12',3,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s55-12',3,ref(7),[]),2,'$guideline_id'(product,'cdc2022-opioid-s55-12',3,ref(6),[])).
% S4: A potentially-sedating-anticonvulsant-example includes a gabapentin-example and includes a pregabalin-example.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s55-12',4,ref(1),[]),'potentially-sedating-anticonvulsant-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s55-12',4,ref(1),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s55-12',4,ref(2),[]),'gabapentin-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s55-12',4,ref(2),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s55-12',4,ref(3),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s55-12',4,ref(3),[]),1,'$guideline_id'(product,'cdc2022-opioid-s55-12',4,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s55-12',4,ref(3),[]),2,'$guideline_id'(product,'cdc2022-opioid-s55-12',4,ref(2),[])).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s55-12',4,ref(4),[]),'pregabalin-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s55-12',4,ref(4),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s55-12',4,ref(5),[]),include).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s55-12',4,ref(5),[]),1,'$guideline_id'(product,'cdc2022-opioid-s55-12',4,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s55-12',4,ref(5),[]),2,'$guideline_id'(product,'cdc2022-opioid-s55-12',4,ref(4),[])).

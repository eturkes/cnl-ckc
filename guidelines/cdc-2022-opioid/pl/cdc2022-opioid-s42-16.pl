% cdc2022-opioid-s42-16.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s42-16',ace_sha256(fd0817dd8cb9359342236690d3aa9038e5d861b0be7485325c22e56aa93617fd),ulex(sha256('2733db8c734f4672e1d598cc0911538cb926c6072c8cc42898ec8aa2d7176379')),vocabulary(sha256('335c132fd0e3d3163ba40c255e7db41eecc36492d8098263c505b17f44bbef5a'))).
% S1: If a patient has a continuous-opioid-use that lasts for at least 1 week for less than 1 month then every clinician may consider a gradual-taper for the patient.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s42-16',1,box(1),[A,B,C,D,E,F,G]),may) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'continuous-opioid-use',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,E,last), guideline_pp(actual,E,for,C), guideline_cardinality(actual,C,na,H,I), guideline_unit(actual,C,J), guideline_satisfies(H,I,J,geq,1,week), guideline_interval(actual,E,duration,C,J,none), guideline_pp(actual,E,for,D), guideline_interval(actual,E,duration,D,K,none), guideline_arg(actual,E,1,B), guideline_cardinality(actual,D,na,L,M), guideline_unit(actual,D,K), guideline_satisfies(L,M,K,less,1,month), guideline_event(actual,F,have), guideline_arg(actual,F,1,A), guideline_arg(actual,F,2,B), guideline_entity(actual,G,clinician,countable), guideline_cardinality(actual,G,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s42-16',1,box(1),[A,B,C,D,E,F,G]),'$guideline_id'(product,'cdc2022-opioid-s42-16',1,ref(8),[A,B,C,D,E,F,G]),'gradual-taper',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'continuous-opioid-use',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,E,last), guideline_pp(actual,E,for,C), guideline_cardinality(actual,C,na,H,I), guideline_unit(actual,C,J), guideline_satisfies(H,I,J,geq,1,week), guideline_interval(actual,E,duration,C,J,none), guideline_pp(actual,E,for,D), guideline_interval(actual,E,duration,D,K,none), guideline_arg(actual,E,1,B), guideline_cardinality(actual,D,na,L,M), guideline_unit(actual,D,K), guideline_satisfies(L,M,K,less,1,month), guideline_event(actual,F,have), guideline_arg(actual,F,1,A), guideline_arg(actual,F,2,B), guideline_entity(actual,G,clinician,countable), guideline_cardinality(actual,G,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s42-16',1,box(1),[A,B,C,D,E,F,G]),'$guideline_id'(product,'cdc2022-opioid-s42-16',1,ref(8),[A,B,C,D,E,F,G]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'continuous-opioid-use',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,E,last), guideline_pp(actual,E,for,C), guideline_cardinality(actual,C,na,H,I), guideline_unit(actual,C,J), guideline_satisfies(H,I,J,geq,1,week), guideline_interval(actual,E,duration,C,J,none), guideline_pp(actual,E,for,D), guideline_interval(actual,E,duration,D,K,none), guideline_arg(actual,E,1,B), guideline_cardinality(actual,D,na,L,M), guideline_unit(actual,D,K), guideline_satisfies(L,M,K,less,1,month), guideline_event(actual,F,have), guideline_arg(actual,F,1,A), guideline_arg(actual,F,2,B), guideline_entity(actual,G,clinician,countable), guideline_cardinality(actual,G,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s42-16',1,box(1),[A,B,C,D,E,F,G]),'$guideline_id'(product,'cdc2022-opioid-s42-16',1,ref(9),[A,B,C,D,E,F,G]),consider) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'continuous-opioid-use',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,E,last), guideline_pp(actual,E,for,C), guideline_cardinality(actual,C,na,H,I), guideline_unit(actual,C,J), guideline_satisfies(H,I,J,geq,1,week), guideline_interval(actual,E,duration,C,J,none), guideline_pp(actual,E,for,D), guideline_interval(actual,E,duration,D,K,none), guideline_arg(actual,E,1,B), guideline_cardinality(actual,D,na,L,M), guideline_unit(actual,D,K), guideline_satisfies(L,M,K,less,1,month), guideline_event(actual,F,have), guideline_arg(actual,F,1,A), guideline_arg(actual,F,2,B), guideline_entity(actual,G,clinician,countable), guideline_cardinality(actual,G,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s42-16',1,box(1),[A,B,C,D,E,F,G]),'$guideline_id'(product,'cdc2022-opioid-s42-16',1,ref(9),[A,B,C,D,E,F,G]),1,G) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'continuous-opioid-use',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,E,last), guideline_pp(actual,E,for,C), guideline_cardinality(actual,C,na,H,I), guideline_unit(actual,C,J), guideline_satisfies(H,I,J,geq,1,week), guideline_interval(actual,E,duration,C,J,none), guideline_pp(actual,E,for,D), guideline_interval(actual,E,duration,D,K,none), guideline_arg(actual,E,1,B), guideline_cardinality(actual,D,na,L,M), guideline_unit(actual,D,K), guideline_satisfies(L,M,K,less,1,month), guideline_event(actual,F,have), guideline_arg(actual,F,1,A), guideline_arg(actual,F,2,B), guideline_entity(actual,G,clinician,countable), guideline_cardinality(actual,G,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s42-16',1,box(1),[A,B,C,D,E,F,G]),'$guideline_id'(product,'cdc2022-opioid-s42-16',1,ref(9),[A,B,C,D,E,F,G]),2,'$guideline_id'(product,'cdc2022-opioid-s42-16',1,ref(8),[A,B,C,D,E,F,G])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'continuous-opioid-use',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,E,last), guideline_pp(actual,E,for,C), guideline_cardinality(actual,C,na,H,I), guideline_unit(actual,C,J), guideline_satisfies(H,I,J,geq,1,week), guideline_interval(actual,E,duration,C,J,none), guideline_pp(actual,E,for,D), guideline_interval(actual,E,duration,D,K,none), guideline_arg(actual,E,1,B), guideline_cardinality(actual,D,na,L,M), guideline_unit(actual,D,K), guideline_satisfies(L,M,K,less,1,month), guideline_event(actual,F,have), guideline_arg(actual,F,1,A), guideline_arg(actual,F,2,B), guideline_entity(actual,G,clinician,countable), guideline_cardinality(actual,G,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s42-16',1,box(1),[A,B,C,D,E,F,G]),'$guideline_id'(product,'cdc2022-opioid-s42-16',1,ref(9),[A,B,C,D,E,F,G]),for,A) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'continuous-opioid-use',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,E,last), guideline_pp(actual,E,for,C), guideline_cardinality(actual,C,na,H,I), guideline_unit(actual,C,J), guideline_satisfies(H,I,J,geq,1,week), guideline_interval(actual,E,duration,C,J,none), guideline_pp(actual,E,for,D), guideline_interval(actual,E,duration,D,K,none), guideline_arg(actual,E,1,B), guideline_cardinality(actual,D,na,L,M), guideline_unit(actual,D,K), guideline_satisfies(L,M,K,less,1,month), guideline_event(actual,F,have), guideline_arg(actual,F,1,A), guideline_arg(actual,F,2,B), guideline_entity(actual,G,clinician,countable), guideline_cardinality(actual,G,na,eq,1).
% S2: A gradual-taper-example reduces a daily-opioid-dosage-example by 20 approximate percent at an interval of exactly 2 days.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s42-16',2,ref(1),[]),'gradual-taper-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s42-16',2,ref(1),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s42-16',2,ref(2),[]),'daily-opioid-dosage-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s42-16',2,ref(2),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s42-16',2,ref(3),[]),percent,countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s42-16',2,ref(3),[]),na,eq,20).
guideline_property(actual,'$guideline_id'(product,'cdc2022-opioid-s42-16',2,ref(3),[]),approximate,pos).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s42-16',2,ref(4),[]),interval,countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s42-16',2,ref(4),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s42-16',2,ref(5),[]),day,countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s42-16',2,ref(5),[]),na,exactly,2).
guideline_unit(actual,'$guideline_id'(product,'cdc2022-opioid-s42-16',2,ref(5),[]),day).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s42-16',2,ref(6),[]),reduce).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s42-16',2,ref(6),[]),1,'$guideline_id'(product,'cdc2022-opioid-s42-16',2,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s42-16',2,ref(6),[]),2,'$guideline_id'(product,'cdc2022-opioid-s42-16',2,ref(2),[])).
guideline_pp(actual,'$guideline_id'(product,'cdc2022-opioid-s42-16',2,ref(6),[]),at,'$guideline_id'(product,'cdc2022-opioid-s42-16',2,ref(4),[])).
guideline_recurrence(actual,'$guideline_id'(product,'cdc2022-opioid-s42-16',2,ref(6),[]),'$guideline_id'(product,'cdc2022-opioid-s42-16',2,ref(5),[]),day).
guideline_pp(actual,'$guideline_id'(product,'cdc2022-opioid-s42-16',2,ref(6),[]),by,'$guideline_id'(product,'cdc2022-opioid-s42-16',2,ref(3),[])).

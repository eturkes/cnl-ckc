% world.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document(world,ace_sha256(c23d479f9176e2d39175abfee92ec3021849b07eee4abf88685cc22d8439c52b),ulex(sha256(ea08df2c0364239536194c128dd430bbd88114b5508f5ab45882331c92505c99)),vocabulary(sha256('05eeed04deb22824331a3748ab69daba7285d901ff655fc1df56792e87853e5a'))).
% S1: A clinician prescribes an opioid to a patient.
guideline_entity(actual,'$guideline_id'(product,world,1,ref(1),[]),clinician,countable).
guideline_cardinality(actual,'$guideline_id'(product,world,1,ref(1),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,world,1,ref(2),[]),opioid,countable).
guideline_cardinality(actual,'$guideline_id'(product,world,1,ref(2),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,world,1,ref(3),[]),patient,countable).
guideline_cardinality(actual,'$guideline_id'(product,world,1,ref(3),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,world,1,ref(4),[]),prescribe).
guideline_arg(actual,'$guideline_id'(product,world,1,ref(4),[]),1,'$guideline_id'(product,world,1,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,world,1,ref(4),[]),2,'$guideline_id'(product,world,1,ref(2),[])).
guideline_pp(actual,'$guideline_id'(product,world,1,ref(4),[]),to,'$guideline_id'(product,world,1,ref(3),[])).
% S2: The patient has an opioid-dosage.
guideline_entity(actual,'$guideline_id'(product,world,2,ref(1),[]),'opioid-dosage',countable).
guideline_cardinality(actual,'$guideline_id'(product,world,2,ref(1),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,world,2,ref(2),[]),have).
guideline_arg(actual,'$guideline_id'(product,world,2,ref(2),[]),1,'$guideline_id'(product,world,1,ref(3),[])).
guideline_arg(actual,'$guideline_id'(product,world,2,ref(2),[]),2,'$guideline_id'(product,world,2,ref(1),[])).
% S3: The opioid-dosage contains exactly 40 oxycodone-milligrams-per-day.
guideline_event(actual,'$guideline_id'(product,world,3,ref(1),[]),contain).
guideline_arg(actual,'$guideline_id'(product,world,3,ref(1),[]),1,'$guideline_id'(product,world,2,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,world,3,ref(1),[]),2,'$guideline_id'(product,world,3,ref(3),[])).
guideline_entity(actual,'$guideline_id'(product,world,3,ref(3),[]),'oxycodone-milligram-per-day',countable).
guideline_cardinality(actual,'$guideline_id'(product,world,3,ref(3),[]),na,exactly,40).

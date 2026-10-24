% cdc2022-opioid-s22-15.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s22-15',ace_sha256(cc10a3b9277ef5ad7ce9c6df4c2829838f234abb6ad81b4b2b52df292201de3a),ulex(sha256('45608095e3f77853fdabcce148a9eb7633a8ae776353b8f008c753b846886588')),vocabulary(sha256(c36289aa40f75805a83501ca057b29dd1ba7292b025716853f654890b77ec50c))).
% S1: Every new-long-term-opioid-use-risk-finding is a previously-opioid-naive-patient-specific-finding.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s22-15',1,ref(2),[A]),'previously-opioid-naive-patient-specific-finding',countable) :- guideline_entity(actual,A,'new-long-term-opioid-use-risk-finding',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s22-15',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'new-long-term-opioid-use-risk-finding',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s22-15',1,ref(3),[A]),be) :- guideline_entity(actual,A,'new-long-term-opioid-use-risk-finding',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s22-15',1,ref(3),[A]),1,A) :- guideline_entity(actual,A,'new-long-term-opioid-use-risk-finding',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s22-15',1,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s22-15',1,ref(2),[A])) :- guideline_entity(actual,A,'new-long-term-opioid-use-risk-finding',countable), guideline_cardinality(actual,A,na,eq,1).
% S2: If a patient receives a long-term-opioid-therapy and a clinician prescribes an additional-opioid-medication to the patient for a superimposed-new-acute-pain then the additional-opioid-medication can possibly carry a dosage-escalation-risk.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s22-15',2,box(1),[A,B,C,D,E,F,G]),can) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'long-term-opioid-therapy',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,receive), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1), guideline_entity(actual,E,'additional-opioid-medication',countable), guideline_cardinality(actual,E,na,eq,1), guideline_entity(actual,F,'superimposed-new-acute-pain',countable), guideline_cardinality(actual,F,na,eq,1), guideline_event(actual,G,prescribe), guideline_arg(actual,G,1,D), guideline_arg(actual,G,2,E), guideline_pp(actual,G,for,F), guideline_pp(actual,G,to,A).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s22-15',2,box(1),[A,B,C,D,E,F,G]),'$guideline_id'(product,'cdc2022-opioid-s22-15',2,ref(8),[A,B,C,D,E,F,G]),'dosage-escalation-risk',countable) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'long-term-opioid-therapy',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,receive), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1), guideline_entity(actual,E,'additional-opioid-medication',countable), guideline_cardinality(actual,E,na,eq,1), guideline_entity(actual,F,'superimposed-new-acute-pain',countable), guideline_cardinality(actual,F,na,eq,1), guideline_event(actual,G,prescribe), guideline_arg(actual,G,1,D), guideline_arg(actual,G,2,E), guideline_pp(actual,G,for,F), guideline_pp(actual,G,to,A).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s22-15',2,box(1),[A,B,C,D,E,F,G]),'$guideline_id'(product,'cdc2022-opioid-s22-15',2,ref(8),[A,B,C,D,E,F,G]),na,eq,1) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'long-term-opioid-therapy',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,receive), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1), guideline_entity(actual,E,'additional-opioid-medication',countable), guideline_cardinality(actual,E,na,eq,1), guideline_entity(actual,F,'superimposed-new-acute-pain',countable), guideline_cardinality(actual,F,na,eq,1), guideline_event(actual,G,prescribe), guideline_arg(actual,G,1,D), guideline_arg(actual,G,2,E), guideline_pp(actual,G,for,F), guideline_pp(actual,G,to,A).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-s22-15',2,box(1),[A,B,C,D,E,F,G]),'$guideline_id'(product,'cdc2022-opioid-s22-15',2,ref(9),[A,B,C,D,E,F,G]),possibility,possibly) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'long-term-opioid-therapy',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,receive), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1), guideline_entity(actual,E,'additional-opioid-medication',countable), guideline_cardinality(actual,E,na,eq,1), guideline_entity(actual,F,'superimposed-new-acute-pain',countable), guideline_cardinality(actual,F,na,eq,1), guideline_event(actual,G,prescribe), guideline_arg(actual,G,1,D), guideline_arg(actual,G,2,E), guideline_pp(actual,G,for,F), guideline_pp(actual,G,to,A).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s22-15',2,box(1),[A,B,C,D,E,F,G]),'$guideline_id'(product,'cdc2022-opioid-s22-15',2,ref(9),[A,B,C,D,E,F,G]),carry) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'long-term-opioid-therapy',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,receive), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1), guideline_entity(actual,E,'additional-opioid-medication',countable), guideline_cardinality(actual,E,na,eq,1), guideline_entity(actual,F,'superimposed-new-acute-pain',countable), guideline_cardinality(actual,F,na,eq,1), guideline_event(actual,G,prescribe), guideline_arg(actual,G,1,D), guideline_arg(actual,G,2,E), guideline_pp(actual,G,for,F), guideline_pp(actual,G,to,A).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s22-15',2,box(1),[A,B,C,D,E,F,G]),'$guideline_id'(product,'cdc2022-opioid-s22-15',2,ref(9),[A,B,C,D,E,F,G]),1,E) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'long-term-opioid-therapy',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,receive), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1), guideline_entity(actual,E,'additional-opioid-medication',countable), guideline_cardinality(actual,E,na,eq,1), guideline_entity(actual,F,'superimposed-new-acute-pain',countable), guideline_cardinality(actual,F,na,eq,1), guideline_event(actual,G,prescribe), guideline_arg(actual,G,1,D), guideline_arg(actual,G,2,E), guideline_pp(actual,G,for,F), guideline_pp(actual,G,to,A).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s22-15',2,box(1),[A,B,C,D,E,F,G]),'$guideline_id'(product,'cdc2022-opioid-s22-15',2,ref(9),[A,B,C,D,E,F,G]),2,'$guideline_id'(product,'cdc2022-opioid-s22-15',2,ref(8),[A,B,C,D,E,F,G])) :- guideline_entity(actual,A,patient,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'long-term-opioid-therapy',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,receive), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_entity(actual,D,clinician,countable), guideline_cardinality(actual,D,na,eq,1), guideline_entity(actual,E,'additional-opioid-medication',countable), guideline_cardinality(actual,E,na,eq,1), guideline_entity(actual,F,'superimposed-new-acute-pain',countable), guideline_cardinality(actual,F,na,eq,1), guideline_event(actual,G,prescribe), guideline_arg(actual,G,1,D), guideline_arg(actual,G,2,E), guideline_pp(actual,G,for,F), guideline_pp(actual,G,to,A).

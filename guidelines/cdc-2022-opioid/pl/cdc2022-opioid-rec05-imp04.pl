% cdc2022-opioid-rec05-imp04.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-rec05-imp04',ace_sha256(cf99c16b55b526d00df1e746b27f470fc2e795b50c0a1bfcd49faf9d9d63e9b5),ulex(sha256('64c086862c2f48d491a9fde0ba9f0fcf090e25e84456b9749332ec3ace211ce4')),vocabulary(sha256(c36289aa40f75805a83501ca057b29dd1ba7292b025716853f654890b77ec50c))).
% S1: If a continued-opioid-benefit matches a continued-opioid-risk then a shared-decision-making particularly is important with a patient.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp04',1,ref(4),[A,B,C]),'shared-decision-making',countable) :- guideline_entity(actual,A,'continued-opioid-benefit',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'continued-opioid-risk',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,match), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp04',1,ref(4),[A,B,C]),na,eq,1) :- guideline_entity(actual,A,'continued-opioid-benefit',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'continued-opioid-risk',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,match), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp04',1,ref(5),[A,B,C]),patient,countable) :- guideline_entity(actual,A,'continued-opioid-benefit',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'continued-opioid-risk',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,match), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp04',1,ref(5),[A,B,C]),na,eq,1) :- guideline_entity(actual,A,'continued-opioid-benefit',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'continued-opioid-risk',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,match), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_qualifier(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp04',1,ref(7),[A,B,C]),focus,particularly) :- guideline_entity(actual,A,'continued-opioid-benefit',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'continued-opioid-risk',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,match), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_property(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp04',1,ref(6),[A,B,C]),important,pos) :- guideline_entity(actual,A,'continued-opioid-benefit',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'continued-opioid-risk',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,match), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp04',1,ref(7),[A,B,C]),be) :- guideline_entity(actual,A,'continued-opioid-benefit',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'continued-opioid-risk',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,match), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp04',1,ref(7),[A,B,C]),1,'$guideline_id'(product,'cdc2022-opioid-rec05-imp04',1,ref(4),[A,B,C])) :- guideline_entity(actual,A,'continued-opioid-benefit',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'continued-opioid-risk',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,match), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp04',1,ref(7),[A,B,C]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp04',1,ref(6),[A,B,C])) :- guideline_entity(actual,A,'continued-opioid-benefit',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'continued-opioid-risk',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,match), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
guideline_pp(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp04',1,ref(7),[A,B,C]),with,'$guideline_id'(product,'cdc2022-opioid-rec05-imp04',1,ref(5),[A,B,C])) :- guideline_entity(actual,A,'continued-opioid-benefit',countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,'continued-opioid-risk',countable), guideline_cardinality(actual,B,na,eq,1), guideline_event(actual,C,match), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B).
% S2: If a continued-opioid-benefit is unclear and a continued-opioid-risk is unclear then a shared-decision-making particularly is important with a patient.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp04',2,ref(7),[A,B,C,D,E,F]),'shared-decision-making',countable) :- guideline_entity(actual,A,'continued-opioid-benefit',countable), guideline_cardinality(actual,A,na,eq,1), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_property(actual,B,unclear,pos), guideline_entity(actual,D,'continued-opioid-risk',countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,F,be), guideline_arg(actual,F,1,D), guideline_arg(actual,F,2,E), guideline_property(actual,E,unclear,pos).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp04',2,ref(7),[A,B,C,D,E,F]),na,eq,1) :- guideline_entity(actual,A,'continued-opioid-benefit',countable), guideline_cardinality(actual,A,na,eq,1), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_property(actual,B,unclear,pos), guideline_entity(actual,D,'continued-opioid-risk',countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,F,be), guideline_arg(actual,F,1,D), guideline_arg(actual,F,2,E), guideline_property(actual,E,unclear,pos).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp04',2,ref(8),[A,B,C,D,E,F]),patient,countable) :- guideline_entity(actual,A,'continued-opioid-benefit',countable), guideline_cardinality(actual,A,na,eq,1), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_property(actual,B,unclear,pos), guideline_entity(actual,D,'continued-opioid-risk',countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,F,be), guideline_arg(actual,F,1,D), guideline_arg(actual,F,2,E), guideline_property(actual,E,unclear,pos).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp04',2,ref(8),[A,B,C,D,E,F]),na,eq,1) :- guideline_entity(actual,A,'continued-opioid-benefit',countable), guideline_cardinality(actual,A,na,eq,1), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_property(actual,B,unclear,pos), guideline_entity(actual,D,'continued-opioid-risk',countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,F,be), guideline_arg(actual,F,1,D), guideline_arg(actual,F,2,E), guideline_property(actual,E,unclear,pos).
guideline_qualifier(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp04',2,ref(10),[A,B,C,D,E,F]),focus,particularly) :- guideline_entity(actual,A,'continued-opioid-benefit',countable), guideline_cardinality(actual,A,na,eq,1), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_property(actual,B,unclear,pos), guideline_entity(actual,D,'continued-opioid-risk',countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,F,be), guideline_arg(actual,F,1,D), guideline_arg(actual,F,2,E), guideline_property(actual,E,unclear,pos).
guideline_property(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp04',2,ref(9),[A,B,C,D,E,F]),important,pos) :- guideline_entity(actual,A,'continued-opioid-benefit',countable), guideline_cardinality(actual,A,na,eq,1), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_property(actual,B,unclear,pos), guideline_entity(actual,D,'continued-opioid-risk',countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,F,be), guideline_arg(actual,F,1,D), guideline_arg(actual,F,2,E), guideline_property(actual,E,unclear,pos).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp04',2,ref(10),[A,B,C,D,E,F]),be) :- guideline_entity(actual,A,'continued-opioid-benefit',countable), guideline_cardinality(actual,A,na,eq,1), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_property(actual,B,unclear,pos), guideline_entity(actual,D,'continued-opioid-risk',countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,F,be), guideline_arg(actual,F,1,D), guideline_arg(actual,F,2,E), guideline_property(actual,E,unclear,pos).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp04',2,ref(10),[A,B,C,D,E,F]),1,'$guideline_id'(product,'cdc2022-opioid-rec05-imp04',2,ref(7),[A,B,C,D,E,F])) :- guideline_entity(actual,A,'continued-opioid-benefit',countable), guideline_cardinality(actual,A,na,eq,1), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_property(actual,B,unclear,pos), guideline_entity(actual,D,'continued-opioid-risk',countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,F,be), guideline_arg(actual,F,1,D), guideline_arg(actual,F,2,E), guideline_property(actual,E,unclear,pos).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp04',2,ref(10),[A,B,C,D,E,F]),2,'$guideline_id'(product,'cdc2022-opioid-rec05-imp04',2,ref(9),[A,B,C,D,E,F])) :- guideline_entity(actual,A,'continued-opioid-benefit',countable), guideline_cardinality(actual,A,na,eq,1), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_property(actual,B,unclear,pos), guideline_entity(actual,D,'continued-opioid-risk',countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,F,be), guideline_arg(actual,F,1,D), guideline_arg(actual,F,2,E), guideline_property(actual,E,unclear,pos).
guideline_pp(actual,'$guideline_id'(product,'cdc2022-opioid-rec05-imp04',2,ref(10),[A,B,C,D,E,F]),with,'$guideline_id'(product,'cdc2022-opioid-rec05-imp04',2,ref(8),[A,B,C,D,E,F])) :- guideline_entity(actual,A,'continued-opioid-benefit',countable), guideline_cardinality(actual,A,na,eq,1), guideline_event(actual,C,be), guideline_arg(actual,C,1,A), guideline_arg(actual,C,2,B), guideline_property(actual,B,unclear,pos), guideline_entity(actual,D,'continued-opioid-risk',countable), guideline_cardinality(actual,D,na,eq,1), guideline_event(actual,F,be), guideline_arg(actual,F,1,D), guideline_arg(actual,F,2,E), guideline_property(actual,E,unclear,pos).

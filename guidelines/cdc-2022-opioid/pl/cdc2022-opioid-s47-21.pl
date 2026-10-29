% cdc2022-opioid-s47-21.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s47-21',ace_sha256(d00286f2eb4df53071e9cb8f7de4823177582a9e3edc9d2c42690a3bdc866dda),ulex(sha256(fba48e003101de64df33400ab2595fa7589360ac19f4c77ac3d17bd085f5b8c0)),vocabulary(sha256('335c132fd0e3d3163ba40c255e7db41eecc36492d8098263c505b17f44bbef5a'))).
% S1: Every opioid-use-criminalization discourages a pregnant-person-care-seeking.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s47-21',1,ref(2),[A]),'pregnant-person-care-seeking',countable) :- guideline_entity(actual,A,'opioid-use-criminalization',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s47-21',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-use-criminalization',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s47-21',1,ref(3),[A]),discourage) :- guideline_entity(actual,A,'opioid-use-criminalization',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s47-21',1,ref(3),[A]),1,A) :- guideline_entity(actual,A,'opioid-use-criminalization',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s47-21',1,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s47-21',1,ref(2),[A])) :- guideline_entity(actual,A,'opioid-use-criminalization',countable), guideline_cardinality(actual,A,na,eq,1).
% S2: Every opioid-use-criminalization discourages a postpartum-person-care-seeking.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s47-21',2,ref(2),[A]),'postpartum-person-care-seeking',countable) :- guideline_entity(actual,A,'opioid-use-criminalization',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s47-21',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-use-criminalization',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s47-21',2,ref(3),[A]),discourage) :- guideline_entity(actual,A,'opioid-use-criminalization',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s47-21',2,ref(3),[A]),1,A) :- guideline_entity(actual,A,'opioid-use-criminalization',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s47-21',2,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s47-21',2,ref(2),[A])) :- guideline_entity(actual,A,'opioid-use-criminalization',countable), guideline_cardinality(actual,A,na,eq,1).
% S3: Every opioid-use-criminalization discourages a parenting-person-care-seeking.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s47-21',3,ref(2),[A]),'parenting-person-care-seeking',countable) :- guideline_entity(actual,A,'opioid-use-criminalization',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s47-21',3,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-use-criminalization',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s47-21',3,ref(3),[A]),discourage) :- guideline_entity(actual,A,'opioid-use-criminalization',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s47-21',3,ref(3),[A]),1,A) :- guideline_entity(actual,A,'opioid-use-criminalization',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s47-21',3,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s47-21',3,ref(2),[A])) :- guideline_entity(actual,A,'opioid-use-criminalization',countable), guideline_cardinality(actual,A,na,eq,1).
% S4: Every opioid-use-punishment discourages a pregnant-person-care-seeking.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s47-21',4,ref(2),[A]),'pregnant-person-care-seeking',countable) :- guideline_entity(actual,A,'opioid-use-punishment',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s47-21',4,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-use-punishment',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s47-21',4,ref(3),[A]),discourage) :- guideline_entity(actual,A,'opioid-use-punishment',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s47-21',4,ref(3),[A]),1,A) :- guideline_entity(actual,A,'opioid-use-punishment',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s47-21',4,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s47-21',4,ref(2),[A])) :- guideline_entity(actual,A,'opioid-use-punishment',countable), guideline_cardinality(actual,A,na,eq,1).
% S5: Every opioid-use-punishment discourages a postpartum-person-care-seeking.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s47-21',5,ref(2),[A]),'postpartum-person-care-seeking',countable) :- guideline_entity(actual,A,'opioid-use-punishment',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s47-21',5,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-use-punishment',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s47-21',5,ref(3),[A]),discourage) :- guideline_entity(actual,A,'opioid-use-punishment',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s47-21',5,ref(3),[A]),1,A) :- guideline_entity(actual,A,'opioid-use-punishment',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s47-21',5,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s47-21',5,ref(2),[A])) :- guideline_entity(actual,A,'opioid-use-punishment',countable), guideline_cardinality(actual,A,na,eq,1).
% S6: Every opioid-use-punishment discourages a parenting-person-care-seeking.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s47-21',6,ref(2),[A]),'parenting-person-care-seeking',countable) :- guideline_entity(actual,A,'opioid-use-punishment',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s47-21',6,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'opioid-use-punishment',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s47-21',6,ref(3),[A]),discourage) :- guideline_entity(actual,A,'opioid-use-punishment',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s47-21',6,ref(3),[A]),1,A) :- guideline_entity(actual,A,'opioid-use-punishment',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s47-21',6,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s47-21',6,ref(2),[A])) :- guideline_entity(actual,A,'opioid-use-punishment',countable), guideline_cardinality(actual,A,na,eq,1).
% S7: An opioid-use-punishment-example is a threatened-child-custody-loss-example.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s47-21',7,ref(1),[]),'opioid-use-punishment-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s47-21',7,ref(1),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s47-21',7,ref(2),[]),'threatened-child-custody-loss-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s47-21',7,ref(2),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s47-21',7,ref(3),[]),be).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s47-21',7,ref(3),[]),1,'$guideline_id'(product,'cdc2022-opioid-s47-21',7,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s47-21',7,ref(3),[]),2,'$guideline_id'(product,'cdc2022-opioid-s47-21',7,ref(2),[])).
% S8: An opioid-use-example is an opioid-use-disorder-treatment-example.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s47-21',8,ref(1),[]),'opioid-use-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s47-21',8,ref(1),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s47-21',8,ref(2),[]),'opioid-use-disorder-treatment-example',countable).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s47-21',8,ref(2),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s47-21',8,ref(3),[]),be).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s47-21',8,ref(3),[]),1,'$guideline_id'(product,'cdc2022-opioid-s47-21',8,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s47-21',8,ref(3),[]),2,'$guideline_id'(product,'cdc2022-opioid-s47-21',8,ref(2),[])).
% S9: Every nonpunitive-public-health-treatment-approach results in a better-outcome.
guideline_entity(actual,'$guideline_id'(product,'cdc2022-opioid-s47-21',9,ref(2),[A]),'better-outcome',countable) :- guideline_entity(actual,A,'nonpunitive-public-health-treatment-approach',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality(actual,'$guideline_id'(product,'cdc2022-opioid-s47-21',9,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'nonpunitive-public-health-treatment-approach',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event(actual,'$guideline_id'(product,'cdc2022-opioid-s47-21',9,ref(3),[A]),result) :- guideline_entity(actual,A,'nonpunitive-public-health-treatment-approach',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg(actual,'$guideline_id'(product,'cdc2022-opioid-s47-21',9,ref(3),[A]),1,A) :- guideline_entity(actual,A,'nonpunitive-public-health-treatment-approach',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp(actual,'$guideline_id'(product,'cdc2022-opioid-s47-21',9,ref(3),[A]),in,'$guideline_id'(product,'cdc2022-opioid-s47-21',9,ref(2),[A])) :- guideline_entity(actual,A,'nonpunitive-public-health-treatment-approach',countable), guideline_cardinality(actual,A,na,eq,1).

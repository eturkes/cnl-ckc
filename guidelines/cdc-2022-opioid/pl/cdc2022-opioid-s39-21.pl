% cdc2022-opioid-s39-21.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_document('cdc2022-opioid-s39-21',ace_sha256(d1a1492fcee0b1d7caa8d500624a22cbc848f0a827050ddced5f8977670682a5),ulex(sha256(fba48e003101de64df33400ab2595fa7589360ac19f4c77ac3d17bd085f5b8c0)),vocabulary(sha256('335c132fd0e3d3163ba40c255e7db41eecc36492d8098263c505b17f44bbef5a'))).
% S1: Every NSAID can reportedly address a withdrawal-muscle-ache.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s39-21',1,box(1),[A]),can) :- guideline_entity(actual,A,'NSAID',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s39-21',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',1,ref(2),[A]),'withdrawal-muscle-ache',countable) :- guideline_entity(actual,A,'NSAID',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s39-21',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'NSAID',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-s39-21',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',1,ref(3),[A]),possibility,reportedly) :- guideline_entity(actual,A,'NSAID',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s39-21',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',1,ref(3),[A]),address) :- guideline_entity(actual,A,'NSAID',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s39-21',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',1,ref(3),[A]),1,A) :- guideline_entity(actual,A,'NSAID',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s39-21',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',1,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s39-21',1,ref(2),[A])) :- guideline_entity(actual,A,'NSAID',countable), guideline_cardinality(actual,A,na,eq,1).
% S2: Every acetaminophen can reportedly address a withdrawal-muscle-ache.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s39-21',2,box(1),[A]),can) :- guideline_entity(actual,A,acetaminophen,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s39-21',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',2,ref(2),[A]),'withdrawal-muscle-ache',countable) :- guideline_entity(actual,A,acetaminophen,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s39-21',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,acetaminophen,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-s39-21',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',2,ref(3),[A]),possibility,reportedly) :- guideline_entity(actual,A,acetaminophen,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s39-21',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',2,ref(3),[A]),address) :- guideline_entity(actual,A,acetaminophen,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s39-21',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',2,ref(3),[A]),1,A) :- guideline_entity(actual,A,acetaminophen,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s39-21',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',2,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s39-21',2,ref(2),[A])) :- guideline_entity(actual,A,acetaminophen,countable), guideline_cardinality(actual,A,na,eq,1).
% S3: Every topical-menthol can reportedly address a withdrawal-muscle-ache.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s39-21',3,box(1),[A]),can) :- guideline_entity(actual,A,'topical-menthol',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s39-21',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',3,ref(2),[A]),'withdrawal-muscle-ache',countable) :- guideline_entity(actual,A,'topical-menthol',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s39-21',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',3,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'topical-menthol',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-s39-21',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',3,ref(3),[A]),possibility,reportedly) :- guideline_entity(actual,A,'topical-menthol',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s39-21',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',3,ref(3),[A]),address) :- guideline_entity(actual,A,'topical-menthol',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s39-21',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',3,ref(3),[A]),1,A) :- guideline_entity(actual,A,'topical-menthol',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s39-21',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',3,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s39-21',3,ref(2),[A])) :- guideline_entity(actual,A,'topical-menthol',countable), guideline_cardinality(actual,A,na,eq,1).
% S4: Every methyl-salicylate can reportedly address a withdrawal-muscle-ache.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s39-21',4,box(1),[A]),can) :- guideline_entity(actual,A,'methyl-salicylate',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s39-21',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',4,ref(2),[A]),'withdrawal-muscle-ache',countable) :- guideline_entity(actual,A,'methyl-salicylate',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s39-21',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',4,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'methyl-salicylate',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-s39-21',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',4,ref(3),[A]),possibility,reportedly) :- guideline_entity(actual,A,'methyl-salicylate',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s39-21',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',4,ref(3),[A]),address) :- guideline_entity(actual,A,'methyl-salicylate',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s39-21',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',4,ref(3),[A]),1,A) :- guideline_entity(actual,A,'methyl-salicylate',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s39-21',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',4,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s39-21',4,ref(2),[A])) :- guideline_entity(actual,A,'methyl-salicylate',countable), guideline_cardinality(actual,A,na,eq,1).
% S5: Every trazodone can reportedly address a withdrawal-sleep-disturbance.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s39-21',5,box(1),[A]),can) :- guideline_entity(actual,A,trazodone,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s39-21',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',5,ref(2),[A]),'withdrawal-sleep-disturbance',countable) :- guideline_entity(actual,A,trazodone,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s39-21',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',5,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,trazodone,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-s39-21',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',5,ref(3),[A]),possibility,reportedly) :- guideline_entity(actual,A,trazodone,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s39-21',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',5,ref(3),[A]),address) :- guideline_entity(actual,A,trazodone,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s39-21',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',5,ref(3),[A]),1,A) :- guideline_entity(actual,A,trazodone,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s39-21',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',5,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s39-21',5,ref(2),[A])) :- guideline_entity(actual,A,trazodone,countable), guideline_cardinality(actual,A,na,eq,1).
% S6: Every prochlorperazine can reportedly address a withdrawal-nausea.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s39-21',6,box(1),[A]),can) :- guideline_entity(actual,A,prochlorperazine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s39-21',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',6,ref(2),[A]),'withdrawal-nausea',countable) :- guideline_entity(actual,A,prochlorperazine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s39-21',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',6,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,prochlorperazine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-s39-21',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',6,ref(3),[A]),possibility,reportedly) :- guideline_entity(actual,A,prochlorperazine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s39-21',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',6,ref(3),[A]),address) :- guideline_entity(actual,A,prochlorperazine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s39-21',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',6,ref(3),[A]),1,A) :- guideline_entity(actual,A,prochlorperazine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s39-21',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',6,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s39-21',6,ref(2),[A])) :- guideline_entity(actual,A,prochlorperazine,countable), guideline_cardinality(actual,A,na,eq,1).
% S7: Every promethazine can reportedly address a withdrawal-nausea.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s39-21',7,box(1),[A]),can) :- guideline_entity(actual,A,promethazine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s39-21',7,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',7,ref(2),[A]),'withdrawal-nausea',countable) :- guideline_entity(actual,A,promethazine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s39-21',7,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',7,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,promethazine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-s39-21',7,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',7,ref(3),[A]),possibility,reportedly) :- guideline_entity(actual,A,promethazine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s39-21',7,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',7,ref(3),[A]),address) :- guideline_entity(actual,A,promethazine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s39-21',7,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',7,ref(3),[A]),1,A) :- guideline_entity(actual,A,promethazine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s39-21',7,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',7,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s39-21',7,ref(2),[A])) :- guideline_entity(actual,A,promethazine,countable), guideline_cardinality(actual,A,na,eq,1).
% S8: Every ondansetron can reportedly address a withdrawal-nausea.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s39-21',8,box(1),[A]),can) :- guideline_entity(actual,A,ondansetron,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s39-21',8,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',8,ref(2),[A]),'withdrawal-nausea',countable) :- guideline_entity(actual,A,ondansetron,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s39-21',8,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',8,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,ondansetron,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-s39-21',8,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',8,ref(3),[A]),possibility,reportedly) :- guideline_entity(actual,A,ondansetron,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s39-21',8,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',8,ref(3),[A]),address) :- guideline_entity(actual,A,ondansetron,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s39-21',8,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',8,ref(3),[A]),1,A) :- guideline_entity(actual,A,ondansetron,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s39-21',8,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',8,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s39-21',8,ref(2),[A])) :- guideline_entity(actual,A,ondansetron,countable), guideline_cardinality(actual,A,na,eq,1).
% S9: Every dicyclomine can reportedly address a withdrawal-abdominal-cramping.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s39-21',9,box(1),[A]),can) :- guideline_entity(actual,A,dicyclomine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s39-21',9,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',9,ref(2),[A]),'withdrawal-abdominal-cramping',countable) :- guideline_entity(actual,A,dicyclomine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s39-21',9,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',9,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,dicyclomine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-s39-21',9,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',9,ref(3),[A]),possibility,reportedly) :- guideline_entity(actual,A,dicyclomine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s39-21',9,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',9,ref(3),[A]),address) :- guideline_entity(actual,A,dicyclomine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s39-21',9,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',9,ref(3),[A]),1,A) :- guideline_entity(actual,A,dicyclomine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s39-21',9,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',9,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s39-21',9,ref(2),[A])) :- guideline_entity(actual,A,dicyclomine,countable), guideline_cardinality(actual,A,na,eq,1).
% S10: Every loperamide can reportedly address a withdrawal-diarrhea.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s39-21',10,box(1),[A]),can) :- guideline_entity(actual,A,loperamide,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s39-21',10,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',10,ref(2),[A]),'withdrawal-diarrhea',countable) :- guideline_entity(actual,A,loperamide,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s39-21',10,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',10,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,loperamide,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-s39-21',10,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',10,ref(3),[A]),possibility,reportedly) :- guideline_entity(actual,A,loperamide,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s39-21',10,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',10,ref(3),[A]),address) :- guideline_entity(actual,A,loperamide,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s39-21',10,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',10,ref(3),[A]),1,A) :- guideline_entity(actual,A,loperamide,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s39-21',10,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',10,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s39-21',10,ref(2),[A])) :- guideline_entity(actual,A,loperamide,countable), guideline_cardinality(actual,A,na,eq,1).
% S11: Every bismuth-subsalicylate can reportedly address a withdrawal-diarrhea.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s39-21',11,box(1),[A]),can) :- guideline_entity(actual,A,'bismuth-subsalicylate',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s39-21',11,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',11,ref(2),[A]),'withdrawal-diarrhea',countable) :- guideline_entity(actual,A,'bismuth-subsalicylate',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s39-21',11,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',11,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'bismuth-subsalicylate',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_qualifier('$guideline_id'(context,'cdc2022-opioid-s39-21',11,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',11,ref(3),[A]),possibility,reportedly) :- guideline_entity(actual,A,'bismuth-subsalicylate',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s39-21',11,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',11,ref(3),[A]),address) :- guideline_entity(actual,A,'bismuth-subsalicylate',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s39-21',11,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',11,ref(3),[A]),1,A) :- guideline_entity(actual,A,'bismuth-subsalicylate',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s39-21',11,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',11,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s39-21',11,ref(2),[A])) :- guideline_entity(actual,A,'bismuth-subsalicylate',countable), guideline_cardinality(actual,A,na,eq,1).

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
guideline_document('cdc2022-opioid-s39-21',ace_sha256(a060e772cbff4e38f2530343f1e902ef6cee7f31f02a72399aa1692a4b078534),ulex(sha256(de64e41d6ddfdcb89810d9fc156aa0ac54562519e3bd4b94b8f7324f773ceb91)),vocabulary(sha256('06621a3dca592ca4385262a4a36dd8184670cdaef31827cd8b224c8c870c2f7d'))).
% S1: Every NSAID can address a withdrawal-muscle-ache.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s39-21',1,box(1),[A]),can) :- guideline_entity(actual,A,'NSAID',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s39-21',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',1,ref(2),[A]),'withdrawal-muscle-ache',countable) :- guideline_entity(actual,A,'NSAID',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s39-21',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'NSAID',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s39-21',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',1,ref(3),[A]),address) :- guideline_entity(actual,A,'NSAID',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s39-21',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',1,ref(3),[A]),1,A) :- guideline_entity(actual,A,'NSAID',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s39-21',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',1,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s39-21',1,ref(2),[A])) :- guideline_entity(actual,A,'NSAID',countable), guideline_cardinality(actual,A,na,eq,1).
% S2: Every acetaminophen can address a withdrawal-muscle-ache.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s39-21',2,box(1),[A]),can) :- guideline_entity(actual,A,acetaminophen,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s39-21',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',2,ref(2),[A]),'withdrawal-muscle-ache',countable) :- guideline_entity(actual,A,acetaminophen,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s39-21',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',2,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,acetaminophen,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s39-21',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',2,ref(3),[A]),address) :- guideline_entity(actual,A,acetaminophen,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s39-21',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',2,ref(3),[A]),1,A) :- guideline_entity(actual,A,acetaminophen,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s39-21',2,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',2,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s39-21',2,ref(2),[A])) :- guideline_entity(actual,A,acetaminophen,countable), guideline_cardinality(actual,A,na,eq,1).
% S3: Every topical-menthol can address a withdrawal-muscle-ache.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s39-21',3,box(1),[A]),can) :- guideline_entity(actual,A,'topical-menthol',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s39-21',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',3,ref(2),[A]),'withdrawal-muscle-ache',countable) :- guideline_entity(actual,A,'topical-menthol',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s39-21',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',3,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'topical-menthol',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s39-21',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',3,ref(3),[A]),address) :- guideline_entity(actual,A,'topical-menthol',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s39-21',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',3,ref(3),[A]),1,A) :- guideline_entity(actual,A,'topical-menthol',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s39-21',3,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',3,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s39-21',3,ref(2),[A])) :- guideline_entity(actual,A,'topical-menthol',countable), guideline_cardinality(actual,A,na,eq,1).
% S4: Every methyl-salicylate can address a withdrawal-muscle-ache.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s39-21',4,box(1),[A]),can) :- guideline_entity(actual,A,'methyl-salicylate',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s39-21',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',4,ref(2),[A]),'withdrawal-muscle-ache',countable) :- guideline_entity(actual,A,'methyl-salicylate',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s39-21',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',4,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'methyl-salicylate',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s39-21',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',4,ref(3),[A]),address) :- guideline_entity(actual,A,'methyl-salicylate',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s39-21',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',4,ref(3),[A]),1,A) :- guideline_entity(actual,A,'methyl-salicylate',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s39-21',4,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',4,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s39-21',4,ref(2),[A])) :- guideline_entity(actual,A,'methyl-salicylate',countable), guideline_cardinality(actual,A,na,eq,1).
% S5: Every trazodone can address a withdrawal-sleep-disturbance.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s39-21',5,box(1),[A]),can) :- guideline_entity(actual,A,trazodone,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s39-21',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',5,ref(2),[A]),'withdrawal-sleep-disturbance',countable) :- guideline_entity(actual,A,trazodone,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s39-21',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',5,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,trazodone,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s39-21',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',5,ref(3),[A]),address) :- guideline_entity(actual,A,trazodone,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s39-21',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',5,ref(3),[A]),1,A) :- guideline_entity(actual,A,trazodone,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s39-21',5,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',5,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s39-21',5,ref(2),[A])) :- guideline_entity(actual,A,trazodone,countable), guideline_cardinality(actual,A,na,eq,1).
% S6: Every prochlorperazine can address a withdrawal-nausea.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s39-21',6,box(1),[A]),can) :- guideline_entity(actual,A,prochlorperazine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s39-21',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',6,ref(2),[A]),'withdrawal-nausea',countable) :- guideline_entity(actual,A,prochlorperazine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s39-21',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',6,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,prochlorperazine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s39-21',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',6,ref(3),[A]),address) :- guideline_entity(actual,A,prochlorperazine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s39-21',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',6,ref(3),[A]),1,A) :- guideline_entity(actual,A,prochlorperazine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s39-21',6,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',6,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s39-21',6,ref(2),[A])) :- guideline_entity(actual,A,prochlorperazine,countable), guideline_cardinality(actual,A,na,eq,1).
% S7: Every promethazine can address a withdrawal-nausea.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s39-21',7,box(1),[A]),can) :- guideline_entity(actual,A,promethazine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s39-21',7,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',7,ref(2),[A]),'withdrawal-nausea',countable) :- guideline_entity(actual,A,promethazine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s39-21',7,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',7,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,promethazine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s39-21',7,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',7,ref(3),[A]),address) :- guideline_entity(actual,A,promethazine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s39-21',7,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',7,ref(3),[A]),1,A) :- guideline_entity(actual,A,promethazine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s39-21',7,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',7,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s39-21',7,ref(2),[A])) :- guideline_entity(actual,A,promethazine,countable), guideline_cardinality(actual,A,na,eq,1).
% S8: Every ondansetron can address a withdrawal-nausea.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s39-21',8,box(1),[A]),can) :- guideline_entity(actual,A,ondansetron,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s39-21',8,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',8,ref(2),[A]),'withdrawal-nausea',countable) :- guideline_entity(actual,A,ondansetron,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s39-21',8,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',8,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,ondansetron,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s39-21',8,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',8,ref(3),[A]),address) :- guideline_entity(actual,A,ondansetron,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s39-21',8,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',8,ref(3),[A]),1,A) :- guideline_entity(actual,A,ondansetron,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s39-21',8,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',8,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s39-21',8,ref(2),[A])) :- guideline_entity(actual,A,ondansetron,countable), guideline_cardinality(actual,A,na,eq,1).
% S9: Every dicyclomine can address a withdrawal-abdominal-cramping.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s39-21',9,box(1),[A]),can) :- guideline_entity(actual,A,dicyclomine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s39-21',9,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',9,ref(2),[A]),'withdrawal-abdominal-cramping',countable) :- guideline_entity(actual,A,dicyclomine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s39-21',9,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',9,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,dicyclomine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s39-21',9,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',9,ref(3),[A]),address) :- guideline_entity(actual,A,dicyclomine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s39-21',9,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',9,ref(3),[A]),1,A) :- guideline_entity(actual,A,dicyclomine,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s39-21',9,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',9,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s39-21',9,ref(2),[A])) :- guideline_entity(actual,A,dicyclomine,countable), guideline_cardinality(actual,A,na,eq,1).
% S10: Every loperamide can address a withdrawal-diarrhea.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s39-21',10,box(1),[A]),can) :- guideline_entity(actual,A,loperamide,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s39-21',10,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',10,ref(2),[A]),'withdrawal-diarrhea',countable) :- guideline_entity(actual,A,loperamide,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s39-21',10,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',10,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,loperamide,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s39-21',10,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',10,ref(3),[A]),address) :- guideline_entity(actual,A,loperamide,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s39-21',10,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',10,ref(3),[A]),1,A) :- guideline_entity(actual,A,loperamide,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s39-21',10,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',10,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s39-21',10,ref(2),[A])) :- guideline_entity(actual,A,loperamide,countable), guideline_cardinality(actual,A,na,eq,1).
% S11: Every bismuth-subsalicylate can address a withdrawal-diarrhea.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s39-21',11,box(1),[A]),can) :- guideline_entity(actual,A,'bismuth-subsalicylate',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s39-21',11,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',11,ref(2),[A]),'withdrawal-diarrhea',countable) :- guideline_entity(actual,A,'bismuth-subsalicylate',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s39-21',11,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',11,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,'bismuth-subsalicylate',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s39-21',11,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',11,ref(3),[A]),address) :- guideline_entity(actual,A,'bismuth-subsalicylate',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s39-21',11,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',11,ref(3),[A]),1,A) :- guideline_entity(actual,A,'bismuth-subsalicylate',countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s39-21',11,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s39-21',11,ref(3),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s39-21',11,ref(2),[A])) :- guideline_entity(actual,A,'bismuth-subsalicylate',countable), guideline_cardinality(actual,A,na,eq,1).

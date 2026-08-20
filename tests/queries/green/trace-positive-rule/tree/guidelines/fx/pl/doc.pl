% synthetic attributed product; regeneration recipe: .agent/memory.md (M4.7 queries suite).
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
guideline_document(doc,ace_sha256(f2cbd2e45cf351ebdc1c338853a7d245488c07c9153399f9bfccfe476b65d5df),ulex(none)).
% S1: Synthetic attributed fixture sentence.
guideline_entity(actual,'$guideline_id'(product,doc,1,ref(1),[]),patient,countable) :- guideline_property(actual,'$guideline_id'(product,doc,1,ref(1),[]),eligible,pos).
guideline_property(actual,'$guideline_id'(product,doc,1,ref(1),[]),eligible,pos).
guideline_cardinality(actual,'$guideline_id'(product,doc,1,ref(1),[]),na,eq,1).
guideline_event(actual,'$guideline_id'(product,doc,1,ref(2),[]),wait).
guideline_arg(actual,'$guideline_id'(product,doc,1,ref(2),[]),1,'$guideline_id'(product,doc,1,ref(1),[])).

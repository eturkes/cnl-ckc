% doc.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
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
guideline_schema_version(2).
guideline_document(doc,ace_sha256(e1b497038774f29540d369419e71988662932bc7328ebf9f177897d947daf92f),ulex(none),temporal(sha256('1b017245079fd0206a790060fc3208db1db4c72059eda61145257591e61c5e23'))).
% S1: A man waits for 3 days.
guideline_entity(actual,'$guideline_id'(product,doc,1,ref(1),[]),man,countable).
guideline_cardinality(actual,'$guideline_id'(product,doc,1,ref(1),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,doc,1,ref(2),[]),day,countable).
guideline_cardinality(actual,'$guideline_id'(product,doc,1,ref(2),[]),na,eq,3).
guideline_event(actual,'$guideline_id'(product,doc,1,ref(3),[]),wait).
guideline_arg(actual,'$guideline_id'(product,doc,1,ref(3),[]),1,'$guideline_id'(product,doc,1,ref(1),[])).
guideline_pp(actual,'$guideline_id'(product,doc,1,ref(3),[]),for,'$guideline_id'(product,doc,1,ref(2),[])).
guideline_interval(actual,'$guideline_id'(product,doc,1,ref(3),[]),duration,'$guideline_id'(product,doc,1,ref(2),[]),day,none).
% S2: A woman reads a book at an interval of 2 weeks.
guideline_entity(actual,'$guideline_id'(product,doc,2,ref(1),[]),woman,countable).
guideline_cardinality(actual,'$guideline_id'(product,doc,2,ref(1),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,doc,2,ref(2),[]),book,countable).
guideline_cardinality(actual,'$guideline_id'(product,doc,2,ref(2),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,doc,2,ref(3),[]),interval,countable).
guideline_cardinality(actual,'$guideline_id'(product,doc,2,ref(3),[]),na,eq,1).
guideline_entity(actual,'$guideline_id'(product,doc,2,ref(4),[]),week,countable).
guideline_cardinality(actual,'$guideline_id'(product,doc,2,ref(4),[]),na,eq,2).
guideline_event(actual,'$guideline_id'(product,doc,2,ref(5),[]),read).
guideline_arg(actual,'$guideline_id'(product,doc,2,ref(5),[]),1,'$guideline_id'(product,doc,2,ref(1),[])).
guideline_arg(actual,'$guideline_id'(product,doc,2,ref(5),[]),2,'$guideline_id'(product,doc,2,ref(2),[])).
guideline_pp(actual,'$guideline_id'(product,doc,2,ref(5),[]),at,'$guideline_id'(product,doc,2,ref(3),[])).
guideline_recurrence(actual,'$guideline_id'(product,doc,2,ref(5),[]),'$guideline_id'(product,doc,2,ref(4),[]),week).

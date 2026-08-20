% q-empty compiled from ACE question by ace_to_pl question mode; do not edit.
'$guideline_query'(v1,'q-empty',ace_sha256('78aaad8e2a574386a38ad952ecb8ea9b3984a6d239c9ca7233b8d59e0aa557a5'),ulex(none)).
% Q1: Which patient waits?
'$guideline_query_projection'(goal(','(guideline_entity(actual,A,patient,countable),','(guideline_cardinality(actual,A,na,eq,1),','(guideline_event(actual,B,wait),guideline_arg(actual,B,1,A))))),answers([answer(A,noun(patient,countable))])).

% q-stale-answer compiled from ACE question by ace_to_pl question mode; do not edit.
'$guideline_query'(v1,'q-stale-answer',ace_sha256(f0fc847f6ad59f03e524f9c0a387dfe763b1af3e727502099de026977bb89615),ulex(none)).
% Q1: Does a patient wait?
'$guideline_query_projection'(goal(','(guideline_entity(actual,A,patient,countable),','(guideline_cardinality(actual,A,na,eq,1),','(guideline_event(actual,B,wait),guideline_arg(actual,B,1,A))))),answers([])).

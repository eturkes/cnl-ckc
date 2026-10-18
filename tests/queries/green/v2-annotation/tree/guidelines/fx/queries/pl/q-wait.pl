% q-wait compiled from ACE question by ace_to_pl question mode; do not edit.
'$guideline_query'(v1,'q-wait',ace_sha256(da2ae7ef909a40fc2248a3180f5791495c7761c1bbf82d63e17acd713600ef18),ulex(none),vocabulary(sha256('4d554e69a44f325ae34f930739183f4abd6a6019b7ebe43057cc2b65462c7c21'))).
% Q1: Who waits for 3 days?
'$guideline_query_projection'(goal(','(guideline_entity(actual,A,day,countable),','(guideline_cardinality(actual,A,na,eq,3),','(guideline_unit(actual,A,day),','(guideline_event(actual,B,wait),','(guideline_arg(actual,B,1,C),','(guideline_pp(actual,B,for,A),guideline_interval(actual,B,duration,A,day,none)))))))),answers([answer(C,wh(who))])).

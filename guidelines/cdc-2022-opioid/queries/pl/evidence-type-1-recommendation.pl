% evidence-type-1-recommendation compiled from ACE question by ace_to_pl question mode; do not edit.
'$guideline_query'(v1,'evidence-type-1-recommendation',ace_sha256(faeb2a82d1512ecc03939e815f8c54338c3b99418cedc28f2fd1d784fd188ce1),ulex(sha256(ef57ba7180864095bab88b95f11ddb1a2c858593eba715dbfbe874db09506c5b))).
% Q1: Which recommendation is an evidence-type-1-recommendation?
'$guideline_query_projection'(goal(','(guideline_entity(actual,A,recommendation,countable),','(guideline_cardinality(actual,A,na,eq,1),','(guideline_entity(actual,B,'evidence-type-1-recommendation',countable),','(guideline_cardinality(actual,B,na,eq,1),','(guideline_event(actual,C,be),','(guideline_arg(actual,C,1,A),guideline_arg(actual,C,2,B)))))))),answers([answer(A,noun(recommendation,countable))])).

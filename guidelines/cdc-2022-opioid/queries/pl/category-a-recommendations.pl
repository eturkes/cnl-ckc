% category-a-recommendations compiled from ACE question by ace_to_pl question mode; do not edit.
'$guideline_query'(v1,'category-a-recommendations',ace_sha256(e1a55c47923fd26d266c56eb8b19cefba225d57e9095a2bfaec155bd392ff541),ulex(sha256(ef57ba7180864095bab88b95f11ddb1a2c858593eba715dbfbe874db09506c5b))).
% Q1: Which recommendation is a category-A-recommendation?
'$guideline_query_projection'(goal(','(guideline_entity(actual,A,recommendation,countable),','(guideline_cardinality(actual,A,na,eq,1),','(guideline_entity(actual,B,'category-A-recommendation',countable),','(guideline_cardinality(actual,B,na,eq,1),','(guideline_event(actual,C,be),','(guideline_arg(actual,C,1,A),guideline_arg(actual,C,2,B)))))))),answers([answer(A,noun(recommendation,countable))])).

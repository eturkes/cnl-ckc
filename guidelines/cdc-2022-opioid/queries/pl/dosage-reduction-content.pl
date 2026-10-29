% dosage-reduction-content compiled from ACE question by ace_to_pl question mode; do not edit.
'$guideline_query'(v1,'dosage-reduction-content',ace_sha256(a690fc31ba3d8f4a33072957b321d16d1b9b8807dc4b60a5f7433e9a807c3c0e),ulex(sha256(fba48e003101de64df33400ab2595fa7589360ac19f4c77ac3d17bd085f5b8c0)),vocabulary(sha256('335c132fd0e3d3163ba40c255e7db41eecc36492d8098263c505b17f44bbef5a'))).
% Q1: What does a dosage-reduction have?
'$guideline_query_projection'(goal(','(guideline_entity(actual,A,'dosage-reduction',countable),','(guideline_cardinality(actual,A,na,eq,1),','(guideline_event(actual,B,have),','(guideline_arg(actual,B,1,A),guideline_arg(actual,B,2,C)))))),answers([answer(C,wh(what))])).

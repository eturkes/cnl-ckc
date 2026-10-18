% dosage-reduction-content compiled from ACE question by ace_to_pl question mode; do not edit.
'$guideline_query'(v1,'dosage-reduction-content',ace_sha256(a690fc31ba3d8f4a33072957b321d16d1b9b8807dc4b60a5f7433e9a807c3c0e),ulex(sha256('3e37dcfbb36dbdd0830d644235c88914b28def633a7ac43d2b5bb53faf8ba507')),vocabulary(sha256('9d7632637554f8cbfe72e7d34a9b9d14c29eb8593aebb1c128a870c5dcf77ead'))).
% Q1: What does a dosage-reduction have?
'$guideline_query_projection'(goal(','(guideline_entity(actual,A,'dosage-reduction',countable),','(guideline_cardinality(actual,A,na,eq,1),','(guideline_event(actual,B,have),','(guideline_arg(actual,B,1,A),guideline_arg(actual,B,2,C)))))),answers([answer(C,wh(what))])).

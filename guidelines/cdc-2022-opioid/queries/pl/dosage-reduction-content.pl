% dosage-reduction-content compiled from ACE question by ace_to_pl question mode; do not edit.
'$guideline_query'(v1,'dosage-reduction-content',ace_sha256(a690fc31ba3d8f4a33072957b321d16d1b9b8807dc4b60a5f7433e9a807c3c0e),ulex(sha256('64c086862c2f48d491a9fde0ba9f0fcf090e25e84456b9749332ec3ace211ce4')),vocabulary(sha256('06621a3dca592ca4385262a4a36dd8184670cdaef31827cd8b224c8c870c2f7d'))).
% Q1: What does a dosage-reduction have?
'$guideline_query_projection'(goal(','(guideline_entity(actual,A,'dosage-reduction',countable),','(guideline_cardinality(actual,A,na,eq,1),','(guideline_event(actual,B,have),','(guideline_arg(actual,B,1,A),guideline_arg(actual,B,2,C)))))),answers([answer(C,wh(what))])).

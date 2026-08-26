% q-back compiled from ACE question by ace_to_pl question mode; do not edit.
'$guideline_query'(v1,'q-back',ace_sha256('6e9b8aeeb79450a9d8771475d9d5cfcc8bc708bc79bcc67f557f7b0abbda8ee5'),ulex(none)).
% Q1: Is there a patient?
'$guideline_query_projection'(goal(','(guideline_entity(actual,A,patient,countable),guideline_cardinality(actual,A,na,eq,1))),answers([])).

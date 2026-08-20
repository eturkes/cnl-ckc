guideline_schema_version(1).
guideline_document('yn-limit-doc',ace_sha256(x),ulex(none)).
guideline_entity(actual,p1,patient,countable) :- between(1,200000,N), N =:= 200000.
guideline_entity(actual,p1,patient,countable).
guideline_cardinality(actual,p1,na,eq,1).
guideline_event(actual,e1,wait).
guideline_arg(actual,e1,1,p1).

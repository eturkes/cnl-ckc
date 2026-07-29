ace_front_end_record(1).
document(docid('relation-ambiguous-referent'),source_sha256('0000000000000000000000000000000000000000000000000000000000000000'),ulex(none)).
drs([A,B,C],[-(relation(A,of,named('Mary')),/(1,2)),-(predicate(B,be,named('John'),A),/(1,3)),-(predicate(C,be,named('Jane'),A),/(1,4)),question(drs([D],[-(predicate(D,wait,named('John')),/(2,3))]))]).

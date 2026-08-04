ace_front_end_record(1).
document(docid('temporal-after'),source_sha256('0000000000000000000000000000000000000000000000000000000000000000'),ulex(none)).
drs([A],[-(predicate(A,wait,named('John')),/(1,2)),-(modifier_pp(A,after,named('Therapy-start')),/(1,3)),=>(drs([B],[-(predicate(B,wait,named('John')),/(2,2)),-(modifier_pp(B,after,named('Therapy-start')),/(2,3))]),drs([C],[-(predicate(C,recover,named('John')),/(2,5))])),question(drs([D],[-(predicate(D,recover,named('John')),/(3,3))]))]).

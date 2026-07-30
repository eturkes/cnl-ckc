ace_front_end_record(1).
document(docid('m6-disjunction'),source_sha256('1111111111111111111111111111111111111111111111111111111111111111'),ulex(none)).
drs([A,B,C],[-(object(A,patient,countable,na,eq,1),/(1,4)),-(predicate(B,be,named('John'),A),/(1,2)),-(predicate(C,wait,named('John')),/(2,2)),=>(drs([D],[-(object(D,patient,countable,na,eq,1),/(3,2)),v(drs([E],[-(predicate(E,wait,D),/(3,4))]),drs([F],[-(predicate(F,sleep,D),/(3,7))]))]),drs([G],[-(predicate(G,recover,D),/(3,8))])),question(drs([H],[-(predicate(H,recover,named('John')),/(4,3))]))]).

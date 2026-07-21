ace_front_end_record(1).
document(docid('p10-wh-prefix'),source_sha256('1010101010101010101010101010101010101010101010101010101010101010'),ulex(none)).
drs([A,B,C],[-(object(A,patient,countable,na,eq,1),/(1,4)),-(predicate(B,be,named('John'),A),/(1,2)),-(predicate(C,wait,named('John')),/(2,2)),=>(drs([D,E],[-(object(D,patient,countable,na,eq,1),/(3,2)),-(predicate(E,wait,D),/(3,4))]),drs([F],[-(predicate(F,recover,D),/(3,5))])),question(drs([G,H],[-(query(G,who),/(4,1)),-(predicate(H,recover,G),/(4,2))]))]).

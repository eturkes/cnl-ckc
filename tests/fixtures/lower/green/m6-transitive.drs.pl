ace_front_end_record(1).
document(docid('m6-transitive'),source_sha256('3234b9d178211bd8e791ee9c3e37ddbf8ae02d198c0ea200cf48e9a19737af11'),ulex(none)).
drs([A,B,C],[-(predicate(A,like,named('John'),named('Mary')),/(1,2)),-(object(B,patient,countable,na,eq,1),/(2,4)),-(predicate(C,be,named('John'),B),/(2,2)),=>(drs([D,E],[-(object(D,patient,countable,na,eq,1),/(3,2)),-(predicate(E,like,D,named('Mary')),/(3,4))]),drs([F],[-(predicate(F,help,D,named('Mary')),/(3,6))])),question(drs([G],[-(predicate(G,help,named('John'),named('Mary')),/(4,3))]))]).

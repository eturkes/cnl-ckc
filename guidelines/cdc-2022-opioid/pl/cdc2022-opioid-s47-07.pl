% cdc2022-opioid-s47-07.pl compiled from ACE by ace_to_pl; regenerate via tools/goal.py; do not edit.
:- multifile(guideline_schema_version/1).
:- discontiguous(guideline_schema_version/1).
:- multifile(guideline_document/3).
:- discontiguous(guideline_document/3).
:- multifile(guideline_entity/4).
:- discontiguous(guideline_entity/4).
:- multifile(guideline_cardinality/5).
:- discontiguous(guideline_cardinality/5).
:- multifile(guideline_event/3).
:- discontiguous(guideline_event/3).
:- multifile(guideline_arg/4).
:- discontiguous(guideline_arg/4).
:- multifile(guideline_pp/4).
:- discontiguous(guideline_pp/4).
:- multifile(guideline_property/4).
:- discontiguous(guideline_property/4).
:- multifile(guideline_operator/3).
:- discontiguous(guideline_operator/3).
guideline_schema_version(1).
guideline_document('cdc2022-opioid-s47-07',ace_sha256('7e201a460229fcd922b2ee37ab43263bd75eec2df08a5f695eb5c7f73abe3275'),ulex(sha256('4a882a1996ea910df0fbf91fea897c0805e67d48f9e1ce2fcdbbbd9250c193ae'))).
% S1: Every clinician should review a prescribed-medication during a prepregnancy-care and should review a nonprescribed-medication during a prepregnancy-care and should review a prescribed-medication during an interpregnancy-care and should review a nonprescribed-medication during an interpregnancy-care.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s47-07',1,box(1),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s47-07',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s47-07',1,ref(2),[A]),'prescribed-medication',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s47-07',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s47-07',1,ref(2),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s47-07',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s47-07',1,ref(3),[A]),'prepregnancy-care',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s47-07',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s47-07',1,ref(3),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s47-07',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s47-07',1,ref(4),[A]),review) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s47-07',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s47-07',1,ref(4),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s47-07',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s47-07',1,ref(4),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s47-07',1,ref(2),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s47-07',1,box(1),[A]),'$guideline_id'(product,'cdc2022-opioid-s47-07',1,ref(4),[A]),during,'$guideline_id'(product,'cdc2022-opioid-s47-07',1,ref(3),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s47-07',1,box(2),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s47-07',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s47-07',1,ref(5),[A]),'nonprescribed-medication',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s47-07',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s47-07',1,ref(5),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s47-07',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s47-07',1,ref(6),[A]),'prepregnancy-care',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s47-07',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s47-07',1,ref(6),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s47-07',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s47-07',1,ref(7),[A]),review) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s47-07',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s47-07',1,ref(7),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s47-07',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s47-07',1,ref(7),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s47-07',1,ref(5),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s47-07',1,box(2),[A]),'$guideline_id'(product,'cdc2022-opioid-s47-07',1,ref(7),[A]),during,'$guideline_id'(product,'cdc2022-opioid-s47-07',1,ref(6),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s47-07',1,box(3),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s47-07',1,box(3),[A]),'$guideline_id'(product,'cdc2022-opioid-s47-07',1,ref(8),[A]),'prescribed-medication',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s47-07',1,box(3),[A]),'$guideline_id'(product,'cdc2022-opioid-s47-07',1,ref(8),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s47-07',1,box(3),[A]),'$guideline_id'(product,'cdc2022-opioid-s47-07',1,ref(9),[A]),'interpregnancy-care',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s47-07',1,box(3),[A]),'$guideline_id'(product,'cdc2022-opioid-s47-07',1,ref(9),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s47-07',1,box(3),[A]),'$guideline_id'(product,'cdc2022-opioid-s47-07',1,ref(10),[A]),review) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s47-07',1,box(3),[A]),'$guideline_id'(product,'cdc2022-opioid-s47-07',1,ref(10),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s47-07',1,box(3),[A]),'$guideline_id'(product,'cdc2022-opioid-s47-07',1,ref(10),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s47-07',1,ref(8),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s47-07',1,box(3),[A]),'$guideline_id'(product,'cdc2022-opioid-s47-07',1,ref(10),[A]),during,'$guideline_id'(product,'cdc2022-opioid-s47-07',1,ref(9),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s47-07',1,box(4),[A]),should) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s47-07',1,box(4),[A]),'$guideline_id'(product,'cdc2022-opioid-s47-07',1,ref(11),[A]),'nonprescribed-medication',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s47-07',1,box(4),[A]),'$guideline_id'(product,'cdc2022-opioid-s47-07',1,ref(11),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s47-07',1,box(4),[A]),'$guideline_id'(product,'cdc2022-opioid-s47-07',1,ref(12),[A]),'interpregnancy-care',countable) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s47-07',1,box(4),[A]),'$guideline_id'(product,'cdc2022-opioid-s47-07',1,ref(12),[A]),na,eq,1) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s47-07',1,box(4),[A]),'$guideline_id'(product,'cdc2022-opioid-s47-07',1,ref(13),[A]),review) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s47-07',1,box(4),[A]),'$guideline_id'(product,'cdc2022-opioid-s47-07',1,ref(13),[A]),1,A) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s47-07',1,box(4),[A]),'$guideline_id'(product,'cdc2022-opioid-s47-07',1,ref(13),[A]),2,'$guideline_id'(product,'cdc2022-opioid-s47-07',1,ref(11),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s47-07',1,box(4),[A]),'$guideline_id'(product,'cdc2022-opioid-s47-07',1,ref(13),[A]),during,'$guideline_id'(product,'cdc2022-opioid-s47-07',1,ref(12),[A])) :- guideline_entity(actual,A,clinician,countable), guideline_cardinality(actual,A,na,eq,1).

% cdc2022-opioid-s3-02.pl compiled from ACE by ace_to_pl; regenerate via ckc compile; do not edit.
:- multifile(guideline_schema_version/1).
:- discontiguous(guideline_schema_version/1).
:- multifile(guideline_document/4).
:- discontiguous(guideline_document/4).
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
:- multifile(guideline_interval/6).
:- discontiguous(guideline_interval/6).
:- multifile(guideline_recurrence/4).
:- discontiguous(guideline_recurrence/4).
:- multifile(guideline_frequency/5).
:- discontiguous(guideline_frequency/5).
:- multifile(guideline_order/4).
:- discontiguous(guideline_order/4).
:- multifile(guideline_recurrence_window/7).
:- discontiguous(guideline_recurrence_window/7).
:- multifile(guideline_range/3).
:- discontiguous(guideline_range/3).
:- multifile(guideline_grade/5).
:- discontiguous(guideline_grade/5).
:- multifile(guideline_preference/4).
:- discontiguous(guideline_preference/4).
:- multifile(guideline_qualifier/4).
:- discontiguous(guideline_qualifier/4).
:- multifile(guideline_unit/3).
:- discontiguous(guideline_unit/3).
:- multifile(guideline_exclusion/1).
:- discontiguous(guideline_exclusion/1).
:- multifile(guideline_supersession/4).
:- discontiguous(guideline_supersession/4).
:- multifile(guideline_evaluation_date/1).
:- discontiguous(guideline_evaluation_date/1).
guideline_schema_version(1).
guideline_document('cdc2022-opioid-s3-02',ace_sha256(bc5d30e9658d3a1bc51929eb528cd0ed9868fdc37a72fdf54b8465f53524f785),ulex(sha256(fba48e003101de64df33400ab2595fa7589360ac19f4c77ac3d17bd085f5b8c0)),vocabulary(sha256('335c132fd0e3d3163ba40c255e7db41eecc36492d8098263c505b17f44bbef5a'))).
% S1: If there is a recommendation and there is a clinician then it is recommended that the clinician does not apply the recommendation as an inflexible-standard.
guideline_operator(actual,'$guideline_id'(context,'cdc2022-opioid-s3-02',1,box(1),[A,B]),should) :- guideline_entity(actual,A,recommendation,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,clinician,countable), guideline_cardinality(actual,B,na,eq,1), \+ guideline_exclusion('cdc-2022-opioid').
guideline_operator('$guideline_id'(context,'cdc2022-opioid-s3-02',1,box(1),[A,B]),'$guideline_id'(context,'cdc2022-opioid-s3-02',1,box(2),[A,B]),-) :- guideline_entity(actual,A,recommendation,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,clinician,countable), guideline_cardinality(actual,B,na,eq,1).
guideline_entity('$guideline_id'(context,'cdc2022-opioid-s3-02',1,box(2),[A,B]),'$guideline_id'(product,'cdc2022-opioid-s3-02',1,ref(3),[A,B]),'inflexible-standard',countable) :- guideline_entity(actual,A,recommendation,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,clinician,countable), guideline_cardinality(actual,B,na,eq,1).
guideline_cardinality('$guideline_id'(context,'cdc2022-opioid-s3-02',1,box(2),[A,B]),'$guideline_id'(product,'cdc2022-opioid-s3-02',1,ref(3),[A,B]),na,eq,1) :- guideline_entity(actual,A,recommendation,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,clinician,countable), guideline_cardinality(actual,B,na,eq,1).
guideline_event('$guideline_id'(context,'cdc2022-opioid-s3-02',1,box(2),[A,B]),'$guideline_id'(product,'cdc2022-opioid-s3-02',1,ref(4),[A,B]),apply) :- guideline_entity(actual,A,recommendation,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,clinician,countable), guideline_cardinality(actual,B,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s3-02',1,box(2),[A,B]),'$guideline_id'(product,'cdc2022-opioid-s3-02',1,ref(4),[A,B]),1,B) :- guideline_entity(actual,A,recommendation,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,clinician,countable), guideline_cardinality(actual,B,na,eq,1).
guideline_arg('$guideline_id'(context,'cdc2022-opioid-s3-02',1,box(2),[A,B]),'$guideline_id'(product,'cdc2022-opioid-s3-02',1,ref(4),[A,B]),2,A) :- guideline_entity(actual,A,recommendation,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,clinician,countable), guideline_cardinality(actual,B,na,eq,1).
guideline_pp('$guideline_id'(context,'cdc2022-opioid-s3-02',1,box(2),[A,B]),'$guideline_id'(product,'cdc2022-opioid-s3-02',1,ref(4),[A,B]),as,'$guideline_id'(product,'cdc2022-opioid-s3-02',1,ref(3),[A,B])) :- guideline_entity(actual,A,recommendation,countable), guideline_cardinality(actual,A,na,eq,1), guideline_entity(actual,B,clinician,countable), guideline_cardinality(actual,B,na,eq,1).

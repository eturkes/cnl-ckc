cnl_guideline_mapping(1).
mapping_document('synthetic-guideline','doc-a',ace(relpath('tests/fixtures/mapping/doc-a.ace'),ace_sha256('0000000000000000000000000000000000000000000000000000000000000000')),ulex(relpath('tests/fixtures/mapping/doc-a.ulex'),ulex_sha256('1111111111111111111111111111111111111111111111111111111111111111'))).
mapping_document('synthetic-guideline','doc-b',ace(relpath('tests/fixtures/mapping/doc-b.ace'),ace_sha256('2222222222222222222222222222222222222222222222222222222222222222')),ulex(relpath('tests/fixtures/mapping/doc-b.ulex'),ulex_sha256('3333333333333333333333333333333333333333333333333333333333333333'))).
mapping_document('synthetic-guideline','doc-c',ace(relpath('tests/fixtures/mapping/doc-c.ace'),ace_sha256('4444444444444444444444444444444444444444444444444444444444444444')),ulex(relpath('tests/fixtures/mapping/doc-c.ulex'),ulex_sha256('5555555555555555555555555555555555555555555555555555555555555555'))).
mapping_region('synthetic-guideline','region-a').
mapping_region('synthetic-guideline','region-b').
mapping_region('synthetic-guideline','region-c').
mapping_region('synthetic-guideline','region-d').
mapping_claim('synthetic-guideline','region-a','claim.not-proved',projection(applicability),docid('doc-b'),items([query_id(sentence(2),clause(2)),rule_id(sentence(2),clause(1))]),expected_answer(answer(query_id(sentence(2),clause(2)),pred(requires,[named(person),named(review)]),not_proved))).
mapping_claim('synthetic-guideline','region-a','claim.proved',projection(applicability),docid('doc-a'),items([query_id(sentence(1),clause(3)),rule_id(sentence(1),clause(1)),rule_id(sentence(1),clause(2))]),expected_answer(answer(query_id(sentence(1),clause(3)),pred(applies,[named(patient)]),proved))).
mapping_claim('synthetic-guideline','region-c','claim.wh',projection(action_kind),docid('doc-c'),items([query_id(sentence(3),clause(2)),rule_id(sentence(3),clause(1))]),expected_answer(answer(query_id(sentence(3),clause(2)),wh(who),pred(eligible,[var(1)]),answers([pred(eligible,[named('z z')]),pred(eligible,[named(a)])])))).
mapping_residual('synthetic-guideline','region-a','residual.01',class(copula_head),detail(quote('Copula source wording.'),note('Requires a copula-head extension.'))).
mapping_residual('synthetic-guideline','region-a','residual.02',class(transitive_relation),detail(quote('Transitive relation source wording.'),note('Requires a transitive-relation extension.'))).
mapping_residual('synthetic-guideline','region-a','residual.03',class(property),detail(quote('Property source wording.'),note('Requires a property profile extension.'))).
mapping_residual('synthetic-guideline','region-b','residual.04',class(dose_quantity),detail(quote('Dose quantity source wording.'),note('Requires a dose-quantity constructor.'))).
mapping_residual('synthetic-guideline','region-b','residual.05',class(temporal),detail(quote('Temporal source wording.'),note('Requires a temporal constructor.'))).
mapping_residual('synthetic-guideline','region-b','residual.06',class(direction_strength),detail(quote('Direction and strength source wording.'),note('Requires a direction-strength constructor.'))).
mapping_residual('synthetic-guideline','region-c','residual.07',class(certainty),detail(quote('Certainty source wording.'),note('Requires a certainty constructor.'))).
mapping_residual('synthetic-guideline','region-c','residual.08',class(population_threshold),detail(quote('Population threshold source wording.'),note('Requires a population-threshold constructor.'))).
mapping_residual('synthetic-guideline','region-c','residual.09',class(labeled_exception),detail(quote('Labeled exception source wording.'),note('Requires a labeled-exception constructor.'))).
mapping_residual('synthetic-guideline','region-d','residual.10',class(multi_entity),detail(quote('Multiple entity source wording.'),note('Requires a multi-entity constructor.'))).
mapping_residual('synthetic-guideline','region-d','residual.11',class(disjunction),detail(quote('Disjunctive source wording.'),note('Requires a disjunction constructor.'))).
mapping_residual('synthetic-guideline','region-d','residual.12',class(scope_deferred),detail(quote('Deferred scope source wording.'),note('Requires a later scope decision.'))).

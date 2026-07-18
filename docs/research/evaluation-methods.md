# Evaluation methods for M5 acceptance

Plan-phase research findings (2026-07) for the symbolic ACE → APE DRS → project IR → Prolog pipeline. A metric earns a place only where it tests fidelity, completeness, deterministic execution, or evidence quality.

## 1. Acceptance claim

M5 should establish only:

> For the registered source/version and declared ACE subset, the pinned pipeline preserves reviewed recommendation semantics, compiles deterministically, answers registered queries correctly, replays each explanation from source-linked rules, and exposes every omitted normative candidate as a typed residual.

It does not establish clinical validity, patient benefit, full-document coverage, or unrestricted patient-care execution.

## 2. Layered metric family

One score is unsafe because later stages mask earlier errors. Report exact fractions with fixed denominators.

| Layer | Measures | Detects |
| --- | --- | --- |
| Source inventory | candidate; claimed/residual/silent-unclaimed counts | skipped recommendations |
| ACE profile | parse/profile acceptance; warning-free; canonical DRS match | unintended parser construction |
| DRS→IR | exact rule match; field precision/recall; provenance completeness | changed direction, bound, exception, scope |
| IR→Prolog | compile success; byte/hash stability; clause coverage | lowering loss/nondeterminism |
| Queries | answer accuracy; conflict precision/recall/F1; no-conflict reason accuracy | executable semantics errors |
| Evidence | proof replay; source-link and core/witness match | unauditable plausible answers |
| Reproducibility | fresh-process hash equality; manifest completeness | hidden state/tool drift |

Conformance gates and quality metrics remain distinct. Parse success is not faithfulness; a correct conflict verdict does not prove the rule is correct.

## 3. Gold item and adjudication

Each gold item contains source region+digest, reviewed ACE, expected DRS, versioned IR, canonical Prolog, positive/negative queries, expected answer/evidence, and adjudication record.

Use independent clinical/domain and formal-semantics review where practical. A third reviewer adjudicates every disagreement affecting population, action, direction, bound/operator/unit, exception, time, or source support. Report raw agreement plus Cohen’s kappa for categorical fields; for small/rare categories, raw counts matter more. Span work may add Krippendorff’s alpha or gamma. Published precedents report gamma near 0.94 for GGPONC 2.0 and inter-/intra-annotator F-measure at least 0.93 in clinical parsing; these are context, not binding thresholds.

## 4. Faithfulness metrics

### 4.1 Exact rule match

Canonicalize sets, IDs, rationals, units, and DNF order:

```text
exact_rule_match = 1 iff projected(actual_IR) = projected(gold_IR)
```

The projection includes action/target, direction, strength/certainty when present, every branch/atom, exact interval quantity/unit/value/side/openness, exceptions, and time. Exclude generated IDs only with an explicit alignment map. Score provenance separately, but require it for acceptance. One wrong direction, omitted exception, or strictness change fails exact match.

### 4.2 Field diagnostics

Report action/target, direction, strength/certainty accuracy; context-atom precision/recall/F1; exact interval accuracy; exception and source-region precision/recall; DNF branch exact-match. Keep direction and interval rows separate rather than averaging them into metadata.

### 4.3 Behavioral/evidence faithfulness

```text
behavioral_accuracy = correct answers / registered queries
proof_replay_rate   = replayable derivations / derivations emitted
```

Behavior is weaker than IR equality. Proof-of-concept runs (§7) produced perfect conflict verdicts with exact-IR faithfulness 0.70 versus 0.90 because wrong thresholds preserved overlap. Thus verdicts never replace exact IR comparison.

Conflict comparison includes category/kind, participating rules, same-action/opposing-direction core, overlap witness or reason-coded no-conflict evidence, and source regions. Compare canonical sets after ID alignment. Label a complete contributing set accurately if no minimal-core algorithm ran.

## 5. Degeneration probes

Every probe has positive and negative controls, so rejecting everything cannot pass.

| Family | Mutation | Expected observation |
| --- | --- | --- |
| Direction | `for` ↔ `against`/`contraindicate` | IR and paired conflict result change |
| Endpoint | `at least` ↔ `more than`; `at most` ↔ `less than` | openness changes |
| Dense rational | `18 < X < 19` | nonempty over rationals |
| Boundary | `age >= 18` vs `age < 18` | empty intersection; documented no-conflict |
| Unit | equivalent conversion; dimension mismatch | same canonical value; mismatch rejects |
| Exception | add/remove/change scope | conditioned applicability changes; raw overlap may not |
| Negation | explicit negative vs missing fact | classical negation remains distinct from NAF |
| DNF | reorder branches; `or`↔`and` | reorder invariant; connective mutation detected |
| Convention | synonym, oblique modality, age term | mapped semantics or typed residual |
| Provenance | move evidence label/sentence | semantic rule may stay; link changes |
| Parser profile | pronoun, anaphora, capitalized OOV, warning | fail-closed rejection |
| Unsupported field | recognized unmappable slot | typed residual/rejection, never silent drop |
| Clause order | reorder independent Prolog clauses | canonical bytes/answers stable |

Generated batteries need independent oracles. Interval tests cross value type with side/openness; shape-only masks can pass integer-only or marker-ignoring mutants vacuously. Every mutant needs an accepted base that reaches it.

## 6. Claim completeness and fresh-document recall

### 6.1 Region inventory

Independently label every source region normative-candidate or not. Each candidate ends exactly as accepted claim, typed residual, or silent-unclaimed failure.

```text
claim_completeness = (claimed + typed_residual) / normative_candidates
rule_recall        = claimed / normative_candidates
silent_omission    = silent_unclaimed / normative_candidates
claim_precision    = supported_claims / accepted_claims
```

`claim_completeness = 1` is a hard gate. Residuals make handling transparent; `rule_recall` reveals actual formalized coverage.

### 6.2 Fresh-document instrument

1. Set A authors mappings, ACE patterns, lexicon, and tests.
2. Freeze pipeline, evaluator, and mapping store.
3. Set B contains unseen recommendation passages.
4. Run with no Set B-driven edits.
5. Measure acceptance, exact faithfulness, rule recall, residual rate, false claims.

Any repair creates a new candidate/evaluator version. Stratify Set B by simple conjunction, DNF, interval, dose/unit, exception, contraindication, and temporal relation; aggregate recall can hide failure on a rare class.

## 7. Proof-of-concept lessons

A small plan-phase proof of concept yielded transferable priors:

- an IR boundary outperformed direct formal-target emission in a small synthetic task;
- grammar closed syntax/cross-field admission gaps but could not guarantee correct clinical direction;
- constrained multi-hop translation compounded errors;
- compact forms avoided repetition-loop truncation;
- invented DSLs were stably wrong on direction, proving stability without faithfulness;
- verdicts saturated before exact semantics, so exact IR comparison was essential;
- indirect phrasing/convention terms dented faithfulness; meanings belong in versioned data.

The PoC did **not** establish real-guideline performance, complete phenomenon coverage, clinician validity, out-of-sample generalization, or patient-care soundness. It used 20 synthetic require/forbid rules and 10 conflict/no-conflict pairs; some raw per-sample rows were not preserved. Treat its numbers as design priors.

## 8. M5 thresholds

Hard gates for the registered slice:

- 100% positive-case parse/profile acceptance and expected negative-case rejection;
- 100% canonical DRS, IR, and Prolog golden match after approved normalization;
- 100% registered query accuracy, including documented no-conflicts;
- 100% proof replay;
- `claim_completeness = 1`, `silent_omission = 0`;
- 100% canonical hash equality across at least two fresh processes;
- zero untyped failures or silently dropped supported fields.

Also report rule recall, field F1, residual rate, and pre-adjudication agreement. A residual can preserve completeness while exposing limited coverage.

## 9. Report artifacts

Emit canonical JSON plus deterministic Markdown containing source/version/digest and rights row; ACE/APE/SWI/compiler identities; evaluator/test digests; per-region claim/residual status; per-rule gold/actual diff; per-query answer, proof, core or no-conflict reason; raw numerators/denominators; stable failure codes; reviewer agreement/adjudication; replay command/hashes; explicit excluded sections and limitations.

Use `not_applicable` for zero denominators/inapplicable metrics, never a missing row. Aggregate only after preserving raw rows.

## 10. Public precedents

- EBM-NLP: `https://github.com/bepnye/EBM-NLP`.
- GGPONC 2.0: `https://aclanthology.org/2022.lrec-1.389/`.
- ProofNet: `https://arxiv.org/abs/2302.12433`; BEq/BEq+: `https://arxiv.org/abs/2406.07222`.
- Metamorphic testing: Chen, Cheung, and Yiu, 1998; QuickCheck: Claessen and Hughes, 2000.
- CPG-on-FHIR: `https://hl7.org/fhir/uv/cpg/`.

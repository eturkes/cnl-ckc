# Provenance, trust, and licensing

M0.1 governance baseline. `MUST`/`MUST NOT` = normative.

## Pins

| upstream | repo URL | pinned commit | license | role | vendor status |
|---|---|---|---|---|---|
| e-- | `https://github.com/frmoded/e--` | `da8c3b34d2493180da8df65b127a3841f9a4e609` | Apache-2.0 | canonical E-- → derived Python compiler | fork vendored M1 |
| APE | `https://github.com/Attempto/APE` | `5f4d5354a45fb772763bf1a9543f508f15b28982` | LGPL-3.0-or-later | ACE → DRS parser, isolated SWI-Prolog process | fork vendored M2 |
| Clex | `https://github.com/Attempto/Clex` | `20960a5ce07776cb211a8cfb25dc8c81fcdf25e2` | GPL-3.0 | general-English lexicon candidate | **EXCLUDED** — decision below |
| AceRules | `https://github.com/tkuhn/acerules` | `5b7afb7bdfbce56027997307f9b798af53551223` | LGPL-3.0; `only`/`or-later` option unstated | semantics reference only: courteous logic + stable models | never vendored; no code reuse; reuse ⇒ license-review escalation |
| RACE | no source repository; Attempto webservice only | — | N/A — no acquired source | historical reasoner reference only | zero dependency; reasoner = first-party M4 |
| SWI-Prolog | `https://github.com/SWI-Prolog/swipl-devel` | `V9.2.9` → `e3b19512e69a544f05b1bffbd14f3a0b519ad04d` | BSD-2-Clause | system Prolog runtime | never vendored |

AceRules license evidence at its pin: `LICENSE.txt`, first nonblank line = `GNU LESSER GENERAL PUBLIC LICENSE`; next line = `Version 3, 29 June 2007`. `README.md` states `The code is available under the LGPL license. See LICENSE.txt for the details.` License family + version are verified; repository-level text does not select SPDX `only` vs `or-later`. Code reuse remains prohibited unless escalated for license review.

## Roles + trust

- E-- MUST be the sole authored source for all first-party Python glue, tests, and tooling.
- Generated Python, IR, and Prolog = byte-accepted derived artifacts. MUST NOT hand-edit; corrections MUST enter canonical E--, canonical ACE, the IR specification, or the relevant compiler stage, then regenerate.
- Python generation MUST use the deterministic compiler path only: normalization off; zero `{{ ... }}` slots before emission.
- ACE → DRS → IR → Prolog MUST be deterministic; accepted IR/Prolog bytes come only from that path.
- ACE MUST be the sole authored source for every clinical or guideline-specific rule.
- Hand-authored Prolog allowlist = APE-fork adapter; DRS → IR lowering; IR → Prolog compiler; validators; explanation machinery; generic inference kernel. Clinical rule content MUST NOT enter this layer.
- Python = byte transport, orchestration, and integration only. It MUST NOT inspect DRS semantics or encode clinical semantics.

## TCB boundary

TCB = vendored, hash-pinned e-- compiler fork + its unchanged external dependencies. This narrow boundary is the sole manually maintained Python exception; every first-party Python file outside it MUST compile from canonical E--.

Vendored APE = trusted parser input to the compilation pipeline, but **not** part of the Python TCB: it runs in a separate SWI-Prolog process; Python moves bytes only and never inspects DRS.

Every TCB change requires all three gates:

1. bump the fork-commit pin in provenance;
2. regenerate every accepted derived Python artifact;
3. rerun full acceptance: strict path, normalization off, slot-free input, deterministic byte equality, and generated-artifact drift checks.

A compiler dependency-set/hash change is a TCB change and follows the same gates.

## Vendor-directory license layout

Directories land in M1/M2; this layout is binding before import.

### `vendor/e--/`

- retain upstream Apache-2.0 `LICENSE` verbatim;
- carry `PROVENANCE`: upstream URL, pinned fork commit, and local-patch policy;
- local patches MUST be enumerated, minimal, and independently re-applicable.

### `vendor/ape/`

- retain upstream LGPL-3.0-or-later `COPYING` files verbatim;
- fork modifications remain LGPL-3.0-or-later;
- carry `PROVENANCE`: upstream URL, pinned fork commit, and local-patch policy;
- local patches MUST be enumerated, minimal, and independently re-applicable;
- containment posture: APE runs as a separate SWI-Prolog process over stdio/files → LGPL-covered code + modifications stay inside `vendor/ape/`; Python exchanges bytes only.

A future root license MUST NOT replace or obscure either vendor license.

## Clex decision

**Status: DECIDED — exclude Clex.**

Rationale:

- Clex = GPL-3.0 → propagation risk + licensing complexity conflict with the intended Apache-2.0 first-party / LGPL APE posture.
- APE already bundles LGPL `prolog/lexicon/clex_lexicon.pl` with roughly a few thousand entries → sufficient bootstrap coverage.
- Clinical vocabulary is domain-specific → a project-owned declarative lexicon manifest is required regardless; compile it deterministically to APE user-lexicon facts consumed through `-ulexfile`.

Revisit trigger: M5 guideline authoring demonstrates general-English coverage gaps that the project manifest cannot economically fill. Required response = create a **BLOCKED** proposal for user decision; do not import Clex implicitly. Proposal options: accept GPL for an isolated lexicon-only artifact, or generate coverage from a permissively licensed wordlist.

## Top-level license — proposal only

**Status: PROPOSAL, requirement-level, awaiting user decision. No top-level license is selected.**

Proposed default = Apache-2.0 for all first-party work: matches e-- upstream + supplies an explicit patent grant. Vendor directories retain their own licenses; future root `LICENSE` + `NOTICE` MUST document the split. Alternative = MIT.

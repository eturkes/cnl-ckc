# Full-text reference corpus

This directory retains primary sources used to design the deterministic ACE front end,
rule compiler, and clinical-guideline authoring workflow. All local hashes are SHA-256
over the exact stored bytes. Retrieval date for every item is **2026-07-18**.

Copyright remains with the authors and publishers. A file's public availability does not
by itself grant redistribution rights; follow the item-specific note below.

## Shiffman et al. — clinical guidelines in ACE

- **Citation:** Richard N. Shiffman, George Michel, Michael Krauthammer, Norbert E.
  Fuchs, Kaarel Kaljurand, and Tobias Kuhn. “Writing Clinical Practice Guidelines in
  Controlled Natural Language.” In *CNL 2009 Workshop*, Lecture Notes in Artificial
  Intelligence 5972, pp. 265–280. Springer, 2010.
- **DOI:** <https://doi.org/10.1007/978-3-642-14418-9_16>
- **Local file:** `shiffman2010-writing-cpg-in-cnl.pdf`
- **SHA-256:** `c699922ad967cd73d46455cb3e46905c0417844a1759cd3caedd0af2e0321cc0`
- **Rights:** Springer-copyright material; retained for internal research use; do not
  redistribute.
- **Relevance:** Primary evidence for M2 and M5. It reports an independent translation
  experiment over all eleven key-action statements in a pediatric urinary-tract-infection
  guideline, the clinical vocabulary and modality extensions that ACE required, and the
  proposed predictive authoring workflow.

## Fuchs, Kaljurand, and Kuhn — ACE 6.7 DRS specification

- **Citation:** Norbert E. Fuchs, Kaarel Kaljurand, and Tobias Kuhn. *Discourse
  Representation Structures for ACE 6.7*. Technical report, University of Zurich, 2013.
- **URL:** <https://attempto.ifi.uzh.ch/site/pubs/papers/drs_report_67.pdf>
- **Local file:** `drs-report-67.pdf`
- **SHA-256:** `85839ff5266504e40012cdc7e8484a99f7b1238c76e00831b8253346dc27b0b4`
- **Rights:** Freely downloadable from the Attempto project site; the report states no
  explicit reuse license. Retain as a research reference and verify rights before external
  redistribution.
- **Relevance:** Normative input for M2's ACE-to-DRS adapter and canonical DRS format. It
  documents the reified flat representation, implication, strong negation, negation as
  failure, modality, subordination, and discourse-reference behavior that the adapter must
  either preserve or reject explicitly.

## Kuhn — AceRules

- **Citation:** Tobias Kuhn. “AceRules: Executing Rules in Controlled Natural Language.”
  In *Web Reasoning and Rule Systems: First International Conference, RR 2007*, Lecture
  Notes in Computer Science 4524, pp. 299–308. Springer, 2007.
- **DOI:** <https://doi.org/10.1007/978-3-540-72982-2_24>
- **Public full text:**
  <https://attempto.ifi.uzh.ch/site/pubs/papers/kuhn07acerules.pdf>
- **Local file:** `kuhn2007-acerules.pdf`
- **SHA-256:** `69b3575450abedabb94963ec0cfd270a29f1b0f31a5e93c3580aa359c6d0ccae`
- **Rights:** Springer-copyright material; retained for internal research use; do not
  redistribute.
- **Relevance:** Reference architecture for M4: ACE rules compiled to executable rule
  semantics, with controlled-language answers and traces. It is useful for separating
  strong negation from negation as failure and for designing proof-derived explanations;
  its semantics are precedent, not an implicit specification for this project.

## Kuhn — survey and PENS classification

- **Citation:** Tobias Kuhn. “A Survey and Classification of Controlled Natural
  Languages.” *Computational Linguistics* 40(1):121–170, 2014.
- **DOI:** <https://doi.org/10.1162/COLI_a_00168>
- **Landing page:** <https://aclanthology.org/J14-1005/>
- **Local file:** `kuhn2014-cnl-survey.pdf`
- **SHA-256:** `befc7ebb73c5bfb9de26f66b7a836465de7e6ae8de0fd4e527051165dbd44855`
- **Rights:** ACL Anthology materials published before 2016 are distributed under
  CC BY-NC-SA 3.0; reuse must preserve attribution, noncommercial use, and share-alike
  terms.
- **Relevance:** Landscape source for M2 and M5. The PENS dimensions make the trade-off
  explicit: an ACE-derived clinical profile should sacrifice general-English coverage for
  deterministic interpretation, a compact exact specification, and domain-fixed semantics.

## Kaljurand — ACE as a Semantic Web language

- **Citation:** Kaarel Kaljurand. *Attempto Controlled English as a Semantic Web
  Language*. Dissertationes Mathematicae Universitatis Tartuensis 55. Tartu University
  Press, 2007.
- **DOI:** <https://doi.org/10.5167/uzh-33194>
- **Repository record:** <https://hdl.handle.net/10062/4876>
- **Local file:** `kaljurand2007-ace-semantic-web.pdf`
- **SHA-256:** `9e283a87a3ec8bad0831214d1de2025e7b53e525332566d1eb74d0d060ba4e00`
- **Rights:** Publicly downloadable university-repository copy; copyright is retained by
  the author and no explicit reuse license is supplied. Retained for internal research use;
  do not redistribute without checking permission.
- **Relevance:** Deep M2 background on APE, ACE lexicons, DRS and ontology mappings, and
  the engineering boundary between controlled surface language and formal representation.

## ACE 6.7 language documentation — URL only

- **Citation:** Attempto Group. *Attempto Controlled English 6.7 documentation*.
  University of Zurich, 2013.
- **Official pages:**
  - [ACE in a Nutshell](https://attempto.ifi.uzh.ch/site/docs/ace_nutshell.html)
  - [Construction Rules](https://attempto.ifi.uzh.ch/site/docs/ace_constructionrules.html)
  - [Interpretation Rules](https://attempto.ifi.uzh.ch/site/docs/ace_interpretationrules.html)
  - [Syntax Report](https://attempto.ifi.uzh.ch/site/docs/syntax_report.html)
  - [Lexicon Specification](https://attempto.ifi.uzh.ch/site/docs/ace_lexicon.html)
- **Local file:** not fetched — URL only. The authoritative reference is published as HTML;
  no official standalone ACE 6.7 language-reference PDF was located.
- **SHA-256:** not applicable.
- **Rights:** Publicly readable project documentation with no explicit document reuse
  license on the pages. Link rather than redistribute.
- **Relevance:** Operational specification for M2's accepted sentence forms, fixed
  attachment and scope rules, content-word lexicon facts, and unsafe fallback behavior that
  the clinical profile must turn into hard errors.

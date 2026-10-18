# license-tag — archives ship LICENSE beside NOTICE (contract; archived at unit close)

Row = reviewer-5 K7-F1 (nt1 review, `.agent/archive/review-nt.md` K7): release archives ship NOTICE, which names the root LICENSE, without LICENSE itself. Accept (the row's): LICENSE travels in every archive as a tag file beside NOTICE (release spec tag list + K4 custody rows), `dist_cases` asserts its bytes equal HEAD's LICENSE, the release manifest re-derives.

## Tier

shell (archive assembly = enumerated shell tier). No theorem changes: `ckc_spec::release::release_manifest` + `tagmanifest_lines` take the tag set as input ⇒ LICENSE gains its K4 `member` row + tagmanifest digest through the existing proofs. Spec edit = tag-list prose only → spec-manifest re-pin. Shallow ⇒ MAIN implements + self-reviews (code diff lenses).

## Design

- D1 `pipeline_release::derive`: tags = bagit.txt, README-dist.md, NOTICE + `LICENSE` = `git show HEAD:LICENSE` bytes verbatim; absent → `dist: no-input LICENSE` (NOTICE's law). Bag name + epoch keep the source-commit pathspec (guidelines + compiler + lexicon), as for NOTICE.
- D2 Release manifest gains `member	LICENSE	<sha256>	<size>`, sorted with the other members (`LICENSE` < `NOTICE` bytewise); tagmanifest-sha256.txt lists LICENSE; member meters +1.
- D3 `dist_archive::extract` (ckc check's bag validator) admits `LICENSE` at the bag root.
- D4 Every synthetic repo that derives a release plan commits a LICENSE: `dist_probes` seed, dist_cases `base_repo`, c1_surface `ExportRepo`, nt1-surface `make_repo`.
- D5 Spec prose: `ckc-spec/src/release.rs` tag-list comments name LICENSE; `ckc trust-audit --write rust` re-pins spec-manifest (escape rows unchanged).
- D6 Docs: REFERENCE § Export payload list + § Operating step 5 release inputs, `ops.md` release-input list, `rust.md` dist_cases note name LICENSE.
- D7 `release-manifest.tsv` re-derived (`ckc release-manifest`).

## Predicates

- P1 dist_cases: a12 committed_state dirties the working LICENSE → bag LICENSE == `git show HEAD:LICENSE`, dirty bytes absent; bag_layout requires LICENSE; a02 bagit_closure member closure includes LICENSE (K4 member row); consumer_copy LICENSE == the fixture's committed bytes. Red on the unfixed binary (`CKC_DIST_BIN=` a 83569b1d build), green on the unit.
- P2 Live: `ckc dist build` at the unit commit → `<bag>/LICENSE` cmp-equal to HEAD's LICENSE; `sha256sum -c manifest-sha256.txt tagmanifest-sha256.txt` OK incl. LICENSE; the manifest's LICENSE row digest + size = LICENSE's.
- P3 `ckc check` green (manifest fresh, dist probes, two byte-identical live builds, extraction); full `just gate` green at the unit commit; deferred row pruned in it.

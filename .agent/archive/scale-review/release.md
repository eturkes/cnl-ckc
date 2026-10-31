# scale-release

Scope = committed release/repository/CI/test slice at `9129d719`; concurrent orientation commits excluded from the test inventory. SC1–SC8 fixed before target reads. Targets = manifest/amend (M), BagIt/dist (D), publication/external limits (P), repository/history (R), CI/gate orchestration (C), corpus-dependent integration harnesses (T). 48/48 target×check rows adjudicated. `ok` includes a checked non-applicability; `revise` = scale-plan work, not a claim that today's one-guideline release is wrong. `revise-before-2nd-guideline` = an existing one-guideline assumption that another guideline can invalidate.

Notation: B = committed guideline bytes; A = release members; N = tracked paths; Z = compressed bag bytes. ×100/×1,000 = independent CDC-sized envelopes, not forecasts. Decimal MB/GB unless MiB/GiB stated. Measurements = one bounded local process at a time; CI timings = existing run `41022878056`, not a new gate. Full gates/check/certify/test were not run.

| id | component (file:line) | operation | check | scaling | cost ×1 | projected ×100 / ×1,000 | breaks at | revision needed | verdict |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| S1 | M `rust/ckc-kernel/src/release_rows.rs:197–245` | manifest ordering + rights partition | SC1 | **Θ(A²) insertion shifts + Θ(A·G) profile lookup**, derived; primitive ×1/2/4 digest = 1.99/6.49/24.85 ms | 1,517 synthetic payload members; full CLI 0.32 s | ~152k/1.52m members; quadratic sort term ×10⁴/10⁶ | far before 1,000: one sort projects ~20 s/~2,000 s from small probe, not a measured large run | verified O(A log A) sort or sorted-input construction; indexed guideline/profile lookup | revise |
| S2 | M `rust/ckc/src/check/pipeline_release.rs:382–406`; `check/process.rs:64` | derive manifest | SC2 | O(B+A+L) RAM; complete tar, cloned subprocess input, all member bytes; no streaming | tar 13,987,840 B; manifest CLI peak 38,468 KiB | bulk tar+members alone ~2.7 GB/~27 GB; peak coefficient host/allocator dependent | RAM-dependent; ×1,000 exceeds ordinary runner memory | stream git members into digest metadata; avoid storing payload for a digest-only command | revise |
| S3 | M `release_rows.rs:197–245,247–261` | sorting + serialization | SC3 | executable sort/partition loops; decimal recursion ≤20 digits for u64 | no G-driven call stack | same stack depth | no G threshold identified | retain iterative implementation; recursive ghost spec is erased, not a runtime stack claim | ok |
| S4 | M `pipeline_release.rs:311–493`; `justfile:1003–1017` | amend after any release-input commit | SC4 | re-archive, read and hash **all** guidelines even for one changed ledger row; no digest cache | 0.32 s; all 12.8 MB guidelines read | each tiny change repeats global work; sort dominates at large A | scales with every corpus edit, not changed bytes | content-addressed per-member/per-guideline metadata; deterministic root manifest | revise |
| S5 | M `rust/ckc/src/check/dist_check.rs:27–75` | release law inside `ckc check` | SC5 | derive once + two live builds, each deriving again; Θ(A²) work repeated ≥3 times | existing CI dist meter at +17 s after spec-lint, includes fixture probes | unoptimized sort term alone minutes/~hours at 1,000 | large A can hit child build watchdog before 6 h job cap | reuse validated plan within one check; shard/cache component attestations while retaining double-build equality | revise |
| S6 | M `pipeline_release.rs:428–444` | merge rights + labels | SC6 | global BTreeMaps; duplicate docids explicitly refused; all guidelines represented | 492 labels | 49,200/492,000 labels | no missing composition property in this custody-only operation identified | keep global uniqueness/root binding; emission/answer semantics belong to their own gates | ok |
| S7 | M `justfile:1003–1017`; `docs/REFERENCE.md:1786–1788` | review-batch commits | SC7 | one global generated file changes for every ledger/input batch; global rederive per batch | one line, one guideline | independent authoring/review streams contend on one generated file | multi-guideline parallel work | per-guideline manifests + deterministic aggregate generation; retain posted-commit custody | revise |
| S8 | M `release-manifest.tsv:12`; `pipeline_release.rs:513–521` | commit monolithic manifest | SC8 | measured 233,737 B = 1,522 member +492 label +11 meta rows; Θ(A+D) | 0.234 MB blob | ~23.4 MB/~233.7 MB | **~449 CDC-like guidelines crosses GitHub's >100 MiB regular-blob block**; inside ~850 scope | keep each committed manifest bounded by a guideline/chunk; root lists digests, not every document | revise |
| S9 | D `check/dist.rs:153–175`; `dist_archive.rs:55–75` | build tar.gz | SC1 | O(B·compression cost + A²); level-9 single-stream gzip; manifest + digest sorts | actual build 0.73 s; 1,930,593 B | bytes ~193 MB/~1.93 GB; cannot extrapolate whole time linearly because sort is quadratic | A, then compression/IO | streaming archive with O(A log A) metadata; assess compression level under deterministic-byte contract | revise |
| S10 | D `dist.rs:153–178`; `dist_archive.rs:83–88`; `dist_check.rs:76–91` | build, compare, extract | SC2 | payload B + gzip Z; check holds derived payload, two compressed outputs and full inflated tar | build peak 37,864 KiB; extraction peak not measured | tens of GB at 1,000; coefficient not measured | host RAM well before archive-format size ceiling | streaming gzip to pending file; incremental compare/hash; streaming validated extraction | revise |
| S11 | D `dist_archive.rs:62–75,100–182` | archive member loop | SC3 | iterative build/extract; nested path mkdir depends path depth, not A | bounded layout depth | unchanged stack with more guidelines | no G-driven recursion found | preserve iterative member validation | ok |
| S12 | D `dist_check.rs:27–91`; `dist.rs:36–58,153–175` | deterministic-build gate | SC4 | 3 global derives +2 full compressions +inflate+all checksums; hashes members again for BagIt | 2 fresh byte-identical builds required | all unchanged bytes repeated every check | gate growth from every corpus addition | reuse plan, content-keyed component artifacts; preserve two-build comparison as grading check | revise |
| S13 | D `dist_probes.rs:105–116` | child dist build watchdog | SC5 | child uses shared 300 s wall limit; global Θ(A²) manifest work | 0.73 s local build | quadratic primitive projects ~2,000 s per large sort at 1,000, already >300 s | precise G unmeasured; stress envelope fails model before job cap | remove global quadratic work, then size-aware chunked export; do not merely raise watchdogs | revise |
| S14 | D `dist.rs:36–58`; `pipeline_release.rs:428–493` | reject/contested guard, rights partition | SC6 | considers all exported guideline labels/profiles; one global root bound to all members | global custody check present | same property over all G | no per-guideline-only hole identified here | retain digest/rights/label guarantees when partitioning | ok |
| S15 | D `dist.rs:52–58`; `docs/REFERENCE.md:1810–1832` | release a full snapshot | SC7 | one rejected/contested document blocks the single all-G archive; every candidate rebuilds/downloads all G | 492 documents in snapshot | 49,200/492,000 documents coupled to one release line | review/release concurrency, not a measured wall limit | decide versioned component releases + a composition/root release record; preserve clinician-review prerequisite | revise |
| S16 | D `dist_archive.rs:11–35,62–68` | tar representation | SC8 | size field = 11 octal digits (<8 GiB/member); total archive has no analogous fixed cap; LongLink covers long names | largest guideline blob 1.42 MB | current largest per-doc/source size stays same under G replication | only an individually huge member, not G itself | no immediate archive-format revision; streaming/resource issues are S10/S13 | ok |
| S17 | P `justfile:499–518` | rename/copy + sidecar | SC1 | O(Z) extra copy/hash; no nested G scan in wrapper | 1.93 MB file | 193 MB/1.93 GB copied+hashed | IO/bandwidth dependent | wrapper itself adequate; solve underlying global build separately | ok |
| S18 | P `justfile:512–518` | install + sha256sum | SC2 | file-stream tools; wrapper does not read archive into shell variable | bounded wrapper memory | bounded apart from delegated `ckc dist build` | no wrapper RAM limit found | retain file streaming | ok |
| S19 | P `justfile:499–519` | publication helper | SC3 | fixed shell sequence | fixed | fixed | none driven by G | none | ok |
| S20 | P `justfile:510–518` | repeated release candidate | SC4 | every call delegates fresh global dist build, then copies and hashes again | 0.73 s dist + wrapper IO | all-G bytes repeated for unchanged tag content | expensive at large snapshot | re-use a verified content-addressed archive when same input root/epoch; do not weaken identity binding | revise |
| S21 | P `.github/workflows/ci.yml:94–100`; `docs/REFERENCE.md:1826–1832` | upload/deposit | SC5 | CI uploads ckc only; release assets + Zenodo deposit remain user actions, not a hidden CI upload | two published KB assets | upload time ∝Z; no per-guideline job here | no extra 6 h failure attributable to publication helper itself | expose publication size preflight; orchestration failures are C rows | ok |
| S22 | P `justfile:516–518`; `docs/REFERENCE.md:1746–1749` | checksum/signature identity | SC6 | checksum binds the entire archive independent of number of guidelines | one SHA-256 line | one line | no cross-guideline identity gap identified | retain root identity if archive split into components | ok |
| S23 | P `docs/REFERENCE.md:1813–1832` | notes, tag, GitHub, DOI | SC7 | human notes require source edition/digest **of each guideline**; release/tag/deposit steps otherwise constant | one source record | 100/1,000 source records per notes version | human maintenance burden; rate unmeasured | derive machine-checked source inventory from manifests; concise human release summary; user still authorizes remote actions | revise |
| S24 | P primary sources E1/E2 | GitHub asset + Zenodo record limits | SC8 | GitHub: <2 GiB/file, ≤1,000 assets/release; Zenodo: default 50 GB/record, ≤100 files, extra quota up to total 200 GB | 1,930,593 B archive +105 B sidecar | 193,059,300 B/1,930,593,000 B archive | **not broken at ×1,000** under this compression; GitHub ceiling ~1,113 replicas; larger PDFs move it earlier | add size preflight; plan sharded archives/root record before single asset cap; ≤99 archive parts plus one root would fit Zenodo's file count | revise |
| S25 | R `.github/workflows/ci.yml:33–37,121–125`; `justfile:562` | checkout/history scan | SC1 | checkout/index traversal ∝N+B; full secret scan ∝history/changed-byte stream, not just HEAD; no supported universal git-runtime exponent | HEAD 12,006 paths, 58,863,796 B; H=815 | HEAD ~1.33 GB/~12.86 GB if non-corpus files fixed; history growth additional | workload/host dependent | separate immutable corpus objects from code workflows; retained custody/history proof; measure fresh-clone/scan slope | revise |
| S26 | R HEAD inventory; `c1_surface.rs:326–330` | materialize working trees | SC2 | working-tree disk O(B+N), shared git objects do not share checked-out bytes | current guidelines 12,814,651 B | guideline bytes ~1.28 GB/~12.81 GB, before checkout copies | disk/memory-map budget; git RAM curve not measured | bound checked-out corpus per job; content/object-backed reads | revise |
| S27 | R inventory paths | directory traversal | SC3 | G increases sibling dirs, not depth; deepest tracked path has 11 separators (whole repo) | bounded structural depth | unchanged under replication | no G-driven stack failure identified | preserve bounded layout; term depth is a separate dimension | ok |
| S28 | R `.github/workflows/ci.yml:36,124,184–188`; `justfile:562–564` | repeated git transfer/audit | SC4 | rust +check fetch full history independently; certify is default shallow checkout, **not a third full-history fetch**; history scanned anew each gate | 41,069,607 B HEAD-reachable on-disk git objects; local all-ref packs ~119 MB differ | repeats enlarged corpus/history per run | increasingly IO-bound | job-specific content/custody caches; incremental secret scan with equal-strength baseline/full coverage | revise |
| S29 | R `justfile:562`; `.github/workflows/ci.yml:36,124` | CI history cost | SC5 | H and historical PDF/generated revisions accumulate even if current G unchanged | gitleaks 9 s in existing run; fresh checkout timing not isolated | projection unconfirmed: no H×G clone benchmark run | bounded by job time/disk eventually | benchmark H separately; avoid unconditional unchanged-history rescan while retaining audit guarantee | revise |
| S30 | R `pipeline_release.rs:440–443` | cross-guideline namespace | SC6 | global duplicate-docid guard exists; paths namespace guideline files | 492 distinct docids | global uniqueness remains checkable | no repository-only composition gap identified | preserve global ids/root identity; semantic joins are other slices' remit | ok |
| S31 | R `docs/REFERENCE.md:1782–1808`; `justfile:1003–1017` | collaborate on corpus/ledgers | SC7 | review rows merge per guideline but global manifest touches all streams; no measured reviewer rate | one guideline's ledger stream | many independent ledger batches/manifest churn | process contention, before capacity hard limit | per-guideline custody artifacts, deterministic root merge; retain ancestor/digest attestation | revise |
| S32 | R `release-manifest.tsv:1`; primary source E3 | repository/blob growth | SC8 | regular blobs >100 MiB blocked; recommended repository <1 GB ideally, <5 GB strongly (recommendations, not hard repository caps) | 1.42 MB source PDF; PL 9.62 MB; fixtures 9.38 MB; no current >100 MiB blob | raw checkout ~1.33 GB/~12.86 GB, compressed history cannot be inferred from raw | manifest ~449 guidelines; any future source PDF >100 MiB can fail earlier; raw-size advice crossed ~75/~387 CDC additions, **not a packed-git forecast** | per-guideline manifests + immutable source/object storage; a naive LFS switch would make current `git archive` hash/export pointer bytes and needs custody/export adaptation | revise |
| S33 | C `justfile:969–999`; `.github/workflows/ci.yml:107–202` | full gate scheduling | SC1 | global check + sequential compile/queries and certify loops over G; only check∥certify jobs after rust | check 21.15 min; certify 9.16 min | linear components alone exceed hosted-job budget at ×100; global quadratic work worsens | see S37 | content-keyed work units; bounded shards; explicit global composition stage | revise |
| S34 | C `.github/workflows/ci.yml:31,110,171`; `c1_surface.rs:326–330` | per-job resources | SC2 | distinct full source checkouts; some rust tests hold 8 checkout copies + many SWI corpora; not mitigated by the binary artifact seam | checkout 58.86 MB, plus tools/build/fixture scratch | c1 pool alone ~10.6 GB/~102.9 GB raw checkout copies (non-corpus files fixed) | disk/RAM budget, actual runner capacities not probed | sparse/bounded fixture roots; cap parallel memory; keep full composition checks isolated | revise |
| S35 | C `.github/workflows/ci.yml:28–202` | workflow execution stack | SC3 | fixed job DAG + iterative recipe loops | fixed DAG depth | fixed | none driven by G | none | ok |
| S36 | C `justfile:435–453,530–550,910–999` | unchanged code + corpus gates | SC4 | verifier cleans/rechecks every code revision (G-independent); entire tests/mobile/check/certify still rerun for data-only change | verify 1,399 s; tests 1,969 s; mobile ~92 s | **do not multiply all 3,521 rust seconds by G**; only data-dependent subset grows | throughput before hard cap | content-keyed verifier/test/corpus attestations with full dependency identities + planted-bad firing; keep each grading property | revise |
| S37 | C `justfile:979–982,996`; `.github/workflows/ci.yml:107–202`; E4 | hosted-job deadline | SC5 | current no matrix; primary hosted max =6 h/job; Kani alone has explicit 60 min | certify guideline 417.44 s +fixed cases 132.05 s; fresh compile 345.94 s +queries 27.89 s | certify ~11.6 h/~116 h; fresh compile+queries alone ~10.4 h/~104 h; check remainder additional | **~52 guidelines certify**, ~58 guidelines fresh compile+queries alone, at same throughput; earlier under global overhead | split work across bounded batches/jobs, cache unchanged attestations; preserve global adversarial/composition gate | revise |
| S38 | C `rust/ckc/tests/consumer_recipe.rs:100–108`; `justfile:996` | per-guideline emission vs composition | SC6 | certify is per-guideline emission correspondence; all-G patient composition already exercised by consumer_recipe/SGW/BF, so it is not absent | one guideline today makes distinction invisible | corpus mixtures need correct oracles + native new-mode coverage | before independent G2 makes T's assumptions visible | keep per-document emission checks; add/repair explicit mixed-guideline clinical worlds for IMPLEMENT acceptance | revise-before-2nd-guideline |
| S39 | C `.github/workflows/ci.yml:15–18`; `justfile:30–66` | review/commit gate latency | SC7 | all-G gate on each closing commit; one rust bottleneck before both corpus jobs; ci-ref cancellation avoids obsolete rerun, not this cost | ~1 h rust chain then ~21 min check | days per full run under unchanged linear work, or job fails | already long; scale worsens | fast changed-input gate + cached/sharded full-strength close; separate code proof gate from data attestations | revise |
| S40 | C `.github/workflows/ci.yml:94–100`; E4 | workflow/artifact size | SC8 | current upload = static ckc, not corpus; fixed 3 non-weekly jobs; hosted matrix ≤256 jobs/workflow | one small binary artifact, retention 1 day | no G-driven uploaded artifact size today | a future one-job/guideline matrix cannot represent 850 guidelines in one matrix | use ≤256 bounded batches or child workflows, not naive G-row matrix | ok |
| S41 | T inventory T1–T12 below | live-corpus harnesses | SC1 | DA2 fresh parses/compiles Θ(GD); SGW/BF byte census ∝C; global recipe sorts/materializes answer sets; fixed fixtures/history are not ×G | 492 docs +6 questions; DA2 suite 396 s total (mixed fixed/live) | at least 49,200/492,000 doc checks in each live census; no whole-suite timing multiplier justified | data passes/child watchdogs, then S45 argv | cache by input+implementation/oracle digests; shard local laws; retain mixed-composition property stage | revise |
| S42 | T `consumer_recipe.rs:263–285,325–349`; `tests/consumer/driver.pl.in:45–49` | all-G SWI loads + outputs | SC2 | patient-world case spawns 8 whole-corpus SWI processes; supersession case adds3; libtest can overlap these with other tests; `findall` retains all answers before sort | SWI RSS not measured; exact process fan-out source-derived | O(8B+table/answer state), potentially more simultaneous; logical-state blow-up not bounded by file bytes | memory dependent; not a measured limit threshold | bounded concurrency + reusable immutable load/evaluation snapshots; stream/set-aware comparison where contract permits | revise |
| S43 | T `tests/da2/support.rs:45–56`; `tests/bf/prolog.rs:53–95` | traversal/term reading | SC3 | file recursion depends depth (layout-bounded); BF parser recursion depends individual term/list depth, not number of guidelines | no G-driven stack counterexample | G replication leaves term/path depth unchanged | individual pathological term may still exhaust stack; not tested here | no G-specific stack revision; retain separate term-depth acceptance | ok |
| S44 | T `tests/da2/preservation.rs:236–328`; `tests/sgw/census.rs:157–207` | replay unchanged corpus | SC4 | DA2 runs old+new dump per ACE, another compile per guideline ACE, answer+trace per query; SGW re-reads all emissions; no content cache | ≥3 parser passes/live ACE, before check/compile/certify repeat them | ≥147,600/1,476,000 document-parser launches from these three passes alone | gate latency + unnecessary subprocess setup | reusable compiler service/worker + oracle-keyed cached attestations, not removal of cases | revise |
| S45 | T `consumer_recipe.rs:128`; `tests/sgw/census.rs:15`; `bf_consumers.rs:83`; `tests/da2/preservation.rs:331` | launch corpus load / query census | SC5 | **one `-g` argument contains every path**; Linux page-size-derived per-argument limit; DA2 separately pins total Q=6 | exact reduced goal 44,119 B passes; ×3 132,217 B fails errno7 | ×100≈4.4 MB/×1,000≈44 MB cannot exec; DA2 fails as soon Q≠6 | **~G3 with current paths; G2 with any added question** | write loader/goal to file/stdin; parameterize per-guideline Q inventory with fixed historical subset if needed | revise-before-2nd-guideline |
| S46 | T `consumer_recipe.rs:239–253,263–304`; `tests/consumer/driver.pl.in:1–17` | all-G acceptance oracles | SC6 | CDC excluded-world oracle requires **global** should/must/may=0; isolated per-guideline answer pins compared against all-G KB; generic queries can legitimately gain answers | G1 masks distinction; reduced semantic counterexample red | any independent applicable G2 can violate valid behavior | **G2**, not a performance issue | qualify expected exclusions/boxes by guideline; distinguish per-guideline demos from explicit combined-composition answers; mixed positive +negative controls | revise-before-2nd-guideline |
| S47 | T `rd_disclosure.rs:143–168,427–452`; `rev_k.rs:152–184,741–768` | golden-page upkeep | SC7 | RD/rev-k are **fixture/frozen-history pages**, not current corpus pages; new fixtures require pin/name-list upkeep, unrelated to each new guideline | RD base21 +one post-base; REV-K272 pins | constant with G unless fixtures deliberately expanded | no per-guideline human golden-update requirement found | retain bounded fixtures; add generated scale/mixed-world cases rather than one copied UI golden per guideline | ok |
| S48 | T `check_mutants.rs:533–569`; `c1_surface.rs:326–330`; T inventory | HEAD clone scratch | SC8 | full working trees per worker despite shared object DB; check-mutants ≤4, c1 pool8, s1/s2 each≤4 in their processes | c1 pool ~471 MB raw copies today | ~10.6 GB/~102.9 GB c1 raw copies; other pools also repeat work serially | disk budget before thousands of guidelines | minimal self-contained validator fixtures/sparse staged trees; keep a separate full-HEAD integration control | revise |

## Ranked summary

1. **Before G2 — S45/S46/S38 (major):** DA2 total-Q=6 and global CDC-exclusion oracle encode the one-guideline corpus. Separate isolated demonstration pins from actual mixed-guideline answer contracts. The small cross-guideline counterexample is red on the current global-zero oracle.
2. **~G3 — S45 (major):** absolute corpus paths embedded in one argv string stop exec at 132,217 B on this Linux host. SGW P1, consumer_recipe and BF SWI loads share the mechanism. File/stdin loaders solve this without weakening tests.
3. **Tens of guidelines — S33–S39/S41/S44/S48 (major):** sequential whole-guideline compile/certify alone violate 6 h/job around 52–58 CDC-equivalents; repeated parser passes and full-HEAD fixture clones add work. Partition by input identity + dependency/oracle identity, with a distinct mixed-composition gate.
4. **Hundreds — S1/S5/S9/S13 (major):** verified release code uses insertion sort and linear profile lookup. At ~1.5m members a single quadratic sort projects ~2,000 s; dist child wall cap=300 s. This is a derived stress estimate, not a large-scale run.
5. **~449 — S8/S32 (major):** one committed release-manifest crosses GitHub's 100 MiB blob block. This is a concrete repository-hosting limit **inside the stated ~850-guideline goal**. Split metadata before it, independently of the engine redesign.
6. **×1,000 memory/disk — S2/S10/S26/S34/S42/S48 (major):** tar/member materialization grows to tens of GB; 8 full-HEAD checkout copies alone ~103 GB. Stream export metadata/payload and bound test working sets/concurrency.
7. **~1,113 compressed CDC equivalents — S24 (minor for present envelope):** GitHub's single-asset limit. ×1,000 produces ~1.93 GB, still below 2 GiB; Zenodo 50 GB default is not close for this envelope. Future PDFs/compression vary. Add preflight and root+component release identity, rather than claiming publication already fails at ×1,000.
8. **Process — S7/S15/S23/S31/S39 (minor, scale-plan decision):** global manifest/release coupling and manual per-G notes. Clinician review remains necessary; no review-rate or staffing forecast was measured.

## Finding evidence

### S1/S5/S9/S13 — quadratic verified release ordering

Severity = major; acceptance = release/check remains operable at the target corpus size. Exact anchor bytes:
- `rust/ckc-kernel/src/release_rows.rs:214`: `out.insert(i, x);`
- `rust/ckc-kernel/src/release_rows.rs:235`: `r = insert(x, r);`
- `rust/ckc-kernel/src/release_rows.rs:136`: `let profile = lookup_exec(profiles, &g);`

`sorted` walks input backwards. On already ascending input it inserts each next smaller path at index0 and shifts all prior EMember structs: A(A−1)/2 moves of a 56-byte struct. At 1,517 members =64,393,616 shifted bytes; 100×/1,000× ≈644 GB/64 TB shifted per sort, before paths/digests/output allocation. Sorting is not recursive at runtime. `partition` calls the linear profile lookup for each member, adding Θ(A·G).

Probe = `scale-release/release-bench.rs`, directly linked against the existing pinned 1.99.1 release rlib; no target source edited, no cargo build. Seed = current member metadata, synthetic distinct guideline paths/profiles; all redistributable; no claim of a fully validated replicated corpus. Commands + results in `scale-release/release-bench.log`:

```
release-bench <repo>/release-manifest.tsv 1  # manifest 0.004128 s; digest 0.001990 s; 4,220 KiB
release-bench <repo>/release-manifest.tsv 2  # manifest 0.008951 s; digest 0.006488 s; 5,840 KiB
release-bench <repo>/release-manifest.tsv 4  # manifest 0.031002 s; digest 0.024854 s; 8,708 KiB
```

All rc0. Initial compile used shell default rustc1.97.1 against rlib1.99.1 and failed E0514; setting `RUSTUP_TOOLCHAIN=1.99.1` compiled successfully. No large ×100/×1,000 benchmark run; timing extrapolations are conditional, throughput/allocator/cache may change. Proof is the executable algorithm, not a fitted empirical exponent.

### S2/S4/S10/S12 — unnecessary global materialization

Severity = major. Exact bytes:
- `pipeline_release.rs:384`: `&["archive", "--format=tar", "HEAD", "guidelines"],`
- `pipeline_release.rs:403`: `fs::read(&archived).map_err(|_| error("no-input archive"))?,`
- `check/process.rs:64`: `let input = input.unwrap_or_default().to_vec();`
- `dist_archive.rs:59`: `.write(Vec::new(), Compression::new(9));`
- `dist_archive.rs:86`: `.read_to_end(&mut tar)`

The manifest-only operation builds a complete export plan containing all member bytes. Build adds compressed bytes in memory; check derives another plan, invokes build twice, retains/computes archive bytes, inflates a whole tar, then runs sha256sum. Existing-file collision also reads full old compressed archive (`dist.rs:124`). Red correctness test not applicable: current bytes are correct; this is a source-proven resource-growth finding.

Actual CLI smoke in an isolated **no-new-commit** shared clone under probe; `TMPDIR` also inside probe:
- `ckc release-manifest`: rc0, 0.32 s, peak38,468 KiB; `cmp` against committed manifest rc0.
- `ckc dist build <probe>/dist`: rc0, 0.73 s, peak37,864 KiB, 1,525 members, 1,930,593 B, sha256 `c8cc65d6e2b28f350bffde0f6933e14c1946e97556bc1311da95555720eb50a5`.
- These are hot local runs; no claim that disk-cold/CI throughput matches them. Dist extraction/check peak was not measured because full `ckc check` was outside the brief.

### S8/S32 — one tracked manifest outgrows regular Git blobs

Severity = major. `release-manifest.tsv` contains 2,025 lines/233,737 B: 1,522 member,492 label,11 meta. Derivation writes one global file (`pipeline_release.rs:513–521`); amendment commits that same file (`justfile:1016`). `233,737×1,000=233,737,000 B >104,857,600 B`. Approximate crossing =449 equally-sized guideline inventories; fixed meta/shared tags make the exact crossing slightly different. This is a size projection plus primary hosting contract, **not an attempted remote push**. E3 supplies the hard file limit. An unbounded monolithic source PDF can independently reach it at any G.

### S45 — executable argument ceiling + fixed global Q

Severity = major. Source anchors matched with `rg -Fn`:
- `rust/ckc/tests/consumer_recipe.rs:128`: `.arg(format!("consult([{files}]), {goal}"))`
- `rust/ckc/tests/bf_consumers.rs:83`: `.arg(format!("consult([{files}]),{goal}"))`
- `tests/sgw/census.rs:15`: `&format!("consult([{names}]),writeln(loaded({})),halt.", files.len()),`
- `tests/da2/preservation.rs:331`: `assert_eq!(queries.len(), 6);`

Red launch-envelope test (copies the target's path quoting/join mechanism; uses `/usr/bin/true` to avoid doing a large Prolog load):

```
python3 -I <probe>/argv-red.py <repo> 1
# docs=492 goal_bytes=44119 factor=1
# PASS exec rc=0
python3 -I <probe>/argv-red.py <repo> 3
# docs=1476 goal_bytes=132217 factor=3
# FAIL exec errno=7 Argument list too long; host page_size=4096
# rc=1
```

The more detailed inventory probe gave ×2=88,167 B success and ×4=176,265 B errno7 (one-space-different goal). The per-string limit matters before aggregate `ARG_MAX` (here4,194,304 B). Replication duplicates path bytes only: it tests exec capacity, not semantic behavior of duplicate guidelines. Extra loader/driver paths and longer CI checkout paths cannot improve this bound. No full integration suite was run.

DA2 P9's `inputs` comes from recursively enumerating all `guidelines` ACE; `queries` filters by parent-directory name. The exact-six assertion is unconditional and thus rejects a second guideline with a committed question. This is a text-exact contract assumption; no full DA2 suite run was needed/allowed to establish the arithmetic.

### S46 — composition oracle confuses one guideline's exclusion with every guideline

Severity = major. `consumer_recipe.rs:100–108` loads every guideline into each patient world. The excluded flag comes from the four **CDC** worlds (`:263–269`). At `:296–300` it requires `(n("should"), n("must"), n("may")) == (0,0,0)` over the whole composition. At `:302` a CDC-positive world likewise assumes there is no exclusion of any loaded guideline. These are not valid all-guideline invariants.

Reduced semantic red test (constructed typed namespace fixture; **not a certified ACE compilation**):

```
swipl -q -f none -F none -s <probe>/scope-red.pl -g run -t 'halt(9)'
# exclusion=1 should=1
# FAIL consumer_recipe global excluded-world oracle requires should=0
# rc=1
```

The second guideline's box is deliberately independent of CDC's exclusion. This is correct product behavior and false under the harness oracle. The target Rust test itself was not run; the probe isolates its global-zero predicate.

Separately `consumer_recipe.rs:245` builds each question/answer comparison, then `:249` uses the all-guideline load. `tests/consumer/driver.pl.in:1–17` requires exact equality with the committed answer set. Compilation/pinning of those questions is per-guideline; e.g. `guidelines/cdc-2022-opioid/queries/pl/category-a-recommendations.pl:4` is a generic noun/event query, not restricted by a guideline id. A later guideline can legitimately add matches. This second risk is **derived, not exercised with a second compiled guideline**.

### S33–S40 — deadline + scheduling evidence

Severity = major for deadline, scale-plan decision for cache/scheduling. Source = `scale-release/ci-41022878056.log` captured with read-only `gh run view --log`; rc0. Primary cap = E4. Existing run:

| component | source timestamps / duration | scaling qualification |
| --- | --- | --- |
| verify | 1,399 s | code/proof size, **not** G |
| tests | 1,969 s; slowest DA2 396.05, check-mutants243.19, S2 227.52, C1 192.70, S1 120.76 s | fixed fixtures + live-corpus work mixed; do not assign entire total to G |
| rust recipe total | 3,521 s | includes constant proof work |
| mobile | ~92 s | HEAD render + three page-level browser passes, fixture controls; G-dependent live pages, other UI slice owns renderer |
| certify guideline |08:56:13.127→09:03:10.566 =417.44 s | sequential per-guideline loop |
| certify cases |09:03:10.566→09:05:22.615 =132.05 s | current fixed mutation table |
| check entry→check ok |08:56:20.434→09:11:07.495 =887.06 s | mixture of global/per-guideline/fixed probes |
| ui check |09:11:07.495→09:11:15.770 =8.27 s |495 pages |
| fresh compile |09:11:15.770→09:17:01.710 =345.94 s | sequential outer G loop, workerized docs inside |
| queries |09:17:01.710→09:17:29.601 =27.89 s |per-guideline query load/evaluation |

Conditional unchanged-throughput equations: certify `132+417.44G`, so G52 >21,600 s; fresh compile+queries alone `373.83G`, so G58 >21,600 s **before** check/UI overhead. No ×100/×1,000 run. Current jobs `check` and `certify` both need `rust`; no matrix. Only Kani specifies `timeout-minutes:60`; hosted defaults still impose the 6 h limit. `just mobile` performs HEAD probe + full-corpus no-viewport and no-narrow-rule probes (`justfile:919,946–947`); it is another live-corpus scan, not fixed UI goldens.

## Integration-test census

Scope = all 50 committed paths under `rust/ckc/tests/` (includes `support/nt_projection.rs`) at audit baseline, plus their `#[path]` source modules. Directory-traversal/HEAD-clone/source-root references inspected; fixture-only carriers distinguished from live corpus. Newly added `imp_characterization.rs` belongs to concurrent IMPLEMENT work and was not audited here. This is a source census, not a full execution census.

| id | suite / source anchor | dependence + work | at ×100 / ×1,000 | disposition |
| --- | --- | --- | --- | --- |
| T1 | `sgw_acceptance.rs:4` → `tests/sgw/census.rs:20–74,157–207`; `support.rs:161–164` | P1 co-loads schema+allG docs five times (no/dose/adult/minor/cancer); P2 each corpus+fixture+green file through independent+kernel readers/speller; P3 fixed historical 492-doc population | P1 paths49,200/492,000 and argv failure much earlier; P2 ~100/1,000× live bytes; P3 not ×G | S41/S44/S45 |
| T2 | `consumer_recipe.rs:100–108,239–381` | ~14 all-corpus SWI loads/test-binary (8 world/mode +3 dates +questions +dose +plain); table/findall/sort; 900 s outer run,120 s per open goal | large whole-KB tables/RAM; Q and all-G semantics differ; argv fails first | S42/S45/S46 |
| T3 | `bf_consumers.rs:45–86,111–172` | three global SWI loads +two native full-composition calls; each dose answer gets own composition; fixed 20/60/120 s controls | repeated allG load; targeted native engine cost belongs engine slice; SWI argv failure | S42/S44/S45 |
| T4 | `bf_emission.rs:394–433` → `tests/bf/prolog.rs:53–95,123–157` | scans allG PL, builds one doc's parsed clause tree, canonicalizes/reorders every rule through independent scheduler |49,200/492,000 docs; memory bounded to a doc, term depth distinct | shard/cache local body law; keep independent model |
| T5 | `da2_acceptance.rs:11` → `tests/da2/preservation.rs:236–376` | P8 all guideline+fixture ACE: old+current DRS parse; P9 all guideline ACE: fresh compile; per query answer+trace loads its guideline; **global Q=6**; four workers (`preservation.rs:9–23`) |≥3 parser passes/doc→147,600/1,476,000 launches for documents alone; Θ(Q·D_g) query load; hard Q assumption immediate | S41/S44/S45; no dropped preservation cases |
| T6 | `check_mutants.rs:484–569` | ≤4 full HEAD clones;109 mutant checks reset between rows; scanner often traverses all corpus before first-violation pin; 180 s watchdog | Θ(N) setup/prefix checks per fixed mutant; disk4× working tree; not109× full SWI stage | minimal mutant corpus roots +one full-HEAD control |
| T7 | `c1_surface.rs:264–280,326–330`; `s1_compiler.rs:641–686`; `s2_compiler.rs:501–544` | C1 pool8 and S1/S2 ≤4 full HEAD clone workers; fixed table mutants execute global `ckc check` prefix; compiler/alias fixtures themselves fixed | checkout/prefix work grows G, fixture case count does not | S34/S48; de-couple table law from unrelated corpus |
| T8 | `cs1_surface.rs`→`tests/cs1-surface/table.rs:338–397`; `nt1_surface.rs`→`tests/nt1-surface/release.rs:97–127`; `sup1_surface.rs`→`tests/sup1-surface/table.rs:611–704` | each clones full HEAD to test scoped/table/schema laws, then stops at fake SWI; local surface/portability scenes fixed | setup/all-corpus check prefixes grow; not complete SWI corpus cost for each case | S48; minimal law fixtures +integration control |
| T9 | `dist_cases.rs:1954–2113` | three live HEAD-derived clones for live/blocked/amended integration cases; global writer +global check release prefix; many other archive fixture cases bounded | repeated global archive/manifest algorithms; clone disk ∝B | S1/S4/S48; preserve live deterministic-build control |
| T10 | `eso_acceptance.rs:39–81` | explicitly loads only CDC, substitutes one fixed document, answers one fixed question | constant in added G; grows only if CDC itself grows | retain bounded historical/characterization role; not allG engine coverage |
| T11 | `rd_disclosure.rs:143–168,427–452`; `rev_k.rs:152–184,741–768`; `ur_ui.rs`; `sup4_ui.rs`; `n22_viewport.rs`; `n23_separators.rs`; `ui_fixtures.rs:341` | renders fixed/frozen/live **fixture trees** and generated tally/route cases; historical archives/tests/ui, not guidelines HEAD | no automatic G growth; baseline revisions/pins grow only with chosen fixtures/UI features | S47; do not multiply these pins/pages by G |
| T12 | remaining acceptance/codec/align/trust/copy/CLI/socket/lock/commit/recipes/v1 suites | explicit fixtures, generated domains or fixed schema/compiler files; `ui_socket.rs:330` clones its **tiny fixture**, not project HEAD; trust copies rust workspace | G-independent except deliberately expanded fixture set or code/schema revisions | no hidden whole-corpus loop found in those carriers |

## Repository measurements

`scale-release/inventory.py` uses `git ls-tree -rlz HEAD` metadata, not generated caches. `inventory.log` captured baseline:

| committed category | files | bytes |
| --- | ---: | ---: |
| all HEAD |12,006|58,863,796|
| guideline PL |492|9,617,517|
| guideline ACE |492|273,325|
| align |492|443,864|
| audit |6|451,541|
| queries (ACE/PL/answers/traces) |24|27,301|
| source files (PDF+normalized/evidence) |6|1,700,876|
| guideline/root metadata |6|300,227|
| tests |8,368|9,384,961|
| rust (source+vendored dependencies, target excluded) |1,798|25,824,339|

Guidelines total =12,814,651 B; one source PDF1,418,584 B; largest compiled doc457,770 B. Current all-ref object-store packs116,477 KiB; HEAD-reachable on-disk objects41,069,607 B; these are different populations. H=815 at baseline. Git compression/delta/history costs cannot be extrapolated by multiplying today's whole `.git` by G. Fixtures are mostly fixed9.38 MB; a fixture per future corpus artifact would add another Θ(G) population, but that policy is not assumed.

## External primary evidence

Sources visited read-only in owned Neo tabs; verbatim source text captured with `webtext`, rc0. Files under `scale-release/sources/`. No authenticated account change, upload, publish, or remote mutation.

- **E1** GitHub, About releases — https://docs.github.com/en/repositories/releasing-projects-on-github/about-releases ; `gh-releases.txt:27`: **“Up to 1000 release assets may be associated with a single release. Each file included in a release must be under 2 GiB. There is no limit on the total size of a release, nor bandwidth usage.”** This release-specific page supplies the asset limit; the general large-file page's release paragraph instead refers to a plan's LFS limit, so prefer the explicit release contract rather than silently conflating them.
- **E2** Zenodo, Manage files — https://help.zenodo.org/docs/deposit/manage-files/ ; `zenodo-files.txt:1,4`: **“You can upload up to a 100 files and a total volume of 50GB (50,000,000,000 bytes).”** **“We support file uploads up 50GB (= 50,000 MB) and max 100 files. If you would like to upload up to 200GB of data, please see Manage storage quota.”** Manage storage quota — https://help.zenodo.org/docs/deposit/manage-quota/ ; `zenodo-quota.txt:1`: **“Each record comes with a default storage quota of 50GB and you can upload an unlimited number of records under 50GB. For larger files, you also have an additional allowance of up to 150GB that can be distributed across your uploads as needed.”** The100-file limit counts uploaded files, not members inside one tar.gz. No project Zenodo record quota/DOI was inspected; actual account allowance unconfirmed.
- **E3** GitHub, About large files — https://docs.github.com/en/repositories/working-with-files/managing-large-files/about-large-files-on-github ; `gh-large-files.txt:7,13,23`: **“If you attempt to add or update a file that is larger than 50 MiB, you will receive a warning from Git.”** **“GitHub blocks files larger than 100 MiB.”** **“We recommend repositories remain small, ideally less than 1 GB, and less than 5 GB is strongly recommended.”** Repository advice is not stated as a universal hard1/5GB limit.
- **E4** GitHub, Actions limits — https://docs.github.com/en/actions/reference/limits ; `gh-actions-limits.txt:25`: **“All GitHub-hosted runners | Job execution time | 6 hours | Each job in a workflow can run for up to 6 hours of execution time. If a job reaches this limit, the job is terminated and fails.”** `:13`: **“A job matrix can generate a maximum of 256 jobs per workflow run. This limit applies to both GitHub-hosted and self-hosted runners.”**
- Current asset readback: `gh release view kb-2026-10-29-rc.1 --repo eturkes/cnl-ckc --json assets`: archive1,930,593 B; sidecar105 B. This is a read, not an upload-size experiment.

## Verification + limits

Passed: fixed check-set source audit; Rust LSP documentSymbol on real dist.rs; local manifest derivation byte-equal to committed manifest (`cmp` rc0); actual dist build smoke rc0; primitive ×1/×2/×4 probes rc0; positive argv exec rc0; primary text extraction rc0; existing CI log/asset reads rc0. Intentionally red: argv ×3 rc1/errno7; constructed global-exclusion oracle rc1. Not run: full `just gate`, `ckc check`, `ckc certify`, `just test`, full individual integration suites, ×100/×1,000 workloads, fresh network clone, real remote oversize upload/push, large-H benchmarks, SWI RSS/composition performance, dist-check extraction RSS. Derived risks are labelled; fixed fixture suites are not misrepresented as live-corpus scans.

Coverage limits: test inventory anchored to baseline50 committed paths; concurrent IMPLEMENT tests excluded. Source searches used bounded output for orientation, then opened all identified live-corpus/HEAD-clone mechanisms; remaining fixture-only classification is a source census, not exhaustive dynamic filesystem tracing. Header/history/pack/config behavior of git internals not reimplemented or fully profiled. Small timing probes are warm single runs, not medians or a throughput guarantee. Report does not assert a forecast for guideline size, parser hardness, clinical answer count, review effort or packed history. The probe rlib predates concurrent orientation-only commits; `git diff 9129d719 HEAD` over audited release/kernel/recipe/workflow/test sources was empty (rc0). Probe CLI clone HEAD = `eda3f56c`; release inputs match baseline (`git diff --exit-code 9129d719 HEAD -- guidelines release-manifest.tsv vendor/ape/prolog/ace_to_pl.pl`, rc0). No target repository edits or commits by this reviewer.

reviewer-5-DONE-1

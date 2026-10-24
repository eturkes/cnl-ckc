# Pre-release request — closed Tasks

Request: "Address all `.agent/deferred.md` rows relevant for pre-release of this repo. Once you have confirmed everything is complete, make the pre-release." Outcome: the five deferred rows bearing on the cut landed; the cut waits for SUP (routed to IMPLEMENT) + SKO (user rulings, AskUserQuestion: SUP first; Skolem role entities before the cut). Plan + rulings = 065d814b; route = the commit archiving this file.

- [x] 83ffd544 RR1 test fail-fast (deferred row; pre-release request): `just test` runs every binary (`--no-fail-fast`) + names each failed one.
- [x] c81f0b14 RR2 absolute home paths in tracked files (deferred row; portability law): repo-relative archive text, k5-sound generators take the repo root.
- [x] 4910f7ff RR3 published static `ckc` for reviewers (deferred row, security review R2): path-trimmed reproducible musl build, release assets + digest, README reviewer path.
- [x] 3726eed9 RR4 signed release tags (deferred row, security review P4; user ruling: SSH-signed, MAIN signs with the user's key): allowed signers, § Releasing, consumer verify command.
- [x] b6d22fa8 RR5 reviewer disclosure (deferred row, security review D3; user ruling: the form states it; kernel contract `rd`).

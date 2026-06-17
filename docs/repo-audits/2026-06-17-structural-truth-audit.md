# OcentraParent Structural Truth Audit

Date: 2026-06-17
Branch: `codex/tracking-plan-full-continuation-a`
Purpose: stable repo-level audit reference before reviewing per-plan thread self-assessments.

This document records structural issues that can make plan reports look green when the repo is not actually done. It is intended to be updated as each plan-thread report is reviewed.

## How to use this audit

Use this file when checking any plan report that claims completion, readiness, or proof coverage.

A plan report is not accepted as true until it matches:

1. actual source ownership;
2. real executable tests;
3. tracked proof generator commands;
4. local or CI run evidence;
5. architecture-gate status;
6. known dependency boundaries;
7. no stale checklist/proof wording.

Generated proof artifacts under `output/` and `test-results/` are intentionally untracked. Missing generated artifacts in Git are not a defect by themselves. A defect exists only when a report claims proof without a tracked command, local/CI evidence, and a clear pass/fail rule.

## Audit baseline

| Area | Current concern | Review rule |
| --- | --- | --- |
| Test topology | Empty `.gitkeep` folders and broad risk-category scaffolds exist. | Count only real executable tests. Empty folders do not count. |
| Rust tests | Some crates keep many tests inside `src/` with `#[cfg(test)]`. | Public behavior should be tested through crate-level `tests/` where feasible. Source-adjacent tests require justification. |
| Required-test gate | Current required-test presence check is too weak. | Passing the presence check does not prove coverage. |
| Architecture policy | Repo bans re-exports, but many crate/package roots still use them. | Distinguish changed-scope pass from repo-wide clean. |
| CI coverage | CI is segmented and does not equal full local validation. | Identify exact CI job or local command that covered the claim. |
| Package coverage | Root package build/test scripts are hand-maintained and can omit real packages. | Every package/crate needs a build/test/CI matrix entry or explicit exclusion. |
| DRY/common code | Similar runtime event handoff code repeats across sibling crates. | Add tests first, then extract common behavior if duplication is confirmed. |
| Plan ownership | Broad frontage packages can hide owner drift. | Identify actual owning crate/package/app before accepting closure. |

## Known structural findings

| ID | Finding | Why it matters | Action |
| --- | --- | --- | --- |
| ST-01 | Empty test scaffold folders exist under crate test trees. | Folder presence can be mistaken for real coverage. | Inventory `.gitkeep`-only test folders and delete or replace with real tests. |
| ST-02 | The required-test script allows weak false green. | A crate can pass with inline tests only; a package can pass with one unrelated test. | Add a stricter topology audit that reports real test files by package/crate/risk area. |
| ST-03 | Rust inline tests are heavy in `agent-core` and `agent-protocol`. | Tests can become source-adjacent implementation scaffolds instead of public API proof. | Classify each inline test as valid private seam or move candidate. |
| ST-04 | Rust `pub use` appears in many crate roots despite no-reexport rules. | Repo-wide architecture-clean claims are suspect. | Decide whether to enforce globally now, stage cleanup, or document explicit exceptions. |
| ST-05 | CI Rust coverage is segmented. | Some workspace crates may not be covered by PR CI jobs. | Build a crate-to-CI matrix and add missing coverage or mark local-only. |
| ST-06 | Domain package coverage is narrower than package count. | Some packages may not be in `build:contracts` or `test:contract`. | Build a package-to-command matrix and fail on omitted workspaces. |
| ST-07 | Repeated child-domain runtime event handoff patterns exist. | Duplicated logic can diverge across app, app-game, browser, and network cores. | Test current behavior, then consider common helper/core extraction. |
| ST-08 | Proof destinations are intentionally untracked. | GitHub cannot prove local generated artifacts by file presence. | Require tracked generator commands and local/CI run evidence in PR_READY reports. |

## Thread report review template

Use this template for every pasted thread self-assessment.

| Field | Finding |
| --- | --- |
| Plan |  |
| Claimed status |  |
| Actual source owners |  |
| Misplaced source/tests |  |
| Real executable tests |  |
| Empty scaffold folders |  |
| Inline `src` tests |  |
| Proof generator commands |  |
| Generated proof destinations |  |
| Local run evidence |  |
| CI coverage |  |
| Architecture gate status |  |
| DRY/common-core concerns |  |
| Dependency blockers |  |
| Verdict |  |
| Best next slice |  |

## Verdict vocabulary

| Verdict | Meaning |
| --- | --- |
| `done` | Source, tests, proof generator, run evidence, docs, and dependency boundaries agree. |
| `partial` | Real work exists, but closure is blocked by missing tests, proof, dependencies, or ownership drift. |
| `false-green` | Docs/checklists/report say done, but code/tests/proof do not support it. |
| `missing` | Claimed surface has no meaningful implementation or executable proof. |
| `blocked` | Work cannot honestly proceed until a predecessor contract/proof/decision lands. |

## Immediate audit work queue

1. Create a full test-topology inventory for `crates/`, `packages/`, and `apps/`.
2. Create a crate/package-to-CI matrix.
3. Create a source ownership drift map for broad frontage packages and repeated runtime patterns.
4. Decide the Rust/TypeScript re-export policy state: global cleanup, staged cleanup, or explicit exceptions.
5. Review each pasted plan-thread report against this file before accepting any completion claim.

## Current working rule

Do not accept a plan completion report from checklist counts, folder scaffolds, generated proof paths, or optimistic docs alone. Accept only source-backed, test-backed, proof-generator-backed, and locally/CI-run-backed claims.

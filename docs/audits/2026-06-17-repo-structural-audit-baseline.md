# Repo Structural Audit Baseline

Date: 2026-06-17
Branch: `codex/tracking-plan-full-continuation-a`

This checkpoint preserves repo-level structural audit findings before per-plan thread reports are reviewed.

## Audit scope

| Area | Check |
| --- | --- |
| Test topology | Empty scaffold folders, real executable tests, misplaced source-adjacent tests. |
| Validation gates | Whether local and CI commands actually cover the claimed crate/package/plan. |
| Architecture policy | Re-export/barrel violations and whether a claim is changed-scope or repo-wide. |
| Ownership | Code or tests living in broad aggregate packages instead of the narrow owner. |
| DRY | Repeated runtime or protocol patterns that should be common-core candidates. |
| Proof | Generated proof artifacts are intentionally untracked; claims need tracked generator commands and local/CI run evidence. |

## Baseline findings

| ID | Finding | Risk | Follow-up |
| --- | --- | --- | --- |
| F-01 | Empty `.gitkeep` test scaffold folders exist under crate test trees. | Folder presence can be mistaken for test coverage. | Inventory and delete or replace with real tests. |
| F-02 | Required-test presence check is weak. | A crate/package can pass with only inline tests or one minimal test file. | Add a stricter test-topology audit. |
| F-03 | Some Rust crates have large `#[cfg(test)]` source-adjacent test sets. | Public behavior may not be tested through public crate APIs. | Classify inline tests as valid private seam or move candidates. |
| F-04 | Rust no-reexport policy conflicts with many crate roots using `pub use`. | Architecture-clean claims can be false unless scoped narrowly. | Decide global policy and run a dedicated cleanup or exception decision. |
| F-05 | CI Rust jobs are segmented and do not equal `cargo test --workspace`. | CI may not cover every crate touched by a plan. | Build crate-to-CI coverage matrix. |
| F-06 | Domain package CI and root contract scripts appear hand-maintained. | New packages can be omitted from build/test gates. | Build package-to-CI coverage matrix. |
| F-07 | Similar runtime event handoff logic repeats across sibling crates. | Divergence and duplicate fixes. | Add tests first, then consider common-core extraction. |
| F-08 | Generated proof roots are intentionally gitignored. | GitHub-only review cannot inspect local generated artifacts. | Require tracked generator command plus local/CI run evidence in reports. |

## Thread report review table

For each pasted thread report, fill:

| Field | Required answer |
| --- | --- |
| Plan | Exact plan folder. |
| Claimed status | Self-assessed status. |
| Actual source owners | Exact crates/packages/apps. |
| Tests present | Real executable tests only. |
| Empty scaffolds | Empty folders or `.gitkeep` placeholders. |
| Inline tests | Valid private seam or move-to-tests candidate. |
| Proof generator | Exact tracked command/script. |
| CI coverage | Covered, local-only, or uncovered. |
| Architecture gate | Changed-scope, package-scope, or repo-wide. |
| DRY/ownership concern | Duplicate logic or misplaced source. |
| Verdict | Done, partial, false-green, or missing. |
| Next slice | Smallest useful closure slice. |

## Working conclusion

Plan reports should be reviewed through source, executable tests, tracked proof generators, CI/local command evidence, and architecture gates. Do not accept checklist counts, folder scaffolds, or generated-artifact paths alone as closure.

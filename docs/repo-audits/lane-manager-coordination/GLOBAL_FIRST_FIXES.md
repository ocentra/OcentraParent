# Global First Fixes

These fixes run before broad plan implementation. They are not optional because many per-plan self-assessments report the same false-green mechanisms.

## G0 Archive hygiene

| Problem | Action | Owner |
| --- | --- | --- |
| Legacy numbered self-assessment duplicates can be mistaken for canonical files. | Keep only canonical `*-selfaudit.md` as review inputs, or move legacy numbered files to an ignored/quarantine note. Update `per-thread-self-assessments/INDEX.md`. | Lane manager |

## G1 Test topology inventory

| Problem | Action | Owner |
| --- | --- | --- |
| Empty `.gitkeep` folders are counted by humans as coverage. | Generate inventory of empty scaffold folders under `crates/*/tests`, `packages/*/tests`, and `apps/*/tests`. | Repo-audit WP01 |
| Rust tests live inline in `src` and are claimed as plan proof. | Classify inline tests as valid private seam or move-to-`tests/` candidate. | Repo-audit WP01 |
| TS packages pass with one unrelated test. | Map package exports to real test files where feasible. | Repo-audit WP01 |

## G2 CI/package coverage matrix

| Problem | Action | Owner |
| --- | --- | --- |
| CI segmented Rust jobs do not equal `cargo test --workspace`. | Matrix every crate against local commands and CI jobs. | Repo-audit WP02 |
| Root `build:contracts` and `test:contract` are hand-maintained. | Matrix every package and fail or flag omissions. | Repo-audit WP02 |

## G3 Architecture policy reconciliation

| Problem | Action | Owner |
| --- | --- | --- |
| Rust `pub use` and TS re-export debt conflict with no-reexport policy. | Decide global cleanup, staged cleanup, or explicit exceptions. | Repo-audit WP03 |
| Scoped passes are overreported as repo-wide clean. | Require architecture claim scope in every plan report. | Lane manager |

## G4 Ownership/orphan/legacy map

| Problem | Action | Owner |
| --- | --- | --- |
| Broad frontage packages hide owner drift. | Inventory `parent-domain`, `portal-domain`, `agent-protocol-domain`, `agent-core`, `agent-protocol`. | Repo-audit WP04 |
| Old pre-eventing/transitional code can shadow new owners. | Add `why does this file still exist?` classification: owner, adapter, frontage, legacy, delete candidate. | Repo-audit WP04 |

## G5 DRY/common-core map

| Problem | Action | Owner |
| --- | --- | --- |
| Similar runtime event-chain assembly repeats across app/app-game/browser/network and may spread to tracking/screen. | Inventory copies and required tests before extraction. | Repo-audit WP05 |
| Similar proof/manual-required state machines repeat across plans. | Identify common vocabulary only where semantics match. | Repo-audit WP05 |

## Exit condition for global phase

The lane manager may start broad plan execution only after G1-G5 are either complete or explicitly assigned to lanes with path ownership locked.

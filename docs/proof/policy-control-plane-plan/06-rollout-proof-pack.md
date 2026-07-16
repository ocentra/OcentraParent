# WP06 Rollout Proof Pack

This file tracks the proof IDs required by `workpacks/06-rollout-proof-and-route-gate.md`.

| Proof ID | State | Evidence |
| --- | --- | --- |
| `policy-rollout.proof-pack-complete` | Open | workpack-specific closeout artifacts are still missing for WP02/WP03/WP04/WP05 |
| `policy-rollout.source-proof` | Present | `01-source-of-truth-matrix-proof.md`, `01-schema-negative-proof.md`, `01-version-skew-proof.md`, `01-duplicate-truth-negative-proof.md`, `01-ai-preview-not-write-proof.md`, `01-authz-negative-proof.md` |
| `policy-rollout.preview-proof` | Open | no WP02 artifact bundle in this checkout |
| `policy-rollout.schedule-proof` | Present | `07-schedule-timezone-proof.md`, `07-dst-boundary-proof.md`, `07-time-budget-reset-proof.md`, `07-conflict-precedence-proof.md`, `07-offline-timer-recovery-proof.md` |
| `policy-rollout.compiler-proof` | Open | no WP03 artifact bundle in this checkout |
| `policy-rollout.delivery-proof` | Open | no WP04 artifact bundle in this checkout |
| `policy-rollout.override-proof` | Open | no WP05 artifact bundle in this checkout |
| `policy-rollout.authz-negative-proof` | Present | `01-authz-negative-proof.md` |
| `policy-rollout.rollback-proof` | Present | `08-rollback-event-linkage-proof.md` plus `01-source-of-truth-matrix-proof.md` |
| `policy-rollout.route-sync` | Present | `06-route-sync-proof.md` |
| `policy-rollout.manual-required-gap-register` | Present | `06-manual-required-gap-register.md` |
| `policy-rollout.no-overclaim` | Present | `06-no-overclaim-proof.md` |

## Read with

- `00-scope-summary.md`
- `01-negative-case-proof.md`
- `02-no-claim-boundary.md`
- `16-validation-commands.log`

WP08 event-model closeout artifacts now also exist under `08-*.md`; WP06 does not assign them a dedicated standalone row, but they are part of the current core proof set.

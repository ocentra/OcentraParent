# Proof And Test Inventory

Status: route-synced.

This file tracks the canonical proof root and the already-audited validation pointers for `policy-control-plane-plan`. It is a proof-route inventory inside the plan folder because WP06 owns route truth and proof-manifest truth for this plan.

## Canonical proof root

```text
docs/proof/policy-control-plane-plan/
```

## Manifest and command log

- Manifest: `docs/proof/policy-control-plane-plan/PLAN_PROOF_MANIFEST.md`
- Focused command log: `docs/proof/policy-control-plane-plan/16-validation-commands.log`

## Current workpack proof inventory

| Workpack | Current proof state | Evidence |
| --- | --- | --- |
| WP01 source of truth | Present | `01-source-of-truth-matrix-proof.md`, `01-schema-negative-proof.md`, `01-version-skew-proof.md`, `01-duplicate-truth-negative-proof.md`, `01-ai-preview-not-write-proof.md`, `01-authz-negative-proof.md` |
| WP02 parent authoring/preview | Open | no `02-*.md` closeout bundle under the canonical root; `02-no-claim-boundary.md` is universal only |
| WP03 domain policy compilers | Open | no `03-*.md` closeout bundle under the canonical root; deleted stale `03-*.md` files do not count |
| WP04 delivery/ack/audit | Open | no `04-*.md` closeout bundle under the canonical root |
| WP05 ask-parent/overrides | Open | no `05-*.md` closeout bundle under the canonical root |
| WP06 rollout proof/route gate | Present | `00-scope-summary.md`, `06-rollout-proof-pack.md`, `06-route-sync-proof.md`, `06-no-overclaim-proof.md`, `06-manual-required-gap-register.md`, `PLAN_PROOF_MANIFEST.md`, `16-validation-commands.log` |
| WP07 schedule/conflict | Present | `07-schedule-timezone-proof.md`, `07-dst-boundary-proof.md`, `07-time-budget-reset-proof.md`, `07-conflict-precedence-proof.md`, `07-offline-timer-recovery-proof.md` |
| WP08 event model | Present | `08-event-family-registry-proof.md`, `08-event-idempotency-proof.md`, `08-event-replay-ordering-proof.md`, `08-rollback-event-linkage-proof.md`, `08-event-redaction-proof.md` |

## Universal guardrails

- `00-scope-summary.md`
- `01-negative-case-proof.md`
- `02-no-claim-boundary.md`
- `16-validation-commands.log`

## Failure conditions

- Do not mark DONE or PR_READY until open workpacks have their named closeout bundles under the canonical root.
- Do not treat universal guardrail files as substitutes for missing workpack closeout artifacts.
- Do not create a second proof root or point this plan back at an `output/` proof location.

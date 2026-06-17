# Screen AI Pipeline Plan State

<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-ai-pipeline-plan`
> Doc: `Screen AI Pipeline Plan State`
> Kind: current state and open gaps.
> Read when: Immediately after plan AGENTS.md; use for current state and no-claim boundaries.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.

<!-- /agent-capsule -->

Generated from the existing `screen-ai-pipeline-plan` docs. This is the default resume/status file; large historical docs are linked, not embedded.

## Scope

This folder is the required second-stage integration plan for the complete screen-capture plus AI-analysis plus policy/action path.

## Resume route

1. Read this file.
2. Read `NEXT_ACTIONS.md` when starting/resuming.
3. Read `WORKPACK_INDEX.md`.
4. Open only the assigned workpack.
5. Use `CHECKLIST_INDEX.md` for exact checklist sections.
6. Use `PROOF_INDEX.md` for proof artifacts.

## Current snapshot source

- No `current-*.md` snapshot exists.
- Use this file, `NEXT_ACTIONS.md`, `WORKPACK_INDEX.md`, and the assigned workpack as the current audited route until fresh proof is retained.

## What is already present / proved

- Real source surface exists across `crates/agent-service`, `crates/agent-core`, `packages/screen-domain`, `packages/ai-domain`, `packages/portal-domain`, and `apps/portal`.
- Real test surface exists across `packages/*/tests`, `apps/portal/tests`, and `scripts/test`.
- `implementation-checklist.md`, the workpacks, and `pipeline-proof-matrix.md` all define screen-to-AI pipeline scenarios and proof expectations under `output/screen-ai-pipeline-proof/`.

## Open gaps / missing product runtime

- No retained proof root currently exists at `output/screen-ai-pipeline-proof/`.
- `docs/proof/screen-ai-pipeline-plan/PLAN_PROOF_MANIFEST.md` is missing.
- The prior checked/open status in this folder was stale; the current audit recount shows every checklist and workpack box open.
- Scoped architecture validation is currently red on existing re-export surfaces:
  - `packages/screen-domain/src/screen-evidence.ts`
  - `packages/portal-domain/src/contracts.ts`
  - `packages/parent-domain/src/local-ai-runtime.ts`

## Checklist summary

- Full checklist: [implementation-checklist.md](implementation-checklist.md) (not default context).
- Checkbox rows detected: 134 total, 0 checked, 134 unchecked.
- Checklist index: [CHECKLIST_INDEX.md](CHECKLIST_INDEX.md).

## Workpack summary

- Workpacks indexed: 10.
- Workpacks with open checkboxes: 10.
- Workpacks with all detected boxes checked: 0.
- Workpacks with no checkbox status: 0.

### Active/open workpacks

- [01 - Prerequisite Merge And Branch Gate](workpacks/01-prerequisite-merge-and-branch-gate.md) - 0/5 checked, 5 open.
- [02 - Real Trigger To Capture Gate](workpacks/02-real-trigger-to-capture-gate.md) - 0/9 checked, 9 open.
- [03 - Capture To AI Analysis Gate](workpacks/03-capture-to-ai-analysis-gate.md) - 0/6 checked, 6 open.
- [04 - AI Result To Policy Gate](workpacks/04-ai-result-to-policy-gate.md) - 0/5 checked, 5 open.
- [05 - Policy Action Dry-Run Gate](workpacks/05-policy-action-dry-run-gate.md) - 0/7 checked, 7 open.
- [06 - Journal Read Model And Portal Gate](workpacks/06-journal-read-model-and-portal-gate.md) - 0/6 checked, 6 open.
- [07 - Deletion Retention And Custody Gate](workpacks/07-deletion-retention-and-custody-gate.md) - 0/7 checked, 7 open.
- [08 - Live Operator Proof Gate](workpacks/08-live-operator-proof-gate.md) - 0/11 checked, 11 open.
- [09 - Performance Cadence And Backpressure Gate](workpacks/09-performance-cadence-and-backpressure-gate.md) - 0/6 checked, 6 open.
- [10 - Final Rollout And PR Gate](workpacks/10-final-rollout-and-pr-gate.md) - 0/8 checked, 8 open.

## Default no-read list

- `README_FULL_ORIGINAL.md` unless you need historical full README context.
- Full `implementation-checklist.md` unless `CHECKLIST_INDEX.md` names exact section/row.
- All workpacks; use `WORKPACK_INDEX.md`.
- Source inventories and pasted-content audits unless source ownership is unclear.
- Historical checkpoint/proof docs unless `PROOF_INDEX.md` or the assigned workpack names them.

## Health / consistency

- See `PLAN_HEALTH.md` before claiming the whole plan is complete or stale.
- Treat any older `checked`, retained-proof, or completion wording elsewhere in this folder as stale until it matches the current proof artifacts and checklist rows.

## HID Execution Guard (added 2026-06-12)

- Scope and completion source:
  - follow [PLAN_HID_MATRIX.md](../../PLAN_HID_MATRIX.md) execution slice, then this plan's assigned `WORKPACK_INDEX.md` and `NEXT_ACTIONS.md`.
  - do not mark this plan complete from checklist deltas alone.
- Before any checked update, attach:
  - a real test run log (or explicit known blocker) from the assigned implementation boundary,
  - retained scenario proof under `output/screen-ai-pipeline-proof/`,
  - a supporting manifest under `docs/proof/screen-ai-pipeline-plan/` when the workpack claims slice closure.
- Current audit state: the plan proof root is absent and `docs/proof/screen-ai-pipeline-plan/PLAN_PROOF_MANIFEST.md` is missing, so no checklist row is currently eligible for a fresh checked claim.
- Failure rule: no PR-ready claim until replay/idempotency, authZ/replay, and rollback/teardown proofs are present for the assigned slice and retained locally.

## HID execution blueprint

Continue execution from: [PLAN_EXECUTION_BLUEPRINT.md](PLAN_EXECUTION_BLUEPRINT.md).
Update this plan only via the blueprint and matching workpack checklist.

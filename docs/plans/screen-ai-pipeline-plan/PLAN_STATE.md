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

## Current ownership interpretation

```text
screen-ai-pipeline-plan:
  Integration proof owner for trigger -> capture -> AI context/result -> policy handoff -> dry-run/action boundary -> journal/read-model/portal -> custody/delete -> live operator -> performance/backpressure -> rollout.

screen-plan:
  Raw capture mechanics, protected surfaces, disclosure, local screen settings, screenshot custody, and capture-source ownership.

screen-domain:
  Screen capture, evidence, OCR, VLM, disclosure, settings, screen-intelligence router, and screen handoff contracts. This is a real contract package.

ai-plan/crates/schema:
  Shared AI context/result/provider/degradation contracts. `ai-domain` is package identity and focused tests; canonical shared AI contracts live in `crates/schema`.

policy-control-plane-plan:
  Policy authority, parent-rule precedence, policy decision semantics, and no-direct-AI-authority boundary.

v0-8-enforcement-control-plan:
  Adapter execution, rollback, supported runtime proof, and enforcement authority.

data-custody-storage-plan:
  Retention/export/delete/privacy/custody for raw images, queue artifacts, screenshots, AI outputs, and proof artifacts.

portal-ux-household-surfaces-plan:
  Rendered parent-visible projections, screenshots, route proof, and no-fake-data UI boundaries.

browser/app-game/network/tracking plans:
  Trigger/source truth for browser, native app/game, network, tracking, and related source evidence.

agent-protocol/agent-service/agent-core:
  Selected protocol, service, queue, journal, read-model, and WebSocket seams.
```

## Resume route

1. Read this file.
2. Read `NEXT_ACTIONS.md` when starting/resuming.
3. Read `WORKPACK_INDEX.md`.
4. Use `WORKPACK_FAMILIES.md` only when owner/proof family is unclear.
5. Open only the assigned workpack.
6. Use `CHECKLIST_INDEX.md` for exact checklist sections.
7. Use `PROOF_INDEX.md` for proof artifacts.

## Current snapshot source

- No `current-*.md` snapshot exists.
- Use this file, `NEXT_ACTIONS.md`, `WORKPACK_INDEX.md`, `WORKPACK_FAMILIES.md` when needed, and the assigned workpack as the current audited route until fresh proof is retained.

## What is already present / proved

- Real source surface exists across `crates/agent-service`, `crates/agent-core`, `packages/screen-domain`, `packages/ai-domain`, `packages/portal-domain`, and `apps/portal`.
- Real test surface exists across `packages/*/tests`, `apps/portal/tests`, and `scripts/test`.
- `implementation-checklist.md`, the workpacks, and `pipeline-proof-matrix.md` all define screen-to-AI pipeline scenarios and proof expectations under `output/screen-ai-pipeline-proof/`.
- Current source role interpretation: `screen-domain` is a real screen contract package; `ai-domain` is package identity/focused tests with canonical shared AI contracts in `crates/schema`.

## Open gaps / missing product runtime

- No retained proof root currently exists at `output/screen-ai-pipeline-proof/`.
- `docs/proof/screen-ai-pipeline-plan/PLAN_PROOF_MANIFEST.md` is missing.
- The prior checked/open status in this folder was stale; the current audit recount shows every checklist and workpack box open.
- Scoped architecture validation is currently red on existing re-export surfaces:
  - `packages/screen-domain/src/screen-evidence.ts`
  - `packages/portal-domain/src/contracts.ts`
  - `packages/parent-domain/src/local-ai-runtime.ts`

## Production reachability audit (2026-08-16)

The following is a source-reachability audit only. It does not check any
workpack row, retained proof, live operator result, or rollout gate.

| Workpack | Shipped production path and current boundary |
| --- | --- |
| WP01 | Prerequisite/branch gate only; no product runtime slice. |
| WP02 | `agent-service` cadence and foreground loops call the configured screen-capture adapter, then persist encrypted queue/journal/store state. Capture is Windows-owned; unsupported foreground/capture paths remain degraded. Trigger ownership and live capture proof remain open. |
| WP03 | `agent-service` claims the encrypted queue and invokes the configured external adapter process; JSON output is parsed into a screen result. Provider/model contract ownership remains in AI/schema, and adapter/model health is not live proof. A failed event-runtime handoff now releases the queue claim for retry. |
| WP04 | Parsed results can reach the screen event bridge and policy references, but policy authority remains owned by `policy-control-plane-plan`; invalid or unavailable generations remain policy-ineligible. No enforcement authority is present here. |
| WP05 | The screen event runtime emits the policy/action boundary and does not execute enforcement. Dry-run/action proof and the enforcement adapter remain separate gates. |
| WP06 | Journal/store/read-model and portal projection callers exist. Portal/read-model presence is not pipeline proof; retained proof and manifest are absent. |
| WP07 | Queue completion/removal and retention-sweeper callers exist with local custody paths. Deletion/retention policy proof and external custody authority remain open. |
| WP08 | Live-operator proof gate only; no additional production caller. Required live URLs/apps and artifacts are absent. |
| WP09 | Cadence, foreground polling, bounded queue scan, lease heartbeat, and adapter timeout paths exist. Stress/backpressure proof and external adapter/model health remain open. |
| WP10 | Rollout/aggregate proof gate only; no additional production runtime caller. |

The analysis runtime now fails closed at the result-to-event handoff:
`crates/agent-service/src/screen_ai_analysis_runtime.rs` requires a started
`ScreenAiServiceEventRuntime`, a persisted read-model row, and successful
row-ready publication before completing the encrypted queue entry. Missing or
failed handoff leaves the durable queue item retryable. This is code-drafted;
tests, retained proof, and live validation remain deferred.

## Current coupling risks

```text
- Missing retained proof root blocks closure.
- Missing PLAN_PROOF_MANIFEST blocks slice closure.
- Source-only proof is not product proof.
- Mock-only proof is not product proof.
- Local capture proof is not AI analysis proof.
- AI result proof is not policy authority.
- Policy decision proof is not enforcement execution.
- Dry-run proof is not adapter runtime proof.
- Live-operator artifact-gate proof is not a live capture rerun.
- Custody proof without deletion/retention artifacts is incomplete.
- Portal screenshot proof is not pipeline/runtime proof.
```

## Current proof interpretation

```text
output/screen-ai-pipeline-proof/ is the scenario-based retained proof root.
docs/proof/screen-ai-pipeline-plan/PLAN_PROOF_MANIFEST.md is required before any slice-level closeout claim.
Workpack proof may use scenario-local proof-summary.json or richer numbered bundles, but the selected workpack must state which artifact shape is accepted before any row is checked.
All rows remain open until retained artifacts, command logs, negative cases, no-claim boundaries, and proof manifest entries exist.
```

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
- `WORKPACK_FAMILIES.md` unless selected workpack owner/proof family is unclear.
- Source inventories and pasted-content audits unless source ownership is unclear.
- Historical checkpoint/proof docs unless `PROOF_INDEX.md` or the assigned workpack names them.

## Health / consistency

- See `PLAN_HEALTH.md` before claiming the whole plan is complete or stale.
- Treat any older `checked`, retained-proof, or completion wording elsewhere in this folder as stale until it matches the current proof artifacts and checklist rows.

## HID Execution Guard

- Scope and completion source:
  - follow [PLAN_HID_MATRIX.md](../../PLAN_HID_MATRIX.md) execution slice, then this plan's assigned `WORKPACK_INDEX.md` and `NEXT_ACTIONS.md`.
  - use `WORKPACK_FAMILIES.md` only when owner/proof family is unclear.
  - do not mark this plan complete from checklist deltas alone.
- Before any checked update, attach:
  - a real test run log or explicit known blocker from the assigned implementation boundary,
  - retained scenario proof under `output/screen-ai-pipeline-proof/`,
  - a supporting manifest under `docs/proof/screen-ai-pipeline-plan/` when the workpack claims slice closure.
- Current audit state: the plan proof root is absent and `docs/proof/screen-ai-pipeline-plan/PLAN_PROOF_MANIFEST.md` is missing, so no checklist row is currently eligible for a fresh checked claim.
- Failure rule: no PR-ready claim until replay/idempotency, authZ/replay, rollback/teardown, redaction/custody, no-direct-policy-authority, and retained proof artifacts are present for the assigned slice.

## HID execution blueprint

Continue execution from: [PLAN_EXECUTION_BLUEPRINT.md](PLAN_EXECUTION_BLUEPRINT.md).
Update this plan only via the blueprint and matching workpack checklist.

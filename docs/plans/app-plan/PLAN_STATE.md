<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-plan`
> Doc: `PLAN_STATE.md`
> Kind: plan state and current gap summary.
> Read when: After this plan is selected and before opening workpacks.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If state changes, update NEXT_ACTIONS.md, WORKPACK_INDEX.md, CHECKLIST_INDEX.md, and feature/checklist rows as needed.

<!-- /agent-capsule -->

# Native Apps Plan State

## Current Product Scope

This plan owns native app identity, installed inventory, process/runtime, foreground app evidence, app-only policy targets, app catalog/settings, and legacy app-plan reconciliation.

## Current ownership interpretation

```text
crates/schema:
  Canonical shared native-app/app-game contracts when app shapes cross package, crate, app, or plan boundaries.

app-core:
  Child-local Rust native-app observation, evidence-event, AI-request, policy-request, and source-readiness boundary.

app-plan:
  App-only route, app-only product meaning, app-specific reconciliation, and proof expectation owner.

app-game-plan:
  Shared native app/game evidence spine, combined app/game runtime/read-model chains, native game slices, and most generated handoff chains.

agent-protocol and agent-service:
  Wire/service/read-model boundaries when selected. They are not default owners for every native-app contract.

portal-domain and apps/portal:
  Parent-visible app status projections. They do not observe OS state, classify apps, run timers, or enforce.

Policy, enforcement, notification, child-runtime, setup, payment, data-custody, LAN, and remote plans:
  Adjacent sibling owners or handoff consumers. They must not re-own native-app source truth.
```

## Current coupling risks

```text
- No `packages/app-domain` package is assumed by this plan. Do not invent it as an owner unless a selected workpack creates and proves it.
- Shared native app/game implementation usually belongs to app-game-plan; use this plan only for app-only narrowing or explicit app-plan reconciliation.
- Generated handoff workpacks are not implementation scope by themselves. A selected workpack must identify owner path and proof family before source edits.
- Portal rows, policy preview rows, notification rows, and child UX rows do not prove native-app source/runtime readiness unless service/protocol/runtime proof exists.
- App-game-plan proof cannot close this plan unless the selected workpack names the app-only handoff and no-claim boundary.
```

## Current proof interpretation

```text
Route normalization is not runtime support.
Package preview or scaffold proof is not product readiness.
Staged journal/read-model proof is not live source subscription proof unless the workpack proves that tier.
Portal row proof is not native-app source capture or service proof.
Policy dry-run proof is not enforcement proof.
Platform preflight proof is not platform parity.
Manual-required adapter proof is not adapter execution.
```

## Current Route Status

- Status: plan routing restored/normalized after local folder cleanup.
- Default action: choose one workpack from [WORKPACK_INDEX.md](WORKPACK_INDEX.md), then choose expected tests/proof from [TEST_PROOF_EXPECTATIONS.md](TEST_PROOF_EXPECTATIONS.md).
- Current limitation: this plan defines ownership, expected proof, and handoff boundaries. It does not claim implementation is complete.

## Latest selected-slice evidence (2026-08-09)

WP01 contract-boundary code and tests are locally validated: schema-domain
build/type-check and 11/11 focused Vitest tests pass, while app-core focused
runtime-decision tests pass 8/8. The local proof manifest is
`docs/proof/app-plan/slice-01-app-runtime-decision.md`. The plan remains open
for inventory/runtime capture, service/portal integration, platform proof, CI,
review, and merge evidence.

## Open Product Gaps

- Product acceptance rows need to be reconciled against the named feature and expectation docs.
- Source ownership must be assigned before implementation work starts.
- Proof artifacts must be created by implementation work; this plan only defines expected proof.
- Adjacent implementation plans must be updated only when their workpack is selected.

## No-Read Boundary

Do not read adjacent plans or source trees until a workpack names the exact handoff.

Use `WORKPACK_FAMILIES.md` only when the selected workpack owner/proof family is unclear; do not use it as permission to scan a whole family.

## HID Execution Guard (added 2026-06-12)

- Scope and completion source:
  - follow [PLAN_HID_MATRIX.md](../../PLAN_HID_MATRIX.md) execution slice, then this plan's assigned WORKPACK_INDEX.md and NEXT_ACTIONS.md.
  - do not mark this plan complete from checklist deltas alone.
- Before any checked update, attach:
  - a real test run log (or explicit known blocker) from the assigned implementation boundary,
  - a proof manifest under docs/proof/app-plan/.
- Required proof manifest names:
  - docs/proof/app-plan/slice-01-*.md
  - docs/proof/app-plan/slice-02-*.md
  - docs/proof/app-plan/slice-03-*.md
  - each proof file must include commands, pass/fail,
    negative-cases, and manual-required notes.
- Failure rule: no PR-ready claim until replay/idempotency, authZ/replay, and rollback/teardown proofs are present for the assigned slice.

## HID execution blueprint

Continue execution from: [PLAN_EXECUTION_BLUEPRINT.md](PLAN_EXECUTION_BLUEPRINT.md).
Update this plan only via the blueprint and matching workpack checklist.

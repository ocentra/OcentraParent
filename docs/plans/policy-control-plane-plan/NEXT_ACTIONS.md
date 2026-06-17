# Next Actions

## Scope and ownership

- Plan owner: `policy-control-plane-plan/AGENTS.md`.
- Ownership boundary: policy contract source truth, schedule/time-budget/conflict semantics, parent authoring and ask-parent flows, domain compiler handoff, delivery/ack/audit, event model, and route-gated proof.
- Scope boundary: do not move into domain adapter implementation until the matching workpack and proof paths are selected.

## Current focus

- `workpacks/01-policy-source-of-truth.md`, `workpacks/07-schedule-time-budget-conflict-model.md`, and `workpacks/08-policy-event-model.md` now have closeout artifacts; the next locally-owned closeout target is WP03 because the compiler core is present but the `03-*.md` proof bundle is stale/deleted in this checkout.

## Ordered workpacks

- [x] Close `workpacks/01-policy-source-of-truth.md` with typed source-of-truth and versioning proof.
- [x] Close `workpacks/07-schedule-time-budget-conflict-model.md` with timezone/DST and conflict precedence proof.
- [ ] Close `workpacks/02-parent-authoring-preview.md` with preview, conflict, and mobile/accessibility proof.
- [ ] Close `workpacks/03-domain-policy-compilers.md` with deterministic compiler contracts and handoffs.
- [x] Close `workpacks/08-policy-event-model.md` with event family, idempotency, and replay proof.
- [ ] Close `workpacks/04-delivery-ack-audit.md` with per-device/domain delivery and audit proof.
- [ ] Close `workpacks/05-ask-parent-overrides.md` with approval, expiry, replay, and assistant-preview proof.
- [x] Close `workpacks/06-rollout-proof-and-route-gate.md` with route sync and no-overclaim proof.

## Decision routes and failure conditions

- If source truth remains ambiguous, hold implementation lanes and keep the contract open.
- If schedule precedence or conflict resolution is unresolved, block parent-visible rollout.
- If delivery ack or rollback evidence is missing, do not claim active policy.
- If event replay or audit linkage is missing, do not treat the control plane as execution-grade.
- If parent authoring, parent confirmation, or child-agent validation still depends on other plans, keep WP02 and WP05 open instead of promoting contract-only passes.

## Proof and proof path

- Canonical proof root: `docs/proof/policy-control-plane-plan/`.
- Required proof links live in `PROOF_AND_TEST_INVENTORY.md` and the workpack proof artifact lists.
- The root now contains universal guardrail files plus checked closeout bundles for WP01, WP07, and WP08, and the WP06 route bundle.
- Current state stays open until WP02/WP03/WP04/WP05 each have matching closeout artifacts and scoped validation.

## Blocker classification

- Real dependency blockers: unfinished portal authoring/approval surfaces, unfinished parent-assistant confirmation/chat integration, and remaining device-trust/data-custody/enforcement handoffs.
- External platform constraints: real iOS/macOS proof remains external to this Windows host when a selected workpack requires it.
- Avoidable local execution gaps: missing WP02/WP03/WP04/WP05 proof bundles, stale/deleted WP03 proof files in this checkout, overbroad portal workspace test script, and the broader architecture gate failure in `packages/agent-protocol-domain`.

## State

- This plan is execution-grade architecture with real contract coverage; WP01, WP06, WP07, and WP08 are now checked, but overall implementation/proof closure remains open until WP02/WP03/WP04/WP05 close honestly.

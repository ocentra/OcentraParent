# Next Actions

## Scope and ownership

- Plan owner: `policy-control-plane-plan/AGENTS.md`.
- Ownership boundary: policy contract source truth, schedule/time-budget/conflict semantics, parent authoring and ask-parent flows, domain compiler handoff, delivery/ack/audit, event model, and route-gated proof.
- Scope boundary: do not move into domain adapter implementation until the matching workpack and proof paths are selected.

## Ordered workpacks

- [ ] Close `workpacks/01-policy-source-of-truth.md` with typed source-of-truth and versioning proof.
- [ ] Close `workpacks/07-schedule-time-budget-conflict-model.md` with timezone/DST and conflict precedence proof.
- [ ] Close `workpacks/02-parent-authoring-preview.md` with preview, conflict, and mobile/accessibility proof.
- [ ] Close `workpacks/03-domain-policy-compilers.md` with deterministic compiler contracts and handoffs.
- [x] Close `workpacks/08-policy-event-model.md` with event family, idempotency, and replay proof.
- [ ] Close `workpacks/04-delivery-ack-audit.md` with per-device/domain delivery and audit proof.
- [ ] Close `workpacks/05-ask-parent-overrides.md` with approval, expiry, replay, and assistant-preview proof.
- [ ] Close `workpacks/06-rollout-proof-and-route-gate.md` with route sync and no-overclaim proof.

## Decision routes and failure conditions

- If source truth remains ambiguous, hold implementation lanes and keep the contract open.
- If schedule precedence or conflict resolution is unresolved, block parent-visible rollout.
- If delivery ack or rollback evidence is missing, do not claim active policy.
- If event replay or audit linkage is missing, do not treat the control plane as execution-grade.

## Proof and proof path

- Required proof links live in `PROOF_AND_TEST_INVENTORY.md` and the workpack proof artifact lists.
- Current state stays open until each workpack has matching proof artifacts and route sync.

## State

- This plan is execution-grade architecture; implementation is still open until the proof set closes.

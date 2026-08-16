# Policy Control Plane Next Actions

## Scope and ownership

- Plan owner: `policy-control-plane-plan/AGENTS.md`.
- Ownership boundary: policy contract source truth, schedule/time-budget/conflict semantics, parent authoring and ask-parent flows, domain compiler handoff, delivery/ack/audit, event model, and route-gated proof.
- Scope boundary: do not move into domain adapter implementation until the matching workpack and proof paths are selected.
- Use `WORKPACK_FAMILIES.md` only when the selected workpack owner/proof family is unclear.

## Current focus

- The WP06 route/proof truth repair points the plan docs at one canonical proof root and records that state in `docs/proof/policy-control-plane-plan/PLAN_PROOF_MANIFEST.md`.
- `workpacks/03-domain-policy-compilers.md` has an owner-surface closeout bundle built from the current `@ocentra-parent/policy-domain` and `ocentra-policy-control-core` validation slice.
- `workpacks/04-delivery-ack-audit.md` has an owner-surface closeout bundle built from the current delivery/source/event validation slice in `packages/policy-domain` and `crates/policy-control-core`.
- `workpacks/01-policy-source-of-truth.md`, `workpacks/03-domain-policy-compilers.md`, `workpacks/04-delivery-ack-audit.md`, `workpacks/06-rollout-proof-and-route-gate.md`, `workpacks/07-schedule-time-budget-conflict-model.md`, and `workpacks/08-policy-event-model.md` now have closeout artifacts; the plan-local targets that remain open are WP02 and WP05.
- WP02 still depends on rendered parent authoring/preview/conflict/approval surfaces outside this owner slice.
- WP02 code pass checkpoint: Rust-owned draft validation/staging, trusted preview-row authority projection, typed confirmed-request relay, and bounded handle mark-before-dispatch/restore-on-failure/commit-on-success now back the portal draft/confirm/cancel surface; missing authority context fails closed for manual review, validation and tests remain deferred, and no runtime completion claim is made.
- WP05 still depends on parent confirmation, assistant/portal seams, child-agent validation, notification handoff, expiry/replay proof, and audit integration outside this owner slice.

## Ordered workpacks

- [x] Close `workpacks/01-policy-source-of-truth.md` with typed source-of-truth and versioning proof.
- [x] Close `workpacks/07-schedule-time-budget-conflict-model.md` with timezone/DST and conflict precedence proof.
- [ ] Close `workpacks/02-parent-authoring-preview.md` with preview, conflict, and mobile/accessibility proof.
- [x] Close `workpacks/03-domain-policy-compilers.md` with deterministic compiler contracts and handoffs.
- [x] Close `workpacks/04-delivery-ack-audit.md` with per-device/domain delivery and audit proof.
- [x] Close `workpacks/08-policy-event-model.md` with event family, idempotency, and replay proof.
- [ ] Close `workpacks/05-ask-parent-overrides.md` with approval, expiry, replay, and assistant-preview proof.
- [x] Close `workpacks/06-rollout-proof-and-route-gate.md` with route sync and no-overclaim proof.

## Decision routes and failure conditions

- If source truth remains ambiguous, hold implementation lanes and keep the contract open.
- If schedule precedence or conflict resolution is unresolved, block parent-visible rollout.
- If delivery ack or rollback evidence is missing, do not claim active policy.
- If event replay or audit linkage is missing, do not treat the control plane as execution-grade.
- If parent authoring, parent confirmation, or child-agent validation still depends on other plans, keep WP02 and WP05 open instead of promoting contract-only passes.
- Do not use compiler/event/source-truth proof to close WP02 or WP05.

## Proof and proof path

- Canonical proof root: `docs/proof/policy-control-plane-plan/`.
- Required proof links live in `PROOF_INDEX.md`, `PROOF_AND_TEST_INVENTORY.md`, and the workpack proof artifact lists.
- The root contains universal guardrail files, `PLAN_PROOF_MANIFEST.md`, checked closeout bundles for WP01, WP03, WP04, WP06, WP07, and WP08, and open gaps for WP02/WP05.
- Current state stays open until WP02/WP05 each have matching closeout artifacts and scoped validation or explicit dependency blockers.

## Blocker classification

- Real dependency blockers: unfinished portal authoring/approval surfaces, unfinished parent-assistant confirmation/chat integration, remaining child-agent validation/notification handoffs, and remaining device-trust/data-custody/enforcement handoffs.
- External platform constraints: real iOS/macOS proof remains external to this Windows host when a selected workpack requires it.
- Avoidable local execution gaps: missing WP02/WP05 proof bundles, overbroad portal workspace test script, and the broader architecture gate failure in `packages/agent-protocol-domain`.

## State

- This plan is execution-grade architecture with real contract coverage; WP01, WP03, WP04, WP06, WP07, and WP08 are checked, but overall implementation/proof closure remains open until WP02/WP05 close honestly.

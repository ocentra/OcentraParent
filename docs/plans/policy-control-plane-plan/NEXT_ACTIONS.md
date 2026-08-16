# Policy Control Plane Next Actions

## Scope and ownership

- Plan owner: `policy-control-plane-plan/AGENTS.md`.
- Ownership boundary: policy contract source truth, schedule/time-budget/conflict semantics, parent authoring and ask-parent flows, domain compiler handoff, delivery/ack/audit, event model, and route-gated proof.
- Scope boundary: do not move into domain adapter implementation until the matching workpack and proof paths are selected.
- Use `WORKPACK_FAMILIES.md` only when the selected workpack owner/proof family is unclear.

## Current focus

- The WP06 route/proof truth repair points the plan docs at one canonical proof root and records that state in `docs/proof/policy-control-plane-plan/PLAN_PROOF_MANIFEST.md`.
- `workpacks/03-domain-policy-compilers.md` has deterministic Rust contract code and retained contract proof. The formerly cited `@ocentra-parent/policy-domain` package is absent, and no shipped caller loads an authoritative source and invokes a domain compiler.
- `workpacks/04-delivery-ack-audit.md` has contract/receipt evidence in the Rust owners, but no trusted execution authority or active delivery composition.
- WP01 and WP03 are production-open despite their contract artifacts. WP02/WP04/WP05 are also open; WP06/WP07/WP08 retain routed proof bundles without upgrading those production gaps.
- WP01 dependency gate is explicit: first consume Cloudflare WP06's durable identity/household owner (transitive Account Identity WP08) plus Device Trust WP01/WP03 trusted-device and step-up context; then add the durable source repository, trusted resolver, startup recovery, and shipped registration/active-version query caller. Caller-supplied authority and fixture/manual/debug custody remain invalid substitutes.
- WP02 still depends on rendered parent authoring/preview/conflict/approval surfaces outside this owner slice.
- WP02 code pass checkpoint: Rust-owned draft validation/staging, trusted preview-row authority projection, typed confirmed-request relay, and bounded handle mark-before-dispatch/restore-on-failure/commit-on-success now back the portal draft/confirm/cancel surface; missing authority context fails closed for manual review, validation and tests remain deferred, and no runtime completion claim is made.
- WP05 production-code checkpoint: the Rust resolution command/service and delivery-binding validation exist, and a drafted typed decision staging/relay slice now maps `action_dispatch.rs`, `policy_preview.rs`, and `policy_preview/resolution.rs`. It projects `Modify` from trusted preview context only, binds actor identity to the local active-controller record, and fails closed for missing or ambiguous account/identity context. The rendered portal callback, account/identity provider composition, notification-provider dispatch, WP11 durability, runtime integration, validation/tests, and proof remain open; no completion claim is made.
- WP05 no-claim correction: `crates/agent-service/src/websocket/policy_request_resolution/apply.rs` leaves `notification_handoff_claim_state` `Unclaimed` until a notification-owned provider dispatch exists; policy resolution alone is not notification delivery.

## Ordered workpacks

- [ ] Complete `workpacks/01-policy-source-of-truth.md` after Cloudflare WP06 (transitive Account Identity WP08) and Device Trust WP01/WP03 authority prerequisites are available, then add trusted identity-backed durable source persistence, startup recovery, and a shipped registration/active-version query caller; existing type/proof coverage is contract-only.
- [x] Close `workpacks/07-schedule-time-budget-conflict-model.md` with timezone/DST and conflict precedence proof.
- [ ] Close `workpacks/02-parent-authoring-preview.md` with preview, conflict, and mobile/accessibility proof.
- [ ] Complete `workpacks/03-domain-policy-compilers.md` only after WP01's authoritative source registration/active-version query boundary exists, with a shipped active-source-to-domain compiler/persist-or-deliver composition; deterministic library tests are insufficient.
- [ ] Complete `workpacks/04-delivery-ack-audit.md` with trusted per-device/domain delivery authority and an inspectable execution trace; caller-built receipts are evidence only.
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
- The root contains universal guardrail files, `PLAN_PROOF_MANIFEST.md`, contract bundles for WP01/WP03, routed bundles for WP06/WP07/WP08, and open/runtime-blocked gaps for WP01-WP05.
- Current state stays open until the production caller/authority gaps in WP01-WP05 close honestly; proof presence alone does not close them.

## Blocker classification

- Real dependency blockers: unfinished portal authoring/approval surfaces, an unconsumed WP05 portal parent-resolution callback, missing account/identity actor projection and notification-provider dispatch, remaining child-agent validation, and remaining device-trust/data-custody/enforcement handoffs.
- External platform constraints: real iOS/macOS proof remains external to this Windows host when a selected workpack requires it.
- Avoidable local execution gaps: missing WP02/WP05 proof bundles, overbroad portal workspace test script, and the broader architecture gate failure in `packages/agent-protocol-domain`.

## State

- This plan has real contract architecture but is not execution-grade: WP01/WP03 lack production source/compiler composition, WP02/WP04/WP05 remain open, and WP06/WP07/WP08 route evidence without curing those gaps.

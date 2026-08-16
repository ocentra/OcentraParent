<!-- agent-capsule -->

> Agent Capsule
> Plan: `policy-control-plane-plan`
> Doc: `Policy Control Plane Workpack Families`
> Kind: owner-path classifier for selected workpacks.
> Read when: only after `WORKPACK_INDEX.md` selects or names a workpack and owner/proof family is unclear.
> Stop rule: classify the selected workpack only; do not use this file as permission to scan every workpack.
> Proves: routing and owner-path classification only.
> Does not prove: policy runtime readiness, UI readiness, domain effect readiness, enforcement readiness, or PR readiness.
> Proof rule: if this file changes route/status claims, update `AGENTS.md`, `PLAN_STATE.md`, and any affected selected workpack route.

<!-- /agent-capsule -->

# Policy Control Plane Workpack Families

Use this file to classify a selected workpack before opening source. This plan owns policy control-plane truth and proof boundaries. It consumes portal, account, device-trust, data-custody, eventing, domain, AI, notification, and enforcement handoffs; it does not own those sibling runtime surfaces.

## Source truth and versioning family

```text
Workpacks:
WP01 Policy Source Of Truth

Owners:
schema-domain for canonical shared policy shapes when cross-boundary
policy-control-core for Rust source/authority helper proof when selected
policy-domain only as package/proof-consumer surface unless public exports exist
account-identity-family-plan for actor/role/household authority

Rule:
Policy source truth must be typed, versioned, parent-authorized, and auditable. UI state, compiler output, domain cache, or assistant draft is not source truth.
```

## Parent authoring and preview family

```text
Workpacks:
WP02 Parent Authoring Preview

Owners:
policy-control-plane-plan for authoring contract, preview states, confirmation contract, unsupported/manual-required state, and no-claim boundary
portal-ux-household-surfaces-plan for rendered UX implementation
AI plan only for draft suggestions through typed preview handoff

Rule:
Preview explains policy impact before save. Preview does not enforce, and assistant drafts remain preview-only until parent confirmation.
```

## Domain compiler and handoff family

```text
Workpacks:
WP03 Domain Policy Compilers

Owners:
policy-control-plane-plan for deterministic compiler contracts, source versioning, unsupported/manual-required outputs, rollback refs, and handoff shape
domain plans for app/game, browser, network, tracking, screen, AI, notification, and enforcement effects

Rule:
Compilers produce versioned domain artifacts; they do not mutate runtime, claim enforcement, or become source truth.
```

## Delivery, acknowledgement, and audit family

```text
Workpacks:
WP04 Delivery Ack Audit

Owners:
policy-control-plane-plan for delivery contract, per-device/domain status, ack requirement, retry/degraded states, rollback refs, and audit proof
eventing-plan for reusable event bus/idempotency/replay mechanics
enforcement/domain plans for runtime apply behavior only after handoff

Rule:
Policy is not globally active from one ack. Offline, partial, rejected, superseded, and manual-required states must remain visible and audited.
```

## Ask-parent and override family

```text
Workpacks:
WP05 Ask Parent Overrides

Owners:
policy-control-plane-plan for request/approval/bonus/override state machines and audit semantics
account-identity-family-plan for parent/observer/revoked role authority
device-trust-bootstrap-plan for high-risk parent presence/step-up when selected
AI plan for draft-only suggestions
notification plan for notification delivery handoff

Rule:
Child and AI paths cannot approve. Parent confirmation, expiry, replay defense, scope limits, and audit are required before an override becomes policy.
```

## Rollout proof and route gate family

```text
Workpacks:
WP06 Rollout Proof And Route Gate

Owners:
policy-control-plane-plan proof docs under `docs/proof/policy-control-plane-plan/`
PLAN_STATE, WORKPACK_INDEX, NEXT_ACTIONS, PROOF_INDEX, TEST_PROOF_EXPECTATIONS, PLAN_HEALTH, and selected workpacks when state changes

Rule:
Rollout proof may aggregate only accepted proof roots or exact carried blockers. Contract passes, compiler proof, route docs, or proof manifest presence cannot become full policy readiness.
```

## Schedule, time budget, and conflict family

```text
Workpacks:
WP07 Schedule Time Budget Conflict Model

Owners:
policy-control-plane-plan for timezone/DST, schedule, time budget, precedence, conflict, supersede, and manual-required semantics
portal and domain plans only through typed preview/compiler handoff

Rule:
Schedule and conflict proof must include DST/timezone boundaries, overlapping rules, stale devices, manual-required states, and deterministic precedence.
```

## Event model and replay family

```text
Workpacks:
WP08 Policy Event Model

Owners:
policy-control-plane-plan for policy event families, event references, audit linkage, and no-claim boundaries
eventing-plan for reusable idempotency, replay, journal, request/response, and delivery mechanics

Rule:
Event model proof is not delivery proof by itself. It must show idempotency/replay/audit linkage and keep eventing mechanics in the eventing plan.
```

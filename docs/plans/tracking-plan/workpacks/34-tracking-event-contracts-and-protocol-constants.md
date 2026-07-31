# WP34 Tracking Event Contracts And Protocol Constants

<!-- agent-capsule -->

> Agent Capsule
> Plan: `tracking-plan`
> Doc: `WP34 Tracking Event Contracts And Protocol Constants`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: event contract/source-boundary proof only.
> Does not prove: runtime dispatch, provider delivery, physical platform behavior, production readiness, or product-ready tracking.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Purpose

Define tracking, location, geofence, expected-place, nearby-place, live-mode,
notification, and escalation event contracts before runtime code publishes or
subscribes to them.

## Source Inputs

- `docs/plans/tracking-plan/README.md`
- `docs/plans/tracking-plan/source-index.md`
- `docs/plans/eventing-plan/03-event-taxonomy-and-parent-integration.md`
- `docs/plans/eventing-plan/05-implementation-workpacks.md`
- `crates/ocentra-eventing/README.md`

## Target State

Parent tracking event contracts exist in protocol/domain-owned constants and
validated schemas. Runtime code does not invent local event strings, and
tracking consumes the reusable eventing crate instead of creating a private bus.

Rust owns tracking runtime decisions. TypeScript mirrors portal/protocol-facing
contracts and renders read models, but it does not own tracking business
decisions. Tracking state helpers and local durable state should live in
`crates/agent-core`; WebSocket modules should remain transport/event-response
construction only.

Tracking event contracts must reuse common eventing, journal, replay,
evidence-ref, provider-status, custody/retention, LAN/network/browser/app-game,
notification, and AI-boundary infrastructure. Add tracking-specific payload
meaning and safety rules only where the shared layer cannot express the
location/geofence/expected-place/nearby-place semantics.

## Central schema boundary

```text
schema-domain owns canonical tracking event payload schemas and cross-boundary constants.
protocol/eventing boundaries may own neutral envelope, idempotency, journal, replay, dead-letter, and request/response shapes.
tracking-domain may expose helpers/proof adapters only.
tracking-core may mirror canonical payloads for runtime use only.
```

If an event payload crosses a package, crate, plan, service, portal, policy,
notification, custody, or proof boundary, it must not be canonical inside
`tracking-domain` or `tracking-core`.

## Required proof fields

The selected proof must name, at minimum:

```text
canonical_schema_owner_state
event_constant_owner_state
event_payload_schema_state
protocol_parity_state
rust_mirror_state
local_event_string_rejection_state
generic_eventing_reuse_state
private_bus_state
duplicate_event_type_state
schema_roundtrip_state
missing_causation_rejection_state
missing_evidence_ref_rejection_state
ai_boundary_state
notification_policy_ref_state
live_mode_ttl_audit_state
manual_required_gap_state
no_runtime_dispatch_claim
no_product_ready_claim
no_claim
```

## Required Source Behavior

Canonical tracking config event owners live in
`packages/schema-domain/src/agent-tracking-retention-settings-write-command.ts`
under `AgentTrackingConfigCommandFlowEventType` and
`AgentTrackingConfigUpdateEventType`.

- Add event type constants for:
  - `AgentTrackingConfigCommandFlowEventType.ChangeRequested`
  - `AgentTrackingConfigCommandFlowEventType.ChangeApproved`
  - `AgentTrackingConfigCommandFlowEventType.ChangeRejected`
  - `AgentTrackingConfigUpdateEventType.Applied`
  - `location.evidence.observed`
  - `geofence.transition.evaluated`
  - `expected_place.status.evaluated`
  - `nearby_place.analysis.requested`
  - `nearby_place.analysis.completed`
  - `tracking.detection.completed`
  - `tracking.live_mode.start_requested`
  - `tracking.live_mode.started`
  - `tracking.live_mode.stop_requested`
  - `tracking.live_mode.stopped`
  - `notification.intent.created`
  - `notification.dispatch.requested`
  - `notification.dispatch.result_observed`
  - `escalation.intent.created`
  - `escalation.result_observed`
- Add schema-backed payloads with event id, correlation id, causation id,
  aggregate key, evidence refs, policy refs, custody, retention, capability,
  uncertainty, TTL/deadline, idempotency, and audit refs where applicable.
- Keep generic reusable eventing code free of tracking product types.
- Do not duplicate generic event bus, journal, replay, evidence-ref,
  provider-status, notification, AI-provider, LAN, network, browser, or
  app/game mechanics in tracking-specific source.
- Add duplicate event-type tests and schema/serde roundtrip tests.
- Add source-boundary guards or focused tests that reject local string invention
  where the repository already supports that check.
- Put new Rust tracking tests under crate-level `tests/` folders when behavior
  is exposed through a public crate API.

## Tests After Code

- Event constants are unique.
- Every event type has a schema or Rust payload contract.
- Every schema/payload roundtrips.
- Missing causation is rejected for command/derived events.
- Missing evidence refs are rejected for evidence-derived events.
- AI events cannot contain policy, enforcement, notification, live-mode, or
  escalation command fields.
- Notification and escalation events require policy decision refs.
- Live-mode events require TTL, reason, start/stop condition, and audit ref.
- Nearby-place events require ambiguity or uncertainty state.

## Proof After Tests Pass

Proof root:

```text
output/tracking-plan-proof/34-tracking-event-contracts/
```

The proof must list source files changed, tests changed, commands run, event
types covered, schema owner for each public event, claims proven, claims not
proven, and manual-required gaps. It must not be accepted if
`sourceFilesChanged` is empty or only lists proof files.

## Manual-Required Gaps

- Runtime dispatch is not proven by constants/contracts alone.
- Parent config flow, detection cascade, notification dispatch, provider
  receipt, child-device runtime, physical-device behavior, authority, and
  production behavior remain unclaimed until later workpacks implement them.

## 2026-07-31 execution evidence

WP34 now has a Rust-owned canonical catalog in
`crates/schema/src/tracking_event_contracts.rs`. It covers the nineteen named
event identities and validates cross-boundary correlation, aggregate,
idempotency, causation, evidence, policy, live-mode TTL/audit, and AI
uncertainty/authority boundaries. The older agent-protocol tracking constants
remain transitional consumers; this workpack does not claim their runtime
migration or dispatch.

Proof: `output/tracking-plan-proof/34-tracking-event-contracts/`.

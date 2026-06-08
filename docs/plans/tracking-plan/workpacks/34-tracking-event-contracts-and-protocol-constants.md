# WP34 Tracking Event Contracts And Protocol Constants

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

## Required Source Behavior

- Add event type constants for:
  - `tracking.config.change_requested`
  - `tracking.config.change_approved`
  - `tracking.config.change_rejected`
  - `tracking.config.applied`
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
types covered, claims proven, claims not proven, and manual-required gaps. It
must not be accepted if `sourceFilesChanged` is empty or only lists proof files.

## Manual-Required Gaps

- Runtime dispatch is not proven by constants/contracts alone.
- Parent config flow, detection cascade, notification dispatch, provider
  receipt, child-device runtime, physical-device behavior, authority, and
  production behavior remain unclaimed until later workpacks implement them.

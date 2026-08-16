# Current Tracking Snapshot

<!-- agent-capsule -->

> Agent Capsule
> Plan: `tracking-plan`
> Doc: `Current Tracking Snapshot`
> Kind: current snapshot; read for status/gap claims.
> Proves: the current routed status only, not product readiness or proof closure.

<!-- /agent-capsule -->

## Snapshot date

2026-08-15.

## Code-first status

- Authoritative matrix: [CODE_AUDIT.md](CODE_AUDIT.md).
- All 42 graph-imported workpacks have reviewed code/test topology.
- 24 workpacks have no bounded Phase 1 source/test-writing gap; three are
  imported reference packets and four are coordination/proof-routing packets.
- 18 workpacks retain concrete production-code or expected-test gaps.
- Focused tests, Enforcer acceptance, proof regeneration, and physical-device
  checks were not run in this Phase 1 audit.

## Runtime that exists

- Rust-owned tracking identifiers, config/runtime events, constants, read-model
  DTOs, and event-contract validation.
- Location observation validation with malformed coordinate/accuracy negatives.
- Device freshness, heartbeat, sync, battery, connectivity, radio, pending
  upload, service, permission, and capability decisions.
- Geofence and expected-place evaluation with accuracy, stale, ambiguity,
  grace, midnight, holiday, and trip cases.
- Idempotent parent acknowledgement and child check-in request/receipt/recorded
  flows.
- Nearby-place provider abstraction and ambiguity/no-authority behavior.
- Tracking AI request/result evidence validation and family isolation.
- Tracking policy compilation plus child-local nearby/expected-place policy
  evaluation.
- Alert severity, duplicate/missing-evidence suppression, and notification
  intent conversion.
- Process-local parent config and child detection event flows.
- ActivityStore/SQLite-backed tracking read-model queries and service payloads.
- Rust-owned parent snapshot metadata and portal tracking presentation states.

## Source truth corrections

- `packages/tracking-domain` is absent; it is not a current owner.
- `scripts/test/tracking-*.mjs` is absent; old proof commands and generated
  `output/tracking-plan-proof` claims cannot be treated as clean-checkout
  verification.
- The former statement that first-class tracking event families were missing is
  stale: WP34 contracts and WP35/WP36 process-local flows now exist.
- Those flows are not durable. `TrackingRuntimeEventFlow` creates an in-memory
  `EventBus`, while the SQLite read model is a separate ActivityStore path.

## Missing product runtime

- Android foreground/background/system-geofence/battery-connectivity adapters.
- iOS Core Location foreground/background/region/significant-change adapters.
- Desktop presence-hint collectors with provenance enforcement.
- Production retention/delete/export propagation and a durable parent-defined
  place database.
- Concrete Places/POI and AI provider execution routes.
- Durable temporary-live and missing-device lifecycle composition.
- Durable notification outbox/provider receipt/quiet-hours/escalation engine.
- Durable tracking journal append/restart replay/idempotent SQLite projection.
- Event-to-portal restart proof and actual child product UI.

## Highest-impact next slice

WP37 is the first code dependency: persist the already-written tracking runtime
event chain, replay it after restart, and project it idempotently into the
existing SQLite read model. That makes WP35/WP36 durable and unblocks an honest
WP39 event-to-portal chain. WP38/WP27 follow for notification and escalation.

## No-claim boundaries

- LAN/IP/Wi-Fi presence is not GPS or precise child location.
- Nearby-place and AI results are evidence, not household/policy authority.
- Notification intent is not provider delivery or receipt.
- Hosted portal cards are not child-device or production runtime proof.
- Android/iOS background behavior is not proved without physical-device
  lifecycle evidence.
- Planning docs, graph mappings, and old ignored output are not completion
  evidence.

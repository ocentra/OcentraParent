# Tracking Plan Code/Test Audit

Date: 2026-08-15
Branch baseline: `develop` at `bd27c29498b011d2d11640c7290fe178dbcf49b9`

## Authority

This is the code-first Phase 1 status for all 42 graph-imported tracking
workpacks. It is based on current production source and expected tests. A graph
mapping proves reviewed topology only; it does not prove that the mapped code
fulfils the workpack.

Focused tests, Enforcer acceptance, physical-device proof, generated proof, CI,
and merge state are later phases. Historical `output/` artifacts and checked
Markdown boxes were not used as implementation evidence.

## Production reachability pass (2026-08-16)

This pass followed the shipped agent/child/parent entrypoints rather than
counting mapped files as runtime completion. No production slice was accepted:
the smallest honest WP37 change requires an existing composition owner for the
tracking journal, event-to-`ActivityEvent` mapping, durable key/path
configuration, startup replay, and idempotent projection. That owner was
absent; WP40 now records the required route without claiming implementation.

| Workpacks | Reachable production path | Effect and remaining gap |
| --- | --- | --- |
| WP01-WP02 | Plan/source reconciliation only | Documentation scope; no product entrypoint or runtime effect. |
| WP03-WP06 | `agent-protocol` contracts and `tracking-core` validation/status decisions | Typed inputs and fail-closed model decisions exist; no platform capture caller. |
| WP07 | `agent-service` retention-settings write plus `tracking-core` transforms | Settings/value transformations only; no production cleanup/export worker or end-to-end custody path. |
| WP08-WP13 | Neutral `tracking-core` location/status/geofence logic and portal manual/unsupported states | No Android/iOS/desktop acquisition or lifecycle caller; platform claims remain manual-required. |
| WP14-WP19 | `tracking-core` geofence, schedule, acknowledgement, check-in, and provider-abstraction functions | Process-local decisions only; no durable runtime/provider owner. |
| WP20 | `tracking-core` nearby-place abstraction/generated placeholder | No concrete POI/HTTP provider, credential boundary, or delivery caller. |
| WP21 | `tracking-core` local taxonomy/ambiguity model | Value classification only; no provider-backed product effect. |
| WP22 | `tracking-core::local_place_store::Store` | In-memory store; no durable parent-defined-place owner or reopen path. |
| WP23-WP24 | `child-ai-core`/`tracking-core` evidence boundary and deterministic helper | Evidence/decision contracts only; no selected provider execution, retry, or receipt owner. |
| WP25-WP26 | Policy compiler/evaluation and notification-intent conversion | Compile/evaluate/intent effects only; no delivery or enforcement authority. |
| WP27-WP29 | Alerting, escalation/live-mode/missing-device decision functions | No durable timer/session/lifecycle owner or authority-controlled platform action. |
| WP30-WP33 | Rust snapshots, portal presentation, coordination/proof routing | Presentation and proof routing do not create runtime capture, mutation, or delivery. |
| WP34-WP36 | `parent-runtime-core`/`child-runtime` config and detection flows; `TrackingRuntimeEventFlow::new` creates `EventBus::new` | Typed process-local cascades and receipts exist; no durable tracking journal or store projection. |
| WP37 | `child-runtime::tracking_runtime_flow` -> `tracking-core::read_model`; separate `agent-core::ActivityStore::ingest_journal` capture path | Blocked behind WP40: no tracking-event serializer, configured journal owner, startup replay, or idempotent cascade-to-SQLite composition. |
| WP38 | `child-runtime::tracking_runtime_flow::subscriptions` emits notification intent into in-memory state | Blocked: no durable outbox, provider receipt, quiet-hours/escalation timer, retry, dead-letter, or acknowledgement lifecycle. |
| WP39 | `agent-service::build_activity_tracking_read_model_report` reads `ActivityStore`; portal renders the Rust-owned snapshot | Blocked: the live tracking cascade never feeds that store, so restart-safe event-to-portal reachability is absent. |
| WP40 | No shipped owner; required child/service runtime composition is absent | **Incomplete / route only** | New dependency owner required for trusted tracking ingress, durable journal configuration, startup replay, and idempotent projection. No code, tests, proof, or product claim exists. |

The existing matrix below remains the detailed Phase 1 model/test audit. Its
“Complete for Phase 1” rows must not be read as shipped capture, durability,
provider delivery, or product readiness.

## Result

- 43/43 workpacks have exact reviewed code/test topology in the engineering
  graph.
- 24/43 have no Phase 1 source/test-writing gap in their bounded scope. Three of
  those are imported reference packets and four are coordination/proof-routing
  packets with no product-code owner.
- 19/43 retain a concrete production-code or expected-test gap.
- The live Tracking implementation is Rust-first. The former
  `packages/tracking-domain` owner and the advertised
  `scripts/test/tracking-*.mjs` suite do not exist in this checkout.

## Workpack matrix

| Workpack | Current code/test evidence | Phase 1 | Remaining source/test gap |
| --- | --- | --- | --- |
| WP01 Source index/reconciliation | Current Rust, service, runtime, policy, AI, notification, and portal owners were re-inventoried and the stale package/script claims were removed. | **Complete for bounded Phase 1** | Proof publication and product-status reconciliation remain later. |
| WP02 Current snapshot/gap map | The snapshot is reconciled to the same live source/test inventory and explicit no-claim boundaries. | **Complete for bounded Phase 1** | Focused validation and proof regeneration were not run. |
| WP03 Contract boundary/schemas | Rust protocol identifiers, runtime/config events, constants, generated bridge ownership, and contract tests cover the active tracking shape family. | **Complete for Phase 1** | TypeScript is generated/presentation-only; no `tracking-domain` authority remains. |
| WP04 Location evidence model | `TrackingLocationObservedEvent` plus latitude/longitude/accuracy validation and negative tests are written. | **Complete for Phase 1** | Platform acquisition belongs to WP08-WP12. |
| WP05 Device status model | Rust device status covers freshness, heartbeat, sync, battery, connectivity, radio, backlog, and explicit service states with focused tests. | **Complete for Phase 1** | OS collection belongs to platform adapters. |
| WP06 Permission/capability model | Rust capability evaluation distinguishes foreground/background, approximate-only, denied, restricted, unsupported, unavailable, and manual-required states with tests. | **Complete for Phase 1** | OS permission acquisition belongs to platform adapters. |
| WP07 Retention/custody | Rust delete/export transformations, a local settings file, SQLite read-model tombstones, service write responses, and tests exist. | **Incomplete** | There is no production tracking evidence-store cleanup/export worker, atomic/concurrent settings-store hardening, or end-to-end delete propagation across journal/projection/UI. |
| WP08 Android foreground adapter | Neutral location/status validation can consume Android-shaped observations. | **Incomplete** | No Android Fused Location/foreground service production adapter or platform integration test exists. |
| WP09 Android background/geofence adapter | Neutral geofence/capability logic and tests exist. | **Incomplete** | No Android background permission/system geofence registration, delivery, restart, or denial-path adapter/test exists. |
| WP10 Android battery/connectivity/status adapter | Neutral battery/connectivity/status evaluation is tested. | **Incomplete** | No Android OS battery/connectivity collector or live status-to-runtime integration test exists. |
| WP11 iOS foreground adapter | Neutral location/status validation can represent iOS capability states. | **Incomplete** | No Core Location foreground adapter, authorization flow, or simulator/device integration test exists. |
| WP12 iOS background/region adapter | Neutral geofence/capability logic exists. | **Incomplete** | No iOS Always/region/significant-change registration, relaunch delivery, denial, or lifecycle integration test exists. |
| WP13 Desktop presence hints | The UI and status model preserve manual/unsupported states. | **Incomplete** | No Windows/macOS/Linux presence-hint adapter or test proves hint provenance while forbidding precise-location claims. |
| WP14 Geofence rule model | Rust geofence rule refs, capability/accuracy/grace inputs, event contracts, and tests are written. | **Complete for Phase 1** | Platform registration remains WP09/WP12. |
| WP15 Geofence transition engine | Enter, exit, dwell, unchanged, ambiguous, stale, grace, ordering, and citation behavior are implemented and tested. | **Complete for Phase 1** | Durable replay belongs to WP37. |
| WP16 Expected-place schedule engine | Schedule windows, midnight crossing, grace, holidays, trips, missed arrival, early exit, stale/manual states, and tests are written. | **Complete for Phase 1** | Durable scheduling/recovery is a later runtime concern. |
| WP17 Parent acknowledgement/exceptions | Idempotent acknowledgement plus expected-place holiday/trip exceptions are implemented and tested. | **Complete for Phase 1** | Provider/UI delivery is outside this bounded model. |
| WP18 Child check-in flow | Typed request/receipt/recorded events, parent request flow, child subscription, stale/duplicate/unsupported handling, and integration tests are written. | **Complete for Phase 1** | Physical child-device delivery proof is later. |
| WP19 Nearby-place provider abstraction | Provider availability, radius, candidate ambiguity, evidence-only authority, and no-policy-direct tests are written. | **Complete for Phase 1** | Concrete providers belong to WP20. |
| WP20 Google Places/POI adapter | Only local-cache/unavailable provider decisions and a generated placeholder surface exist. | **Incomplete** | No Google Places, MapKit, or OpenStreetMap HTTP adapter, credential/redaction boundary, retry/limit handling, or adapter integration tests exist. |
| WP21 Place taxonomy/ambiguity | Rust place kinds, risk categories, candidate ambiguity, confidence basis, and local/nearby mapping tests are written. | **Complete for Phase 1** | Provider quality proof remains WP20. |
| WP22 Parent-defined place database | Create/update/import/delete/tombstone/export/match logic and tests exist as value transformations. | **Incomplete** | The `Store` is an in-memory struct; there is no durable database, reopen/migration/concurrent-write behavior, or filesystem/SQLite test. |
| WP23 AI location-safety contracts | Tracking request/result evidence refs, private-payload rejection, family isolation, stale correlation, and evidence-only authority are tested across tracking-core and child-ai-core. | **Complete for Phase 1** | Provider execution belongs to WP24. |
| WP24 AI provider routing | Boundary validation and a deterministic classification helper exist. | **Incomplete** | No selected local/remote provider route, availability scheduler, execution receipt, timeout/retry, or provider integration test exists. |
| WP25 Tracking policy compiler | Rust policy-control compiler supports the Tracking domain, and child-policy evaluates nearby/expected-place decisions with ready/unsupported/rollback tests. | **Complete for Phase 1** | Delivery proof is a later cross-plan phase. |
| WP26 Alert/notification model | Alert severity, duplicate/missing-evidence suppression, parent-notification intent, policy-source authority, and tests are written. | **Complete for Phase 1** | Provider delivery and escalation belong to WP27/WP38. |
| WP27 Escalation engine | The schema reserves escalation event families and alert evaluation exists. | **Incomplete** | No escalation state machine, timer/quiet-hours worker, cancellation/ack transition, persistence, retry, or lifecycle tests exist. |
| WP28 Temporary live tracking | Authority/disclosure/expiry/auto-stop decision logic and rollback tests exist. | **Incomplete** | No durable session owner, cadence scheduler, restart recovery, platform sampling control, or end-to-end stop test exists. |
| WP29 Missing-device mode | A last-known-only decision model and hosted presentation helper exist. | **Incomplete** | No durable missing-device lifecycle, recovery/clear transition, authority-controlled platform action, or end-to-end UI/runtime test exists. |
| WP30 Parent/child UI surfaces | Rust snapshot metadata, portal tracking cards, hosted/manual states, and parent-route tests exist. | **Incomplete** | The child runtime UI, live parent mutations, real platform/provider delivery states, and product end-to-end interaction tests are absent. |
| WP31 Platform proof routing | This coordination-only workpack owns no product code and correctly keeps real Android/iOS/desktop claims manual-required. | **Complete for bounded Phase 1** | Phase 3 must supply actual device artifacts before platform claims move. |
| WP32 Journal/SQLite/read model | SQLite ActivityStore projection, query guards, tombstone visibility, migration/differential checks, service payload, and focused tests are written. | **Complete for Phase 1** | First-class tracking journal replay is separately owned by WP37. |
| WP33 Rollout/proof gate | This packet owns no product implementation. | **Incomplete** | Its advertised executable `scripts/test/tracking-*.mjs` aggregate/proof gates are absent, so clean-checkout verification cannot be regenerated as documented. |
| WP34 Event contracts/constants | Canonical Rust tracking event registry, runtime event contracts, identifiers, constants, schema negatives, and contract tests are written. | **Complete for Phase 1** | Runtime durability belongs to WP37. |
| WP35 Parent config command flow | Parent approval/rejection chain, typed events, child apply/persist response, audit/read-model hops, and focused parent/child tests are written. | **Complete for Phase 1** | Physical child service delivery remains later proof. |
| WP36 Detection cascade | Child event-bus cascade covers location validation, evidence, geofence, expected place, AI, policy, alert, notification, and check-in branches with integration tests. | **Complete for Phase 1** | The flow is process-local; persistence belongs to WP37 and delivery/escalation to WP38. |
| WP37 Journal replay/projection | Process-local EventBus journal snapshots and separate SQLite read-model querying exist. | **Incomplete** | `TrackingRuntimeEventFlow` uses a fresh in-memory bus; no durable append/restart replay/idempotent projection chain connects runtime events to SQLite. WP40 is the reviewed composition dependency. |
| WP38 Notification/escalation event flow | Notification intent is emitted from the in-memory detection cascade and tracking event schemas reserve notification/escalation families. | **Incomplete** | No durable outbox/provider receipt/quiet-hours/escalation timer/retry/dead-letter/ack lifecycle or integrated tests exist. |
| WP39 Portal event read-model proof | Portal renders Rust-owned tracking snapshots and ActivityStore-backed rows with focused presentation/service tests. | **Incomplete** | The live tracking cascade is not durably projected into that read model, and no end-to-end event-to-portal restart test closes the chain. |
| WP40 Trusted runtime ingress/journal composition | No production owner exists; this packet records the missing boundary and required outcome only. | **Incomplete** | Must be implemented in the real child/service lifecycle before WP37 can be legally selected. |
| Device location tracking capability guide | Imported reference packet; no product code belongs to it. | **Complete for bounded Phase 1** | It remains design/reference input. |
| Device location tracking schema proposal | Imported reference packet; Rust owners supersede its historical TypeScript-like proposal shapes. | **Complete for bounded Phase 1** | It remains design/reference input. |
| Tracking control settings inventory | Imported settings inventory; no product runtime belongs in the document. | **Complete for bounded Phase 1** | Individual controls still require their owning workpacks. |

## Highest-impact implementation order after Phase 1

1. WP40: trusted tracking runtime ingress and durable journal composition. This
   is the missing production owner required before WP37.
2. WP37: durable tracking journal replay and idempotent SQLite projection. This
   consumes WP40 and turns the already-written WP35/WP36 event flows into
   restart-safe product state.
3. WP38 and WP27: durable notification outbox/receipt plus escalation timer and
   acknowledgement lifecycle.
4. WP22 and WP07: durable parent-defined places and evidence retention/custody
   execution.
5. WP08-WP12: real platform acquisition adapters and lifecycle tests.
6. WP20/WP24: real POI and AI provider routing boundaries.
7. WP28-WP30/WP39: live/missing-device runtime composition and complete
   parent/child product surfaces.

## Phase boundary

This audit did not run product tests or regenerate proof. Phase 2 must select
focused tests and Enforcer checks for each implementation slice. Phase 3 then
regenerates tracked proof from a clean checkout before any workpack or product
readiness claim moves.

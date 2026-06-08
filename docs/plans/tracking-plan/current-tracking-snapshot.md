# Current Tracking Snapshot

<!-- agent-capsule -->

> Agent Capsule
> Plan: `tracking-plan`
> Doc: `Current Tracking Snapshot`
> Kind: current snapshot; read for status/gap claims.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.

<!-- /agent-capsule -->

## Snapshot Date

2026-06-08.

## Source Inputs That Exist

- Feature doc exists.
- Expectation doc exists.
- Capability guide exists.
- Schema proposal exists.
- Tracking settings inventory exists with 338 raw settings.
- Location posture modes are represented as design inputs: Off, Last known,
  Check-in, Arrival alerts, Temporary live, and Missing device.
- Capability/degraded vocabulary exists as raw inventory input, including
  service-disabled, manual-required, offline-last-known-only, and
  battery-throttled.
- Device-status design inputs exist for last heartbeat, last location sample,
  last sync, battery percentage, charging state, low-power mode, and pending
  upload count.

## Local/CI Proof Now Exists

- Focused TypeScript contract proof exists for tracking evidence,
  geofence/place models, tracking AI safety evidence, tracking policy/action
  contracts, and proof-routing states.
- P1 deterministic proof exists for geofence transitions, expected-place
  decisions, acknowledgement/exception handling, child check-in, temporary live
  tracking, missing-device mode, retention/delete/export, and report/export
  read-model boundaries.
- Android emulator foreground/background scaffold proof exists, including
  emulator foreground permission and local background/geofence storage rows. It
  is not physical-device or Android system geofence delivery proof.
- WP08/WP30/WP33 child-runtime Android emulator readiness bridge proof links the
  Android emulator package, foreground-service, permission, and local emulator
  geofence evidence into child-runtime readiness accounting while preserving
  actual child-device runtime artifacts as P4 manual-required proof.
- iOS simulator/manual-required proof exists for package-preview/simulator
  routing and privacy-disclosure release gates. It is not iOS Always/region or
  real-device proof.
- Desktop presence proof exists only as hint-only LAN/IP/Wi-Fi state, not
  precise desktop location proof.
- Rust ActivityStore and service read-model proof exists for local tracking
  event ingestion and the `agent.activity.tracking.read-model.get` service
  path.
- Hosted parent-route UI proof exists for the current tracking route, hosted
  child check-in/runtime cards, evidence drawer, report/export surfaces,
  notification parent-surface history, retention settings local write result,
  and unsupported/manual platform states. It is not full product parent/child
  runtime UI proof.
- Service-data UI proof now carries a service-backed citation matrix that ties
  the hosted coverage card to the same read-model command, event, payload,
  citation fields, active evidence refs, tombstone deleted-evidence refs, and
  no-claim boundaries used by the live citation rows.
- Retention product-settings writable execution proof now carries a derivation
  matrix from accepted local service state to the local runtime artifact,
  preserving source refs, service revision, snapshot ref, durable store ref,
  applied values, and no-claim boundaries while keeping platform enforcement
  and product-ready retention false.
- Full product UI local runtime artifact capture proof now consumes the
  retention writable execution derivation proof and the child-runtime artifact
  gate proof as closure evidence while keeping full product UI runtime,
  child-device runtime, physical-device, authority, provider delivery,
  production UI, and product-ready claims false.
- Provider and notification local proof exists for POI/provider mapping,
  provider-notification intent, notification preference preflight/status handoff,
  notification receipt boundary, local outbox readiness, provider-delivery
  artifact gates, and provider-runtime blocker accounting. It is not live
  provider credentials, dispatch, webhook ingestion, or receipt runtime proof.
- Authority, child-runtime, full-product UI, production worker, retention
  runtime, physical-device, and escalation artifact gates now enumerate required
  real-runtime artifacts while keeping product claims false.
- The real-runtime handoff proof now carries row-level blocker ids, required
  manual validation commands, artifact acceptance notes, and a generated manual
  validation runbook for the P4 handoff rows so the remaining Android, iOS,
  child-runtime, product UI, authority, provider, retention, production, and
  escalation gaps can be closed without inventing duplicate truth.
- Product-readiness closure proof exists through
  `node scripts/test/tracking-product-readiness-closure-proof.mjs`; it verifies
  the local/CI proof accounting chain, now carries aggregate evidence for five
  observed full-product UI local artifacts, one retention writable execution
  derivation row, ten child-runtime artifact gaps, and the claim-audit blocker
  counts, and keeps product-ready tracking false.

## Runtime/Product Claims Still Missing

- Real Android physical-device background location and system geofence delivery
  proof.
- Real iOS Always/region/background physical-device proof.
- Product-ready retention settings execution and platform runtime enforcement
  proof beyond the local writable execution artifact.
- Actual child-device delivery/runtime execution and rendered child-device UI
  proof.
- Full product parent/child UI beyond the hosted parent proof route.
- Authority-enrolled device-owner/managed-profile/supervised-device hard-control
  runtime proof.
- Live provider delivery/receipt runtime with credentials, adapter dispatch,
  webhook ingestion, retry/quiet-hours runtime, and parent notification history
  runtime.
- Production durable workers, durable outbox/history/storage, escalation worker,
  quiet-hours timer, and production support proof.

## Manual Required

- Android background location proof.
- Android geofence proof.
- iOS Always/region/background proof.
- Desktop precise location proof.
- Remote sync and remote AI proof.
- Emergency/critical escalation proof.
- Managed-device/MDM/lost-mode proof.

## No-Claim Boundaries

- LAN presence is not GPS.
- IP location is not precise location.
- Wi-Fi presence is not proof the child is physically there.
- A laptop left at home does not prove the child is home.
- Low-accuracy nearby places do not prove the child is inside one listed place.
- AI risk level is not household authority.
- Notification delivery is not evidence custody.
- Planning docs are not platform proof.

## Product Claim Gates

| Claim                     | Allowed only when                                                                               |
| ------------------------- | ----------------------------------------------------------------------------------------------- |
| Current location          | Fresh sample exists and UI shows accuracy, source, and freshness.                               |
| Last known location       | Stored sample exists and UI labels stale/last-known status.                                     |
| Arrived at school         | Geofence or expected-place rule cites location evidence and accuracy/grace checks pass.         |
| Left expected place       | Expected-place rule, schedule, tolerance, location evidence, and stale checks pass.             |
| Near hospital/cinema/mall | Nearby-place evidence cites source location, query radius, provider, ambiguity, and confidence. |
| Device offline            | Device status evidence shows heartbeat/connectivity state and last seen time.                   |
| Background tracking works | Platform permission/background behavior is proved on a real device.                             |
| Critical alert            | Parent policy rule configures critical behavior and evidence/action refs exist.                 |

## Product Claim Boundary

The repo now has planning documents, raw inventory, focused contract proof,
local/CI runtime fixture proof, hosted-route UI proof, service read-model proof,
artifact gates, and closure proof for tracking. Claims must stay limited to the
recorded proof tier. Physical-device, authority-enrolled, provider runtime,
production, full parent/child UI, and product-ready tracking claims remain
false until matching real-runtime artifacts exist.

## Contracts That Exist

Dedicated TypeScript tracking contracts now exist under
`packages/activity-domain/src/tracking.ts` and
`packages/parent-domain/src/tracking-location-policy.ts`, with tests in
`packages/activity-domain/tests/tracking.test.ts` and
`packages/parent-domain/tests/tracking-location-policy.test.ts`.

The focused proof root is `output/tracking-plan-proof/`. Complete contract
proof exists for workpacks 03, 04, 05, 06, 13, 14, 17, 18, 19, 21, 23, 24,
26, and 31. Partial contract proof exists for 07, 15, 16, 22, 25, 27, 28, 29,
32, and 33 because runtime delete/export, engines, stores, escalation, live
tracking, missing-device UI/runtime, Rust journal/SQLite, and full platform/UI
proof remain pending.

## Feature Routing Snapshot

`location-geofence-device-status` is the owning feature doc. It can consume
evidence from browser/app/LAN/evidence-store features by reference, but
tracking must own the child-location claim boundary and no-claim rules.

## Rust Runtime That Exists

Rust tracking proof now includes ActivityStore ingestion/read-model tests and a
real service read-model path. It does not yet prove mobile platform adapters,
provider dispatch, production workers, authority control, or product-ready
runtime behavior.

## Portal That Exists

Hosted portal proof exists for the current tracking parent route, service-backed
summary/citation states, report/export rows, notification parent surface,
retention settings local write result, child check-in/runtime proof cards, and
unsupported/manual platform states. Full product parent/child UI and actual
child-device runtime UI remain proof-gated.

## Proof That Exists

Existing proof includes documentation proof, focused contract proof, local/CI
runtime fixture proof, hosted UI proof, service/read-model proof, artifact
gates, and closure proof:

- source index;
- current snapshot;
- pasted-content coverage audit;
- workpack split;
- product-doc references.
- `node scripts/test/tracking-plan-contract-proof.mjs`;
- `node scripts/test/tracking-source-reconciliation-gap-map-proof.mjs`;
- `node scripts/test/tracking-product-readiness-closure-proof.mjs`;
- generated proof roots under `output/tracking-plan-proof/`;
- activity-domain and parent-domain tracking contract builds/tests;
- schema-boundary/source-shape guard proof.

## Remaining Product-Claim Blockers

- `android-physical-background-proof-required`
- `ios-physical-region-proof-required`
- `retention-writable-product-settings-required`
- `retention-platform-runtime-enforcement-required`
- `actual-child-device-runtime-required`
- `full-product-parent-child-ui-required`
- `authority-enrollment-proof-required`
- `provider-delivery-receipt-runtime-required`
- `production-durable-workers-required`

These blockers are expected. Local/CI proof accounting can be complete while
product-ready tracking remains false.

## Enhancement Rule

Future workers may mark a status complete only when the matching proof pack
exists and the product feature doc plus capability checklist were updated or
explicitly justified as unchanged.

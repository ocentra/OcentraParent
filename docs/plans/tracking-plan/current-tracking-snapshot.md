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

2026-06-02.

## Existing

- Feature doc exists.
- Expectation doc exists.
- Capability guide exists.
- Schema proposal exists.
- Tracking settings inventory exists with 338 raw settings.
- Focused TypeScript contract proof now exists for tracking evidence,
  geofence/place models, tracking AI safety evidence, tracking policy/action
  contracts, and proof-routing states.
- Location posture modes are represented as design inputs: Off, Last known,
  Check-in, Arrival alerts, Temporary live, and Missing device.
- Capability/degraded vocabulary exists as raw inventory input, including
  service-disabled, manual-required, offline-last-known-only, and
  battery-throttled.
- Device-status design inputs exist for last heartbeat, last location sample,
  last sync, battery percentage, charging state, low-power mode, and pending
  upload count.

## Missing Product Runtime

- Platform adapter proof.
- Journal/SQLite read models.
- Parent/child UI.
- Notification/escalation engine.
- Retention/delete/export proof.
- Android/iOS foreground and background location runtime.
- Nearby-place provider runtime.
- Expected-place and geofence transition runtime engines.

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

The repo now has planning documents, raw inventory, and a focused
contract-proof spine for tracking. Claims must stay limited to contract proof
until platform adapters, journal/read models, UI, provider runtime, and full
proof packs exist.

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

No Rust tracking runtime proof exists in this snapshot. Rust implementation is
now unblocked by explicit TypeScript domain contracts, but still needs
test-backed protocol mirroring, journal ingest, SQLite replay/query, and
delete/export proof before runtime claims can move.

## Portal That Exists

No product-complete parent or child tracking UI exists in this snapshot.
Portal work remains blocked on contracts, read models, and UI snapshot states.

## Proof That Exists

Existing proof includes documentation proof plus focused contract proof:

- source index;
- current snapshot;
- pasted-content coverage audit;
- workpack split;
- product-doc references.
- `node scripts/test/tracking-plan-contract-proof.mjs`;
- generated proof roots under `output/tracking-plan-proof/`;
- activity-domain and parent-domain tracking contract builds/tests;
- schema-boundary/source-shape guard proof.

## Current Gaps

The main gaps are runtime contracts, real platform permission proof,
journal/SQLite ingest, read models, WebSocket protocol, parent/child UI,
notification policy integration, retention/delete/export proof, and
Playwright/manual evidence.

## Enhancement Rule

Future workers may mark a status complete only when the matching proof pack
exists and the product feature doc plus capability checklist were updated or
explicitly justified as unchanged.

# Current Tracking Snapshot

## Snapshot Date

2026-06-02.

## Existing

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

## Missing

- Runtime `LocationEvidence` contract.
- Runtime `DeviceStatusEvidence` contract.
- Runtime `LocationCapabilityStatus` contract.
- Runtime `LocationRetentionPolicy` contract.
- Runtime `GeofenceRule` and `GeofenceTransition` contracts.
- Expected-place schedule model.
- Parent acknowledgement and exception model.
- Nearby-place evidence model.
- AI safety analysis contracts.
- Platform adapter proof.
- Journal/SQLite read models.
- Parent/child UI.
- Notification/escalation engine.
- Retention/delete/export proof.

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

The repo currently has planning documents and raw inventory, not a
product-complete tracking runtime. Claims must stay limited to planning
coverage until contracts, platform adapters, journal/read models, UI, and
proof packs exist.

## Contracts That Exist

No dedicated runtime `LocationEvidence`, `DeviceStatusEvidence`,
`LocationCapabilityStatus`, `GeofenceTransition`, `NearbyPlaceEvidence`,
`LocationAiSafetyResult`, or `LocationAlert` contract is product-complete yet.
The available contract source is the capability guide, schema proposal,
tracking inventory, and expectation docs.

## Feature Routing Snapshot

`location-geofence-device-status` is the owning feature doc. It can consume
evidence from browser/app/LAN/evidence-store features by reference, but
tracking must own the child-location claim boundary and no-claim rules.

## Rust Runtime That Exists

No Rust tracking runtime proof exists in this snapshot. Rust implementation is
blocked on explicit TypeScript domain contracts and test-backed protocol
mirroring.

## Portal That Exists

No product-complete parent or child tracking UI exists in this snapshot.
Portal work remains blocked on contracts, read models, and UI snapshot states.

## Proof That Exists

Existing proof is documentation proof only:

- source index;
- current snapshot;
- pasted-content coverage audit;
- workpack split;
- product-doc references.

## Current Gaps

The main gaps are runtime contracts, real platform permission proof,
journal/SQLite ingest, read models, WebSocket protocol, parent/child UI,
notification policy integration, retention/delete/export proof, and
Playwright/manual evidence.

## Enhancement Rule

Future workers may mark a status complete only when the matching proof pack
exists and the product feature doc plus capability checklist were updated or
explicitly justified as unchanged.

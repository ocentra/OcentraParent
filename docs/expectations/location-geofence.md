<!-- agent-capsule -->

> Agent Capsule
> Doc: Location And Geofence Expectations
> Kind: expectation/acceptance documentation; read only when selected by feature doc, plan route, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# Location And Geofence Expectations

Location is a separate family-safety product area. It must not be implied by LAN
presence, IP address, or device pairing.

## Parent Outcome

- Parent can see last-known location where the child device and platform allow
  it.
- Parent can configure geofences and alerts.
- Parent can see device battery/connectivity status when available.
- Parent can distinguish live, stale, unavailable, permission-denied, and
  parent-cache states.
- Parent can disable location collection and retention.

## Child-Device Outcome

- Child-device agent or platform wrapper records location evidence only when
  the parent setting, platform permission, and OS policy allow it.
- Location events carry source, accuracy, timestamp, custody, retention, and
  stale-state metadata.
- Geofence evaluation is local or explicitly documented as parent-owned/relay
  behavior.

## Data Scope

Location data may include coordinates, accuracy, provider, timestamp, battery,
connectivity, geofence id, transition type, retention policy, and audit refs.

Location data must not be inferred from network metadata and presented as GPS.

## Contract Boundary

Expected contract families:

- `LocationEvidence`
- `LocationCapabilityStatus`
- `GeofenceRule`
- `GeofenceTransition`
- `DeviceBatteryStatus`
- `LocationRetentionPolicy`
- `LocationAlert`

## Acceptance

- Parent-visible source state is explicit.
- Geofence alerts cite evidence and rule references.
- Unsupported platforms show unavailable/manual-required states.
- Retention/delete behavior is visible and tested.

## Validation Gates

- TypeScript schema tests for location, capability, geofence, battery, alert,
  retention, and degraded states.
- Platform proof for Android/iOS permissions and background behavior.
- Portal tests for map/list/status UI when UI exists.
- Privacy/security review before any remote location sync or alerting.

## Non-Goals

- Do not infer precise location from IP address.
- Do not claim background mobile location without OS permission proof.
- Do not store location in Ocentra-hosted systems by default.

## Done Signal

A parent can see and configure location/geofence behavior with explicit source,
accuracy, custody, retention, alert, and platform support status.

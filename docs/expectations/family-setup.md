<!-- agent-capsule -->

> Agent Capsule
> Doc: Family Setup Expectations
> Kind: expectation/acceptance documentation; read only when selected by feature doc, plan route, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# Family Setup Expectations

Family setup is the first product experience. It must make Ocentra usable by a
nontechnical parent before policy, AI, reports, or enforcement can be trusted.

## Parent Outcome

- Parent can create or join a household.
- Parent can add child profiles.
- Parent can add child devices and see each device role/status.
- Parent can invite or remove co-parents and observers.
- Parent can recover from lost, stale, revoked, or offline devices.
- Parent can understand whether the current view is live local, LAN, relay,
  parent cache, parent-owned storage, or unavailable.

## Child-Device Outcome

- Child device has a stable local device identity.
- Child device knows its role and household trust state.
- Child device rejects stale, wrong-household, replayed, revoked, or
  unauthorized commands.
- Child device records setup, pairing, role, revocation, and recovery events.

## Data Scope

Family setup may include household id, parent id, child profile id, device id,
role, trust state, route state, display label, setup status, invite status,
revocation state, and audit references.

Family setup must not include raw child activity evidence in Ocentra-hosted
account systems by default.

## Contract Boundary

Expected contract families:

- `HouseholdProfile`
- `ParentMember`
- `ChildProfile`
- `DeviceRegistration`
- `DeviceRole`
- `ParentControllerLease`
- `ObserverPermission`
- `SetupInvite`
- `RecoveryState`
- `SetupAuditEvent`

## Acceptance

- Family setup works without editing files.
- Every device has a visible role and source state.
- Co-parent and observer permissions are separate from active controller
  authority.
- Revoked/stale/replayed commands are rejected by the child-device authority.
- Parent can see what remains incomplete before enabling rules or enforcement.

## Validation Gates

- TypeScript schema tests for family, member, child profile, device, invite,
  role, controller lease, observer, and recovery states.
- Rust parity tests for any setup shape crossing the service boundary.
- Portal tests for setup state, add-device state, role state, and unavailable
  state when UI exists.
- Real LAN or relay proof before claiming remote setup behavior.

## Non-Goals

- Do not make cloud account membership the source of child-device authority.
- Do not give observers write authority.
- Do not imply a device is protected before the child-agent capability status is
  known.

## Done Signal

A parent can create a household, add child profiles/devices, understand each
device role/status, and recover or revoke access through typed, audited flows.

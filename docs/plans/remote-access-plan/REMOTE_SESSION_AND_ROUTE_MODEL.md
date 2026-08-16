# Remote Session And Route Model

Remote access is a typed route/session fabric, not a generic remote flag.

## Required model entities

```text
RemoteRoute
RemoteSession
RemoteCapabilityGrant
RemoteTarget
RemoteAuditEvent
RemoteRollbackRef
RemoteGrantReason
RemoteDeviceState
```

## Required route kinds

```text
localhost
local-network
wan-direct
relay
parent-owned-relay
queued
offline
unavailable
stale
```

## Required session states

```text
requested
authorized
paired
connecting
active
degraded
paused
stopped
expired
revoked
denied
failed
reconnectPending
superseded
```

## Current-pass capability families

```text
device-health
activity-summary
report-query
rule-update
approval-decision
screen-snapshot
live-screen-view
support-diagnostics
```

## Deferred capability families

```text
remote-input
remote-control
app-control
browser-control
game-control
network-control
location-presence
assistant-action
```

## Required behavior

- Route, session, and capability state stay separate.
- Route choice is visible and testable.
- Initial pairing creates standing access until revoke or device removal.
- Live view and remote input/control are separate capabilities.
- Remote desktop is a deferred capability family, not the authorization model.
- Parent-visible and child-visible session state are both first-class.
- Revoked or removed grants cannot be resurrected by reconnect.
- Audit refs are required for granted, denied, degraded, revoked, removed, and failed paths.

## Routing and handoff rules

- `screen-plan` owns capture primitives and protected-surface rules.
- `lan-plan` owns local pairing and LAN transport.
- `account-identity-family-plan` owns parent, child, co-parent, support/admin, and device authority.
- `data-custody-storage-plan` owns retention/export/delete custody for remote artifacts.
- `portal-ux-household-surfaces-plan` owns the rendered parent surface once the route/session model exists.

## Negative cases

```text
one generic remote flag authorizes unrelated actions
live view and remote input collapse into one capability
route changes are invisible to the parent
pairing must be repeated on every visit
reconnect resurrects revoked access
child-visible state is omitted
audit ref missing on deny or revoke
```

## Proof expectation

The model is closed only when the proof inventory shows route, session, capability, audit, and negative-authZ coverage across parent, child, and relay states for the current live-view pass.

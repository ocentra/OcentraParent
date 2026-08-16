# Parent Remote UI Expectations

This doc defines the parent-facing UI states for remote live access, standing paired access, route understanding, live view, and deferred control.

## Required screens

```text
Remote home
Selected child/device strip
Route board
Pairing/status panel
Live view panel
Session/paired access panel
Event/audit rail
Manual-required gaps
```

## Required UI states

```text
paired
active
revoked
removed
denied
failed
reconnectPending
stale
offline
unavailable
queued
parentOwnedRelay
local-network
wan-direct
relay
pairingRequired
manualRequired
blockedSurface
```

## UI language constraints

Use:

```text
Paired
Reachable at home
Remote relay
View only
Waiting for child device
Permission needed on child device
Manual proof required
```

Do not use:

```text
Active everywhere
Saved and enforced
Hidden access
Always on
Works on all devices
```

unless proof exists.

## Required UI behavior

- Parent can tell which child and device are selected.
- Parent can see whether the device is paired, active, revoked, removed, offline, stale, or unavailable.
- Parent can see whether the route is local, LAN, WAN direct, relay, parent-owned relay, queued, offline, stale, or unavailable.
- Parent can see whether the capability is view-only.
- Parent can see why access is denied, removed, paused, degraded, revoked, or unavailable.
- Child-visible state must be reflected honestly in the parent view.
- Mobile and accessibility behavior are required, not implied.

## Negative cases

```text
route state is hidden behind a spinner
paired and unpaired use the same label
offline child is shown as active
remove-device state is hidden behind success colors
reconnect looks like fresh authorization
child-visible state is missing from the parent surface
```

## Proof expectation

The UI doc closes only when the proof inventory shows route, session, pairing, relay, live-view, and copy-state coverage, plus mobile/accessibility artifacts. Deferred control proof is not required for the current pass.

# Remote Access Plan Decisions

This file records the non-negotiable remote-access decisions that keep the plan from collapsing into local capture, LAN pairing, or generic remote-desktop tooling.

## RA-001: Remote access is a capability fabric

Decision:

```text
Remote access is a typed capability fabric with explicit route, session, grant, and audit state.
```

Rules:

```text
Live view is not control.
Remote control is not the auth model.
Support/admin diagnostics are not hidden access.
Remote desktop is just one capability family on the fabric.
```

## RA-002: Direct first, relay when needed

Decision:

```text
The fabric should attempt direct routes first and fall back to relay when direct is unavailable, denied, or too weak for the granted capability.
```

Rules:

```text
Localhost and LAN remain visible route kinds.
WAN direct and relay are first-class product states.
Forced relay is a valid proof mode.
Route choice must stay observable to the parent.
```

## RA-003: View first, control deferred

Decision:

```text
Screen visibility and live view are the current-pass remote-access capabilities. Remote input/control is a deferred capability family and is not claimed by this pass.
```

Rules:

```text
View-only is the active remote-access behavior in this pass.
Standing paired access may later support control, but control is not claimed here.
No control path may be hidden inside a view-only claim.
```

## RA-004: Consent and disclosure are first-class

Decision:

```text
Remote grants are established by initial pairing and remain active until parent revokes access or removes the device, with child-visible disclosure where the platform or product policy requires it.
```

Rules:

```text
Parent authority is necessary for pairing.
No repeated permission prompts are part of the standing-access model.
Child-visible stop/revoke state is part of the contract.
Revocation wins over reconnect.
```

## RA-005: No raw screen retention by default

Decision:

```text
Remote access does not retain raw screen frames or input payloads by default.
```

Rules:

```text
Snapshots/recording are opt-in and custody-governed.
Relay diagnostics are redacted.
Evidence custody stays explicit.
```

## RA-006: RustDesk is reference material only

Decision:

```text
RustDesk is used to borrow architecture and proof ideas, not source code, protocol schemas, UI code, or generated artifacts.
```

Rules:

```text
Borrow the split between rendezvous and relay.
Borrow session-scoped capability ideas.
Borrow platform-specific service lifecycle lessons.
Do not copy AGPL code or schema shapes.
```

## RA-007: Adjacent plans keep ownership

Decision:

```text
Screen-plan owns capture primitives, lan-plan owns local pairing, account-identity-family-plan owns actor/session/device authority, and data-custody-storage-plan owns retention/export/delete custody.
```

Rules:

```text
Remote-access-plan owns the remote session fabric, route/session state, capability grants, relay behavior, and parent-visible proof boundary.
```

## RA-008: Rollback and failure states are product states

Decision:

```text
Denied, revoked, expired, degraded, unavailable, stale, reconnect-pending, and failed are product states, not incidental errors.
```

Rules:

```text
Every denial has a visible reason and audit ref.
Revoke must stop reconnect from resurrecting a grant.
```

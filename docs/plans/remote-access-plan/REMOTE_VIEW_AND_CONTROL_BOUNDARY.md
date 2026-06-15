# Remote View And Control Boundary

View-only is the current-pass boundary. Standing paired access keeps the parent connected until revoke or device removal. Control is deferred.

## View-only

View-only includes:

```text
screen snapshot
live screen view
remote activity/health visibility
report query
rule-update preview
approval-decision preview
```

Rules:

```text
View-only can exist without control.
View-only does not grant input authority.
View-only must show route, state, and custody status.
View-only access stays standing after pairing until revoke or device removal.
```

## Deferred control

Control includes:

```text
remote input
remote control
app-control
browser-control
game-control
network-control
```

Rules:

```text
Control requires stronger confirmation than view-only.
Control must have a stop/revoke path.
Control must never be hidden inside a view-only claim.
Control cannot bypass policy, OS permission, or blocked-surface rules.
```

## Child-visible state

The controlled device must show:

```text
parent identity
active capability
route/session state
paired/active/revoked/removed state
stop/revoke path
OS permission state
degraded/unavailable reason
```

## Retention boundary

- Raw frames are not retained by default.
- Snapshots/recording require explicit opt-in and custody proof.
- Relay diagnostics remain redacted.
- Protected surfaces must surface explicit blocked or permission-required states.

## Negative cases

```text
view-only secretly injects input
control is granted by a view-only session
raw frames are retained by default
blocked surfaces appear as success
child-visible stop path is missing
paired state is hidden
```

## Proof expectation

The boundary is closed only when the proof inventory shows view-only, protected-surface, no-raw-retention, child-visible session, and route/reconnect evidence for the current pass. Remote-control negative-path evidence is deferred until that workpack is opened.

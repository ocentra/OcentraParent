# Relay And Abuse Model

Relay is transport, not custody. The current pass carries live-view routes and standing paired access, not repeated permission prompts.

## Required relay states

```text
direct
lan-direct
wan-direct
relay
parent-owned-relay
queued
offline
unavailable
stale
partial-outage
backpressure
reconnectPending
```

## Required controls

- Authenticated, capability-scoped session setup.
- Rate limits and connection limits by household/device.
- Replay rejection for tokens and session requests.
- Cross-household routing isolation.
- Redacted logs, metrics, and alerts.
- Explicit timeout and retry behavior.
- Progressive delay or lockout for repeated failures.

## Required behavior

- Route choice stays visible to the parent.
- Relay failure becomes a product state, not a silent fallback.
- Partial outage and degraded states remain honest.
- Diagnostics never store raw screen/input payloads by default.
- Support/admin cannot bypass parent-visible session grants.
- Standing paired access remains visible until revoke or device removal.

## Threat surfaces

```text
unauthorized session creation
token replay
relay resource exhaustion
slow dependency
retry storm
cross-household routing bug
frame/content retention leak
origin/host confusion
open redirect or URL hijack
cache poisoning or stale grant reuse
```

## Negative cases

```text
relay treated as trusted pipe
raw screen payload appears in diagnostics
cross-household session is allowed
token replay succeeds
parent sees relay failure as success
```

## Proof expectation

The relay model is closed only when the proof inventory shows rate-limit, retry-storm/backpressure, cross-household denial, replay denial, redacted diagnostics, partial-outage, and alerting evidence for the current pass.

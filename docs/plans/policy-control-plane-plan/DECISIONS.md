# Policy Control Plane Plan Decisions

This file records the non-negotiable policy-control-plane decisions that keep the plan from collapsing into portal UI, AI text, or domain-local config.

## PCP-001: One policy source of truth

Decision:

```text
Parent-authored policy source documents are the only parent policy source of truth.
```

Rules:

```text
Portal UI is not source truth.
AI assistant text is not source truth.
Compiled domain artifacts are not source truth.
Enforcement results are not source truth.
Domain caches are not source truth.
Reports, audit rows, and read-model rows are not source truth.
```

## PCP-002: Policy lifecycle separation

Decision:

```text
Parent intent, draft, preview, confirmed policy, compiled domain policy, delivered update, acknowledged state, active state, enforcement result, and audit event are separate artifacts.
```

No work may collapse these into one `policy active` boolean.

## PCP-003: Child-device local evaluator authority

Decision:

```text
Portal and web surfaces author and preview. Child-device and local services validate and evaluate against local evidence, schedules, timers, AI references, and capability state before enforcement handoff.
```

Web and portal must not evaluate or enforce final policy.

## PCP-004: AI cannot write policy

Decision:

```text
AI or assistant output may draft or explain proposed policy actions. AI output remains preview-only until an authorized parent confirms a typed action.
```

Required states:

```text
assistantDrafted
parentPreviewed
parentConfirmed
parentRejected
expired
manualRequired
```

## PCP-005: Schedules use explicit timezone semantics

Decision:

```text
Every schedule and time budget carries timezone, recurrence, exception, reset, grace, expiry, and clock-source semantics.
```

Required schedule concepts:

```text
timezone id
local start/end
recurrence rule
exception dates
holiday/school-night/weekend mode if used
budget window
reset rule
grace period
effective start/end
expiry
clock source
DST ambiguous/nonexistent time behavior
```

## PCP-006: Deterministic conflict precedence

Decision:

```text
Conflicting policies must resolve deterministically or produce explicit manualRequired. Silent last-write-wins is forbidden unless recorded as a conscious versioned decision.
```

Default precedence order:

```text
emergency/safety block
parent owner explicit block
time budget exceeded
active override / bonus time
parent owner explicit allow
co-parent rule if authorized
template/default rule
domain unavailable/manualRequired
unknown
```

This order may be refined, but it must stay explicit and tested.

## PCP-007: Domain compilers are deterministic adapters

Decision:

```text
Domain compilers consume parent policy source documents and produce versioned domain-specific policy artifacts. They do not own parent policy truth and cannot silently ignore unsupported capability.
```

Domains:

```text
app/game
browser/site/social/video
network/domain
tracking/location/geofence
screen/capture
AI/local evaluator context
enforcement handoff
notification/ask-parent
```

## PCP-008: Delivery and ack are per target

Decision:

```text
Policy delivery status is per household, child profile, child device, domain, and policy version.
```

No global active claim until all required target and domain ack states are known or explicitly degraded/manual-required.

## PCP-009: Ask-parent and overrides are policy mutations

Decision:

```text
Ask-parent, bonus time, temporary allow, temporary block, exception, and approval flows are typed, expiring, scoped policy mutations with audit refs.
```

Child requests cannot self-approve. Double-submit and replay cannot grant extra time.

## PCP-010: Rollback and supersede are first-class

Decision:

```text
Every confirmed policy version must have a rollback/supersede story. Rejected, expired, superseded, and partially applied states must retain audit refs.
```

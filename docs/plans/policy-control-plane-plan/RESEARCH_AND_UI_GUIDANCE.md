# Policy Control Plane Research And UI Guidance

This document tells future agents how to research, model, and write policy-control work without creating vague settings or unsafe shortcuts.

Do not write implementation code here. This is architecture guidance, platform guidance, policy model guidance, schedule guidance, UI guidance, proof guidance, and no-claim guidance.

## Primary research anchors

Use current official docs when updating this plan.

### Authorization / policy model

- NIST SP 800-162 ABAC
- OWASP Authorization Cheat Sheet
- role/action/resource/environment authorization
- capability grants
- household role matrix

### Schedule/time

- RFC 5545 recurrence concepts
- timezone identifiers
- daylight saving transitions
- ambiguous/nonexistent local time
- clock skew
- offline device time recovery

### Windows

- App Control for Business / WDAC
- AppLocker where relevant
- process/app policy feasibility
- browser/app/network enforcement limitations
- child service local timer behavior

### Android

- DevicePolicyManager
- Device Owner / Profile Owner
- managed profile
- Accessibility
- UsageStats
- VPN/DNS
- foreground/background restrictions
- notification/action delivery constraints

### Apple

- FamilyControls
- DeviceActivity
- ManagedSettings
- Screen Time authorization
- Network Extension
- entitlement/device proof limits

### Browser

- managed browser policies
- extension/native-host boundaries
- browser policy is a domain compiler/effect, not source truth

### Security/reliability

- idempotency
- replay protection
- out-of-order delivery
- audit event models
- rollback
- redaction
- offline retry

## Core product rule

Policy control plane owns parent policy truth.

It does not own:

```text
portal component styling
domain adapter execution
browser runtime blocking
app/game process blocking
network/VPN enforcement
location/geofence runtime
screen capture runtime
AI model execution
billing/subscription authority
identity/session authority
data export/delete custody
```

It does own:

```text
parent policy source document
policy version
policy lifecycle
policy schedule/time budget semantics
policy conflict precedence
parent authoring/preview contract
domain compiler contract matrix
delivery/ack/audit lifecycle
ask-parent/override lifecycle
```

## Correct policy mental model

```text
parent intent
-> draft policy
-> validation
-> dry-run preview
-> parent confirmation
-> versioned policy source document
-> domain compiler outputs
-> delivery queue
-> child/device/domain acknowledgement
-> active/degraded/rejected/partial state
-> audit trail
-> rollback/supersede path
```

## Entity model guidance

Keep these distinct:

```text
source document
draft/preview state
confirmed version
compiled artifact
delivery record
ack state
active state
audit event
rollback ref
```

Do not merge them into one stored boolean or one UI flag.

## Authorization guidance

- Parent, co-parent, child, and support/admin roles need explicit authority boundaries.
- Actor, role, resource, and environment must all be visible in the policy model.
- Capability grants are not the same as policy truth.
- Revoked or cross-household actors must fail early.

## Schedule/time budget guidance

- Timezone is explicit.
- Local times, recurrence rules, exception dates, reset rules, grace periods, and expiry all matter.
- School-night/weekend/holiday mode is allowed only when explicitly modeled.
- Clock source and device drift must be defined.
- Bonus time and overrides expire.

## Conflict precedence guidance

- Define conflict precedence explicitly.
- Prefer deterministic resolution over silent last-write-wins.
- Surface `manualRequired` when the local state cannot be trusted.
- Preserve audit refs and rollback refs for every conflict decision.

## Parent authoring UI guidance

### Required screens

```text
Policy home
Create rule
Choose child/device target
Choose app/site/category/location/screen target
Choose schedule/time budget
Choose action: allow/warn/ask-parent/time-limit/block/manual-review
Preview impact
Conflict resolution
Confirm policy
Delivery status
Ask-parent requests
Bonus time / override
Audit history
Rollback/supersede
Manual-required gaps
```

### Required UI states

```text
draft
previewLoading
previewReady
previewFailed
conflictDetected
unsupportedTarget
manualRequired
staleDevice
offlineChild
scheduleAmbiguous
scheduleInvalid
confirmationRequired
confirmed
queued
delivered
acknowledged
active
partiallyActive
rejected
rolledBack
superseded
expired
```

### UI language constraints

Use:

```text
Draft
Preview only
Confirmation required
Queued
Delivered
Acknowledge by device
Active on this device
Partially active
Manual proof required
Unsupported on this platform
```

Do not use:

```text
Blocked
Protected
Active everywhere
Saved and enforced
AI applied it
Works on all devices
Remote ready
```

unless proof exists.

## Domain compiler guidance

Domain compilers convert source policy into domain artifacts.

Compiler input:

```text
policy version
rules
targets
schedules
exceptions
time budgets
child/device scope
capability state
domain support matrix
custody/evidence requirements
```

Compiler output:

```text
compiled artifact id
source policy version
domain
target refs
supported capability state
manual-required state
delivery target
rollback ref
audit refs
no-claim flags
```

Each domain compiler must not:

```text
write source policy truth
mutate runtime directly
silently drop unsupported capability
claim enforcement
claim UI delivery
claim platform support without proof
```

## Delivery/ack guidance

Delivery is per child/device/domain.

Required delivery dimensions:

```text
policy version
target child profile
target child device
domain
delivery route
queued at
sent at
acknowledged at
applied at
rejected reason
retry count
expiry
rollback ref
audit refs
```

States:

```text
queued
delivering
delivered
acknowledged
applied
rejected
expiredBeforeDelivery
retryScheduled
offlineQueued
partialDomainApply
blockedByPermission
blockedByCapability
manualRequired
rolledBack
```

## Ask-parent / override guidance

Ask-parent is a temporary policy request path.

Child request must carry:

```text
request id
household
child profile
child device
target
requested action
reason
evidence refs
policy version
expiry
delivery route
audit ref
```

Parent response can be:

```text
grant
deny
modify
grant bonus time
grant one-time allow
block
expire
manualRequired
```

Rules:

```text
child cannot self-approve
observer cannot approve
revoked parent cannot approve
assistant cannot approve
double-submit cannot grant twice
replay cannot re-grant
override must expire
audit must show why and who approved
```

## AI assistant guidance

AI can:

```text
summarize policy
explain conflict
draft suggested rule
draft bonus-time response
draft safer template
flag manual-required risk
```

AI cannot:

```text
write confirmed policy
approve child request
grant bonus time
start enforcement
bypass preview
override parent decision
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

## Platform guidance

### Windows

Windows is likely the first runtime proof target.

Policy plan should model:

```text
parent-authored policy source
child service local evaluator
schedule/timer local state
domain compiler outputs
enforcement handoff to Windows-specific owners
offline local safety behavior
rollback
```

Windows support must stay tied to actual domain and enforcement proof.

### Android

Android policy behavior depends on:

```text
DevicePolicyManager
Device Owner / Profile Owner
managed profile
UsageStats
Accessibility
VPN/DNS
foreground/background constraints
notification/action delivery
```

Do not claim Android support from generic contracts only.

### iOS

iOS child policy behavior depends on:

```text
FamilyControls
DeviceActivity
ManagedSettings
Screen Time authorization
Network Extension where relevant
entitlement/device proof
```

Do not claim iOS child policy support without entitlement/device proof.

### Browser

Browser policy is domain effect only.

Do not let:

```text
browser tab state
extension state
managed profile state
AI browser summary
```

become source policy truth.

### Network

Network policy must distinguish:

```text
domain/category target
IP/protocol target
VPN/proxy/tunnel state
DNS/VPN capability
manual-required state
```

### Tracking/location

Tracking policy must distinguish:

```text
geofence rule
expected place schedule
location permission state
background capability
stale location
manual-required platform state
```

### Screen

Screen policy must distinguish:

```text
screen evidence ref
capture capability
AI classification confidence
deletion/custody state
manual-required state
```

## Research hint per workpack

### WP01 research hint

Questions:

```text
What is policy source truth?
What is only draft/preview/cache/read-model?
What domains currently store local policy-looking state?
What must be migrated into source truth or marked compiled/cache only?
What version/migration states are needed?
```

### WP02 research hint

Questions:

```text
Can parent understand the rule?
Can parent preview before save?
Are conflicts visible?
Are unsupported/manual-required targets visible?
Can AI draft without applying?
Can cancel avoid mutation?
```

### WP03 research hint

Questions:

```text
What compiler outputs exist per domain?
What is unsupported?
What is manual-required?
Does compiler mutate runtime?
Does output include source policy version?
Is output deterministic?
```

### WP04 research hint

Questions:

```text
What is delivered?
Who acknowledged?
Which device/domain applied?
What if offline?
What if duplicate/out-of-order?
What if permission lost?
Where is rollback?
```

### WP05 research hint

Questions:

```text
Can child request more time?
Who approves?
Can request replay?
Can double-submit grant twice?
Does override expire?
Can assistant only draft?
Is notification/audit linked?
```

### WP07 research hint

Questions:

```text
What happens at DST transition?
What if child changes timezone?
What if device clock is wrong?
How does budget reset?
How do exception/bonus/override conflict?
```

### WP08 research hint

Questions:

```text
What event families exist?
What is aggregate key?
What is idempotency key?
What is replay behavior?
What is audit event?
What is dead-letter/manual-required path?
```

## Required new docs and artifacts

Create:

```text
docs/plans/policy-control-plane-plan/POLICY_SOURCE_OF_TRUTH.md
docs/plans/policy-control-plane-plan/POLICY_LIFECYCLE.md
docs/plans/policy-control-plane-plan/SCHEDULE_TIME_BUDGET_MODEL.md
docs/plans/policy-control-plane-plan/POLICY_CONFLICT_PRECEDENCE.md
docs/plans/policy-control-plane-plan/DOMAIN_COMPILER_CONTRACTS.md
docs/plans/policy-control-plane-plan/DELIVERY_ACK_AUDIT_MODEL.md
docs/plans/policy-control-plane-plan/ASK_PARENT_OVERRIDE_MODEL.md
docs/plans/policy-control-plane-plan/PARENT_AUTHORING_UI_EXPECTATIONS.md
docs/plans/policy-control-plane-plan/RESEARCH_AND_UI_GUIDANCE.md
docs/plans/policy-control-plane-plan/PROOF_AND_TEST_INVENTORY.md
```

## Final instruction

Do not leave this plan as policy settings.

The plan must become an execution contract that tells future agents:

```text
what policy truth is
what policy truth is not
how parent authors safely
how schedules work
how conflicts resolve
how domains compile
how delivery/ack works
how ask-parent works
how AI stays preview-only
how rollback/audit works
which UI states are required
which proof closes each claim
which claims remain false
```

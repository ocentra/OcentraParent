# Policy Source Of Truth

Policy source documents are the only parent policy source of truth.

## What policy truth is

- A typed, schema-versioned, household-scoped, actor-audited policy document.
- A versioned source document that records parent intent before it becomes compiled or delivered policy.
- The parent-visible record used to decide target, schedule, exceptions, overrides, and policy versioning.

## What policy truth is not

- Portal UI state.
- AI draft text.
- Compiled domain artifacts.
- Enforcement results.
- Domain caches.
- Read-model rows, audit rows, or report rows.

## Required entity model

```text
FamilyPolicySet
PolicyVersion
PolicyRule
PolicyTarget
PolicySchedule
TimeBudget
PolicyTemplate
PolicyException
PolicyOverride
PermissionRequest
PolicyDecision
PolicyDeliveryState
PolicyAuditEvent
PolicyRollbackRef
```

## Required document shape

Every policy mutation must carry:

```text
actor
role
household
child profile or device scope
resource target
schedule/time context
version
reason
confirmation state
delivery state
ack state
audit ref
negative proof
```

## Required compatibility rules

- The document model is typed and schema-versioned.
- Version skew must be rejected or explicitly migrated.
- Duplicate truth must be rejected.
- Wrong-household and revoked-actor writes must fail.
- Policy source truth must remain separate from compiled artifacts and delivery state.

## Required states

```text
draft
previewed
confirmed
queued
delivered
acknowledged
active
partiallyActive
rejected
superseded
rolledBack
stale
expired
manualRequired
```

## Negative cases

```text
portal local state becomes source truth
AI draft writes policy
compiled browser policy mutates source truth
wrong-household actor reads or writes policy
revoked parent changes policy
schema/version mismatch accepted
policy marked active before delivery/ack
duplicate source policy accepted
```

## Proof expectation

This model is closed only when the source-of-truth proof inventory proves schema rejection, version skew handling, duplicate truth rejection, authZ rejection, and AI-preview-only behavior.

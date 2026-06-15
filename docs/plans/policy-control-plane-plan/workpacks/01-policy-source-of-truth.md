# Workpack 01: Policy Source Of Truth

Goal: define a typed, versioned, household-scoped parent policy source of truth and prove that no portal, AI, compiled artifact, or domain cache can replace it.

Owns: source document shape, policy versioning, actor/role/household scope, schema validation, migration boundary, duplicate-truth rejection, and custody boundaries for export/delete/sync.

Handoff: portal and AI may draft or preview only. Domain plans consume compiled outputs only. Data custody owns export/delete/sync custody.

## Required source entities

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

## Required behavior

- Parent-authored policy source documents are the only parent policy source of truth.
- Parent intent, draft, preview, confirmed source policy, compiled domain policy, delivered state, ack state, enforcement result, and audit event remain separate artifacts.
- Portal UI is not source truth.
- AI draft text is not source truth.
- Compiled domain artifacts are not source truth.
- Domain caches are not source truth.
- Wrong-household and revoked-actor writes must fail.
- Version skew must be rejected or explicitly migrated.

## Required proof IDs

```text
policy-source.schema-negative
policy-source.source-of-truth-matrix
policy-source.version-skew
policy-source.migration-boundary
policy-source.duplicate-truth-rejected
policy-source.domain-cache-not-truth
policy-source.portal-ui-not-truth
policy-source.ai-preview-not-write
policy-source.audit-ref-required
policy-source.authz-role-matrix
policy-source.wrong-household-rejected
policy-source.revoked-actor-rejected
policy-source.export-delete-custody
policy-source.policy-version-supersede
```

## Negative cases

```text
domain plan stores independent parent policy truth
portal local state becomes source truth
AI draft writes policy
compiled browser policy mutates source truth
wrong-household actor reads or writes policy
revoked parent changes policy
schema/version mismatch accepted
policy marked active before delivery/ack
duplicate source policy accepted
```

## Proof artifact expectations

```text
docs/proof/policy-control-plane-plan/01-source-of-truth-matrix-proof.md
docs/proof/policy-control-plane-plan/01-schema-negative-proof.md
docs/proof/policy-control-plane-plan/01-version-skew-proof.md
docs/proof/policy-control-plane-plan/01-duplicate-truth-negative-proof.md
docs/proof/policy-control-plane-plan/01-ai-preview-not-write-proof.md
docs/proof/policy-control-plane-plan/01-authz-negative-proof.md
```

## Failure

Do not let each domain invent its own parent policy truth, and do not mark a policy active before delivery, ack, and audit proof exist.

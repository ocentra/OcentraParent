# Workpack 01: Policy Source Of Truth

Goal: define a typed, versioned, household-scoped parent policy source of truth and prove that no portal, AI, compiled artifact, or domain cache can replace it.

Owns: source document shape, policy versioning, actor/role/household scope, schema validation, migration boundary, duplicate-truth rejection, and custody boundaries for export/delete/sync.

Handoff: portal and AI may draft or preview only. Domain plans consume compiled outputs only. Data custody owns export/delete/sync custody.

## Production authority boundary

The current Rust source/validator contracts are not a production source of truth:
the authority-bearing registration APIs still accept caller-supplied actor
authority, and this workpack has no non-test durable repository, active-version
query, startup recovery, or shipped registration caller. A contract test or a
portal/provisioning input therefore cannot make WP01 ready.

Production implementation must consume the owner-backed authority chain before
it can register or activate a policy source:

- Account Identity WP08 owns the canonical account/household/member/session
  authority. It is reached transitively through Cloudflare WP06.
- Cloudflare WP06 owns the durable D1-backed identity/household persistence
  and migration/binding boundary required by that authority chain.
- Device Trust WP01 owns trusted-device source-of-truth context, and Device
  Trust WP03 owns parent step-up authorization for policy-changing operations.

These are hard implementation prerequisites, not completion claims for those
owner workpacks. WP01 remains open until a durable source repository, trusted
authority resolver, startup recovery path, and shipped registration/active
version query caller exist. Caller-provided authority, provisioning input,
fixture/manual auth, debug custody, or device-lifecycle storage must not be
accepted as a substitute. Policy WP03 must consume this authoritative source
registration/query boundary before composing domain compilers.

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

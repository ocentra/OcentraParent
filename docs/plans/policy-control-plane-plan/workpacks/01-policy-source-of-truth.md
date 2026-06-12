# Workpack 01: Policy Source of Truth

Goal: define one typed source of truth for parent policy.

Expected shape:

- Policy documents are versioned, household-scoped, actor-audited, and schema-validated.
- Parent intent, compiled domain policy, enforcement result, and audit event are separate artifacts.
- Policy can be exported/deleted/synced through data custody rules.

Expected proof:

- Schema negative tests.
- Version skew and migration proof.
- Duplicate truth rejection.
- Audit references.
- Account/role authorization proof.
- Export/delete/sync custody proof for policy data.
- Domain consumer handoff proof.

Failure: domain plans storing independent parent policy truth without a control-plane contract.

## Decision Tree

| If the assignment touches...  | Required route                                            |
| ----------------------------- | --------------------------------------------------------- |
| Parent-authored policy model  | this workpack and account role matrix                     |
| Domain-specific compiled rule | owning domain plan plus policy compiler workpack          |
| Portal authoring UI           | portal-ux policy authoring workpack                       |
| Assistant-generated policy    | AI/portal assistant preview; parent confirmation required |
| Export/delete/sync            | data-custody-storage-plan                                 |
| Enforcement result            | v0-8-enforcement-control-plan                             |

## Execution Detail

Minimum context:

- `docs/expectations/policy.md`
- `docs/features/policy-schedules-approvals.md`
- `docs/architecture/policy-control-catalog-worker-prompt.md`
- `docs/plans/account-identity-family-plan/AGENTS.md`

Required model:

- Parent-authored policy source.
- Policy version.
- Target children/devices/resources.
- Schedule/exception set.
- Domain compiler outputs.
- Delivery state.
- Enforcement result.
- Audit refs.
- Actor/role/session authority.
- Conflict set and precedence.
- Retention/export/delete metadata.
- Migration/version compatibility state.

Rules:

- Parent intent is not runtime enforcement.
- Compiled domain policy is not source truth.
- AI suggestions are not policy until parent confirms a typed action.
- Domain plans may cache compiled policy but cannot become independent source truth.
- Every policy mutation has actor, role, reason, version, and audit reference.
- Policy status must distinguish draft, preview, confirmed, delivered, acknowledged, active, rejected, rolled back, and stale.

Expected tests/proof names:

- `policy-source.schema-negative`
- `policy-source.version-skew`
- `policy-source.duplicate-truth-rejected`
- `policy-source.ai-preview-not-write`
- `policy-source.audit-ref`
- `policy-source.authz-role-matrix`
- `policy-source.domain-cache-not-truth`
- `policy-source.export-delete-custody`
- `policy-source.conflict-precedence`

Proof artifact expectations:

- Source-of-truth matrix.
- Schema/contract fixture list.
- Consumer domain handoff table.
- Mutation lifecycle examples.
- Denied actor and wrong-household negative cases.

## Failure Conditions

- Do not let app/browser/network/tracking/screen plans each invent parent policy truth.
- Do not treat assistant text as policy.
- Do not mark a policy active before delivery/ack/enforcement status proves the claim.

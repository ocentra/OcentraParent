# Invite Recovery Model

## Scope

This model covers invites, recovery, deletion/export handoff, household transfer, and support-assisted recovery.

The current contract seeds in `packages/family-domain/src/setup-lifecycle.ts` are the source vocabulary for this plan:

- `SetupInvitePurpose`: `co-parent-invite`, `observer-invite`, `child-device-pairing`, `household-transfer`
- `SetupInviteState`: `pending`, `accepted`, `expired`, `revoked`
- `RecoveryKind`: `forgot-login`, `lost-parent-device`, `compromised-account`, `child-reinstall`, `household-transfer`
- `RecoveryState`: `pending-identity-proof`, `owner-approval-required`, `approved`, `completed`, `revoked`
- `RecoveryIdentityProofState`: `verified`, `pending`, `failed`
- `RecoverySupportChannel`: `self-serve`, `household-owner-assisted`, `support-assisted`
- `RecoveryDataCustodyHandoffState`: `none`, `export-delete-handoff-required`, `household-transfer-handoff-required`
- `SetupAuditEventKind`: `household-created`, `child-profile-added`, `device-paired`, `member-invited`, `member-revoked`, `recovery-approved`, `recovery-completed`

## Invite State Rules

| State | Meaning | Notes |
| --- | --- | --- |
| pending | Waiting for acceptance | Must be single-use and scoped to the target role and household. |
| accepted | Invite consumed | The invite cannot be reused. |
| expired | Invite timed out | Acceptance must fail. |
| revoked | Invite cancelled | Acceptance must fail. |

Invite rules:

- Invites are scoped to household, target role, purpose, and expiry.
- Purpose and target role must match.
- Single-use is mandatory for setup invites.
- Wrong-household and wrong-role acceptance is always rejected.

## Recovery State Rules

| State | Meaning | Notes |
| --- | --- | --- |
| pending-identity-proof | Waiting for proof | Identity proof must be explicit and typed. |
| owner-approval-required | Waiting for owner approval | Used for higher-risk recovery paths. |
| approved | Recovery authorized | Still may require data-custody handoff. |
| completed | Recovery finished | The recovery path is done. |
| revoked | Recovery cancelled | Further actions must fail. |

Recovery rules:

- `forgot-login`, `lost-parent-device`, `compromised-account`, `child-reinstall`, and `household-transfer` are separate recovery kinds.
- Recovery cannot grant access to child evidence without household authority.
- Support-assisted recovery must be audited and minimized.
- Household transfer and delete/export recovery paths must hand off to data custody rather than silently mutating family data.

## Failure Conditions

- A recovery path that bypasses owner approval when the model says approval is required is wrong.
- A support-assisted path that can read child evidence by default is wrong.
- An accepted invite that can still be reused is wrong.
- A transfer or delete/export path with no custody handoff proof is wrong.

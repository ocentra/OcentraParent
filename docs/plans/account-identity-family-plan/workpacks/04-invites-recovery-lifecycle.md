# Workpack 04: Invites Recovery Lifecycle

Goal: define parent invite, co-parent invite, child-device invite, recovery, deletion, and transfer lifecycle.

Expected shape:

- Invites are scoped to household, role, device intent, expiry, and inviter authority.
- Recovery distinguishes forgotten login, lost parent device, compromised account, child reinstall, and household transfer.
- Delete/export/recovery flows route to data custody for storage effects.
- Every recovery path has fraud/abuse controls and audit.

Expected proof:

- Expired, replayed, and revoked invite proof.
- Wrong-household and wrong-role proof.
- Recovery identity proof.
- Deletion/transfer handoff proof.

Failure: recovery flow that bypasses household owner authority or data custody obligations.

## Execution Detail

Minimum context:

- `docs/expectations/family-setup.md`
- `docs/expectations/data-custody.md`
- `packages/family-domain/src/setup-lifecycle.ts`

Required lifecycle:

- Parent owner invite.
- Co-parent invite.
- Observer invite.
- Child device invite/pairing.
- Invite expiry.
- Invite revocation.
- Account recovery.
- Lost parent device recovery.
- Compromised account recovery.
- Child reinstall recovery.
- Household transfer.
- Account deletion and export handoff.

Rules:

- Recovery cannot grant access to child evidence without household authority.
- Delete/export side effects route through data custody.
- Invites are single-purpose and scoped.
- Support recovery must be auditable and minimized.

Expected tests/proof names:

- `account-identity.invite.state-machine`
- `account-identity.invite.single-use`
- `account-identity.invite.expired-rejected`
- `account-identity.invite.revoked-rejected`
- `account-identity.invite.replayed-rejected`
- `account-identity.invite.wrong-household-rejected`
- `account-identity.invite.wrong-role-rejected`
- `account-identity.invite.co-parent-scope`
- `account-identity.invite.observer-scope`
- `account-identity.invite.child-device-pairing-scope`
- `account-identity.recovery.state-machine`
- `account-identity.recovery.forgot-login`
- `account-identity.recovery.lost-parent-device`
- `account-identity.recovery.compromised-account`
- `account-identity.recovery.child-reinstall`
- `account-identity.recovery.household-transfer`
- `account-identity.recovery.owner-approval-required`
- `account-identity.recovery.rate-limit`
- `account-identity.recovery.enumeration-resistant`
- `account-identity.recovery.delete-export-handoff`
- `account-identity.recovery.support-audited`

Proof artifact expectations:

- `04-invite-state-machine-proof.md`
- `04-invite-negative-proof.md`
- `04-recovery-state-machine-proof.md`
- `04-recovery-abuse-proof.md`
- `04-delete-export-handoff-proof.md`
- `04-support-recovery-audit-proof.md`

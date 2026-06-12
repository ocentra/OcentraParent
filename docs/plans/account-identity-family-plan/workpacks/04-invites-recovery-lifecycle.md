# Workpack 04: Invites Recovery Lifecycle

Goal: define parent invite, co-parent invite, child-device invite, recovery, deletion, and transfer lifecycle.

Expected shape:

- Invites are scoped to household, role, device intent, expiry, and inviter authority.
- Recovery distinguishes forgotten login, lost parent device, compromised account, child reinstall, and household transfer.
- Delete/export/recovery flows route to data custody for storage effects.
- Every recovery path has fraud/abuse controls and audit.

Expected proof:

- Expired/replayed/revoked invite proof.
- Wrong-household and wrong-role proof.
- Recovery identity proof.
- Deletion/transfer handoff proof.

Failure: recovery flow that bypasses household owner authority or data custody obligations.

## Execution Detail

Minimum context:

- `docs/expectations/family-setup.md`
- `docs/expectations/data-custody.md`
- `docs/plans/data-custody-storage-plan/AGENTS.md`

Required lifecycle:

- Parent owner invite.
- Co-parent invite.
- Child device invite/pairing.
- Invite expiry.
- Invite revocation.
- Account recovery.
- Lost parent device recovery.
- Household transfer.
- Account deletion.

Rules:

- Recovery cannot grant access to child evidence without household authority.
- Delete/export side effects route through data custody.
- Invites are single-purpose and scoped.
- Support recovery must be auditable and minimized.

Expected tests/proof names:

- `invite.expired-rejected`
- `invite.revoked-rejected`
- `invite.wrong-role-rejected`
- `recovery.lost-parent-device`
- `recovery.delete-export-handoff`
- `recovery.support-audited`

Proof artifact expectations:

- Invite/recovery state machine.
- Abuse/fraud notes.
- Handoff to data custody for deletion/export.

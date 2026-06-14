# Workpack 05: Device Ownership AuthZ

Goal: define authorization for parent devices, child devices, service agents, and remote-capability sessions.

Expected shape:

- Device identity is bound to household membership and can be revoked.
- Parent controller authority is separate from child service authority.
- Sensitive actions require role, device, session freshness, and action-specific capability.
- Remote access and policy changes require elevated confirmation where applicable.

Expected proof:

- AuthZ matrix by actor/action/resource.
- Privilege escalation tests.
- Stale/revoked device tests.
- Cross-family rejection tests.

Failure: any agent can act on a household because it knows a user id, device id, or local endpoint.

## Execution Detail

Minimum context:

- `docs/features/child-agent-local-service.md`
- `docs/features/remote-lan-mobile-platforms.md`
- `docs/plans/lan-plan/AGENTS.md`
- `docs/plans/remote-access-plan/AGENTS.md`
- `packages/family-domain/src/household-authority.ts`

Required authority dimensions:

- Actor identity.
- Household membership.
- Role.
- Device membership.
- Device trust state.
- Session freshness.
- Capability grant.
- Revocation status.

Expected action matrix:

- Pair child device.
- Revoke child device.
- View child status.
- Change policy.
- Start remote view.
- Start remote control.
- Export/delete data.
- Manage billing.

Expected tests/proof names:

- `account-identity.device.authority-matrix`
- `account-identity.device.parent-controller-authority`
- `account-identity.device.parent-observer-read-only`
- `account-identity.device.child-agent-authority`
- `account-identity.device.pending-device-limited`
- `account-identity.device.trusted-device-allowed`
- `account-identity.device.revoked-device-denied`
- `account-identity.device.disabled-device-denied`
- `account-identity.device.wrong-household-denied`
- `account-identity.device.stale-device-denied`
- `account-identity.device.controller-lease-required`
- `account-identity.device.controller-lease-expired`
- `account-identity.device.controller-lease-revoked`
- `account-identity.device.remote-view-requires-capability`
- `account-identity.device.remote-control-requires-capability`
- `account-identity.device.export-delete-requires-owner`
- `account-identity.device.billing-requires-parent-owner`
- `account-identity.device.audit-events-required`

Proof artifact expectations:

- `05-device-authority-matrix.md`
- `05-revoked-device-negative-proof.md`
- `05-wrong-household-negative-proof.md`
- `05-controller-lease-proof.md`
- `05-remote-capability-proof.md`
- `05-export-delete-owner-proof.md`

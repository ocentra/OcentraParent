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

- `device-authz.matrix`
- `device-authz.revoked-device-denied`
- `device-authz.wrong-household-denied`
- `device-authz.remote-requires-capability`
- `device-authz.billing-requires-parent`

Proof artifact expectations:

- Actor/action/resource matrix.
- Negative tests for every privileged action family.
- Audit event expectations.

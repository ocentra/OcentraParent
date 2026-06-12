# Workpack 02: Identity Household Role Model

Goal: define the authority model for users, households, child profiles, roles, and devices.

Expected shape:

- User identity is not the same as household membership.
- Child profile is not the same as child device.
- Device membership requires proof and can be revoked.
- Roles include parent owner, co-parent/guardian, child profile, child device agent, support/admin if any, and revoked/disabled states.
- Every cross-household read/write path is denied unless explicitly authorized.

Expected proof:

- Role matrix.
- Cross-family negative tests.
- Device transfer/revoke states.
- Audit event expectations.

Failure: policy, remote access, data export, or setup flows acting on child data without household-role-device authority.

## Execution Detail

Minimum context:

- `docs/features/family-setup-device-roles.md`
- `docs/expectations/family-setup.md`
- `docs/expectations/policy.md`
- `docs/plans/setup-install-provisioning-plan/AGENTS.md`

Required model:

- Account user.
- Household.
- Household membership.
- Parent owner.
- Co-parent/guardian.
- Child profile.
- Parent device/controller.
- Child device/agent.
- Service/admin/support actor if any.
- Revoked, disabled, pending, and invited states.

Authorization rules:

- Child profile cannot authorize device access by itself.
- Device id cannot authorize parent action by itself.
- Parent account cannot access another household by id guessing.
- Support/admin access must be separate, audited, and minimized.

Expected tests/proof names:

- `identity.role-matrix`
- `identity.cross-family-denied`
- `identity.child-profile-not-device`
- `identity.revoked-member-denied`
- `identity.support-admin-audited`

Proof artifact expectations:

- Role/action/resource matrix.
- Negative authZ tests.
- Audit event expectations.

# Workpack 02: Identity Household Role Model

Goal: define the authority model for users, households, child profiles, roles, and devices.

Expected shape:

- User identity is not the same as household membership.
- Child profile is not the same as child device.
- Device membership requires proof and can be revoked.
- Roles include parent owner, co-parent/guardian, observer, child profile, child device agent, support/admin, and revoked/disabled states.
- Every cross-household read/write path is denied unless explicitly authorized.

Expected proof:

- Role matrix.
- Cross-family negative tests.
- Observer read-only proof.
- Support-admin boundary proof.

Failure: policy, remote access, data export, or setup flows acting on child data without household-role-device authority.

## Execution Detail

Minimum context:

- `docs/features/family-setup-device-roles.md`
- `docs/expectations/family-setup.md`
- `docs/expectations/policy.md`
- `packages/family-domain/src/household-authority.ts`
- `packages/family-domain/tests/unit/household-authority.test.ts`

Required model:

- Account user.
- Household.
- Household membership.
- Parent owner.
- Co-parent/guardian.
- Observer.
- Child profile.
- Parent controller device.
- Parent observer device.
- Child device/agent.
- Support/admin actor if any.
- Revoked, disabled, pending, and invited states.

Authorization rules:

- Child profile cannot authorize device access by itself.
- Device id cannot authorize parent action by itself.
- Parent account cannot access another household by id guessing.
- Support/admin access must be separate, audited, and minimized.

Expected tests/proof names:

- `account-identity.identity.entity-model`
- `account-identity.identity.role-matrix`
- `account-identity.identity.membership-state-machine`
- `account-identity.identity.child-profile-not-device`
- `account-identity.identity.user-not-household-member`
- `account-identity.identity.parent-owner-authority`
- `account-identity.identity.co-parent-authority`
- `account-identity.identity.observer-read-only`
- `account-identity.identity.support-admin-minimized`
- `account-identity.identity.revoked-member-denied`
- `account-identity.identity.disabled-member-denied`
- `account-identity.identity.cross-family-denied`
- `account-identity.identity.audit-events-required`

Proof artifact expectations:

- `02-identity-entity-model-proof.md`
- `02-role-action-resource-matrix.md`
- `02-cross-family-negative-proof.md`
- `02-observer-read-only-proof.md`
- `02-support-admin-boundary-proof.md`

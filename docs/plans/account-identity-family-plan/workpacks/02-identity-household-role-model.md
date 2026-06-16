<!-- agent-capsule -->

> Agent Capsule
> Plan: `account-identity-family-plan`
> Doc: `WP02 Identity Household Role Model`
> Kind: assigned implementation workpack.
> Read when: selected by WORKPACK_INDEX.md or explicit assignment.
> Stop rule: do not open sibling workpacks; do not implement sessions, invites, UI, payment, policy, remote, or device-trust here.
> Proves: account/household/role/device authority model only after tests/proof pass.
> Does not prove: secure sessions, invite/recovery, trusted-device bootstrap, or setup UI readiness.
> Proof rule: before DONE, write all WP02 proof artifacts and command log.

<!-- /agent-capsule -->

# WP02 Identity Household Role Model

## Goal

Define the Ocentra authority model for users, households, memberships, child profiles, roles, support/admin actors, and devices.

## Required inputs

```text
workpacks/01-auth-provider-decision.md
RESEARCH_AND_DECISIONS.md
docs/features/family-setup-device-roles.md
docs/expectations/family-setup.md
docs/expectations/policy.md
packages/family-domain/src/household-authority.ts
packages/family-domain/tests/unit/household-authority.test.ts
```

## Target model

Required entities/states:

```text
AccountUser
Household
HouseholdMembership
HouseholdRole
ParentOwner
CoParentGuardian
Observer
ChildProfile
ParentControllerDevice
ParentObserverDevice
ChildDeviceAgent
SupportAdminActor
PendingMembership
InvitedMembership
RevokedMembership
DisabledMembership
```

Required rules:

```text
user identity is not household membership
child profile is not child device
parent account cannot read/write another household
observer is read-only
support/admin is separate, audited, and minimized
child profile cannot authorize device access
parent owner/co-parent authority is explicit by action
revoked/disabled/pending actors are denied or degraded
```

## Expected source changes

Likely paths:

```text
packages/family-domain/src/household-authority.ts
packages/family-domain/src/references.ts
packages/family-domain/src/reference-primitives.ts
packages/family-domain/tests/unit/household-authority.test.ts
packages/family-domain/package.json if exports change
```

Do not edit sibling plans.

## Required proof root

```text
output/account-identity-family-plan-proof/02-identity-household-role-model/
```

Required artifacts:

```text
00-identity-entity-model-proof.md
01-role-action-resource-matrix.md
02-membership-state-machine-proof.md
03-cross-family-negative-proof.md
04-observer-read-only-proof.md
05-support-admin-boundary-proof.md
06-audit-event-proof.md
16-validation-commands.log
```

## Acceptance criteria

- [ ] Account user, household, membership, role, child profile, and device refs are typed.
- [ ] Role/action/resource matrix exists.
- [ ] Membership state machine exists.
- [ ] Cross-family access is denied by tests/proof.
- [ ] Observer read-only behavior is tested/proven.
- [ ] Support/admin boundary is audited/minimized.
- [ ] Child profile does not imply child-device authority.
- [ ] Audit event requirements are explicit.
- [ ] Focused commands pass or blockers are recorded.
- [ ] Checklist rows updated only after proof.

## Focused commands

```bash
npm run build --workspace @ocentra-parent/family-domain
npm run test --workspace @ocentra-parent/family-domain -- household
npm run test --workspace @ocentra-parent/family-domain -- authority
npm run lint:architecture -- --files packages/family-domain
```

## Negative cases

- Wrong-household read/write is denied.
- Child profile id alone is rejected as device authority.
- Revoked, disabled, and pending members cannot perform restricted actions.
- Observer cannot perform write/control actions.
- Support/admin action requires explicit support-admin rule plus audit ref.
- Role alone cannot grant action without household, resource, and device context.

## Manual-required gaps

Session freshness, invite/recovery lifecycle, and parent trusted-device proof stay open for WP03/WP04/device-trust handoff.

## Fill before DONE

```text
Workpack id and branch:
Entity/model changes:
Touched files:
Validation commands and results:
Proof artifacts:
Known gaps/manual-required states:
No-claim boundaries:
```

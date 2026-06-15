<!-- agent-capsule -->

> Agent Capsule
> Plan: `account-identity-family-plan`
> Doc: `WP07 Parent Account Family Setup UI`
> Kind: assigned implementation workpack.
> Read when: selected by WORKPACK_INDEX.md or explicit assignment.
> Stop rule: do not create UI-only fake setup; contracts/source states must exist or blockers must be recorded.
> Proves: parent-visible setup UI state only after tests/proof pass.
> Does not prove: provider auth readiness, device trust, LAN/remote transport, or product-ready setup.
> Proof rule: before DONE, write all WP07 proof artifacts and command log.

<!-- /agent-capsule -->

# WP07 Parent Account Family Setup UI

## Goal

Define and, when assigned, implement the first-run parent account and family setup UI states over typed account/family contracts.

## Required inputs

```text
workpacks/02-identity-household-role-model.md
workpacks/03-session-token-lifecycle.md
workpacks/04-invites-recovery-lifecycle.md
workpacks/05-device-ownership-authz.md
docs/features/family-setup-device-roles.md
docs/expectations/family-setup.md
docs/expectations/portal.md
docs/expectations/platforms.md
packages/family-domain/src/**
packages/portal-domain/src/**
apps/portal/src/** selected route only
```

## Required UI states

```text
welcome/sign-in
signed-in-no-household
create-household
join-household
create-child-profile
add-child-device
pair-child-device
pending-device
trusted-device
revoked-device
stale-device
invite-co-parent
invite-observer
observer-read-only
recovery
support-access-status
account-security-settings
manual-required
```

## Required source/custody labels

```text
live local
LAN
parent cache
parent-owned storage
stale
degraded
unavailable
manual-required
```

## UI rules

```text
UI reads typed contracts/read models.
UI does not imply login equals household/device trust.
UI does not show fake child activity data.
UI distinguishes parent account, child profile, and child device.
UI labels unavailable/manual-required states visibly.
UI keeps support/admin separate from parent owner.
UI uses domain-owned route text/DOM ids where available.
```

## Expected source changes

Likely paths:

```text
packages/family-domain/src/**
packages/portal-domain/src/**
apps/portal/src/** selected setup route/components only
apps/portal/tests/** account/family setup tests
apps/portal/e2e/** account/family setup proof only if e2e is assigned
```

Do not edit setup-install-provisioning-plan unless the user explicitly assigns route-sync.

## Required proof root

```text
output/account-identity-family-plan-proof/07-parent-account-family-setup-ui/
```

Required artifacts:

```text
00-first-run-ui-state-machine.md
01-household-setup-ui-proof.md
02-device-role-ui-proof.md
03-observer-read-only-ui-proof.md
04-recovery-ui-proof.md
05-mobile-parent-child-claim-split-proof.md
06-source-custody-label-proof.md
16-validation-commands.log
```

## Acceptance criteria

- [ ] First-run UI state machine exists.
- [ ] Sign-in/no-household/create/join states are visible/tested.
- [ ] Add child profile flow is visible/tested or blocked with exact missing contract.
- [ ] Pair child device flow is visible/tested or blocked with exact missing device-trust/LAN handoff.
- [ ] Co-parent and observer invite states are visible/tested.
- [ ] Role visibility matrix is honored.
- [ ] Revoked/stale/expired/manual-required states are visible.
- [ ] Recovery and support-access status states are visible.
- [ ] Source/custody labels are visible and honest.
- [ ] UI does not imply login equals device trust.
- [ ] UI does not present hosted child activity storage as available by default.
- [ ] Focused portal/domain commands pass or blockers are recorded.

## Focused commands

```bash
npm run build --workspace @ocentra-parent/family-domain
npm run build --workspace @ocentra-parent/portal-domain
npm run test --workspace @ocentra-parent/portal -- account
npm run test --workspace @ocentra-parent/portal -- family
npm run test:e2e --workspace @ocentra-parent/portal -- account
npm run lint:architecture -- --files packages/family-domain packages/portal-domain apps/portal
```

If UI/e2e tests do not exist yet, record the exact missing test path and keep relevant rows open.

## Negative cases

- Logged-in user with no household sees no policy/payment/remote authority.
- Observer cannot see owner-only controls.
- Revoked device shows unavailable/revoked state, not trusted.
- Expired session shows reauth/manual-required state.
- Child profile exists but device is not trusted.
- Support access is visible as separate audited support state, not parent owner.

## Manual-required gaps

Real child-device pairing proof remains gated by device-trust/LAN plans. Hosted account site remains setup-install scope. Child activity evidence remains data-custody scope.

## Fill before DONE

```text
Workpack id and branch:
UI/contract changes:
Touched files:
Validation commands and results:
Proof artifacts:
Known gaps/manual-required states:
No-claim boundaries:
```

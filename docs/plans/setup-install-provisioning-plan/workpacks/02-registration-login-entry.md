<!-- agent-capsule -->

> Agent Capsule
> Plan: `setup-install-provisioning-plan`
> Doc: `WP02 Registration Login Entry`
> Kind: assigned implementation/research workpack.
> Read when: selected by WORKPACK_INDEX.md or explicit assignment.
> Stop rule: do not implement identity provider, token/session lifecycle, household membership, invites, or recovery authority here.
> Proves: public/setup route entry and account handoff only after proof artifacts exist.
> Does not prove: auth provider readiness, secure session, household authority, or setup readiness.
> Proof rule: before DONE, write all WP02 proof artifacts and command log.

<!-- /agent-capsule -->

# WP02 Registration Login Entry

## Goal

Define how the family site starts account creation/login/resume/recovery without owning account identity internals.

## Required inputs

```text
workpacks/01-family-web-info-site.md
RESEARCH_AND_DECISIONS.md
docs/plans/account-identity-family-plan/AGENTS.md
docs/plans/account-identity-family-plan/WORKPACK_INDEX.md
docs/expectations/family-setup.md
docs/expectations/portal.md
docs/features/family-setup-device-roles.md
```

## Owned scope

```text
register/login/logout/invite/resume/recovery route state map
account-identity handoff contract
unauthenticated/authenticated/no-household route labels
expired/revoked/wrong-household invite visible states
provider unavailable visible state
public-site to account-entry transition
```

## Out of scope

```text
provider selection
session/token implementation
household membership implementation
invite/recovery authority
profile/device creation
```

## Required output

```text
auth entry route map
state matrix: unauthenticated, authenticated-no-household, household-no-profile, household-profile-no-device, paired, degraded
handoff contract: data sent to account identity and data forbidden here
recovery copy/failure states
```

## Required proof root

```text
output/setup-install-provisioning-plan-proof/02-registration-login-entry/
```

Required artifacts:

```text
00-registration-route-state-proof.md
01-auth-handoff-contract-proof.md
02-invite-negative-state-proof.md
03-no-sensitive-data-before-household-proof.md
04-registration-ui-state-proof.md
05-provider-unavailable-state-proof.md
16-validation-commands.log
```

## Acceptance criteria

- [ ] Register/login/logout/invite/resume/recovery route states are defined.
- [ ] Handoff to account-identity plan is explicit.
- [ ] Account provider/session ownership remains outside this plan.
- [ ] Expired/revoked/wrong-household invite states are visible.
- [ ] Provider unavailable state is visible.
- [ ] Sensitive profile/device data is not collected before household authority.
- [ ] Recovery state is a link/handoff, not local recovery authority.
- [ ] Focused commands pass or blocker recorded.

## Focused commands

```bash
node -e "console.log('registration-login-entry-handoff')"
npm run lint:architecture -- --files docs/plans/setup-install-provisioning-plan docs/plans/account-identity-family-plan
```

If site/portal routes exist later:

```bash
npm run test --workspace @ocentra-parent/portal -- setup
npm run test:e2e --workspace @ocentra-parent/portal -- setup
```

## Negative states

- Website route owns account/session implementation.
- Registration flow creates profile/device before household authority.
- Expired/revoked invite looks like a successful join.
- Provider unavailable looks like account success.
- Handoff sends sensitive profile/device data before authority exists.

## Manual-required gaps

Provider selection and all account/session/recovery implementation remain blocked on `account-identity-family-plan` proof.

## Fill before DONE

```text
Workpack id and branch:
Route/handoff changes:
Touched files:
Validation commands and results:
Proof artifacts:
Known gaps/manual-required states:
No-claim boundaries:
```

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

## Ownership boundary

```text
setup-install-provisioning-plan owns account-entry route labels, handoff matrix, public-to-account transition, invite negative states, and provider-unavailable setup labels.
account-identity-family-plan owns provider selection, token/session lifecycle, household membership, invites, roles, and recovery authority.
```

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

## Required proof fields

The selected proof must name, at minimum:

```text
route_state
account_handoff_state
provider_session_state
unauthenticated_state
authenticated_no_household_state
household_no_profile_state
household_profile_no_device_state
invite_expired_state
invite_revoked_state
invite_wrong_household_state
provider_unavailable_state
sensitive_data_before_authority_state
recovery_authority_state
no_auth_implementation_claim
no_session_claim
no_setup_ready_claim
no_claim
```

These are proof-routing fields, not implementation code prescriptions.

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
Workpack id and branch: WP02 Registration Login Entry / codex/tracking-plan-full-continuation-a
Route/handoff changes: proved the typed registration/login/logout/invite/resume/recovery route map, the explicit account-identity handoff contract, the visible negative invite states, the no-sensitive-data-before-household boundary, the typed registration UI state map, and the provider-unavailable visible state.
Touched files: output/setup-install-provisioning-plan-proof/02-registration-login-entry/00-registration-route-state-proof.md, output/setup-install-provisioning-plan-proof/02-registration-login-entry/01-auth-handoff-contract-proof.md, output/setup-install-provisioning-plan-proof/02-registration-login-entry/02-invite-negative-state-proof.md, output/setup-install-provisioning-plan-proof/02-registration-login-entry/03-no-sensitive-data-before-household-proof.md, output/setup-install-provisioning-plan-proof/02-registration-login-entry/04-registration-ui-state-proof.md, output/setup-install-provisioning-plan-proof/02-registration-login-entry/05-provider-unavailable-state-proof.md, output/setup-install-provisioning-plan-proof/02-registration-login-entry/16-validation-commands.log, docs/plans/setup-install-provisioning-plan/CHECKLIST_INDEX.md, docs/plans/setup-install-provisioning-plan/WORKPACK_INDEX.md, docs/plans/setup-install-provisioning-plan/PLAN_STATE.md, docs/plans/setup-install-provisioning-plan/PROOF_INDEX.md, docs/plans/setup-install-provisioning-plan/workpacks/02-registration-login-entry.md, output/setup-install-provisioning-plan-proof/06-rollout-proof-and-route-gate/00-rollout-proof-pack.md, output/setup-install-provisioning-plan-proof/06-rollout-proof-and-route-gate/04-manual-required-gap-register.md
Validation commands and results: `node -e "console.log('registration-login-entry-handoff')"` PASS; `npm run lint:architecture -- --files packages/setup-domain/src/registration-entry.ts packages/setup-domain/tests/unit/registration-entry.test.ts packages/production-domain/src/family-web-route-map.ts packages/production-domain/src/family-web-route-map-read-model.ts packages/production-domain/tests/unit/family-web-route-map.test.ts docs/plans/setup-install-provisioning-plan docs/plans/account-identity-family-plan` PASS; `npm run build --workspace @ocentra-parent/setup-domain` PASS; `npm run test --workspace @ocentra-parent/setup-domain -- registration-entry` PASS (1 file, 8 tests); `npm run build --workspace @ocentra-parent/production-domain` PASS; `npm run test --workspace @ocentra-parent/production-domain -- family-web-route-map` PASS (2 files, 8 tests).
Proof artifacts: output/setup-install-provisioning-plan-proof/02-registration-login-entry/00-registration-route-state-proof.md, output/setup-install-provisioning-plan-proof/02-registration-login-entry/01-auth-handoff-contract-proof.md, output/setup-install-provisioning-plan-proof/02-registration-login-entry/02-invite-negative-state-proof.md, output/setup-install-provisioning-plan-proof/02-registration-login-entry/03-no-sensitive-data-before-household-proof.md, output/setup-install-provisioning-plan-proof/02-registration-login-entry/04-registration-ui-state-proof.md, output/setup-install-provisioning-plan-proof/02-registration-login-entry/05-provider-unavailable-state-proof.md, output/setup-install-provisioning-plan-proof/02-registration-login-entry/16-validation-commands.log
Known gaps/manual-required states: live account provider selection, session/token implementation, household membership implementation, invite/recovery authority, and rendered first-run portal proof remain owned outside WP02; this slice proves typed handoff/state contracts only.
No-claim boundaries: no live identity provider/session/household/device/profile/recovery authority claim, no public deploy claim, no installer claim, and no pairing/readiness claim.
```

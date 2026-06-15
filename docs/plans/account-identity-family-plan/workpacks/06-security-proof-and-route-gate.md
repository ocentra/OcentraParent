# Workpack 06: Security Proof And Route Gate

Goal: define the proof gate before account/identity routes can be called ready.

Expected proof:

- Auth provider decision.
- AuthN route tests.
- AuthZ matrix tests.
- Token lifecycle and replay tests.
- Rate limit and abuse tests.
- Origin/header/open-redirect tests where relevant.
- Logging redaction and alert proof.
- Route/index sync.
- Household/device ownership proof and cross-family isolation.
- Invite/recovery abuse and expiry proof.
- Provider outage and degraded-mode proof.

Failure: PR_READY without negative security proof and explicit remaining gaps.

## Decision Tree

| If readiness claim mentions... | Required proof |
| ------------------------------ | -------------- |
| Login/register                 | provider decision, authN negative tests, session creation proof |
| Household roles                | parent/co-parent/observer/child/support authZ matrix |
| Device ownership               | selected household/device binding and cross-family denial |
| Invites/recovery               | expiry, replay, brute force, revoked invite, wrong actor proof |
| Tokens/sessions                | expiry, refresh, replay, rotation, logout, revoked actor proof |
| Public site or portal handoff  | setup-install route proof and no open redirect/origin bug |

Required security states:

- `unauthenticated`
- `authenticatedNoHousehold`
- `householdMemberNoDevice`
- `observerReadOnly`
- `controllerAllowed`
- `coParentApprovalRequired`
- `revokedActor`
- `expiredSession`
- `providerUnavailable`
- `manualSupportRequired`

## Execution Detail

Minimum context:

- `TEST_PROOF_EXPECTATIONS.md`
- `PROOF_AND_TEST_INVENTORY.md`

Required proof pack:

- Auth provider decision.
- Account/household role matrix.
- Session/token lifecycle tests.
- Invite/recovery lifecycle tests.
- Device ownership authZ tests.
- Abuse/rate-limit/origin/open-redirect tests.
- Logging redaction and alert proof.

Expected tests/proof names:

- `account-identity.rollout.provider-decision-present`
- `account-identity.rollout.authn-negative-proof`
- `account-identity.rollout.authz-matrix-proof`
- `account-identity.rollout.token-replay-proof`
- `account-identity.rollout.recovery-abuse-proof`
- `account-identity.rollout.invite-abuse-proof`
- `account-identity.rollout.cross-family-denied`
- `account-identity.rollout.revoked-actor-denied`
- `account-identity.rollout.device-ownership-proof`
- `account-identity.rollout.provider-outage-state`
- `account-identity.rollout.origin-header-proof`
- `account-identity.rollout.open-redirect-proof`
- `account-identity.rollout.csrf-state-proof`
- `account-identity.rollout.logging-redaction-alert`
- `account-identity.rollout.route-sync`
- `account-identity.rollout.manual-required-gap-register`

Proof artifact expectations:

- `06-security-proof-pack.md`
- `06-authn-negative-proof.md`
- `06-authz-matrix-proof.md`
- `06-token-replay-proof.md`
- `06-recovery-abuse-proof.md`
- `06-route-sync-proof.md`
- `06-logging-redaction-proof.md`
- `06-manual-required-gap-register.md`

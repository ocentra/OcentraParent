# Workpack 06: Security Proof and Route Gate

Goal: define the proof gate before account/identity routes can be called ready.

Expected proof:

- Auth provider decision.
- AuthN route tests.
- AuthZ matrix tests.
- Token lifecycle and replay tests.
- Rate limit/abuse tests.
- Origin/header/open-redirect tests.
- Logging redaction and alert proof.
- Route/index sync.
- Household/device ownership proof and cross-family isolation.
- Invite/recovery abuse and expiry proof.
- Provider outage/degraded mode proof.

Failure: PR_READY without negative security proof and explicit remaining gaps.

## Decision Tree

| If readiness claim mentions... | Required proof                                                  |
| ------------------------------ | --------------------------------------------------------------- |
| Login/register                 | provider decision, authN negative tests, session creation proof |
| Household roles                | parent/co-parent/observer/child/support authZ matrix            |
| Device ownership               | selected household/device binding and cross-family denial       |
| Invites/recovery               | expiry, replay, brute force, revoked invite, wrong actor proof  |
| Tokens/sessions                | expiry, refresh, replay, rotation, logout, revoked actor proof  |
| Public site or portal handoff  | setup-install route proof and no open redirect/origin bug       |

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

- This plan's `TEST_PROOF_EXPECTATIONS.md`.
- `docs/agent/TEST_PROOF_DECISION_MATRIX.md` only after the workpack is selected and a global security surface is touched.

Required proof pack:

- Auth provider decision.
- Account/household role matrix.
- Session/token lifecycle tests.
- Invite/recovery lifecycle tests.
- Device ownership authZ tests.
- Abuse/rate-limit/origin/open-redirect tests.
- Logging redaction and alert proof.

Expected tests/proof names:

- `account.rollout.provider-decision-present`
- `account.rollout.authn-negative-proof`
- `account.rollout.authz-matrix-proof`
- `account.rollout.token-replay-proof`
- `account.rollout.recovery-abuse-proof`
- `account.rollout.route-sync`
- `account.rollout.cross-family-denied`
- `account.rollout.device-ownership-proof`
- `account.rollout.provider-outage-state`
- `account.rollout.logging-redaction-alert`

Failure examples:

- Green login test only.
- No wrong-household proof.
- No revoked actor proof.
- No replay/expiry proof.
- No route/index sync.
- No denied observer/control proof.
- No invite/recovery brute-force proof.
- Logs include tokens, child identifiers, or raw private data.

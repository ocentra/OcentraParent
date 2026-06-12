# Test and Proof Expectations

| Risk surface           | Expected proof                                                                                                   |
| ---------------------- | ---------------------------------------------------------------------------------------------------------------- |
| Auth provider decision | Decision record with rejected options, staged migration, data custody impact, and cost/vendor lock-in notes.     |
| Registration/login     | Success, invalid credentials, expired link, replayed token, rate limit, account lock/recovery proof.             |
| Role authorization     | Parent, co-parent, child, support/admin, revoked user, wrong household, and no-role rejection proof.             |
| Device authority       | New device, revoked device, transferred device, stale child agent, wrong household, and replay proof.            |
| Session lifecycle      | Expiry, refresh, logout, global revocation, stolen token replay, clock skew, and concurrent session proof.       |
| Abuse/security         | brute force, enumeration, CSRF/origin/header, open redirect, token fixation, logging redaction, and alert proof. |
| PR gate                | Workpack updates, route sync, proof artifacts, skipped-risk notes.                                               |

## Where tests should live

- Place identity-family tests in account/domain package tests and proof output directories once assigned workpacks land.
- Keep provider decision and session tests in the same workspace as token/session runtime boundaries for shared fixtures.
- Prefer contract tests + integration tests over mock-only provider fixtures for auth and custody transitions.

## Expected test/proof inventory

- `account-identity.auth-provider.decisions`: migration/rejection matrix for provider choice and custody impact.
- `account-identity.auth-session.replay-idempotency`: token lifecycle rejects replay, stolen tokens, and stale sessions.
- `account-identity.authz.role-boundary`: role/device/household authorization rejects cross-family and missing-role actions.
- `account-identity.recovery.rate-limit`: recovery lockout, enumeration, and rate-limit abuse paths stay negative-first.
- `account-identity.observability.audit`: logs/metrics/alerts cover auth decisions and denial reasons with redaction.

## Failure conditions

- No account/identity DONE without authN/authZ, replay protection, token lifecycle, and observability proof.
- No account/identity DONE if role/device/household boundaries are proven only by happy path.
- No account/identity DONE if abuse and recovery proof does not include negative and stale/retry cases.

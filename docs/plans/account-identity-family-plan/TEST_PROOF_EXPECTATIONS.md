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

Failure: no account/identity DONE without authN, authZ, replay, token lifecycle, rate-limit, recovery, and observability proof.

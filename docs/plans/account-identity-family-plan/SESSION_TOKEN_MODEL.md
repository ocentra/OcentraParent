# Session Token Model

## Scope

This model separates browser sessions, device credentials, invite tokens, recovery tokens, and controller leases.

## Token Families

| Token family | Purpose | Lifetime | Revocation | Must not authorize |
| --- | --- | --- | --- | --- |
| Browser session | Signed-in parent session | Short-lived with refresh | Logout, global revoke, account disable | Device trust, policy changes, billing, export/delete without additional checks |
| Device credential | Parent or child device identity | Long-lived but revocable | Device revoke, household revoke, disable | Cross-family authority or stale session authority |
| Invite token | Single-purpose invite acceptance | Expiring and single-use | Revoked, expired, accepted | Any action outside the invite purpose and target role |
| Recovery token | Regain account/control authority | Expiring and step-up gated | Revoked, completed, expired | Households or devices outside the recovery scope |
| Controller lease | Temporary parent-controller grant | Short-lived and action-specific | Expired or revoked | Any action outside the leased capability |

## Rules

- Browser session, device credential, pairing token, invite token, recovery token, and controller lease are separate artifacts.
- Privileged actions require session freshness plus household membership plus device trust plus role authority.
- Replay is denied by stateful revocation and one-time token usage where applicable.
- Clock skew is bounded and tested; stale tokens must fail safely.
- Token payloads and logs must be redacted; raw secrets never belong in proof artifacts.
- Logout and global revoke invalidate future privileged actions, even if a token string still exists somewhere.

## Required Inputs For Privileged Actions

- Actor identity
- Household membership state
- Device trust state
- Session freshness state
- Capability grant state
- Device ownership scope
- Audit requirement state
- Elevated confirmation state where the action requires it

## Failure Conditions

- A valid login token is not enough to pair, revoke, export, delete, or manage billing.
- A device credential is not a family membership proof.
- A single-use invite or recovery token that can be reused is a failure.
- Raw token values in logs or proof files are a failure.

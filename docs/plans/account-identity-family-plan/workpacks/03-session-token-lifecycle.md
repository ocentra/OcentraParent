# Workpack 03: Session Token Lifecycle

Goal: define session, token, refresh, expiry, replay, revocation, and clock-skew expectations.

Expected shape:

- Short-lived user sessions with explicit refresh behavior.
- Device/service credentials are separate from browser user sessions.
- Pairing, invite, and recovery tokens are single-purpose, scoped, expiring, and revocable.
- Logout and global revoke invalidate future privileged actions.
- Token verification produces redacted audit events.

Expected proof:

- Expiry boundary and clock-skew tests.
- Replay and duplicate-submit tests.
- Revocation and logout tests.
- Token misuse logging and alert expectations.

Failure: treating a valid login token as sufficient for device, policy, remote, or export authority.

## Execution Detail

Minimum context:

- `workpacks/01-auth-provider-decision.md`
- `packages/family-domain/src/household-authority.ts`
- `packages/family-domain/src/setup-lifecycle.ts`

Required lifecycle:

- Login/session creation.
- Session refresh.
- Logout.
- Global revocation.
- Device credential issuance.
- Invite token issuance.
- Recovery token issuance.
- Controller lease issuance.
- Expiry and clock-skew handling.

Rules:

- Browser session, device credential, pairing token, invite token, recovery token, and controller lease are separate.
- All privileged actions need session freshness and role/device authority.
- Token payloads must avoid child sensitive data.

Expected tests/proof names:

- `account-identity.session.credential-type-matrix`
- `account-identity.session.login-created`
- `account-identity.session.refresh-rotates`
- `account-identity.session.refresh-revoked-denied`
- `account-identity.session.logout-invalidates`
- `account-identity.session.global-revoke-invalidates`
- `account-identity.session.expiry-boundary`
- `account-identity.session.clock-skew`
- `account-identity.session.replay-rejected`
- `account-identity.session.stolen-token-denied`
- `account-identity.session.device-token-not-user-token`
- `account-identity.session.invite-token-not-session`
- `account-identity.session.recovery-token-not-session`
- `account-identity.session.remote-grant-not-user-session`
- `account-identity.session.freshness-required-for-sensitive-action`
- `account-identity.session.redacted-audit-log`

Proof artifact expectations:

- `03-credential-type-matrix.md`
- `03-token-expiry-replay-proof.md`
- `03-refresh-revocation-proof.md`
- `03-session-freshness-proof.md`
- `03-token-redaction-proof.md`

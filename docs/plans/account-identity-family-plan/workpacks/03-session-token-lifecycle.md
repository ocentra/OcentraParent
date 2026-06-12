# Workpack 03: Session Token Lifecycle

Goal: define session, token, refresh, expiry, replay, revocation, and clock-skew expectations.

Expected shape:

- Short-lived user sessions with explicit refresh behavior.
- Device/service credentials are separate from browser user sessions.
- Pairing/invite tokens are single-purpose, scoped, expiring, and revocable.
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

- `docs/plans/account-identity-family-plan/workpacks/01-auth-provider-decision.md`
- `docs/plans/setup-install-provisioning-plan/workpacks/02-registration-login-entry.md`
- `docs/plans/remote-access-plan/AGENTS.md` only for remote-session authority.

Required lifecycle:

- Login/session creation.
- Session refresh.
- Logout.
- Global revocation.
- Device credential issuance.
- Invite/pairing token issuance.
- Recovery token issuance.
- Expiry and clock-skew handling.

Rules:

- Browser session, device credential, pairing token, invite token, and remote session grant are separate.
- All privileged actions need session freshness and role/device authority.
- Token payloads must avoid child sensitive data.

Expected tests/proof names:

- `session.expiry-boundary`
- `session.refresh-revoked`
- `session.logout-invalidates`
- `session.replay-rejected`
- `session.clock-skew`
- `session.device-token-not-user-token`

Proof artifact expectations:

- Token lifecycle matrix.
- Negative replay/expiry logs with redaction.
- Alert/metric expectations for repeated failures.

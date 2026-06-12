# Workpack 04: Session Consent Grants

Goal: define consent, disclosure, grants, expiry, and revocation for remote access.

Expected shape:

- Parent authority is necessary but not always sufficient; sensitive modes need additional confirmation and child-visible disclosure where applicable.
- Grants expire and can be revoked by parent, policy, device state, account state, or platform capability loss.
- Every denial includes user-visible reason and audit state.

Expected proof:

- Grant expiry/revoke tests.
- Wrong actor/wrong household/wrong device tests.
- Child disclosure artifact where applicable.
- Audit trail proof.
- Reconnect and crash recovery proof.
- Emergency/parent-support misuse boundary proof.
- Consent state visible in portal/desktop and on child device where applicable.

Failure: persistent remote access grant with no expiry, revocation, or disclosure.

## Decision Tree

| If session mode is...    | Required grant proof                                                       |
| ------------------------ | -------------------------------------------------------------------------- |
| Remote live view         | parent role, selected device, session expiry, disclosure, custody state    |
| Remote input/control     | stronger confirmation, platform authority, revocation, child-visible state |
| Support/admin assistance | explicit parent grant, redacted diagnostics, no hidden access              |
| LAN-local only           | LAN proof; no remote relay claim                                           |
| Relay/cloud route        | relay security workpack and account/device authority proof                 |

## Execution Detail

Minimum context:

- `docs/plans/account-identity-family-plan/workpacks/03-session-token-lifecycle.md`
- `docs/plans/policy-control-plane-plan/AGENTS.md`
- `docs/expectations/portal.md`

Required lifecycle:

- Requested.
- Parent confirmed.
- Child/device notified or disclosed where applicable.
- Active.
- Paused.
- Stopped.
- Expired.
- Revoked.
- Denied.
- Failed/degraded.
- Reconnect pending.
- Superseded by newer grant.
- Killed by account/device/policy/capability loss.

Rules:

- Grants must be scoped to action and device.
- Grants must expire.
- Revocation wins over reconnect.
- Policy and permission loss can force stop.
- Observer/co-parent roles require explicit authZ; account membership alone is not control authority.
- No support/admin access exists without parent-visible grant and audit.

Expected tests/proof names:

- `remote-grant.expiry`
- `remote-grant.revocation-wins`
- `remote-grant.permission-loss-stop`
- `remote-grant.child-disclosure`
- `remote-grant.audit`
- `remote-grant.reconnect-after-revoke-denied`
- `remote-grant.wrong-household-device-denied`
- `remote-grant.support-access-visible`

Proof artifact expectations:

- Session/grant state machine.
- UI artifacts for approval/active/stopped/denied.
- Redacted audit event examples.
- Explicit expiry duration, revocation trigger, actor role, selected device, and route type.

## Failure Conditions

- Do not create persistent remote access by default.
- Do not claim child disclosure where the child platform/UI artifact is missing.
- Do not allow reconnect to resurrect a revoked or expired session.

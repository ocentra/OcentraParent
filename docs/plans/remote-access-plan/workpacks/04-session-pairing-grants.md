# Workpack 04: Pairing And Standing Access Grants

Goal: define pairing, disclosure, standing access, revoke/remove-device, and audit for remote access.

Expected shape:

- Parent authority is necessary for pairing and standing access.
- Grants are scoped by household, child device, route, and capability.
- Standing access remains until revoked or device removed.
- Every denial includes user-visible reason and audit state.

Expected proof:

- Pairing/grant persistence tests.
- Wrong actor/wrong household/wrong device tests.
- Child disclosure artifact where applicable.
- Audit trail proof.
- Reconnect and crash recovery proof.
- Parent-support misuse boundary proof.
- Access state visible in portal/desktop and on child device where applicable.

Failure: persistent remote access grant with no revoke/remove-device path.

## Decision Tree

| If session mode is...    | Required grant proof                                                     |
| ------------------------ | ------------------------------------------------------------------------ |
| Remote live view         | paired access, selected device, state, disclosure, custody state        |
| Deferred remote control  | stronger confirmation, platform authority, revocation, child-visible state |
| Support/admin assistance | explicit parent grant, redacted diagnostics, no hidden access            |
| LAN-local only          | LAN proof; no remote relay claim                                          |
| Relay/cloud route       | relay security workpack and account/device authority proof                |

## Execution Detail

Minimum context:

- `docs/plans/account-identity-family-plan/workpacks/03-session-token-lifecycle.md`
- `docs/plans/portal-ux-household-surfaces-plan/AGENTS.md`
- `docs/plans/account-identity-family-plan/AGENTS.md`

Required lifecycle:

- Requested.
- Parent confirmed.
- Child/device notified or disclosed where applicable.
- Paired.
- Active.
- Paused.
- Stopped.
- Revoked.
- Removed.
- Denied.
- Failed/degraded.
- Reconnect pending.
- Superseded by newer grant.
- Killed by account/device/policy/capability loss.

Rules:

- Grants must be scoped to action and device.
- Standing access is the default after pairing.
- Revocation and device removal win over reconnect.
- Policy and permission loss can force stop.
- Observer/co-parent roles require explicit authZ; account membership alone is not control authority.
- No support/admin access exists without parent-visible grant and audit.
- No repeated permission prompts are part of the current pass.

Expected tests/proof names:

- `remote-grant.paired-access`
- `remote-grant.revocation-wins`
- `remote-grant.remove-device-wins`
- `remote-grant.child-disclosure`
- `remote-grant.audit`
- `remote-grant.reconnect-after-revoke-denied`
- `remote-grant.wrong-household-device-denied`
- `remote-grant.support-access-visible`

Proof artifact expectations:

- Session/pairing state machine.
- UI artifacts for paired/active/stopped/denied.
- Redacted audit event examples.
- Explicit pairing target, revocation/removal trigger, actor role, selected device, and route type.

## Failure Conditions

- Do not create persistent remote access without pairing and revoke/remove-device paths.
- Do not claim child disclosure where the child platform/UI artifact is missing.
- Do not allow reconnect to resurrect a revoked or removed session.

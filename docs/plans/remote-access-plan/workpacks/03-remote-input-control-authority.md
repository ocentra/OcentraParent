# Workpack 03: Remote Input Control Authority

Goal: define remote input/control as a separate high-risk capability deferred from the current live-view pass.

Expected shape:

- Remote input is off by default.
- Parent confirmation and session freshness are required when this workpack is opened.
- Input scope, stop/escape behavior, blocked surfaces, and child disclosure are explicit.
- Control cannot bypass policy, OS permission, or child safety constraints.

Expected proof:

- Privilege escalation negative tests.
- Stop/revoke proof.
- Replay/out-of-order input proof.
- Unsupported platform/manual-required proof.

Failure: remote desktop/control hidden inside live view or LAN transport work.

## Execution Detail

This workpack is not part of the current live-view pass.

Minimum context:

- `docs/architecture/rustdesk_remote_capabilities_first_pass.md`
- `docs/roadmaps/roadmap-v2-parent-owned-remote-access-cloud-relay.md`
- `docs/expectations/tamper-uninstall-protection.md`
- `docs/plans/account-identity-family-plan/workpacks/05-device-ownership-authz.md`

Required model:

- Remote input is separate from remote view.
- Control scope: keyboard, pointer, app focus, admin surfaces, blocked surfaces.
- Stop path: parent stop, child stop where applicable, permission loss, policy stop, timeout, account revoke.
- High-risk actions require fresh confirmation and audit.

Expected tests/proof names:

- `remote-control.requires-fresh-confirmation`
- `remote-control.stop-path`
- `remote-control.revoked-session-blocked`
- `remote-control.blocked-surface`
- `remote-control.replay-input-rejected`

Proof artifact expectations:

- Input authority matrix.
- Abuse/privilege escalation tests.
- Child disclosure artifact.
- Logs/traces with no sensitive screen payload.

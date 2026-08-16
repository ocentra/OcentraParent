# Workpack 03: Remote Input Control Authority

Goal: define remote input/control as a separate high-risk capability deferred from the current live-view pass.

## Ownership boundary

```text
WP03 is a future control slice and is not part of the current live-view pass.
remote-access-plan will own remote-control capability semantics only when this workpack is explicitly opened.
account/device-trust plans own parent authority and fresh confirmation/step-up.
enforcement/platform plans own action authority, platform permission, stop paths, and adapter execution.
screen/portal plans may provide visible context, but not control authority.
```

## Expected shape

- Remote input is off by default.
- Parent confirmation and session freshness are required when this workpack is opened.
- Input scope, stop/escape behavior, blocked surfaces, and child disclosure are explicit.
- Control cannot bypass policy, OS permission, or child safety constraints.

## Required proof fields for future control slice

When this deferred workpack is explicitly opened, the selected proof must name, at minimum:

```text
control_capability_state
fresh_confirmation_state
parent_authority_state
session_freshness_state
input_scope_state
blocked_surface_state
child_disclosure_state
stop_escape_state
revocation_state
policy_constraint_state
platform_permission_state
replay_input_state
privilege_escalation_state
manual_required_state
no_live_view_claim
no_claim
```

These are proof-routing fields, not implementation code prescriptions.

## Expected proof

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

## Current-pass no-claim

Current live-view work must record `control_state: deferred` or equivalent no-claim. No current-pass READY claim may imply keyboard, pointer, app focus, admin, filesystem, shell, or child-device control.

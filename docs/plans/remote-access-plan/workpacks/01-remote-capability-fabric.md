# Workpack 01: Remote Capability Fabric

Goal: define the remote session and capability model.

Expected shape:

- Capability grants are scoped by household, child device, parent actor, action type, expiry, consent state, and revocation state.
- Live view and remote control are separate capabilities.
- Session state includes requested, authorized, connecting, active, degraded, stopped, expired, revoked, denied, and failed.
- Every session has audit references and redacted diagnostics.

Expected proof:

- Capability schema/contract proof when implemented.
- AuthZ matrix.
- Replay/stale/revoked session negative proof.
- Route handoffs to account, screen, LAN, and data custody.

Failure: one generic remote flag that authorizes unrelated remote actions.

## Execution Detail

Minimum context:

- `docs/architecture/remote-capability-fabric-v2-plan.md`
- `docs/architecture/rustdesk_remote_capabilities_first_pass.md`
- `docs/roadmaps/roadmap-v2-parent-owned-remote-access-cloud-relay.md`
- `docs/plans/account-identity-family-plan/AGENTS.md`

Required model:

- Remote capability type: live view, screenshot request, file/log diagnostic if allowed, remote input/control.
- Actor: parent owner, co-parent, support/admin if any, child agent.
- Resource: household, child profile, child device, session, relay.
- Grant: requested, approved, active, expired, revoked, denied, failed.
- Audit: who requested, who approved, what capability, when, why, and proof refs.

Agent decision tree:

- If capture adapter is the task, route to `screen-plan`.
- If transport is LAN-only, route to `lan-plan`.
- If identity/role/device authority is unclear, route to `account-identity-family-plan`.
- If the task is session/capability/grant semantics, stay here.

Expected tests/proof names:

- `remote-capability.grant-scope`
- `remote-capability.live-view-not-control`
- `remote-capability.revoked-grant-denied`
- `remote-capability.wrong-household-denied`
- `remote-capability.audit-complete`

Proof artifact expectations:

- Capability matrix.
- Session state transition table.
- Negative authZ proof.
- Adjacent-plan handoff list.

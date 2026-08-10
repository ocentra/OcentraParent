# Workpack 01: Remote Capability Fabric

Goal: define the paired remote session and capability model.

## Ownership boundary

```text
remote-access-plan owns capability type, grant state, session state, route model, standing-access semantics, and no-claim boundary.
account-identity-family-plan owns account/household/role/session/device authority.
device-trust-bootstrap-plan owns parent presence and trusted-device step-up.
screen-plan owns capture primitives, not remote authority.
lan-plan owns LAN-only transport, not relay-backed remote access.
```

## Expected shape

- Capability grants are scoped by household, child device, parent actor, action type, pairing state, and revocation/remove-device state.
- Live view and remote control are separate capabilities, but only live view is current-pass.
- Session state includes requested, authorized, paired, connecting, active, degraded, stopped, removed, revoked, denied, and failed.
- Every session has audit references and redacted diagnostics.

## Required proof fields

The selected proof must name, at minimum:

```text
capability_type
actor_role
household_ref
child_device_ref
parent_actor_ref
pairing_state
grant_state
standing_access_state
revocation_state
removed_device_state
session_state
replay_state
stale_session_state
control_claim_state
audit_ref
redacted_diagnostics_state
adjacent_handoff_state
no_claim
```

These are proof-routing fields, not implementation code prescriptions.

## Expected proof

- Capability schema/contract proof when implemented.
- AuthZ matrix.
- Replay/stale/revoked/removed session negative proof.
- Route handoffs to account, screen, LAN, and data custody.

Failure: one generic remote flag that authorizes unrelated remote actions or hides pairing state.

## Execution Detail

Minimum context:

- `docs/architecture/remote-capability-fabric-v2-plan.md`
- `docs/architecture/rustdesk_remote_capabilities_first_pass.md`
- `docs/roadmaps/roadmap-v2-parent-owned-remote-access-cloud-relay.md`
- `docs/plans/account-identity-family-plan/AGENTS.md`

Required model:

- Remote capability type: live view, screenshot request, file/log diagnostic if allowed, deferred remote input/control.
- Actor: parent owner, co-parent, support/admin if any, child agent.
- Resource: household, child profile, child device, pair, session, relay.
- Grant: requested, paired, active, removed, revoked, denied, failed.
- Audit: who requested, who paired, what capability, when, why, and proof refs.

Agent decision tree:

- If capture adapter is the task, route to `screen-plan`.
- If transport is LAN-only, route to `lan-plan`.
- If identity/role/device authority is unclear, route to `account-identity-family-plan`.
- If the task is session/capability/grant semantics, stay here.

Expected tests/proof names:

- `remote-capability.paired-access`
- `remote-capability.live-view-not-control`
- `remote-capability.revoked-grant-denied`
- `remote-capability.removed-device-denied`
- `remote-capability.wrong-household-denied`
- `remote-capability.audit-complete`

Proof artifact expectations:

- Capability matrix.
- Session state transition table.
- Negative authZ proof.
- Adjacent-plan handoff list.

## Failure conditions

- Do not use one generic remote flag for live view, screenshot, diagnostic, and control.
- Do not claim live view if pairing, standing access, revocation, and remove-device states are absent.
- Do not claim control from this current-pass capability proof.

## 2026-07-31 Rust contract packet

`crates/schema/src/remote_capability_fabric.rs` now owns the narrow
view-only grant boundary for this workpack. It uses separate capability,
pairing, grant, session, actor-role, and device-trust states; it therefore
cannot turn a generic remote flag into unrelated authority.

The focused contract proof is at
`output/remote-access-plan-proof/01-remote-capability-fabric/`. It proves a
paired, trusted parent live-view grant and rejects deferred control,
cross-household access, support/admin access, unpaired access, missing device
trust, revoked grants, and removed devices.

This packet does not claim a relay, screen stream, capture permission,
standing-access persistence, custody/retention, portal disclosure, or remote
input/control readiness. Those remain owned by the later remote workpacks and
their adjacent plan handoffs.

## Live validation update (2026-08-09)

The narrow Rust-owned capability contract and focused negative tests were
replayed on the consolidated E: branch. Authorization now checks the exact
supported schema version, the authenticated requesting parent actor against
`parent_actor_ref`, the requested child device against `child_device_ref`, and
a nonblank audit reference before allowing live view.
The durable validation manifest is
`docs/proof/remote-access-plan/slice-01-capability-fabric.md`; local reproducible
output is under
`output/remote-access-plan-proof/01-remote-capability-fabric/`.

The graph records this slice as `validation`, not `done`. Pairing workflow,
standing-access persistence, relay/session runtime, device-trust handoff,
revoke/remove runtime behavior, custody, portal disclosure, abuse controls,
CI, review, and merge remain open.

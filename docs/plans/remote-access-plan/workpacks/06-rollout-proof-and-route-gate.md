# Workpack 06: Rollout Proof And Route Gate

Goal: define the gate for remote access readiness.

## Ownership boundary

```text
WP06 aggregates remote-access-plan proof roots only.
Screen, LAN, account, device-trust, data-custody, portal, eventing, and protocol owners remain separate unless their handoff proof is explicitly accepted.
Remote control WP03 is deferred and cannot be used to claim current live-view readiness.
```

## Expected proof

- Paired capability/session model.
- Live view proof if touched.
- Deferred control proof if touched only by explicit future WP03 assignment.
- AuthZ/replay/revocation proof.
- Relay unavailable/degraded proof.
- Privacy/custody proof.
- Route/index sync.

Failure: PR_READY from local capture, local LAN, or UI-only proof.

## Execution Detail

Required proof pack:

- Paired capability/session model.
- Account/household/device authZ proof.
- Live view proof if touched.
- Deferred control proof if explicitly opened; otherwise a no-control no-claim.
- Pairing/disclosure proof.
- Relay failure/abuse proof.
- Privacy/retention proof.
- Route/index sync.

## Required rollout fields

The selected rollout proof must name, at minimum:

```text
rollout_gate_id
accepted_proof_roots
missing_proof_roots
carried_blockers
capability_model_state
pairing_grant_state
standing_access_state
revocation_remove_device_state
live_view_state
relay_state
relay_abuse_state
custody_retention_state
child_disclosure_state
account_authority_state
device_trust_state
portal_state
control_state
claims_allowed
claims_blocked
manual_required_gaps
no_claim
```

These are proof-routing fields, not implementation code prescriptions.

Expected tests/proof names:

- `remote.rollout.capability-model`
- `remote.rollout.authz-negative-proof`
- `remote.rollout.relay-failure-proof`
- `remote.rollout.privacy-proof`
- `remote.rollout.no-overclaim`

Failure examples:

- Only local screenshot proof.
- Only LAN pairing proof.
- No revoked-session negative test.
- No child disclosure/degraded state proof.
- Live-view proof used as remote control proof.
- Relay availability used as retention permission.

## Research Gate

This rollout gate cannot be closed from stale legacy docs. The assigned agent must inspect remote architecture docs, screen capture code/docs, LAN transport code/docs, portal remote routes, local service capabilities, and RustDesk comparison notes. Remote view, deferred control, relay retention, child disclosure, and pairing decisions must be discussed with Sujan before implementation claims.

## Required Route Updates

- Remote live view routes here before `screen-plan` or `lan-plan`.
- `screen-plan` may prove capture primitives only.
- `lan-plan` may prove local transport only.
- `account-identity-family-plan` must prove actor/device/session authority before remote access is considered safe.
- `device-trust-bootstrap-plan` must prove parent presence or step-up where selected.
- `data-custody-storage-plan` must prove retention/export/delete rules before any remote artifact retention claim.

## Minimum DONE Report

The report must name:

- capability type.
- actor/household/device authority.
- paired grant state.
- standing access state.
- revoke/remove-device state.
- disclosure state.
- relay path and failure mode.
- retention/custody state.
- abuse/rate-limit proof.
- explicit unsupported/manual-required platforms.
- explicit remote-control deferred no-claim for the current pass.

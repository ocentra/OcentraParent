# Workpack 06: Rollout Proof and Route Gate

Goal: define the gate for remote access readiness.

Expected proof:

- Capability/session model.
- Live view proof if touched.
- Remote control proof if touched.
- AuthZ/replay/revocation proof.
- Relay unavailable/degraded proof.
- Privacy/custody proof.
- Route/index sync.

Failure: PR_READY from local capture, local LAN, or UI-only proof.

## Execution Detail

Required proof pack:

- Capability/session model.
- Account/household/device authZ proof.
- Live view proof if touched.
- Remote control proof if touched.
- Consent/disclosure proof.
- Relay failure/abuse proof.
- Privacy/retention proof.
- Route/index sync.

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

## Research Gate

This rollout gate cannot be closed from first-pass docs. The assigned agent must inspect remote architecture docs, screen capture code/docs, LAN transport code/docs, portal remote routes, local service capabilities, and RustDesk comparison notes. Remote view, remote control, relay retention, child disclosure, and consent decisions must be discussed with Sujan before implementation claims.

## Required Route Updates

- Remote screen/live view and remote desktop/control route here before `screen-plan` or `lan-plan`.
- `screen-plan` may prove capture primitives only.
- `lan-plan` may prove local transport only.
- `account-identity-family-plan` must prove actor/device/session authority before remote access is considered safe.

## Minimum DONE Report

The report must name:

- capability type.
- actor/household/device authority.
- session grant state.
- consent/disclosure state.
- relay path and failure mode.
- retention/custody state.
- abuse/rate-limit proof.
- explicit unsupported/manual-required platforms.

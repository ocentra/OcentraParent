# 18 Signed Child Hello And Heartbeat

Sources: [20-step plan](../v0-9-lan-discovery-20-step-plan.md),
[test blueprint](../v0-9-lan-discovery-test-blueprint.md),
[UI/UX guide](../ui-ux-requirements-guide.md), and
[folder README](../README.md).

## Where We Are

Current V0.9 proof includes local/LAN route and command validation, but physical
household readiness still lacks a signed LAN child hello and heartbeat from a
second child-agent device.

Branch `codex/v0-9-lan-signed-discovery-relay-spine` adds signed discovery
spine rows and rejection states for the branch proof harness. That is a
verified protocol/service proof layer for signed discovery readiness, but it
does not replace a real paired child-agent hello and heartbeat from a second
physical device.

## Where We Want To Be

After pairing, the child agent connects outward to the parent when possible and
sends a signed hello with protocol version, device id, install id, family hash,
optional child profile hash, platform, hostname, agent version, local IPs, MACs
where available, capabilities, timestamp, nonce, and signature. Heartbeats
update last confirmed, online, stale, and offline state.

## Requirement Checklist

- [x] Current branch records signed-discovery proof rows and rejection outcomes
      for invalid, revoked, wrong-target, unavailable, and manual-required
      states in typed contracts and Rust protocol/service parity.
- [x] Current branch keeps physical second-device household proof
      manual-required instead of claiming CI can prove it.
- [x] Portal LAN selected-device details and Activity/Network diagnostics now
      show signed-hello/signed-heartbeat manual-required labels from the typed
      read model.
- [ ] Reject unsigned, invalid-signature, expired, replayed, wrong-family,
      wrong-device, unpaired, and wrong-protocol hello payloads.
- [ ] Accept unknown future capability only when the contract says it is safe.
- [ ] Record child-agent capabilities without mixing in browser/app/screen-time
      control claims.
- [ ] Keep Android and iOS MAC/background limits explicit and manual-required
      until real device proof exists.
- [ ] Transition online to stale to offline without deleting the device record.

## Acceptance And Proof

- Child hello tests cover valid signed hello, missing device id, missing nonce,
  invalid signature, wrong family, expired timestamp, replay, unknown future
  version, and unknown capability.
- Heartbeat explicit-timestamp tests cover valid heartbeat, wrong signature, timeout,
  late heartbeat recovery, stale, and offline transitions.
- Manual proof captures second physical child-agent hello and heartbeat before
  production household LAN readiness is claimed.

## Parallel Ownership Notes

This is security-sensitive. Pairing keys, route checks, nonce stores, and
heartbeat state must stay shared with assignment/revocation work.

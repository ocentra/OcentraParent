# 18 Signed Child Hello And Heartbeat

Sources: [20-step plan](../v0-9-lan-discovery-20-step-plan.md),
[test blueprint](../v0-9-lan-discovery-test-blueprint.md),
[UI/UX guide](../ui-ux-requirements-guide.md), and
[folder README](../README.md).

## Where We Are

Current V0.9 proof includes local/LAN route and command validation, but physical
household readiness still lacks a signed LAN child hello and heartbeat from a
second child-agent device.

## Where We Want To Be

After pairing, the child agent connects outward to the parent when possible and
sends a signed hello with protocol version, device id, install id, family hash,
optional child profile hash, platform, hostname, agent version, local IPs, MACs
where available, capabilities, timestamp, nonce, and signature. Heartbeats
update last confirmed, online, stale, and offline state.

## Requirement Checklist

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
- Heartbeat fake-clock tests cover valid heartbeat, wrong signature, timeout,
  late heartbeat recovery, stale, and offline transitions.
- Manual proof captures second physical child-agent hello and heartbeat before
  production household LAN readiness is claimed.

## Parallel Ownership Notes

This is security-sensitive. Pairing keys, route checks, nonce stores, and
heartbeat state must stay shared with assignment/revocation work.

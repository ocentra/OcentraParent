# 17 Parent And Child mDNS Advertisements

Sources: [20-step plan](../v0-9-lan-discovery-20-step-plan.md),
[test blueprint](../v0-9-lan-discovery-test-blueprint.md),
[UI/UX guide](../ui-ux-requirements-guide.md), and
[folder README](../README.md).

## Where We Are

The current service can run loopback/LAN command paths, but production parent
and child mDNS advertisement is not complete. Agent presence cannot be inferred
from generic hostnames or LAN rows.

## Where We Want To Be

Parent advertises `_ocentra-parent._tcp.local` with protocol version, family
hash, and pairing state. Child advertises `_ocentra-agent._tcp.local` with
protocol version, opaque device id, platform, agent version, and paired state.
TXT records do not leak child names, email, raw policy, or sensitive profile
data.

## Requirement Checklist

- [ ] Define parent advertisement contract and Rust protocol parity.
- [ ] Define child advertisement contract and Rust protocol parity.
- [ ] Use opaque ids and hashes only in broadcast metadata.
- [ ] Treat advertisements as discovery hints, not confirmation.
- [ ] Add lifecycle behavior for advertise start, update, stop, and degraded
      platform support.

## Acceptance And Proof

- Contract and service tests cover valid advertisement, missing required fields,
  unsupported platform, paired/unpaired state, and sanitized TXT values.
- Security tests prove spoofed `_ocentra-agent` announcements do not confirm a
  device without signed hello.
- Platform docs keep Android/iOS background and Bonjour limits explicit.

## Parallel Ownership Notes

This can run in parallel with signed hello design, but the two workpacks must
share ids, family hash, protocol version, and capability constants.

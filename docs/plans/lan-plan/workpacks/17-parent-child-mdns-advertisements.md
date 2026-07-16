# 17 Parent And Child mDNS Advertisements

<!-- agent-capsule -->

> Agent Capsule
> Plan: `lan-plan`
> Doc: `17 Parent And Child mDNS Advertisements`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

Sources: [20-step plan](../v0-9-lan-discovery-20-step-plan.md),
[test blueprint](../v0-9-lan-discovery-test-blueprint.md),
[UI/UX guide](../ui-ux-requirements-guide.md), and
[folder README](../README.md).

## Where We Are

The local Rust slice is complete in the owning `lan-core` and `agent-service`
files: parent and child advertisement contracts exist, packet encoding exists,
runtime lifecycle sync and goodbye handling exist, and status surfaces keep the
result hint-only. Agent presence still cannot be inferred from generic
hostnames, LAN rows, or unsigned `_ocentra-agent` announcements.
Signed-child confirmation remains intentionally out of scope for this workpack
and stays owned by the signed hello or heartbeat path in Workpack `18`.

## Where We Want To Be

Parent advertises `_ocentra-parent._tcp.local` with protocol version, family
hash, and pairing state. Child advertises `_ocentra-agent._tcp.local` with
protocol version, opaque device id, platform, agent version, and paired state.
TXT records do not leak child names, email, raw policy, or sensitive profile
data.

## Requirement Checklist

- [x] Define parent advertisement contract and Rust protocol parity.
- [x] Define child advertisement contract and Rust protocol parity.
- [x] Use opaque ids and hashes only in broadcast metadata.
- [x] Treat advertisements as discovery hints, not confirmation.
- [x] Add lifecycle behavior for advertise start, update, stop, and degraded
      platform support.

## Acceptance And Proof

- Local Rust validation covers valid advertisement construction, missing
  required fields, hashed and opaque TXT metadata, sanitized TXT values, packet
  encoding, and lifecycle start/update/stop/degraded behavior.
- Contract and runtime status surfaces keep advertisements hint-only; spoofed
  or unsigned `_ocentra-agent` announcements do not confirm a child without the
  signed hello path owned elsewhere.
- Android/iOS background multicast behavior and macOS physical/background
  Bonjour behavior remain the only W17 blocker and stay manual-required outside
  this single-machine proof environment.

## Parallel Ownership Notes

This can run in parallel with signed hello design, but the two workpacks must
share ids, family hash, protocol version, and capability constants.

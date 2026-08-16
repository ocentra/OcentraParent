<!-- agent-capsule -->

> Agent Capsule
> Doc: V0.9 Household LAN Production Discovery Proof Boundary - 2026-05-29
> Kind: historical checkpoint/proof documentation; read only when CHECKPOINT_INDEX or PROOF_INDEX names it.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

# V0.9 Household LAN Production Discovery Proof Boundary - 2026-05-29

Branch: `codex/v0-9-household-lan-production-discovery-proof`

## Scope

This checkpoint hardens the V0.9 household LAN proof boundary after PR #143.
It keeps local real-service proof separate from physical household LAN proof.

The focused proof command verifies:

- explicit production discovery labels for discovered, pending, paired,
  revoked, stale, offline, and unavailable/manual-gated states,
- paired and failed-unpaired route behavior from local Rust service proof,
- wrong-origin, wrong-device, replay, stale, revoked, observer-read-only,
  missing-lease, expired-lease, and wrong-controller rejection evidence,
- selected-device stale/offline readiness through Rust service and registry
  proof assertions,
- a claim-upgrade verifier that refuses product-ready household LAN readiness
  without physical two-device, router, firewall, origin, stale/offline,
  failed-unpaired, and provider artifacts.

## Proof Command

```powershell
node scripts/test/v0-9-household-lan-production-discovery-proof.mjs
```

The command writes:

```text
test-results/v0-9-household-lan-production-discovery-proof/proof.json
```

It consumes:

```text
test-results/v0-9-household-lan-proof-readiness/proof.json
test-results/v0-9-production-lan-multidevice-hardening/proof.json
test-results/v0-9-lan-discovery-challenge-mvp/proof.json
test-results/v0-9-lan-pairing-control-mvp/proof.json
docs/expectations/pre-ai-proof-matrix.json
```

## Honest Boundaries

- Local multi-service proof uses real Rust service processes, but it is not
  household router discovery.
- A generated manual checklist is not physical two-device evidence.
- Stale/offline selected-device Rust proof is not firewall, router, mobile
  background, or physical-device behavior.
- Parent mobile controller authority remains manual-required until real
  Android and iOS package/device proof exists.
- Cloud relay is not implemented or counted as LAN proof.

## Manual Upgrade Requirements

Before upgrading V0.9 to product-ready household LAN readiness, archive proof
JSON from this commit and add real artifacts for:

1. two distinct physical devices on the same household LAN,
2. router reachability and firewall or OS prompt state,
3. parent and child host names or IPs,
4. origin allowlist used by the parent/controller host,
5. route selection, takeover, revocation, wrong-origin, wrong-device, replay,
   stale, offline, and failed-unpaired results on physical devices,
6. LAN AI provider advertised, accepted, rejected, and degraded behavior from a
   real opted-in provider host,
7. Android and iOS parent mobile observer/controller behavior,
8. authenticated cloud relay proof if cloud relay becomes part of the product
   path.

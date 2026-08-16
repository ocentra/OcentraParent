<!-- agent-capsule -->

> Agent Capsule
> Doc: V0.9 Production LAN And Mobile Controller Proof - 2026-05-29
> Kind: historical checkpoint/proof documentation; read only when CHECKPOINT_INDEX or PROOF_INDEX names it.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

# V0.9 Production LAN And Mobile Controller Proof - 2026-05-29

Branch: `codex/v0-9-production-lan-mobile-controller-proof`

## Scope

This checkpoint hardens the current V0.9 proof boundary for production LAN and
parent mobile controller/observer behavior. It does not claim product-ready
household LAN discovery, mobile UX parity, cloud relay, Android child policy, or
iOS Family Controls.

The proof command composes the existing household LAN readiness gate and parent
mobile shell runtime proof, then emits one focused artifact for:

- two local Rust service process proof,
- selected route recovery,
- trusted-device registry persistence,
- controller lease renewal/release/takeover,
- observer read-only rejection,
- wrong-device, stale, replay, missing-lease, expired-lease, and wrong-controller rejection,
- revocation before subsequent control rejection,
- parent mobile Android observer and iOS manual-required controller boundaries,
- cloud relay non-implementation.

## Proof Command

```powershell
node scripts/test/v0-9-production-lan-mobile-controller-proof.mjs
```

The command writes:

```text
test-results/v0-9-production-lan-mobile-controller-proof/proof.json
```

It consumes:

```text
test-results/v0-9-production-lan-multidevice-hardening/proof.json
test-results/v0-9-household-lan-proof-readiness/proof.json
test-results/parent-mobile-shell-runtime-proof/proof.json
```

## Honest Boundaries

- Local multi-service proof is real Rust service proof, not physical household
  router discovery.
- Parent mobile Android remains observer read-only in the backend proof.
- Parent mobile iOS controller takeover remains manual-required.
- Parent mobile local model execution remains disabled by default.
- Cloud relay remains not implemented and is not counted as LAN proof.
- Android device-owner policy, iOS Family Controls, signing, stores, and mobile
  background LAN behavior remain manual-required until real platform artifacts
  exist.

## Manual Upgrade Requirements

Before upgrading V0.9 to product-ready household LAN or mobile controller
readiness, archive proof JSON from this commit and add real artifacts for:

1. two distinct physical devices on the same household LAN,
2. router reachability and firewall or OS prompt state,
3. parent and child host names or IPs,
4. route selection, controller takeover, revocation, wrong-origin, wrong-device,
   stale, replay, and failed-unpaired behavior on physical devices,
5. Android and iOS parent mobile package/device observer and controller behavior,
6. an authenticated cloud relay proof if cloud relay becomes part of the product path.

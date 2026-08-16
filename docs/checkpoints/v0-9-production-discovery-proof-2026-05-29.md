<!-- agent-capsule -->

> Agent Capsule
> Doc: V0.9 Production Discovery Proof
> Kind: historical checkpoint/proof documentation; read only when CHECKPOINT_INDEX or PROOF_INDEX names it.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# V0.9 Production Discovery Proof

Branch: `codex/v0-9-production-discovery-proof`

This checkpoint hardens the V0.9 production discovery proof boundary after PR #162. It keeps the proof non-visual and product-truth focused: local Rust service proof can support selected-route trust and local multi-service discovery mechanics, but it cannot upgrade claims for physical household router discovery, mobile controller UX, cloud relay, or real router/firewall behavior.

## Covered

- Selected route contracts now expose the selected pairing id, selected-route trust state, stale timestamp, and offline timestamp.
- Rust trusted-registry selection preserves that selected-route trust data through local status reporting and restart recovery.
- The proof command `node scripts/test/v0-9-production-discovery-proof.mjs` composes the household LAN production discovery boundary proof and the local multi-service LAN hardening artifacts.
- Wrong-origin and wrong-device rejection remain required proof artifacts before any product claim can upgrade.

## Still Not Claimed

- Physical two-device household LAN discovery.
- Router/firewall or OS prompt behavior.
- Mobile controller write-authority UX or mobile background behavior.
- Cloud relay routing, storage, or authentication.

## Validation

Focused validation is run from this branch and reported through the hub with the final commit.

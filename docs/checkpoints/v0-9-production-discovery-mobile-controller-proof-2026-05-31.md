<!-- agent-capsule -->

> Agent Capsule
> Doc: V0.9 Production Discovery Mobile Controller Proof
> Kind: historical checkpoint/proof documentation; read only when CHECKPOINT_INDEX or PROOF_INDEX names it.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

# V0.9 Production Discovery Mobile Controller Proof

Date: 2026-05-31

Branch: `codex/v0-9-production-discovery-mobile-controller-proof`

## Scope

This checkpoint records the hardened aggregate proof for production discovery
and parent-mobile controller/observer backend behavior after PR190.

The proof covers:

- explicit production discovery states: discovered, pending, paired, revoked,
  stale, offline, and unavailable;
- paired route acceptance plus failed-unpaired, wrong-origin, wrong-device,
  replay, revoked, stale, offline, and unsupported-route rejection reasons;
- Android and iOS observer operation read models, including status, policy
  preview, capability refresh, takeover request, release, LAN AI submit, write,
  approve, pair, and revoke behavior;
- selected-device route recovery, trusted registry persistence, selected-route
  trust, wrong-device rejection, stale/replay/lease rejection, and revocation
  before rejected control;
- aggregate audit/proof custody labels tying route checks, observer operations,
  and manual physical-device proof boundaries together;
- manual-required physical household LAN proof, parent mobile write authority,
  and mobile background behavior;
- not-implemented cloud relay with manual decision required before any relay
  claim.

## Non-Claims

This proof does not claim:

- two physical household devices on the same router or firewall path;
- real Android or iOS parent-mobile write authority;
- mobile background LAN behavior, notifications, signing, stores, device-owner
  policy, or iOS Family Controls;
- cloud relay routing, authentication, storage, or remote control;
- C-owned UI or vendor rendering.

## Validation

Primary evidence command:

```powershell
node scripts/test/v0-9-household-discovery-mobile-controller-product-proof.mjs
```

The command builds contracts, runs the source proof harnesses, parses the
aggregate parent-domain read model, verifies the proof matrix registration, and
emits:

```text
test-results/v0-9-household-discovery-mobile-controller-product-proof/proof.json
```

Before PR-ready handoff, also run:

```powershell
npm run test --workspace @ocentra-parent/parent-domain -- tests/v0-9-household-discovery-mobile-controller-product-proof.test.ts
npm run test:pre-ai-proof
npm run --silent lint:schema-boundaries
npm run validate
```

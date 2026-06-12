<!-- agent-capsule -->

> Agent Capsule
> Doc: V0.9 Production Discovery And Household Route Proof - 2026-05-30
> Kind: historical checkpoint/proof documentation; read only when CHECKPOINT_INDEX or PROOF_INDEX names it.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

# V0.9 Production Discovery And Household Route Proof - 2026-05-30

Branch: `codex/v0-9-production-discovery-household-proof`

## Scope

This checkpoint keeps the V0.9 production discovery proof non-visual and honest
after PR #167. It composes the existing production discovery proof with the
household product read model so local Rust service evidence cannot be mistaken
for physical household LAN readiness.

The focused proof command verifies:

- explicit production discovery states for local real-service proof,
- paired and failed-unpaired household route checks,
- restart recovery of selected route and trusted registry state,
- wrong-origin, wrong-device, replay, revocation, stale, offline, unavailable,
  and manual-required route outcomes,
- a typed parent-domain and Rust protocol parity read model for production
  discovery, route checks, source states, restart recovery, and manual physical
  household proof gates,
- selected provider policy read-model evidence for authorized, unsupported,
  busy, degraded, unavailable, stale, offline, wrong-origin, wrong-device,
  replay, and revoked states,
- explicit physical household LAN and parent mobile controller manual-required
  states,
- explicit cloud-relay `not-implemented` plus `manual-decision-required` states
  before any cloud relay claim can be made.

## Proof Command

```powershell
node scripts/test/v0-9-production-discovery-household-proof.mjs
```

The command writes:

```text
test-results/v0-9-production-discovery-household-proof/proof.json
```

It consumes:

```text
test-results/v0-9-production-discovery-proof/proof.json
test-results/v0-9-household-lan-product-proof/proof.json
test-results/v0-9-household-lan-production-discovery-proof/proof.json
test-results/v0-9-production-lan-multidevice-hardening/proof.json
docs/expectations/pre-ai-proof-matrix.json
```

It validates the built `@ocentra-parent/parent-domain` `./lan-pairing` export
when available, specifically
`V09ProductionDiscoveryHouseholdProofReadModelSchema`.

## Honest Boundaries

- Local multi-service proof uses real Rust service processes, but it is not
  household router discovery.
- A selected local route is not router, firewall, NAT, OS prompt, or mobile
  background proof.
- Parent mobile controller authority remains manual-required until real Android
  and iOS package/device proof exists.
- Cloud relay has no runtime, route, storage, or authentication implementation
  in this proof.

## Manual Upgrade Requirements

Before upgrading V0.9 to product-ready household LAN or cloud-relay readiness,
archive proof JSON from this commit and add real artifacts for:

1. two distinct physical devices on the same household LAN,
2. router reachability and firewall or OS local-network prompt state,
3. parent and child host names or IPs,
4. origin allowlist used by the parent/controller host,
5. route selection, takeover, revocation, wrong-origin, wrong-device, replay,
   stale, offline, unavailable, and failed-unpaired results on physical devices,
6. LAN AI provider advertised, accepted, rejected, and degraded behavior from a
   real opted-in provider host,
7. Android and iOS parent mobile observer/controller behavior,
8. a separate product decision and authenticated cloud relay proof before
   counting cloud relay as implemented.

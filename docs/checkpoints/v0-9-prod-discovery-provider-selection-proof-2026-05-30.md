<!-- agent-capsule -->

> Agent Capsule
> Doc: V0.9 Production Discovery Provider Selection Proof - 2026-05-30
> Kind: historical checkpoint/proof documentation; read only when CHECKPOINT_INDEX or PROOF_INDEX names it.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

# V0.9 Production Discovery Provider Selection Proof - 2026-05-30

Branch: `codex/v0-9-prod-discovery-provider-selection-proof`

## Scope

This checkpoint keeps the V0.9 production discovery/provider-selection proof
non-visual and honest after PR #170. It adds typed provider-selection contracts,
Rust protocol parity, and a Rust service read model so provider selection can be
machine-checked without claiming physical household LAN readiness.

The focused proof verifies:

- production discovery provider candidates for selected, rejected, degraded,
  unavailable, manual-required, and not-implemented states,
- authorized provider policy plus unsupported, unpaired, stale, offline,
  route-blocked, and unavailable refusal states,
- paired and failed-unpaired discovery behavior inherited from the production
  discovery household proof,
- physical household provider proof as `manual-required`,
- optional cloud relay provider selection as `not-implemented` plus
  `manual-decision-required`.

## Proof Command

```powershell
node scripts/test/v0-9-prod-discovery-provider-selection-proof.mjs
```

The command writes:

```text
test-results/v0-9-prod-discovery-provider-selection-proof/proof.json
```

It consumes or validates:

```text
packages/parent-domain/src/lan-pairing-provider-selection-proof.ts
packages/parent-domain/tests/lan-pairing-provider-selection-proof.test.ts
crates/agent-protocol/src/lan_pairing_provider_selection.rs
crates/agent-service/src/lan_pairing_provider_selection_read_model.rs
test-results/v0-9-production-discovery-household-proof/proof.json
docs/expectations/pre-ai-proof-matrix.json
```

## Honest Boundaries

- The proof uses contracts and local Rust service read-model tests; it is not
  two-device household router discovery.
- A selected provider route is not router, firewall, NAT, OS local-network
  permission, or mobile background proof.
- Parent mobile controller UX remains outside this non-visual slice.
- Cloud relay has no runtime, route, storage, or authentication implementation
  in this proof.

## Manual Upgrade Requirements

Before upgrading provider selection to household-ready or cloud-ready, archive
proof JSON from this commit and add real artifacts for:

1. two distinct physical devices on the same household LAN,
2. real provider host identity and advertised capability evidence,
3. router reachability and firewall or OS local-network prompt state,
4. parent/controller origin allowlist from the physical controller host,
5. provider selection, stale/offline, revocation, failed-unpaired, wrong-origin,
   wrong-device, replay, unavailable, unsupported, busy, and degraded behavior
   on physical devices,
6. Android and iOS parent mobile observer/controller behavior,
7. a separate product decision and authenticated cloud relay proof before
   counting cloud relay provider selection as implemented.

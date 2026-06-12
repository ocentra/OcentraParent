<!-- agent-capsule -->

> Agent Capsule
> Doc: V0.9 Household Multi-Device Proof Gates
> Kind: historical checkpoint/proof documentation; read only when CHECKPOINT_INDEX or PROOF_INDEX names it.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

# V0.9 Household Multi-Device Proof Gates

Date: 2026-06-01
Branch: `codex/v0.9-household-multi-device-proof-gates`

## Scope

This checkpoint adds a non-visual V0.9 household multi-device proof gate that
composes the existing production discovery/mobile-controller product proof,
physical household artifact gate, and local multidevice hardening proof.

The gate covers:

- explicit manual gating for physical two-device, router/subnet, firewall or
  local-network permission, real mobile package, and custody artifacts
- paired and failed-unpaired household route evidence
- allowed-origin, wrong-device, replay, revocation, stale/offline, and
  unsupported-route evidence custody
- selected/trusted-device storage, selected-route recovery, selected-route
  trust, selected-device rejection, and wrong-device storage-security
  follow-through
- a browser-first, portal-visible, non-visual device spine for C-facing
  adapters: local-service LAN discovery boundary, add-device/pairing request
  state machine, trusted-device registry/read model,
  paired/offline/stale/manual-required selected-device states, current
  controller and observer route authority, LAN AI provider readiness, and
  artifact/readiness gates
- cloud relay implementation and remote-control state held at
  `not-implemented` with a manual product decision boundary

## Portal Adapter Spine

The new `portalDeviceSpine` read model is intentionally domain-only. It gives a
non-visual portal adapter these fields without modifying C-owned UI or vendor
paths:

- `lanDiscoveryBoundary`: local-service discovery is machine-proved for the
  browser-first slice while physical household LAN scanning remains
  `manual-required`
- `householdDeviceRegistry`: CI-backed registry proof plus paired, offline,
  stale, and manual-required device entries
- `addDevicePairingRequests`: discovered, pending, paired, rejected, expired,
  revoked, stale, and offline add-device/pairing request states
- `trustedDeviceRegistry`: registry entries plus selected-route recovery and
  trusted-registry proof counts
- `selectedDeviceReadiness`: selected route readiness plus manual-required
  physical artifact state
- `routeState`: current controller route, current observer route, active
  controller authority, observer read-only authority, and manual takeover state
- `lanAiProviderReadiness`: local provider proof, degraded mobile-provider
  state, and manual physical-provider artifact status
- `artifactReadinessGates`: required/collected/missing artifact counts,
  physical readiness state, and cloud-relay state

## Proof Boundary

This proof is intentionally not a physical household readiness claim:

- local multi-service proof is still CI/mechanical evidence
- physical household LAN readiness remains `manual-required`
- router, firewall, OS local-network permission, and real mobile package
  artifacts remain missing until manually collected
- remote desktop/control is explicitly not claimed in this V0.9 browser LAN
  pairing slice
- mobile-controller product UX remains out of this worker slice
- cloud relay routing, authentication, storage, and remote control remain
  `not-implemented`

The pre-AI proof matrix is not changed in this slice because another worker
currently owns that file. This checkpoint and the focused harness provide the
reviewable evidence until the matrix owner can integrate or sequence the entry.

## Validation Target

Focused validation for this slice:

```powershell
cmd /c npm run test --workspace @ocentra-parent/parent-domain -- tests/v0-9-household-multi-device-proof-gates.test.ts
cmd /c node --check scripts/test/v0-9-household-multi-device-proof-gates.mjs
cmd /c npm run build:contracts
cmd /c node scripts/test/v0-9-household-multi-device-proof-gates.mjs
```

Expected proof output:

```text
test-results/v0-9-household-multi-device-proof-gates/proof.json
```

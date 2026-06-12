<!-- agent-capsule -->

> Agent Capsule
> Doc: Browser-First LAN Discovery Add-Device State
> Kind: historical checkpoint/proof documentation; read only when CHECKPOINT_INDEX or PROOF_INDEX names it.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# Browser-First LAN Discovery Add-Device State

Branch: `codex/browser-first-lan-discovery-add-device-state`

This checkpoint adds a renderable add-device read model backed by the Rust LAN pairing status path. The portal can request the existing LAN pairing status command and consume:

- `addDeviceReadModel`: JSON read model with local-service discovery source, add-device state, discovered device entries, pending/expired pairing requests, trusted-device registry entries, selected-device readiness, authority labels, route requirement labels, audit check labels, and honest non-claims.
- `addDeviceState`, `localServiceDiscoveryState`, `physicalHouseholdLanState`, `cloudRelayState`: flattened states for simple UI badges.
- `selectedDeviceReady`, `pendingPairingCount`, `expiredPairingCount`, `trustedDeviceIds`, `revokedDeviceIds`: flattened service-backed summary fields.

Real source:

- Local service discovery is backed by the Rust service status event, selected route state, in-memory or local JSON trusted registry, and pending challenge state.
- Trusted registry and selected-device readiness come from the service runtime registry, not portal fixtures.
- Route/origin/replay/stale/revocation/wrong-device checks remain in existing LAN validation and are exposed as audit labels in the read model.

Honest gaps:

- Physical two-device household LAN proof remains `manual-required`.
- Cloud relay is `unavailable` and not implemented.
- Remote desktop/control is not implemented and is not claimed by this read model.

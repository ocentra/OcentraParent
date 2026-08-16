<!-- agent-capsule -->

> Agent Capsule
> Doc: LAN Browser Discovery Pairing Runtime Checkpoint
> Kind: historical checkpoint/proof documentation; read only when CHECKPOINT_INDEX or PROOF_INDEX names it.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# LAN Browser Discovery Pairing Runtime Checkpoint

Date: 2026-06-01

Branch: `codex/lan-browser-discovery-pairing-runtime`

## Scope

- Adds explicit browser-first LAN runtime command/event contracts for discovery scan and add-device request.
- Routes those commands through the Rust service LAN pairing runtime rather than portal fixtures.
- Keeps discovery honest: the service emits the current local-service command target or trusted registry state, while physical household LAN remains `manual-required`.
- Keeps cloud relay and remote desktop/control out of scope.

## Runtime Events

- `agent.lan-pairing.browser-discovery.scan` reports `agent.lan-pairing.browser-discovery.reported`.
- `agent.lan-pairing.add-device.request` reports `agent.lan-pairing.add-device.reported` after a valid challenge request.
- Wrong-origin add-device requests reject without trusting a device.
- Paired selected routes expose trusted registry entries and selected-device readiness.

## Non-Claims

- No fake household devices.
- No physical two-device/router/firewall proof.
- No cloud relay.
- No remote desktop/control.

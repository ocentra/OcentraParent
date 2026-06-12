<!-- agent-capsule -->

> Agent Capsule
> Doc: Family Setup And Device Roles
> Kind: feature documentation; read only when selected by FEATURE_ROUTE_INDEX, PLAN_INDEX, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# Family Setup And Device Roles

## Parent Outcome

A parent can create a household, add child profiles, pair child devices, invite
co-parents or observers, and understand which device is live, stale, offline,
revoked, local, LAN, relay, cache, or unavailable.

## Ocentra Requirement

Family setup is product foundation. It must not be treated as portal polish.
The child-device agent remains the authority for device role, controller lease,
revocation, stale command rejection, and local capability status.

## Roadmap And Expectations

- Roadmap: V0.9 LAN pairing, V1.0 local MVP, V5 parent policy product.
- Expectations: [family setup](../expectations/family-setup.md),
  [LAN pairing](../expectations/lan-pairing.md),
  [platforms](../expectations/platforms.md).
- Modules: `packages/parent-domain`, `packages/portal-domain`,
  `packages/agent-protocol-domain`, `crates/agent-service`.

## Competitor Pressure

See [Competitor Capability Map](../competitor-capability-map.md), especially
family setup, co-parent/observer roles, multi-device household, and remote
parent access.

Google, Apple, Microsoft, Bark, Qustodio, and similar products make family
groups, child profiles, and parent apps the first product action. Ocentra must
match that usability while keeping child-device authority local.

## Current Ocentra State

- Device roles, controller/observer concepts, trusted registry, route selection,
  stale/offline/degraded states, and LAN pairing proof are in progress.
- The V0.9 LAN spine now has a canonical household device row contract with
  source-backed LAN evidence records, durable parent decisions for
  assign/rename/ignore/restore/trust, selected-device LAN detail rows, and
  Activity > Network diagnostics that expose decision, evidence, route custody,
  signed proof, relay/cache, manual-proof, and audit state from the same read
  model.
- The add-device read model now carries a production household proof summary
  with trusted registry, parent assignment/rename/ignore/revocation, route
  custody, stale/offline selected-device, and manual-required physical
  household proof rows, so family setup can display truthfully what is ready
  versus what still needs real devices.
- The add-device read model also carries a signed discovery/relay spine summary
  with signed child-agent hello/heartbeat manual-required rows, stale/expired/
  replayed/wrong-origin/wrong-device/revoked/anonymous rejection rows, selected
  route custody, parent decision audit rows, relay/cache unavailability,
  parent-owned storage unavailability, and no Ocentra child-data custody state.
- The add-device read model now carries a LAN source matrix that keeps all 20
  plan workpacks and discovery sources visible to Devices/LAN and
  Activity/Network diagnostics. Weak sources are explicitly fenced so they
  cannot confirm a child agent or assign a child profile.
- Parent portal route and device surfaces exist in development form.
- The parent portal Devices/LAN interaction now exposes command-backed add,
  route select, rename, trust, ignore, restore, and revoke controls for
  service-backed LAN slots. Household decisions reuse the existing add-device
  request payload fields, route select/revoke use LAN route commands, and
  unsupported/router rows stay visible-only.
- Live B-lane browser proof now exists for the current service-backed LAN
  surfaces: Devices/LAN controls,
  `output/playwright/lan-source-matrix-plan-completion/devices-lan-source-matrix.png`;
  Activity/Network source-matrix diagnostics,
  `output/playwright/lan-source-matrix-plan-completion/activity-network-source-matrix.png`;
  Network policy targets,
  `output/playwright/lan-source-matrix-plan-completion/policy-network-target-binding.png`;
  and browser checks,
  `output/playwright/lan-source-matrix-plan-completion/browser-proof.json`.
- First-run consumer setup is not product-complete.

## Current Gap

There is no finished first-run flow that a nontechnical parent can use to create
a household, add a child, pair a child device, invite a co-parent, understand
source/proof state, and recover from stale/revoked/offline devices.

## Checklist

- [ ] Household profile contract.
- [ ] Child profile contract and UI.
- [ ] Parent-controller and parent-observer role UI.
- [x] Add-device/pairing read model backed by Rust service state, including
      production household proof rows for trusted registry, route custody,
      stale/offline selected-device, signed discovery/relay spine rows, relay/
      cache non-custody rows, LAN source-matrix rows, and manual-required real
      household proof.
- [ ] First-run add-device UX that lets a parent assign, rename, ignore,
      restore, and trust a device from the portal without raw protocol fields.
- [x] LAN selected-device details, Activity/Network diagnostics, and add-device
      request/route command wiring consume the service-backed read model without
      a second portal-only LAN truth.
- [x] Live browser screenshot proof covers the current Devices/LAN,
      Activity/Network, and Network policy surfaces on the B-lane service path.
- [ ] Revocation and recovery flow. Current proof covers typed revocation state;
      parent recovery UX remains.
- [ ] Source labels: local, LAN, relay, cache, parent-owned storage,
      unavailable. Current proof marks relay/cache and parent-owned storage not
      implemented/unavailable, keeps no Ocentra child-data custody explicit, and
      keeps physical household proof manual-required.
- [x] Portal tests cover LAN slot/parser fixture states for signed discovery,
      route custody, relay/cache unavailable, manual-proof, and parent-decision
      fields.
- [ ] Portal tests for full setup, recovery, and degraded first-run states.
- [ ] Real LAN proof before claiming multi-device household readiness. Current
      proof harness is contract/Rust-service backed and single-machine; two
      physical child-agent hosts plus signed hello/heartbeat remain required.

## Next AI Instructions

Start from the family setup expectation doc. Do not build UI-only setup. Add or
reuse contracts first, then service-backed state, then portal rendering and
tests. Keep parent-account membership separate from child-device authority.

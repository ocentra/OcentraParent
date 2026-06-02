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
  assign/rename/ignore/restore/trust, and Activity > Network diagnostics that
  expose decision/evidence counts from the same read model.
- The add-device read model now carries a production household proof summary
  with trusted registry, parent assignment/rename/ignore/revocation, route
  custody, stale/offline selected-device, and manual-required physical
  household proof rows, so family setup can display truthfully what is ready
  versus what still needs real devices.
- Parent portal route and device surfaces exist in development form.
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
      stale/offline selected-device, and manual-required real household proof.
- [ ] First-run add-device UX that lets a parent assign, rename, ignore,
      restore, and trust a device from the portal without raw protocol fields.
- [ ] Revocation and recovery flow. Current proof covers typed revocation state;
      parent recovery UX remains.
- [ ] Source labels: local, LAN, relay, cache, parent-owned storage,
      unavailable. Current proof marks relay/cache not implemented and keeps
      physical household proof manual-required.
- [ ] Portal tests for setup and degraded states.
- [ ] Real LAN proof before claiming multi-device household readiness. Current
      proof harness is contract/Rust-service backed and single-machine; two
      physical child-agent hosts plus signed hello/heartbeat remain required.

## Next AI Instructions

Start from the family setup expectation doc. Do not build UI-only setup. Add or
reuse contracts first, then service-backed state, then portal rendering and
tests. Keep parent-account membership separate from child-device authority.

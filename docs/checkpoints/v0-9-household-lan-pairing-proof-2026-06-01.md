<!-- agent-capsule -->

> Agent Capsule
> Doc: V0.9 Household LAN Pairing Proof
> Kind: historical checkpoint/proof documentation; read only when CHECKPOINT_INDEX or PROOF_INDEX names it.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

# V0.9 Household LAN Pairing Proof

Date: 2026-06-01

Branch: `codex/v0-9-household-lan-pairing-proof`

## Scope

- Adds a typed parent-domain aggregate proof for the browser-first household LAN pairing slice.
- Composes the existing browser add-device state proof, browser LAN pairing runtime proof, and household LAN readiness gate.
- Keeps the portal adapter boundary non-visual: local-service discovery, add-device pairing states, trusted registry, selected readiness, route security labels, and manual gates are modelled without touching portal UI.
- Adds canonical household device rows derived from the LAN add-device read model so one physical device has one identity across local-service and LAN-neighbor sources, with role badges, route/trust state, hostname/IP/MAC inventory, and child-agent inventory when a Rust agent is reachable.
- Keeps routers and unsupported LAN devices visible but non-enrollable, and makes paired child-agent devices stable targets for devices, policy, browser, app, screen, network, activity, tracking, and AI surfaces.
- Keeps physical household LAN, parent mobile controller write authority, cloud relay, and remote desktop/control unclaimed.

## Proof Command

```powershell
cmd /c node scripts/test/v0-9-household-lan-pairing-proof.mjs
```

The harness writes `test-results/v0-9-household-lan-pairing-proof/proof.json`.

## Evidence Boundary

- Local browser-first proof covers discovered, pending, paired, rejected, expired, revoked, stale, and offline add-device states.
- Browser runtime proof covers scan, add-device request, wrong-origin rejection, and selected readiness events.
- Canonical household rows are derived from the add-device read model and cover local-agent plus network-neighbor merge behavior, router/unsupported classification, child-agent inventory packets, and shared devices/policy/browser/app/screen/network/activity/tracking/AI target-surface availability for paired child-agent devices.
- Household readiness proof keeps physical two-device/router/firewall/origin/revocation/stale/offline/mobile artifacts manual-required.
- Cloud relay and remote desktop/control remain not implemented.

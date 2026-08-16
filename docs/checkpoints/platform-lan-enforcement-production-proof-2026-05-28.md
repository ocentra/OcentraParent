<!-- agent-capsule -->

> Agent Capsule
> Doc: Platform LAN And Enforcement Production Proof - 2026-05-28
> Kind: historical checkpoint/proof documentation; read only when CHECKPOINT_INDEX or PROOF_INDEX names it.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

# Platform LAN And Enforcement Production Proof - 2026-05-28

Branch: `codex/platform-lan-enforcement-production-proof`
Base: `e6533114f6294e029294190effac462d90ce9d48`

## Scope

This checkpoint is worker B's V0.8/V0.9 production proof hardening pass after
the V0.8 enforcement hardening and V0.9 LAN multi-device PRs landed. It does
not create a broad parental-control or production LAN claim. It verifies the
real paths that exist and records the manual proof still required for physical
devices and privileged OS behavior.

The proof command covers:

- V0.8 owned-process app time-limit behavior through the real Rust service:
  execute/create, restart recovery, parent cancel/override, unavailable
  missing-state recovery, expiry, audit, encrypted journal, and SQLite storage.
- V0.8 production enforcement hardening through the real Rust service:
  process-control, network-control, and managed-browser-control commands return
  unavailable or manual-required states for broad blocking paths that are not
  proven yet.
- V0.9 production LAN hardening through real Rust service processes: discovery
  challenge, selected-route recovery, controller conflict/takeover,
  wrong-origin and wrong-device rejection, provider routing states, and explicit
  physical two-device manual checklist output.
- Platform/package truth: parent desktop service connection is mechanically
  checked, while parent mobile, Android child, iOS child, signing, stores,
  device-owner policy, Family Controls, and cloud relay stay scaffold,
  manual-required, unavailable, or not implemented as appropriate.

## Proof Command

```powershell
node scripts/test/platform-lan-enforcement-production-proof.mjs
```

This command runs:

- `cmd /c npm run build:contracts`
- `cmd /c node scripts/test/v0-8-windows-app-time-limit-adapter-mvp.mjs`
- `cmd /c node scripts/test/v0-8-production-enforcement-hardening.mjs`
- `cmd /c node scripts/test/v0-9-production-lan-multidevice-hardening.mjs`
- `cmd /c node --test scripts/test/platform-packaging.test.mjs`

The wrapper validates generated artifacts and writes:

- `test-results/platform-lan-enforcement-production-proof/proof.json`
- `test-results/v0-8-windows-app-time-limit-adapter-mvp/*.json`
- `test-results/v0-8-production-enforcement-hardening/*.json`
- `test-results/v0-9-production-lan-multidevice-hardening/proof.json`
- `test-results/platform-roles-lan-ai-provider-pool/proof.json`

## Proof Labels

Expected wrapper labels:

- `v0.8.owned-process-time-limit-service-proof`
- `v0.8.restart-recovery-parent-cancel-expiry-proof`
- `v0.8.manual-required-broad-adapter-state-proof`
- `v0.9.production-lan-local-multiservice-proof`
- `v0.9.physical-household-lan-manual-required`
- `proof-matrix.platform-lan-enforcement-production-states`

## Honest Boundaries

- V0.8 proves a real owned-process app time-limit path where the host supports
  it. It does not prove global app blocking, browser-management enforcement,
  domain/network blocking, anti-tamper, or production rollback outside the
  recorded manual-required/unavailable states.
- V0.9 proves local multi-service mechanics over real Rust service processes.
  It does not prove household router discovery, firewall prompts, mobile
  background LAN behavior, or a finished portal selector.
- Parent mobile controller/observer behavior remains backend/proof-first
  scaffold or unavailable until a real mobile app/device proof exists.
- Android child claims remain capability-specific and manual-required for
  foreground service, storage, protocol bridge, permissions, UsageStats,
  accessibility, VPN/DNS, device-owner, and managed profile.
- iOS child claims remain capability-specific and manual-required for Family
  Controls, DeviceActivity, Screen Time, Network Extension, notifications,
  background execution, signing, and device/TestFlight proof.
- Cloud relay behavior is not implemented or claimed by this branch.

## Owner-Ready Manual Proof

Before upgrading V0.8 or V0.9 to product-complete:

1. Run the production proof command on a real Windows host and archive every
   generated proof JSON with the commit SHA.
2. Repeat V0.9 LAN discovery/control across two physical household devices on
   the same LAN. Record parent and child IPs, firewall state, allowed origin,
   route selection, controller takeover, revocation, wrong-origin rejection,
   restart recovery, and provider routing state.
3. Run parent mobile controller/observer backend proof from a real mobile
   package before claiming mobile controller UX parity.
4. Run Android child-agent proof on emulator or physical hardware for every
   capability being claimed.
5. Run iOS child-agent proof through approved Apple entitlements and
   device/TestFlight paths before claiming child-agent parity.

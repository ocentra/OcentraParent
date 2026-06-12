<!-- agent-capsule -->

> Agent Capsule
> Doc: V0.8 And V0.9 Product Proof Final Pass - 2026-05-27
> Kind: historical checkpoint/proof documentation; read only when CHECKPOINT_INDEX or PROOF_INDEX names it.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

# V0.8 And V0.9 Product Proof Final Pass - 2026-05-27

Branch: `codex/v08-v09-product-proof-final-pass`
Base: `3e12d4e7cc4bee7e5171883ff7fb9b115ac98fc6`

## Scope

This checkpoint is the B final pass for V0.8 enforcement-adapter proof hardening
and V0.9 LAN/multi-device product-proof hardening. It ties the already-merged
runtime paths to one reviewable proof command instead of upgrading claims.

The final-pass proof covers:

- V0.8 app time-limit adapter behavior through the real Rust service:
  execute/create, restart recovery, parent cancel/override, missing-state
  unavailable behavior, expiry, audit, and storage.
- V0.9 LAN behavior through real Rust service processes: controller lease
  conflict and takeover, wrong-origin and wrong-device rejection, trusted
  registry restart behavior, stale/offline selected-device service tests,
  provider advertisement, and degraded LAN AI paths.
- Platform/runtime truth: parent desktop service connection and LAN AI provider
  pool proof remain implemented, while parent mobile, Android child, iOS child,
  signing, stores, device-owner policy, and Family Controls remain
  scaffold/manual-required/unavailable until separately proven.

## Proof Command

```powershell
node scripts/test/v0-8-v0-9-product-proof-final-pass.mjs
```

This command runs:

- `cmd /c npm run build:contracts`
- `cargo test -p ocentra-parent-agent-service lan_pairing_status_reports_stale_and_offline_selected_device_state`
- `cargo test -p ocentra-parent-agent-service controller_lease`
- `cmd /c node scripts/test/v0-8-windows-app-time-limit-adapter-mvp.mjs`
- `cargo build -p ocentra-parent-agent-service`
- `cmd /c node scripts/test/v0-9-lan-pairing-control-mvp.mjs`
- `cmd /c node scripts/test/platform-roles-lan-ai-provider-pool.mjs`

The wrapper validates each generated artifact and writes:

- `test-results/v0-8-v0-9-product-proof-final-pass/proof.json`
- `test-results/v0-8-windows-app-time-limit-adapter-mvp/*.json`
- `test-results/v0-9-lan-pairing-control-mvp/proof.json`
- `test-results/platform-roles-lan-ai-provider-pool/proof.json`

## Proof Labels

Expected wrapper labels:

- `v0.8.enforcement.owned-process-expiry-proven` on Windows, or
  `v0.8.enforcement.non-windows-unavailable-proven` on non-Windows.
- `v0.8.enforcement.restart-recovery-proven`
- `v0.8.enforcement.parent-cancel-override-proven`
- `v0.8.enforcement.audit-and-storage-proven`
- `v0.9.lan.controller-conflict-and-takeover-proven`
- `v0.9.lan.registry-restart-persistence-proven`
- `v0.9.lan.wrong-origin-and-wrong-device-rejection-proven`
- `v0.9.lan.degraded-provider-state-proven`
- `v0.9.lan.provider-selection-available-rejected-degraded-proven`
- `platform.parent-mobile-scaffold-unavailable-state-proven`
- `proof-matrix.final-pass-honest-platform-states-proven`

## Honest Boundaries

- V0.8 is still not a complete parental-control enforcement product. The real
  Windows proof is an owned local process time-limit adapter path, not global
  app blocking, browser enforcement, anti-tamper, or production rollback.
- V0.9 is still not a complete LAN product. The proof uses local real Rust
  service processes and direct WebSocket routing, not production discovery,
  cloud relay, router/firewall proof, or finished portal selector UX.
- Parent desktop packaged service connection is mechanically proven, but
  installer lifecycle, service autostart, signing, notarization, stores, and
  mobile/child platform privileges remain separate proof gates.
- Parent mobile remains observer/scaffold/unavailable or degraded for controller
  takeover and LAN AI behavior until a real mobile app/device proof exists.
- Android child and iOS child remain manual-required or unavailable for
  privileged behavior until device-owner, UsageStats, accessibility, VPN/DNS,
  managed profile, Family Controls, DeviceActivity, Screen Time, Network
  Extension, signing, and device/TestFlight proof exist.

## Owner-Ready Manual Proof

Before upgrading V0.8 or V0.9 to product-complete:

1. Run the final-pass proof harness on a Windows host and archive the generated
   proof JSON with the commit SHA.
2. Repeat V0.9 LAN pairing/control with two physical household devices on the
   same LAN. Record IPs, firewall state, allowed origin, route selection,
   controller takeover, revocation, wrong-origin rejection, and restart state.
3. Run Android child-agent proof on emulator or physical hardware for the exact
   capabilities being claimed.
4. Run iOS child-agent proof through approved Apple entitlements and
   device/TestFlight paths before claiming child-agent parity.

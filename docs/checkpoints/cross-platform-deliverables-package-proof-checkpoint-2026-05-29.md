<!-- agent-capsule -->

> Agent Capsule
> Doc: Cross-Platform Deliverables Package Proof Checkpoint - 2026-05-29
> Kind: historical checkpoint/proof documentation; read only when CHECKPOINT_INDEX or PROOF_INDEX names it.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# Cross-Platform Deliverables Package Proof Checkpoint - 2026-05-29

Branch: `codex/cross-platform-deliverables-proof-checkpoint`
Base: `9c70fb60a0869ee2b841ba4ceeb45c0800483e9a`
Package version: `0.1.1`
Local host: Windows 11 Pro `10.0.26200`, x64, Gigabyte Technology Co., Ltd. X570 AORUS MASTER

## Scope

This checkpoint executes the current cross-platform package and proof runbook
from `docs/architecture/cross-platform-deliverables-checkpoint.md` after PR
#143. It is a proof and evidence branch, not a product feature branch.

The pass records:

- local Windows proof commands that ran on the real checkout at `9c70fb6`;
- GitHub Actions state for package-preview mechanics across Windows, Linux,
  macOS, Android, and iOS;
- generated proof artifact paths under ignored `test-results/`;
- honest manual-required, scaffold, unavailable, or not-implemented states for
  privileged OS/device behavior that CI and this Windows host cannot prove.

## Local Commands Run

All commands below ran from
`C:\Users\sujan\.codex\worktrees\ocentra-parent-codex-d\OcentraParent` on the
checkpoint branch.

| Command                                                             | Result | Evidence                                                                                  |
| ------------------------------------------------------------------- | ------ | ----------------------------------------------------------------------------------------- |
| `cmd /c npm run format:check`                                       | Passed | Prettier checked all matched files.                                                       |
| `cmd /c npm run test:pre-ai-proof`                                  | Passed | `pre-ai-proof-ok: 19 claims checked across 5 platforms; 15 checkpoint scenarios checked.` |
| `cmd /c node scripts/test/platform-os-lan-mobile-proof.mjs`         | Passed | `test-results/platform-os-lan-mobile-proof/proof.json`.                                   |
| `cmd /c node scripts/test/enforcement-lan-mobile-product-proof.mjs` | Passed | `test-results/enforcement-lan-mobile-product-proof/proof.json`.                           |
| `cmd /c node --test scripts/test/platform-packaging.test.mjs`       | Passed | 8 package/proof workflow tests passed.                                                    |

The proof wrappers also ran `build:contracts`, parent-domain capability tests,
Rust service/core proof tests, V0.8 app time-limit proof, V0.8 production
enforcement hardening proof, V0.9 production LAN local multi-service proof, V0.9
household LAN readiness, platform-role/LAN AI provider proof, and managed
browser intervention proof where available on this Windows host.

## Local Proof Output

Generated outputs remain ignored local artifacts. See
`docs/checkpoints/artifacts/cross-platform-deliverables-package-proof-checkpoint-2026-05-29/README.md`
for the artifact manifest.

Key generated records:

- `test-results/platform-os-lan-mobile-proof/proof.json`
- `test-results/enforcement-lan-mobile-product-proof/proof.json`
- `test-results/platform-lan-enforcement-production-proof/proof.json`
- `test-results/v0-8-os-adapter-proof-hardening/proof.json`
- `test-results/v0-9-household-lan-proof-readiness/proof.json`
- `test-results/managed-browser-intervention-proof/2026-05-29T06-37-58-667Z.json`

The managed-browser intervention proof found Chrome, Firefox, and Edge on this
host and produced blocked-site, blocked-YouTube, and allowed-control screenshots
for all three browsers. That is local Windows evidence for the managed-browser
intervention harness only. It is not proof of broad app blocking, unmanaged
exact URL visibility, or production household enforcement.

## GitHub Actions Package State

Live main run captured:

- Run id: `26621839147`
- Run URL: <https://github.com/ocentra/OcentraParent/actions/runs/26621839147>
- Head SHA: `9c70fb60a0869ee2b841ba4ceeb45c0800483e9a`
- Conclusion: success

Package-preview jobs in the main push run:

| Platform | Job                                           | Result                                            | Artifact                               |
| -------- | --------------------------------------------- | ------------------------------------------------- | -------------------------------------- |
| Windows  | `package-preview / Windows MSI Preview`       | Passed build, MSI install/uninstall smoke, upload | `ocentra-parent-windows-x64-preview`   |
| Linux    | `package-preview / Linux DEB Preview`         | Passed DEB build, install/remove smoke, upload    | `ocentra-parent-linux-amd64-preview`   |
| macOS    | `package-preview / macOS PKG Preview`         | Passed PKG build, payload smoke, upload           | `ocentra-parent-macos-preview`         |
| Android  | `package-preview / Android APK Preview`       | Passed APK build and emulator smoke               | `ocentra-parent-android-preview`       |
| iOS      | `package-preview / iOS Simulator App Preview` | Passed simulator build, install, launch smoke     | `ocentra-parent-ios-simulator-preview` |

These are CI mechanical proofs for package, install, launch, and smoke behavior.
They do not prove real host permissions, mobile background behavior, signing,
store review, device-owner policy, iOS Family Controls, or production LAN
behavior.

## Proof Labels Confirmed

`platform-os-lan-mobile-proof` confirmed:

- `cloud-relay.explicit-not-implemented`
- `v0.8.os-adapter-browser-boundary-nonclaim-proof`
- `v0.8.process-terminate-owned-process-proof`
- `v0.9.household-two-device-manual-checklist`
- `v0.9.household-readiness-gate-manual-required`
- `v0.8.managed-browser-intervention-actually-enforced`
- `mobile-platform.capability-specific-states`
- `proof-matrix.platform-os-lan-mobile-proof-states`

`enforcement-lan-mobile-product-proof` confirmed:

- `v0.8.os-enforcement.product-capability-states`
- `v0.8.os-enforcement.browser-boundary-nonclaim-states`
- `v0.9.production-lan.household-manual-proof-boundary`
- `v0.9.production-lan.household-readiness-gate`
- `mobile-platform.package-signing-capability-states`
- `proof-matrix.enforcement-lan-mobile-product-proof`

## Honest Boundaries

Implemented or mechanically proved:

- Current TypeScript/Rust contracts build through `build:contracts`.
- Pre-AI proof matrix shape passes with 19 claims and 15 checkpoint scenarios.
- V0.8 owned-process/time-limit service proof runs through real Rust service
  paths on this Windows host.
- Managed-browser intervention proof ran locally against installed Chrome,
  Firefox, and Edge.
- V0.9 local multi-service LAN proof paths remain mechanically proved through
  real local Rust service processes.
- CI package-preview mechanics are proven for Windows, Linux, macOS, Android,
  and iOS on the main push run for `9c70fb6`.

Manual-required or still unproved:

- Physical two-device household LAN discovery, router reachability, firewall
  prompts, stale/offline selected-device behavior, and failed-unpaired behavior
  on distinct devices.
- Broad OS app blocking and network/domain blocking outside the proved
  owned-process/time-limit and managed-browser intervention boundaries.
- Android child-agent foreground service durability, UsageStats, accessibility,
  VPN/DNS, device-owner, managed profile, package lifecycle on a real device,
  and Play policy behavior.
- iOS Family Controls, DeviceActivity, Screen Time, Network Extension,
  notifications, background execution, signing, TestFlight, entitlement review,
  and real-device behavior.
- macOS Screen Recording, Accessibility, launchd, signing, notarization, and
  real permission behavior beyond CI package payload smoke.
- Production cloud relay remains not implemented and is not counted as local or
  LAN proof.

## PR Body Outline

Scope

- Executed the cross-platform deliverables/package proof checkpoint from latest
  main after #143.
- Recorded local Windows proof outputs and CI package-preview state for Windows,
  Linux, macOS, Android, and iOS.
- Kept unsupported platform, mobile, signing, entitlement, store, cloud relay,
  and physical household LAN states explicit.

Touched files

- `docs/checkpoints/cross-platform-deliverables-package-proof-checkpoint-2026-05-29.md`
- `docs/checkpoints/artifacts/cross-platform-deliverables-package-proof-checkpoint-2026-05-29/README.md`
- `docs/product-roadmap.md`

Validation

- `cmd /c npm run format:check`
- `cmd /c npm run test:pre-ai-proof`
- `cmd /c node scripts/test/platform-os-lan-mobile-proof.mjs`
- `cmd /c node scripts/test/enforcement-lan-mobile-product-proof.mjs`
- `cmd /c node --test scripts/test/platform-packaging.test.mjs`
- `cmd /c npm run validate`
- Final branch validation command list is recorded in the worker report.

Known gaps and risks

- No product claim is upgraded for physical household LAN, Android child-agent,
  iOS child-agent, macOS permissions/signing, stores, cloud relay, or broad OS
  blocking.

Roadmap slice

- V0.7 pre-AI/enforcement cross-platform deliverables and package proof
  checkpoint.

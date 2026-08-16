<!-- agent-capsule -->

> Agent Capsule
> Doc: Cross-Platform Deliverables Package Proof Artifacts - 2026-05-29
> Kind: historical checkpoint/proof documentation; read only when CHECKPOINT_INDEX or PROOF_INDEX names it.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# Cross-Platform Deliverables Package Proof Artifacts - 2026-05-29

This manifest supports
`docs/checkpoints/cross-platform-deliverables-package-proof-checkpoint-2026-05-29.md`.

The generated proof JSON, screenshots, package outputs, and downloaded binaries
remain ignored local artifacts under `test-results/` and `target/`. Do not
commit those binary or generated files.

## Local Proof Artifacts

| Artifact                                                                                | Purpose                                                                                       | Commit state           |
| --------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------- | ---------------------- |
| `test-results/platform-os-lan-mobile-proof/proof.json`                                  | Aggregate OS, LAN, mobile, managed-browser, cloud-relay, and proof-matrix state.              | Ignored local artifact |
| `test-results/enforcement-lan-mobile-product-proof/proof.json`                          | Product-level V0.8/V0.9/mobile proof boundary and package/signing state.                      | Ignored local artifact |
| `test-results/platform-lan-enforcement-production-proof/proof.json`                     | V0.8/V0.9 production proof aggregate consumed by the checkpoint wrappers.                     | Ignored local artifact |
| `test-results/v0-8-os-adapter-proof-hardening/proof.json`                               | V0.8 OS adapter proof hardening aggregate.                                                    | Ignored local artifact |
| `test-results/v0-9-household-lan-proof-readiness/proof.json`                            | Local multi-service LAN proof and physical household LAN manual-required gate.                | Ignored local artifact |
| `test-results/managed-browser-intervention-proof/2026-05-29T06-37-58-667Z.json`         | Managed-browser intervention proof for installed Chrome, Firefox, and Edge.                   | Ignored local artifact |
| `test-results/managed-browser-intervention-proof/2026-05-29T06-37-58-667Z-screenshots/` | Blocked-site, blocked-YouTube, and allowed-control screenshots for Chrome, Firefox, and Edge. | Ignored local artifact |

## Local Host

| Field           | Value                                                |
| --------------- | ---------------------------------------------------- |
| Commit          | `9c70fb60a0869ee2b841ba4ceeb45c0800483e9a`           |
| Branch          | `codex/cross-platform-deliverables-proof-checkpoint` |
| Package version | `0.1.1`                                              |
| OS              | Microsoft Windows 11 Pro `10.0.26200`, 64-bit        |
| Host model      | Gigabyte Technology Co., Ltd. X570 AORUS MASTER      |

## Current Main CI Run

| Field      | Value                                                               |
| ---------- | ------------------------------------------------------------------- |
| Run id     | `26621839147`                                                       |
| Run URL    | <https://github.com/ocentra/OcentraParent/actions/runs/26621839147> |
| Head SHA   | `9c70fb60a0869ee2b841ba4ceeb45c0800483e9a`                          |
| Conclusion | success                                                             |

Current main run state:

| Job                                                    | State   |
| ------------------------------------------------------ | ------- |
| `fail-fast / Format, Lint, Types, Rust Check`          | success |
| `secret-scan / Secrets and Sensitive Files`            | success |
| `dependency-policy / Dependency Audit, Licenses, SBOM` | success |
| `build / Production Build`                             | success |
| `validate / Pre-AI Proof Matrix`                       | success |
| `validate / Full Validation Gate`                      | success |
| `validate / Real Portal To Rust E2E (windows-latest)`  | success |
| `validate / Real Portal To Rust E2E (macos-latest)`    | success |
| `validate / Real Portal To Rust E2E (ubuntu-latest)`   | success |
| `package-preview / Windows MSI Preview`                | success |
| `package-preview / Linux DEB Preview`                  | success |
| `package-preview / macOS PKG Preview`                  | success |
| `package-preview / Android APK Preview`                | success |
| `package-preview / iOS Simulator App Preview`          | success |

Artifacts from the current main run:

| Artifact                               | ID           | Digest                                                                    | Size             |
| -------------------------------------- | ------------ | ------------------------------------------------------------------------- | ---------------- |
| `ocentra-parent-windows-x64-preview`   | `7284716982` | `sha256:9e248abed3f95f65d59ed62bfbffc903c931b28b209d3b2e3351a22d51d140b2` | `20058271` bytes |
| `ocentra-parent-linux-amd64-preview`   | `7284662003` | `sha256:4d5e8274b4df1046e99b5b6797cef93767f7a9b20df9289c12bf18933e82147e` | `5436409` bytes  |
| `ocentra-parent-macos-preview`         | `7284653372` | `sha256:6d42c327593fddbcc07edef6dd78c1fe5fdf2e610fe1c42b905fb66bbff57c7e` | `6102171` bytes  |
| `ocentra-parent-android-preview`       | `7284664788` | `sha256:9dfa6bc7e84b83fdb30df661bf64b99b142bf69ee191542ed869b205d37c040f` | `11821` bytes    |
| `ocentra-parent-ios-simulator-preview` | `7284664571` | `sha256:4a009c2409235a816f98ebfe6c55ffd2467e4b6f36b773462548842bea6506f5` | `86155` bytes    |
| `ocentra-parent-security-sbom`         | `7284568989` | `sha256:407b45837c2ab2070c7629a955fdfc2e3f781f0d190b68c4e95d9b662b87459f` | `176743` bytes   |

## Completed Package-Preview Source Run

PR #143 also completed package-preview before merge and is retained here as the
pre-merge comparison source.

| Field      | Value                                                               |
| ---------- | ------------------------------------------------------------------- |
| Run id     | `26621063516`                                                       |
| Run URL    | <https://github.com/ocentra/OcentraParent/actions/runs/26621063516> |
| Head SHA   | `8c395f5720e4f69e507213823d792d5ffc24f6fc`                          |
| Conclusion | success                                                             |

| Artifact                               | ID           | Digest                                                                    | Size             |
| -------------------------------------- | ------------ | ------------------------------------------------------------------------- | ---------------- |
| `ocentra-parent-windows-x64-preview`   | `7284396980` | `sha256:93cedde318916ea1bdff70cb891545672f248e2ec2f61036f99fb45a65c83837` | `20059133` bytes |
| `ocentra-parent-linux-amd64-preview`   | `7284343121` | `sha256:6c3947a02c29cbf0b0bff107bbf9c07332257cf1d48bb56d1700fc396601c837` | `5436381` bytes  |
| `ocentra-parent-macos-preview`         | `7284346515` | `sha256:ddb9d3a341192afcdd455b5885c21db15e4d97f41a8fe669bcf5462a6ec08c5c` | `6102178` bytes  |
| `ocentra-parent-android-preview`       | `7284355723` | `sha256:b682f94169d00afae1cc66e01fa2e7f0dc5211c4eee170f0b6033ae40149a7bb` | `11816` bytes    |
| `ocentra-parent-ios-simulator-preview` | `7284357973` | `sha256:1f4ba20a7d364cef4934b4fdf157f3f9f90cae2ac834c717784d547e6c4642b5` | `86195` bytes    |
| `ocentra-parent-security-sbom`         | `7284261128` | `sha256:b0611d7753818e4f2f257074f2852cd496a0400a84a4f1e62554bff602ebfc24` | `176742` bytes   |

## Manual-Required Evidence Not Captured Here

These remain required before product support claims are upgraded:

- two physical devices on the same household LAN with router/firewall evidence;
- Android physical-device foreground service, notification, UsageStats,
  accessibility, VPN/DNS, device-owner, managed-profile, and package lifecycle
  proof;
- iOS signing, TestFlight, Family Controls, DeviceActivity, Screen Time,
  Network Extension, notifications, background execution, and real-device proof;
- macOS permission, launchd, signing, notarization, and real host behavior;
- production cloud relay, which is still not implemented.

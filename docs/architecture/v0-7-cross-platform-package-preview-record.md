<!-- agent-capsule -->

> Agent Capsule
> Doc: V0.7 Cross-Platform Package Preview Record After PR87
> Kind: architecture/reference documentation; read only when selected by plan route, source router, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

# V0.7 Cross-Platform Package Preview Record After PR87

## Scope

This record captures the post-PR87 CI package-preview and LAN checkpoint
evidence from current `main`. It is a docs/evidence/status artifact only. It
does not add feature implementation, portal UI work, vendor files, roadmap
reconciliation, proof-matrix upgrades, production package publishing,
production LAN authentication, or real two-device LAN proof.

The authoritative checkpoint runbook remains
`docs/architecture/cross-platform-deliverables-checkpoint.md`. This record uses
the same proof labels to separate hosted CI mechanics from real host, device,
network, signing, store, and household LAN proof.

## Run Metadata

| Field               | Value                                                                                                              |
| ------------------- | ------------------------------------------------------------------------------------------------------------------ |
| Proof date          | 2026-05-24                                                                                                         |
| Worktree            | `C:\Users\sujan\.codex\worktrees\ocentra-parent-codex-b\OcentraParent`                                             |
| Branch              | `codex/v0.7-cross-platform-package-preview-proof-after-pr87`                                                       |
| Baseline commit     | `4aade13fe7fe9dff294932efbbdbdcfccba4c5e8`                                                                         |
| Baseline subject    | `Prove LAN registry persistence restart behavior (#87)`                                                            |
| Package/app version | `0.1.1`                                                                                                            |
| GitHub Actions run  | `26371210839`                                                                                                      |
| Sensitive data      | No child activity, screenshots, private browser history, raw evidence payloads, decrypted logs, or device secrets. |

## Source Inputs

- `docs/architecture/cross-platform-deliverables-checkpoint.md`
- `docs/architecture/current-main-proof-refresh-2026-05-24.md`
- `docs/architecture/local-lan-manual-proof-runbook.md`
- PR #87: <https://github.com/ocentra/OcentraParent/pull/87>
- GitHub Actions CI Gate run:
  <https://github.com/ocentra/OcentraParent/actions/runs/26371210839>
- Hub follow-up `codex-b-msg-20260524T201505577Z-140`, which confirmed all
  five package-preview jobs green and preserved the CI-mechanical/manual-gap
  proof boundary.

## Fresh Commands And Results

| Command                                                                                       | Result                                                                                          | Proof label           |
| --------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------- | --------------------- |
| `git fetch origin --prune`                                                                    | Passed before branch creation.                                                                  | `implemented`         |
| `git switch -c codex/v0.7-cross-platform-package-preview-proof-after-pr87 origin/main`        | Passed; branch created from `origin/main` at `4aade13`.                                         | `implemented`         |
| `cmd /c npm run lanes:guard`                                                                  | Passed on the assigned codex-b branch.                                                          | `implemented`         |
| `cmd /c npm run hub:ack`                                                                      | Passed for `codex-b-msg-20260524T200709591Z-139`.                                               | `implemented`         |
| `cmd /c npm run hub:ack`                                                                      | Passed for `codex-b-msg-20260524T201505577Z-140`.                                               | `implemented`         |
| `cmd /c npm run hub:lock -- --paths docs/architecture/v0-7-cross-platform-package-preview...` | Passed; this record is the only locked edit path.                                               | `implemented`         |
| `gh pr view 87 --repo ocentra/OcentraParent ...`                                              | Passed; PR #87 scope and merge commit recorded below.                                           | `implemented`         |
| `gh run watch 26371210839 --repo ocentra/OcentraParent --exit-status`                         | Passed; run completed successfully after Windows MSI package-preview finished.                  | `ci-mechanical-proof` |
| `gh run view 26371210839 --repo ocentra/OcentraParent --json ...`                             | Passed; final run/job ledger recorded below.                                                    | `ci-mechanical-proof` |
| `gh api repos/ocentra/OcentraParent/actions/runs/26371210839/artifacts --paginate`            | Passed; package-preview and SBOM artifacts were listed and were not expired at inspection time. | `ci-mechanical-proof` |

## PR87 Scope Impact

| PR  | Merge commit | Scope impact                                                                                                                                                                                                                                                | Current proof meaning                                                                                                                                                   |
| --- | ------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| #87 | `4aade13`    | Added the V0.9 LAN persistent trusted-device registry proof spine across parent-domain contracts, Rust protocol support values, and Rust service runtime behavior. It adds explicit `local-json-registry` and `restore-trusted-registry-unselected` states. | Proves the typed/service-backed local registry persistence spine and default fail-closed restart behavior. It does not prove production LAN auth or real household LAN. |

PR #87 improves LAN restart and trusted-registry evidence, but it does not
change the manual proof bar for real two-device LAN, firewall/router behavior,
production LAN authentication, service-install persistence, encrypted registry
storage, signing, stores, or mobile entitlements.

## CI Gate Snapshot

| Field              | Value                                                               |
| ------------------ | ------------------------------------------------------------------- |
| Workflow           | `CI Gate`                                                           |
| Run id             | `26371210839`                                                       |
| Run URL            | <https://github.com/ocentra/OcentraParent/actions/runs/26371210839> |
| Head SHA           | `4aade13fe7fe9dff294932efbbdbdcfccba4c5e8`                          |
| Branch             | `main`                                                              |
| Event              | `push`                                                              |
| Status             | `completed`                                                         |
| Conclusion         | `success`                                                           |
| Created UTC        | 2026-05-24 19:55:06 UTC                                             |
| Updated UTC        | 2026-05-24 20:10:20 UTC                                             |
| Checkpoint meaning | Current `main` is green after PR #87.                               |

## CI Job Ledger

| Job name                                               | Result    | Evidence recorded                                                                                  |
| ------------------------------------------------------ | --------- | -------------------------------------------------------------------------------------------------- |
| `fail-fast / Format, Lint, Types, Rust Check`          | `success` | Formatting, release-version policy, package lint, TypeScript type-check, and Rust check completed. |
| `secret-scan / Secrets and Sensitive Files`            | `success` | Repository secret scanner and Gitleaks completed.                                                  |
| `build / Production Build`                             | `success` | Production build gate completed.                                                                   |
| `validate / Pre-AI Proof Matrix`                       | `success` | Pre-AI proof matrix check completed.                                                               |
| `dependency-policy / Dependency Audit, Licenses, SBOM` | `success` | Dependency policy, license policy, cargo audit, and SBOM upload completed.                         |
| `validate / Full Validation Gate`                      | `success` | Full validation gate completed in CI.                                                              |
| `validate / Real Portal To Rust E2E (ubuntu-latest)`   | `success` | Hosted Ubuntu portal-to-Rust E2E completed against the real Rust service.                          |
| `validate / Real Portal To Rust E2E (windows-latest)`  | `success` | Hosted Windows portal-to-Rust E2E completed against the real Rust service.                         |
| `validate / Real Portal To Rust E2E (macos-latest)`    | `success` | Hosted macOS portal-to-Rust E2E completed against the real Rust service.                           |
| `package-preview / iOS Simulator App Preview`          | `success` | iOS simulator app build and simulator install/launch smoke completed.                              |
| `package-preview / Windows MSI Preview`                | `success` | Windows MSI build and CI install/uninstall smoke completed; MSI preview artifact uploaded.         |
| `package-preview / macOS PKG Preview`                  | `success` | macOS PKG build and payload smoke completed.                                                       |
| `package-preview / Android APK Preview`                | `success` | Android APK build and emulator install/launch smoke completed.                                     |
| `package-preview / Linux DEB Preview`                  | `success` | Linux DEB build and CI install/remove smoke completed.                                             |

## Package-Preview Platform Conclusions

| Platform target | Package-preview job                           | CI result | Current proof label   | Honest conclusion                                                                                                                                                                                                                                                                                                                                        |
| --------------- | --------------------------------------------- | --------- | --------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Windows         | `package-preview / Windows MSI Preview`       | `success` | `ci-mechanical-proof` | Hosted runner built the MSI, ran CI install/uninstall smoke, and uploaded the preview artifact. Real child-PC install, service autostart after reboot, update, uninstall data retention, and production signing remain manual-required.                                                                                                                  |
| Linux           | `package-preview / Linux DEB Preview`         | `success` | `ci-mechanical-proof` | Hosted Ubuntu runner built the DEB, ran install/remove smoke, and uploaded the preview artifact. Real desktop service-manager behavior and Linux-specific capture adapters remain not-yet-proven unless separately tested.                                                                                                                               |
| macOS           | `package-preview / macOS PKG Preview`         | `success` | `ci-mechanical-proof` | Hosted macOS runner built the PKG, ran payload smoke, and uploaded the preview artifact. Launchd behavior, signing, notarization, permissions, and store distribution remain manual-required.                                                                                                                                                            |
| Android         | `package-preview / Android APK Preview`       | `success` | `ci-mechanical-proof` | Hosted Android job built the APK, started an emulator with KVM, ran install/launch smoke, and uploaded the preview artifact. Physical-device, device-owner, managed-profile, accessibility, VPN/DNS, and foreground-service proof remain manual-required or not-yet-proven.                                                                              |
| iOS             | `package-preview / iOS Simulator App Preview` | `success` | `ci-mechanical-proof` | Hosted macOS runner built the iOS simulator app, ran simulator install/launch smoke, and uploaded the preview artifact. TestFlight, device install, Family Controls, Screen Time, Network Extension, notifications, background execution, and entitlements remain manual-required, permission-required, blocked, or unavailable until separately proven. |

## Uploaded Artifact Ledger

| Artifact name                          | Size in bytes | Digest                                                                    | Created UTC             | Expired | Proof level           |
| -------------------------------------- | ------------: | ------------------------------------------------------------------------- | ----------------------- | ------- | --------------------- |
| `ocentra-parent-windows-x64-preview`   |    19,035,350 | `sha256:9746a48ad1ac164e517c94220ea7531c6fbaca5524e1a627683ead3387babd48` | 2026-05-24 20:10:16 UTC | `false` | `ci-mechanical-proof` |
| `ocentra-parent-ios-simulator-preview` |        86,193 | `sha256:05b3408b287551121b710e3296df984634ad279c31390491bbd4f0e11e7d1632` | 2026-05-24 20:07:21 UTC | `false` | `ci-mechanical-proof` |
| `ocentra-parent-android-preview`       |        11,829 | `sha256:43e6be4ac24f80a7bcea5000724c8c7395a6b24e6bc9b1d7b1435505f9d089af` | 2026-05-24 20:07:19 UTC | `false` | `ci-mechanical-proof` |
| `ocentra-parent-linux-amd64-preview`   |     4,252,487 | `sha256:e6030c5458321c4148e9f755aa6ba5d7f073ff0a52ec0d57cb71c9d78ca14f62` | 2026-05-24 20:07:16 UTC | `false` | `ci-mechanical-proof` |
| `ocentra-parent-macos-preview`         |     4,556,296 | `sha256:7781d5e0dccdd086d7439b680567d0a8b3a2a08c239b805c05ae511ffadb14b0` | 2026-05-24 20:06:07 UTC | `false` | `ci-mechanical-proof` |
| `ocentra-parent-security-sbom`         |       176,725 | `sha256:476db297a4a48211a4e5dc7b9a0fd81f66d4f5e221ba0c00c6fe541c278a5344` | 2026-05-24 20:02:15 UTC | `false` | `ci-mechanical-proof` |

These artifacts prove preview build/upload and hosted smoke mechanics only.
They are not production release assets, signed distribution proof, notarized
packages, store submissions, TestFlight proof, managed-device proof, or
reboot/autostart proof.

## LAN Paired And Unpaired Proof Labels

| LAN area                                            | Current evidence after PR87                                                                                                                                                                                 | Current proof label             | Boundary and required follow-up                                                                                                             |
| --------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------- |
| Default in-memory restart behavior                  | PR #87 service tests prove restart without registry persistence returns safe unpaired/fail-closed state and rejects old signed LAN control.                                                                 | `implemented` for service spine | This is a local service/protocol proof, not a production LAN deployment proof.                                                              |
| Explicit local JSON trusted-device registry         | PR #87 adds `OCENTRA_PARENT_AGENT_LAN_PAIRING_REGISTRY_PATH`, `local-json-registry`, and `restore-trusted-registry-unselected`; tests prove trusted devices restore after restart as paired but unselected. | `implemented` for opt-in spine  | Local JSON persistence is explicit opt-in. Production storage hardening, encryption, path policy, and failure surfacing remain future work. |
| Persistent revocation after restart                 | PR #87 tests prove revocation persists through restart when the explicit local JSON registry is configured, and old signed control rejects as `revoked`.                                                    | `implemented` for service spine | Real installed-service restart, reboot, and data-retention behavior remain manual-required.                                                 |
| Paired control after route selection                | Service tests prove restored trusted devices must be selected before signed child control is accepted.                                                                                                      | `implemented` for service spine | Real parent-to-child LAN route selection across two devices was not run.                                                                    |
| Unpaired or unselected control rejection            | Existing LAN service tests and PR #87 tests prove anonymous, unpaired, unselected, revoked, stale, replayed, wrong-origin, and wrong-device requests reject before child execution.                         | `implemented` for service spine | Real household negative test from a second device, firewall/router state, and LAN address proof remain manual-required.                     |
| LAN discovery/challenge HTTP surface                | Runtime status keeps unsupported/planned HTTP discovery/challenge/proof/control/registry endpoints explicit.                                                                                                | `scaffold-only`                 | Served production HTTP discovery/challenge flow is not implemented or claimed.                                                              |
| Production LAN authentication and household pairing | No two physical devices, router/firewall state, real paired request, or failed unpaired request was recorded in this pass.                                                                                  | `manual-required`               | Run the local/LAN manual proof runbook with two devices, explicit pairing, allowed origin, paired request, and failed unpaired request.     |

## Proof Matrix Handling

Do not update `docs/expectations/pre-ai-proof-matrix.json` from this record.
The CI run proves green current-main mechanics, package previews, portal-to-Rust
E2E mechanics, and the PR #87 service/protocol LAN persistence spine. It does
not prove privileged package lifecycle rows, production LAN auth, household LAN
pairing, signing, stores, mobile entitlements, or physical-device behavior.

## CI Annotations

- `validate / Real Portal To Rust E2E (windows-latest)` reported GitHub's
  `windows-latest` runner redirect notice toward `windows-2025-vs2026` by
  June 15, 2026. This is CI environment drift to watch, not a product proof.
- `package-preview / Android APK Preview` reported GitHub's Node.js 20 action
  deprecation notice for JavaScript actions. This is CI maintenance work to
  track before GitHub's Node 24 default/removal dates, not a runtime product
  failure in this run.

## Explicit Omissions

- No portal, vendor, C-owned, runtime, service, package script, or workflow
  files were touched.
- No A-owned validation record was edited.
- No `docs/product-roadmap.md` or
  `docs/expectations/pre-ai-proof-matrix.json` edit was made.
- No package-preview artifact was downloaded or installed locally.
- No local Windows MSI install, elevated service check, reboot, autostart,
  update, uninstall, installed-service data retention, or local portal against
  installed service proof was run.
- No real two-device LAN pairing, firewall/router behavior, paired request, or
  failed unpaired request was run.
- No macOS host install/signing/notarization, Android physical-device, iOS
  TestFlight/device, store, entitlement, or managed-device proof was run.

## Known Gaps And Risks

- Green package-preview CI is still CI-mechanical proof for packaging and smoke
  behavior. Real package lifecycle proof remains manual-required per platform.
- PR #87 proves local service/protocol LAN persistence behavior, but production
  LAN authentication, discovery, firewall/router behavior, and two-device
  household proof remain manual-required.
- Local JSON registry persistence is explicit opt-in and not yet production
  storage hardening.
- GitHub runner and action-version drift is visible in annotations and should
  be tracked separately from product proof.

## Roadmap Slice

V0.7 cross-platform package-preview and LAN checkpoint evidence after PR #87.
This record makes current `main` package-preview and LAN proof labels
reviewable without overstating support claims or changing proof-matrix state.

## PR Body Outline

```text
Scope
- Added a V0.7 package-preview and LAN checkpoint evidence record after PR #87.
- Captured GitHub Actions run 26371210839, package-preview job outcomes, artifact names, artifact digests, and platform proof labels.
- Recorded LAN paired/unpaired and trusted-registry proof labels after PR #87 without claiming production LAN readiness.
- Kept scope docs/evidence/status only; no portal, vendor, runtime, workflow, package script, roadmap, proof-matrix, or A-owned record edits.

Touched files
- docs/architecture/v0-7-cross-platform-package-preview-record.md

Validation
- cmd /c npm run format:check
- cmd /c npm run test:pre-ai-proof
- git diff --check
- cmd /c npm run lanes:guard
- cmd /c npm run hub:guard

Known gaps and risks
- Package previews are green but remain CI-mechanical proof.
- Real package lifecycle, signing, stores, TestFlight, entitlements, and physical-device behavior remain manual-required or not-yet-proven.
- PR #87 LAN persistence proof is service/protocol-backed, but production LAN auth, firewall/router behavior, and real two-device household proof remain manual-required.
```

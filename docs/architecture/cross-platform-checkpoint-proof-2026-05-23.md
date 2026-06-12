<!-- agent-capsule -->

> Agent Capsule
> Doc: Cross-Platform Checkpoint Proof - 2026-05-23
> Kind: architecture/reference documentation; read only when selected by plan route, source router, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

# Cross-Platform Checkpoint Proof - 2026-05-23

## Scope

This record prepares the V0.7 cross-platform deliverables checkpoint proof/status
pass from the current `main` baseline. It is a docs/evidence/status artifact
only. It does not add portal UI work, V0.8 enforcement, platform adapters,
policy implementation, proof-matrix upgrades, or model execution.

The authoritative runbook remains
`docs/architecture/cross-platform-deliverables-checkpoint.md`. This record
captures what this Windows worker could prove now, what current GitHub Actions
already proves mechanically, and which OS/device proof steps remain unavailable
or manual-required.

## Run Metadata

| Field               | Value                                                                                                     |
| ------------------- | --------------------------------------------------------------------------------------------------------- |
| Proof date          | 2026-05-23                                                                                                |
| Worktree            | `C:\Users\sujan\.codex\worktrees\ocentra-parent-codex-b\OcentraParent`                                    |
| Branch              | `codex/cross-platform-checkpoint-proof`                                                                   |
| Baseline commit     | `cebc2a158f9b97bcc02c14786cca7ed502255190`                                                                |
| Baseline subject    | `fix: recover managed browser service proof`                                                              |
| Package/app version | `0.1.1`                                                                                                   |
| Local host OS       | Windows 11 Pro `10.0.26200`, build `26200`, `AMD64`                                                       |
| Local host model    | `X570 AORUS MASTER`                                                                                       |
| Local CPU           | `AMD Ryzen 9 3900X 12-Core Processor`                                                                     |
| Active LAN adapter  | `Ethernet 2`, IPv4 `192.168.2.10`                                                                         |
| Sensitive data      | No raw child activity, browser history, screenshots, or private logs were recorded in this repo artifact. |

## Source Inputs

- `.ocentra-ai/rules/ocentra-parent-rules.mdc`
- `.ocentra-ai/rules/ocentra-parent-validation.mdc`
- `.ocentra-ai/rules/ocentra-parent-source-shape.mdc`
- `.ocentra-ai/rules/ocentra-parent-domain-boundaries.mdc`
- `docs/product-roadmap.md`
- `docs/architecture/cross-platform-deliverables-checkpoint.md`
- `docs/architecture/v0.7-checkpoint-validation-evidence-report-2026-05-22.md`
- `docs/architecture/platform-package-proof-ledger-2026-05-22.md`
- `docs/architecture/v0-7-checkpoint-acceptance-summary-2026-05-22.md`
- `docs/architecture/windows-lan-checkpoint-proof-2026-05-22.md`
- `docs/architecture/controlled-local-evidence-proof-results-2026-05-22.md`

## Fresh Commands And Results

| Command                                                                                 | Result                                                                                                                                                                                                                                              | Proof label                                         |
| --------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------- |
| `git fetch origin main`                                                                 | Passed; latest `origin/main` fetched before branch creation.                                                                                                                                                                                        | `implemented`                                       |
| `git switch -c codex/cross-platform-checkpoint-proof origin/main`                       | Passed; branch created from `origin/main`.                                                                                                                                                                                                          | `implemented`                                       |
| `git rev-parse HEAD`                                                                    | `cebc2a158f9b97bcc02c14786cca7ed502255190`.                                                                                                                                                                                                         | `implemented`                                       |
| `git status --short --branch`                                                           | Clean before edits: `## codex/cross-platform-checkpoint-proof...origin/main`.                                                                                                                                                                       | `implemented`                                       |
| `cmd /c npm run lanes:claim -- --lane codex-b ... --force`                              | Passed; codex-b claimed for this checkpoint proof/status branch.                                                                                                                                                                                    | `implemented`                                       |
| `cmd /c npm run lanes:guard`                                                            | Passed before editing.                                                                                                                                                                                                                              | `implemented`                                       |
| `cmd /c npm run hub:guard`                                                              | Passed before editing.                                                                                                                                                                                                                              | `implemented`                                       |
| `gh run list --branch main --workflow "CI Gate" --limit 5 --json ...`                   | Passed; latest `main` CI Gate run is `26313044898`, `success`, for commit `cebc2a158f9b97bcc02c14786cca7ed502255190`.                                                                                                                               | `ci-mechanical-proof`                               |
| `gh run view 26313044898 --json jobs,conclusion,status,createdAt,updatedAt,headSha,url` | Passed; all CI Gate jobs for the latest `main` run completed with `success`.                                                                                                                                                                        | `ci-mechanical-proof`                               |
| `gh api repos/ocentra/OcentraParent/actions/runs/26313044898/artifacts ...`             | Passed; package-preview and SBOM artifacts are listed below and were not expired at inspection time.                                                                                                                                                | `ci-mechanical-proof`                               |
| `cmd /c npm run format:check`                                                           | Passed: `All matched files use Prettier code style!`.                                                                                                                                                                                               | `implemented`                                       |
| `cmd /c npm run test:pre-ai-proof`                                                      | Passed: `pre-ai-proof-ok: 11 claims checked across 5 platforms; 7 checkpoint scenarios checked.`                                                                                                                                                    | `implemented`                                       |
| `cmd /c npm run validate`                                                               | Passed. The full repo gate completed successfully, including release version, pre-AI proof, lint/schema/source/test-double guards, package lint/type-check, tests, Rust validation, integration smoke, LAN smoke, portal smoke, and Playwright E2E. | `implemented`                                       |
| `wsl.exe --list --verbose`                                                              | Passed; `Ubuntu-22.04` and `docker-desktop` WSL distributions are present and stopped.                                                                                                                                                              | `manual-required`                                   |
| `wsl.exe -d Ubuntu-22.04 -- bash -lc "uname -a; ..."`                                   | Passed; WSL2 reports Linux kernel `5.15.167.4-microsoft-standard-WSL2`, with `npm=/usr/bin/npm` and `cargo=/root/.cargo/bin/cargo`.                                                                                                                 | `manual-required`                                   |
| `wsl.exe -d Ubuntu-22.04 -- bash -lc "cd ... && npm run test:pre-ai-proof"`             | Passed with the same `pre-ai-proof-ok: 11 claims checked across 5 platforms; 7 checkpoint scenarios checked.` structural result.                                                                                                                    | `ci-mechanical-proof` for WSL structural proof only |
| `docker version --format ...`                                                           | Failed because `docker` is not available on this Windows PATH.                                                                                                                                                                                      | `manual-required`                                   |
| `adb version`                                                                           | Passed; Android Debug Bridge `1.0.41`, version `35.0.2-12147458`, is installed.                                                                                                                                                                     | `manual-required`                                   |
| `adb devices -l`                                                                        | Passed; no Android devices or emulators were attached.                                                                                                                                                                                              | `manual-required`                                   |
| `Get-Command xcodebuild -ErrorAction SilentlyContinue`                                  | Returned no command on this Windows host.                                                                                                                                                                                                           | `not-applicable` for this machine                   |
| `Get-NetIPConfiguration ...`                                                            | Passed; active local LAN candidate is `Ethernet 2` at `192.168.2.10`, with WSL and local virtual adapters also present.                                                                                                                             | `implemented` for metadata                          |

## Local Full Validation Evidence

`cmd /c npm run validate` ran:

```text
npm run release:version
npm run test:pre-ai-proof
npm run lint
npm run test
npm run validate:rust
npm run test:integration
npm run test:e2e
```

Key observed results:

- Release version alignment passed: `Release version 0.1.1 is aligned across 18 source(s).`
- Pre-AI proof matrix passed: `11 claims checked across 5 platforms; 7 checkpoint scenarios checked.`
- Schema/source/test guardrails passed:
  - no direct Zod source usage across 214 checked files;
  - no manual string brands or naked domain string aliases;
  - no inline app string literals across 30 checked files;
  - no inline Rust service/core string literals across 139 checked files;
  - required test scaffold is present for all source workspaces;
  - no test doubles across 351 checked source files;
  - source-shape guard passed with warning-only near-budget files/functions;
  - AI rule index check passed.
- Turbo lint/type-check passed: `24 successful, 24 total`.
- Rust tests passed for:
  - `ocentra_parent_agent_core`: 64 passed;
  - `ocentra_parent_agent_maintenance`: 4 passed;
  - `manifest_contract`: 4 passed;
  - `ocentra_parent_agent_protocol`: 45 passed;
  - `ocentra_parent_agent_service`: 49 passed.
- Real service smoke paths passed:
  - `websocket-local-smoke-ok:agent.connection.ready,agent.health.reported,agent.activity.ingest.status.reported`
  - `websocket-lan-smoke-ok:agent.connection.ready,agent.health.reported`
  - `portal-local-smoke-ok`
  - Playwright portal E2E: `1 passed`.

The source-shape warnings are advisory and should remain split-before-growth
signals. This docs/status branch did not modify the warned source files.

## Latest Main CI Snapshot

| Field              | Value                                                                                     |
| ------------------ | ----------------------------------------------------------------------------------------- |
| Workflow           | `CI Gate`                                                                                 |
| Run id             | `26313044898`                                                                             |
| Run URL            | <https://github.com/ocentra/OcentraParent/actions/runs/26313044898>                       |
| Head SHA           | `cebc2a158f9b97bcc02c14786cca7ed502255190`                                                |
| Display title      | `fix: recover managed browser service proof`                                              |
| Status             | `completed`                                                                               |
| Conclusion         | `success`                                                                                 |
| Created UTC        | 2026-05-22 21:33:15 UTC                                                                   |
| Updated UTC        | 2026-05-22 21:45:38 UTC                                                                   |
| Checkpoint meaning | Current `main` has green CI and package-preview mechanics after PR #66 and PR #67 merges. |

### CI Job Ledger

| Job name                                               | Result    | CI-mechanical proof recorded                                                                       |
| ------------------------------------------------------ | --------- | -------------------------------------------------------------------------------------------------- |
| `fail-fast / Format, Lint, Types, Rust Check`          | `success` | Formatting, release-version policy, package lint, TypeScript type-check, and Rust check completed. |
| `secret-scan / Secrets and Sensitive Files`            | `success` | Repository secret scanner and Gitleaks completed.                                                  |
| `validate / Pre-AI Proof Matrix`                       | `success` | Pre-AI proof matrix check completed.                                                               |
| `build / Production Build`                             | `success` | Production build gate completed.                                                                   |
| `dependency-policy / Dependency Audit, Licenses, SBOM` | `success` | Dependency policy, license policy, cargo audit, and SBOM upload completed.                         |
| `validate / Real Portal To Rust E2E (windows-latest)`  | `success` | Hosted Windows portal-to-Rust E2E completed against the real Rust service.                         |
| `validate / Real Portal To Rust E2E (ubuntu-latest)`   | `success` | Hosted Ubuntu portal-to-Rust E2E completed against the real Rust service.                          |
| `validate / Real Portal To Rust E2E (macos-latest)`    | `success` | Hosted macOS portal-to-Rust E2E completed against the real Rust service.                           |
| `validate / Full Validation Gate`                      | `success` | Full validation gate completed in CI.                                                              |
| `package-preview / macOS PKG Preview`                  | `success` | macOS PKG build, payload smoke, and artifact upload completed.                                     |
| `package-preview / Windows MSI Preview`                | `success` | Windows MSI build, CI install/uninstall smoke, and artifact upload completed.                      |
| `package-preview / iOS Simulator App Preview`          | `success` | iOS simulator app build, simulator install/launch smoke, and artifact upload completed.            |
| `package-preview / Android APK Preview`                | `success` | Android APK build, emulator install/launch smoke, and artifact upload completed.                   |
| `package-preview / Linux DEB Preview`                  | `success` | Linux DEB build, CI install/remove smoke, and artifact upload completed.                           |

### Uploaded Artifact Ledger

| Artifact name                          | Size in bytes | Created UTC             | Expired | Proof level           |
| -------------------------------------- | ------------: | ----------------------- | ------- | --------------------- |
| `ocentra-parent-windows-x64-preview`   |    18,959,288 | 2026-05-22 21:45:33 UTC | `false` | `ci-mechanical-proof` |
| `ocentra-parent-android-preview`       |        11,828 | 2026-05-22 21:43:42 UTC | `false` | `ci-mechanical-proof` |
| `ocentra-parent-ios-simulator-preview` |        86,165 | 2026-05-22 21:43:35 UTC | `false` | `ci-mechanical-proof` |
| `ocentra-parent-linux-amd64-preview`   |     4,153,457 | 2026-05-22 21:43:14 UTC | `false` | `ci-mechanical-proof` |
| `ocentra-parent-macos-preview`         |     4,435,965 | 2026-05-22 21:42:23 UTC | `false` | `ci-mechanical-proof` |
| `ocentra-parent-security-sbom`         |       166,613 | 2026-05-22 21:39:24 UTC | `false` | `ci-mechanical-proof` |

These artifacts prove preview package mechanics only. They are not production
release assets, store submissions, TestFlight proof, signing trust, notarized
packages, real device proof, or reboot/autostart proof.

## Cross-Platform Checkpoint Status

| Platform or area                     | Fresh proof in this pass                                                                                                                                                                                       | Current label                                                                                                                                                      | Required follow-up before claim upgrade                                                                                                                     |
| ------------------------------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Shared repo validation               | Windows-local `format:check`, `test:pre-ai-proof`, and full `validate` passed on `cebc2a1`. Latest `main` CI Gate also passed on the same commit.                                                              | `implemented` for shared local mechanics; `ci-mechanical-proof` for hosted CI                                                                                      | Continue to run full validation before PR-ready handoffs.                                                                                                   |
| Windows local PC metadata            | Fresh host metadata and active LAN adapter were recorded from this Windows PC.                                                                                                                                 | `implemented` for metadata                                                                                                                                         | Package install, autostart, reboot, uninstall, and privileged runtime checks still need artifact-backed manual proof.                                       |
| Windows local service and portal     | Full validate launched real local service, LAN smoke, portal local smoke, and Playwright portal E2E.                                                                                                           | `implemented` for loopback/LAN smoke mechanics                                                                                                                     | Do not treat smoke as proof of package lifecycle, household LAN, or privileged OS behavior.                                                                 |
| Windows package lifecycle            | Latest CI proves MSI build and CI install/uninstall smoke. No Windows preview artifact was downloaded, installed, rebooted, or uninstalled on this host in this pass.                                          | `ci-mechanical-proof`; lifecycle remains `manual-required`                                                                                                         | Install current artifact on a real Windows host, check service/autostart after reboot, update/uninstall cleanup, and data retention.                        |
| Windows evidence/capture claims      | This pass did not rerun controlled foreground, browser, app duration, screen queue, or network/domain runtime probes. Existing records remain authoritative.                                                   | Mixed: foreground window has prior `implemented`; managed browser URL/title, duration, screen queue, and domain proof remain `not-yet-proven` or `manual-required` | Run fresh product-path probes only when assigned, keeping synthetic evidence and explicit degraded states.                                                  |
| Linux CI                             | Latest CI proves Ubuntu portal-to-Rust E2E and Linux DEB package build/install/remove smoke.                                                                                                                   | `ci-mechanical-proof`                                                                                                                                              | Repeat service/package proof in WSL or Docker before broader Linux claims.                                                                                  |
| Linux WSL                            | WSL2 Ubuntu exists. Lightweight `npm run test:pre-ai-proof` passed inside WSL from the same checkout. No WSL `npm ci`, full Linux validation, DEB install, service launch, or package lifecycle proof was run. | `ci-mechanical-proof` for structural proof only; runtime/package proof remains `manual-required`                                                                   | Run WSL/Docker install, launch, filesystem, journal, SQLite, and service-manager proof against Linux artifacts.                                             |
| Docker                               | Docker CLI is not available on this Windows PATH.                                                                                                                                                              | `manual-required`                                                                                                                                                  | Install/start Docker Desktop or use WSL directly for Linux package proof.                                                                                   |
| macOS                                | Latest CI proves hosted macOS portal-to-Rust E2E and PKG build/payload smoke. This Windows worker has no macOS host or `xcodebuild`.                                                                           | `ci-mechanical-proof`; real Mac proof remains `manual-required`                                                                                                    | Run on a Mac for launchd, permission, signing, notarization, and real package launch behavior.                                                              |
| Android emulator                     | Latest CI proves Android APK build and emulator install/launch smoke. Local `adb` is installed, but no emulator/device was attached in this pass.                                                              | `ci-mechanical-proof`; local/manual proof remains `manual-required`                                                                                                | Start an emulator or connect a device, then record app/agent role, permissions, foreground service, storage, and notification states.                       |
| Android physical device              | `adb devices -l` showed no attached devices.                                                                                                                                                                   | `manual-required`                                                                                                                                                  | Use a real Android device for foreground service durability, notification permission, UsageStats/accessibility/VPN or device-owner states when implemented. |
| iOS simulator                        | Latest CI proves simulator app build/install/launch smoke. This Windows worker has no Xcode path.                                                                                                              | `ci-mechanical-proof`; local Mac/Xcode proof remains `manual-required`                                                                                             | Run on Mac/Xcode for simulator details, signing team, provisioning, TestFlight notes, and entitlement status.                                               |
| iOS device, TestFlight, entitlements | No iOS device, TestFlight, Family Controls, DeviceActivity, Screen Time, Network Extension, notification entitlement, or background execution proof is possible from this Windows worker.                      | `manual-required`, `permission-required`, or `blocked` by capability                                                                                               | Gather Apple credential/entitlement/device evidence before upgrading any iOS capability claim.                                                              |
| LAN parent-to-child                  | Local validation ran LAN smoke and the active LAN adapter is present. No second parent device, explicit pairing, paired request, or failed unpaired request was run.                                           | `ci-mechanical-proof` for LAN smoke; household LAN remains `manual-required`                                                                                       | Use two real devices with explicit pairing, allowed origin, child identity, and failed unpaired request evidence.                                           |
| Local AI runtime/model execution     | Validation covers local runtime/status contract tests and disabled/unavailable boundaries. No model execution was run.                                                                                         | `implemented` for honest status boundaries; no execution claim                                                                                                     | Do not run or claim model execution until the user explicitly resumes that scope.                                                                           |
| Enforcement                          | Dry-run policy preview and disabled enforcement handoff remain validated. No V0.8 enforcement was added or tested.                                                                                             | No enforcement claim                                                                                                                                               | Keep blocking, app control, timers, rollback, and notification delivery out of scope until V0.8 is assigned.                                                |

## Explicit Omissions

- No product code or proof-matrix file was changed.
- No portal UI, C-owned files, V0.8 enforcement adapter, model execution,
  platform policy implementation, or package code was touched.
- No package-preview artifact was downloaded or installed locally.
- No Windows autostart, reboot survival, update, uninstall, or data-retention
  lifecycle proof was run.
- No fresh managed browser URL/title, timed app/game duration, screen queue, or
  network/domain controlled runtime proof was run in this pass.
- No Docker proof was run because `docker` was not available on PATH.
- No Android emulator or physical-device proof was run because `adb devices -l`
  reported no attached devices.
- No macOS or iOS local proof was run because this worker is Windows and
  `xcodebuild` is not available.
- No two-device LAN proof was run because the pass did not have a second real
  parent/child device and did not perform pairing.

## Proof Matrix Handling

Do not update `docs/expectations/pre-ai-proof-matrix.json` from this record.
The current proof is strong enough to confirm shared validation, latest main CI,
package-preview mechanics, WSL structural pre-AI proof, and local Windows host
metadata. It is not enough to upgrade privileged OS, package lifecycle, real
household LAN, mobile device, signing, store, entitlement, model execution, or
enforcement claims.

## Known Gaps And Risks

- CI/package-preview mechanics are green on latest `main`, but real OS/device
  proof remains incomplete for Windows package lifecycle, macOS permissions,
  Android device behavior, iOS entitlements/TestFlight, and production signing.
- WSL exists and can run the structural pre-AI proof check, but this pass did
  not run Linux package install, service launch, journal/SQLite filesystem
  checks, or a full Linux validation gate.
- Android tooling exists, but no attached emulator or physical device was
  available for local proof.
- The current proof records still preserve `not-yet-proven` states for managed
  browser URL/title through the service path, timed app/game duration, screen
  queue status, and full network/domain attribution.
- Two-device LAN pairing and failed-unpaired request proof remain
  manual-required.

## Roadmap Slice

V0.7 pre-AI/enforcement cross-platform checkpoint proof/status pass. This record
keeps the project at a reviewable checkpoint before more AI, model execution, or
V0.8 enforcement work resumes.

## PR Body Outline

```text
Scope
- Added a dated cross-platform checkpoint proof/status record for latest main at cebc2a1.
- Recorded fresh Windows-local validation, WSL structural pre-AI proof, host/network metadata, and current green main CI/package-preview evidence.
- Separated CI-mechanical proof from real OS/device, package lifecycle, LAN, signing, store, entitlement, model execution, and enforcement proof.
- Kept scope docs/evidence/status only; no portal UI, product code, proof-matrix upgrade, V0.8 enforcement, or model execution.

Touched files
- docs/architecture/cross-platform-checkpoint-proof-2026-05-23.md

Validation
- cmd /c npm run format:check
- cmd /c npm run test:pre-ai-proof
- cmd /c npm run validate
- wsl.exe -d Ubuntu-22.04 -- bash -lc "cd ... && npm run test:pre-ai-proof"
- git diff --check
- cmd /c npm run lanes:guard
- cmd /c npm run hub:guard

Known gaps and risks
- Real OS/device proof is still manual-required for Windows package lifecycle, macOS permissions/signing/notarization, Android device behavior, iOS device/TestFlight/entitlements, production signing/store paths, two-device LAN pairing, and reboot/autostart/uninstall checks.
- Docker was unavailable on PATH, Android had no attached device, and this Windows worker cannot run Xcode/macOS/iOS local proof.
- No model execution or enforcement was run or claimed.

Roadmap slice
- V0.7 cross-platform deliverables checkpoint proof/status pass before further AI or enforcement work.
```

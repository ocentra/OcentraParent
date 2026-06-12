<!-- agent-capsule -->

> Agent Capsule
> Doc: V0.7 CI Checkpoint Evidence Refresh After PR96
> Kind: historical checkpoint/proof documentation; read only when CHECKPOINT_INDEX or PROOF_INDEX names it.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

# V0.7 CI Checkpoint Evidence Refresh After PR96

## Scope

This record captures current-main CI and package-preview mechanical proof after
PR #96. It is a docs/evidence artifact only. It does not add product behavior,
portal UI work, package scripts, workflow changes, proof-matrix upgrades,
roadmap reconciliation, enforcement, model execution, or production release
claims.

The proof boundary stays narrow: GitHub Actions proves repeatable build,
validation, E2E, scan, dependency, SBOM, and preview-package mechanics for the
named commit. Real OS, device, household LAN, signing, store, entitlement,
autostart, reboot, update, uninstall, and physical-device behavior remains
manual-required or not-yet-proven unless a separate manual proof record exists.

## Run Metadata

| Field               | Value                                                                                                            |
| ------------------- | ---------------------------------------------------------------------------------------------------------------- |
| Proof date          | 2026-05-25                                                                                                       |
| Worktree            | `C:\Users\sujan\.codex\worktrees\ocentra-parent-codex-b\OcentraParent`                                           |
| Branch              | `codex/v0.7-ci-checkpoint-evidence-refresh`                                                                      |
| Baseline commit     | `98eaf55b9b8507992cc076fe612e2194de8c90eb`                                                                       |
| Baseline subject    | `Make precommit gate fast and opt-in full validation (#96)`                                                      |
| Package/app version | `0.1.1`                                                                                                          |
| GitHub Actions run  | <https://github.com/ocentra/OcentraParent/actions/runs/26401270250>                                              |
| Workflow            | `CI Gate`                                                                                                        |
| Run status          | `completed`                                                                                                      |
| Run conclusion      | `success`                                                                                                        |
| Run head branch     | `main`                                                                                                           |
| Created UTC         | 2026-05-25 12:47:23 UTC                                                                                          |
| Updated UTC         | 2026-05-25 13:02:31 UTC                                                                                          |
| Sensitive data      | No child activity, screenshots, private browser history, decrypted evidence, local filesystem paths, or secrets. |

## Source Inputs

- `AGENTS.md`
- `.ocentra-ai/rules/ocentra-parent-rules.mdc`
- `.ocentra-ai/rules/ocentra-parent-validation.mdc`
- `docs/architecture/worktree-lanes.md`
- `docs/architecture/primary-coordinator-reminder.md`
- `docs/product-roadmap.md`
- `docs/architecture/cross-platform-deliverables-checkpoint.md`
- GitHub Actions CI Gate run:
  <https://github.com/ocentra/OcentraParent/actions/runs/26401270250>
- `gh run view 26401270250 --repo ocentra/OcentraParent --json ...`
- `gh api repos/ocentra/OcentraParent/actions/runs/26401270250/artifacts`

## CI Job Ledger

| Required evidence area | Job name                                               | Result    | Mechanical proof recorded                                                                  |
| ---------------------- | ------------------------------------------------------ | --------- | ------------------------------------------------------------------------------------------ |
| Fail-fast gate         | `fail-fast / Format, Lint, Types, Rust Check`          | `success` | Format check, release-version policy, lint, TypeScript type-check, and Rust check passed.  |
| Secret scan            | `secret-scan / Secrets and Sensitive Files`            | `success` | Repository secret scanner and Gitleaks passed.                                             |
| Dependency policy      | `dependency-policy / Dependency Audit, Licenses, SBOM` | `success` | Dependency/license policy, cargo audit setup, SBOM metadata write, and SBOM upload passed. |
| Pre-AI proof matrix    | `validate / Pre-AI Proof Matrix`                       | `success` | Pre-AI proof matrix check passed.                                                          |
| Production build       | `build / Production Build`                             | `success` | Production build gate passed.                                                              |
| Full validation        | `validate / Full Validation Gate`                      | `success` | Full validation gate passed in CI after Playwright Chromium install.                       |
| Real portal-to-Rust    | `validate / Real Portal To Rust E2E (ubuntu-latest)`   | `success` | Hosted Ubuntu real portal-to-Rust E2E passed against the real Rust service.                |
| Real portal-to-Rust    | `validate / Real Portal To Rust E2E (windows-latest)`  | `success` | Hosted Windows real portal-to-Rust E2E passed against the real Rust service.               |
| Real portal-to-Rust    | `validate / Real Portal To Rust E2E (macos-latest)`    | `success` | Hosted macOS real portal-to-Rust E2E passed against the real Rust service.                 |
| Package preview        | `package-preview / Windows MSI Preview`                | `success` | Windows MSI build, CI install/uninstall smoke, and artifact upload passed.                 |
| Package preview        | `package-preview / Linux DEB Preview`                  | `success` | Linux DEB build, CI install/remove smoke, and artifact upload passed.                      |
| Package preview        | `package-preview / macOS PKG Preview`                  | `success` | macOS PKG build, payload smoke, and artifact upload passed.                                |
| Package preview        | `package-preview / Android APK Preview`                | `success` | Android APK build, emulator install/launch smoke, and artifact upload passed.              |
| Package preview        | `package-preview / iOS Simulator App Preview`          | `success` | iOS simulator app build, simulator install/launch smoke, and artifact upload passed.       |

## Uploaded Artifact Ledger

| Artifact name                          | Size in bytes | Digest                                                                    | Created UTC             | Expired | Proof label           |
| -------------------------------------- | ------------: | ------------------------------------------------------------------------- | ----------------------- | ------- | --------------------- |
| `ocentra-parent-windows-x64-preview`   |    19,040,098 | `sha256:24441854d39ec259924c3b4afafeeecfb0bd4a6196682e5ac686f1970a8e525f` | 2026-05-25 13:02:26 UTC | `false` | `ci-mechanical-proof` |
| `ocentra-parent-linux-amd64-preview`   |     4,250,757 | `sha256:69b1ca3bf44baa5b84c342dce4f7992ebacf725e99ef349e8558885d60d7717c` | 2026-05-25 12:59:13 UTC | `false` | `ci-mechanical-proof` |
| `ocentra-parent-ios-simulator-preview` |        86,155 | `sha256:428fdd69f7449bd4f98ba9575fd5e29abe3c0d80bba58974a8c0edfe02504456` | 2026-05-25 12:59:04 UTC | `false` | `ci-mechanical-proof` |
| `ocentra-parent-android-preview`       |        11,832 | `sha256:d6e2378d4e12222c4a86a5e419bd06efc5fb2908720da248a6fc7d8938b7ba8b` | 2026-05-25 12:59:03 UTC | `false` | `ci-mechanical-proof` |
| `ocentra-parent-macos-preview`         |     4,560,883 | `sha256:dbcdac69c19bf8d54091f616ca289c06da62218ec9d183bff491befa588903c8` | 2026-05-25 12:58:41 UTC | `false` | `ci-mechanical-proof` |
| `ocentra-parent-security-sbom`         |       176,725 | `sha256:30ea993475c4542d59b5b3383d1b1377b4a8055abe888d5a53f8655d71504487` | 2026-05-25 12:54:57 UTC | `false` | `ci-mechanical-proof` |

Artifacts prove preview build/upload and hosted smoke mechanics only. They are
not production release assets, signed distribution proof, notarized packages,
store submissions, TestFlight proof, managed-device proof, or reboot/autostart
proof.

## Platform Proof Boundary

| Platform or area                  | CI mechanical proof from run 26401270250                                                                  | Manual proof state to preserve                                                                                                                                                                                                               |
| --------------------------------- | --------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Shared TypeScript/Rust validation | Fail-fast, full validation, production build, and pre-AI proof matrix passed on current `main`.           | Keep privileged OS/device behavior separate from shared validation mechanics.                                                                                                                                                                |
| Portal-to-Rust E2E                | Real portal-to-Rust E2E passed on hosted Ubuntu, Windows, and macOS runners.                              | Hosted E2E does not prove installed-service lifecycle, household LAN, real child-PC permissions, or platform capture adapters.                                                                                                               |
| Windows package preview           | MSI build plus hosted install/uninstall smoke passed.                                                     | Real elevated install, service autostart, reboot survival, update, uninstall data behavior, production signing, and local portal against installed service remain manual-required.                                                           |
| Linux package preview             | DEB build plus hosted install/remove smoke passed.                                                        | WSL/Docker package launch, service-manager behavior, filesystem semantics, desktop capture, network visibility, and enforcement remain manual-required or not-yet-proven.                                                                    |
| macOS package preview             | PKG build plus hosted payload smoke passed.                                                               | Mac host launch, launchd behavior, signing, notarization, permissions, stores, and platform adapter behavior remain manual-required or not-yet-proven.                                                                                       |
| Android package preview           | APK build plus hosted emulator install/launch smoke passed.                                               | Physical-device behavior, foreground-service durability, notification permission, UsageStats, accessibility, VPN/DNS, device-owner, and managed-profile proof remains manual-required or not-yet-proven.                                     |
| iOS simulator preview             | Simulator app build plus hosted simulator install/launch smoke passed.                                    | TestFlight, device install, provisioning, Family Controls, DeviceActivity, Screen Time, Network Extension, notifications, background execution, and entitlement proof remains manual-required, permission-required, blocked, or unavailable. |
| LAN and multi-device control      | CI continues to prove local transport and real-service mechanics where tests cover them.                  | Real two-device household LAN, pairing, failed-unpaired request, firewall/router behavior, route selection, and production LAN authentication remain manual-required.                                                                        |
| Security and dependency posture   | Secret scan, Gitleaks, dependency/license policy, SBOM metadata generation, and SBOM upload passed in CI. | This does not replace threat-model review, production signing, store review, entitlement review, or real-device abuse-resistance proof.                                                                                                      |

## Remaining Proof Workstreams

Use these workstreams to turn the CI package into real proof records. Do not
change proof labels to `implemented` until the owner has attached command
output, logs, UI screenshots or copied diagnostics, artifact names, platform
metadata, permission state, and observed results from the named current-main
commit or a later reviewed commit.

| Workstream                     | Owner-ready next step                                                                                                                                         | Evidence to attach                                                                                                                     | Current label to preserve                               |
| ------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------- |
| CI traceability                | Confirm run `26401270250` remains accessible and artifact digests match this record before PR review.                                                         | `gh run view` JSON, artifact list with digests, job URLs, and conclusion `success`.                                                    | `ci-mechanical-proof`                                   |
| Windows controlled evidence    | Run the current V0.7 evidence preview path on a real Windows child PC with synthetic low-sensitivity activity.                                                | Host metadata, service health, portal diagnostic output, evidence read-model output, and permission/degraded states.                   | `manual-required` for privileged evidence               |
| Windows package lifecycle      | Download the Windows MSI preview artifact, install it with logging, inspect services, reboot, re-check autostart, uninstall, and record data-retention state. | Install/uninstall logs, `Get-Service` output, reboot notes, process cleanup, portal-against-installed-service result, and data notes.  | `manual-required` for installed lifecycle proof         |
| LAN two-device checkpoint      | Run one child host and one parent host on the same LAN; record a reachable paired path and a failed unpaired or wrong-origin path.                            | Parent/child OS and IP range, ports, allowed origin, pairing step or explicit absence, service logs, portal output, negative request.  | `manual-required` for household LAN proof               |
| Linux WSL or Docker            | Repeat shared validation and service/package smoke in WSL or Docker, then keep capture/network/enforcement states unavailable or not-yet-proven as observed.  | WSL/Docker metadata, package install/remove or service launch output, journal/SQLite path notes, unsupported capability status.        | `manual-required` or `not-yet-proven` for Linux runtime |
| macOS host                     | Download the macOS PKG preview on a Mac, inspect/install it, record launchd/signing/notarization/permission states, and run loopback service proof if wired.  | `pkgutil`, `installer`, `launchctl`, `spctl`, permission screenshots or copied settings, service/portal status output.                 | `manual-required` for real Mac behavior                 |
| Android emulator and device    | Install the APK on emulator and then a physical device when available; keep parent-app and child-agent claims separate.                                       | `adb devices`, APK install output, launch output, permission state, foreground/background notes, package id `ca.ocentra.parent.agent`. | `ci-mechanical-proof` for emulator; device manual gap   |
| iOS simulator and entitlements | Run simulator proof on a Mac and separately record provisioning, TestFlight, device, and entitlement availability for `ca.ocentra.parent.agent`.              | `xcodebuild`, `simctl`, bundle id, signing team, provisioning/TestFlight status, entitlement notes.                                    | `ci-mechanical-proof` for simulator; entitlement gap    |

## Command Seeds For Remaining Proof

These commands are starting points for the next proof owners. They are not
recorded as run by this branch.

Shared checkpoint preflight:

```powershell
git fetch origin main
git switch --detach 98eaf55b9b8507992cc076fe612e2194de8c90eb
git status --short --branch
cmd /c npm ci
cmd /c npm run format:check
cmd /c npm run test:pre-ai-proof
cmd /c npm run validate
gh run view 26401270250 --repo ocentra/OcentraParent --json databaseId,headSha,headBranch,status,conclusion,jobs,url
gh api repos/ocentra/OcentraParent/actions/runs/26401270250/artifacts
```

Windows local service and portal proof:

```powershell
$env:OCENTRA_PARENT_AGENT_PORT = "4677"
$env:OCENTRA_PARENT_PORTAL_PORT = "4678"
cmd /c npm run dev:agent
```

```powershell
$env:OCENTRA_PARENT_AGENT_PORT = "4677"
$env:OCENTRA_PARENT_PORTAL_PORT = "4678"
cmd /c npm run dev:portal
```

```powershell
curl.exe -i http://127.0.0.1:4677/health
Start-Process "http://127.0.0.1:4678/#/commands"
```

Windows MSI lifecycle proof:

```powershell
New-Item -ItemType Directory -Force artifacts\pr96\windows
gh run download 26401270250 --repo ocentra/OcentraParent --name ocentra-parent-windows-x64-preview --dir artifacts\pr96\windows
msiexec /i artifacts\pr96\windows\*.msi /qn /norestart /l*v artifacts\pr96\windows\install.log
Get-Service OcentraParentAgent,OcentraParentUpdater
Restart-Computer
Get-Service OcentraParentAgent,OcentraParentUpdater
msiexec /x artifacts\pr96\windows\*.msi /qn /norestart /l*v artifacts\pr96\windows\uninstall.log
Get-Service OcentraParentAgent,OcentraParentUpdater
```

LAN parent-to-child proof:

```powershell
$env:OCENTRA_PARENT_DEV_NETWORK = "lan"
$env:OCENTRA_PARENT_LAN_HOST = "<child-lan-ip>"
$env:OCENTRA_PARENT_AGENT_PORT = "4677"
$env:OCENTRA_PARENT_PORTAL_PORT = "4678"
cmd /c npm run dev:lan
```

```powershell
curl.exe -i --max-time 10 http://<child-lan-ip>:4677/health
curl.exe -i --max-time 5 http://<child-lan-ip>:4679/health
curl.exe -i -H "Origin: http://example.invalid" --max-time 10 http://<child-lan-ip>:4677/health
Start-Process "http://<child-lan-ip>:4678/#/commands"
```

Linux WSL or Docker proof:

```powershell
wsl.exe --list --verbose
wsl.exe -d Ubuntu-22.04 -- bash -lc "cd /mnt/c/Users/sujan/.codex/worktrees/ocentra-parent-codex-b/OcentraParent && git status --short --branch && npm run test:pre-ai-proof"
gh run download 26401270250 --repo ocentra/OcentraParent --name ocentra-parent-linux-amd64-preview --dir artifacts/pr96/linux
```

macOS host proof:

```bash
gh run download 26401270250 --repo ocentra/OcentraParent --name ocentra-parent-macos-preview --dir artifacts/pr96/macos
pkgutil --expand artifacts/pr96/macos/*.pkg artifacts/pr96/macos/pkg-expanded
sudo installer -pkg artifacts/pr96/macos/*.pkg -target /
launchctl print system/com.ocentra.parent.agent
spctl --assess --verbose artifacts/pr96/macos/*.pkg
```

Android emulator or device proof:

```powershell
gh run download 26401270250 --repo ocentra/OcentraParent --name ocentra-parent-android-preview --dir artifacts\pr96\android
adb devices
adb install -r artifacts\pr96\android\*.apk
adb shell monkey -p ca.ocentra.parent.agent 1
adb shell dumpsys package ca.ocentra.parent.agent
```

iOS simulator proof and entitlement notes:

```bash
gh run download 26401270250 --repo ocentra/OcentraParent --name ocentra-parent-ios-simulator-preview --dir artifacts/pr96/ios
xcrun simctl list devices available
xcrun simctl boot "iPhone 16" || true
xcrun simctl install booted artifacts/pr96/ios/*.app
xcrun simctl launch booted ca.ocentra.parent.agent
codesign -d --entitlements :- artifacts/pr96/ios/*.app
```

## Explicit Non-Claims

- No `docs/expectations/pre-ai-proof-matrix.json` update is made from this
  record.
- No `docs/product-roadmap.md` update is made from this record.
- No A-owned roadmap/proof reconciliation file is edited by this record.
- No C-owned portal, portal-domain, or vendor files are edited by this record.
- No package-preview artifact was downloaded or installed locally in this pass.
- No local Windows MSI install, elevated service check, reboot, autostart,
  update, uninstall, installed-service data retention, or local portal against
  installed service proof was run.
- No real two-device LAN pairing, firewall/router behavior, paired request, or
  failed unpaired request was run.
- No macOS host install/signing/notarization, Android physical-device, iOS
  TestFlight/device, store, entitlement, or managed-device proof was run.

## Known Gaps And Risks

- Run `26401270250` is a current-main CI success after PR #96, but it remains
  CI-mechanical evidence. It must not upgrade product support claims by itself.
- Package previews prove build and hosted smoke mechanics only; real package
  lifecycle proof remains manual-required per platform.
- Hosted real portal-to-Rust E2E proves the service path can run on CI runners;
  it does not prove privileged OS capture, local household behavior, service
  autostart, LAN router/firewall behavior, or mobile policy behavior.
- Manual checkpoint proof still needs real hosts/devices for Windows, Linux
  WSL/Docker, macOS, Android emulator plus device, iOS simulator plus
  TestFlight/device/entitlement notes, LAN paired/unpaired checks, and package
  lifecycle checks.

## Roadmap Slice

V0.7 current-main checkpoint CI/package-preview evidence refresh after PR #96.
This record makes the current CI mechanical proof reviewable while preserving
manual-required and not-yet-proven labels for OS/device/platform support.

## PR Body Outline

```text
Scope
- Expanded the V0.7 checkpoint evidence package after PR #96.
- Recorded current-main CI Gate run 26401270250 for commit 98eaf55 across fail-fast, secret scan, dependency/SBOM, pre-AI proof, full validation, real portal-to-Rust E2E, and package-preview jobs.
- Added owner-ready remaining proof workstreams and command seeds for Windows, LAN, Linux, macOS, Android, iOS, and package lifecycle checks.
- Updated the V0.7 cross-platform gap tracker to point at the PR96 CI evidence package without upgrading manual proof claims.
- Kept scope docs/evidence only; no portal, runtime, workflow, package script, roadmap, proof-matrix, A-owned current-main proof, or C-owned path changes.

Touched files
- docs/checkpoints/v0-7-ci-checkpoint-evidence-refresh-2026-05-25.md
- docs/architecture/v07-cross-platform-proof-gap-tracker.md

Validation
- cmd /c npm run format:check
- cmd /c npm run test:pre-ai-proof
- node --test scripts/test/platform-packaging.test.mjs scripts/test/release-windows-assets.test.mjs scripts/test/workflow-ci-trigger.test.mjs
- git diff --check
- cmd /c npm run lanes:guard
- cmd /c npm run hub:guard

Known gaps and risks
- CI and package-preview evidence remains CI-mechanical proof only.
- Real Windows, LAN, Linux WSL/Docker, macOS, Android physical-device, iOS entitlement/TestFlight, package lifecycle, signing, store, reboot, and autostart proof remains manual-required or not-yet-proven.

Roadmap slice
- V0.7 current-main CI/package/manual-gap evidence package after PR #96.
```

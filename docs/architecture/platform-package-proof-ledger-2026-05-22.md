<!-- agent-capsule -->

> Agent Capsule
> Doc: Platform Package Proof Ledger - 2026-05-22
> Kind: architecture/reference documentation; read only when selected by plan route, source router, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

# Platform Package Proof Ledger - 2026-05-22

This ledger records the current CI and package-preview proof state for the
pre-AI cross-platform checkpoint. It is a proof inventory, not a support-claim
upgrade. It separates hosted CI mechanics from real OS, household LAN, mobile
device, signing, store, entitlement, permission, reboot, and autostart proof.

## Source Inputs

- Cross-platform checkpoint:
  `docs/architecture/cross-platform-deliverables-checkpoint.md`
- Validation gate definition:
  `docs/architecture/validation-gates.md`
- Current GitHub Actions state captured from `ocentra/OcentraParent` at
  2026-05-22 18:56 UTC.

The checkpoint labels remain authoritative for this record:
`ci-mechanical-proof`, `manual-required`, `permission-required`, `disabled`,
`degraded`, `unavailable`, `scaffold-only`, `not-yet-proven`, `blocked`, and
`not-applicable`.

## GitHub Actions Snapshot

| Field               | Value                                                                                  |
| ------------------- | -------------------------------------------------------------------------------------- |
| Baseline branch     | `main`                                                                                 |
| Baseline commit     | `7d110b67661ad4c9b42fc7b3237aa698f51f41df` (`7d110b6`)                                 |
| Commit title        | `Tighten cross-platform proof runbook`                                                 |
| Baseline run        | CI Gate run `26304936088`                                                              |
| Baseline run URL    | <https://github.com/ocentra/OcentraParent/actions/runs/26304936088>                    |
| Baseline conclusion | `success`                                                                              |
| Run window          | Created 2026-05-22 18:25:04 UTC; completed 2026-05-22 18:36:14 UTC                     |
| Current caveat      | A later pull-request CI Gate for `codex/parent-portal-product-shell` failed; it is not |
|                     | counted as `main` platform proof in this ledger.                                       |

## Baseline CI Job Ledger

All jobs below belong to CI Gate run `26304936088` on `main` at commit
`7d110b67661ad4c9b42fc7b3237aa698f51f41df`.

| Job name                                               | Result    | CI-mechanical proof recorded                                                                 |
| ------------------------------------------------------ | --------- | -------------------------------------------------------------------------------------------- |
| `fail-fast / Format, Lint, Types, Rust Check`          | `success` | Formatting, release version policy, package lint, TypeScript type-check, and Rust check ran. |
| `secret-scan / Secrets and Sensitive Files`            | `success` | Repository secret scan and Gitleaks ran.                                                     |
| `build / Production Build`                             | `success` | Portal/package build gate completed.                                                         |
| `dependency-policy / Dependency Audit, Licenses, SBOM` | `success` | Dependency policy, license policy, cargo audit, and SBOM metadata upload completed.          |
| `validate / Pre-AI Proof Matrix`                       | `success` | Pre-AI proof matrix check completed without upgrading claims here.                           |
| `validate / Full Validation Gate`                      | `success` | Root validation gate completed, including local service, transport, Rust, and portal checks. |
| `validate / Real Portal To Rust E2E (ubuntu-latest)`   | `success` | Hosted Linux portal-to-Rust E2E path completed against the real Rust service.                |
| `validate / Real Portal To Rust E2E (macos-latest)`    | `success` | Hosted macOS portal-to-Rust E2E path completed against the real Rust service.                |
| `validate / Real Portal To Rust E2E (windows-latest)`  | `success` | Hosted Windows portal-to-Rust E2E path completed against the real Rust service.              |
| `package-preview / macOS PKG Preview`                  | `success` | macOS PKG build, payload smoke, and artifact upload completed.                               |
| `package-preview / Windows MSI Preview`                | `success` | Windows MSI build, CI install/uninstall smoke, and artifact upload completed.                |
| `package-preview / Linux DEB Preview`                  | `success` | Linux DEB build, CI install/remove smoke, and artifact upload completed.                     |
| `package-preview / iOS Simulator App Preview`          | `success` | iOS simulator app build, simulator install/launch smoke, and artifact upload completed.      |
| `package-preview / Android APK Preview`                | `success` | Android APK build, emulator install/launch smoke, and artifact upload completed.             |

## Uploaded Artifact Ledger

Artifacts were captured from CI Gate run `26304936088`. They were not expired
at the time of inspection.

| Artifact name                          | Size in bytes | Created UTC             | Proof level           |
| -------------------------------------- | ------------- | ----------------------- | --------------------- |
| `ocentra-parent-windows-x64-preview`   | 18,954,601    | 2026-05-22 18:36:10 UTC | `ci-mechanical-proof` |
| `ocentra-parent-linux-amd64-preview`   | 4,153,239     | 2026-05-22 18:32:45 UTC | `ci-mechanical-proof` |
| `ocentra-parent-macos-preview`         | 4,433,297     | 2026-05-22 18:31:53 UTC | `ci-mechanical-proof` |
| `ocentra-parent-android-preview`       | 11,826        | 2026-05-22 18:32:36 UTC | `ci-mechanical-proof` |
| `ocentra-parent-ios-simulator-preview` | 86,157        | 2026-05-22 18:33:25 UTC | `ci-mechanical-proof` |
| `ocentra-parent-security-sbom`         | 166,615       | 2026-05-22 18:29:58 UTC | `ci-mechanical-proof` |

Preview artifacts are CI build outputs for testing. They are not production
release assets, app-store submissions, TestFlight proof, notarized packages,
trusted updater manifests, or signed distribution proof.

## What CI Proves Today

Current CI proves repeatable scaffold mechanics for the reviewed commit:

- format, lint, type-check, Rust check, secret scan, dependency policy, license
  policy, SBOM metadata, build, and full validation gates;
- the pre-AI proof matrix still accepts the current claim registry;
- real portal-to-Rust E2E mechanics work on hosted Windows, Linux, and macOS
  runners;
- Windows MSI, Linux DEB, macOS PKG, Android APK, and iOS simulator preview
  artifacts can be built and smoke-checked in the hosted CI paths named above;
- CI package-preview smoke checks can prove build, install, remove, payload, or
  launch mechanics where the runner supports that operation.

CI does not prove privileged household behavior, real child-device observation,
production distribution, store review, signing trust, permission grants,
notarization, TestFlight availability, reboot survival, or real two-device LAN
pairing.

## Platform Proof Ledger

| Platform or area                                | Current CI/package-preview proof                                                                                                         | Required manual or external proof                                                                                                                                                                              | Current label                                                                                                          |
| ----------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------- |
| Windows local PC                                | Hosted Windows E2E, MSI build, and CI MSI install/uninstall smoke passed.                                                                | Real Windows child PC install, service behavior after reboot, autostart, uninstall/data retention, foreground app/window, browser, network, screen, and dry-run evidence proof.                                | `ci-mechanical-proof`; privileged child-device claims remain `manual-required` or `not-yet-proven`.                    |
| Linux runner                                    | Hosted Ubuntu E2E, DEB build, and CI install/remove smoke passed.                                                                        | WSL or Docker repeat proof, service-manager behavior, filesystem/data path notes, and explicit unavailable or not-yet-proven labels for unsupported capture/network/enforcement adapters.                      | `ci-mechanical-proof`; platform adapters remain `not-yet-proven` or `unavailable`.                                     |
| macOS runner                                    | Hosted macOS E2E, PKG build, payload smoke, and artifact upload passed.                                                                  | Real Mac launch, launchd behavior, Screen Recording, Accessibility, notifications, signing, notarization, and permission-state evidence.                                                                       | `ci-mechanical-proof`; OS permissions and distribution remain `manual-required` or `permission-required`.              |
| Android emulator                                | APK build, emulator install/launch smoke, and artifact upload passed.                                                                    | Real device proof, notification permission state, foreground-service durability, UsageStats, accessibility, VPN/DNS, device-owner, and managed-profile evidence.                                               | `ci-mechanical-proof` for emulator mechanics; child-agent platform claims remain `scaffold-only` or `not-yet-proven`.  |
| Android physical device                         | No physical-device proof is produced by the current CI run.                                                                              | Install and launch on a real device, then capture foreground-service, permission, notification, managed-profile, or device-owner behavior only when implemented.                                               | `manual-required`, `permission-required`, `scaffold-only`, or `not-yet-proven` by capability.                          |
| iOS simulator                                   | Simulator app build, install/launch smoke, and artifact upload passed.                                                                   | Xcode/simulator evidence can cover only scaffold mechanics unless product paths exist; physical device, TestFlight, signing, and entitlement notes remain separate.                                            | `ci-mechanical-proof` for simulator mechanics; iOS product capabilities remain `scaffold-only` or `not-yet-proven`.    |
| iOS device, TestFlight, entitlements            | No CI proof covers TestFlight, device execution, Family Controls, DeviceActivity, Screen Time, Network Extension, or entitlement review. | Apple signing, provisioning, TestFlight, entitlement approval, and device evidence.                                                                                                                            | `manual-required`, `permission-required`, `blocked`, or `unavailable` until credentials and entitlements exist.        |
| LAN parent-to-child                             | Validation can prove local service transport and LAN bind/origin mechanics in controlled smoke paths.                                    | Real two-device household LAN, firewall/router behavior, explicit pairing, rejected unpaired request, trusted-device registry, and parent-to-child evidence.                                                   | `ci-mechanical-proof` for mechanics; pairing remains `scaffold-only`; household LAN remains `manual-required`.         |
| Installer, autostart, update, reboot, uninstall | Package-preview jobs prove selected build/install/remove/payload/launch smoke mechanics.                                                 | Real installed-artifact lifecycle proof for start-on-install, service registration, autostart after reboot, update behavior, uninstall cleanup, data retention, signing, notarization, and store distribution. | `ci-mechanical-proof` for preview mechanics; production lifecycle claims remain `manual-required` or `not-yet-proven`. |
| Security and release metadata                   | Secret scan, dependency policy, license policy, cargo audit, and SBOM metadata upload passed.                                            | Production release signing secrets, trusted update manifest publication, and release-channel verification on `production`.                                                                                     | `ci-mechanical-proof`; production release trust remains `manual-required` until release credentials are wired.         |

## Unsupported Or Scaffold-Only States To Preserve

Do not upgrade these from this ledger alone:

- V0.8 enforcement, blocking, timers, rollback, app control, notification
  delivery, or policy execution;
- local model execution and model decisioning beyond current dry-run or
  unavailable/probe status;
- production signing, macOS notarization, app-store distribution, TestFlight,
  managed-device enrollment, Android device-owner policy, and iOS Family
  Controls or Screen Time entitlements;
- real child-device foreground process/window, browser URL/tab, app/game
  duration, screen analysis, or network/domain observation without real
  OS/device proof;
- two-device household LAN pairing, trusted-device registry, remote control,
  cloud relay, sync/export, or notification delivery.

## Proof Matrix Handling

This ledger must not change `docs/expectations/pre-ai-proof-matrix.json`.
Future proof-matrix updates need evidence records that name the exact commit,
platform, command or UI action, logs or screenshots, proof owner, and pass/fail
result. A green CI Gate or package-preview artifact can justify
`ci-mechanical-proof`; it cannot justify `implemented` for privileged OS,
household LAN, signing, store, entitlement, or mobile device behavior.

## Next Review Actions

- Use this ledger as the CI/package-preview baseline when planning the next
  manual platform pass.
- Attach manual proof records before changing any platform row from
  `manual-required`, `not-yet-proven`, `permission-required`, `blocked`, or
  `scaffold-only`.
- Keep PR bodies explicit about CI mechanics versus real OS/device proof.
- Re-capture this ledger after any new `main` CI Gate that changes package
  preview jobs, validation gates, release packaging, mobile scaffolds, or
  platform proof expectations.

## PR Body Outline

```text
Scope
- Added a dated platform package proof ledger for the current main CI Gate.
- Recorded current CI and package-preview job results and uploaded artifacts.
- Separated CI-mechanical proof from real OS/device, signing, store,
  entitlement, LAN, lifecycle, and privileged behavior proof.
- Preserved unsupported, manual-required, permission-required, scaffold-only,
  and not-yet-proven states; no product behavior or proof matrix upgrade.

Touched files
- docs/architecture/platform-package-proof-ledger-2026-05-22.md

Validation
- cmd /c npm run format:check
- cmd /c npm run test:pre-ai-proof
- cmd /c npm run lanes:guard
- cmd /c npm run hub:guard

Known gaps and risks
- This ledger captures CI/package-preview state only.
- Real OS/device proof still requires Windows, Linux/WSL-Docker, macOS,
  Android device, iOS device/TestFlight, LAN, signing, store, permission,
  autostart, reboot, update, and uninstall evidence before claims are upgraded.

Roadmap slice
- V0.7 pre-AI/enforcement cross-platform CI/package-preview proof baseline.
```

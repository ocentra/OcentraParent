# V0.7 Cross-Platform Proof Gap Tracker

This tracker turns the V0.7 cross-platform checkpoint into reviewable gap
state. It does not add product behavior, enforcement, model execution, portal
shell work, or proof artifacts. It records what current CI and package preview
can honestly prove, what still needs manual platform evidence, which claims
must stay scaffold-only or blocked, and who owns the next follow-up.

## Source Of Truth

- Product sequence:
  `docs/product-roadmap.md`
- Checkpoint plan:
  `docs/architecture/cross-platform-deliverables-checkpoint.md`
- Local/LAN manual runbook:
  `docs/architecture/local-lan-manual-proof-runbook.md`
- Runtime proof standard:
  `docs/expectations/real-evidence-proof.md`
- Current proof matrix:
  `docs/expectations/pre-ai-proof-matrix.json`
- Platform expectations:
  `docs/expectations/platform-deliverables.md` and
  `docs/expectations/platforms.md`

The current proof matrix is the claim registry. This tracker must not upgrade a
claim until the proof matrix and supporting evidence artifacts are updated from
real command output, logs, UI proof, package evidence, or device evidence.

## Current Proof Baseline

`main` has mechanical proof for the shared scaffold and V0.7 dry-run preview
path: TypeScript/Rust contracts, local Rust service mechanics, local transport,
portal request/render mechanics, SQLite/journal-backed read paths, and package
preview scaffolds. Those are CI or package-preview signals, not privileged
platform proof.

The current matrix still marks real OS/device behaviors as manual or not yet
proven:

| Claim                        | Current proof level                                                       | Gap                                                                                                                  |
| ---------------------------- | ------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------- |
| V0.1 foundation              | CI mechanical proof across Windows, Linux, macOS, Android, and iOS        | Preview artifacts are not store, signing, notarization, or managed-device proof.                                     |
| V0.2 evidence store          | CI mechanical proof across Windows, Linux, macOS, Android, and iOS        | Mobile storage parity is package/runtime compatibility until mobile parent/child flows exist.                        |
| V0.3 Windows process/window  | Windows manual-required; other platforms not-applicable or not-yet-proven | Hosted CI cannot certify real foreground-window capture.                                                             |
| V0.4 Windows network/domain  | Windows manual-required; Linux/macOS/Android/iOS not-yet-proven           | Admin, kernel, driver, VPN, or OS-specific visibility needs real host/device proof.                                  |
| V0.5 live activity portal    | CI mechanical proof for desktop/web mechanics; Android/iOS scaffold-only  | Current Vite portal does not prove mobile parent-app parity.                                                         |
| V0.5.1 browser URL/tab       | Windows manual-required; other platforms not-yet-proven                   | Managed-browser proof must come from real managed profile evidence, not portal state.                                |
| V0.5.2 app/game sessions     | Windows manual-required; other platforms not-yet-proven                   | Real app/game duration needs host-specific observation over time.                                                    |
| V0.5.3 screen evidence queue | Windows and macOS manual-required; other platforms not-yet-proven         | Screen capture and permission prompts need real host proof.                                                          |
| V0.6 local AI contracts      | CI mechanical proof across platforms                                      | Contracts are not model execution or enforcement proof.                                                              |
| V0.7 dry-run policy preview  | CI mechanical proof on desktop/web mechanics; Android/iOS scaffold-only   | Dry-run preview does not prove enforcement adapters or real model execution.                                         |
| Package preview platforms    | CI mechanical proof across target families                                | Preview packaging is not production signing, app stores, TestFlight, notarization, device-owner, or autostart proof. |

## Platform Gap Tracker

| Platform or area                        | Current CI/package-preview proof                                                                                                                                   | Manual proof still required                                                                                                                                                                                                                                     | Current state label                                                                                                               | Follow-up owner                                                                                     |
| --------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------- |
| Windows local PC                        | Shared contracts, Rust service mechanics, portal transport, package preview, and pre-AI matrix checks are mechanically covered.                                    | Run the local Windows child-device pass: process/window, managed browser, network/domain, app/game duration, screen queue permission/degraded state, dry-run policy preview, installer, autostart, reboot, uninstall, and sensitive-detail-minimized artifacts. | `manual-required` for privileged evidence; `ci-mechanical-proof` for shared mechanics.                                            | A-owned manual proof pass for local/LAN artifacts; primary reviews evidence before matrix upgrades. |
| Linux CI                                | Shared TypeScript/Rust/package mechanics can be exercised by hosted runners.                                                                                       | Confirm package/install or launch smoke outputs and record unsupported desktop capture, network visibility, service-manager, and enforcement states separately. Use WSL or Docker where CI cannot represent local assumptions.                                  | `ci-mechanical-proof` for shared mechanics; `not-yet-proven` for platform adapters.                                               | Primary or Linux portability owner.                                                                 |
| Linux WSL/Docker                        | Can supplement CI for shared Rust/domain/package behavior and local service smoke.                                                                                 | Run the same current commit in WSL or Docker, capture service launch and package smoke evidence, and label desktop capture/network/enforcement as unavailable or not-yet-proven unless a Linux adapter exists.                                                  | `manual-required` for environment proof; `scaffold-only` or `not-yet-proven` for desktop capability claims.                       | Primary-assigned platform proof owner.                                                              |
| macOS                                   | CI/package preview can build shared mechanics where available.                                                                                                     | Use the Mac system for app launch, permission prompts, screen recording/accessibility notes, launchd/service behavior, signing/notarization state, and future iOS/Xcode artifacts.                                                                              | `ci-mechanical-proof` for shared mechanics; `manual-required` for permission and package behavior; `not-yet-proven` for adapters. | Primary-assigned Mac proof owner.                                                                   |
| Android emulator                        | Package preview or emulator checks can prove install/launch mechanics when wired.                                                                                  | Record emulator launch, SQLite/journal compatibility, foreground service assumptions, notification permission state, and parent-app versus child-agent scope separately.                                                                                        | `ci-mechanical-proof` or `scaffold-only` until emulator artifacts are attached.                                                   | Android platform proof owner.                                                                       |
| Android physical device                 | No current device-owner, managed-profile, VPN/DNS, accessibility, or foreground-service product claim is proven by CI.                                             | Run physical device package install/launch and permission checks before claiming child-agent support. Device-owner and managed-profile behavior need explicit real-device proof.                                                                                | `manual-required`, `permission-required`, `not-yet-proven`, or `scaffold-only` depending on the capability.                       | Android platform proof owner after coordinator assignment.                                          |
| iOS simulator                           | Simulator or package scaffolds can show build/launch mechanics only.                                                                                               | Use Mac/Xcode simulator output to record parent-app shell or scaffold behavior. Do not treat simulator launch as Family Controls, Screen Time, Network Extension, TestFlight, or entitlement proof.                                                             | `ci-mechanical-proof` or `scaffold-only` for mechanics; `not-yet-proven` for capabilities.                                        | Mac/iOS platform proof owner.                                                                       |
| iOS TestFlight and entitlements         | No current CI signal proves TestFlight, signing, Family Controls, Screen Time, Network Extension, notifications, background execution, or child-agent enforcement. | Record signing and entitlement availability, review state, and unsupported capability limits with exact Apple-approved API notes.                                                                                                                               | `manual-required`, `permission-required`, `blocked`, or `unavailable` until credentials and entitlements exist.                   | Mac/iOS platform proof owner with coordinator review.                                               |
| LAN parent-to-child                     | Current smoke can prove local service LAN bind/origin mechanics only.                                                                                              | A-owned local/LAN proof artifacts are pending. Run real two-device or clearly substituted LAN reachability, origin rejection, stale-service negative checks, and explicit pairing-gap notes.                                                                    | `ci-mechanical-proof` for LAN smoke; `manual-required` for real household LAN; `scaffold-only` for pairing.                       | A-owned manual proof pass; primary updates matrix only after artifacts land.                        |
| Installer, autostart, reboot, uninstall | Package-preview jobs can prove archive/build/install or launch smoke where supported.                                                                              | Exercise real installed artifacts on each platform before claiming service autostart, reboot survival, uninstall cleanup, update behavior, signing, notarization, store distribution, TestFlight, or device-owner mode.                                         | `scaffold-gap`, `manual-required`, or `not-yet-proven`.                                                                           | Release/package proof owner.                                                                        |

## A-Owned Pending Local/LAN Artifacts

The local/LAN manual runbook defines the artifact shape, but this tracker does
not consume that work as complete. The A-owned manual pass is still pending
until the lane produces evidence records with commit SHA, commands, device
details, service logs, portal proof, proof labels, known gaps, and follow-up
owners.

This file must stay independent of A's results file. Do not edit, pre-fill, or
normalize A's local/LAN proof results from this lane.

## Blocked Or Scaffold-Only Claims

Keep these labels unless a later branch adds the real product path and proof:

- V0.8 enforcement adapters, blocking, timers, app control, rollback, and
  enforcement audit behavior.
- Real local model execution and model decisioning beyond current dry-run and
  unavailable/probe status.
- Production signing, macOS notarization, app stores, TestFlight, and managed
  distribution.
- Android device-owner, managed-profile, accessibility, VPN/DNS, and foreground
  service behavior beyond any recorded package/emulator mechanics.
- iOS Family Controls, DeviceActivity, Screen Time, Network Extension,
  notifications, background execution, signing, entitlement, and review proof.
- V0.9 LAN pairing, trusted-device registry, authenticated remote control, and
  multi-device routing.
- V2 cloud relay, parent-owned storage sync/export, remote approvals, and
  cloud-routed reports.
- V3 notification delivery, provider retries, quiet hours, and escalation.

## Proof Matrix And Roadmap Follow-Up

Do not update `docs/expectations/pre-ai-proof-matrix.json` from this tracker
alone. Update the matrix only after proof artifacts exist for the exact commit
and platform under review.

Roadmap follow-up is needed after the manual checkpoint artifacts land:

- Link the proof results from the V0.7 checkpoint section if the coordinator
  accepts the evidence.
- Keep V0.8 enforcement blocked until Windows local and LAN checkpoint findings
  are reviewed.
- Add platform rows only when each row names CI proof, manual proof, scaffold
  gaps, unavailable states, and the owner of remaining gaps.

## Review Checklist

- Current branch and commit are named in the proof record.
- Every platform row separates CI/package mechanics from manual OS/device
  behavior.
- A-owned local/LAN artifact status remains pending until evidence lands.
- No portal shell, package runtime, product behavior, or enforcement path was
  changed by this tracker.
- No claim is upgraded without proof matrix and artifact updates.
- Every blocked or scaffold-only capability has a follow-up owner or coordinator
  assignment.

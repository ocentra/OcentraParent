<!-- agent-capsule -->

> Agent Capsule
> Doc: V0.7 Cross-Platform Proof Gap Tracker
> Kind: architecture/reference documentation; read only when selected by plan route, source router, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

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
- Local/LAN manual results:
  `docs/architecture/local-lan-manual-proof-results-2026-05-22.md`
- Runtime proof standard:
  `docs/expectations/real-evidence-proof.md`
- Current proof matrix:
  `docs/expectations/pre-ai-proof-matrix.json`
- Platform expectations:
  `docs/expectations/platform-deliverables.md` and
  `docs/expectations/platforms.md`
- Current PR #96 CI/package evidence:
  `docs/checkpoints/v0-7-ci-checkpoint-evidence-refresh-2026-05-25.md`
- Current LAN and cross-platform manual proof:
  `docs/checkpoints/v0-7-lan-and-cross-platform-manual-proof-2026-05-25.md`
- Linux package baseline implementation proof:
  `docs/checkpoints/linux-package-baseline-and-package-proof-2026-05-25.md`

The current proof matrix is the claim registry. This tracker must not upgrade a
claim until the proof matrix and supporting evidence artifacts are updated from
real command output, logs, UI proof, package evidence, or device evidence.

## Current PR96 CI Evidence Package

Current `main` at commit `98eaf55b9b8507992cc076fe612e2194de8c90eb`
has green CI Gate run `26401270250` after PR #96. That run is the current
mechanical proof source for fail-fast format/lint/types/Rust check, repository
secret scan, dependency and license policy, SBOM metadata upload, pre-AI proof
matrix, production build, full validation, real portal-to-Rust E2E on hosted
Ubuntu/Windows/macOS runners, and Windows/Linux/macOS/Android/iOS package
preview jobs.

The record at
`docs/checkpoints/v0-7-ci-checkpoint-evidence-refresh-2026-05-25.md` lists the
job ledger, artifact names, artifact digests, and owner-ready remaining proof
commands. Treat that record as `ci-mechanical-proof` only. It does not upgrade
Windows child-PC activity proof, household LAN pairing, package lifecycle,
signing, notarization, store, TestFlight, entitlement, Android physical-device,
iOS device, or reboot/autostart claims.

## Current LAN And Cross-Platform Proof Package

Current `main` at commit `b9ed9dc11849a02eb76134887e4ee64b08b072af`
has green CI Gate run `26415925682` after PR #97 and PR #98. That run is the
current mechanical proof source for fail-fast validation, secret scan,
dependency policy, SBOM upload, pre-AI proof matrix, full validation, real
portal-to-Rust E2E on hosted Ubuntu/Windows/macOS runners, and package-preview
jobs for Windows, Linux, macOS, Android, and iOS simulator.

The record at
`docs/checkpoints/v0-7-lan-and-cross-platform-manual-proof-2026-05-25.md`
adds local worker B evidence from the same current-main commit:

- Windows one-host LAN bind/origin proof on `192.168.2.10` with port pair
  `4677`/`4678`;
- negative wrong-port and offline checks that fail closed;
- real WebSocket LAN smoke through the Rust service, including unpaired
  rejection, direct proof submission, route selection, and paired health
  acceptance;
- artifact download, sidecar hash checks, and package metadata inspection for
  Windows/Linux/macOS/Android/iOS/SBOM artifacts;
- WSL Ubuntu 22.04 pre-AI proof pass plus a real Linux DEB launch blocker:
  artifact binary requires `GLIBC_2.39`, while the host has glibc `2.35`;
- local Android emulator APK install, activity resolution, and launch smoke.

Treat this record as current-main CI/package proof plus local single-host proof.
It does not upgrade household two-device LAN, physical device, macOS host,
iOS/TestFlight/entitlement, signing, store, autostart, reboot, update,
uninstall, or privileged OS capture claims.

## Current Linux Package Baseline Implementation

Branch `codex/linux-package-baseline-and-package-proof` starts from current
`main` commit `0ebfb9e4ffa5352e0afd759725b226d3c6624e12`, whose CI Gate run
`26423129817` is green. The run's Linux preview artifact still belongs to the
pre-change package workflow. The branch record at
`docs/checkpoints/linux-package-baseline-and-package-proof-2026-05-25.md`
selects Ubuntu 22.04/glibc `2.35` as the Linux DEB preview baseline, moves the
Linux preview job to `ubuntu-22.04`, encodes baseline metadata and `libc6`
dependency fields in the DEB, hardens package smoke to launch the extracted
service, and records local WSL Ubuntu 22.04 build/smoke proof.

Treat this as branch evidence until a branch CI run or post-merge CI run uploads
a Linux preview artifact from `ubuntu-22.04`. It narrows the Linux package
baseline gap only; it does not prove Linux desktop capture, network
attribution, enforcement adapters, boot/reboot lifecycle, stores, signing, or
managed-device behavior.

## Current Proof Baseline

`main` has mechanical proof for the shared scaffold and V0.7 dry-run preview
path: TypeScript/Rust contracts, local Rust service mechanics, local transport,
portal request/render mechanics, SQLite/journal-backed read paths, and package
preview scaffolds. PR #96 refreshed the CI package with green current-main
validation and preview mechanics; PR #98 kept that evidence on `main`; the
current B proof package refreshes it again for `b9ed9dc` and adds one-host LAN
bind/origin proof, real dev WebSocket paired/unpaired mechanics, WSL/Linux
package inspection plus a historical glibc blocker, and Android emulator
package smoke. The Linux package baseline branch adds a targeted fix and local
WSL proof for that blocker. Those results do not replace controlled privileged
OS, household two-device LAN, package/autostart/reboot, signing, store,
entitlement, or physical mobile-device proof.

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

| Platform or area                        | Current CI/package-preview proof                                                                                                                                                                                                                                                                                                                                                    | Manual proof still required                                                                                                                                                                                                                                                                               | Current state label                                                                                                                                                                                       | Follow-up owner                                                                                                                      |
| --------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------ |
| Windows local PC                        | Shared contracts, Rust service mechanics, portal transport, package preview, and pre-AI matrix checks are mechanically covered. The merged local/LAN results prove Windows loopback real-service reachability and parent-surface command visibility from commit `4a04d68ffc9cf83c0ea2f1afe794844573ec5b35`.                                                                         | Run the remaining controlled Windows child-device pass: managed browser exact URL/title, fresh foreground app choreography, network/domain generation, timed app/game session, screen queue permission/degraded state, installer, autostart, reboot, uninstall, and sensitive-detail-minimized artifacts. | `implemented` for loopback real-service reachability and parent-surface command visibility; `manual-required` for privileged evidence; `ci-mechanical-proof` for shared mechanics.                        | Primary reviews merged proof results before any matrix upgrade; focused platform proof owners handle remaining controlled scenarios. |
| Linux CI                                | Runs `26415925682` and `26423129817` prove shared TypeScript/Rust mechanics, real portal-to-Rust E2E, DEB build, DEB install/remove smoke, and artifact upload on hosted Linux for current main before this branch.                                                                                                                                                                 | After the Linux baseline branch lands, confirm package-preview `linux-deb` runs on `ubuntu-22.04`, uploads `linux-baseline.json`, and preserves sidecar/package smoke logs. Record unsupported desktop capture, network visibility, service-manager, and enforcement states separately.                   | `ci-mechanical-proof` for shared mechanics and package preview; `not-yet-proven` for platform adapters; Ubuntu 22.04 baseline CI proof is pending branch/post-merge CI.                                   | Primary or Linux portability owner.                                                                                                  |
| Linux WSL/Docker                        | Worker B ran WSL Ubuntu 22.04 `npm run test:pre-ai-proof` successfully during the earlier proof pass. On branch `codex/linux-package-baseline-and-package-proof`, WSL Ubuntu 22.04.5 with glibc `2.35` built the Linux DEB, checked package fields, launched the extracted service on `/health`, and ran DEB install/remove. Docker CLI is not installed on the Windows proof host. | Repeat the hardened smoke from a Linux VM/device and from Docker if Docker becomes available. Record systemd boot/autostart, reboot survival, update behavior, uninstall cleanup, journal/SQLite paths, and unsupported capture/network/enforcement capability labels separately.                         | `implemented` for branch-local Ubuntu 22.04 package build, extracted launch, and WSL install/remove smoke; `unavailable` for Docker on this host; desktop capability claims stay `not-yet-proven`.        | Linux portability/package owner.                                                                                                     |
| macOS                                   | Run `26415925682` proves hosted macOS real portal-to-Rust E2E, PKG build, payload smoke, and artifact upload. Worker B could inspect the PKG archive metadata from Windows only.                                                                                                                                                                                                    | Use the Mac system for app launch, permission prompts, screen recording/accessibility notes, launchd/service behavior, signing/notarization state, and future iOS/Xcode artifacts.                                                                                                                        | `ci-mechanical-proof` for hosted mechanics and package metadata; `manual-required` for permission and package behavior; `not-yet-proven` for adapters.                                                    | Primary-assigned Mac proof owner.                                                                                                    |
| Android emulator                        | Run `26415925682` proves APK build, hosted emulator install/launch smoke, and artifact upload. Worker B also booted local AVD `Pixel_9_Pro_XL_API_35`, installed the CI APK, resolved `ca.ocentra.parent.agent/.MainActivity`, and launched it with `monkey`.                                                                                                                       | Record foreground service assumptions, notification permission state, SQLite/journal compatibility, and parent-app versus child-agent scope separately when product paths exist. Worker B's local `pidof` check did not prove a long-lived agent process.                                                 | `ci-mechanical-proof` for APK install/activity launch; long-lived/physical-device behavior remains `manual-required` or `not-yet-proven`.                                                                 | Android platform proof owner.                                                                                                        |
| Android physical device                 | No current device-owner, managed-profile, VPN/DNS, accessibility, or foreground-service product claim is proven by CI.                                                                                                                                                                                                                                                              | Run physical device package install/launch and permission checks before claiming child-agent support. Device-owner and managed-profile behavior need explicit real-device proof.                                                                                                                          | `manual-required`, `permission-required`, `not-yet-proven`, or `scaffold-only` depending on the capability.                                                                                               | Android platform proof owner after coordinator assignment.                                                                           |
| iOS simulator                           | Run `26415925682` proves iOS simulator app build, simulator install/launch smoke, and artifact upload. Worker B inspected the simulator ZIP from Windows and found `OcentraParentAgent.app`, bundle id `ca.ocentra.parent.agent`, simulator platform metadata, and version `0.1.1`.                                                                                                 | Use Mac/Xcode simulator output to record parent-app shell or scaffold behavior when repeated locally. Do not treat simulator launch as Family Controls, Screen Time, Network Extension, TestFlight, or entitlement proof.                                                                                 | `ci-mechanical-proof` for simulator mechanics and ZIP metadata; `not-yet-proven` for capabilities.                                                                                                        | Mac/iOS platform proof owner.                                                                                                        |
| iOS TestFlight and entitlements         | No current CI signal proves TestFlight, signing, Family Controls, Screen Time, Network Extension, notifications, background execution, or child-agent enforcement.                                                                                                                                                                                                                  | Record signing and entitlement availability, review state, and unsupported capability limits with exact Apple-approved API notes.                                                                                                                                                                         | `manual-required`, `permission-required`, `blocked`, or `unavailable` until credentials and entitlements exist.                                                                                           | Mac/iOS platform proof owner with coordinator review.                                                                                |
| LAN parent-to-child                     | Worker B's current-main proof records one-host LAN bind/origin mechanics on `192.168.2.10`, wrong-port and offline negative checks, and real WebSocket LAN smoke that rejects unpaired control, accepts direct proof submission, selects a route, and accepts paired health through the Rust service.                                                                               | Run real two-device household LAN reachability, parent-device stale-service negative checks, firewall/router notes, and persistent trusted-device registry proof. HTTP discovery, challenge, proof/control, and registry endpoints remain planned unsupported in the current status payload.              | `implemented` for one-host bind/origin and WebSocket paired/unpaired dev mechanics; `manual-required` for real household two-device LAN; `scaffold-only` for HTTP discovery/control and durable registry. | Primary reviews merged proof results; production pairing and two-device LAN remain future owner assignments.                         |
| Installer, autostart, reboot, uninstall | Package-preview jobs can prove archive/build/install or launch smoke where supported.                                                                                                                                                                                                                                                                                               | Exercise real installed artifacts on each platform before claiming service autostart, reboot survival, uninstall cleanup, update behavior, signing, notarization, store distribution, TestFlight, or device-owner mode.                                                                                   | `scaffold-gap`, `manual-required`, or `not-yet-proven`.                                                                                                                                                   | Release/package proof owner.                                                                                                         |

## Owner-Ready Manual Proof Queue

The next proof owners should start from the PR #96 evidence package rather than
from a blank audit. Each row below names the smallest next record that can close
or narrow a gap without upgrading unsupported product claims.

| Queue item                  | Start from                                                                                                                                            | Record as done only when                                                                                                                                                                                                                           | Do not claim                                                                                                                                                                         |
| --------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Windows controlled evidence | `docs/architecture/local-lan-manual-proof-runbook.md` plus the command seeds in `docs/checkpoints/v0-7-ci-checkpoint-evidence-refresh-2026-05-25.md`. | The proof record includes current commit, host metadata, permission state, real Rust service health, portal diagnostic output, evidence read-model output, and synthetic low-sensitivity activity observations.                                    | Do not claim complete browser URL, network, app/game duration, or screen proof when the service reports unavailable, stale, degraded, or partial state.                              |
| Windows MSI lifecycle       | PR #96 artifact `ocentra-parent-windows-x64-preview`.                                                                                                 | Install, service inspection, reboot/autostart check, portal-against-installed-service check, uninstall, process cleanup, and data-retention notes are captured from a real Windows host.                                                           | Do not claim production signing or update behavior from the unsigned preview artifact.                                                                                               |
| LAN paired/unpaired proof   | Current LAN runbook and current V0.9 service/protocol LAN tests.                                                                                      | A parent device and child device record OS/IP range, selected ports, allowed origin, pairing or explicit unpaired state, a successful allowed path where implemented, a failed unpaired/wrong-origin path, and service logs or portal diagnostics. | Do not claim production LAN auth, router/firewall robustness, or cross-device control if only single-machine LAN bind or service tests ran.                                          |
| Linux runtime/package proof | `docs/checkpoints/linux-package-baseline-and-package-proof-2026-05-25.md`, the next branch/post-merge Linux preview artifact, WSL, or Docker.         | WSL/Docker metadata, Ubuntu 22.04/glibc baseline fields, sidecar checks, extracted launch output, install/remove or service output, journal/SQLite path notes, and unsupported capture/network/enforcement capability labels are recorded.         | Do not claim Linux desktop capture, network/domain attribution, service-manager parity, reboot/autostart, update behavior, or enforcement adapters from package install smoke alone. |
| macOS host proof            | PR #96 artifact `ocentra-parent-macos-preview` on a real Mac.                                                                                         | PKG inspection/install, launchd state if claimed, loopback service result if wired, signing/notarization state, and relevant permission settings are recorded.                                                                                     | Do not claim notarization, Screen Recording, Accessibility, Network Extension, or capture parity from hosted CI.                                                                     |
| Android device proof        | PR #96 artifact `ocentra-parent-android-preview`, package id `ca.ocentra.parent.agent`.                                                               | Emulator launch output is separated from physical-device output; real-device permission, foreground/background, notification, UsageStats, accessibility, VPN/DNS, device-owner, and managed-profile states are recorded when available.            | Do not merge parent-app and child-agent claims or claim device-owner/managed-profile support without a managed-device proof.                                                         |
| iOS entitlement proof       | PR #96 artifact `ocentra-parent-ios-simulator-preview`, bundle id `ca.ocentra.parent.agent`, and the Mac/Xcode path.                                  | Simulator launch is recorded separately from TestFlight/device/provisioning; Family Controls, DeviceActivity, Screen Time, Network Extension, notification, background, and signing entitlement availability is explicitly recorded.               | Do not treat simulator launch as entitlement, TestFlight, background monitoring, Screen Time, or child-agent parity proof.                                                           |
| Package/security follow-up  | CI job ledger for `secret-scan`, `dependency-policy`, `ocentra-parent-security-sbom`, and package preview artifacts from run `26415925682`.           | A release owner confirms artifact retention, SBOM retrieval, dependency policy output, preview package scope, Windows signing state, and Linux distro/glibc baseline before any production promotion planning.                                     | Do not claim production release readiness, stores, signing, notarization, legal/compliance approval, or threat-model completion from CI alone.                                       |

## Merged Local/LAN Proof Artifact

PR #57 merged `docs/architecture/local-lan-manual-proof-results-2026-05-22.md`
into `main` as commit `c095a77`. The artifact records commit
`4a04d68ffc9cf83c0ea2f1afe794844573ec5b35`, passing baseline validation, a
Windows loopback proof run, parent-surface command visibility through the real
Rust service, current evidence-preview observations, a single-machine LAN-bind
substitute, and package/autostart/reboot/uninstall gaps.

The artifact does not upgrade the proof matrix by itself. It explicitly leaves
two-device LAN pairing, managed-browser exact URL/title proof, fresh controlled
foreground-window proof, timed app/game duration, screen queue permission proof,
package install/autostart/reboot/uninstall, production signing, stores,
TestFlight, device-owner, and entitlement proof as gaps.

This lane only references the merged artifact. Do not edit, pre-fill, or
normalize local/LAN proof results from this tracker branch.

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
- Production V0.9 LAN pairing, hardened trusted-device registry storage,
  authenticated remote control, and real two-device multi-device routing beyond
  the current service/protocol proof spine.
- V2 cloud relay, parent-owned storage sync/export, remote approvals, and
  cloud-routed reports.
- V3 notification delivery, provider retries, quiet hours, and escalation.

## Proof Matrix And Roadmap Follow-Up

Do not update `docs/expectations/pre-ai-proof-matrix.json` from this tracker
alone. Update the matrix only after proof artifacts exist for the exact commit
and platform under review.

Roadmap follow-up is needed after the coordinator accepts which merged proof
results should influence planning language:

- Link the merged proof results from the V0.7 checkpoint section if the
  coordinator accepts the evidence.
- Keep V0.8 enforcement blocked until Windows local and LAN checkpoint findings
  are reviewed.
- Add platform rows only when each row names CI proof, manual proof, scaffold
  gaps, unavailable states, and the owner of remaining gaps.

## Review Checklist

- Current branch and commit are named in the proof record.
- Every platform row separates CI/package mechanics from manual OS/device
  behavior.
- Current runs `26415925682` and `26423129817`, plus prior PR #96 run
  `26401270250`, are referenced as CI-mechanical proof only.
- Owner-ready remaining proof commands or checklists are present for Windows,
  LAN, Linux, macOS, Android, iOS, and package lifecycle.
- Merged local/LAN proof results are referenced without treating substitute LAN
  or loopback proof as full household pairing proof.
- Package runtime changes are limited to the linked Linux baseline branch; no
  portal shell, product behavior, or enforcement path was changed by this
  tracker.
- No claim is upgraded without proof matrix and artifact updates.
- Every blocked or scaffold-only capability has a follow-up owner or coordinator
  assignment.

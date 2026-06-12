<!-- agent-capsule -->

> Agent Capsule
> Doc: V0.7 Current Main Acceptance Record - 2026-05-25
> Kind: architecture/reference documentation; read only when selected by plan route, source router, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

# V0.7 Current Main Acceptance Record - 2026-05-25

## Scope

This record is the reviewer checklist for current `main` after PR #90 through
PR #96. It explains what is complete on `main`, what is still CI-mechanical
only, what remains manual-required or not-yet-proven, why V0.7 remains the
acceptance gate, and what must pass before larger V0.8/V0.9 implementation
resumes.

This is a docs/proof narrative artifact only. It does not change product code,
portal UI, package workflows, CI evidence, the proof matrix, V0.8 enforcement
behavior, V0.9 LAN behavior, production publishing, or mobile entitlement
claims.

## Current Main Subject

| Field            | Value                                                                       |
| ---------------- | --------------------------------------------------------------------------- |
| Date             | 2026-05-25                                                                  |
| Branch reviewed  | `origin/main` through `98eaf55b9b8507992cc076fe612e2194de8c90eb`            |
| Latest PR in set | PR #96, `Make precommit gate fast and opt-in full validation`               |
| Roadmap gate     | V0.7 current-main checkpoint proof                                          |
| Companion record | `docs/architecture/current-main-proof-refresh-2026-05-25.md`                |
| Separate CI file | B-owned `docs/checkpoints/...` checkpoint evidence refresh, not edited here |

## Status Labels

| Label                | Meaning                                                                                                              |
| -------------------- | -------------------------------------------------------------------------------------------------------------------- |
| `completed-on-main`  | Merged code, contracts, tests, docs, or proof records exist on `main` and are reviewable.                            |
| `ci-mechanical-only` | CI or hosted package-preview mechanics can exercise the path, but real OS/device proof is missing.                   |
| `manual-required`    | A real host, device, package, LAN, permission, or lifecycle pass is needed.                                          |
| `not-yet-proven`     | The product path or adapter has not supplied enough evidence for a support claim.                                    |
| `scaffold-only`      | Shape/package/project scaffolding exists, but product behavior is intentionally absent.                              |
| `blocked`            | A credential, entitlement, OS approval, hardware condition, or policy decision is required before proof can advance. |

## Completed On Main

| Area                         | Completed-on-main state                                                                                                                                                                                                                                                                                         | Boundary                                                                                          |
| ---------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------- |
| Foundation and local runtime | Repository scaffold, TypeScript workspaces, Rust crates, fixed local ports, loopback/LAN dev scripts, WebSocket intent/event protocol, release/package scaffolds, pre-commit hooks, validation gates, and CI/package-preview workflow separation are on `main`.                                                 | CI and package scaffolds do not prove production release readiness.                               |
| Evidence store               | Encrypted append-only activity journal, journal rotation/replay, SQLite query-store direction, activity contracts, and local read-path mechanics are on `main`.                                                                                                                                                 | Real host package lifecycle and rebuild proof still need checkpoint evidence.                     |
| Pre-AI evidence bridges      | Browser URL/tab state, app/game sessions, network flow summaries, and local screen-analysis queue summaries have typed local contracts and Rust protocol/read-model paths on `main`.                                                                                                                            | Current host/device proof still controls support claims.                                          |
| V0.7 dry-run preview         | Local runtime/provider status, evidence context-builder contracts, deterministic dry-run policy evaluator, context-builder read-path hardening, policy-preview service/API response, portal read-model rendering, parent-rule context resolver, provider probe status, and preview quality batch are on `main`. | Enforcement remains disabled and model execution remains unavailable/local-only by default.       |
| V0.8 proof spines            | PR #91, PR #93, and PR #95 prove typed enforcement audit-boundary, timer/recovery identity, and parent approval/override audit reference behavior across contract/protocol/core tests.                                                                                                                          | No OS blocking, adapter delivery, rollback UX, anti-tamper, or notification claim.                |
| V0.9 proof spines            | PR #90, PR #92, and PR #94 prove selected-route rejection, restart registry intent selection, and LAN discovery/challenge/status privacy surfaces through contract/protocol/service tests.                                                                                                                      | No production LAN auth, portal selector UX, router/firewall proof, or two-device household proof. |
| Workflow/tooling             | PR #96 makes the local pre-commit hook a fast source gate and documents explicit heavier gates: `precommit:full`, `test:local`, `validate`, `ci:local`, and focused E2E commands.                                                                                                                               | Tooling progress is not product feature progress.                                                 |

## CI-Mechanical Only

These items may be green through CI, hosted runners, or proof-matrix mechanics,
but they remain insufficient for product acceptance by themselves:

- Pre-AI proof matrix checks for recorded claims.
- Full validation gate, package lint/type-check/test, Rust checks/tests, and
  real portal-to-Rust E2E on hosted Windows/Linux/macOS runners.
- Package-preview builds and hosted smoke checks for Windows MSI, Linux DEB,
  macOS PKG, Android APK, and iOS simulator artifacts.
- Shared contract and protocol proof for V0.8/V0.9 spines.
- Local validation workflow proof from PR #96.

CI-mechanical proof is required, but it does not prove privileged capture,
production package lifecycle, real household LAN, signing/notarization/store
state, mobile device-owner policy, iOS entitlements, local AI model execution,
or OS enforcement adapters.

## Manual-Required Or Not-Yet-Proven

| Area                  | Required proof before support claims advance                                                                                                                                                                                 | Current label                                                         |
| --------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------- |
| Windows evidence flow | Run current main on a real Windows child PC and capture managed-browser URL/title/domain, foreground app/window, network/domain, app/game duration, screen queue, and explicit degraded states.                              | `manual-required`                                                     |
| LAN household flow    | Prove at least one paired parent-to-child request and one failed unpaired request across two devices, with OS/IP/ports, firewall/router notes, custody labels, and product-path artifacts.                                   | `manual-required`                                                     |
| Package lifecycle     | Install preview artifacts, verify service-manager/autostart behavior where claimed, reboot, update, uninstall, and record data retention/removal behavior.                                                                   | `manual-required`                                                     |
| Linux                 | Supplement hosted CI with WSL/Docker or real package/service proof and label desktop capture, network/domain, and enforcement assumptions honestly.                                                                          | `manual-required` or `not-yet-proven`                                 |
| macOS                 | Run package/app proof on the Mac host, record launchd/signing/notarization state, and record permission surfaces only where product claims exist.                                                                            | `manual-required` or `not-yet-proven`                                 |
| Android               | Run emulator plus physical-device checks for foreground service, notification permission, UsageStats, accessibility, VPN/DNS, device-owner, and managed-profile assumptions.                                                 | `manual-required`, `permission-required`, or `not-yet-proven`         |
| iOS                   | Record simulator-only scope, TestFlight/device availability, signing team, provisioning, and entitlement state for Family Controls, DeviceActivity, Screen Time, Network Extension, notifications, and background execution. | `manual-required`, `permission-required`, `blocked`, or `unavailable` |
| V0.8 enforcement      | Add and prove real OS adapter paths, blocking behavior, timer delivery, rollback UX, anti-tamper posture, and notification/audit delivery before enforcement product claims.                                                 | `not-yet-proven`                                                      |
| V0.9 LAN product      | Prove production LAN auth, trusted-device control, selected-device UX, failed-unpaired behavior, router/firewall behavior, and cloud/relay boundaries if introduced.                                                         | `not-yet-proven`                                                      |

## Why V0.7 Remains The Gate

V0.7 is the acceptance gate because it is the last checkpoint where the product
can still honestly prove visibility, evidence custody, dry-run policy-preview
behavior, local transport, and package mechanics before enforcement and
multi-device control raise the risk level.

The V0.8 and V0.9 work currently on `main` is valuable because it creates typed
proof spines for future enforcement and LAN behavior. It does not replace the
V0.7 proof gate. Starting larger enforcement or multi-device implementation
before current evidence, package, LAN, and platform proof is reviewed would make
later product claims hard to audit and easy to overstate.

## Resume Criteria For Larger V0.8/V0.9 Work

Before assigning broader V0.8 or V0.9 implementation, primary should have a
reviewable record for each item below:

1. Current `main` SHA, branch state, and proof record are clean and named.
2. `cmd /c npm run format:check` passes or has an explicit blocker.
3. `cmd /c npm run test:pre-ai-proof` passes for the current proof matrix.
4. `cmd /c npm run validate` passes locally, or the omission is recorded with
   exact reason and CI substitute.
5. Current main CI/package evidence is reviewed in the separate checkpoint
   evidence record.
6. Windows real child-device proof covers the current V0.7 evidence-preview
   flow or records exact unavailable/degraded states.
7. LAN proof records one paired and one failed-unpaired request through product
   paths, or records why that remains manual-required.
8. Package lifecycle proof records install, autostart/service-manager, reboot,
   update, uninstall, and data-retention behavior for available artifacts.
9. Linux, macOS, Android, and iOS rows have current labels for CI proof, manual
   proof, scaffold-only state, blocked state, and permission requirements.
10. The proof matrix and roadmap are updated only where concrete evidence backs
    the label change.
11. The next V0.8/V0.9 work item is split narrowly enough that it cannot be read
    as product-complete enforcement or LAN pairing.

## Non-Claims

This acceptance record does not claim V0.7 is fully accepted. It records the
current acceptance boundary and the evidence still needed.

It also does not claim real OS blocking, production enforcement, household LAN
pairing, local model execution, notification delivery, cloud relay, production
signing, notarization, store readiness, Android device-owner behavior, or iOS
Family Controls behavior.

## PR Body Outline

```text
Scope
- Expanded the V0.7 current-main proof reconciliation after PR #90 through PR #96 into a reviewable acceptance package.
- Updated docs/product-roadmap.md to link both the current-main proof refresh and acceptance record.
- Expanded docs/architecture/current-main-proof-refresh-2026-05-25.md with completed-on-main, CI-mechanical-only, manual-required/not-yet-proven, and V0.8/V0.9 resume-gate language.
- Added docs/architecture/v0-7-current-main-acceptance-record-2026-05-25.md as the reviewer checklist for current-main acceptance.
- Kept B-owned CI/package evidence and C-owned portal/vendor files out of scope.

Touched files
- docs/product-roadmap.md
- docs/architecture/current-main-proof-refresh-2026-05-25.md
- docs/architecture/v0-7-current-main-acceptance-record-2026-05-25.md

Validation
- cmd /c npm run format:check
- cmd /c npm run test:pre-ai-proof
- cmd /c npm run test:tooling
- git diff --check
- cmd /c npm run lanes:guard
- cmd /c npm run hub:guard
- cmd /c npm run validate

Known gaps and risks
- This is a roadmap/proof narrative package; it does not fake manual proof.
- Current-main CI/package evidence is intentionally separate and owned by the B checkpoint file.
- V0.7 acceptance still needs the manual Windows, LAN, package lifecycle, Linux, macOS, Android, and iOS proof called out in the checkpoint runbook.
- V0.8/V0.9 remain proof-spine-only, not product-complete.
```

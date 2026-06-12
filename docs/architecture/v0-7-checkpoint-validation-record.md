<!-- agent-capsule -->

> Agent Capsule
> Doc: V0.7 Checkpoint Validation Record After PR87
> Kind: architecture/reference documentation; read only when selected by plan route, source router, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

# V0.7 Checkpoint Validation Record After PR87

Date: 2026-05-24

Owner: codex-a

Branch: `codex/v0.7-checkpoint-proof-records-after-pr87`

Source branch: `origin/main`

Current main commit: `4aade13fe7fe9dff294932efbbdbdcfccba4c5e8`

Package version: `0.1.1`

Scope: record the current V0.7 checkpoint state after PR #87 without adding
V0.8 enforcement, blocking, portal, vendor, or runtime behavior.

## CI Evidence

GitHub Actions run:
`https://github.com/ocentra/OcentraParent/actions/runs/26371210839`

Run title: `Prove LAN registry persistence restart behavior (#87)`

Workflow: `CI Gate`

Run status: `completed`

Run conclusion: `success`

Run head branch: `main`

Run head SHA: `4aade13fe7fe9dff294932efbbdbdcfccba4c5e8`

Relevant successful jobs:

- `validate / Pre-AI Proof Matrix`
- `validate / Full Validation Gate`
- `validate / Real Portal To Rust E2E (ubuntu-latest)`
- `validate / Real Portal To Rust E2E (windows-latest)`
- `validate / Real Portal To Rust E2E (macos-latest)`
- `package-preview / Windows MSI Preview`
- `package-preview / Linux DEB Preview`
- `package-preview / macOS PKG Preview`
- `package-preview / Android APK Preview`
- `package-preview / iOS Simulator App Preview`

Interpretation: CI proves repeatable contracts, schema checks, local service
mechanics, real portal-to-Rust E2E mechanics, and package-preview mechanics for
the current commit. It does not prove privileged OS behavior, household LAN
pairing, production signing/notarization/store readiness, mobile device-owner
policy, or iOS entitlement behavior.

## Local Commands

Commands run for this record:

```powershell
git fetch origin --prune
git branch --show-current
git status --short --branch
git rev-parse HEAD
cmd /c npm run lanes:status
cmd /c npm run lanes:guard
cmd /c npm run hub:status
cmd /c npm run hub:guard
cmd /c npm run hub:inbox
cmd /c npm run hub:ack
cmd /c npm run hub:report -- --summary "STARTED V0.7 checkpoint proof record after PR87" --details "Locked docs/architecture/v0-7-checkpoint-validation-record.md only; gathering current-main CI and proof-matrix evidence."
cmd /c npm run hub:lock -- --paths "docs/architecture/v0-7-checkpoint-validation-record.md" --reason "V0.7 checkpoint proof record after PR87"
gh run view 26371210839 --json databaseId,headSha,headBranch,status,conclusion,displayTitle,workflowName,url,createdAt,updatedAt,jobs
```

Validation commands for this record:

```powershell
cmd /c npm run format:check    # passed: All matched files use Prettier code style.
cmd /c npm run test:pre-ai-proof # passed: 11 claims checked across 5 platforms; 7 checkpoint scenarios checked.
git diff --check                # passed: no whitespace errors.
cmd /c npm run lanes:guard      # passed: lane-guard-ok for codex-a.
cmd /c npm run hub:guard        # passed: hub-guard-ok for codex-a.
```

The local commit hook also ran `cmd /c npm run validate` for this record. The
first commit attempt reached the portal E2E step and failed because port `4489`
was temporarily held by a non-Parent process; no other lane process was killed.
After the port cleared, the retry passed the full pre-commit validation gate,
including release version, pre-AI proof, lint/schema boundaries, tests, Rust
validation, integration smoke, portal local smoke, portal Playwright E2E, and
build. CI run `26371210839` also covers the same current-main commit with a
successful `validate / Full Validation Gate`.

No manual platform, device, package-lifecycle, or LAN household proof was run in
this codex-a docs lane. Those checks are intentionally recorded below as gaps
because this assignment is a checkpoint proof record, not a platform proof pass.

## Current Proof State

The current post-PR87 main state is acceptable as CI/mechanical V0.7 evidence
only. The pre-AI proof matrix still marks privileged evidence scenarios as
`manual-required`, `not-yet-proven`, `scaffold-only`, or platform-specific
`not-applicable` where appropriate.

Claims that remain CI/mechanical only:

- V0.1 foundation and evidence contracts.
- V0.2 trusted local evidence store mechanics.
- V0.5 live activity portal local transport mechanics.
- V0.6 local AI safety decision contracts.
- V0.7 dry-run policy preview contracts and service/API paths.
- Cross-platform package preview build, install, or launch smoke where CI runner
  support exists.

Claims that must not be upgraded from this record alone:

- Windows foreground process/window observation on a real child PC.
- Managed browser exact URL and active-tab proof from an Ocentra-managed
  profile.
- Windows network/domain attribution under real OS visibility and permission
  constraints.
- App/game duration evidence from real foreground activity over time.
- Screen evidence queue behavior under real permission, disabled, queued,
  summarized, and deletion states.
- Two-device LAN pairing with one successful paired request and one failed
  unpaired request.
- Installed package service/autostart/update/reboot/uninstall/data-retention
  behavior on real hosts.
- Linux WSL/Docker proof beyond hosted CI package and runtime mechanics.
- macOS host launchd, permission, signing, notarization, and service-manager
  behavior.
- Android real-device foreground service, notification, UsageStats,
  accessibility, VPN/DNS, device-owner, and managed-profile behavior.
- iOS TestFlight/device entitlement behavior for Family Controls,
  DeviceActivity, Screen Time, Network Extension, notifications, or background
  execution.

## Remaining Manual-Proof Gaps

Windows local PC:

- Run the current Rust agent on a real Windows child PC.
- Capture foreground process/window evidence through the real service path.
- Capture managed-browser URL/title/domain and active-tab certainty or explicit
  tab-list-only state through an Ocentra-managed browser profile.
- Capture network/domain flow evidence without decrypted payload claims.
- Capture app/game duration using stored first-seen and last-seen evidence.
- Capture screen evidence queue states with real permission and deletion-state
  evidence.
- Record unavailable, degraded, disabled, or permission-required states when the
  host lacks visibility or permissions.

LAN:

- Prove at least one two-device LAN paired request through the real transport.
- Prove one unpaired LAN request fails.
- Record parent and child OS/IP range, selected ports, pairing step, copied
  service status or UI proof, firewall/router state, and custody state.

Package lifecycle:

- Install from preview artifacts on real target hosts where available.
- Launch from installed locations.
- Verify service-manager registration only where claimed.
- Reboot and record autostart behavior.
- Stop, restart, uninstall, and record process/service cleanup.
- Record whether data is retained, removed, or user-controlled.
- Keep preview packages as scaffold/mechanical proof until production signing,
  notarization, store, managed-device, or entitlement credentials are wired and
  proved.

Linux:

- Repeat install or service launch in WSL or Docker when runner privileges are
  not enough.
- Verify Linux filesystem semantics for journal and SQLite paths.
- Keep desktop capture, network/domain observation, and enforcement as
  unsupported or not-yet-proven until adapters and permission assumptions are
  implemented and tested.

macOS:

- Install or launch the package on the Mac host.
- Record launchd behavior only if the package claims it.
- Record signing and notarization state honestly.
- Check Screen Recording, Accessibility, Network Extension, notifications, and
  background permission states only when the corresponding product claim exists.
- Keep process/window, network/domain, screen, and enforcement parity unclaimed
  from package scaffold alone.

Android:

- Run emulator package/launch/local-store checks when available.
- Install on one real Android device.
- Verify foreground service behavior after backgrounding.
- Record notification permission state.
- Record UsageStats, accessibility, VPN/DNS, device-owner, and managed-profile
  modes as absent, blocked, permission-required, or implemented based on real
  proof.

iOS:

- Run simulator launch proof as simulator-only evidence.
- Record bundle id, signing team, provisioning, and TestFlight availability or
  absence.
- Record entitlement state for Family Controls, DeviceActivity, Screen Time,
  Network Extension, notifications, and background execution.
- Do not treat simulator launch as proof of device entitlement behavior.

## Non-Claims

This record does not claim real OS blocking, enforcement adapters, local AI model
execution, notification delivery, cloud relay, mobile device-owner policy, iOS
Family Controls, production signing, notarization, app-store readiness, or
household multi-device LAN behavior.

It records the current proof boundary: V0.7 dry-run evidence and package-preview
mechanics are green in CI; privileged and household behavior remains manual
proof work.

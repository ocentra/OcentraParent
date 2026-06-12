<!-- agent-capsule -->

> Agent Capsule
> Doc: V0.7 Windows Controlled Evidence And Package Lifecycle Proof - 2026-05-25
> Kind: historical checkpoint/proof documentation; read only when CHECKPOINT_INDEX or PROOF_INDEX names it.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

# V0.7 Windows Controlled Evidence And Package Lifecycle Proof - 2026-05-25

## Scope

This record executes the Worker A Windows/local proof pass requested after PR
#97 and PR #98 merged to `main`. It covers current `main` at
`b9ed9dc11849a02eb76134887e4ee64b08b072af` and records what the local Windows
machine could prove through real product paths without manually inserting rows,
mocking evidence, rebooting, or running an elevated installer.

The proof boundary is V0.7 acceptance. V0.8 enforcement and V0.9 LAN proof
spines exist on `main`, but they are not product-complete enforcement or
production LAN pairing.

## Run Metadata

| Field                   | Value                                                                                                                                 |
| ----------------------- | ------------------------------------------------------------------------------------------------------------------------------------- |
| Proof date              | 2026-05-25                                                                                                                            |
| Worker lane             | `codex-a`                                                                                                                             |
| Branch                  | `codex/v0.7-windows-controlled-evidence-and-package-lifecycle-proof`                                                                  |
| Commit under test       | `b9ed9dc11849a02eb76134887e4ee64b08b072af`                                                                                            |
| Package/app version     | `0.1.1`                                                                                                                               |
| Host OS                 | Microsoft Windows 11 Pro `10.0.26200`, build `26200`, `64-bit`                                                                        |
| Host hardware           | Gigabyte Technology Co., Ltd. `X570 AORUS MASTER`, 63.92 GB RAM                                                                       |
| Toolchain               | Node `v22.22.2`, npm `11.7.0`, cargo `1.90.0`, rustc `1.90.0`, gh `2.40.1`                                                            |
| Lane ports              | Rust agent `127.0.0.1:4677`, portal `127.0.0.1:4678`                                                                                  |
| Sensitive data handling | No raw screenshots, browser history beyond `example.com`, child data, secrets, decrypted payloads, or package binaries are committed. |

## Source Inputs

- `docs/product-roadmap.md`
- `docs/architecture/v0-7-current-main-acceptance-record-2026-05-25.md`
- `docs/architecture/current-main-proof-refresh-2026-05-25.md`
- `docs/architecture/cross-platform-deliverables-checkpoint.md`
- `docs/architecture/local-lan-manual-proof-runbook.md`
- `docs/architecture/v07-cross-platform-proof-gap-tracker.md`
- `docs/checkpoints/v0-7-ci-checkpoint-evidence-refresh-2026-05-25.md`
- `docs/expectations/pre-ai-proof-matrix.json`
- GitHub Actions run `26415925682`:
  <https://github.com/ocentra/OcentraParent/actions/runs/26415925682>

## Baseline And Service Commands

| Command                                                                                                                                                                                                                                   | Result                                                                                                | Proof label           |
| ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------- | --------------------- |
| `git fetch origin main`                                                                                                                                                                                                                   | Passed before branch reset; `origin/main` resolved to `b9ed9dc`.                                      | `implemented`         |
| `git switch -C codex/v0.7-windows-controlled-evidence-and-package-lifecycle-proof origin/main`                                                                                                                                            | Passed; branch reset from current `origin/main`.                                                      | `implemented`         |
| `cmd /c npm run hub:inbox`                                                                                                                                                                                                                | Read the `FULL SCOPE: Windows V0.7 controlled proof` assignment and follow-up lock cleanup mail.      | `implemented`         |
| `cmd /c npm run hub:ack`                                                                                                                                                                                                                  | Passed; acknowledged `codex-a-msg-20260525T191410582Z-158`.                                           | `implemented`         |
| `cmd /c npm run hub:report -- --summary "STARTED Windows V0.7 controlled proof" ...`                                                                                                                                                      | Passed; STARTED report `codex-a-report-20260525T191541793Z-209`.                                      | `implemented`         |
| `cmd /c npm run lanes:status`                                                                                                                                                                                                             | Passed; `codex-a` owned this branch and C remained on portal work.                                    | `implemented`         |
| `cmd /c npm run lanes:guard`                                                                                                                                                                                                              | Passed before edits.                                                                                  | `implemented`         |
| `cmd /c npm run hub:status`                                                                                                                                                                                                               | Passed; A ack/report state visible and B/C locks were separate.                                       | `implemented`         |
| `cmd /c npm run hub:guard`                                                                                                                                                                                                                | Passed before edits.                                                                                  | `implemented`         |
| `cmd /c npm run hub:lock -- --paths "docs/checkpoints/v0-7-windows-controlled-evidence-and-package-lifecycle-proof-2026-05-25.md,docs/checkpoints/artifacts/v0-7-windows-controlled-evidence-and-package-lifecycle-proof-2026-05-25" ...` | Passed; lock stayed out of B CI evidence and C portal paths.                                          | `implemented`         |
| `cmd /c npm run format:check`                                                                                                                                                                                                             | Passed; all matched files used Prettier style before proof edits.                                     | `implemented`         |
| `cmd /c npm run test:pre-ai-proof`                                                                                                                                                                                                        | Passed; `11 claims checked across 5 platforms; 7 checkpoint scenarios checked.`                       | `implemented`         |
| `cmd /c "npm run build:contracts && cargo build -p ocentra-parent-agent-service"`                                                                                                                                                         | Passed; contracts built and debug Rust service binary existed for runtime proof.                      | `implemented`         |
| `node scripts/test/real-evidence-proof-checkpoint.mjs`                                                                                                                                                                                    | Passed; `7 scenarios checked; 5 manual-required; 1 scaffold-gap.`                                     | `implemented`         |
| `cmd /c npm run test:integration`                                                                                                                                                                                                         | Passed; `websocket-local-smoke-ok` and `websocket-lan-smoke-ok` through the real Rust service.        | `ci-mechanical-proof` |
| `cmd /c npm run test:e2e`                                                                                                                                                                                                                 | Passed; `portal-local-smoke-ok`; Playwright `1 passed`; real portal-to-Rust command results rendered. | `ci-mechanical-proof` |

## Lane-Port Windows Proof

A temporary proof driver was run from the locked artifact path and removed
before commit. It launched:

- `target/debug/ocentra-parent-agent-service.exe` on `127.0.0.1:4677`;
- Vite portal on `http://127.0.0.1:4678/#/commands`;
- temp activity paths under `%TEMP%` for SQLite, encrypted journal, journal key,
  and dev logs.

Observed product-path results:

| Check                                         | Observed result                                                                                                                                                                                                                                                                                                                                                                                               | Proof label                                                                         |
| --------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------- |
| Rust service health                           | `GET http://127.0.0.1:4677/health` returned HTTP `200`; WebSocket `agent.health.reported` payload had `online: true`, `transport: websocket`.                                                                                                                                                                                                                                                                 | `implemented`                                                                       |
| Portal shell on lane port                     | Portal HTML included `Ocentra Parent`; Playwright loaded the commands route with title `Ocentra Parent` and `Controls` visible. Portal dev log recorded `agentWebSocketUrl: ws://127.0.0.1:4677/api/dev/ws`.                                                                                                                                                                                                  | `implemented`                                                                       |
| Activity ingest                               | `agent.activity.ingest.status.reported` returned `databaseReady: true`, `eventsStored: 51`, `duplicateEvents: 0`.                                                                                                                                                                                                                                                                                             | `implemented`                                                                       |
| Foreground process/window                     | `agent.activity.recent.summary.reported` returned `mostRecentKind: activity.window.focused`, `mostRecentObserver: windows-window`, `mostRecentSubjectKind: window`, `mostRecentSubjectName: Codex`.                                                                                                                                                                                                           | `implemented` for real foreground-window read path on this host                     |
| Activity memory graph                         | `agent.activity.memory-graph.reported` returned `capabilityStatus: ready`, `custody: child-device-activity-store`, `returned: 1`, and digest nodes for `local-dev-agent` and `Codex`.                                                                                                                                                                                                                         | `implemented` for local read model                                                  |
| Network flow                                  | `agent.network.flow.read-model.reported` returned `capabilityStatus: available`, `adapterId: windows-network-snapshot-adapter`, `observer: windows-network`, `processAttributionStatus: process-attributed`, `processName: svchost.exe`, `networkProtocol: udp`, `localIp: 0.0.0.0`, `localPort: 3544`, `returned: 10`. Domain fields were `destinationDomain: null`, `domainAttributionStatus: unavailable`. | `degraded` for domain attribution; `implemented` for network/process flow read path |
| Managed-browser state without explicit bridge | `agent.browser.managed.status.reported` launched a managed Chrome profile with `managedState: running-managed`, `capabilityStatus: bridge-missing`, `queryVisibility: live-local`.                                                                                                                                                                                                                            | `degraded`                                                                          |
| Browser evidence without bridge poll result   | `agent.browser.evidence.recent.reported` returned `returned: 0`, `custodyLabel: unavailable`, `queryVisibility: unavailable`.                                                                                                                                                                                                                                                                                 | `unavailable` until bridge-backed proof runs                                        |
| Browser protection read model                 | `agent.browser.intervention.read-model.reported` returned `managedSessionInterventionCapability: needs-managed-session` and `unmanagedBrowserEnforcement: requires-os-app-control`.                                                                                                                                                                                                                           | `scaffold-gap`                                                                      |
| Local AI runtime                              | `agent.local-ai.runtime.status.reported` returned `privacyMode: local-only`, `executionState: disabled`, `executionAllowed: false`, `capabilityFlags: none`, `unavailableReason: local-ai-model-file-unconfigured`.                                                                                                                                                                                           | `unavailable` by design                                                             |
| Policy preview                                | `agent.policy.preview.read-model.reported` returned `capabilityStatus: ready`, `dryRun: true`, `enforcementHandoffState: disabled`, `policyAction: unknown`, `reasonCodes: no-matching-parent-rule,local-ai-result-missing`, and one evidence reference.                                                                                                                                                      | `implemented` for V0.7 dry-run preview, no enforcement                              |
| Watcher status                                | `agent.watch.status.reported` returned `available: false` with `Watcher status endpoint is available; watcher runtime is not active.`                                                                                                                                                                                                                                                                         | `degraded`                                                                          |

Dev log snippets from the same run showed:

- `Agent service dev runtime started.`
- `Agent health endpoint requested.`
- `Vite dev server started.` with `port: 4678`
- `Portal dev runtime started.` with `agentWebSocketUrl:
ws://127.0.0.1:4677/api/dev/ws`
- Portal command sends for `agent.health.check`,
  `agent.activity.ingest.status.get`, and related commands.

## Managed Browser Proof

| Command                                                                                                                                                                                                    | Result                                                                                                                                                                                                                                                                                                                                                                                | Artifact                                                                                                              | Proof label                                                                                    |
| ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------- |
| `cmd /c npm run test:managed-browser-service-proof`                                                                                                                                                        | Passed with `managed-browser-service-proof-ok=true`; service path proved `url=https://example.com/?ocentra_service_proof=1`, `title=example.com`, `domain=example.com`, `activeState=unknown`, `capability=tab-list-only`, `queryVisibility=live-local`.                                                                                                                              | `test-results/managed-browser-service-proof/2026-05-25T19-24-36-907Z.json` (ignored local artifact)                   | `implemented` for exact URL/title/domain service evidence; `degraded` for active-tab certainty |
| `cmd /c npm run test:managed-browser-matrix`                                                                                                                                                               | First full multi-site/default-profile run timed out after `244042 ms` and was stopped; leftover temp profile processes were removed.                                                                                                                                                                                                                                                  | Partial ignored screenshots under `test-results/managed-browser-profile-matrix/2026-05-25T19-24-49-434Z-screenshots`. | `blocked` by local timeout for broad sweep                                                     |
| `$env:OCENTRA_PARENT_MANAGED_BROWSER_MATRIX_URLS='https://example.com/'; $env:OCENTRA_PARENT_MANAGED_BROWSER_MATRIX_PROFILES='managed-browser-profile-a'; ...; cmd /c npm run test:managed-browser-matrix` | Passed with `managed-browser-profile-matrix-ok=true`; `supportedBrowsers=3`, `managedProfiles=3`, `capturedUrls=3`, `activeProofs=3`, `historyProofs=3`. Chrome and Firefox reported `protocol-activated-tab-reported-visible-and-focused`; Edge reported `single-runtime-visible-tab-observed`. Unsupported Internet Explorer entries were recorded as `unsupported-legacy-browser`. | `test-results/managed-browser-profile-matrix/2026-05-25T19-29-40-034Z.json` plus ignored screenshots.                 | `implemented` for managed-profile browser bridge mechanics on this host                        |

## App/Game And Screen Evidence Boundaries

| Check                                  | Observed result                                                                                                                                                                                                                                                 | Proof label                          |
| -------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------ |
| App/game duration                      | The real service run produced process/window and memory-graph evidence, but no direct service command exposed an app/game session duration row. The memory-graph edge for the foreground window had `durationMs: null`; no app/game session result was claimed. | `not-yet-proven`                     |
| Screen queue permission/degraded state | Current WebSocket command set does not expose a screen queue status command. No screen permission prompt or capture action was run, and no screenshot artifact was inserted.                                                                                    | `manual-required` / `not-yet-proven` |

These are important non-upgrades: contracts and core read models exist on
`main`, but this Windows pass did not produce parent-visible app/game duration
or screen queue proof.

## Windows Package Lifecycle From Run 26415925682

GitHub Actions run `26415925682` is the current-main run for
`b9ed9dc11849a02eb76134887e4ee64b08b072af`. `gh run view` reported
`status: completed`, `conclusion: success`, workflow `CI Gate`, created
`2026-05-25T19:11:03Z`, updated `2026-05-25T19:27:09Z`.

Relevant successful jobs:

- `validate / Full Validation Gate`
- `validate / Real Portal To Rust E2E (windows-latest)`
- `package-preview / Windows MSI Preview`
- `package-preview / Linux DEB Preview`
- `package-preview / macOS PKG Preview`
- `package-preview / Android APK Preview`
- `package-preview / iOS Simulator App Preview`

Artifact API result for `ocentra-parent-windows-x64-preview`:

| Field       | Value                                                                     |
| ----------- | ------------------------------------------------------------------------- |
| Artifact id | `7203840936`                                                              |
| Size        | `19036264` bytes                                                          |
| Digest      | `sha256:d4b693804274d92403fec32adb5e223a535d5bf5617e9fbb76c3302135eae13f` |
| Created UTC | `2026-05-25T19:27:03Z`                                                    |
| Expires UTC | `2026-08-23T19:11:03Z`                                                    |
| Expired     | `false`                                                                   |

The artifact was downloaded to a local temp directory, not committed:
`%TEMP%/ocentra-parent-run-26415925682-windows`.

Downloaded Windows artifact file checks:

| File                                                 |      Size | SHA-256                                                            |
| ---------------------------------------------------- | --------: | ------------------------------------------------------------------ |
| `install-ocentra-parent-agent-windows.ps1`           |    `2765` | `ed5963f2f4aebad7292a49e17ebb058970c0a7068f13329d95251195fb42f183` |
| `latest-windows.json`                                |    `1155` | `e95b18602d4104bdbc422fb4bb5a8fea9ac3b5cc32a7a80fb40c265cde8a6f9b` |
| `ocentra-parent-agent-windows-x64-latest.msi`        | `9781248` | `920cc683445a991b8bce8874138342acb5bb9b80ab982dc616740a3ce0185c79` |
| `ocentra-parent-agent-windows-x64-latest.msi.sha256` |     `111` | `e73faad6c63586f2e7927c6a6f8267f4f40b82c13afa1eece28fae63401113c5` |
| `ocentra-parent-agent-windows-x64-v0.1.1.msi`        | `9781248` | `920cc683445a991b8bce8874138342acb5bb9b80ab982dc616740a3ce0185c79` |
| `ocentra-parent-agent-windows-x64-v0.1.1.msi.sha256` |     `111` | `cb40f20d1fa8fe6c2ccfc55a7ea43dc074c64efcad8a0b23ab97bffce9d8f7bb` |

`latest-windows.json` identified:

- product `Ocentra Parent`;
- package `ocentra-parent-agent`;
- version `0.1.1`;
- target `windows-x64`;
- installer type `msi`;
- scope `per-machine`;
- silent args `/qn /norestart`;
- service id `OcentraParentAgent`;
- updater id `OcentraParentUpdater`;
- artifact SHA-256
  `920CC683445A991B8BCE8874138342ACB5BB9B80AB982DC616740A3CE0185C79`.

Read-only MSI metadata inspection through Windows Installer COM returned:

| MSI property     | Value                                    |
| ---------------- | ---------------------------------------- |
| `ProductName`    | `Ocentra Parent Agent`                   |
| `ProductVersion` | `0.1.1`                                  |
| `Manufacturer`   | `Ocentra`                                |
| `ProductCode`    | `{EE7DEDCF-8379-42C4-BA2A-9F39544F342C}` |
| `UpgradeCode`    | `{0143F5A1-4C10-4C0F-97BE-55EDAF5012BB}` |

`Get-Service -Name OcentraParentAgent,OcentraParentUpdater` returned no local
services before install. The current shell is not elevated
(`IsAdministrator=false`), so local MSI install/uninstall, service
registration, autostart, and reboot survival were not attempted.

Package lifecycle proof label: `ci-mechanical-proof` plus downloaded artifact
inspection. Installed lifecycle remains `manual-required`.

## Omitted Checks And Reasons

| Omitted check                          | Reason                                                                                                                       | Next owner step                                                                                                |
| -------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------- |
| Elevated MSI install/uninstall         | Current shell is not administrator and package scope is `per-machine`; installing would modify local services.               | Run from elevated PowerShell with command plan below.                                                          |
| Reboot/autostart survival              | User explicitly said not to reboot automatically.                                                                            | Run after elevated install on a dedicated proof window.                                                        |
| Update path                            | No production signed update channel is claimed by this preview artifact.                                                     | Release owner to define update proof once update manifest and signing boundary are approved.                   |
| Real app/game duration                 | No direct service command exposed app/game session duration during this proof run; memory graph duration was `null`.         | Add or use a parent-visible service/read-model path, then run a timed app/game session without inserting rows. |
| Screen queue permission/degraded state | No screen queue status command was available through the current WebSocket service path; no permission prompt was triggered. | Add or expose typed screen queue status and run with explicit Windows permission/degraded notes.               |
| Two-device LAN paired/unpaired proof   | Owned separately by B lane; this branch only ran local and CI LAN smoke.                                                     | Use B's LAN proof record for paired/unpaired household LAN evidence.                                           |

## Manual Package Lifecycle Command Plan

Run only from an elevated PowerShell prompt on a Windows proof host:

```powershell
$artifactRoot = Join-Path $env:TEMP 'ocentra-parent-run-26415925682-windows'
New-Item -ItemType Directory -Force -Path $artifactRoot
gh run download 26415925682 --repo ocentra/OcentraParent --name ocentra-parent-windows-x64-preview --dir $artifactRoot
Get-FileHash -Algorithm SHA256 -LiteralPath "$artifactRoot\ocentra-parent-agent-windows-x64-v0.1.1.msi"
msiexec /i "$artifactRoot\ocentra-parent-agent-windows-x64-v0.1.1.msi" /qn /norestart /l*v "$artifactRoot\install.log"
Get-Service OcentraParentAgent,OcentraParentUpdater
curl.exe -i http://127.0.0.1:4477/health
Restart-Computer
# After reboot:
Get-Service OcentraParentAgent,OcentraParentUpdater
curl.exe -i http://127.0.0.1:4477/health
msiexec /x "$artifactRoot\ocentra-parent-agent-windows-x64-v0.1.1.msi" /qn /norestart /l*v "$artifactRoot\uninstall.log"
Get-Service OcentraParentAgent,OcentraParentUpdater -ErrorAction SilentlyContinue
```

Record install/uninstall logs, service state, process cleanup, health response,
data retention/removal behavior, and reboot timestamps before upgrading the
package lifecycle label.

## Proof Label Summary

| Area                                                   | Current label from this pass                                                                                       |
| ------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------ |
| Baseline format/pre-AI proof                           | `implemented`                                                                                                      |
| Real Rust service health on Windows lane port          | `implemented`                                                                                                      |
| Portal-to-service on Windows lane ports                | `implemented` for load/logged command path; standard E2E also passed                                               |
| Foreground process/window read path                    | `implemented` for observed active window on this host                                                              |
| Network/process flow read path                         | `implemented` for flow/process; `degraded` for domain attribution                                                  |
| Managed browser URL/title/domain                       | `implemented` through service proof                                                                                |
| Managed browser active-tab certainty                   | `degraded` in service result (`tab-list-only`/`unknown`); protocol matrix proved active state for managed profiles |
| Browser protection/intervention                        | `scaffold-gap`                                                                                                     |
| Local AI runtime                                       | `unavailable` and local-only by design                                                                             |
| V0.7 policy preview                                    | `implemented` as dry-run with enforcement disabled                                                                 |
| App/game duration                                      | `not-yet-proven`                                                                                                   |
| Screen queue permission/degraded state                 | `manual-required` / `not-yet-proven`                                                                               |
| Windows package artifact                               | `ci-mechanical-proof` plus local artifact inspection                                                               |
| Installed package lifecycle/autostart/reboot/uninstall | `manual-required`                                                                                                  |

## Known Gaps And Risks

- This branch records controlled Windows proof, but it does not complete V0.7
  acceptance by itself. B's LAN/cross-platform proof and coordinator review are
  still required.
- The lane-port proof used temp activity storage and did not preserve raw
  SQLite, journal, or screenshots in Git.
- Network domain attribution remained unavailable in the real service payload,
  even though process-attributed network flow rows were available.
- App/game duration and screen queue proof remain unproved because the current
  parent-visible service path did not expose those results during this run.
- The broad managed-browser matrix timed out locally; the narrowed current run
  passed and is the proof used here.
- Package preview artifacts are unsigned preview mechanics. They are not
  production signing, store readiness, update, installed-service autostart, or
  reboot proof.
- The current non-elevated host could not safely install the per-machine MSI.

## V0.8/V0.9 Resume Gate

Before larger V0.8 enforcement or V0.9 LAN implementation resumes, primary
should require:

1. This Windows proof reviewed against the current `main` SHA.
2. B's LAN/cross-platform manual proof reviewed for paired/unpaired and
   platform rows.
3. Full local validation or CI validation green for the final PR branch.
4. Explicit owner decisions for app/game duration exposure, screen queue status,
   and package lifecycle proof.
5. No proof-matrix upgrades unless evidence records exist for the exact
   platform and product path.

## Roadmap Slice

V0.7 Windows/local controlled evidence and package lifecycle proof after PR #97
and PR #98. This narrows the acceptance gate by proving available Windows local
paths and preserving manual-required labels for unsupported or unsafe checks.

## PR Body Outline

```text
Scope
- Added a Windows V0.7 controlled proof record for current main b9ed9dc after PR #97/#98.
- Recorded baseline validation, lane-port Rust service and portal proof, foreground/window, network flow, managed-browser service/matrix, local-AI unavailable status, dry-run policy preview, and package artifact inspection from run 26415925682.
- Preserved manual-required/not-yet-proven labels for MSI install/autostart/reboot/uninstall, app/game duration, screen queue, update behavior, and two-device LAN.
- Kept B-owned CI evidence and C-owned portal paths untouched.

Touched files
- docs/checkpoints/v0-7-windows-controlled-evidence-and-package-lifecycle-proof-2026-05-25.md
- docs/checkpoints/artifacts/v0-7-windows-controlled-evidence-and-package-lifecycle-proof-2026-05-25/artifact-manifest.md

Validation
- cmd /c npm run format:check
- cmd /c npm run test:pre-ai-proof
- cmd /c "npm run build:contracts && cargo build -p ocentra-parent-agent-service"
- node scripts/test/real-evidence-proof-checkpoint.mjs
- cmd /c npm run test:integration
- cmd /c npm run test:e2e
- cmd /c npm run test:managed-browser-service-proof
- narrowed cmd /c npm run test:managed-browser-matrix
- gh run view 26415925682 --repo ocentra/OcentraParent --json ...
- gh api repos/ocentra/OcentraParent/actions/runs/26415925682/artifacts
- gh run download 26415925682 --repo ocentra/OcentraParent --name ocentra-parent-windows-x64-preview --dir %TEMP%\ocentra-parent-run-26415925682-windows
- git diff --check
- cmd /c npm run lanes:guard
- cmd /c npm run hub:guard
- cmd /c npm run validate

Known gaps and risks
- Non-elevated shell; MSI install/service/autostart/reboot/uninstall remain manual-required.
- Network domain attribution returned unavailable.
- App/game duration and screen queue status remain not-yet-proven through current parent-visible service paths.
- Broad managed-browser matrix timed out; narrowed matrix passed.
- Package artifacts remain preview/CI-mechanical proof, not production release proof.

Roadmap slice
- V0.7 Windows/local controlled evidence and package lifecycle proof.
```

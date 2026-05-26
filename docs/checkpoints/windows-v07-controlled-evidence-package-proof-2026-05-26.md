# Windows V0.7 Controlled Evidence Package Proof - 2026-05-26

## Scope

This record is the Worker A Windows/local proof pass after PR #101 and PR
#102 landed on `main`. It covers current `main` at
`c351dc19b9cc1a90a7b650cb2e8329bcb9618d3e` and verifies the merged Windows
package lifecycle harness against the current-main package-preview artifact.

This is still a V0.7 acceptance proof package. V0.8 enforcement and V0.9 LAN
spines remain scaffold-real proof spines only; this record does not claim
product-complete blocking, production LAN pairing, production signing, store
readiness, or mobile device-policy capability.

No database rows were manually inserted. No raw browser screenshots, child
activity, secrets, package binaries, MSI logs, or temp runtime stores are
committed. The current shell is not elevated, so the installer lifecycle proof
stops at explicit `admin-required`.

## Run Metadata

| Field                   | Value                                                                                                                   |
| ----------------------- | ----------------------------------------------------------------------------------------------------------------------- |
| Proof date              | 2026-05-26                                                                                                              |
| Worker lane             | `codex-a`                                                                                                               |
| Branch                  | `codex/windows-v07-controlled-evidence-package-proof`                                                                   |
| Commit under test       | `c351dc19b9cc1a90a7b650cb2e8329bcb9618d3e`                                                                              |
| Commit subject          | `Add Windows package lifecycle proof harness`                                                                           |
| Package/app version     | `0.1.1`                                                                                                                 |
| Host OS                 | Microsoft Windows 11 Pro `10.0.26200`, build `26200`, `64-bit`                                                          |
| Host hardware           | Gigabyte Technology Co., Ltd. `X570 AORUS MASTER`, 63.92 GB RAM                                                         |
| Toolchain               | Node `v22.22.2`, npm `11.7.0`, cargo `1.90.0`, rustc `1.90.0`, gh `2.40.1`                                              |
| Elevation state         | `not-elevated` (`IsAdministrator=false`)                                                                                |
| Lane proof ports        | Rust agent `127.0.0.1:4677`, portal `127.0.0.1:4678`                                                                    |
| Artifact manifest       | [`artifact-manifest.md`](artifacts/windows-v07-controlled-evidence-package-proof-2026-05-26/artifact-manifest.md)       |
| Sensitive data handling | Proof JSON and downloaded packages stay under ignored `test-results/`; committed docs include summaries and file paths. |

## Source Inputs

- `docs/product-roadmap.md`
- `docs/architecture/v0-7-current-main-acceptance-record-2026-05-25.md`
- `docs/architecture/current-main-proof-refresh-2026-05-25.md`
- `docs/architecture/cross-platform-deliverables-checkpoint.md`
- `docs/architecture/v07-cross-platform-proof-gap-tracker.md`
- `docs/checkpoints/windows-package-lifecycle-proof-harness-2026-05-25.md`
- `docs/checkpoints/v0-7-windows-controlled-evidence-and-package-lifecycle-proof-2026-05-25.md`
- `docs/expectations/pre-ai-proof-matrix.json`
- GitHub Actions run `26456009160`:
  <https://github.com/ocentra/OcentraParent/actions/runs/26456009160>

## Hub And Lane Setup

| Command                                                                                                                                                                                                                                                                | Result                                                                                        | Proof label   |
| ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------- | ------------- |
| `cmd /c npm run hub:inbox`                                                                                                                                                                                                                                             | Read the prepped-worktree hub mail for `codex/windows-v07-controlled-evidence-package-proof`. | `implemented` |
| `cmd /c npm run hub:ack`                                                                                                                                                                                                                                               | Passed; acknowledged `codex-a-msg-20260526T145848807Z-162`.                                   | `implemented` |
| `cmd /c npm run hub:report -- --summary "STARTED Windows V0.7 controlled evidence/package proof" ...`                                                                                                                                                                  | Passed; STARTED report `codex-a-report-20260526T150043781Z-215`.                              | `implemented` |
| `cmd /c npm run lanes:status`                                                                                                                                                                                                                                          | Passed; `codex-a` owned this branch and C remained on portal/protocol work in its own lane.   | `implemented` |
| `cmd /c npm run lanes:guard`                                                                                                                                                                                                                                           | Passed for `codex-a` on `codex/windows-v07-controlled-evidence-package-proof`.                | `implemented` |
| `cmd /c npm run hub:status`                                                                                                                                                                                                                                            | Passed; latest message was acknowledged and no A locks existed before this proof lock.        | `implemented` |
| `cmd /c npm run hub:guard`                                                                                                                                                                                                                                             | Passed before edits.                                                                          | `implemented` |
| `cmd /c npm run hub:lock -- --paths "docs/checkpoints/windows-v07-controlled-evidence-package-proof-2026-05-26.md,docs/checkpoints/artifacts/windows-v07-controlled-evidence-package-proof-2026-05-26,test-results/windows-v07-controlled-evidence-package-proof" ...` | Passed; lock stayed out of B's Linux/CI evidence files and C's portal/protocol paths.         | `implemented` |
| `git fetch --prune origin`                                                                                                                                                                                                                                             | Passed; `HEAD` and `origin/main` both resolved to `c351dc19b9cc1a90a7b650cb2e8329bcb9618d3e`. | `implemented` |

## Baseline Local Validation

| Command                                                                                                                 | Result                                                                                                | Proof label           |
| ----------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------- | --------------------- |
| `cmd /c npm run format:check`                                                                                           | Passed; all matched files used Prettier style before proof-doc edits.                                 | `implemented`         |
| `cmd /c npm run test:pre-ai-proof`                                                                                      | Passed; `11 claims checked across 5 platforms; 7 checkpoint scenarios checked.`                       | `implemented`         |
| `cmd /c node --test scripts/test/windows-package-lifecycle-proof.test.mjs scripts/test/release-windows-assets.test.mjs` | Passed; `12` tests passed.                                                                            | `implemented`         |
| `node scripts/test/real-evidence-proof-checkpoint.mjs`                                                                  | Passed; `7 scenarios checked; 5 manual-required; 1 scaffold-gap.`                                     | `implemented`         |
| `cmd /c npm run test:integration`                                                                                       | Passed; `websocket-local-smoke-ok` and `websocket-lan-smoke-ok` through the real Rust service.        | `ci-mechanical-proof` |
| `cmd /c npm run test:e2e`                                                                                               | Passed; `portal-local-smoke-ok`; Playwright `1 passed`; real portal-to-Rust command results rendered. | `ci-mechanical-proof` |

## Current Main CI And Package Preview

GitHub Actions run `26456009160` is the current-main run for
`c351dc19b9cc1a90a7b650cb2e8329bcb9618d3e`. `gh run view` reported
`status: completed`, `conclusion: success`, workflow `CI Gate`, created
`2026-05-26T14:54:28Z`, updated `2026-05-26T15:08:30Z`.

Relevant successful jobs:

- `fail-fast / Format, Lint, Types, Rust Check`
- `secret-scan / Secrets and Sensitive Files`
- `dependency-policy / Dependency Audit, Licenses, SBOM`
- `validate / Pre-AI Proof Matrix`
- `validate / Real Portal To Rust E2E (ubuntu-latest)`
- `validate / Real Portal To Rust E2E (windows-latest)`
- `validate / Real Portal To Rust E2E (macos-latest)`
- `validate / Full Validation Gate`
- `package-preview / Windows MSI Preview`
- `package-preview / Linux DEB Preview`
- `package-preview / macOS PKG Preview`
- `package-preview / Android APK Preview`
- `package-preview / iOS Simulator App Preview`

Windows preview artifact metadata from the artifact API:

| Field                | Value                                                                     |
| -------------------- | ------------------------------------------------------------------------- |
| Artifact name        | `ocentra-parent-windows-x64-preview`                                      |
| Artifact id          | `7218348399`                                                              |
| Artifact digest      | `sha256:8a782c9c45a735d9097499375f764358068bcd7b0c04ed38ce883512ea06b2d5` |
| Artifact size        | `19036659` bytes                                                          |
| Artifact created UTC | `2026-05-26T15:08:25Z`                                                    |
| Artifact expires UTC | `2026-08-24T14:54:28Z`                                                    |
| Expired              | `false`                                                                   |

This is CI/package-preview proof. It proves the hosted workflow built and smoke
checked a preview package, but it is not production signing, store readiness,
local reboot/autostart survival, data-retention proof, or an elevated install
on this Windows host.

## Lane-Port Windows Runtime Proof

A temporary ignored runner was used from:

```text
test-results/windows-v07-controlled-evidence-package-proof/runtime-lane-ports/runtime-lane-proof.mjs
```

It launched the real debug Rust service on `127.0.0.1:4677`, launched the Vite
portal on `127.0.0.1:4678/#/commands`, used temp SQLite/journal/log paths under
`%TEMP%`, sent real WebSocket commands, wrote sanitized proof JSON, and removed
the temp runtime directory after shutdown.

Proof JSON:

```text
test-results/windows-v07-controlled-evidence-package-proof/runtime-lane-ports/2026-05-26T15-10-13-499Z.json
```

Observed product-path results:

| Check                                          | Observed result                                                                                                                                                                                                                                                                                                         | Proof label                                                               |
| ---------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------- |
| Rust service health                            | `GET /health` returned HTTP `200`; WebSocket `agent.health.reported` returned `online: true`, `transport: websocket`.                                                                                                                                                                                                   | `implemented`                                                             |
| Portal shell on lane port                      | Portal HTML included `Ocentra Parent` at `127.0.0.1:4678/#/commands`; dev logs recorded agent/portal runtime startup.                                                                                                                                                                                                   | `implemented`                                                             |
| Activity ingest                                | `agent.activity.ingest.status.reported` returned `databaseReady: true`, `eventsStored: 51`, `duplicateEvents: 0`.                                                                                                                                                                                                       | `implemented`                                                             |
| Watcher status                                 | `agent.watch.status.reported` returned `available: false`.                                                                                                                                                                                                                                                              | `degraded`                                                                |
| Foreground process/window                      | `agent.activity.recent.summary.reported` returned `returned: 10`, `mostRecentKind: activity.window.focused`, `mostRecentObserver: windows-window`, `mostRecentSubjectKind: window`.                                                                                                                                     | `implemented` for real foreground-window read path on this host           |
| Activity memory graph                          | `agent.activity.memory-graph.reported` returned `capabilityStatus: ready`, `custody: child-device-activity-store`, `returned: 1`.                                                                                                                                                                                       | `implemented` for local read model                                        |
| Network flow                                   | `agent.network.flow.read-model.reported` returned `capabilityStatus: available`, `observer: windows-network`, `adapterId: windows-network-snapshot-adapter`, `returned: 10`, `processAttributionStatus: process-attributed`, `domainAttributionStatus: unavailable`, `networkProtocol: udp`, `destinationDomain: null`. | `implemented` for network/process flow; `degraded` for domain attribution |
| Managed-browser status without explicit bridge | `agent.browser.managed.status.reported` returned `managedState: running-managed`, `capabilityStatus: bridge-missing`, `queryVisibility: live-local`.                                                                                                                                                                    | `degraded`                                                                |
| Browser evidence without bridge result         | `agent.browser.evidence.recent.reported` returned `returned: 0`, `custodyLabel: unavailable`, `queryVisibility: unavailable`.                                                                                                                                                                                           | `unavailable`                                                             |
| Browser intervention read model                | `agent.browser.intervention.read-model.reported` returned `managedSessionInterventionCapability: needs-managed-session` and `unmanagedBrowserEnforcement: requires-os-app-control`.                                                                                                                                     | `scaffold-gap`                                                            |
| Local AI runtime                               | `agent.local-ai.runtime.status.reported` returned `privacyMode: local-only`, `executionState: disabled`, `executionAllowed: false`, `unavailableReason: local-ai-model-file-unconfigured`.                                                                                                                              | `unavailable` by design                                                   |
| Policy preview                                 | `agent.policy.preview.read-model.reported` returned `capabilityStatus: ready`, `custody: child-device-activity-store`, `returned: 10`, `dryRun: true`, `enforcementHandoffState: disabled`, `policyAction: unknown`, `reasonCodes: no-matching-parent-rule,local-ai-result-missing`.                                    | `implemented` for V0.7 dry-run preview; no enforcement handoff            |

## Managed Browser Proof

| Command                                                                                                                                                                                                    | Result                                                                                                                                                                                                                                                                                                                                           | Artifact                                                                                                    | Proof label                                                                                    |
| ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ----------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------- |
| `cmd /c npm run test:managed-browser-service-proof`                                                                                                                                                        | Passed with `managed-browser-service-proof-ok=true`; service path proved `url=https://example.com/?ocentra_service_proof=1`, `title=example.com`, `domain=example.com`, `activeState=unknown`, `capability=tab-list-only`.                                                                                                                       | `test-results/managed-browser-service-proof/2026-05-26T15-04-38-816Z.json`                                  | `implemented` for exact URL/title/domain service evidence; `degraded` for active-tab certainty |
| `$env:OCENTRA_PARENT_MANAGED_BROWSER_MATRIX_URLS='https://example.com/'; $env:OCENTRA_PARENT_MANAGED_BROWSER_MATRIX_PROFILES='managed-browser-profile-a'; ...; cmd /c npm run test:managed-browser-matrix` | Passed with `managed-browser-profile-matrix-ok=true`; `supportedBrowsers=3`, `managedProfiles=3`, `capturedUrls=3`, `activeProofs=3`, `historyProofs=3`; Chrome, Firefox, and Edge each reported `protocol-activated-tab-reported-visible-and-focused`; Internet Explorer entries remained `installed-unsupported`/`unsupported-legacy-browser`. | `test-results/managed-browser-profile-matrix/2026-05-26T15-04-51-640Z.json` plus ignored screenshots folder | `implemented` for managed-profile browser bridge mechanics on this host                        |

## Windows Package Lifecycle Harness Proof

Local non-elevated harness command:

```powershell
cmd /c node scripts\release\windows\package-lifecycle-proof.mjs --run-id 26456009160 --repo ocentra/OcentraParent --out-dir test-results\windows-v07-controlled-evidence-package-proof\package-lifecycle-26456009160-non-elevated --install
```

Result:

- `windows-package-lifecycle-status=ok`
- `windows-package-lifecycle-decision=admin-required`
- Proof JSON:
  `test-results/windows-v07-controlled-evidence-package-proof/package-lifecycle-26456009160-non-elevated/proof.json`

Important proof JSON fields:

| Field                                   | Value                                                                     |
| --------------------------------------- | ------------------------------------------------------------------------- |
| `artifactSource.runId`                  | `26456009160`                                                             |
| `artifactSource.id`                     | `7218348399`                                                              |
| `artifactSource.digest`                 | `sha256:8a782c9c45a735d9097499375f764358068bcd7b0c04ed38ce883512ea06b2d5` |
| `artifact.status`                       | `verified`                                                                |
| `artifact.manifest.version`             | `0.1.1`                                                                   |
| `artifact.manifest.signature.status`    | `present`                                                                 |
| `artifact.manifest.signature.algorithm` | `Ed25519`                                                                 |
| `artifact.manifest.signature.keyId`     | `f225fc81a2ee01e114a25f87d3b55a47`                                        |
| `artifact.files.versionedMsi.sha256`    | `599E938C9E6122FB0ADF08E1F0AEF27C9CFFF884ADF6D073FC713E731AE72800`        |
| `artifact.files.latestMsi.sha256`       | `599E938C9E6122FB0ADF08E1F0AEF27C9CFFF884ADF6D073FC713E731AE72800`        |
| `msiMetadata.status`                    | `read`                                                                    |
| `msiMetadata.properties.ProductName`    | `Ocentra Parent Agent`                                                    |
| `msiMetadata.properties.ProductVersion` | `0.1.1`                                                                   |
| `msiMetadata.properties.Manufacturer`   | `Ocentra`                                                                 |
| `msiMetadata.properties.ProductCode`    | `{F8492FB3-D697-438B-9966-96E51F968574}`                                  |
| `msiMetadata.properties.UpgradeCode`    | `{0143F5A1-4C10-4C0F-97BE-55EDAF5012BB}`                                  |
| `elevation.status`                      | `not-elevated`                                                            |
| `lifecycle.decision.status`             | `admin-required`                                                          |
| `lifecycle.decision.reason`             | `requires-elevated-shell`                                                 |
| `lifecycle.decision.installAttempted`   | `false`                                                                   |
| `lifecycle.decision.rebootAttempted`    | `false`                                                                   |

The harness verified the downloaded artifact shape, `latest-windows.json`,
versioned MSI, latest MSI, both checksum sidecars, bootstrap installer policy
strings, read-only MSI metadata, and host elevation state. It did not run
`msiexec.exe` from this non-elevated shell and did not attempt a reboot.

## App/Game And Screen Evidence Boundaries

| Check                                  | Observed result                                                                                                                                                                                                   | Proof label                          |
| -------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------ |
| App/game duration                      | Runtime proof produced foreground window, memory graph, and process/network evidence. It did not produce a direct parent-visible app/game duration row; no app/game duration upgrade is claimed.                  | `not-yet-proven`                     |
| Screen queue permission/degraded state | `node scripts/test/real-evidence-proof-checkpoint.mjs` still reports manual-required scenarios. No screen permission prompt or screenshot capture was run, and no screen queue status command was available here. | `manual-required` / `not-yet-proven` |

## Omitted Checks And Reasons

| Omitted check                          | Reason                                                                                                           | Next owner step                                                                                                                                       |
| -------------------------------------- | ---------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------- |
| Elevated MSI install/uninstall         | Current shell is not administrator; package scope is `per-machine`; installing would modify local services.      | Run the harness command from an Administrator PowerShell on a proof host where service install/remove is allowed.                                     |
| Reboot/autostart survival              | User explicitly prohibited automatic reboot; harness intentionally has no reboot command.                        | After an elevated install on a proof host, manually reboot, then record service state, health response, timestamps, and uninstall cleanup.            |
| Update path                            | Current artifact is a CI preview; production update channel and signing policy are not claimed here.             | Release owner should define update proof once production signing/update boundary is approved.                                                         |
| Manifest cryptographic verification    | The harness checks the preview signature envelope for presence; the trusted preview public key is not committed. | Add a trusted preview public key or publish a verifiable signature chain before claiming cryptographic manifest validation.                           |
| Real app/game duration                 | No direct parent-visible app/game session duration result was produced by this pass.                             | Add or use a service/read-model path, then run a timed app/game session through product paths without manual row insertion.                           |
| Screen queue permission/degraded state | No screen permission prompt or typed screen queue status command was executed in this pass.                      | Expose or use typed screen queue status, then record permission-required/degraded states without committing screenshots.                              |
| Two-device LAN paired/unpaired proof   | This branch only ran local and CI LAN smoke. B owns Linux/runtime/package refresh; C owns portal/protocol work.  | Primary should use the dedicated LAN proof record before larger V0.9 work resumes.                                                                    |
| Production signing/store proof         | CI preview artifacts are unsigned preview mechanics.                                                             | Production release owner must run the production-branch workflow with approved signing/store credentials when that milestone is explicitly requested. |

## Proof Label Summary

| Area                                                   | Current label from this pass                                                                        |
| ------------------------------------------------------ | --------------------------------------------------------------------------------------------------- |
| Current main CI gate                                   | `ci-mechanical-proof` and green for run `26456009160`                                               |
| Baseline format/pre-AI proof                           | `implemented`                                                                                       |
| Windows package lifecycle harness tests                | `implemented`                                                                                       |
| Real Rust service health on Windows lane port          | `implemented`                                                                                       |
| Portal-to-service on Windows lane ports                | `implemented`                                                                                       |
| Foreground process/window read path                    | `implemented` for observed active window on this host                                               |
| Network/process flow read path                         | `implemented` for flow/process; `degraded` for domain attribution                                   |
| Managed browser URL/title/domain                       | `implemented` through service proof                                                                 |
| Managed browser active-tab certainty                   | `degraded` in service result; narrowed browser matrix proved managed-profile active-state mechanics |
| Browser protection/intervention                        | `scaffold-gap`                                                                                      |
| Watcher runtime                                        | `degraded` (`available: false`)                                                                     |
| Local AI runtime                                       | `unavailable` and local-only by design                                                              |
| V0.7 policy preview                                    | `implemented` as dry-run with enforcement disabled                                                  |
| App/game duration                                      | `not-yet-proven`                                                                                    |
| Screen queue permission/degraded state                 | `manual-required` / `not-yet-proven`                                                                |
| Windows package artifact                               | `ci-mechanical-proof` plus local artifact inspection                                                |
| Installed package lifecycle/autostart/reboot/uninstall | `manual-required`                                                                                   |

## Final Branch Validation

| Command                                                                                                                                                                                                                                       | Result                                                                                                                                                                                                                                                                                                                    |
| --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `cmd /c npm run format:check`                                                                                                                                                                                                                 | Passed after proof docs were added; all matched files use Prettier style.                                                                                                                                                                                                                                                 |
| `cmd /c npm run test:pre-ai-proof`                                                                                                                                                                                                            | Passed; `11 claims checked across 5 platforms; 7 checkpoint scenarios checked.`                                                                                                                                                                                                                                           |
| `cmd /c node --test scripts/test/windows-package-lifecycle-proof.test.mjs scripts/test/release-windows-assets.test.mjs`                                                                                                                       | Passed; `12` tests passed.                                                                                                                                                                                                                                                                                                |
| `node scripts/test/real-evidence-proof-checkpoint.mjs`                                                                                                                                                                                        | Passed; `7 scenarios checked; 5 manual-required; 1 scaffold-gap.`                                                                                                                                                                                                                                                         |
| `cmd /c npm run test:integration`                                                                                                                                                                                                             | Passed; `websocket-local-smoke-ok` and `websocket-lan-smoke-ok` through the real Rust service.                                                                                                                                                                                                                            |
| `cmd /c npm run test:e2e`                                                                                                                                                                                                                     | Passed; `portal-local-smoke-ok`; Playwright `1 passed`.                                                                                                                                                                                                                                                                   |
| `cmd /c npm run test:managed-browser-service-proof`                                                                                                                                                                                           | Passed; evidence JSON `test-results/managed-browser-service-proof/2026-05-26T15-04-38-816Z.json`.                                                                                                                                                                                                                         |
| `$env:OCENTRA_PARENT_MANAGED_BROWSER_MATRIX_URLS='https://example.com/'; $env:OCENTRA_PARENT_MANAGED_BROWSER_MATRIX_PROFILES='managed-browser-profile-a'; ...; cmd /c npm run test:managed-browser-matrix`                                    | Passed; evidence JSON `test-results/managed-browser-profile-matrix/2026-05-26T15-04-51-640Z.json`; `supportedBrowsers=3`, `managedProfiles=3`, `capturedUrls=3`, `activeProofs=3`, `historyProofs=3`.                                                                                                                     |
| `cmd /c node scripts\release\windows\package-lifecycle-proof.mjs --run-id 26456009160 --repo ocentra/OcentraParent --out-dir test-results\windows-v07-controlled-evidence-package-proof\package-lifecycle-26456009160-non-elevated --install` | Passed; `windows-package-lifecycle-status=ok`, `windows-package-lifecycle-decision=admin-required`.                                                                                                                                                                                                                       |
| `gh run view 26456009160 --repo ocentra/OcentraParent --json status,conclusion,updatedAt,jobs`                                                                                                                                                | Passed; run `26456009160` was `completed` / `success` at `2026-05-26T15:08:30Z`.                                                                                                                                                                                                                                          |
| `gh api repos/ocentra/OcentraParent/actions/runs/26456009160/artifacts`                                                                                                                                                                       | Passed; Windows artifact `ocentra-parent-windows-x64-preview`, id `7218348399`, digest `sha256:8a782c9c45a735d9097499375f764358068bcd7b0c04ed38ce883512ea06b2d5`.                                                                                                                                                         |
| `cmd /c npm run validate`                                                                                                                                                                                                                     | Passed; release version, pre-AI proof, schema/string/test-double/source-shape guards, Turbo lint/type-check/test, Rust format/clippy/check/tests, integration smoke, local portal smoke, and Playwright E2E completed. Existing source-shape advisories and the Vite large-chunk warning remained warnings, not failures. |

## Known Gaps And Risks

- This branch improves the Windows/local V0.7 evidence package but does not
  complete V0.7 acceptance by itself. Coordinator review, LAN/two-device proof,
  and remaining cross-platform proof labels still matter.
- Current package lifecycle proof is non-elevated. It proves artifact integrity,
  MSI metadata, sidecars, and admin-required state, not installed service
  behavior on this host.
- Network domain attribution remained unavailable in the real service payload,
  even though process-attributed network flow rows were available.
- Browser service proof gives exact URL/title/domain, but its active state is
  still `unknown`; the narrowed matrix supplies managed-profile active-state
  mechanics separately.
- App/game duration and screen queue proof remain unproved through current
  parent-visible paths.
- CI artifacts are preview artifacts, not production signing, update, store,
  reboot/autostart, or production release proof.

## V0.8/V0.9 Resume Gate

Before larger V0.8 enforcement or V0.9 LAN implementation resumes, primary
should require:

1. Review of this Windows proof against current `main` SHA `c351dc1`.
2. Review of B/primary cross-platform proof for Linux, macOS, Android, iOS, and
   LAN rows where applicable.
3. Full branch validation and final PR CI green.
4. Explicit owner decisions for elevated package lifecycle, reboot/autostart,
   app/game duration, screen queue status, and domain attribution.
5. No proof-matrix or roadmap upgrades unless exact product-path evidence
   exists for the platform and behavior being claimed.

## Roadmap Slice

V0.7 Windows/local controlled evidence and package lifecycle proof after PR
#101/#102. This record proves the current Windows local paths that can be run
safely from this host and preserves manual-required labels for privileged,
reboot, app/game duration, screen queue, and production release behavior.

## PR Body Outline

```text
Scope
- Added a Windows V0.7 controlled evidence/package proof record for current main c351dc1 after PR #101/#102.
- Recorded local Windows service/portal lane-port proof, managed-browser service and narrowed profile matrix proof, current-main CI/package-preview status, and non-elevated package lifecycle harness output from run 26456009160.
- Preserved manual-required/not-yet-proven labels for elevated MSI install/uninstall, reboot/autostart, app/game duration, screen queue, production signing/update/store, and two-device LAN proof.
- Kept B-owned Linux/CI evidence and C-owned portal/protocol paths untouched.

Touched files
- docs/checkpoints/windows-v07-controlled-evidence-package-proof-2026-05-26.md
- docs/checkpoints/artifacts/windows-v07-controlled-evidence-package-proof-2026-05-26/artifact-manifest.md

Validation
- cmd /c npm run format:check
- cmd /c npm run test:pre-ai-proof
- cmd /c node --test scripts/test/windows-package-lifecycle-proof.test.mjs scripts/test/release-windows-assets.test.mjs
- node scripts/test/real-evidence-proof-checkpoint.mjs
- cmd /c npm run test:integration
- cmd /c npm run test:e2e
- cmd /c npm run test:managed-browser-service-proof
- narrowed cmd /c npm run test:managed-browser-matrix
- cmd /c node scripts\release\windows\package-lifecycle-proof.mjs --run-id 26456009160 ...
- gh run view 26456009160 --repo ocentra/OcentraParent --json ...
- gh api repos/ocentra/OcentraParent/actions/runs/26456009160/artifacts
- cmd /c npm run validate
- git diff --check
- cmd /c npm run lanes:guard
- cmd /c npm run hub:guard

Known gaps and risks
- Non-elevated shell; MSI install/service/autostart/reboot/uninstall remain manual-required.
- Network domain attribution returned unavailable.
- App/game duration and screen queue status remain not-yet-proven through current parent-visible paths.
- Package artifacts remain preview/CI-mechanical proof, not production release proof.

Roadmap slice
- V0.7 Windows/local controlled evidence and package lifecycle proof.
```

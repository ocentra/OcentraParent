<!-- agent-capsule -->

> Agent Capsule
> Doc: V1.0 Windows Local MVP Install Proof - 2026-05-23
> Kind: architecture/reference documentation; read only when selected by plan route, source router, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

# V1.0 Windows Local MVP Install Proof - 2026-05-23

## Scope

This record covers the V1.0 Windows-first local MVP
install/autostart/restart-survival proof slice from the current `main`
baseline. It determines what is already mechanically proven by CI package
preview and what still needs real Windows host proof before installer claims
can be upgraded.

This is a proof/status artifact only. It does not add portal content, V0.9 LAN
pairing runtime work, V0.8 enforcement runtime work, production publishing,
signing/store claims, cloud auth, billing, or fake install proof.

## Run Metadata

| Field               | Value                                                                                                             |
| ------------------- | ----------------------------------------------------------------------------------------------------------------- |
| Proof date          | 2026-05-23                                                                                                        |
| Worktree            | `C:\Users\sujan\.codex\worktrees\ocentra-parent-codex-a\OcentraParent`                                            |
| Branch              | `codex/v1-0-windows-local-mvp-install-proof`                                                                      |
| Baseline commit     | `7293d5ecc9f260e13c3c0270e36c8437645ca318`                                                                        |
| Baseline subject    | `Add V0.8 enforcement runtime spine (#69)`                                                                        |
| Package/app version | `0.1.1`                                                                                                           |
| Local host OS       | Windows 11 Pro `10.0.26200`, build `26200`, `AMD64`                                                               |
| Local host model    | `X570 AORUS MASTER`                                                                                               |
| Local shell state   | Non-elevated PowerShell; `net session` returned `System error 5 has occurred. Access is denied.`                  |
| Installed services  | `Get-Service` found no `OcentraParentAgent` or `OcentraParentUpdater` service installed before this proof record. |
| Sensitive data      | No child activity, screenshots, private browser history, or installer logs were recorded in this repo artifact.   |

## Source Inputs

- `AGENTS.md`
- `.ocentra-ai/rules/ocentra-parent-rules.mdc`
- `.ocentra-ai/rules/ocentra-parent-validation.mdc`
- `.ocentra-ai/rules/ocentra-parent-source-shape.mdc`
- `.ocentra-ai/rules/ocentra-parent-domain-boundaries.mdc`
- `docs/product-roadmap.md` V1.0
- `docs/roadmaps/roadmap-v1-0-local-mvp.md`
- `docs/expectations/release-installer.md`
- `docs/expectations/platform-deliverables.md`
- `docs/architecture/release-update.md`
- `docs/architecture/platform-package-proof-ledger-2026-05-22.md`
- `docs/architecture/cross-platform-checkpoint-proof-2026-05-23.md`
- `.github/workflows/package-preview.yml`
- `scripts/smoke/windows-msi-smoke.ps1`
- `scripts/test/platform-packaging.test.mjs`
- `scripts/test/release-windows-assets.test.mjs`

## Fresh Commands And Results

| Command                                                                                   | Result                                                                                                              | Proof label           |
| ----------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------- | --------------------- |
| `git fetch origin main`                                                                   | Passed; latest `origin/main` fetched before branch creation.                                                        | `implemented`         |
| `git switch -c codex/v1-0-windows-local-mvp-install-proof origin/main`                    | Passed; branch created from `origin/main`.                                                                          | `implemented`         |
| `cmd /c npm run lanes:guard`                                                              | Passed after branch setup.                                                                                          | `implemented`         |
| `cmd /c npm run hub:guard`                                                                | Passed after hub ack. A pre-ack guard correctly failed because this assignment mail was unread.                     | `implemented`         |
| `cmd /c npm run hub:ack`                                                                  | Passed for `codex-a-msg-20260523T163933047Z-116`.                                                                   | `implemented`         |
| `git rev-parse HEAD`                                                                      | `7293d5ecc9f260e13c3c0270e36c8437645ca318`.                                                                         | `implemented`         |
| `git log -1 --oneline`                                                                    | `7293d5e Add V0.8 enforcement runtime spine (#69)`.                                                                 | `implemented`         |
| `node -p "require('./package.json').version"`                                             | `0.1.1`.                                                                                                            | `implemented`         |
| `gh run view 26337687447 --json jobs,conclusion,status,createdAt,updatedAt,headSha,url`   | Passed; CI Gate run `26337687447` is completed with `success` for commit `7293d5e`.                                 | `ci-mechanical-proof` |
| `gh api repos/ocentra/OcentraParent/actions/runs/26337687447/artifacts ...`               | Passed; package-preview and SBOM artifacts were listed and not expired at inspection time.                          | `ci-mechanical-proof` |
| `whoami /groups`                                                                          | Passed; local Administrators group is present as `Group used for deny only`, confirming this shell is not elevated. | `manual-required`     |
| `cmd /c net session`                                                                      | Failed with `System error 5 has occurred. Access is denied.`                                                        | `manual-required`     |
| `Get-Service -Name OcentraParentAgent,OcentraParentUpdater -ErrorAction SilentlyContinue` | Passed; no matching installed services were found locally.                                                          | `manual-required`     |
| `Get-CimInstance Win32_OperatingSystem`; `Get-CimInstance Win32_ComputerSystem`           | Passed; local Windows host metadata recorded above.                                                                 | `implemented`         |

## Current Main CI Snapshot

| Field       | Value                                                               |
| ----------- | ------------------------------------------------------------------- |
| Workflow    | `CI Gate`                                                           |
| Run id      | `26337687447`                                                       |
| Run URL     | <https://github.com/ocentra/OcentraParent/actions/runs/26337687447> |
| Head SHA    | `7293d5ecc9f260e13c3c0270e36c8437645ca318`                          |
| Display     | `Add V0.8 enforcement runtime spine (#69)`                          |
| Status      | `completed`                                                         |
| Conclusion  | `success`                                                           |
| Created UTC | 2026-05-23 16:22:18 UTC                                             |
| Updated UTC | 2026-05-23 16:37:18 UTC                                             |

### Relevant CI Job Ledger

| Job name                                               | Result    | V1.0 proof meaning                                                                                 |
| ------------------------------------------------------ | --------- | -------------------------------------------------------------------------------------------------- |
| `validate / Full Validation Gate`                      | `success` | Current `main` validation passed, including real local service, LAN smoke, portal smoke, and E2E.  |
| `validate / Real Portal To Rust E2E (windows-latest)`  | `success` | Hosted Windows portal-to-Rust E2E completed against the real Rust service.                         |
| `dependency-policy / Dependency Audit, Licenses, SBOM` | `success` | Dependency policy, cargo audit, license policy, and SBOM metadata completed.                       |
| `package-preview / Windows MSI Preview`                | `success` | Windows MSI build, CI install/uninstall smoke, service start check, and artifact upload completed. |
| `package-preview / Linux DEB Preview`                  | `success` | Linux DEB build and CI install/remove smoke completed.                                             |
| `package-preview / macOS PKG Preview`                  | `success` | macOS PKG build and payload smoke completed.                                                       |
| `package-preview / Android APK Preview`                | `success` | Android APK build and emulator install/launch smoke completed.                                     |
| `package-preview / iOS Simulator App Preview`          | `success` | iOS simulator app build/install/launch smoke completed.                                            |

### Uploaded Artifact Ledger

| Artifact name                          | Size in bytes | Created UTC             | Expired | Proof level           |
| -------------------------------------- | ------------: | ----------------------- | ------- | --------------------- |
| `ocentra-parent-windows-x64-preview`   |    18,960,396 | 2026-05-23 16:37:13 UTC | `false` | `ci-mechanical-proof` |
| `ocentra-parent-linux-amd64-preview`   |     4,154,987 | 2026-05-23 16:34:30 UTC | `false` | `ci-mechanical-proof` |
| `ocentra-parent-macos-preview`         |     4,440,385 | 2026-05-23 16:33:18 UTC | `false` | `ci-mechanical-proof` |
| `ocentra-parent-android-preview`       |        11,833 | 2026-05-23 16:34:32 UTC | `false` | `ci-mechanical-proof` |
| `ocentra-parent-ios-simulator-preview` |        86,184 | 2026-05-23 16:34:20 UTC | `false` | `ci-mechanical-proof` |
| `ocentra-parent-security-sbom`         |       176,725 | 2026-05-23 16:29:21 UTC | `false` | `ci-mechanical-proof` |

These artifacts are preview artifacts. They are not production release assets,
store submissions, notarized packages, certificate-signed distribution proof, or
trusted production update-channel proof.

## What Is Already Proven

| V1.0 area                  | Current proof                                                                                                                                                      | Current label           |
| -------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ----------------------- |
| Windows MSI build          | `package-preview / Windows MSI Preview` built `ocentra-parent-agent-windows-x64-latest.msi` from current `main`.                                                   | `ci-mechanical-proof`   |
| Windows install smoke      | `scripts/smoke/windows-msi-smoke.ps1` installed the MSI with `msiexec /i ... /qn /norestart` in CI.                                                                | `ci-mechanical-proof`   |
| Service start on install   | The CI smoke script checked both `OcentraParentAgent` and `OcentraParentUpdater` with `Get-Service` and required `Status = Running` after install.                 | `ci-mechanical-proof`   |
| Windows uninstall smoke    | The CI smoke script uninstalled with `msiexec /x ... /qn /norestart` and failed if either service remained.                                                        | `ci-mechanical-proof`   |
| Diagnostic install logs    | The smoke script writes verbose MSI logs to `target/release-packages/smoke/*.log` and uploads them on failure.                                                     | `ci-mechanical-proof`   |
| Local service/portal smoke | Current `main` CI full validation and hosted Windows E2E ran real service/portal paths, but through dev/test launch, not installed MSI lifecycle.                  | `ci-mechanical-proof`   |
| Release boundary           | `docs/architecture/release-update.md`, `.github/workflows/release.yml`, and package tests preserve the `main` preview versus `production` release boundary.        | `implemented`           |
| Platform matrix honesty    | `docs/expectations/pre-ai-proof-matrix.json` keeps package-installed service/autostart as a scaffold gap rather than a completed privileged Windows install claim. | `implemented` for guard |

## What Still Needs Real Windows Proof

| V1.0 area                       | Why current proof is not enough                                                                                                  | Required real proof gate                                                                                                                                                     |
| ------------------------------- | -------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Real child-PC install           | CI installed on a hosted runner, not a chosen child Windows PC.                                                                  | Download the current `ocentra-parent-windows-x64-preview` artifact or production MSI, install from elevated PowerShell, and capture installer logs and service state.        |
| Service autostart after reboot  | CI smoke validates start-on-install only; it does not reboot the runner and confirm service recovery.                            | Reboot the Windows host after install, then record `Get-Service OcentraParentAgent,OcentraParentUpdater`, service start types, service uptime, and agent health endpoint.    |
| Restart survival                | CI does not prove journal/query continuity across a real restart.                                                                | Before reboot, write low-sensitivity evidence; after reboot, verify the service is running and the journal/query store still exposes the expected synthetic evidence refs.   |
| Local portal launch against MSI | CI full validation launches the dev/test service and portal, not a parent-visible portal flow backed by the installed MSI agent. | Open the local parent portal against the installed agent endpoint and record synthetic activity visibility without exposing private activity.                                |
| Journal/query rebuild           | Current validation proves storage paths and smoke ingest, but this worker did not run a rebuild from an installed service state. | Run the repository rebuild/check command or service-supported rebuild path against installed test data, then verify SQLite query results match journal-backed evidence refs. |
| Uninstall data behavior         | CI checks service removal only. It does not record ProgramData retention/removal expectations for a real host.                   | After uninstall, record whether services are gone, install root cleanup state, ProgramData retention state, and any intentional retained encrypted journal/query files.      |
| Update/autoupdate behavior      | Package previews use an ephemeral update key and do not publish a trusted release channel.                                       | Use a production-promoted release with trusted signing keys and record updater service behavior against a newer signed manifest.                                             |
| Production signing/store trust  | No Authenticode, store, notarization, or mobile store claim is wired for this Windows MVP proof.                                 | Add signing credentials and a release promotion proof only when explicitly requested for production distribution.                                                            |

## Local Manual-Proof Decision

This worker did not install the MSI locally because the shell is not elevated:

```text
cmd /c net session
System error 5 has occurred.
Access is denied.
```

The local services are also not currently installed:

```text
Get-Service -Name OcentraParentAgent,OcentraParentUpdater -ErrorAction SilentlyContinue
No OcentraParentAgent or OcentraParentUpdater service installed
```

Therefore, this record keeps real Windows install, reboot, autostart,
restart-survival, local portal launch against installed service, journal/query
rebuild from installed data, and uninstall data behavior as `manual-required`.
It does not upgrade those claims from the CI package-preview result alone.

## Suggested Manual Windows Proof Script

Run from an elevated PowerShell on the target Windows host after downloading the
current preview or production MSI:

```powershell
$msi = "C:\Path\To\ocentra-parent-agent-windows-x64-latest.msi"
$installLog = "$env:TEMP\ocentra-parent-msi-install.log"
$uninstallLog = "$env:TEMP\ocentra-parent-msi-uninstall.log"

msiexec /i $msi /qn /norestart /L*v $installLog
Get-Service OcentraParentAgent,OcentraParentUpdater |
  Select-Object Name, Status, StartType, ServiceType

# Open the local parent portal or run the product-approved health/evidence
# command against the installed agent. Use synthetic test activity only.

Restart-Computer

# After reboot:
Get-Service OcentraParentAgent,OcentraParentUpdater |
  Select-Object Name, Status, StartType, ServiceType

# Verify agent health, local portal visibility, and journal/query rebuild with
# synthetic evidence refs. Record command names, timestamps, and redacted output.

msiexec /x $msi /qn /norestart /L*v $uninstallLog
Get-Service OcentraParentAgent,OcentraParentUpdater -ErrorAction SilentlyContinue
```

Attach or summarize:

- installer artifact name, SHA256, and source run/release;
- install and uninstall log paths;
- service state before install, after install, after reboot, and after uninstall;
- local portal URL or command used;
- synthetic evidence ids only, not private browsing/app data;
- journal/query rebuild command and result;
- Program Files and ProgramData retention/cleanup state;
- any Windows Event Viewer service errors if service start or uninstall fails.

## Repo Change Decision

No product code, workflow, package script, or proof-matrix change is needed for
this slice. The existing Windows package preview already proves the narrow CI
mechanics it can honestly prove. The remaining gaps require an elevated local
install/reboot pass or a production-release proof pass, not a source-code change.

## Proof Matrix Handling

Do not update `docs/expectations/pre-ai-proof-matrix.json` from this record. The
existing `package-installed-service-autostart-gaps` scenario is still correct:
CI package preview proves mechanical packaging/install smoke, while real
installed service autostart, reboot survival, signing, store, notarization,
device-owner mode, and TestFlight remain outside completed runtime claims.

## Known Gaps And Risks

- This record depends on green `main` CI run `26337687447`; future source or
  workflow changes need a fresh CI snapshot.
- The local worker shell is not elevated, so no local MSI install/uninstall,
  reboot, or service autostart proof was run.
- No package-preview artifact was downloaded or installed locally in this pass.
- No local portal launch against an installed MSI service was run.
- No installed-service journal/query rebuild or post-reboot evidence continuity
  proof was run.
- No production signing, Authenticode, store distribution, or trusted updater
  release proof was run or claimed.

## Roadmap Slice

V1.0 Local MVP Windows-first install/autostart proof slice. This record makes
the install proof boundary concrete and keeps current claims honest: CI proves
preview MSI build, install/start/uninstall smoke, and artifact upload; real
Windows installed-service lifecycle proof remains manual-required.

## PR Body Outline

```text
Scope
- Added a dated V1.0 Windows Local MVP install/autostart proof record.
- Cited current green main CI run 26337687447 and its Windows MSI Preview job.
- Separated CI package-preview proof from real Windows child-PC install,
  reboot/autostart, restart survival, local portal, journal/query rebuild,
  uninstall data behavior, and production signing proof.
- Kept scope docs/proof only; no portal content, LAN pairing, V0.8 runtime,
  package workflow, production release, signing, cloud auth, or billing changes.

Touched files
- docs/architecture/v1-0-windows-local-mvp-install-proof-2026-05-23.md

Validation
- cmd /c npm run format:check
- cmd /c npm run test:pre-ai-proof
- node --test scripts/test/platform-packaging.test.mjs scripts/test/release-windows-assets.test.mjs
- git diff --check
- cmd /c npm run lanes:guard
- cmd /c npm run hub:guard

Known gaps and risks
- No local elevated MSI install/uninstall, reboot, service autostart, portal
  against installed service, journal/query rebuild, or uninstall data behavior
  proof was run.
- No production signing/store/update-channel proof was run or claimed.
- Current CI package preview remains ci-mechanical-proof only for privileged
  Windows install lifecycle claims.
```

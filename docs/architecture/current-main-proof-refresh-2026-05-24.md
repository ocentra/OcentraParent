<!-- agent-capsule -->

> Agent Capsule
> Doc: Current Main Proof Refresh - 2026-05-24
> Kind: architecture/reference documentation; read only when selected by plan route, source router, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

# Current Main Proof Refresh - 2026-05-24

## Scope

This record refreshes the current `main` proof/status checkpoint after PR #79
and PR #80 merged. It is a docs/evidence/status artifact only. It does not add
feature implementation, portal UI work, roadmap reconciliation, proof-matrix
upgrades, V0.8 adapter behavior, production LAN auth, package publishing, or
mobile entitlement claims.

The authoritative checkpoint runbook remains
`docs/architecture/cross-platform-deliverables-checkpoint.md`. The prior
checkpoint proof record,
`docs/architecture/cross-platform-checkpoint-proof-2026-05-23.md`, remains the
pattern for separating CI-mechanical proof from real OS, device, LAN, package,
signing, store, and entitlement proof.

## Run Metadata

| Field               | Value                                                                                                              |
| ------------------- | ------------------------------------------------------------------------------------------------------------------ |
| Proof date          | 2026-05-24                                                                                                         |
| Worktree            | `C:\Users\sujan\.codex\worktrees\ocentra-parent-codex-b\OcentraParent`                                             |
| Branch              | `codex/current-main-proof-refresh-post-pr80`                                                                       |
| Baseline commit     | `1719328b1524cc31ee5a9f0921a47a956b16b006`                                                                         |
| Baseline subject    | `Add V0.9 LAN audit evidence spine`                                                                                |
| Package/app version | `0.1.1`                                                                                                            |
| Latest main CI run  | `26362675528`                                                                                                      |
| Sensitive data      | No child activity, screenshots, private browser history, raw evidence payloads, decrypted logs, or device secrets. |

## Source Inputs

- `docs/architecture/cross-platform-deliverables-checkpoint.md`
- `docs/architecture/cross-platform-checkpoint-proof-2026-05-23.md`
- `docs/expectations/pre-ai-proof-matrix.json`
- PR #79: <https://github.com/ocentra/OcentraParent/pull/79>
- PR #80: <https://github.com/ocentra/OcentraParent/pull/80>
- Current main CI run:
  <https://github.com/ocentra/OcentraParent/actions/runs/26362675528>

## Fresh Commands And Results

| Command                                                                                 | Result                                                                                               | Proof label           |
| --------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------- | --------------------- |
| `git fetch origin main`                                                                 | Passed; latest `origin/main` fetched before branch creation.                                         | `implemented`         |
| `git switch -c codex/current-main-proof-refresh-post-pr80 origin/main`                  | Passed; branch created from `origin/main` at `1719328`.                                              | `implemented`         |
| `cmd /c npm run hub:ack`                                                                | Passed for `codex-b-msg-20260524T141739739Z-127`.                                                    | `implemented`         |
| `cmd /c npm run hub:lock -- --paths docs/architecture/current-main-proof-refresh-...`   | Passed; this proof/status record is the only locked edit path.                                       | `implemented`         |
| `gh pr view 79 --json number,title,mergedAt,mergeCommit,headRefName,baseRefName,body`   | Passed; PR #79 scope and merge commit recorded below.                                                | `implemented`         |
| `gh pr view 80 --json number,title,mergedAt,mergeCommit,headRefName,baseRefName,body`   | Passed; PR #80 scope and merge commit recorded below.                                                | `implemented`         |
| `gh run view 26362675528 --json jobs,conclusion,status,createdAt,updatedAt,headSha,url` | Passed; current `main` CI Gate run completed with `success` at commit `1719328`.                     | `ci-mechanical-proof` |
| `gh api repos/ocentra/OcentraParent/actions/runs/26362675528/artifacts ...`             | Passed; package-preview and SBOM artifacts are listed below and were not expired at inspection time. | `ci-mechanical-proof` |
| `cmd /c npm run format:check`                                                           | Passed on this docs-only branch before commit.                                                       | `implemented`         |
| `cmd /c npm run test:pre-ai-proof`                                                      | Passed on this docs-only branch before commit.                                                       | `implemented`         |
| `git diff --check`                                                                      | Passed on this docs-only branch before commit.                                                       | `implemented`         |
| `cmd /c npm run lanes:guard`                                                            | Passed on this docs-only branch before commit.                                                       | `implemented`         |
| `cmd /c npm run hub:guard`                                                              | Passed on this docs-only branch before commit.                                                       | `implemented`         |

## PR79 And PR80 Scope Impact

| PR  | Merge commit | Scope impact                                                                                                                                                                         | Current proof meaning                                                                                                       |
| --- | ------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | --------------------------------------------------------------------------------------------------------------------------- |
| #79 | `daee09b`    | Added the V0.8 enforcement timer recovery spine across parent-domain contracts, Rust protocol serialization, and Rust core enforcement timer event mapping and tests.                | Improves typed timer recovery evidence in contracts/core. It does not prove OS enforcement adapters, blocking, or delivery. |
| #80 | `1719328`    | Added the V0.9 LAN audit evidence spine so LAN parent intents and LAN pairing/control audit events can carry local activity-event evidence references without raw evidence payloads. | Improves typed LAN audit reviewability in contracts/service. It does not prove production LAN auth, discovery, or firewall. |

Both PRs are part of the post-V0.7 roadmap buildout. They do not change the
manual proof bar from the checkpoint runbook: privileged OS behavior, package
lifecycle behavior, two-device LAN behavior, signing, stores, and mobile
entitlements still require real host/device proof before support claims are
upgraded.

## Latest Main CI Snapshot

| Field              | Value                                                               |
| ------------------ | ------------------------------------------------------------------- |
| Workflow           | `CI Gate`                                                           |
| Run id             | `26362675528`                                                       |
| Run URL            | <https://github.com/ocentra/OcentraParent/actions/runs/26362675528> |
| Head SHA           | `1719328b1524cc31ee5a9f0921a47a956b16b006`                          |
| Display title      | `Add V0.9 LAN audit evidence spine`                                 |
| Branch             | `main`                                                              |
| Status             | `completed`                                                         |
| Conclusion         | `success`                                                           |
| Created UTC        | 2026-05-24 13:33:17 UTC                                             |
| Updated UTC        | 2026-05-24 13:49:10 UTC                                             |
| Checkpoint meaning | Current `main` is green after PR #79 and PR #80.                    |

### CI Job Ledger

| Job name                                               | Result    | CI-mechanical proof recorded                                                                       |
| ------------------------------------------------------ | --------- | -------------------------------------------------------------------------------------------------- |
| `fail-fast / Format, Lint, Types, Rust Check`          | `success` | Formatting, release-version policy, package lint, TypeScript type-check, and Rust check completed. |
| `secret-scan / Secrets and Sensitive Files`            | `success` | Repository secret scanner and Gitleaks completed.                                                  |
| `dependency-policy / Dependency Audit, Licenses, SBOM` | `success` | Dependency policy, license policy, cargo audit, and SBOM upload completed.                         |
| `validate / Pre-AI Proof Matrix`                       | `success` | Pre-AI proof matrix check completed.                                                               |
| `build / Production Build`                             | `success` | Production build gate completed.                                                                   |
| `validate / Full Validation Gate`                      | `success` | Full validation gate completed in CI.                                                              |
| `validate / Real Portal To Rust E2E (macos-latest)`    | `success` | Hosted macOS portal-to-Rust E2E completed against the real Rust service.                           |
| `validate / Real Portal To Rust E2E (windows-latest)`  | `success` | Hosted Windows portal-to-Rust E2E completed against the real Rust service.                         |
| `validate / Real Portal To Rust E2E (ubuntu-latest)`   | `success` | Hosted Ubuntu portal-to-Rust E2E completed against the real Rust service.                          |
| `package-preview / Android APK Preview`                | `success` | Android APK build and emulator install/launch smoke completed.                                     |
| `package-preview / Windows MSI Preview`                | `success` | Windows MSI build and CI install/uninstall smoke completed.                                        |
| `package-preview / macOS PKG Preview`                  | `success` | macOS PKG build and payload smoke completed.                                                       |
| `package-preview / Linux DEB Preview`                  | `success` | Linux DEB build and CI install/remove smoke completed.                                             |
| `package-preview / iOS Simulator App Preview`          | `success` | iOS simulator app build/install/launch smoke completed.                                            |

### Uploaded Artifact Ledger

| Artifact name                          | Size in bytes | Created UTC             | Expired | Proof level           |
| -------------------------------------- | ------------: | ----------------------- | ------- | --------------------- |
| `ocentra-parent-windows-x64-preview`   |    19,017,963 | 2026-05-24 13:49:04 UTC | `false` | `ci-mechanical-proof` |
| `ocentra-parent-android-preview`       |        11,830 | 2026-05-24 13:46:13 UTC | `false` | `ci-mechanical-proof` |
| `ocentra-parent-ios-simulator-preview` |        86,167 | 2026-05-24 13:46:09 UTC | `false` | `ci-mechanical-proof` |
| `ocentra-parent-linux-amd64-preview`   |     4,213,697 | 2026-05-24 13:45:55 UTC | `false` | `ci-mechanical-proof` |
| `ocentra-parent-macos-preview`         |     4,518,153 | 2026-05-24 13:45:02 UTC | `false` | `ci-mechanical-proof` |
| `ocentra-parent-security-sbom`         |       176,724 | 2026-05-24 13:40:35 UTC | `false` | `ci-mechanical-proof` |

These artifacts prove preview package mechanics only. They are not production
release assets, signed distribution proof, notarized packages, store
submissions, TestFlight proof, managed-device proof, or reboot/autostart proof.

## Current Package And Proof Labels

| Area                                    | Current evidence                                                                                                          | Current label           | Claim boundary                                                                                              |
| --------------------------------------- | ------------------------------------------------------------------------------------------------------------------------- | ----------------------- | ----------------------------------------------------------------------------------------------------------- |
| Shared validation and contracts         | Current main CI Gate passed full validation, pre-AI proof, build, lint/type-check, Rust check, and production build jobs. | `implemented`           | CI validates repo mechanics and typed contracts, not privileged OS behavior.                                |
| Real portal-to-Rust transport           | Hosted Windows, Ubuntu, and macOS portal-to-Rust E2E jobs passed against the real Rust service.                           | `ci-mechanical-proof`   | Hosted E2E proves transport mechanics only.                                                                 |
| Cross-platform package previews         | Windows MSI, Linux DEB, macOS PKG, Android APK, and iOS simulator package preview jobs all passed.                        | `ci-mechanical-proof`   | Preview artifacts do not prove production signing, stores, notarization, TestFlight, or managed devices.    |
| V0.8 enforcement timer recovery spine   | PR #79 adds typed timer recovery events and tests.                                                                        | `implemented` for spine | No OS adapter, blocking, timer enforcement delivery, rollback UI, or notification delivery claim.           |
| V0.9 LAN audit evidence spine           | PR #80 adds typed local evidence references for LAN intents and audit events.                                             | `implemented` for spine | Evidence references cite local activity-event IDs only; no raw evidence payload or production LAN auth.     |
| Package installed service and autostart | `package-installed-service-autostart-gaps` remains the proof-matrix package gap scenario.                                 | `manual-required`       | Real host install, service autostart, reboot survival, update, uninstall, and data-retention proof pending. |
| Household LAN pairing                   | CI LAN/service smoke and LAN audit evidence contracts exist.                                                              | `manual-required`       | Two real devices, firewall/router state, paired request, and failed unpaired request proof still pending.   |

## Proof Matrix Handling

Do not update `docs/expectations/pre-ai-proof-matrix.json` from this record.
The current evidence proves green current-main CI, package-preview mechanics,
V0.8 timer recovery spine contracts/core behavior, and V0.9 LAN audit evidence
contracts/service behavior. It does not prove the exact privileged/manual rows
that would justify a proof-matrix upgrade.

## Explicit Omissions

- No `docs/product-roadmap.md` edit was made because roadmap reconciliation is
  owned elsewhere.
- No `docs/expectations/pre-ai-proof-matrix.json` edit was made because this
  slice does not add concrete manual proof for a row upgrade.
- No portal, vendor, or C-owned manage IA files were touched.
- No package-preview artifact was downloaded or installed locally.
- No local Windows MSI install, elevated service check, reboot, autostart,
  update, uninstall, or data-retention proof was run.
- No macOS host, Android physical device, iOS device, TestFlight, store,
  signing, notarization, or entitlement proof was run.
- No two-device LAN pairing, firewall/router behavior, paired request, or
  failed unpaired request proof was run.
- No local AI model execution, V0.8 OS enforcement adapter, notification
  delivery, cloud relay, billing, or production auth proof was run.

## Known Gaps And Risks

- Current `main` is green after PR #79 and PR #80, but green CI is still
  CI-mechanical proof for package and platform behavior.
- V0.8 has a typed timer recovery spine, but real enforcement adapters,
  blocking behavior, timer delivery, rollback UX, and notifications still need
  future implementation and proof.
- V0.9 LAN audit evidence is typed and service-backed, but production LAN auth,
  discovery, firewall behavior, two-device proof, and manual household proof
  remain manual-required.
- Windows package lifecycle claims still need elevated real-host install,
  reboot/autostart, update, uninstall, data-retention, and installed-service
  portal proof.
- Mobile and Apple platform claims still require physical-device, signing,
  TestFlight, entitlement, store, and managed-device evidence where applicable.

## Roadmap Slice

Current-main proof/status refresh after PR #79 and PR #80. This record keeps
the project reviewable at commit `1719328` without overstating support claims
or changing roadmap/proof-matrix state.

## PR Body Outline

```text
Scope
- Added a dated current-main proof/status refresh for commit 1719328 after PR #79 and PR #80.
- Captured current main CI Gate run 26362675528, job results, package-preview artifacts, and PR79/PR80 scope impact.
- Recorded current package/proof labels and manual-required gaps without upgrading roadmap or proof-matrix claims.
- Kept scope docs/evidence/status only; no product code, portal/vendor files, roadmap edit, proof-matrix edit, feature implementation, package publishing, or production auth changes.

Touched files
- docs/architecture/current-main-proof-refresh-2026-05-24.md

Validation
- cmd /c npm run format:check
- cmd /c npm run test:pre-ai-proof
- git diff --check
- cmd /c npm run lanes:guard
- cmd /c npm run hub:guard

Known gaps and risks
- CI and package previews are green but remain CI-mechanical proof for privileged package/platform behavior.
- Real Windows install/reboot/autostart/update/uninstall/data-retention proof remains manual-required.
- Two-device LAN pairing/firewall/manual household proof remains manual-required.
- macOS signing/notarization, Android physical-device behavior, iOS TestFlight/entitlements, stores, production auth, AI execution, and enforcement adapter proof remain unproven.
```

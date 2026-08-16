<!-- agent-capsule -->

> Agent Capsule
> Doc: V0.7 Checkpoint Acceptance Summary - 2026-05-22
> Kind: architecture/reference documentation; read only when selected by plan route, source router, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

# V0.7 Checkpoint Acceptance Summary - 2026-05-22

## Scope

This summary records the V0.7 checkpoint state after PR #60, PR #61, PR #63,
and PR #64 landed on `main`. It is a docs-only acceptance handoff: no product
code, no proof-matrix upgrade, no V0.8 enforcement, and no model execution.

## Current Main State

| Field                  | Value                                                                 |
| ---------------------- | --------------------------------------------------------------------- |
| Branch                 | `main`                                                                |
| Current head           | `30d8846e5cf8d9c3b4d07b72b4bbe36347a87832`                            |
| Current head subject   | `Record Windows LAN checkpoint proof`                                 |
| Package/app version    | `0.1.1`                                                               |
| Latest main CI run     | `26307438103`                                                         |
| Latest main CI URL     | <https://github.com/ocentra/OcentraParent/actions/runs/26307438103>   |
| Latest main CI status  | `completed`                                                           |
| Latest main CI result  | `success`                                                             |
| Latest main CI window  | Created 2026-05-22 19:20:46 UTC; completed 2026-05-22 19:32:17 UTC    |
| Superseded main CI run | `26307386665` was cancelled after PR #64 advanced `main` past PR #63. |

## Merged Checkpoint Work

| PR  | Merge commit | Scope on `main`                                                                                                                                                                                                     | Acceptance meaning                                                                                                                                                        |
| --- | ------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| #60 | `1db6691`    | Added `docs/architecture/v0.7-checkpoint-validation-evidence-report-2026-05-22.md`, recording a full current-main validation pass after PR #59.                                                                     | `main` had a locally reproduced checkpoint validation record with `format:check`, `test:pre-ai-proof`, and full `validate` passing.                                       |
| #61 | `7d110b6`    | Tightened `docs/architecture/cross-platform-deliverables-checkpoint.md` and roadmap next actions into an executable manual proof runbook.                                                                           | The checkpoint has an operator checklist and proof-label vocabulary for separating CI mechanics from real OS/device proof.                                                |
| #63 | `4292f66`    | Added `docs/architecture/platform-package-proof-ledger-2026-05-22.md`, recording CI Gate run `26304936088`, preview package jobs, uploaded artifacts, and proof labels.                                             | Package-preview and cross-platform CI mechanics are captured as `ci-mechanical-proof`, without upgrading signing, store, or device claims.                                |
| #64 | `30d8846`    | Added `docs/architecture/windows-lan-checkpoint-proof-2026-05-22.md`, recording Windows host metadata, LAN interface metadata, carried-forward local/LAN proof, controlled evidence status, and explicit omissions. | Windows real-PC metadata and local proof records are consolidated while two-device LAN, package lifecycle, managed browser, duration, and screen queue proof stay honest. |

## Current CI Acceptance

The latest `main` CI Gate run, `26307438103`, completed successfully on commit
`30d8846e5cf8d9c3b4d07b72b4bbe36347a87832`.

The run is the current acceptance baseline for CI-mechanical checkpoint proof.
It replaces the cancelled post-PR #63 `main` run because PR #64 advanced `main`
before that earlier run could finish.

Accepted CI proof level:

- format, lint, type-check, Rust check, secret scan, build, dependency policy,
  license policy, SBOM, and full validation mechanics;
- `test:pre-ai-proof` structural validation for 11 claims across 5 platforms
  and 7 checkpoint scenarios;
- real portal-to-Rust E2E on hosted Windows, Ubuntu, and macOS runners;
- package-preview mechanics for Windows MSI, Linux DEB, macOS PKG, Android APK,
  and iOS simulator app paths where the CI runner supports the operation.

Not accepted from CI alone:

- real household LAN pairing;
- Windows service autostart after install and reboot;
- foreground browser URL/title capture through a managed session;
- timed app/game duration;
- screen evidence queue permission and deletion-state behavior;
- production signing, notarization, store, TestFlight, device-owner, or mobile
  entitlement behavior.

## PR #62 Blocker

PR #62, `Build parent portal product shell`, remains open:
<https://github.com/ocentra/OcentraParent/pull/62>.

Its CI Gate run `26306115584` failed:
<https://github.com/ocentra/OcentraParent/actions/runs/26306115584>.

The failure is in `fail-fast / Format, Lint, Types, Rust Check`, job
`77443263462`. Format and release-version checks passed, but the lint step
failed during `@ocentra-parent/parent-desktop#type-check`, which runs
`npm run tauri:check` and then `cargo check --manifest-path src-tauri/Cargo.toml`.

The failing Linux runner did not have required Tauri/GTK system libraries for
the parent desktop scaffold:

- `gdk-3.0` required by `gdk-sys` was not found.
- `gio-2.0` required by `gio-sys` was not found.
- `glib-2.0` required by `glib-sys` was not found.
- `gobject-2.0` required by `gobject-sys` was not found.

Checkpoint interpretation:

- PR #62 is not counted as accepted `main` proof.
- The parent portal product shell and parent-desktop scaffold remain blocked
  on CI environment/dependency handling for Linux Tauri checks.
- This blocker does not invalidate the current `main` V0.7 checkpoint baseline,
  because PR #62 has not merged.

## Remaining Manual-Required Or Not-Yet-Proven Gaps

The V0.7 checkpoint can proceed only as a pre-enforcement, pre-real-model
acceptance baseline. The following gaps must stay explicit before V0.8:

| Area                                       | Current state                                                                                                                                     | Label                                                            |
| ------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------- |
| Managed browser exact URL/title            | A controlled Edge DevTools target was visible externally, but the Rust service bridge returned `browser-bridge-io-error` and stored no URL/title. | `not-yet-proven` through product service                         |
| Foreground Windows app/window              | Fresh synthetic Notepad foreground-window capture was proven through the service path and memory graph.                                           | `implemented` for the controlled Windows host proof              |
| Timed app/game duration                    | Current exposed evidence graph did not include `durationMs` or `observedUntil`; no fresh app/game duration read model was proven.                 | `not-yet-proven`                                                 |
| Network/domain attribution                 | Windows network read model reports process attribution while preserving domain attribution as unavailable.                                        | `manual-required` for controlled domain proof                    |
| Screen evidence queue                      | Protocol types exist, but no exposed service command proved live queue state, permission state, mutation, or deletion behavior.                   | `not-yet-proven`                                                 |
| Loopback service and parent portal         | Real Rust service, WebSocket, health, portal smoke, and command rendering paths are proven through local/manual and CI paths.                     | `implemented` for loopback mechanics                             |
| Single-machine LAN substitute              | LAN bind/origin mechanics were observed on one Windows PC at `192.168.2.10`.                                                                      | `ci-mechanical-proof` plus `manual-required` for household proof |
| Two-device LAN pairing                     | No second physical parent device, explicit pairing step, paired request, or failed unpaired request was run.                                      | `manual-required`                                                |
| Package install/autostart/reboot/uninstall | CI previews prove package mechanics, but real installed service lifecycle, reboot survival, update, uninstall, and data-retention were not run.   | `manual-required` and `scaffold-only` by platform                |
| Signing, stores, notarization, TestFlight  | No production signing, notarization, app-store, TestFlight, device-owner, or entitlement evidence exists.                                         | `manual-required`, `permission-required`, or `blocked`           |
| Local AI model execution                   | Runtime/provider status is local-only, degraded/unavailable, and execution-disabled.                                                              | `implemented` for honest disabled status; no execution claim     |
| Enforcement                                | Policy preview remains dry-run only and enforcement handoff is disabled.                                                                          | no V0.8 enforcement claim                                        |

## Acceptance Position

The accepted V0.7 state on `main` is:

- CI-mechanical proof is green on the latest `main` commit.
- Checkpoint validation, cross-platform runbook, package proof ledger, and
  Windows/LAN proof records are committed.
- The current proof records distinguish real product proof from scaffold,
  manual-required, and not-yet-proven states.
- No V0.8 enforcement, blocking, notification delivery, or real model execution
  has been introduced.

The checkpoint is not a production support claim. It is an acceptance baseline
for deciding what must be manually proven before enforcement or real local model
execution resumes.

## PR Body Outline

```text
Scope
- Added a V0.7 checkpoint acceptance summary after PR #60/#61/#63/#64.
- Recorded current main head and latest green main CI Gate run.
- Summarized what checkpoint proof records are now on main.
- Documented the open PR #62 blocker and remaining manual-required/not-yet-proven gaps.
- Kept scope docs-only; no product code, proof-matrix upgrade, enforcement, or model execution.

Touched files
- docs/architecture/v0-7-checkpoint-acceptance-summary-2026-05-22.md

Validation
- cmd /c npm run format:check
- cmd /c npm run test:pre-ai-proof
- git diff --check
- cmd /c npm run lanes:guard
- cmd /c npm run hub:guard

Known gaps and risks
- PR #62 remains blocked by Linux Tauri system-library requirements for parent-desktop checks.
- Managed browser URL/title, timed app/game duration, screen queue status, two-device LAN, package lifecycle, signing, stores, TestFlight, device-owner, and entitlement proof remain manual-required or not-yet-proven.

Roadmap slice
- V0.7 checkpoint acceptance summary before V0.8 enforcement or real model execution.
```

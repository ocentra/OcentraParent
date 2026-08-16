<!-- agent-capsule -->

> Agent Capsule
> Doc: Windows And LAN Checkpoint Proof - 2026-05-22
> Kind: architecture/reference documentation; read only when selected by plan route, source router, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

# Windows And LAN Checkpoint Proof - 2026-05-22

## Run Metadata

- Run name: V0.7 Windows and LAN checkpoint proof record
- Commit: `7d110b67661ad4c9b42fc7b3237aa698f51f41df`
- Branch: `codex/windows-lan-checkpoint-proof-record`
- Package/app version: `0.1.1`
- Mode: docs-only checkpoint proof ledger from current main plus fresh host metadata
- Child device: Windows 11 Pro `10.0.26200`, 64-bit, `GAMEDEV`, Gigabyte Technology Co. `X570 AORUS MASTER`
- Parent device: same Windows PC for local notes; no second physical parent device was available in this pass
- Network state: `Ethernet 2`, `192.168.2.10`; gateway `192.168.2.1`; firewall/router state not changed
- Sensitive details minimized: yes; this repo record does not include raw activity digests, raw browser history, screenshots, private logs, package paths containing user data, or temp database files

## Fresh Commands Run In This Pass

| Command                                                                                                               | Result                                                                                                     | Proof label       |
| --------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------- | ----------------- |
| `git status --short --branch`                                                                                         | Clean before edits: `## codex/windows-lan-checkpoint-proof-record...origin/main`                           | `implemented`     |
| `git rev-parse HEAD`                                                                                                  | `7d110b67661ad4c9b42fc7b3237aa698f51f41df`                                                                 | `implemented`     |
| `node -p "require('./package.json').version"`                                                                         | `0.1.1`                                                                                                    | `implemented`     |
| `Get-ComputerInfo \| Select-Object OsName, OsVersion, OsArchitecture, CsName, CsManufacturer, CsModel \| Format-List` | Confirmed the Windows 11 Pro real-PC host metadata listed above                                            | `implemented`     |
| `Get-NetIPConfiguration \| Select-Object InterfaceAlias,IPv4Address,IPv4DefaultGateway,DnsServer \| Format-List`      | Confirmed the active `Ethernet 2` LAN interface at `192.168.2.10`; other local/virtual adapters also exist | `manual-required` |
| `cmd /c npm run lanes:status`                                                                                         | Passed; codex-a is claimed on `codex/windows-lan-checkpoint-proof-record`                                  | `implemented`     |
| `cmd /c npm run lanes:guard`                                                                                          | Passed before editing                                                                                      | `implemented`     |
| `cmd /c npm run hub:status`                                                                                           | Passed; latest codex-a message is acknowledged                                                             | `implemented`     |
| `cmd /c npm run hub:guard`                                                                                            | Passed before editing                                                                                      | `implemented`     |

## Evidence Records Already On Main

This pass did not duplicate raw local screenshots or temp databases. It uses the
following repo records as the current committed evidence baseline and preserves
their labels:

| Evidence record                                                           | Current proof carried forward                                                                                                                                                                                                                                                                                      | Proof label                                                                                                                                         |
| ------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | --------------------------------------------------------------------------------------------------------------------------------------------------- |
| `docs/architecture/local-lan-manual-proof-results-2026-05-22.md`          | Records full `cmd /c npm run validate`, loopback real Rust service plus portal proof on ports `4577`/`4578`, single-machine LAN-bind substitute at `192.168.2.10`, service health payload, portal WebSocket command visibility, local AI runtime degraded status, and package lifecycle omissions.                 | `implemented` for loopback real-service reachability; `ci-mechanical-proof` plus `manual-required` for single-machine LAN substitute                |
| `docs/architecture/controlled-local-evidence-proof-results-2026-05-22.md` | Records a controlled Edge DevTools target, service bridge failure for managed browser ingestion, fresh Notepad foreground-window evidence, activity memory graph visibility, no timed duration, no exposed screen queue command, and passed `format:check`, `test:pre-ai-proof`, `git diff --check`, and validate. | `implemented` for fresh Windows foreground-window capture; `not-yet-proven` for browser URL/title through service, timed duration, and screen queue |

## Windows Real-PC Checkpoint State

| Checkpoint row                        | Evidence in current repo state                                                                                                                                                             | Proof label                                                            | Follow-up owner / next action                                                                                       |
| ------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ---------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------- |
| Real Windows host metadata            | Fresh command output confirms Windows 11 Pro `10.0.26200`, 64-bit, physical PC model, and active Ethernet interface.                                                                       | `implemented`                                                          | None for metadata.                                                                                                  |
| Shared local validation               | This branch ran `format:check`, `test:pre-ai-proof`, `git diff --check`, `lanes:guard`, and `hub:guard`; the commit hook also ran full `validate`.                                         | `implemented` for the docs-only focused gate plus full pre-commit gate | Run fresh manual runtime proof if this record is expanded beyond docs or a coordinator requests current-host proof. |
| Loopback real service and portal      | Carried forward from `local-lan-manual-proof-results-2026-05-22.md`: real Rust service, portal, health payload, WebSocket command results, and portal overview were observed on this host. | `implemented` for the recorded commit and local run                    | Rerun when current checkpoint requires fresh runtime proof or when product code changes.                            |
| Fresh foreground app/window evidence  | Carried forward from controlled proof: Notepad synthetic window was captured through the service read path and memory graph.                                                               | `implemented` for fresh Windows foreground-window capture              | Keep sensitive app/window details synthetic in future records.                                                      |
| Managed browser exact URL/title       | Controlled proof saw the live Edge DevTools target externally, but the Rust service bridge returned `browser-bridge-io-error` and no recent URL/title row.                                 | `not-yet-proven`                                                       | Fix/prove the managed browser bridge through product service before upgrading.                                      |
| Timed app/game duration               | Controlled proof returned foreground graph edges without `durationMs` or `observedUntil`; no exposed read model proved timed duration.                                                     | `not-yet-proven`                                                       | Add or expose a real duration read path before claiming app/game duration.                                          |
| Network/domain attribution            | Local/LAN record observed Windows network read-model availability with process attribution and explicit domain attribution `unavailable`.                                                  | `manual-required`                                                      | Run a controlled network/domain proof with known synthetic destination and required permissions before upgrading.   |
| Screen evidence queue                 | Controlled proof found protocol types but no service command exposing live screen queue status.                                                                                            | `not-yet-proven`                                                       | Expose and prove a real queue status command before claiming screen queue behavior.                                 |
| Local AI runtime/provider status      | Prior proof observed `privacyMode=local-only`, local adapter unavailable, execution disabled, provider unavailable, and `executionAllowed=false`.                                          | `implemented` for honest degraded/local-only status                    | Do not run or claim model execution until the user explicitly resumes that track.                                   |
| Policy preview / enforcement boundary | Prior proof observed dry-run policy preview with enforcement handoff disabled.                                                                                                             | `implemented` for dry-run preview boundary; no enforcement claim       | V0.8 enforcement remains out of scope.                                                                              |
| Package install/autostart/reboot      | No installable artifact, service autostart, reboot survival, uninstall cleanup, signing, store, or data-retention check was run in this worker pass.                                       | `manual-required` and `scaffold-only` where CI package previews apply  | Run against actual package artifacts on the target host before upgrading installer claims.                          |

## LAN Checkpoint State

| Checkpoint row                     | Evidence in current repo state                                                                                                                                                                      | Proof label                | Follow-up owner / next action                                                                                           |
| ---------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------- | ----------------------------------------------------------------------------------------------------------------------- |
| LAN interface metadata             | Fresh `Get-NetIPConfiguration` confirms a reachable local interface candidate: `Ethernet 2` at `192.168.2.10` with gateway `192.168.2.1`.                                                           | `implemented` for metadata | Record firewall/router state during any two-device run.                                                                 |
| Single-machine LAN bind substitute | Prior local/LAN proof ran `cmd /c npm run dev:lan`, bound agent and portal to `0.0.0.0`, used `192.168.2.10`, checked health, portal HTTP, WebSocket, wrong-port failure, and CORS origin behavior. | `ci-mechanical-proof`      | Repeat after product code changes or before checkpoint acceptance if a fresh current-commit LAN substitute is required. |
| Two-device parent-to-child LAN     | No second physical parent device was available in this pass; no pairing step, paired request, or failed unpaired request was run.                                                                   | `manual-required`          | Run with two real devices, explicit pairing, allowed origin, and failed unpaired request before upgrading.              |
| LAN authenticated control          | Current evidence is dev LAN smoke and origin checks only; trusted-device registry and authenticated remote control are out of V0.7 scope.                                                           | `not-yet-proven`           | Keep control/enforcement claims out until V0.9 or later implementation is assigned.                                     |

## Validation

| Command                            | Result                                                                                                                                                                                                                                                                    |
| ---------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `cmd /c npm run format:check`      | Passed: all matched files use Prettier code style                                                                                                                                                                                                                         |
| `cmd /c npm run test:pre-ai-proof` | Passed: `pre-ai-proof-ok: 11 claims checked across 5 platforms; 7 checkpoint scenarios checked.`                                                                                                                                                                          |
| `git diff --check`                 | Passed                                                                                                                                                                                                                                                                    |
| `cmd /c npm run lanes:guard`       | Passed before and after edit                                                                                                                                                                                                                                              |
| `cmd /c npm run hub:guard`         | Passed before and after edit                                                                                                                                                                                                                                              |
| Pre-commit `npm run validate`      | Passed: version alignment, pre-AI proof, schema/source/test-double guards, Turbo lint/type-check/test, Rust checks/tests, local WebSocket smoke, LAN smoke, portal local smoke, portal Playwright E2E, and build. Source-shape warnings remain advisory and pre-existing. |

## Known Gaps And Risks

- This is a docs/proof-record branch only. It does not implement product code,
  V0.8 enforcement, model execution, package changes, or proof-matrix upgrades.
- Two-device LAN proof remains manual-required because this worker pass had only
  a single Windows PC context.
- Managed browser exact URL/title evidence remains not-yet-proven through the
  service bridge despite the browser exposing a DevTools target externally.
- Timed app/game duration and screen evidence queue status remain not-yet-proven
  through current exposed service commands.
- Package install, autostart, reboot survival, update, uninstall, signing,
  notarization, store, mobile entitlement, and device-owner claims remain
  manual-required or scaffold-only until run against real artifacts and devices.

## Proof Matrix Follow-Up

Do not upgrade `docs/expectations/pre-ai-proof-matrix.json` from this record.
The record consolidates current Windows/LAN evidence and omissions, but it does
not add new product proof beyond host metadata and validation commands on this
branch.

## PR Body Outline

```text
Scope
- Added a Windows/LAN checkpoint proof record for 2026-05-22.
- Recorded current branch, host, network, and coordination command evidence.
- Carried forward existing committed local/LAN and controlled local proof labels without overclaiming.
- Kept unrun two-device LAN, package lifecycle, managed browser, timed duration, and screen queue checks as manual-required or not-yet-proven.
- Kept scope docs-only; no product code, enforcement, model execution, or proof-matrix upgrade.

Touched files
- docs/architecture/windows-lan-checkpoint-proof-2026-05-22.md

Validation
- cmd /c npm run format:check
- cmd /c npm run test:pre-ai-proof
- git diff --check
- cmd /c npm run lanes:guard
- cmd /c npm run hub:guard
- pre-commit npm run validate

Known gaps and risks
- Two-device LAN pairing and failed-unpaired request proof still require a second real device.
- Package install/autostart/reboot/uninstall behavior still requires real artifacts.
- Managed browser URL/title, timed app/game duration, and screen queue status remain not-yet-proven through product service commands.

Roadmap slice
- V0.7 Windows real-PC and LAN checkpoint evidence record before further AI or enforcement work.
```

<!-- agent-capsule -->

> Agent Capsule
> Doc: Local And LAN Manual Proof Results - 2026-05-22
> Kind: architecture/reference documentation; read only when selected by plan route, source router, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# Local And LAN Manual Proof Results - 2026-05-22

## Run Metadata

- Run name: V0.7 local/LAN manual proof pass from current main
- Commit: `4a04d68ffc9cf83c0ea2f1afe794844573ec5b35`
- Branch: `codex/v07-local-lan-manual-proof-pass`
- Package/app version: `0.1.1`
- Mode: loopback and single-machine LAN-bind substitute
- Child device: Windows 11 Pro `10.0.26200`, 64-bit, `GAMEDEV`, Gigabyte Technology Co. `X570 AORUS MASTER`
- Parent device: same machine for loopback and LAN substitute; no second physical parent device was available in this pass
- Network state: `Ethernet 2`, `192.168.2.10/24`, gateway `192.168.2.1`; firewall state not changed
- Sensitive details minimized: yes; this repo record redacts raw activity digests, raw browser/window history, and private screenshots

## Baseline Gate

| Command                            | Result                                                                                                                                                                                                                                                                           |
| ---------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `git status --short --branch`      | Clean: `## codex/v07-local-lan-manual-proof-pass...origin/main`                                                                                                                                                                                                                  |
| `git rev-parse HEAD`               | `4a04d68ffc9cf83c0ea2f1afe794844573ec5b35`                                                                                                                                                                                                                                       |
| `cmd /c npm run format:check`      | Passed: Prettier reported all matched files use code style                                                                                                                                                                                                                       |
| `cmd /c npm run test:pre-ai-proof` | Passed: `pre-ai-proof-ok: 11 claims checked across 5 platforms; 7 checkpoint scenarios checked.`                                                                                                                                                                                 |
| `cmd /c npm run validate`          | Passed: version alignment, pre-AI proof, schema/source/test-double guards, Turbo lint/type-check/test, Rust checks/tests, local WebSocket smoke, LAN smoke, portal local smoke, and portal Playwright E2E all completed. Source-shape warnings remain advisory and pre-existing. |

## Loopback Proof

- Agent command: `OCENTRA_PARENT_AGENT_PORT=4577`, `OCENTRA_PARENT_PORTAL_PORT=4578`, `cmd /c npm run dev:agent`
- Portal command: `OCENTRA_PARENT_AGENT_PORT=4577`, `OCENTRA_PARENT_PORTAL_PORT=4578`, `cmd /c npm run dev:portal`
- Agent URL: `http://127.0.0.1:4577/health`
- Portal URL: `http://127.0.0.1:4578/#/commands`
- WebSocket URL: `ws://127.0.0.1:4577/api/dev/ws`
- Observed result: agent process served health from PID `42556`; portal process served Vite from PID `27464`
- Health payload: `schemaVersion=1`, `deviceId=local-dev-agent`, `hostname=GAMEDEV`, `platform=windows`, `serviceVersion=0.1.1`, `captureEnabled=true`, `policyEngineEnabled=false`
- Portal HTTP result: `HTTP/1.1 200 OK`, title `Ocentra Parent Dev Portal`
- Portal-driven proof: headless Chromium loaded the portal and observed the live service events below through the parent surface:
  - `agent.health.reported`
  - `agent.activity.ingest.status.reported`
  - `agent.activity.recent.summary.reported`
  - `agent.browser.evidence.recent.reported`
  - `agent.activity.memory-graph.reported`
  - `agent.browser.intervention.read-model.reported`
  - `agent.browser.managed.status.reported`
  - `agent.network.flow.read-model.reported`
  - `agent.local-ai.runtime.status.reported`
  - `agent.policy.preview.read-model.reported`
- Overview proof: portal overview contained `Agent WebSocket connected`, `Live activity`, `Network flow`, `Policy preview`, and `Enforcement disabled; preview only.`
- Temporary local artifacts: `C:\Users\sujan\AppData\Local\Temp\ocentra-proof-a-20260522-123543\portal-overview-4578.png`, `portal-proof-summary.json`, and `websocket-proof-summary.json`. These were not committed because they may contain local machine activity details.
- Proof label: `implemented` for loopback real-service reachability and parent-surface command visibility; `ci-mechanical-proof` for repeatable evidence read-model mechanics; privileged OS behavior remains scoped in the rows below.

## Evidence Preview Checks

| Check                                   | Observed result                                                                                                                                                                                                                                  | Proof label                                                                            | Notes                                                                                                                                                 |
| --------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | -------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------- |
| Managed browser URL/tab evidence        | Service returned `agent.browser.evidence.recent.reported` with `returned=0`, `queryVisibility=unavailable`; managed bridge status returned `running-managed` plus `bridge-missing`.                                                              | `manual-required`                                                                      | No Ocentra-managed browser session with a known test URL was launched during this pass. Exact URL/title proof remains manual-required.                |
| Foreground process/window evidence      | Recent summary returned real stored Windows window evidence with `mostRecentKind=activity.window.focused`, observer `windows-window`, and parent surface rendered the result.                                                                    | `manual-required`                                                                      | Confirms real stored service/read-model path; this pass did not perform a fresh foreground-app choreography with a known synthetic app.               |
| Network/domain evidence                 | Network flow read model returned `agent.network.flow.read-model.reported`, `adapterId=windows-network-snapshot-adapter`, `capabilityStatus=available`, `processAttributionStatus=process-attributed`, and `domainAttributionStatus=unavailable`. | `manual-required`                                                                      | Confirms typed local read-model path and unknown domain attribution is preserved; no privileged packet, VPN, or admin visibility proof was attempted. |
| App/game session evidence               | Activity memory graph returned `capabilityStatus=ready`, `custody=child-device-activity-store`, and stored app/game-classified evidence references.                                                                                              | `manual-required`                                                                      | Confirms stored evidence graph path; this pass did not run a fresh timed app/game session with controlled start/stop times.                           |
| Screen evidence queue state             | No dedicated screen-evidence queue command was exposed through this manual portal pass.                                                                                                                                                          | `not-yet-proven`                                                                       | Real screen permission, queue mutation, deletion, and no-raw-screenshot behavior still require a focused host-permission pass.                        |
| Parent-rule context and dry-run preview | Policy preview returned `dryRun=true`, `enforcementHandoffState=disabled`, `policyAction=unknown`, `reasonCodes=no-matching-parent-rule,local-ai-result-missing`, and one evidence reference.                                                    | `implemented` for dry-run read path; `manual-required` for richer parent-rule scenario | Enforcement remained disabled. This pass did not author a new local parent rule for a controlled target.                                              |
| Local provider/runtime status           | Runtime status returned `privacyMode=local-only`, `adapterBoundary=local-adapter-unavailable`, `executionState=disabled`, `providerSource=unavailable`, and `executionAllowed=false`.                                                            | `implemented` for degraded/local-only status                                           | No real model execution was attempted or claimed.                                                                                                     |

## LAN Substitute Proof

- LAN command: `OCENTRA_PARENT_DEV_NETWORK=lan`, `OCENTRA_PARENT_LAN_HOST=192.168.2.10`, `OCENTRA_PARENT_AGENT_PORT=4577`, `OCENTRA_PARENT_PORTAL_PORT=4578`, `cmd /c npm run dev:lan`
- Agent bind: `0.0.0.0:4577`
- Portal bind: `0.0.0.0:4578`
- Portal URL: `http://192.168.2.10:4578/#/commands`
- Health check: `curl.exe -i --max-time 10 http://192.168.2.10:4577/health` returned `HTTP/1.1 200 OK` with `platform=windows`, `serviceVersion=0.1.1`, `captureEnabled=true`, `policyEngineEnabled=false`
- Portal check: `curl.exe -i --max-time 10 http://192.168.2.10:4578/` returned `HTTP/1.1 200 OK` and the `Ocentra Parent Dev Portal` document
- WebSocket check: `ws://192.168.2.10:4577/api/dev/ws` with target route `local-network` returned `agent.health.reported`
- Negative wrong-port check: `curl.exe -i --max-time 5 http://192.168.2.10:4579/health` failed to connect
- Origin check: `Origin: http://192.168.2.10:4578` returned `access-control-allow-origin: http://192.168.2.10:4578`; `Origin: http://example.invalid` returned no `access-control-allow-origin`
- Proof label: `ci-mechanical-proof` plus `manual-required` for household two-device pairing, because this was a single-machine LAN-bind substitute

## Package, Autostart, Reboot, And Uninstall

- Installable artifact: not available in this worker checkout during the pass
- Reboot and autostart: not run
- Uninstall and data-retention behavior: not run
- Signing, store, notarization, TestFlight, device-owner, and entitlement proof: not run
- Proof label: `scaffold-only` for package-preview mechanics already covered by CI; `manual-required` for installed service/autostart/reboot/uninstall behavior on real artifacts

## Known Gaps And Risks

- Two-device LAN pairing, trusted-device registry, and authenticated remote control remain unproven and out of scope for V0.7.
- Managed browser exact URL/title proof still needs a real managed browser session with a known test URL.
- Fresh foreground-window, app/game duration, and screen evidence queue proof need controlled manual actions and permission-state capture.
- Package install, service autostart, reboot survival, uninstall cleanup, and production signing/store claims remain unproven without real preview or production artifacts.
- The local activity store contained older machine activity. This proof record deliberately avoids committing raw copied diagnostics or screenshots with sensitive details.

## Follow-Up

- The proof matrix does not need a claim upgrade from this pass. Keep privileged rows as `manual-required` until the controlled manual artifacts above are collected.
- A focused follow-up should use synthetic app/window/browser activity and a second LAN device before the coordinator promotes any manual-required row to implemented.

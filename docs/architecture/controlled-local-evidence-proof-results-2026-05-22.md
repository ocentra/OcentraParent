<!-- agent-capsule -->

> Agent Capsule
> Doc: Controlled Local Evidence Proof Results - 2026-05-22
> Kind: architecture/reference documentation; read only when selected by plan route, source router, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# Controlled Local Evidence Proof Results - 2026-05-22

## Run Metadata

- Run name: V0.7 controlled local evidence follow-up
- Commit: `c095a779ce7f6594611a4582b4df011a5a6b689c`
- Branch: `codex/v07-controlled-evidence-follow-up`
- Package/app version: `0.1.1`
- Mode: loopback-only controlled proof probes
- Agent port: `4577`
- Portal port context: `4578`
- Managed browser bridge probe port: `9577`
- Synthetic activity only: yes
- Sensitive details minimized: yes; this repo record does not include raw activity digests, raw process lists, private screenshots, browser history, or temp database files

## Controlled Probe Setup

The proof probes used temp-only local artifacts and the existing debug Rust service binary at `target/debug/ocentra-parent-agent-service.exe`. No product behavior, enforcement, blocking, model execution, capture hook, or source code change was added.

- Managed browser setup: Microsoft Edge from `C:/Program Files (x86)/Microsoft/Edge/Application/msedge.exe` launched with a temp non-default profile and loopback DevTools on `127.0.0.1:9577`.
- Controlled browser page: a temp local HTTP server served `http://127.0.0.1:50268/controlled-proof` with title `Ocentra Controlled Proof Page`.
- Foreground app setup: Notepad opened a temp file named `ocentra-controlled-foreground-proof.txt`.
- Agent setup: the Rust service used temp `OCENTRA_PARENT_ACTIVITY_DB_PATH`, `OCENTRA_PARENT_ACTIVITY_JOURNAL_PATH`, `OCENTRA_PARENT_ACTIVITY_JOURNAL_KEY_PATH`, and `OCENTRA_PARENT_DEV_LOG_DIR` values.
- WebSocket URL: `ws://127.0.0.1:4577/api/dev/ws`.

## Evidence Checks

| Check                                    | Observed result                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               | Proof label                                                            | Notes                                                                                                                                                                                                                                 |
| ---------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Controlled managed browser URL/title     | Node verified the live Edge DevTools target before service polling: `type=page`, `url=http://127.0.0.1:50268/controlled-proof`, `title=Ocentra Controlled Proof Page`. The existing service command `agent.browser.managed.bridge.poll` returned `agent.browser.managed.status.reported` with `managedState=bridge-disconnected`, `capabilityStatus=bridge-missing`, and `reason=browser-bridge-io-error`. The follow-up `agent.browser.evidence.recent.get` returned `returned=0`, `queryVisibility=unavailable`, and null URL/title fields. | `not-yet-proven` through the service command                           | The controlled browser itself exposed the target, but the current Rust bridge reader did not successfully ingest the live Edge DevTools endpoint. Do not upgrade the browser URL/title proof row from manual-required on this result. |
| Fresh foreground app/window              | After opening Notepad with the synthetic temp file and waiting for the startup sampler, `agent.activity.ingest.status.reported` returned `databaseReady=true`, `eventsStored=51`, and last event `activity-window-focused-available-...`. `agent.activity.recent.summary.get` returned `mostRecentKind=activity.window.focused`, `mostRecentObserver=windows-window`, `mostRecentSubjectKind=window`, and `mostRecentSubjectName=ocentra-controlled-foreground-proof.txt - Notepad`.                                                          | `implemented` for fresh foreground-window capture on this Windows host | This proves the existing Windows sampler and query store can capture a fresh low-sensitivity foreground window through the service read path.                                                                                         |
| Activity memory graph for foreground app | `agent.activity.memory-graph.get` returned `capabilityStatus=ready`, `returnedNodeCount=2`, and `returnedEdgeCount=1`. The nodes were the local device and a Notepad app/window node. The edge was `edgeKind=active-during` from the device to the Notepad node.                                                                                                                                                                                                                                                                              | `implemented` for foreground app graph visibility                      | The graph is derived from the same fresh window evidence and keeps custody as `child-device-activity-store`.                                                                                                                          |
| Timed app/game duration                  | The same memory graph returned the `active-during` edge with `durationMs=null` and `observedUntil=null`; no current WebSocket command exposes a fresh app/game session read model with measured running or foreground duration.                                                                                                                                                                                                                                                                                                               | `not-yet-proven`                                                       | This pass proves foreground presence, not timed duration. A later pass needs a real start/stop duration path or an exposed app/game read model before promoting this claim.                                                           |
| Screen evidence queue status             | The exposed WebSocket command contract includes activity, browser evidence, browser managed status, browser intervention, network flow, local AI runtime/chat, policy preview, health, log snapshot, and watch status commands. It does not expose a screen evidence queue or recent screen summary command.                                                                                                                                                                                                                                  | `not-yet-proven`                                                       | Rust protocol screen evidence queue types exist, but no existing service command was available in this pass to read live queue status.                                                                                                |
| Network read-model side observation      | `agent.network.flow.read-model.get` returned `capabilityStatus=available`, `adapterId=windows-network-snapshot-adapter`, `processAttributionStatus=process-attributed`, and `domainAttributionStatus=unavailable`.                                                                                                                                                                                                                                                                                                                            | informational                                                          | This was not the main follow-up scope, but it confirms the existing Windows network read model still reports process-attributed rows while keeping domain attribution honest.                                                         |
| Browser intervention side observation    | `agent.browser.intervention.read-model.get` returned `managedSessionInterventionCapability=needs-managed-session` and `unmanagedBrowserEnforcement=requires-os-app-control`.                                                                                                                                                                                                                                                                                                                                                                  | informational                                                          | No enforcement or intervention was attempted.                                                                                                                                                                                         |

## Validation

| Command                            | Result                                                                                                                                                                                                                                                                       |
| ---------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `cmd /c npm run format:check`      | Passed after formatting this Markdown artifact with Prettier.                                                                                                                                                                                                                |
| `cmd /c npm run test:pre-ai-proof` | Passed: `pre-ai-proof-ok: 11 claims checked across 5 platforms; 7 checkpoint scenarios checked.`                                                                                                                                                                             |
| `git diff --check`                 | Passed.                                                                                                                                                                                                                                                                      |
| `cmd /c npm run lanes:guard`       | Passed: `lane-guard-ok: lane=codex-a owner=sujan thread=codex-a branch=codex/v07-controlled-evidence-follow-up`.                                                                                                                                                             |
| `cmd /c npm run hub:guard`         | Passed: `hub-guard-ok: lane=codex-a`.                                                                                                                                                                                                                                        |
| `cmd /c npm run validate`          | Passed: version alignment, pre-AI proof, schema/source/test-double guards, Turbo lint/type-check/test, Rust checks/tests, local WebSocket smoke, LAN smoke, portal local smoke, and portal Playwright E2E completed. Source-shape warnings remain advisory and pre-existing. |

## Known Gaps And Risks

- The controlled Edge DevTools target was visible outside the service, but the current Rust bridge command returned `browser-bridge-io-error`; browser URL/title evidence remains not-yet-proven through the product service path.
- Fresh foreground-window capture is proven for a synthetic Notepad window on this Windows host, but timed app/game duration is not proven because the exposed graph edge had no duration.
- Screen evidence queue state remains unproven because no existing WebSocket command exposes it.
- Temp local proof directories, the activity SQLite database, the journal, browser profile, and Notepad file were deleted after each probe and were not committed.

## Follow-Up

- Investigate the managed-browser bridge reader against a live Edge/Chrome DevTools endpoint before promoting browser URL/title proof. The current command should either ingest the target or report a more actionable adapter error.
- Add or expose a real screen-evidence queue status read command before claiming screen queue proof.
- Add a real timed app/game session proof path before claiming foreground or running duration metrics.

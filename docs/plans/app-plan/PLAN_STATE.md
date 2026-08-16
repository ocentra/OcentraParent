<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-plan`
> Doc: `PLAN_STATE.md`
> Kind: plan state and current gap summary.
> Read when: After this plan is selected and before opening workpacks.
> Stop rule: Continue only through `NEXT_ACTIONS.md`, `WORKPACK_INDEX.md`, and the selected workpack.
> Proves: current routed plan state; code completion still requires the mapped source/tests and later validation.
> Proof rule: Keep this file aligned with `CODE_AUDIT.md`, `NEXT_ACTIONS.md`, `WORKPACK_INDEX.md`, and the engineering graph.

<!-- /agent-capsule -->

# Native Apps Plan State

## Current state

- Plan status: **Phase 1 audited; implementation incomplete**.
- Audit date: 2026-08-15.
- Authoritative code/test inventory: [CODE_AUDIT.md](CODE_AUDIT.md).
- Graph coverage: **95/95 reviewed workpack maps**.
- Bounded Phase 1: **80 complete, 15 incomplete**.
- Phase 2 focused tests/Enforcer: not run for this plan-wide audit.
- Phase 3 proof: not regenerated.

Code-pass note (2026-08-16): WP18/WP49 now have a Rust route-to-compiler draft,
and WP16 now derives unknown candidates from Rust inventory evidence. WP17 now
promotes explicit inventory risk categories and unknown-process rows into
advisory Rust candidates on `codex/app-plan-code-pass`. These slices are
unvalidated with tests/proof/checklist closure deferred; no product completion
claim is made. WP17 detection-to-route/compiler consumption is also drafted;
runtime service wiring and validation remain deferred. WP19 now has a Rust
category/risk compilation-to-time-budget handoff; restart/service scheduling
and validation remain deferred.
WP20 now has a Rust time-budget-to-child-notice bridge; live UI/service
delivery and validation remain deferred.
WP49 now consumes Rust risk detection into the service policy-readiness
surface as an explicit manual-required category/risk routing row. Compiler
service consumption remains deferred until a parent-authored rule/device/user
context is available; no enforcement claim is made.
WP15 now hydrates the existing portal app/game dashboard from the local
agent-service app-use and games read models through the parent runtime bridge;
the production slice is unvalidated and tests/proof/checklist closure remain
deferred.
WP48’s existing source-freshness dashboard ownership is reconciled to the
vendor portal implementation. WP63 now renders its existing typed source
panel sections beside capability and evidence panels; both remain unvalidated
with tests/proof/checklist deferred. WP64/WP65 now emit and consume typed
provider/preference status boundaries; WP66 joins them through the canonical
parent-surface builder and WP67 renders the joined rows with a legacy fallback.
All four remain unvalidated with tests/proof/checklist deferred.

Code-pass note (2026-08-16): WP59’s public scheduler persistence boundary now
revalidates the canonical scheduler bridge read model before writing atomic
records. Invalid source rows/counts/claims remain rejected; provider delivery,
retry workers, quiet-hours timers, child delivery, and UI remain unclaimed.

The 15 source/test-writing gaps are WP15, WP16, WP17, WP18, WP19, WP20,
WP26, WP48, WP49, WP62, WP63, WP64, WP65, and
WP102.

## Production reachability audit (2026-08-16)

This audit separates shipped runtime reachability from the Phase 1 source/test
inventory above. A contract, projection, checked row, proof panel, or focused
test counts only when a shipped entrypoint supplies trusted app/process input
and the code produces a material inventory, policy, event, or control effect.
The table is the current per-workpack runtime truth; historical `Complete`,
`checked`, and proof labels do not promote a live app-control claim.

| WP | Shipped caller and material effect | Production gap / boundary |
| --- | --- | --- |
| 01 | `child-runtime` routes generic app evidence/policy events through `app-core`; app-game protocol shapes feed agent read models. | `app-core::evaluate_app_runtime` has no shipped caller and no native app-policy source/control composition. |
| 02 | Documentation/source-index only. | No runtime entrypoint. |
| 03 | Snapshot/gap documentation only. | No runtime entrypoint. |
| 04 | Agent protocol identity shapes are consumed by inventory/read-model projection. | Identity enrichment is not an app policy/control decision. |
| 05 | `agent-core::activity_store_app_game` creates typed inventory rows used by service capture/read models. | Inventory model does not authorize policy or action. |
| 06 | `agent-service::activity_capture::spawn_startup_activity_capture` calls the Windows inventory source and persists journal/SQLite events. | Windows-only source; no non-Windows provider. |
| 07 | The same startup capture calls the Windows Store/AppX package source and persists package events. | Store lifecycle/approval interception is absent. |
| 08 | Startup capture calls the `sysinfo` process source and persists runtime rows. | Process presence is not policy classification or control authority. |
| 09 | Startup capture calls the Windows foreground-window source and persists redacted foreground rows. | Foreground evidence does not itself authorize an action. |
| 10 | Typed authority/capability rows feed service policy-readiness payloads. | No cross-platform native control provider is connected. |
| 11 | Taxonomy/category candidates are read by the app-game model and readiness payload. | No live policy-owned catalog/enrichment source. |
| 12 | `ActivityStore` sessionization/rollups feed the app-game service read model. | No policy schedule or allow-once consumer. |
| 13 | Startup capture persists app/process/foreground/launcher events through journal/SQLite ingest. | No approval/risk lifecycle is attached to ingest. |
| 14 | Agent service activity APIs/websocket reports expose the app-game read model. | Read-model/report effect only; no parent policy runtime. |
| 15 | Parent runtime loads app-use/games snapshots for `AppGameSessions`; portal has generated fields and proof panels. | No cohesive portal consumer for inventory/running/foreground/session/evidence/risk surfaces. |
| 16 | Approval types/reducer/persistence functions exist in `app-game-core`. | No production caller opens an unknown-app request, expiry, replay, or parent response. |
| 17 | Service policy-readiness derives category-risk counts by calling `detect_app_game_risk_candidate` on persisted inventory. | Detector output is a manual/readiness row only; no durable candidate or policy route. |
| 18 | Compiler is called only inside the app-game risk-routing/detection module. | No shipped caller supplies parent rule, device, user, schedule, and trusted adapter context. |
| 19 | Time-budget functions are library code only. | No runtime composes authoritative sessions with schedule/bonus/allow-once state. |
| 20 | Child-UX notice builders are library code only. | No child runtime delivery, persistence, or app warning/block request caller. |
| 21 | Websocket enforcement timer commands validate current ActivityStore app-session evidence, then `agent-core::enforcement_adapter` attempts Windows process termination by PID/name and journals the result. | The final adapter path does not verify executable path, process start time, or owner SID; identity hardening remains absent. This is narrow time-limit only, not broad app blocking or policy compilation. |
| 22 | Coordination/proof routing only. | No product source. |
| 23 | Generic child-runtime AI handoff exists; no app-specific classifier caller. | Digest/evidence boundary does not produce app control. |
| 24 | Coordination/platform routing only. | Unsupported platform actions remain manual-required. |
| 25 | Handoff/readiness shapes can describe install evidence. | No production package-manager interception or approval UX caller. |
| 26 | No product caller; performance scope only. | Required load/performance harness is deferred. |
| 27 | Manual-proof routing only. | No product source. |
| 28 | Rollout/PR coordination only. | No product source. |
| 29 | Protocol evidence/identity shapes are consumed by agent read-model paths. | Runtime enrichment and policy ownership remain absent. |
| 30 | Authority/classifier protocol rows are rendered by policy-readiness reports. | Rows are staged evidence, not live authority acquisition. |
| 31 | Classifier/action rows are journaled/projected with the app-game read model. | No live classifier producer feeds policy action. |
| 32 | Windows process source is reached by recurring startup capture. | No broad process-blocking policy caller. |
| 33 | Process events flow through service capture and SQLite projection. | No policy/risk transition consumes the rows durably. |
| 34 | Agent service capture invokes the real process source. | Capture effect only; no app control provider. |
| 35 | `service_runtime` starts recurring activity capture on supported Windows. | Freshness does not become policy enforcement. |
| 36 | Windows foreground source is reached by recurring capture. | No foreground-to-policy composition. |
| 37 | Service capture persists foreground events. | No action decision or child warning caller. |
| 38 | Service activity API exposes authority/classifier read-model rows. | Projection only; no live classifier execution owner. |
| 39 | Policy-readiness report computes classifier/authority counts from the model. | Counts cannot authorize control. |
| 40 | Boundary read-model event is exposed through the app-game websocket report. | Event/report effect only. |
| 41 | Windows inventory source is reached by recurring capture. | No cross-platform inventory authority. |
| 42 | Service capture persists Windows inventory rows. | No install/policy action caller. |
| 43 | Windows Store package source is reached by recurring capture. | No package lifecycle control. |
| 44 | Service capture persists Store package rows. | No approval interception. |
| 45 | Windows uninstall-registry source is reached by recurring capture. | Registry evidence does not prove install control. |
| 46 | Service capture persists registry inventory rows. | No durable approval/action owner. |
| 47 | Activity surface read models derive source-status/freshness rows from persisted app data. | Status is observational; no policy consumption. |
| 48 | Generated bridge types decode `sourceStatusRows`; route snapshot loads app-game data only on `AppGameSessions`. | Portal source-kind freshness/capability/evidence UI is not a live product surface. |
| 49 | Category/risk routing types and readiness rows exist; detector can produce manual-required routing metadata. | No service caller supplies authoritative rule/device/user context to the compiler. |
| 53 | Agent service builds notification-readiness rows from the app-game model. | No provider delivery or receipt. |
| 54 | Parent runtime/portal can render policy-readiness projections. | Renderer is status/projection only; no app action. |
| 56 | Agent service notification read-model report is reachable over websocket. | It explicitly keeps provider/outbox/scheduler delivery unclaimed. |
| 58 | Local-outbox bridge/store code is invoked by bounded service/readiness paths. | No live provider delivery or receipt authority. |
| 59 | Scheduler bridge validates/persists linked records for bounded readiness flows. | No quiet-hours/retry worker/provider execution. |
| 60 | Audit-history bridge preserves queued/manual/unavailable metadata. | No durable production delivery history consumer. |
| 61 | Provider-preflight bridge classifies persisted scheduler records. | No credentials/provider adapter or OS/app action caller. |
| 62 | Preference-preflight logic is consumed by notification readiness code. | No durable preference producer or delivery outcome source. |
| 63 | Generated source rows reach parent read-model snapshots. | Portal source panel polish/empty-stale-degraded rendering is absent. |
| 64 | Provider-status DTO/readiness rows are renderable through parent surfaces. | No producer derives status from provider attempts/receipts. |
| 65 | Preference-status DTO/readiness rows are renderable through parent surfaces. | No producer derives status from durable settings/delivery outcomes. |
| 66 | Canonical parent-surface builder joins typed readiness rows. | Upstream provider/preference producers are missing; no app control effect. |
| 67 | Portal parent-surface renderer consumes the joined snapshot. | Presentation only; delivery and app action remain unclaimed. |
| 74 | Source-freshness policy-consumption contract validates app read-model freshness/evidence. | No policy runtime or enforcement claim. |
| 75 | Rust source-freshness preview gate is reachable from preview projections. | Preview does not schedule or enforce. |
| 76 | Source-gated preview read model is rendered through parent policy surfaces. | Projection only. |
| 78 | Timer handoff projection records readiness requirements. | It does not start a timer. |
| 79 | Timer status projection classifies readiness. | It does not schedule a timer. |
| 81 | Timer runtime-readiness projection is produced. | No runtime execution caller. |
| 82 | Scheduler-persistence projection records missing requirements. | It does not persist/execute a scheduler. |
| 83 | Audit/rollback handoff projection records requirements. | No audit/rollback execution. |
| 84 | Audit/rollback read-model projection exists. | No durable audit runtime. |
| 85 | Parent-intent projection exists. | No parent action runtime. |
| 86 | Service-readiness handoff projection exists. | No service execution claim. |
| 87 | Service-readiness read model projection exists. | No live service producer beyond projection. |
| 88 | Protocol handoff projection exists. | No native app wire command/event. |
| 89 | Protocol read-model projection exists. | No service endpoint. |
| 90 | Command-handoff requirement projection exists. | No agent command caller. |
| 91 | Handler-handoff requirement projection exists. | No service handler. |
| 92 | Read-API requirement projection exists. | No endpoint. |
| 93 | Read-API response requirement projection exists. | No response runtime. |
| 94 | Response-consumer requirement projection exists. | No consumer runtime. |
| 95 | Parent-surface handoff projection exists. | No rendering/action claim. |
| 96 | Parent-surface read-model handoff projection exists. | No parent runtime consumer. |
| 97 | Parent-surface status handoff projection exists. | No service/portal status producer. |
| 98 | Parent-surface status read-model projection exists. | No service/portal runtime claim. |
| 99 | Status-read-model parent-surface handoff projection exists. | No parent-surface action. |
| 100 | Parent-surface read-model handoff projection exists. | No live service/portal consumer. |
| 101 | Terminal parent-surface read-model projection exists. | Pure model; not live composition. |
| 102 | No current production implementation matches the documented service handoff. | Must be implemented or explicitly retired/merged into WP103; no speculative wrapper. |
| 103 | Agent service timer read-model builder/report is reachable through activity APIs. | Status/read model only; no source-gated app policy scheduler caller. |
| 104 | Agent service emits the typed timer parent-surface event/report. | Event transport does not create app policy authority. |
| 105 | Timer read-API command/report routing is reachable. | No native-app compiler-to-timer composition. |
| 106 | Typed timer response encoding is reachable. | Response shape only. |
| 107 | Parent runtime consumes timer snapshots. | No live app policy source/action beyond the separate PID/name-validated Windows timer path. |
| 108 | Portal timer surface renders the parent snapshot. | UI/projection only; no app control authority. |

### Audit conclusion and stale topology

- The real shipped chain is `agent-service::service_runtime` -> recurring
  Windows inventory/process/foreground capture -> encrypted journal/SQLite ->
  `activity_api`/websocket -> parent snapshots. A separate websocket timer
  command can validate fresh app runtime evidence and attempt Windows process
  termination by PID/name. Final executable-path, process-start-time, and
  owner-identity checks are not carried through that adapter path. Neither
  chain supplies the missing native-app policy compiler, approval lifecycle,
  child UX, provider delivery, or broad control authority.
- `app_game_policy_target_compiler`, unknown approval persistence, time-budget
  composition, child-UX outbox, and notification/provider bridges are not
  connected to a trusted parent-authored app-policy entrypoint. Readiness and
  portal rows must remain manual/status projections.
- The graph reports mapped `code-and-tests` roots and Phase 1 validation states,
  but no workpack completion contract has reviewed implementation/test/proof/
  checklist evidence. The 80/15 Phase 1 split is bounded source/test topology,
  not live runtime completion.
- Historical package owners (`packages/activity-domain`,
  `packages/parent-domain`, `packages/agent-protocol-domain`, and
  `packages/text-domain`) and old app-game proof scripts are absent; current
  ownership is Rust-first through `agent-core`, `agent-service`, protocol, and
  generated bridge surfaces.

No legal source slice is unblocked by this audit. The next real dependency
slice remains WP18/WP49 only after a parent-authored rule/device/user context
producer and policy-runtime owner are identified; then WP16/WP17 approval/risk
durability, WP19/WP20 schedule/child delivery, and WP62-WP65 provider/preference
status can be composed without promoting DTO-only or proof-only seams.

## Current ownership

```text
app-core
  app-only observation/runtime decision boundary

app-game-core
  shared app/game projections and handoff models; many are pure models

agent-protocol
  shared wire DTOs and authority/readiness/adapter contracts

agent-core + agent-service
  real Windows acquisition, recurring capture, journal/SQLite projection,
  read models/events, and scoped PID/name-validated Windows time-limit execution;
  final executable-path, start-time, and owner-identity hardening is absent

schema + schema-domain
  Rust-generated cross-boundary TypeScript contracts

parent-runtime-core + portal
  parent-visible projections; never OS observation or enforcement authority
```

`packages/activity-domain`, `packages/parent-domain`,
`packages/agent-protocol-domain`, and `packages/text-domain` are not tracked
implementation owners in this checkout. Their paths in older workpacks are
migration history.

## Product truth

The Windows evidence spine is real through service capture and local
journal/SQLite read models. The product is not release-ready because durable
review/risk state, the native-app policy compiler/runtime composition, child
request UX, notification delivery/history, and complete parent
inventory/freshness surfaces are not written.

WP74-WP101 are mostly typed, tested no-claim projections. Their production
call-site inventory does not show live agent-service composition, so they must
not be promoted as runtime policy/timer support.

## Next implementation frontier

1. WP18/WP49 compiler and category/risk routing.
2. WP16/WP17 durable unknown/new-app review and live risk candidates.
3. WP19/WP20 time-budget runtime composition and child UX.
4. WP62-WP65 notification preference/status pipeline.
5. WP15/WP48/WP63 parent inventory/freshness UI.
6. WP26 load/performance harnesses.
7. WP102 implement or explicitly merge/retire into WP103.

## Completion boundary

Do not use this Phase 1 audit to mark the plan DONE. After code/test-writing
gaps close, run focused Phase 2 tests and Enforcer by touched risk surface.
Only then regenerate Phase 3 proof from a clean checkout and reconcile product
acceptance.

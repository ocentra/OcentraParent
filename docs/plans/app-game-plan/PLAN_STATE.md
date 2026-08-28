# App + Game Plan State

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `App + Game Plan State`
> Kind: current state and open gaps.
> Read when: Immediately after plan AGENTS.md; use for current state and no-claim boundaries.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.

<!-- /agent-capsule -->

Generated from the existing `app-game-plan` docs. This is the default resume/status file; large historical docs are linked, not embedded.

## Code-first audit baseline - 2026-08-15

- The executable graph imports **220** App + Game workpacks, not 214.
- All **220/220** now have reviewed current code/test ownership.
- **172** have current production source plus expected checked-in tests for their
  bounded scope; **20** are reviewed no-code coordination/proof/reference
  packets; **28** retain a concrete production-code or expected-test gap.
- The authoritative row-by-row source/test result is
  [CODE_AUDIT.md](CODE_AUDIT.md). It overrides historical checkbox, branch,
  removed-package, and ignored-proof wording for Phase 1 status.
- This audit changes ownership/status routing only. It does not claim Phase 2
  focused tests/Enforcer, Phase 3 proof, or release readiness.

## WP191/WP197 code-and-test-source checkpoint — 2026-08-28

Canonical `04783a5b7` contains WP191's complete typed fail-closed production
boundary and all three expected real test roots. Canonical `6eb1785c3` contains
WP197's reviewed Docker preflight source and all six expected real test roots.
None of these nine tests has been executed in the code-first phase. Live Linux
foreground-tool/process custody, active capture authority, proof, CI, READY,
and DONE remain open.

## WP197 source integration checkpoint — 2026-08-25

The Linux Docker host preflight source is integrated in the canonical tree
through `5bfb2f6f3` from `origin/codex/app-game-wp197-repair-round4-aug24` at
`23c08da016`. The current owner is Rust in `crates/agent-protocol` and
`crates/agent-service`, including the preflight, process/cleanup/path/output
helpers, service wiring, and websocket report/admission paths; the old
`packages/parent-domain` file list in the workpack is stale. The six expected
test roots listed below were subsequently added at canonical `6eb1785c3`, but
remain unexecuted:

- `crates/agent-service/tests/unit/app_game_linux_docker_host_preflight.rs`
- `crates/agent-service/tests/unit/app_game_linux_docker_host_preflight_parser_tests.rs`
- `crates/agent-service/tests/unit/app_game_linux_docker_host_preflight_path_security_tests.rs`
- `crates/agent-service/tests/unit/app_game_linux_docker_host_preflight_cleanup_tests.rs`
- `crates/agent-service/tests/unit/app_game_platform_probe_cache_tests.rs`
- `crates/agent-service/tests/unit/app_game_platform_proof_status_route_rejection_tests.rs`

Source integration is not test execution, retained proof, runtime readiness, or
workpack closure.

## Combined-plan production reachability audit - 2026-08-16

The source/test baseline above is not a live product-chain claim. A second,
production-only trace was performed from the shipped agent-service entrypoint
through parent composition and native execution. The index links 214 numeric
workpacks while the executable graph imports 220; that six-row topology
discrepancy remains stale and is not completion evidence.

| Workpacks | Reachable production caller and material effect | Remaining production gap / authority boundary |
| --- | --- | --- |
| WP01-WP16 | `service_runtime` starts recurring Windows capture; agent-core sources inventory, process, foreground, launcher, and app/game observations; encrypted journal/SQLite and activity-surface read models persist and expose evidence. WP10's accepted source shares one process snapshot and generation-safe process identity across launcher and foreground capture while retaining the generic window identity. Canonical `51d9819a9` adds the six mapped real test roots for launcher/candidate separation, generation identity, foreground joins, PID reuse, invalid ingest, and no-false-known-game behavior. The production portal route imports the tracked vendor surface and renders WP16's dashboard, source status, capability matrix, and evidence drawer. | WP10 test execution and retained proof are open; live launcher manifest/catalog crawling and the external publisher/classifier proof owner remain absent. WP16 lacks its focused intent/state/hostile-metadata test family, and its Game budgets tile remains `policy proof pending` because no budget row reaches the service read model. Identity merge, richer subscriptions, and non-Windows acquisition remain absent. Existing read models are evidence, not policy or control. |
| WP17 | Rust typed unknown-candidate producer and Eventing request/response/expiry/replay lifecycle are callable only inside app-game-core. | No service composition feeds a parent approval request into child delivery or an adapter. |
| WP18-WP20 | Rust game-budget, policy-target compiler, and time-budget evaluator compositions consume bounded stored summaries and remain dry-run. | No parent-authored live rule/context producer, service evaluator caller, scheduler/timer runtime, or enforcement handoff. |
| WP21-WP22 | Controlled child-UX tokens/outbox contracts exist; the timer command path validates current session evidence and can attempt Windows termination by PID/name. | No child delivery/UI/provider runtime. Timer authority is narrow: executable path, process start time, and owner-SID hardening are not carried into this PID/name path; the authenticated native termination resolver is a separate unused path. |
| WP23-WP28 | Proof, extension, install/store, performance, and rollout/reference packets have no shipped app/game control caller. | They remain proof/reference or missing-platform-owner work, not production implementation. |
| WP29-WP31 | Rust protocol parity and journal/SQLite storage persist staged authority/classifier rows. | No live classifier execution, policy consumer, dedicated service event, or portal authority/classifier surface. |
| WP32-WP48 | Existing service activity-surface composition carries staged evidence/identity/authority/classifier refs into app-use/games read models. The mounted WP48 portal dashboard consumes `sourceStatusRows` and renders source counts, freshness, capability, timestamp, and evidence summaries. | The refs are projection evidence, not trusted policy/compiler/evaluator authority. WP48's named focused intent test is absent, and the shallow route scaffold does not prove empty/stale/degraded semantics. |
| WP49, WP177 | `app_game_policy_readiness_sources` calls risk detection and the protocol → service → parent-runtime readiness row path is reachable. | This is readiness/status only. It does not invoke the compiler/evaluator with parent-authored policy or authorize an adapter. |
| WP51 | The evaluator is a Rust library consumed by app-game-core dry-run composition. | No external service/runtime caller supplies trusted policy context. |
| WP52-WP54, WP56 | Policy-readiness, notification intent, portal, and notification service read-model builders are reachable projections. | No authority, provider delivery, child delivery, receipt, or enforcement effect follows from these rows. |
| WP58-WP62, WP64-WP65 | Local outbox, scheduler, audit-history, provider/preference preflight, and status bridges validate and project persisted rows; missing or unsafe input remains manual-required/unavailable. | No production scheduler writer/runtime, provider credentials, preference mutation, delivery, receipt, or child runtime caller exists. |
| WP63, WP66-WP70 | WP63's split source-panel intent groups App use/Game freshness rows and the mounted SVG surface renders its metrics; the remaining parent-surface intent/renderer/read-model and policy-preview handoffs are callable projections. | WP63's focused grouping/hostile-metadata test is absent. These projections do not create policy authority, live notification delivery, timer execution, or adapter execution. |
| WP73-WP76 | Source-gated freshness/preview read models can consume the existing activity summaries. | No source-gated policy scheduler or evaluator execution consumes them. |
| WP78-WP108 | Timer handoff, persistence, rollback, readiness, service event/read API, and parent-surface chains are implemented as typed/status projections. | No parent-authored policy-to-scheduler runtime or durable live timer delivery is wired through the chain. |
| WP109-WP158 | Timer parent surfaces, child UX, preference/setup, local outbox, provider/preference and receipt handoff contracts are reachable where composed. | They stop at fail-closed/manual-required handoffs; child UI, provider runtime, service delivery, receipts, and native execution are absent. |
| WP159-WP165 | Safety gates, rejection/readiness, dashboard detail, and count panels expose conservative status. | Static/readiness projections do not prove policy execution or native custody. |
| WP166-WP176 | Portal action → parent-runtime → agent-service adapter-dispatch routing exists. Preflight can describe the scoped Windows timer; execute validates stored session evidence, while generic ProcessControl/TerminateProcess execution returns `ManualRequired`. | No trusted parent policy/compiler result reaches an authenticated native adapter. The scoped timer remains the separate PID/name path above; no broad app/game blocking or child-device adapter exists. |
| WP178-WP180 | Host capability/readiness rows are generated from local capability probes. | They are capability/status projections, not a native provider or enforcement authority. |
| WP181-WP222 | Android/Linux/physical-device, cross-platform preflight, child-delivery, and retained-proof packets have no shipped runtime caller in this tree. | Platform/provider ownership, delivery, receipt, rollback, and physical proof remain unimplemented or deferred. |

The one real shipped chain is therefore `service_runtime` → Windows
acquisition → encrypted journal/SQLite → agent-service read models →
parent-runtime snapshot. The chain stops before a trusted parent-authored
policy/compiler/schedule/approval producer, live child delivery, or
authenticated native adapter. There is no legal source correction in this
pass: adding another DTO, readiness row, proof bridge, or generic projection
would create an unreachable seam. The next real implementation dependencies
are, in order, compiler/evaluator context and service composition (WP19/WP51),
approval/risk service composition (WP17/WP49), time-budget scheduling (WP20),
child delivery/provider/receipt (WP21 and WP58-WP65), and only then authenticated
adapter execution (WP166-WP176). Tests, Enforcer validation, proof, and CI
remain deferred.

## Active production-code follow-up - 2026-08-16

- WP48 and WP63 are the same kind of false source gap as WP16. The tracked
  dashboard consumes `sourceStatusRows`; the split source-panel intent groups
  App use/Game rows; and the mounted SVG renders source freshness metrics. The
  workpacks remain open because their named focused intent test is absent and
  the shallow route scaffold does not prove empty/stale/degraded grouping or
  hostile metadata. No source-authority, policy, delivery, or adapter claim
  follows from the presentation path.

- WP16's cohesive production dashboard is present and reachable through
  `ParentPortalRoute.tsx` -> `vendor-parent-portal-surface.js` -> the tracked
  vendor `ParentPortalSvgSurface.tsx`. The surface renders service rows,
  source/freshness status, capability limitations, counts, and evidence. The
  existing Playwright route scaffold is shallow; the expected intent/state
  matrix and hostile/long-metadata tests are absent. Game budgets is explicitly
  `policy proof pending` because no budget row reaches the service read model.
  This removes the false "no cohesive surface" source gap without closing WP16.

- WP102 is an explicit no-code supersession decision, not another source gap.
  Its proposed `packages/parent-domain` intermediate would duplicate the real
  Rust protocol and agent-service parent-surface read-model boundary already
  owned by WP103. No production caller consumes the WP101 test-only builder, so
  generating a second dead builder would not advance runtime reachability.
  WP103 focused execution/proof and the missing live handoff from the bounded
  WP101 contract into product runtime remain open.

- WP101's retired `packages/parent-domain` roots were also a false code gap.
  The Rust-owned parent-safe read-model contract shapes and builder in
  `parent_surface_status.rs` and `tail.rs`, together with the real
  `app_game_source_gated_policy_preview_timer_followthrough` contract test,
  satisfy bounded Phase 1 source/test writing. Focused execution, proof,
  product runtime, and parent rendering remain open.

- WP100's retired `packages/parent-domain` roots were a false code gap. The
  Rust-owned shapes and builder in `parent_surface_status.rs` and `tail.rs`,
  together with the real `app_game_source_gated_policy_preview_timer_followthrough`
  contract test, satisfy bounded Phase 1 source/test writing. Focused execution,
  proof, product runtime, and parent rendering remain open.

- WP58 now has a fail-closed production boundary in
  `crates/app-game-core/src/app_game_notification_local_outbox_bridge.rs` and
  `crates/app-game-core/src/app_game_notification_local_outbox_bridge_read_model_validation.rs`:
  bridge construction and persistence reject malformed source rows, mismatched
  bridge/entry/alert identities, provider or scheduler claims, and non-local
  delivery state before writing the canonical local outbox.
- This is code drafted only. Tests, validation, Enforcer gates, retained proof,
  and broad completion are deferred; provider delivery, scheduler runtime,
  service composition, UI, child delivery, and adapter dispatch remain outside
  WP58.

- WP60 now has a consolidated Rust-owned fail-closed validator for the projected
  notification audit-history read model in
  `crates/app-game-core/src/app_game_notification_audit_history_bridge.rs`.
  This is code-drafted and unvalidated: tests, retained proof, durable
  production history/query, provider delivery/receipts, and runtime composition
  remain open. It does not claim WP60 DONE.

- WP64/WP65 now have an agent-service-owned composition boundary that loads and
  verifies a service-owned WP59 scheduler bridge plus its private scheduler
  proof store before invoking the Rust WP61/WP62 preflight builders. Only paired
  verified rows reach provider/preference status read models; absent, malformed,
  symlinked, or mismatched scheduler evidence yields explicit
  invalid/manual-required or unavailable status. This is a consumer-only seam;
  no production scheduler writer/runtime composition is claimed. Delivery,
  preference mutation, timer execution, UI, child delivery, enforcement,
  durability, tests, and proof remain open; these workpacks are code-drafted and
  not DONE.

## Scope

This folder is the shared native app and native game control plan. It exists because apps and games share the low-level evidence spine, but they do not share product meaning.

## Current ownership interpretation

```text
agent-protocol + agent-core:
  Canonical app/game contracts, Windows acquisition, journal/SQLite projection,
  sessionization, and parent-safe evidence/read-model boundaries.

app-game-core:
  Rust-owned source freshness, policy-preview, timer-handoff, notification-intent,
  and runtime-decision models. Generated TypeScript is an edge, not authority.

agent-service + parent-runtime-core + apps/portal:
  Service composition, parent bridge, and parent-visible projections/actions.
  Projection/readiness rows do not prove the missing runtime named by a no-claim flag.

platforms/android/agent:
  Tracked Android UsageEvents, Accessibility, delivery, receipt, and notification
  sources. Focused tracked App/Game Java tests are still missing where CODE_AUDIT says so.

packages/schema-domain:
  Generated validation/decoder edges only. The removed activity-domain,
  parent-domain, agent-protocol-domain, text-domain, and app-game-domain paths
  are not current implementation owners.
```

## Current coupling risks

```text
- Historical workpacks still name removed TypeScript owners and missing
  `scripts/test/app-game-*` runners. Use `CODE_AUDIT.md` and the engineering
  graph for current ownership; do not recreate those deleted packages.
- Generated handoff workpacks are not implementation scope by themselves. A selected workpack must identify the owner path and proof family before source edits.
- Portal rows, policy preview rows, notification rows, and child UX rows do not prove live app/game source readiness unless service/protocol/runtime proof exists.
- AI classifier digest rows prove only digest/result handoff unless they include stored app/game evidence refs and validated AI output. They do not prove AI runtime or OS scanning.
```

## Resume route

1. Read this file.
2. Read `NEXT_ACTIONS.md` when starting/resuming.
3. Read `WORKPACK_INDEX.md`.
4. Open only the assigned workpack.
5. Use `CHECKLIST_INDEX.md` for exact checklist sections.
6. Use `PROOF_INDEX.md` for proof artifacts.

## Current snapshot source

- Snapshot: [current-app-game-snapshot.md](current-app-game-snapshot.md)

## What is already present / proved

- Real Windows inventory, process, foreground, launcher, sessionization,
  encrypted journal/SQLite projection, recurring service capture, and bounded
  read-model tests are present.
- Scoped owned-process time-limit dispatch, parent preference request
  persistence/outbox, adapter readiness/dispatch, platform status, receipt,
  parent-runtime, and portal surfaces are present with bounded tests.
- These do not erase the 33 Phase 1 gaps or claim cross-platform/provider/child
  delivery and physical proof.

## Historical gap narrative

The detailed bullets below preserve older packet history. They are not current
Phase 1 authority where they conflict with `CODE_AUDIT.md`.

- App/game identity contracts are present, but runtime identity merge behavior and adapter-fed identity refs are not implemented yet.
- Inventory evidence row contracts and Rust inventory-row parity are present, Windows installed-record plus Store/UWP package parser proof exists, staged journal/SQLite replay proof now projects inventory rows, and service activity-surface read models now expose typed inventory state, but live platform crawling and dedicated portal dashboard rows are not implemented yet.
- Runtime evidence contracts and Rust runtime-row parity are present, a staged Windows process runtime parser proof exists, staged journal/SQLite replay proof now projects running-now rows, and service activity-surface read models now expose typed runtime state. Bounded live process capture now refreshes that same service path, but executable metadata crawling, richer subscriptions, and dedicated portal runtime rows are not implemented yet.
- Foreground evidence contracts and Rust foreground-row parity are present, a staged Windows foreground-window parser proof exists, core live active-window source proof now emits foreground rows and journal events with opaque window/title refs, journal/SQLite replay now projects foreground-now rows, the bounded service capture bridge can append optional foreground rows, and service activity-surface read models expose typed foreground state. Dedicated portal foreground rows, subscribed foreground transitions, and content-aware claims are not implemented.
- Rust protocol parity now mirrors the WP01 evidence claim, AI digest reference/classification digest, WP04 identity/identity-merge shapes, the app/game control authority/action-result schemas, the platform authority matrix, and the WP24 parent-domain classifier boundary. WP31 adds staged journal/SQLite storage and read-model projection for evidence claim, identity, authority, action-result, platform authority matrix, and classifier result rows. WP38 carries those staged row refs through existing app-use/games service read-model evidence vectors. Live source subscriptions, classifier execution, dedicated classifier service events, policy runtime, portal authority/classifier rows, and adapter execution remain incomplete.
- Journal and SQLite ingest now covers staged app/game inventory, runtime, foreground, launcher, daily rollup, evidence-claim, identity, approval authority, approval action-result, platform authority matrix, and AI classifier result rows. The service still maps only the established app-use and games activity-surface rows, but those rows now retain staged authority/classifier storage refs in their evidence vectors. The new authority/classifier rows are not yet wired to live source subscriptions, dedicated service events, policy consumers, or portal dashboard rows.
- Portal App/Game Sessions dashboard rows now consume the app-use and games activity-surface DTOs through a shared dashboard intent, but approval, policy, game-budget, live source, and platform-authority surfaces remain incomplete.
- Unknown approval flow now has a Rust-owned typed candidate producer and durable synchronized Eventing lifecycle for request, parent response, expiry, restart replay, idempotency, and manual-required unsupported blocks. Service composition, notification delivery, service read models, parent/child UI, and adapter execution remain incomplete.
- Category/unknown policy-readiness consumption is implemented across Rust protocol, the live service read-model projection, parent-runtime panel intent, and the portal route. It exposes candidate/review rows and counts without inferring adapter dispatch. This closes WP177's readiness-only Phase 1/2 scope; live compiler/evaluator consumption and retained proof remain separate open plan gaps.
- Native game budget policy now has a Rust-owned `ocentra-app-game-core` composition over the WP19 compiler and WP51 evaluator. It counts known games and parent-approved launcher-game candidates, excludes launcher-only/unapproved candidates, rejects incoherent/duplicate/bypass inputs, preserves advisory rating/UGC/multiplayer/purchase signals, and stays dry-run with no adapter dispatch. Service persistence, portal authoring/preview UI, bonus-time integration, notifications, retained proof, and adapter execution remain open.
- App/game policy target compiler now has parent-domain contract proof for app/game targets, identity/unknown/category/schedule/capability/authority proof, device/local-user/freshness rejection, dry-run-only decisions, and manual-required unproved block-launch. It does not yet provide Rust/service parity, runtime evaluator execution, portal rule authoring/preview UI, timer integration, notifications, rollback, or adapter execution.
- App/game time-budget policy now has a Rust-owned `ocentra-app-game-core` composition from stored session summaries into the WP51 evaluator. It preserves daily/weekly scope, running versus foreground duration, bound schedule evidence, pending/approved bonus audit refs, effective budget math, and active/recovered timer refs while remaining dry-run. Service persistence, portal/child UX, notification delivery, timer scheduling/rollback, retained proof, and adapter execution remain open.
- Child-facing app/game UX now has a Rust-owned controlled-token contract in `ocentra-app-game-core` for respectful limited, approval-needed, time-warning, request submitted/approved/denied, manual-required, and unavailable states. It requires evidence plus child reason/status refs for ask-parent actions, accepts no arbitrary copy/private diagnostics, and never claims adapter dispatch. That WP21 slice alone does not provide live child UI, native overlay rendering, portal preview screenshots, notification delivery, service persistence, adapter execution, or platform shield/block behavior.
- The child UX local-outbox bridge now validates those controlled-token notices against timer child-UX artifact refs, converts deliverable states to the canonical notification-outbox record, blocks manual/unavailable states, and persists atomically with restart/idempotency/conflict tests. Service composition, scheduler/provider/receipt delivery, child rendering, and parent notification UI remain open.
- The child UX scheduler bridge now maps reopened honest local-outbox rows into the shared Rust `due-local` record, preserves source/scheduler/evidence refs, blocks non-queued states, fails closed on unsafe claims, and persists conservative proof rows atomically with restart/idempotency/conflict tests. Production scheduling, retry/quiet-hours execution, provider/preference preflight, receipts, service composition, and UI remain open.
- The child UX provider preflight now accepts only persisted identity-bound `due-local` scheduler rows, preserves scheduler/outbox/evidence/policy/audit refs, generates distinct adapter/credential/smoke-proof requirements, and keeps manual/dead-letter or unsafe rows blocked. Provider execution, credential custody, receipts, retry/quiet-hours runtime, cloud routing, service composition, and UI remain open.
- The child UX provider-status handoff now maps those honest preflight rows into canonical identity-bound V0.8 manual-required or unavailable entries, preserves readiness/manual-proof/preference/audit refs, and rejects claimed or incomplete inputs. It does not claim provider execution, receipts, credentials, retry/quiet-hours runtime, cloud routing, parent UI, or child delivery.
- The child UX preference preflight now maps persisted due-local scheduler rows to explicit parent preference, notification frequency, and quiet-hours requirements, keeps manual/dead-letter rows blocked, and rejects unpersisted, mismatched, claimed, or duplicate requirements. Preference mutation, quiet-hours execution, delivery, receipts, parent UI, and child delivery remain open.
- The child UX preference-status handoff now maps those preflight rows into manual setup/manual-required or channel-disabled/not-sent App/Game status rows, preserves safe scheduler/provider/preference/quiet-hours/rule/retry/evidence/audit refs, and rejects unsafe or malformed inputs. Preference mutation, runtime delivery, receipts, parent UI, and child delivery remain open.

## Current proof interpretation

```text
Staged journal/SQLite proof is not live source subscription proof.
Service read-model refs are not dedicated portal rows unless the portal proof exists.
AI classifier digest proof is not AI runtime execution and does not prove AI scanned the machine.
Policy dry-run proof is not enforcement proof.
Manual-required block-launch proof is not adapter execution.
Platform preflight proof is not platform parity.
Portal dashboard proof is not source capture, timer, or adapter proof.
Notification handoff proof is not delivery readiness unless provider/outbox/scheduler proof exists.
Checked generated handoff rows do not override the selected workpack proof root and E2E tier.
```

## Manual-required or no-claim boundaries

- App/game session contracts and read-model proof exist.
- App/game evidence claim, AI classification digest, and parent app/game control authority schemas now exist as TypeScript contract proof.
- App/game layered identity and identity-merge schemas now exist as TypeScript contract proof.
- Rust protocol parity now mirrors the app/game evidence claim, AI digest reference, AI classification digest, layered identity, and identity-merge shapes from `packages/activity-domain` with serialization proof.
- Rust protocol parity now also mirrors the parent-domain app/game approval authority/action-result, platform authority matrix, and AI classifier result boundary shapes with serialization proof and no live adapter claim.
- App/game journal/SQLite ingest now stores and projects the newly mirrored evidence claim, identity, approval authority, approval action-result, platform authority matrix, and AI classifier result protocol rows through staged encrypted-journal replay with no-use, manual-required, and AI-cannot-enforce rejection guards.
- App/game service read models now preserve refs for those staged evidence-claim, identity, approval authority/action-result, platform authority matrix, and AI classifier result rows in the existing app-use/games evidence vectors, without adding live classifier execution, policy consumption, dedicated portal rows, or adapter execution.
- App/game inventory evidence rows now exist as TypeScript contract proof with source, custody, category candidates, stale/permission-limited states, and no-use guards.

## Checklist summary

- Full checklist: [implementation-checklist.md](implementation-checklist.md) (not default context).
- Checkbox rows detected: 229 total, 211 checked, 18 unchecked.
- Checklist index: [CHECKLIST_INDEX.md](CHECKLIST_INDEX.md).

## Workpack summary

- Workpacks indexed: 220.
- Workpacks with open checkboxes: 0.
- Workpacks with all detected boxes checked: 54.
- Workpacks with no checkbox status: 160.

### Active/open workpacks

- The six app/game capability, schema, and settings guides are reviewed
  no-code reference/control-routing packets. They do not claim product
  implementation or proof completion.
- Thirty-three implementation/test-writing gaps remain; select them through
  `CODE_AUDIT.md` and `WORKPACK_INDEX.md`.

## Default no-read list

- `README_FULL_ORIGINAL.md` unless you need historical full README context.
- Full `implementation-checklist.md` unless `CHECKLIST_INDEX.md` names exact section/row.
- All workpacks; use `WORKPACK_INDEX.md`.
- `WORKPACK_FAMILIES.md` unless the selected workpack owner/proof family is unclear.
- Source inventories and pasted-content audits unless source ownership is unclear.
- Historical checkpoint/proof docs unless `PROOF_INDEX.md` or the assigned workpack names them.

## Health / consistency

- See `PLAN_HEALTH.md` before claiming the whole plan is complete or stale.
- Use the E2E tiers in `TEST_PROOF_EXPECTATIONS.md` before any feature-complete or PR_READY claim.
- Use `WORKPACK_FAMILIES.md` only to classify the selected workpack; do not use it as permission to scan a whole family.

## HID Execution Guard (added 2026-06-12)

- Scope and completion source:
  - follow [PLAN_HID_MATRIX.md](../../PLAN_HID_MATRIX.md) execution slice, then this plan's assigned WORKPACK_INDEX.md and NEXT_ACTIONS.md.
  - do not mark this plan complete from checklist deltas alone.
- Before any checked update, attach:
  - a real test run log (or explicit known blocker) from the assigned implementation boundary,
  - a proof manifest under docs/proof/app-game-plan/.
- Required proof manifest names:
  - docs/proof/app-game-plan/slice-01-*.md
  - docs/proof/app-game-plan/slice-02-*.md
  - docs/proof/app-game-plan/slice-03-*.md
  - each proof file must include commands, pass/fail,
    negative-cases, and manual-required notes.
- Failure rule: no PR-ready claim until replay/idempotency, authZ/replay, and rollback/teardown proofs are present for the assigned slice.

## HID execution blueprint

Continue execution from: [PLAN_EXECUTION_BLUEPRINT.md](PLAN_EXECUTION_BLUEPRINT.md).
Update this plan only via the blueprint and matching workpack checklist.

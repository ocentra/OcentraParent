# Browser Source Index

<!-- agent-capsule -->

> Agent Capsule
> Plan: `browser-plan`
> Doc: `Browser Source Index`
> Kind: source ownership index; read only when source ownership is unclear.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not inspect broad source from here; use only the named package/crate path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.

<!-- /agent-capsule -->

This index keeps the browser plan tied to existing source documents and code. It
is not a replacement for feature, expectation, roadmap, checklist, package, or
crate ownership docs.

## 2026-08-16 production reachability audit

The production-code pass found no additional reachable Browser slice with a
real owner-backed input. The following boundaries are retained as blockers,
not completion claims:

- WP11 reaches `crates/agent-core/src/browser_bridge_poll.rs` from
  `crates/agent-service/src/browser_runtime_impl/bridge.rs`, but
  `browser_bridge_poll/parse.rs` intentionally emits `unknown` /
  `target-list-only`. Focus/activation, extension, foreground-correlation, and
  owned-shell sources are absent.
- WP21's `crates/agent-core/src/browser_bridge_native_host.rs` is a validator
  with test-only inbound references. No extension package, native-host
  registration, or production IPC caller exists.
- WP20's AppLocker/App Control model is reachable only as a service
  enforcement product-control read model whose states remain static and
  manual-required. Windows policy/runtime authority is absent.
- WP22's `crates/browser-core/src/performance_budget.rs` is fixture/evaluator
  code with no runtime measurement producer or service-health bridge caller.

The graph report currently presents all 30 Browser workpacks as `planned` with
no dependencies; it is stale relative to the source-backed implementation and
blocker notes. Legacy `packages/activity-domain/src/browser*.ts` references are
not active ownership; current TypeScript edge ownership is
`packages/browser-domain`, with Rust runtime ownership under the paths below.

## Product Source Docs

- Owning feature: [Browser and web control](../../features/browser-web-control.md)
- Main expectation: [Browser URL and tab evidence](../../expectations/browser-evidence.md)
- Milestone expectation: [V0.5.1 browser URL and tab evidence capture](../../roadmaps/roadmap-v0-5-1-browser-url-tab-evidence-capture.md)
- Main architecture: [Browser URL and tab evidence capture](../../architecture/browser-url-tab-evidence-capture.md)
- Managed/unmanaged guide: [Managed and unmanaged browser capability guide](../../plans/browser-plan/workpacks/managed-unmanaged-browser.md)
- Parent policy catalog: [Browser policy settings catalog](../../plans/browser-plan/workpacks/browser-policy-settings-catalog.md)
- Schema proposal: [Browser control schema proposal](../../plans/browser-plan/workpacks/browser-control-schema-proposal.md)
- Coverage matrix: [Browser control coverage matrix](../../plans/browser-plan/workpacks/browser-control-coverage-matrix.md)
- Raw catalog: [Browser control 1057 settings inventory](../../plans/browser-plan/workpacks/browser-control-1057-settings-inventory.md)
- Questionnaire forest: [Browser policy questionnaire forest v1](../../plans/browser-plan/workpacks/browser-policy-questionnaire-forest-v1.md)
- Browser intelligence plan:
  [V0.5 browser URL and video AI intelligence](v0-5-browser-url-video-ai-intelligence-plan.md)
- Social/platform gating plan:
  [V0.5 social platform account feed and gating](v0-5-social-platform-account-feed-gating-plan.md)
- Browser-game gating plan:
  [V0.5 browser games cloud gaming and game portal gating](v0-5-browser-games-cloud-gaming-gating-plan.md)
- Implementation tracking:
  [Browser plan implementation checklist](implementation-checklist.md)
- Pasted-content reconciliation:
  [Pasted content coverage audit](pasted-content-coverage-audit.md)
- Adjacent expectation: [AI](../../expectations/ai.md)
- Adjacent expectation: [Social/video control](../../expectations/social-video-control.md)
- Adjacent expectation: [App/game evidence](../../expectations/app-game-evidence.md)

## Routing: Move Here Or Point Here

Browser implementation planning belongs in this folder when it is about:

- browser inventory;
- managed profile/session launch;
- browser bridge custody;
- exact URL/title/tab evidence;
- active-tab certainty;
- unmanaged browser fallback;
- browser policy authoring and compile;
- URL/page/video intelligence planning that starts from browser evidence;
- managed-browser social account creation, feed, short-video, route, and
  approval-gate planning;
- managed-browser game portal, WebGL/canvas, cloud-gaming, unblocked-game,
  game account, purchase, and game approval planning;
- managed warn/block/intervention;
- browser proof gates and UI/UX acceptance.

Shared source docs stay where they are and are pointed to from this folder:

- feature docs stay under `docs/features`;
- expectation docs stay under `docs/expectations`;
- architecture docs stay under `docs/architecture`;
- product checklist and roadmap stay at the docs root;
- package/crate/app ownership docs stay next to their source.

Do not move those shared docs into this plan folder. Link them here and update
them only when the implementation status, acceptance contract, or proof changes.

## Feature Routing

| Feature doc                          | Browser-plan relationship                                                                                                                                                                                            |
| ------------------------------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `browser-web-control.md`             | Owning feature. Browser plan work derives from this file and feeds status/proof updates back to it.                                                                                                                  |
| `app-game-control.md`                | Adjacent app/game feature. Browser games start from managed browser evidence; native games, launchers, process/session duration, and broad app blocking stay here.                                                   |
| `enforcement-integrity-tamper.md`    | Shared enforcement boundary. Browser intervention and unmanaged fallback must remain capability-gated here.                                                                                                          |
| `evidence-store-query.md`            | Shared storage boundary. Browser evidence must journal/replay through shared evidence stores.                                                                                                                        |
| `network-domain-control.md`          | Adjacent weaker evidence source. Network/domain can support domain metadata but not exact tab/page claims.                                                                                                           |
| `policy-schedules-approvals.md`      | Shared policy UX/evaluator boundary. Browser rules compile through parent-domain contracts, not browser UI shortcuts.                                                                                                |
| `screen-evidence-analysis.md`        | Adjacent fallback evidence source. Screen summaries may support later context, not browser URL proof.                                                                                                                |
| `social-video-control.md`            | Adjacent product feature. Browser URL/video intelligence and managed-browser social account/feed gates can feed social/video policy only with evidence, confidence, model/runtime, parent approval, and audit proof. |
| `local-ai-safety-evaluator.md`       | Adjacent AI feature. Browser intelligence uses local AI as evidence and must not let model output directly enforce.                                                                                                  |
| `policy-schedules-approvals.md`      | Shared approval/evaluator feature. Social account creation, secondary-account, and parent-review flows must use typed approval contracts, not browser-only shortcuts.                                                |
| `remote-lan-mobile-platforms.md`     | Platform routing. Android/iOS/browser mobile states remain platform-specific/manual-required here.                                                                                                                   |
| `production-distribution-support.md` | Release/support boundary. Support bundles must redact raw URLs, browser secrets, journals, SQLite, private paths, and screenshots as configured.                                                                     |

## Adjacent Plan Docs

- LAN plan: [LAN plan README](../lan-plan/README.md)
- V0.8 enforcement workpack for managed browser session control:
  [Managed browser session control](../v0-8-enforcement-control-plan/workpacks/06-managed-browser-session-control.md)
- V0.8 enforcement workpack for unmanaged browser fallback:
  [Unmanaged browser fallback](../v0-8-enforcement-control-plan/workpacks/07-unmanaged-browser-fallback.md)
- Portal UX workpack:
  [Browser, app, and network surfaces](../portal-ux-household-surfaces-plan/workpacks/09-browser-app-and-network-surfaces.md)

## TypeScript Ownership

- `packages/browser-domain/src/browser-*.ts`
- `packages/browser-domain/src/browser-ai-*.ts`
- `packages/browser-domain/src/browser-social-*.ts`
- `packages/browser-domain/src/browser-game-*.ts`
- `packages/browser-domain/tests/unit/browser*.test.ts`
- `packages/browser-domain/tests/unit/social*.test.ts`
- `packages/browser-domain/tests/unit/browser-game*.test.ts`
- `packages/parent-domain/src/browser-control-policy.ts`
- `packages/parent-domain/src/browser-control-manifest.ts`
- `packages/parent-domain/src/browser-control-values.ts`
- `packages/parent-domain/src/browser-control-catalog-values.ts`
- `packages/parent-domain/src/browser-control-full-catalog*.ts`
- `packages/parent-domain/src/browser-policy-questionnaire-forest*.ts`
- `packages/agent-protocol-domain/src/browser-policy-adapter.ts`
- `packages/agent-protocol-domain/src/browser-runtime-events.ts`
- `packages/agent-protocol-domain/src/social-*.ts`

TypeScript rule: the primary TypeScript ownership boundary for browser-plan
implementation in this checkout is `packages/browser-domain`. The older
`packages/activity-domain/src/browser*.ts` paths referenced in previous plan
text do not exist here and should not be treated as active ownership. Keep
browser evidence, browser AI, browser-social, and browser-game contract work
inside `browser-domain` unless an ownership boundary genuinely changes. Keep
`parent-domain` focused on policy/catalog/manifest shapes and
`agent-protocol-domain` focused on typed bridge/read-model crossings.

## Rust Ownership

- `crates/agent-protocol/src/browser.rs`
- `crates/agent-protocol/src/browser_managed.rs`
- `crates/agent-protocol/src/browser_read_model.rs`
- `crates/agent-protocol/src/browser_intervention.rs`
- `crates/agent-protocol/src/browser_policy*.rs`
- `crates/agent-protocol/src/local_ai*.rs`
- `crates/agent-protocol/src/activity_memory_graph.rs`
- `crates/agent-protocol/src/app_game*.rs`
- `crates/agent-protocol/src/constants/browser.rs`
- `crates/agent-core/src/browser_managed_discovery.rs`
- `crates/agent-core/src/browser_managed_session.rs`
- `crates/agent-core/src/browser_bridge_*.rs`
- `crates/agent-core/src/activity_store_browser*.rs`
- `crates/agent-core/src/activity_store_app_game*.rs`
- `crates/agent-core/src/activity_store_memory_graph*.rs`
- `crates/agent-core/src/browser_intervention_event*.rs`
- `crates/agent-service/src/browser_runtime*.rs`
- `crates/agent-service/src/browser_policy*.rs`
- `crates/agent-service/src/local_ai*.rs`
- `crates/agent-service/src/activity_memory_graph_payload*.rs`
- `crates/agent-service/src/browser_payload.rs`
- `crates/agent-service/src/browser_evidence_payload.rs`
- `crates/agent-service/src/activity_api/browser_intervention_*.rs`

Rust rule: TypeScript contracts come first, Rust protocol parity second,
`agent-core` runtime helpers third, and `agent-service` command/read-model
wiring fourth.

## Portal Ownership

- `apps/portal/src/browser-status-panel.ts`
- `apps/portal/src/browser-intervention-panel.ts`
- `apps/portal/src/browser-intervention-read-model.ts`
- `apps/portal/src/portal-browser-route-panels.ts`
- `apps/portal/tests/live-activity-browser-status.test.ts`
- `vendor/ocentra-parent-core-ui/AppPages/ParentPortal/BrowserRulesQuestionnaire.tsx`
- `vendor/ocentra-parent-core-ui/AppPages/ParentPortal/BrowserRulesQuestionnaire.css`

Portal rule: render service-backed state and typed manifests. Portal must not
connect to browser bridges, read browser profiles, read SQLite/journals
directly, or invent policy questions outside the manifest.

## Proof Scripts

- `npm run test:managed-browser-matrix`
- `npm run test:managed-browser-service-proof`
- `npm run test:managed-browser-intervention`
- `node scripts/test/browser-plan-closure-audit-proof.mjs`
- `node scripts/test/v0-8-browser-domain-adapter-proof.mjs`
- `node scripts/test/windows-managed-unmanaged-browser-enforcement-proof.mjs`
- `node scripts/test/browser-performance-health-proof.mjs`
- `node scripts/test/browser-plan-e2e-manual-proof-artifacts.mjs`

## Current Test Files

- `packages/browser-domain/tests/unit/browser*.test.ts`
- `packages/browser-domain/tests/unit/social*.test.ts`
- `packages/browser-domain/tests/unit/browser-game*.test.ts`
- `packages/agent-protocol-domain/tests/unit/browser*.test.ts`
- `packages/agent-protocol-domain/tests/unit/social*.test.ts`
- `crates/agent-protocol/src/browser*_tests.rs`
- `crates/agent-core/src/browser*_tests.rs`
- `crates/agent-service/src/browser*_tests.rs`
- `apps/portal/tests/live-activity-browser-status.test.ts`
- `apps/portal/tests/social-*.test.ts`
- `apps/portal/e2e/browser-ai-parent-explanation-ui-proof.spec.ts`
- `apps/portal/e2e/social-*-ui-proof.spec.ts`
- `scripts/test/browser*.mjs`
- `scripts/test/social*.mjs`
- `scripts/test/browser-game*.mjs`

## Source Truth Rule

When a browser workpack changes product state, update the owning feature doc,
matching expectation docs, product capability checklist row, and touched module
README. If the work only adds planning detail inside this folder, no product
status update is required.

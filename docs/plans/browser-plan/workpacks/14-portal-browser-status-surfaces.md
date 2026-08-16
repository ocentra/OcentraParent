# 14 Portal Browser Status Surfaces

<!-- agent-capsule -->

> Agent Capsule
> Plan: `browser-plan`
> Doc: `14 Portal Browser Status Surfaces`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Where We Are

Portal renders managed browser status, evidence summary, and intervention
summary. The richer parent browser workflow is not complete.

2026-08-16 browser-code-pass: the parent-runtime Browser route now consumes
typed agent-service managed-status and intervention read models through the
existing Rust-owned live-activity bridge. The service keeps the canonical
protocol models and emits dedicated JSON transport fields for the bridge
adapter; unrelated routes are not hydrated. This is code-drafted,
unvalidated, and tests/proof/checklist-deferred. It does not add active-tab
focus authority, unmanaged exact-URL authority, OS blocking, or action
delivery.

2026-06-02 codex-d completion: the browser route now exposes service-backed
browser inventory, exact URL capability, and active-tab proof cards from the
typed `agent.browser.inventory.read-model.reported` event. Portal live-activity
state reconstructs the activity-domain browser inventory read model from flat
service payload fields, and the product shell service-state rows keep unmanaged
fallback as report-only/not-claimed instead of exact URL evidence. The proof pack
includes focused portal-domain/portal tests, full portal Playwright E2E, and a
headless Playwright screenshot. Social/video/game read models, child-facing states,
intervention actions, final visual polish, and real OS/browser platform proof
remain later workpacks.

## Where We Want To Be

Parents can inspect installed browsers, managed sessions, tab evidence,
unmanaged bypass, stale/degraded states, policy preview, and interventions from
service-backed read models.

## Scope

- Inventory dashboard.
- Managed session card.
- Tab evidence list.
- Active-state certainty copy.
- Unmanaged bypass cards.
- Intervention/action results.
- URL/video intelligence explanation states when local AI proof exists.
- Social platform overview, approved accounts, pending approvals, secondary
  account attempts, feed/short-video route states, and messaging-route states
  when social read models exist.
- Browser-game dashboard, cloud-gaming approval, educational-game allow,
  game-account/purchase approval, unblocked-site, and unmanaged game bypass
  states when browser-game read models exist.
- Redacted copy/debug details.
- Empty, stale, degraded, manual-required, unsupported states.

## Touched Paths

- `apps/portal/src/portal-browser-route-panels.ts`
- `apps/portal/src/live-activity-state.ts`
- `apps/portal/src/portal-route-content.ts`
- `apps/portal/e2e/portal-route-scaffold-assertions.ts`
- `apps/portal/tests/live-activity-state.test.ts`
- `apps/portal/tests/live-activity-browser-status.test.ts`
- `packages/portal-domain/src/commands.ts`
- `packages/portal-domain/src/contracts.ts`
- `packages/portal-domain/src/details.ts`
- `packages/portal-domain/src/parent-portal-product-shell-row-signals.ts`
- `packages/portal-domain/src/parent-portal-product-shell-row-specs.ts`
- `packages/portal-domain/src/parent-portal-service-state-constants.ts`
- `packages/portal-domain/src/parent-portal-service-state-rows.ts`
- `packages/portal-domain/tests/contracts.test.ts`
- `packages/portal-domain/tests/parent-portal-service-state.test.ts`
- `crates/agent-protocol/src/constants/field.rs`
- `crates/agent-service/src/browser_payload.rs`
- `crates/agent-service/src/activity_api/browser_intervention_payload.rs`
- `crates/parent-runtime-core/src/agent_service_client/snapshots_browser.rs`
- `crates/parent-runtime-core/src/agent_service_client/loaders.rs`
- `crates/parent-runtime-core/src/agent_service_client/types.rs`
- `crates/parent-runtime-core/src/parent_ui_bridge/route_requirements.rs`
- `crates/parent-runtime-core/src/parent_ui_bridge/route_snapshot/dependencies.rs`
- `crates/parent-runtime-core/src/parent_ui_bridge/route_snapshot/dependencies/load.rs`
- `crates/parent-runtime-core/src/parent_ui_bridge/live_activity/snapshot.rs`
- `crates/parent-runtime-core/src/parent_ui_bridge/live_activity/snapshot/browser.rs`
- `crates/parent-runtime-core/src/parent_ui_bridge.rs`

## Tests And Proof

- Mocked-backend Playwright fixtures first.
- Real service Playwright proof when service paths exist.
- `cmd /c npm run test --workspace @ocentra-parent/portal -- live-activity-state.test.ts`
- `cmd /c npm run type-check --workspace @ocentra-parent/portal`
- `cmd /c npm run test --workspace @ocentra-parent/portal-domain`
- Malicious URL/title layout and escaping tests.
- AI classification/degraded-state UI tests once intelligence read models exist.
- Social approval/evidence drawer/child hold screen Playwright tests once
  social read models exist.
- Browser-game dashboard/evidence drawer/child checking screen Playwright tests
  once browser-game read models exist.

## AI Worker Checklist

Fill this before reporting `DONE` or PR-ready:

- [ ] Confirm source docs read: [folder README](../README.md), [source index](../source-index.md), [current snapshot](../current-browser-snapshot.md), [full scope plan](../v0-5-managed-browser-full-scope-plan.md), [test blueprint](../v0-5-managed-browser-test-blueprint.md), [UI/UX guide](../ui-ux-requirements-guide.md), [main checklist](../implementation-checklist.md), and this workpack.
- [ ] Check enhancement overlap: URL/video AI intelligence, social platform/account/feed gating, and browser games/cloud gaming.
- [ ] Hub lock covers this workpack and exact implementation/docs paths.
- [ ] Existing source layout inspected; no parallel browser truth created.
- [ ] Before-state source snapshot recorded in `output/browser-plan-proof/14-portal-browser-status-surfaces/00-source-snapshot.md`.
- [ ] Contracts updated first where this workpack changes behavior.
- [ ] Rust/service/portal parity updated only after contracts exist.
- [ ] Raw evidence artifacts captured where applicable: service event fixture proof exists in `apps/portal/tests/live-activity-state.test.ts`, `apps/portal/tests/live-activity-browser-status.test.ts`, and `packages/portal-domain/tests/parent-portal-service-state.test.ts`; real browser/manual platform evidence is explicitly not claimed in `output/browser-plan-proof/14-portal-browser-status-surfaces/09-manual-platform-proof.md`.
- [ ] Tests/proof listed in this workpack are implemented for the status-surface slice; social/video/game, child-facing, intervention-action, and final visual-polish surfaces remain later workpacks/manual-required.
- [ ] Validation command outputs saved in the proof pack and summarized in [main checklist](../implementation-checklist.md).
- [ ] UI snapshots captured for the touched parent portal route in `output/browser-plan-proof/14-portal-browser-status-surfaces/06-ui-snapshots/browser-route-inventory-status.png`; child UX, block/warn, policy authoring, responsive matrix, and malicious/long evidence text snapshots are not applicable to this WP14 slice because no raw URL/title/game/social text is rendered.
- [ ] Security/no-claim negative proof captured: portal parser tests and portal-domain service-state tests preserve not-claimed exact URL/active-tab capability and report-only unmanaged fallback.
- [ ] Manual platform proof decision captured in `output/browser-plan-proof/14-portal-browser-status-surfaces/09-manual-platform-proof.md`; no real OS/browser/platform claim was upgraded.
- [ ] Evidence/proof artifact paths recorded in [main checklist](../implementation-checklist.md).
- [ ] Feature/expectation/product-checklist/README update decision recorded in [main checklist](../implementation-checklist.md).
- [ ] Known gaps, deferred items, and no-claim boundaries recorded before `DONE`.

## Manual-Required Gaps

UI cannot claim exact evidence, active-tab enforcement, unmanaged exact URL
evidence, or intervention/action success unless later service/platform proof
backs it. WP14 does not claim social/video/game read models, child-facing UX,
native extension/native-host support, or real browser/platform coverage.

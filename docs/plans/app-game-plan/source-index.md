# App + Game Source Index

This index records source truth for native app and native game work. It should
be updated when implementation changes ownership, proof, or routing.

## Product Source Docs

- Owning feature: [App and game control](../../features/app-game-control.md)
- Main expectation: [App/game evidence](../../expectations/app-game-evidence.md)
- Main architecture: [App/game evidence sessions](../../architecture/app-game-evidence-sessions.md)
- Existing native app planning: [native apps plan](../app-plan/README.md)
- App capability guide: [app control capability guide](../../app-control-capability-guide.md)
- App settings source: [app control settings inventory](../../app-control-settings-inventory.md)
- Game settings source: [game control settings inventory](../../game-control-settings-inventory.md)
- Browser-game handoff: [browser games/cloud gaming plan](../browser-plan/v0-5-browser-games-cloud-gaming-gating-plan.md)
- Adjacent expectations: [policy](../../expectations/policy.md),
  [enforcement](../../expectations/enforcement.md),
  [platforms](../../expectations/platforms.md),
  [evidence storage](../../expectations/evidence-storage.md),
  [AI](../../expectations/ai.md)

## Routing Rule

Route here:

- non-browser installed app and game inventory;
- app/game identity and identity merge confidence;
- native process/package/runtime evidence;
- foreground app/game evidence;
- launcher evidence and launcher-game candidate handling;
- app/game sessionization and duration;
- native app category and native game category/rating/risk taxonomy;
- app/game policy targets and time-budget compiler inputs;
- unknown app and unknown game approval;
- native app warning, asking, time-limit, terminate, hide, suspend, shield,
  block-launch, allowlist, and manual-required states;
- game budgets, multiplayer/UGC/purchase signal boundaries, launcher-only
  explanations, and game-specific UI;
- platform authority tiers for native app/game control;
- proof gates and UI/UX acceptance for app/game rows.

Do not route here:

- browser URL/page/video logic;
- browser games or cloud gaming running inside a managed browser;
- decrypted network payloads;
- screen content analysis;
- message/chat content capture;
- full install/purchase approval product flows outside the app/game handoff.

## TypeScript Ownership

Current app/game evidence and session contracts live in:

- `packages/activity-domain/src/app-game-primitives.ts`
- `packages/activity-domain/src/app-game.ts`
- `packages/activity-domain/tests/app-game.test.ts`

Current app-control and app/game policy/catalog meaning lives in:

- `packages/parent-domain/src/app-control-catalog.ts`
- `packages/parent-domain/src/app-control-catalog-schema.ts`
- `packages/parent-domain/src/app-control-catalog-data.ts`
- `packages/parent-domain/src/app-control-guide-catalog-data.ts`
- `packages/parent-domain/src/game-control-catalog.ts`
- `packages/parent-domain/src/game-control-catalog-schema.ts`
- `packages/parent-domain/src/game-control-catalog-data.ts`
- `packages/parent-domain/src/enforcement-policy-dispatch.ts`
- `packages/parent-domain/src/policy.ts`
- `packages/parent-domain/tests/app-control-policy-catalog.test.ts`
- `packages/parent-domain/tests/game-control-policy-catalog.test.ts`
- `packages/parent-domain/tests/enforcement-approval-audit.test.ts`

New shared app/game schemas should extend these packages first. Do not create a
parallel domain package unless the existing package ownership cannot carry the
new contract cleanly.

## Rust Ownership

Current Rust protocol and runtime proof paths include:

- `crates/agent-protocol/src/app_game.rs`
- `crates/agent-protocol/src/app_game_tests.rs`
- `crates/agent-core/src/activity_store_app_game.rs`
- `crates/agent-core/src/activity_store_app_game_observation.rs`
- `crates/agent-core/src/activity_store_app_game_rows.rs`
- `crates/agent-core/src/activity_store_app_game_tests.rs`
- `crates/agent-core/src/enforcement_app_time_limit.rs`
- `crates/agent-core/src/enforcement_app_time_limit_tests.rs`
- `crates/agent-service/src/app.rs`

Rust protocol parity must mirror TypeScript contracts exactly for payloads that
cross the service boundary. Platform adapter behavior belongs behind typed
capability and authority-tier checks.

## Portal Ownership

Current portal-related app/game surfaces include:

- `apps/portal/src/live-activity-state.ts`
- `apps/portal/src/live-activity-panel.ts`
- `apps/portal/src/activity-timeline.ts`
- `apps/portal/src/policy-preview-panel.ts`
- `apps/portal/src/policy-preview-read-model.ts`
- `apps/portal/src/policy-preview-details.ts`
- `apps/portal/src/portal-capability-guidance.ts`
- `apps/portal/src/portal-device-rule-scope.ts`
- `apps/portal/src/PortalAppLayoutSurfacePanel.tsx`
- `apps/portal/src/PortalAppLayoutContentPanel.tsx`
- `apps/portal/public/parent-nav-app.svg`
- `apps/portal/public/parent-nav-games.svg`
- `vendor/ocentra-parent-core-ui/AppPages/ParentPortal/activity-ui-intent.ts`

Portal work must render service-backed rows or explicit UI fixtures. It must not
scan the OS, classify apps, run timers, or call enforcement adapters.

## Existing Proof Scripts

- `node scripts/test/v0-8-windows-app-time-limit-adapter-mvp.mjs`
- `node scripts/test/v0-8-enforcement-policy-dispatch-proof.mjs`
- `node scripts/test/v0-8-enforcement-product-control-spine.mjs`
- `node scripts/test/v0-8-enforcement-integrity-runtime-audit.mjs`
- `node scripts/test/v0-8-cross-platform-enforcement-capability-proof.mjs`
- `node scripts/test/v0-8-enforcement-timer-recovery-mvp.mjs`

These scripts prove scoped app/game paths only. They do not prove broad
installed-app blocking, launcher/game disambiguation, game budgets, ratings, UGC
signals, purchases, or cross-platform parity.

## Current Test Files

- `packages/activity-domain/tests/app-game.test.ts`
- `packages/parent-domain/tests/app-control-policy-catalog.test.ts`
- `packages/parent-domain/tests/game-control-policy-catalog.test.ts`
- `packages/parent-domain/tests/enforcement-approval-audit.test.ts`
- `crates/agent-protocol/src/app_game_tests.rs`
- `crates/agent-core/src/activity_store_app_game_tests.rs`
- `crates/agent-core/src/enforcement_app_time_limit_tests.rs`
- `apps/portal/tests/live-activity-state.test.ts`
- `apps/portal/tests/live-activity-surface-adapter.test.ts`
- `apps/portal/tests/policy-preview-live-activity-state.test.ts`
- `apps/portal/tests/activity-ui-intent.test.ts`

## Source Truth Rule

When this plan and existing feature/expectation docs differ, update the owning
feature or expectation doc before making implementation claims. Until those
locked docs can be edited, record the bridge gap in
[pasted content coverage audit](pasted-content-coverage-audit.md).

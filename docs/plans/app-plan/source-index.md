# Native App Source Index

This index keeps the native app plan tied to existing source documents and code.
It is not a replacement for feature, expectation, roadmap, checklist, package,
or crate ownership docs.

## Product Source Docs

- Owning feature: [App and game control](../../features/app-game-control.md)
- Main expectation: [App and game evidence](../../expectations/app-game-evidence.md)
- Milestone expectation:
  [V0.5.2 app/game evidence sessions](../../expectations/roadmap-v0-5-2-app-game-evidence-sessions.md)
- Main architecture:
  [App and game evidence sessions](../../architecture/app-game-evidence-sessions.md)
- App capability guide: [App control capability guide](../../app-control-capability-guide.md)
- App schema proposal: [App control schema proposal](../../app-control-schema-proposal.md)
- Raw app catalog: [App control settings inventory](../../app-control-settings-inventory.md)
- Implementation tracking:
  [Native Apps Implementation Checklist](implementation-checklist.md)
- Pasted-content reconciliation:
  [Pasted Content Coverage Audit](pasted-content-coverage-audit.md)
- Platform authority plan:
  [V0.5 Native Apps Platform Deep Dive](v0-5-native-apps-platform-deep-dive.md)
- Test and proof blueprint:
  [V0.5 Native Apps Test Blueprint](v0-5-native-apps-test-blueprint.md)
- Adjacent expectation: [Policy](../../expectations/policy.md)
- Adjacent expectation: [Enforcement](../../expectations/enforcement.md)
- Adjacent expectation: [Platforms](../../expectations/platforms.md)
- Adjacent expectation:
  [App install and purchase approval](../../expectations/app-install-purchase-approval.md)
- Adjacent expectation: [Evidence storage](../../expectations/evidence-storage.md)
- Adjacent expectation: [AI](../../expectations/ai.md)

## Routing: Move Here Or Point Here

Native app implementation planning belongs in this folder when it is about:

- non-browser installed app inventory;
- app identity and merge confidence;
- process/package/runtime evidence;
- foreground app evidence;
- app sessionization and duration;
- native app category and risk taxonomy;
- app policy targets and compiler inputs;
- new app and unknown app approval;
- app-specific warn, ask, time-limit, terminate, hide, suspend, shield,
  block-launch, allowlist, and manual-required states;
- platform authority tiers for app control;
- app proof gates and UI/UX acceptance.

Shared source docs stay where they are and are pointed to from this folder:

- feature docs stay under `docs/features`;
- expectation docs stay under `docs/expectations`;
- architecture docs stay under `docs/architecture`;
- product checklist and roadmap stay at the docs root;
- package/crate/app ownership docs stay next to their source;
- raw generated inventories stay at the docs root.

Do not move generated catalog inventories into this plan folder. Link them here
and update them only when their acceptance contract or generated data changes.

## Feature Routing

| Feature doc                          | App-plan relationship                                                                                                                                                                  |
| ------------------------------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `app-game-control.md`                | Owning feature. Native app plan work derives from this file and feeds status/proof updates back to it.                                                                                 |
| `app-install-purchase-approval.md`   | Adjacent install/purchase feature. New app detection may request approval, but store purchase/install product flows stay here.                                                         |
| `browser-web-control.md`             | Adjacent browser feature. Browser games, web apps, browser social/video, and exact URL/tab evidence stay in browser-plan.                                                              |
| `enforcement-integrity-tamper.md`    | Shared enforcement boundary. App hard controls must remain capability-gated here.                                                                                                      |
| `evidence-store-query.md`            | Shared storage boundary. App evidence must journal/replay through shared evidence stores.                                                                                              |
| `local-ai-safety-evaluator.md`       | Adjacent AI feature. App AI/classifier output is evidence and must not directly enforce.                                                                                               |
| `policy-schedules-approvals.md`      | Shared approval/evaluator feature. App rules, bonus time, and ask-parent flows must use typed approval contracts.                                                                      |
| `remote-lan-mobile-platforms.md`     | Platform routing. Android/iOS/mobile states remain platform-specific/manual-required until proof exists.                                                                               |
| `production-distribution-support.md` | Release/support boundary. Support bundles must redact private paths, command lines, child activity, journals, SQLite, screenshots, tokens, and platform proof artifacts as configured. |
| `social-video-control.md`            | Adjacent product feature. Native social/video apps can be detected as apps, but message/content/feed understanding remains social/video scope.                                         |

## Adjacent Plan Docs

- Browser plan: [Browser plan README](../browser-plan/README.md)
- LAN plan: [LAN plan README](../lan-plan/README.md)
- V0.8 enforcement plan:
  [Enforcement control plan](../v0-8-enforcement-control-plan/)
- Portal UX workpack:
  [Browser, app, and network surfaces](../portal-ux-household-surfaces-plan/workpacks/09-browser-app-and-network-surfaces.md)

## TypeScript Ownership

- `packages/activity-domain/src/app-game.ts`
- `packages/activity-domain/src/app-game-category-risk-primitives.ts`
- `packages/activity-domain/src/app-game-category-risk.ts`
- `packages/activity-domain/src/app-game-primitives.ts`
- `packages/activity-domain/src/app-game-session-primitives.ts`
- `packages/activity-domain/src/app-game-foreground.ts`
- `packages/activity-domain/tests/app-game-foreground.test.ts`
- `packages/activity-domain/tests/app-game.test.ts`
- `packages/parent-domain/src/app-control-catalog.ts`
- `packages/parent-domain/src/app-control-catalog-schema.ts`
- `packages/parent-domain/src/app-control-catalog-data.ts`
- `packages/parent-domain/src/app-control-guide-catalog-data.ts`
- `packages/parent-domain/src/enforcement-policy-dispatch.ts`
- `packages/parent-domain/src/app-game-control-authority.ts`
- `packages/parent-domain/src/app-game-control-platform-authority.ts`
- `packages/parent-domain/src/app-game-control-platform-authority-rules.ts`
- `packages/parent-domain/src/policy.ts`
- `packages/parent-domain/tests/app-control-policy-catalog.test.ts`
- `packages/parent-domain/tests/app-game-control-platform-authority.test.ts`
- `packages/parent-domain/tests/enforcement-approval-audit.test.ts`

TypeScript rule: enhance these existing app/app-game paths first. Do not create
a parallel app domain package unless an ownership boundary genuinely changes.
If the implementation splits app-only contracts from app/game contracts, the
split must be schema-backed, test-backed, and reconciled in this source index.

## Rust Ownership

- `crates/agent-protocol/src/app_game.rs`
- `crates/agent-protocol/src/app_game_tests.rs`
- `crates/agent-core/src/activity_store_app_game.rs`
- `crates/agent-core/src/activity_store_app_game/app_game_sessionization.rs`
- `crates/agent-core/src/activity_store_app_game/app_game_session_rollups.rs`
- `crates/agent-core/src/activity_store_app_game/app_game_session_time.rs`
- `crates/agent-core/src/activity_store_app_game/app_game_sessionization_tests.rs`
- `crates/agent-core/src/activity_store_app_game/app_game_journal_sqlite_ingest.rs`
- `crates/agent-core/src/activity_store_app_game/app_game_journal_sqlite_ingest_tests.rs`
- `crates/agent-core/src/activity_store_app_game/app_game_windows_foreground.rs`
- `crates/agent-core/src/activity_store_app_game/app_game_windows_foreground_tests.rs`
- `crates/agent-core/src/activity_store_app_game_observation.rs`
- `crates/agent-core/src/activity_store_app_game_rows.rs`
- `crates/agent-core/src/activity_store_app_game_tests.rs`
- `crates/agent-core/src/enforcement_app_time_limit.rs`
- `crates/agent-core/src/enforcement_app_time_limit_tests.rs`
- `crates/agent-service/src/app.rs`

Rust rule: TypeScript contracts come first, Rust protocol parity second,
`agent-core` runtime/storage helpers third, and `agent-service` command/read-model
wiring fourth.

## Portal Ownership

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

Portal rule: render service-backed state and typed manifests. Portal must not
scan app inventory, inspect processes, read SQLite/journals directly, run AI
classification, run timers, or enforce.

## Proof Scripts

- `node scripts/test/v0-8-windows-app-time-limit-adapter-mvp.mjs`
- `node scripts/test/v0-8-enforcement-policy-dispatch-proof.mjs`
- `node scripts/test/v0-8-enforcement-product-control-spine.mjs`
- `node scripts/test/v0-8-enforcement-integrity-runtime-audit.mjs`
- `node scripts/test/v0-8-cross-platform-enforcement-capability-proof.mjs`
- `node scripts/test/v0-8-enforcement-timer-recovery-mvp.mjs`

Future app-specific proof scripts should use:

```text
output/app-plan-proof/<workpack-id>/
```

## Shared App/Game Proof Bridge

The native app plan uses the shared app/game evidence spine through the early
evidence workpacks instead of creating parallel app-only truth. App-plan proof
packs mirror the app/game proof roots and record product-doc decisions.

| App-plan workpack                     | App-plan proof root                                                 | Shared app/game proof root                                               | Boundary                           |
| ------------------------------------- | ------------------------------------------------------------------- | ------------------------------------------------------------------------ | ---------------------------------- |
| WP01 contract boundary                | `output/app-plan-proof/01-contract-boundary-and-effect-schemas`     | `output/app-game-plan-proof/01-contract-boundary-and-effect-schemas`     | Contract/proof reconciliation only |
| WP02 source reconciliation            | `output/app-plan-proof/02-source-index-and-doc-reconciliation`      | `output/app-game-plan-proof/02-source-index-and-doc-reconciliation`      | Routing/docs only                  |
| WP03 snapshot/gap map                 | `output/app-plan-proof/03-current-app-snapshot-and-gap-map`         | `output/app-game-plan-proof/03-current-app-game-snapshot-and-gap-map`    | Snapshot/gap proof only            |
| WP04 app identity                     | `output/app-plan-proof/04-app-identity-model`                       | `output/app-game-plan-proof/04-app-game-identity-model`                  | Contract proof only                |
| WP05 installed inventory model        | `output/app-plan-proof/05-installed-app-inventory-model`            | `output/app-game-plan-proof/05-inventory-evidence-model`                 | Contract proof only                |
| WP06 Windows installed inventory      | `output/app-plan-proof/06-windows-installed-app-inventory-adapter`  | `output/app-game-plan-proof/06-windows-installed-inventory-adapter`      | Parser proof only                  |
| WP07 Windows Store/UWP/AppX inventory | `output/app-plan-proof/07-windows-store-uwp-appx-inventory-adapter` | `output/app-game-plan-proof/07-windows-store-uwp-appx-inventory-adapter` | Parser proof only                  |
| WP08 Windows process runtime          | `output/app-plan-proof/08-windows-process-runtime-evidence-adapter` | `output/app-game-plan-proof/08-windows-process-runtime-evidence-adapter` | Runtime parser proof only          |
| WP09 Windows foreground evidence      | `output/app-plan-proof/09-windows-foreground-app-evidence-adapter`  | `output/app-game-plan-proof/09-windows-foreground-evidence-adapter`      | Foreground parser proof only       |
| WP10 cross-platform authority matrix  | `output/app-plan-proof/10-cross-platform-authority-matrix`          | `output/app-game-plan-proof/11-cross-platform-authority-matrix`          | Authority contract proof only      |
| WP11 app category/risk taxonomy       | `output/app-plan-proof/11-app-category-and-risk-taxonomy`           | `output/app-game-plan-proof/12-app-game-category-and-risk-taxonomy`      | Category/risk contract proof only  |
| WP12 app sessionization/duration      | `output/app-plan-proof/12-app-sessionization-and-duration-engine`   | `output/app-game-plan-proof/13-sessionization-and-duration-engine`       | SQLite-row session reducer proof   |
| WP13 journal/SQLite app ingest        | `output/app-plan-proof/13-journal-and-sqlite-app-ingest`            | `output/app-game-plan-proof/14-journal-and-sqlite-ingest`                | Encrypted journal replay proof     |

These completed rows do not add live OS crawling, encrypted journal-file
ingest, service events, portal rows, content knowledge, policy execution,
install control, broad blocking, or runtime cross-platform parity. Those claims
remain assigned to later app-plan/app-game workpacks.

The WP12/WP13 sessionization proof narrows the storage gap in two stages:
deterministic replay from stored SQLite observation rows is covered for process
and foreground session summaries plus daily rollups, and staged encrypted
journal-file replay is now covered for typed inventory, runtime, foreground,
launcher, running-now, foreground-now, and daily rollup rows. Service events,
portal rows, policy execution, live source subscriptions, journal
corruption/recovery, and platform authority proof remain later work.

## Current Test Files

- `packages/activity-domain/tests/app-game.test.ts`
- `packages/activity-domain/tests/app-game-category-risk.test.ts`
- `packages/parent-domain/tests/app-control-policy-catalog.test.ts`
- `packages/parent-domain/tests/app-game-control-platform-authority.test.ts`
- `packages/parent-domain/tests/enforcement-approval-audit.test.ts`
- `crates/agent-protocol/src/app_game_tests.rs`
- `crates/agent-core/src/activity_store_app_game/app_game_sessionization_tests.rs`
- `crates/agent-core/src/activity_store_app_game/app_game_journal_sqlite_ingest_tests.rs`
- `crates/agent-core/src/activity_store_app_game_tests.rs`
- `crates/agent-core/src/enforcement_app_time_limit_tests.rs`
- `apps/portal/tests/live-activity-state.test.ts`
- `apps/portal/tests/live-activity-surface-adapter.test.ts`
- `apps/portal/tests/policy-preview-live-activity-state.test.ts`
- `apps/portal/tests/activity-ui-intent.test.ts`

## Source Truth Rule

When an app workpack changes product state, update the owning feature doc,
matching expectation docs, product capability checklist row, and touched module
README. If the work only adds planning detail inside this folder, no product
status update is required.

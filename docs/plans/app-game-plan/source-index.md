# App + Game Source Index

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `App + Game Source Index`
> Kind: source ownership index; read only when source ownership is unclear.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not inspect broad source from here; use only the named package/crate path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.

<!-- /agent-capsule -->

## Current source correction - 2026-08-15

The older TypeScript inventory below is historical. The tracked tree contains
no files under the former `packages/activity-domain`, `packages/parent-domain`,
`packages/agent-protocol-domain`, or `packages/text-domain` App/Game owners, and
no tracked `scripts/test/app-game-*` runners. Current ownership is recorded
workpack-by-workpack in [CODE_AUDIT.md](CODE_AUDIT.md) and
`docs/engineering-graph/code-map.json`: Rust protocol/core/service/parent
runtime, portal/generated schema edges, and Android Java runtime sources.

Do not recreate removed packages to satisfy historical workpack prose.

This index records source truth for native app and native game work. It should
be updated when implementation changes ownership, proof, or routing.

## Product Source Docs

- Owning feature: [App and game control](../../features/app-game-control.md)
- Main expectation: [App/game evidence](../../expectations/app-game-evidence.md)
- Main architecture: [App/game evidence sessions](../../architecture/app-game-evidence-sessions.md)
- Shared implementation route: [app + game plan](README.md)
- Existing native app planning: [native apps plan](../app-plan/README.md)
- App capability guide: [app control capability guide](../../plans/app-game-plan/workpacks/app-control-capability-guide.md)
- App settings source: [app control settings inventory](../../plans/app-game-plan/workpacks/app-control-settings-inventory.md)
- Game settings source: [game control settings inventory](../../plans/app-game-plan/workpacks/game-control-settings-inventory.md)
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
- `packages/activity-domain/src/app-game-identity-primitives.ts`
- `packages/activity-domain/src/app-game-inventory-primitives.ts`
- `packages/activity-domain/src/app-game-inventory.ts`
- `packages/activity-domain/src/app-game-category-risk-primitives.ts`
- `packages/activity-domain/src/app-game-category-risk.ts`
- `packages/activity-domain/src/app-game-runtime.ts`
- `packages/activity-domain/src/app-game-foreground.ts`
- `packages/activity-domain/src/app-game-session-primitives.ts`
- `packages/activity-domain/src/app-game.ts`
- `packages/activity-domain/src/activity-surface.ts`
- `packages/activity-domain/tests/app-game.test.ts`
- `packages/activity-domain/tests/app-game-evidence-claim.test.ts`
- `packages/activity-domain/tests/app-game-identity.test.ts`
- `packages/activity-domain/tests/app-game-inventory.test.ts`
- `packages/activity-domain/tests/app-game-category-risk.test.ts`
- `packages/activity-domain/tests/app-game-runtime.test.ts`
- `packages/activity-domain/tests/app-game-foreground.test.ts`
- `packages/activity-domain/tests/activity-surface.test.ts`

Current app-control and app/game policy/catalog meaning lives in:

- `packages/parent-domain/src/app-control-catalog.ts`
- `packages/parent-domain/src/app-control-catalog-schema.ts`
- `packages/parent-domain/src/app-control-catalog-data.ts`
- `packages/parent-domain/src/app-control-guide-catalog-data.ts`
- `packages/parent-domain/src/game-control-catalog.ts`
- `packages/parent-domain/src/game-control-catalog-schema.ts`
- `packages/parent-domain/src/game-control-catalog-data.ts`
- `packages/parent-domain/src/enforcement-policy-dispatch.ts`
- `packages/parent-domain/src/app-game-child-facing-ux.ts`
- `packages/parent-domain/src/app-game-child-facing-ux-rules.ts`
- `packages/parent-domain/src/app-game-control-approval-flow.ts`
- `packages/parent-domain/src/app-game-control-authority.ts`
- `packages/parent-domain/src/app-game-control-authority-rules.ts`
- `packages/parent-domain/src/app-game-control-platform-authority.ts`
- `packages/parent-domain/src/app-game-control-platform-authority-rules.ts`
- `packages/parent-domain/src/app-game-platform-extension-routing.ts`
- `packages/parent-domain/src/app-game-platform-extension-routing-rules.ts`
- `packages/parent-domain/src/app-game-platform-extension-routing-data.ts`
- `packages/parent-domain/src/app-game-platform-extension-routing-data-support.ts`
- `packages/parent-domain/src/app-game-platform-extension-routing-macos-data.ts`
- `packages/parent-domain/src/app-game-platform-extension-routing-ios-data.ts`
- `packages/parent-domain/src/app-game-platform-extension-routing-android-data.ts`
- `packages/parent-domain/src/app-game-platform-extension-routing-linux-data.ts`
- `packages/parent-domain/src/app-game-performance-health.ts`
- `packages/parent-domain/src/app-game-performance-health-rules.ts`
- `packages/parent-domain/src/app-game-performance-health-proof.ts`
- `packages/parent-domain/src/app-game-policy-target-compiler.ts`
- `packages/parent-domain/src/app-game-policy-target-compiler-rules.ts`
- `packages/parent-domain/src/app-game-time-budget-policy.ts`
- `packages/parent-domain/src/app-game-time-budget-policy-rules.ts`
- `packages/parent-domain/src/app-game-ai-classifier-boundary.ts`
- `packages/parent-domain/src/app-game-ai-classifier-boundary-values.ts`
- `packages/parent-domain/src/app-game-ai-classifier-boundary-data.ts`
- `packages/parent-domain/src/native-game-budget-policy.ts`
- `packages/parent-domain/src/native-game-budget-policy-rules.ts`
- `packages/parent-domain/src/policy.ts`
- `packages/parent-domain/tests/app-control-policy-catalog.test.ts`
- `packages/parent-domain/tests/game-control-policy-catalog.test.ts`
- `packages/parent-domain/tests/app-game-child-facing-ux.test.ts`
- `packages/parent-domain/tests/app-game-control-authority.test.ts`
- `packages/parent-domain/tests/app-game-unknown-approval-flow.test.ts`
- `packages/parent-domain/tests/app-game-control-platform-authority.test.ts`
- `packages/parent-domain/tests/app-game-policy-target-compiler.test.ts`
- `packages/parent-domain/tests/app-game-time-budget-policy.test.ts`
- `packages/parent-domain/tests/app-game-time-budget-policy-recovery.test.ts`
- `packages/parent-domain/tests/app-game-ai-classifier-boundary.test.ts`
- `packages/parent-domain/tests/app-game-platform-extension-routing.test.ts`
- `packages/parent-domain/tests/native-game-budget-policy.test.ts`
- `packages/parent-domain/tests/enforcement-approval-audit.test.ts`

New shared app/game schemas should extend these packages first. Do not create a
parallel domain package unless the existing package ownership cannot carry the
new contract cleanly.

Current app/game child-facing copy tokens live in:

- `packages/text-domain/src/app-game-child-ux-text.ts`
- `packages/text-domain/tests/app-game-child-ux-text.test.ts`

## Rust Ownership

Current Rust protocol and runtime proof paths include:

- `crates/agent-protocol/src/app_game.rs`
- `crates/agent-protocol/src/app_game_tests.rs`
- `crates/agent-core/src/activity_store_app_game.rs`
- `crates/agent-core/src/activity_store/internals.rs`
- `crates/agent-core/src/activity_store_app_game/app_game_windows_inventory.rs`
- `crates/agent-core/src/activity_store_app_game/app_game_windows_inventory_tests.rs`
- `crates/agent-core/src/activity_store_app_game/app_game_windows_store_inventory.rs`
- `crates/agent-core/src/activity_store_app_game/app_game_windows_store_inventory_tests.rs`
- `crates/agent-core/src/activity_store_app_game/app_game_windows_process_runtime.rs`
- `crates/agent-core/src/activity_store_app_game/app_game_windows_process_runtime_tests.rs`
- `crates/agent-core/src/activity_store_app_game/app_game_windows_foreground.rs`
- `crates/agent-core/src/activity_store_app_game/app_game_windows_foreground_tests.rs`
- `crates/agent-core/src/activity_store_app_game/app_game_sessionization.rs`
- `crates/agent-core/src/activity_store_app_game/app_game_session_rollups.rs`
- `crates/agent-core/src/activity_store_app_game/app_game_session_time.rs`
- `crates/agent-core/src/activity_store_app_game/app_game_sessionization_tests.rs`
- `crates/agent-core/src/activity_store_app_game/app_game_journal_sqlite_ingest.rs`
- `crates/agent-core/src/activity_store_app_game/app_game_journal_sqlite_ingest/read_model.rs`
- `crates/agent-core/src/activity_store_app_game/app_game_journal_sqlite_ingest_tests.rs`
- `crates/agent-core/src/activity_store_app_game_observation.rs`
- `crates/agent-core/src/activity_store_app_game_rows.rs`
- `crates/agent-core/src/activity_store_app_game_tests.rs`
- `crates/agent-core/src/enforcement_app_time_limit.rs`
- `crates/agent-core/src/enforcement_app_time_limit_tests.rs`
- `crates/agent-service/src/app.rs`
- `crates/agent-service/src/activity_surface_adapter.rs`
- `crates/agent-service/src/activity_surface_read_models.rs`
- `crates/agent-service/src/activity_surface_read_models/app_use.rs`
- `crates/agent-service/src/activity_surface_read_models/app_use/source.rs`
- `crates/agent-service/src/activity_surface_read_models/games.rs`
- `crates/agent-service/src/activity_surface_read_models/shared.rs`
- `crates/agent-service/src/activity_surface_store.rs`

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
- `packages/portal-domain/src/parent-portal-data.ts`
- `vendor/ocentra-parent-core-ui/AppPages/ParentPortal/activity-ui-intent.ts`
- `vendor/ocentra-parent-core-ui/AppPages/ParentPortal/app-game-dashboard-intent.ts`

Portal work must render service-backed rows or explicit UI fixtures. It must not
scan the OS, classify apps, run timers, or call enforcement adapters.

## Existing Proof Scripts

- `node scripts/test/v0-8-windows-app-time-limit-adapter-mvp.mjs`
- `node scripts/test/v0-8-enforcement-policy-dispatch-proof.mjs`
- `node scripts/test/v0-8-enforcement-product-control-spine.mjs`
- `node scripts/test/v0-8-enforcement-integrity-runtime-audit.mjs`
- `node scripts/test/v0-8-cross-platform-enforcement-capability-proof.mjs`
- `node scripts/test/v0-8-enforcement-timer-recovery-mvp.mjs`
- `node scripts/test/app-game-broad-blocking-proof-gates.mjs`
- `node scripts/test/app-game-ai-classifier-boundary-proof.mjs`
- `node scripts/test/app-game-platform-extension-routing-proof.mjs`
- `node scripts/test/app-game-install-store-handoff-proof.mjs`
- `node scripts/test/app-game-performance-health-proof.mjs`
- `node scripts/test/app-game-plan-rollout-pr-gate.mjs`

These scripts prove scoped app/game paths only. They do not prove broad
installed-app blocking, launcher/game disambiguation, game budgets, ratings, UGC
signals, purchases, or cross-platform parity.

WP01 proof on `codex/app-plan-work` adds contract-only proof under
`output/app-game-plan-proof/01-contract-boundary-and-effect-schemas/`. That
proof does not replace Rust protocol parity, runtime adapter proof, portal UI
proof, or SQLite/journal proof.

WP04 proof on `codex/app-game-identity-contracts` adds contract-only identity
and identity-merge proof under
`output/app-game-plan-proof/04-app-game-identity-model/`. That proof does not
replace inventory adapters, Rust protocol parity, runtime merge behavior, or
portal identity rows.

WP05 proof on `codex/app-game-inventory-evidence-model` adds contract-only
inventory evidence rows with source, custody, category candidates, stale and
permission-limited states, and no-use guards under
`output/app-game-plan-proof/05-inventory-evidence-model/`. That proof does not
replace platform inventory adapters, journal ingest, runtime use evidence, or
portal inventory rows.

WP06 proof on `codex/app-game-windows-installed-inventory` mirrors the WP05
inventory row shape into Rust protocol and adds a narrow `agent-core`
Windows-installed inventory record adapter/parser proof under
`output/app-game-plan-proof/06-windows-installed-inventory-adapter/`. That proof
covers registry-like records, Start Menu shortcut records, launcher manifest
game rows, strong-identity dedupe, display-only non-merge, and no-use guards. It
does not prove live registry crawling, shell-link parsing, executable metadata,
signature/hash collection, journal ingest, service events, portal rows, or broad
blocking.

WP07 proof on `codex/app-game-windows-store-inventory` adds first-class
Microsoft Store/UWP/AppX/MSIX package parser proof under
`output/app-game-plan-proof/07-windows-store-uwp-appx-inventory-adapter/`. It
covers store app rows, store game rows, package/AUMID deterministic runtime
merge checks, AppUserModelId policy-target handoff, and no-use guards. It does
not prove live package enumeration, Store API integration, package install or
purchase approval, journal ingest, service events, portal rows, or broad
package blocking.

WP08 proof on `codex/app-game-windows-process-runtime` adds a first-class
TypeScript app/game runtime evidence contract, Rust protocol parity, and a
staged `agent-core` Windows process runtime parser under
`output/app-game-plan-proof/08-windows-process-runtime-evidence-adapter/`. It
covers process appearance, same-process persistence, process exit closure,
unknown process non-promotion, launcher runtime-only state, permission-limited
metadata, session-ready runtime summaries, and runtime-is-not-foreground guards.
It does not prove live process polling, journal ingest, SQLite replay, service
events, portal rows, policy execution, foreground evidence, content knowledge,
or broad blocking.

WP09 proof on `codex/app-game-windows-foreground-evidence` adds a first-class
TypeScript app/game foreground evidence contract, Rust protocol parity, and a
staged `agent-core` Windows foreground-window parser under
`output/app-game-plan-proof/09-windows-foreground-evidence-adapter/`. It covers
active foreground focus, foreground switch closure, background process no-time
guards, omitted title refs, permission-limited foreground metadata, launcher
focus staying launcher-only, unknown foreground process non-promotion, and
foreground-is-not-content guards. It does not prove live foreground polling,
journal ingest, SQLite replay, service events, portal rows, content knowledge,
policy execution, or broad blocking.

WP10 proof on `codex/app-game-launcher-candidate-model` adds a first-class
TypeScript launcher evidence contract, Rust protocol parity, and a staged
`agent-core` Windows launcher evidence parser under
`output/app-game-plan-proof/10-launcher-evidence-and-game-candidate-model/`.
It covers launcher-only evidence, launcher foreground staying launcher-only,
launcher-game candidate downgrade when child-game proof is missing,
deterministic child-game proof promotion, permission-limited launcher state, and
known-game no-claim guards. It does not prove live launcher manifest crawling,
live child-process linking, journal ingest, SQLite replay, service events,
portal launcher rows, game-budget policy, install/purchase approval, or broad
blocking.

WP11 proof on `codex/app-game-authority-matrix` adds a parent-domain
cross-platform authority matrix contract under
`output/app-game-plan-proof/11-cross-platform-authority-matrix/`. It covers
platform/action rows, authority tiers, setup states, proof states,
parent-visible limitations, proof-needed lists, no-execute guards for
observe-only/manual-required/not-claimed rows, Android Device Owner/Profile
Owner hide/suspend proof, iOS FamilyControls/ManagedSettings shield proof,
macOS MDM/Endpoint/System Extension hard-block proof, Linux mechanism/distro
session proof, Windows AppLocker/App Control hard-block proof routing, and
duplicate platform/action matrix rejection. It does not prove runtime platform
adapters, Rust protocol/service parity, portal UI, or real device authority.

WP12 proof on `codex/app-game-category-risk-taxonomy` adds an activity-domain
category/risk taxonomy contract under
`output/app-game-plan-proof/12-app-game-category-and-risk-taxonomy/`. It covers
native app categories, native game categories, risk candidates, game context
signals, source refs, confidence, reason codes, evidence refs, parent display
overrides, AI digest refs, policy-candidate actions, duplicate candidate
rejection, and no-direct-enforcement guards. It does not prove live catalog
enrichment, local AI classifier quality, policy compiler routing, portal
category UI, service/read-model parity, or adapter authority.

WP13 proof on `codex/app-game-sessionization-duration` adds deterministic
sessionization and daily rollups over stored app/game SQLite observation rows under
`output/app-game-plan-proof/13-sessionization-and-duration-engine/`. It covers
process-derived running duration, foreground-window duration bounded by running
duration, background duration, stale-gap session closure, process-exit closure,
stable replay independent of ingestion order, session end reasons, observation
gap tracking, and daily rollup totals by date and classification. It does not
prove encrypted journal-file ingest/replay, live process or foreground
subscriptions, service events, portal app/game dashboard rows, policy
execution, UI screenshots, live launcher crawling, or broad blocking.

WP14 proof on `codex/app-game-journal-sqlite-ingest` adds staged encrypted
journal-file append/replay plus SQLite projection proof under
`output/app-game-plan-proof/14-journal-and-sqlite-ingest/`. It covers typed
inventory, runtime, foreground, and launcher rows; local-journal custody labels;
SQLite replay into inventory, running-now, foreground-now, launcher, and daily
rollup rows; invalid inventory-use rejection before persistence; and duplicate
runtime observations not double-counting duration. It does not prove live source
adapters, service events, portal app/game dashboard rows, policy execution,
approval flow, journal corruption/recovery, live launcher crawling, or broad
blocking.

WP15 proof on `codex/app-game-read-model-service-events` adds service-backed
app-use and games activity-surface read-model DTO proof under
`output/app-game-plan-proof/15-read-models-and-service-events/`. It covers
TypeScript activity-surface row fields, Rust protocol serialization,
`AppGameServiceReadModel` projection metadata, `agent-core` service read-model
loading, and `agent-service` mapping from replayed inventory, running,
foreground, launcher, and daily rollup rows. It does not prove dedicated portal
dashboard UI, policy/approval read models, live source subscriptions, platform
authority changes, UI screenshots, or broad blocking.

WP17 proof on `codex/app-game-read-model-service-events` adds contract-level
unknown app/game approval flow proof under
`output/app-game-plan-proof/17-unknown-app-game-approval-flow/`. It covers new
inventory apps, unknown runtime processes, portable/installer candidates,
launcher-game candidates, weak unknown game-like executables, child status and
reason references, parent response scope, allow-once expiry, audit-backed
replay state, and manual-required block outcomes. It does not prove live
candidate creation from source adapters, parent/child approval UI,
notification delivery, service persistence/read models, platform adapter hard
blocking, or broad app/game enforcement.

WP18 proof on `codex/app-game-read-model-service-events` adds parent-domain
native game budget policy contract proof under
`output/app-game-plan-proof/18-native-game-budgets-and-launcher-policy/`. It
covers game budget targets, running/foreground duration source selection,
known-game session inclusion, launcher-only exclusion, parent-approved
launcher-game candidate inclusion, rating/UGC/multiplayer/purchase advisory
signal boundaries, dry-run preview decisions, and no adapter handoff. It does
not prove Rust/service parity, budget persistence, portal budget authoring or
preview UI, notifications, bonus-time integration, platform adapter execution,
or broad app/game enforcement.

WP19 proof on `codex/app-game-read-model-service-events` adds parent-domain
app/game policy target compiler contract proof under
`output/app-game-plan-proof/19-policy-target-compiler-for-app-game-rules/`. It
covers specific app/game target identity proof, unknown app/game state proof,
category/risk/multiplayer/UGC/purchase target proof, schedule proof,
capability/authority refs, wrong-device/wrong-local-user/stale evidence
rejection, dry-run-only compiled policy decisions, and manual-required
block-launch without proof. It does not prove Rust/service parity, runtime
evaluator execution, portal authoring or preview UI, notifications, timers,
rollback, platform adapter execution, or broad app/game enforcement.

WP20 proof on `codex/app-game-read-model-service-events` adds parent-domain
app/game time-budget contract proof under
`output/app-game-plan-proof/20-time-budget-schedule-bonus-time-integration/`.
It covers stored app/game session refs, running versus foreground budget modes,
schedule evidence, bonus-time approval/audit refs, ask-parent/manual-required
dry-run states, effective budget math, and restart-recovered timer refs. It
does not prove Rust/service parity, runtime evaluator execution, service
persistence, portal budget authoring or preview UI, notification delivery,
child request UX, adapter execution, or platform timer/rollback execution.

WP21 proof on `codex/app-game-read-model-service-events` adds parent-domain and
text-domain child-facing UX contract proof under
`output/app-game-plan-proof/21-child-facing-warning-and-request-ux/`. It covers
warning, approval-needed, time-limit, request submitted/approved/denied,
manual-required, and unavailable states with safe copy tokens, evidence refs,
child reason/status refs, approval request refs, no private diagnostics, and no
adapter-action overclaim for manual/unavailable states. It does not prove live
child UI, overlay rendering, portal preview screenshots, notification delivery,
service persistence, Rust/WebSocket parity, adapter execution, or platform
shield/block behavior.

WP22 proof on `codex/app-game-read-model-service-events` extends
`scripts/test/v0-8-windows-app-time-limit-adapter-mvp.mjs` and records evidence
under
`output/app-game-plan-proof/22-windows-owned-process-terminate-time-limit-proof/`.
It uses the real Rust service/WebSocket path to prove dry-run no-action, stale
timer action mismatch rejection before adapter execution, timer recovery/cancel
after that rejection, owned/current expiry through the scoped process adapter,
and broad app/game package blocking as an explicit no-claim/manual-required
boundary.

WP23 proof on `codex/app-game-read-model-service-events` adds
`packages/parent-domain/src/app-game-broad-blocking-proof-gates.ts`,
`packages/parent-domain/src/app-game-broad-blocking-proof-gate-data.ts`, and
`packages/parent-domain/src/app-game-broad-blocking-proof-gate-rules.ts`, then
records evidence under
`output/app-game-plan-proof/23-broad-blocking-proof-gates/`. It covers focused
manual-required, unavailable, and not-claimed gates for Windows AppLocker/App
Control block launch, AppLocker audit-only non-enforce proof, macOS hard-block
requirements, Linux mechanism/distro/session requirements, Android normal-mode
hide/suspend requirements, iOS ManagedSettings shield requirements, and iOS
process-kill no-claim state. It proves these gates cannot dispatch adapters and
that supported upgrade candidates need setup, authority-tier, rollback, audit,
and platform-specific proof references. It does not add runtime platform
adapters, service events, portal UI screenshots, rollback execution, or any
broad app/game blocking support claim.

WP24 proof on `codex/app-game-read-model-service-events` adds
`packages/parent-domain/src/app-game-ai-classifier-boundary.ts`,
`packages/parent-domain/src/app-game-ai-classifier-boundary-values.ts`, and
`packages/parent-domain/src/app-game-ai-classifier-boundary-data.ts`, then
records evidence under
`output/app-game-plan-proof/24-ai-classifier-digest-boundary/`. The proof keeps
AI classifier output evidence-only, requires stored evidence refs, bounds
confidence, records model/runtime/prompt and fallback refs, rejects forbidden
action/duration/raw-scan fields before policy handoff, and reruns the existing
activity-domain digest test without editing the codex-a-locked source-spine
package. It does not prove live model quality, provider execution, runtime
service events, portal rendering, policy evaluator consumption, or adapter
enforcement.

WP25 proof on `codex/app-game-read-model-service-events` adds
`packages/parent-domain/src/app-game-platform-extension-routing.ts`,
`packages/parent-domain/src/app-game-platform-extension-routing-rules.ts`, and
split row data under
`packages/parent-domain/src/app-game-platform-extension-routing-*-data.ts`, then
records evidence under
`output/app-game-plan-proof/25-platform-extension-checklist-and-proof-routing/`.
The proof matrix covers all MAC, IOS, ANDROID, and LINUX extension rows with
platform, action scope, authority tier, setup state, manual tags, proof pack
paths, and cross-plan handoff. It rejects duplicate rows, wrong platform
prefixes, generic unsupported labels, and promotion-ready rows missing
authority-tier, permission/setup, rollback, manual-platform, validation, or
proof references. It does not prove live platform adapters, device enrollment,
rollback execution, service events, portal UI, or runtime cross-platform
support.

WP26 proof on `codex/app-game-read-model-service-events` adds
`packages/parent-domain/src/app-game-install-store-handoff.ts`,
`packages/parent-domain/src/app-game-install-store-handoff-rules.ts`,
`packages/parent-domain/src/app-game-install-store-handoff-proof.ts`, and
`scripts/test/app-game-install-store-handoff-proof.mjs`, then records evidence
under `output/app-game-plan-proof/26-install-uninstall-purchase-store-handoffs/`.
The proof matrix covers new app/game inventory, installer/updater process,
store package install, native game purchase signal, uninstall delta, and
tamper/uninstall candidate handoffs. Store and purchase signals stay
context-only, install approval refs must carry evidence, uninstall/tamper rows
route to the enforcement-integrity/tamper docs, and adapter execution plus
policy decisions remain not-claimed. It does not prove live store APIs,
approval UI, platform adapters, billing entitlement logic, uninstall blocking,
or anti-tamper behavior.

WP27 proof on `codex/app-game-read-model-service-events` adds
`packages/parent-domain/src/app-game-performance-health.ts`,
`packages/parent-domain/src/app-game-performance-health-rules.ts`,
`packages/parent-domain/src/app-game-performance-health-proof.ts`, and
`scripts/test/app-game-performance-health-proof.mjs`, then records evidence
under `output/app-game-plan-proof/27-performance-and-service-health/`. The
proof matrix covers inventory scan bounds, runtime polling bounds, foreground
debounce bounds, journal write volume, session replay cost, policy compile
cost, existing dashboard intent row cost, and degraded adapter health state.
Generated smoke covers 1,000 inventory rows, 500 runtime rows, 500 foreground
rows, 10,000 journal records, 100,000 replay observations, 1,000 policy compile
parses, and 500 existing dashboard intent rows. It does not prove live OS
throughput, encrypted journal disk or corruption recovery, browser DOM or
Playwright rendering, live adapters, approval/store behavior, or broad blocking.

WP28 proof on `codex/app-game-read-model-service-events` adds
`scripts/test/app-game-plan-rollout-pr-gate.mjs`, then records evidence under
`output/app-game-plan-proof/28-e2e-manual-proof-rollout-pr-gate/`. The final
gate verifies the app-game WP01-WP27 and app-plan WP01-WP26 proof roots exist,
routes the E2E/manual scenarios to concrete proof roots or explicit
manual-required gaps, writes no-claim/security gates, names platform manual
proof requirements, and records the PR-ready report requirements. It does not
add runtime source crawling, new Rust/service/portal behavior, browser DOM
rendering proof, live approval/notification flows, live classifier/provider
quality, cross-platform adapters, or broad blocking support.

## Current Test Files

- `packages/activity-domain/tests/app-game.test.ts`
- `packages/activity-domain/tests/app-game-evidence-claim.test.ts`
- `packages/activity-domain/tests/app-game-identity.test.ts`
- `packages/activity-domain/tests/app-game-inventory.test.ts`
- `packages/activity-domain/tests/app-game-category-risk.test.ts`
- `packages/activity-domain/tests/app-game-runtime.test.ts`
- `packages/activity-domain/tests/app-game-launcher.test.ts`
- `packages/activity-domain/tests/activity-surface.test.ts`
- `packages/agent-protocol-domain/tests/activity-surface-adapter.test.ts`
- `packages/parent-domain/tests/app-control-policy-catalog.test.ts`
- `packages/parent-domain/tests/app-game-child-facing-ux.test.ts`
- `packages/parent-domain/tests/game-control-policy-catalog.test.ts`
- `packages/parent-domain/tests/app-game-control-authority.test.ts`
- `packages/parent-domain/tests/app-game-unknown-approval-flow.test.ts`
- `packages/parent-domain/tests/app-game-control-platform-authority.test.ts`
- `packages/parent-domain/tests/app-game-policy-target-compiler.test.ts`
- `packages/parent-domain/tests/app-game-time-budget-policy.test.ts`
- `packages/parent-domain/tests/app-game-time-budget-policy-recovery.test.ts`
- `packages/parent-domain/tests/app-game-broad-blocking-proof-gates.test.ts`
- `packages/parent-domain/tests/app-game-ai-classifier-boundary.test.ts`
- `packages/parent-domain/tests/app-game-install-store-handoff.test.ts`
- `packages/parent-domain/tests/app-game-performance-health.test.ts`
- `packages/parent-domain/tests/app-game-platform-extension-routing.test.ts`
- `packages/parent-domain/tests/native-game-budget-policy.test.ts`
- `packages/parent-domain/tests/enforcement-approval-audit.test.ts`
- `crates/agent-protocol/src/app_game_tests.rs`
- `crates/agent-core/src/activity_store_app_game/app_game_sessionization_tests.rs`
- `crates/agent-core/src/activity_store_app_game/app_game_journal_sqlite_ingest_tests.rs`
- `crates/agent-core/src/activity_store_app_game_tests.rs`
- `crates/agent-core/src/activity_store_app_game/app_game_windows_launcher_tests.rs`
- `crates/agent-core/src/enforcement_app_time_limit_tests.rs`
- `apps/portal/tests/live-activity-state.test.ts`
- `apps/portal/tests/live-activity-surface-adapter.test.ts`
- `apps/portal/tests/policy-preview-live-activity-state.test.ts`
- `apps/portal/tests/activity-ui-intent.test.ts`
- `packages/text-domain/tests/app-game-child-ux-text.test.ts`

## Source Truth Rule

When this plan and existing feature/expectation docs differ, update the owning
feature or expectation doc before making implementation claims. Until those
locked docs can be edited, record the bridge gap in
[pasted content coverage audit](pasted-content-coverage-audit.md).

## Doc Reconciliation - 2026-06-02

- `docs/features/app-game-control.md`,
  `docs/expectations/app-game-evidence.md`, and
  `docs/architecture/app-game-evidence-sessions.md` now link this shared
  app/game implementation plan directly.
- `docs/plans/app-plan/README.md` now points shared native app/game evidence and
  native game product-slice work back to this folder while keeping app-only work
  in the native app plan.
- `docs/product-capability-checklist.md` was not changed in this slice because
  no product status or runtime proof changed, and `codex-a` currently owns that
  file lock.
- Browser-game and cloud-gaming work remains routed to browser-plan. This plan
  owns native apps, native games, launchers, process/window/package evidence,
  app/game policy targets, and app/game proof gates.

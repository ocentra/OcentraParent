# Current Native App Snapshot - 2026-06-02

This snapshot records current native app source, proof, UI, and gap state before
the native app plan is split into implementation workpacks.

## Product Claim Boundary

Current docs already require this boundary:

```text
Inventory is not current use.
Running is not foreground.
Foreground is not content.
AI can classify only stored evidence or digests.
Parent policy decides.
Platform adapters enforce only when authority-tier proof exists.
```

The current feature doc says app/game control starts with evidence: inventory,
identity, running time, foreground time, category candidates, and confidence.
Blocking or time limits require typed policy decisions and platform adapter
proof.

## Contracts That Exist

`packages/activity-domain` already defines:

- app/game inventory entries;
- app/game category/risk taxonomy candidates with source refs, confidence,
  evidence refs, parent display overrides, AI digest refs, and no-direct
  enforcement guards;
- process observations;
- foreground evidence rows for active-window focus as staged
  contract/protocol/parser proof;
- session summaries with end reasons, observation gaps, and foreground/background
  duration evidence timestamps;
- daily app/game session rollup contracts;
- session query/result/report shapes;
- AI digest references;
- classification states such as known app, known game, known launcher, possible
  game, unknown process, permission-limited, unsupported platform, stale, and
  adapter error;
- capability, catalog-ready, foreground, observation-mode, launcher-kind, and
  confidence schemas.

`packages/parent-domain` already defines:

- app-control catalog schema/data;
- app-control guide catalog data;
- app-control policy catalog tests;
- enforcement policy dispatch contracts;
- approval/audit tests.

## Feature Routing Snapshot

The owning feature is `docs/features/app-game-control.md`. It owns the app/game
claim boundary and the current checklist for inventory, identity, running and
foreground session evidence, category and unknown-state handling, schedules,
time budgets, ask-parent, bonus-time, adapter capability status, and
blocking/time-limit proof.

Adjacent feature docs reference app control as shared context:

- `app-install-purchase-approval.md` owns full install/purchase product flows.
- `browser-web-control.md` owns browser games and web apps.
- `enforcement-integrity-tamper.md` owns broad enforcement integrity and
  tamper states.
- `evidence-store-query.md` owns common journal/query-store posture.
- `local-ai-safety-evaluator.md` owns local AI/classifier runtime and quality.
- `policy-schedules-approvals.md` owns parent rules, schedules, approvals, and
  bonus-time flows.
- `remote-lan-mobile-platforms.md` owns Android/iOS/mobile platform gaps.
- `production-distribution-support.md` owns signing, store, package, support,
  and redaction proof.
- `social-video-control.md` owns first-class social/video content and account
  semantics beyond native app detection.

This plan folder centralizes native app task planning while those shared feature
docs remain in place.

## Rust Runtime That Exists

`crates/agent-protocol` already has:

- app/game session summary structs;
- app/game session report structs;
- schema version and classification constants.

`crates/agent-core` already has:

- SQLite-backed app/game observation helpers;
- app/game row mapping helpers;
- deterministic session summary/report derivation from stored rows, including
  running duration, foreground duration bounded by running duration, background
  duration, stale-gap closure, process-exit closure, replay-stable ordering, and
  daily rollup helper proof;
- staged encrypted journal-file replay proof for typed app/game inventory,
  runtime, foreground, launcher, running-now, foreground-now, and daily rollup
  rows;
- staged Windows foreground-window parser proof that can apply foreground
  duration to existing runtime summaries without claiming content;
- scoped Windows app time-limit capability helpers;
- owned-process terminate/time-limit target validation;
- app time-limit tests.

`crates/agent-service` currently has a small `app.rs` module and uses the
shared service/runtime paths for read-model exposure. App-use activity-surface
read models now consume the shared app-game service projection so native app
rows can carry inventory, runtime, foreground, rollup, capability, source-count,
and evidence-ref state from staged journal/SQLite replay.

## Portal That Exists

`apps/portal` already renders service-backed live activity and policy-preview
state. The App/Game Sessions route now has a dedicated service-backed dashboard
for app-use and games read-model rows, including separate native app inventory,
running, foreground, unknown/risk/manual-required capability, duration, and
evidence counts. It is not yet a finished native app product flow.

Existing portal/source areas that app work should extend:

- live activity state/panel;
- activity timeline;
- policy preview panel/read model/details;
- capability guidance;
- device rule scope;
- portal layout surface/content panels;
- parent navigation app/game icons;
- core-ui activity intent data.

## Proof That Exists

The native app plan now mirrors the shared app/game proof spine for WP01-WP16:

```text
output/app-plan-proof/01-contract-boundary-and-effect-schemas
output/app-plan-proof/02-source-index-and-doc-reconciliation
output/app-plan-proof/03-current-app-snapshot-and-gap-map
output/app-plan-proof/04-app-identity-model
output/app-plan-proof/05-installed-app-inventory-model
output/app-plan-proof/06-windows-installed-app-inventory-adapter
output/app-plan-proof/07-windows-store-uwp-appx-inventory-adapter
output/app-plan-proof/08-windows-process-runtime-evidence-adapter
output/app-plan-proof/09-windows-foreground-app-evidence-adapter
output/app-plan-proof/10-cross-platform-authority-matrix
output/app-plan-proof/11-app-category-and-risk-taxonomy
output/app-plan-proof/12-app-sessionization-and-duration-engine
output/app-plan-proof/13-journal-and-sqlite-app-ingest
output/app-plan-proof/14-app-read-models-and-service-events
output/app-plan-proof/15-parent-portal-app-inventory-running-session-surfaces
output/app-plan-proof/16-new-app-and-unknown-app-approval-flow
output/app-plan-proof/18-policy-target-compiler-for-app-rules
output/app-plan-proof/19-time-budget-schedule-bonus-time-integration
output/app-plan-proof/20-child-facing-app-warning-block-request-ux
```

Those proof packs point back to `output/app-game-plan-proof/*` for the shared
contract, parser, and docs evidence. They prove staged foundations only; they do
not move product status for live app capture, policy, enforcement, install
approval, or cross-platform parity.

Current focused proof scripts include:

```text
node scripts/test/v0-8-windows-app-time-limit-adapter-mvp.mjs
node scripts/test/v0-8-enforcement-policy-dispatch-proof.mjs
node scripts/test/v0-8-enforcement-product-control-spine.mjs
node scripts/test/v0-8-enforcement-integrity-runtime-audit.mjs
node scripts/test/v0-8-cross-platform-enforcement-capability-proof.mjs
node scripts/test/v0-8-enforcement-timer-recovery-mvp.mjs
```

These are scoped proof harnesses. They do not prove broad app blocking,
polished app catalog UI, install approval, or cross-platform parity.

## Current Gaps

- App-only identity and inventory are not product-complete.
- The current contracts are app/game combined. The app plan may narrow app-only
  vocabulary, but it must reconcile with current `AppGame*` contracts instead
  of creating duplicate truth.
- Windows installed-app inventory needs stronger source-specific proof.
- Windows Store/UWP/AppX identity needs separate proof from Win32 executable
  identity.
- Foreground evidence now has shared app/game contract/protocol/parser proof,
  stored-row sessionization can derive foreground duration, staged
  journal/SQLite replay can project foreground-now rows, service app-use read
  models can expose foreground state, and the portal App/Game Sessions
  dashboard labels foreground separately from inventory/running/content claims,
  but live window capture wiring remains incomplete.
- Session duration and daily app rollups now have deterministic SQLite-row
  replay proof plus staged encrypted journal-file ingest/replay proof, service
  app-use read models can expose daily rollup counts/duration, and the portal
  dashboard shows duration/counts from those read models, but policy/report
  integration, live source subscriptions, and journal corruption/recovery proof
  remain incomplete.
- New/unknown app approval now has contract-level candidate, child-status,
  response-scope, expiry, replay, and manual-required proof, but live native
  candidate production, notification delivery, service persistence/read models,
  parent/child approval UI, and platform hard blocking remain incomplete.
- App policy target compiler now has shared parent-domain contract proof for
  app targets, identity/unknown/category/schedule/capability/authority proof,
  device/local-user/freshness rejection, dry-run-only decisions, and
  manual-required unproved block-launch. It does not yet provide runtime
  evaluator execution, service persistence, portal authoring/preview UI, timers,
  notifications, rollback, or adapter execution.
- Native app time-budget integration now has shared parent-domain contract proof
  for stored session refs, running versus foreground duration modes, schedule
  evidence, bonus-time approval/audit refs, ask-parent/manual-required dry-run
  states, effective budget math, and restart-recovered timer refs. It does not
  yet provide live native app runtime evaluation, service persistence, portal
  budget authoring/preview UI, notification delivery, child request UX, adapter
  execution, or broad installed-app blocking. WP17 risk app detection remains
  open because `packages/activity-domain` was locked by `codex-a` during this
  pass.
- Native app child-facing warning/request UX now has shared parent-domain and
  text-domain contract proof for warning, approval-needed, time-limit, request
  submitted/approved/denied, manual-required, and unavailable states. It does
  not yet provide live native app child UI, portal preview screenshots, native
  overlay rendering, notification delivery, service persistence,
  Rust/WebSocket parity, adapter execution, or broad installed-app blocking.
- Native app owned-process time-limit proof now cross-records the shared
  app/game real-service proof for dry-run no-action, stale action mismatch
  rejection before adapter execution, timer recovery/cancel, and scoped
  owned/current expiry. It is still not broad app/package blocking.
- App category/risk taxonomy now has contract/test proof, but live classifier
  enrichment, policy routing, portal category rows, and runtime app risk
  detection remain incomplete.
- Parent app catalog/dashboard has an initial service-backed App/Game Sessions
  surface, but it is not product-complete.
- Broad app blocking remains manual-required outside scoped owned-process
  proof.
- Platform-specific authority tiers are now modeled as shared app/game
  parent-domain contract proof, but the portal UI and runtime adapter proof are
  not complete.
- macOS, Linux, Android, iOS, MDM, device-owner, supervised, Endpoint Security,
  AppLocker/App Control, Screen Time, ManagedSettings, cgroups/systemd,
  AppArmor/SELinux, Flatpak, Snap, signing, store, and entitlement claims need
  separate proof before product claims.

## Where We Want To Be

The native app subsystem should become a service-backed product flow from code
to UI:

```text
app inventory
-> app identity
-> runtime and foreground evidence
-> session and duration summaries
-> encrypted journal
-> SQLite read model
-> policy/AI evidence refs
-> authority-tier capability check
-> portal status and actions
-> proof artifacts and manual-required gaps
```

Every visible parent claim should answer:

- What app/source produced this?
- Is the app installed, running, foreground, stale, unknown, or unsupported?
- What identity fields back the row?
- How fresh is the evidence?
- Which authority tier is active?
- What can Ocentra do now?
- Which actions are manual-required?
- What proof artifact backs the claim?

## Enhancement Rule

Future app work should enhance the existing code layout:

- add missing contracts to existing domain packages first;
- split app-only from app/game only when the ownership boundary is real and
  test-backed;
- mirror protocol-facing contracts in the existing Rust protocol crate;
- extend current `agent-core` app/game store and enforcement helpers instead of
  replacing them;
- wire through current service read-model and policy paths;
- render through current portal live-activity, policy-preview, layout, and
  capability surfaces;
- reuse existing proof scripts where they cover a claim;
- add new proof only where current scripts cannot cover the claim.

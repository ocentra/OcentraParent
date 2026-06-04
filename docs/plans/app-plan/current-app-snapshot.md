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
time budgets, ask parent, bonus-time, adapter capability status, and
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
- app/game evidence claim, AI digest reference/classification digest, identity,
  and identity-merge structs mirrored from `packages/activity-domain`;
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
- staged encrypted journal-file replay plus SQLite projection proof for shared
  app/game evidence claim, identity, approval authority/action result, platform
  authority matrix, and classifier result protocol rows;
- service app-use/games evidence vectors that now preserve refs for those
  staged evidence claim, identity, approval authority/action result, platform
  authority matrix, and classifier result rows;
- staged Windows foreground-window parser proof that can apply foreground
  duration to existing runtime summaries without claiming content;
- core live foreground-window source proof that maps active-window metadata into
  foreground rows and journal events with opaque window/title refs without
  title/content capture;
- scoped Windows app time-limit capability helpers;
- owned-process terminate/time-limit target validation;
- app time-limit tests.

`crates/agent-service` currently has a small `app.rs` module and uses the
shared service/runtime paths for read-model exposure. App-use activity-surface
read models now consume the shared app-game service projection so native app
rows can carry inventory, runtime, foreground, rollup, capability, source-count,
and evidence-ref state from staged journal/SQLite replay. The service
activity-capture startup path now repeats bounded live process capture on a
protocol-owned cadence so native app/game runtime rows stay fresh in that same
journal/store/read-model path without foreground, policy, or adapter claims.
Those app-use/games read-model rows also carry staged authority/classifier
storage refs through their existing evidence vectors, without adding live
classifier execution, policy consumption, dedicated portal rows, or adapter
execution.
The core active-window foreground source separately proves foreground evidence
row production with opaque window/title refs, and the service capture bridge can
now append optional foreground rows when that source is available. Portal
freshness rows, policy consumption, and adapter execution are not wired yet.

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

The native app plan now mirrors the shared app/game proof spine for completed
and cross-recorded workpacks:

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
output/app-plan-proof/23-app-ai-classifier-digest-boundary
output/app-plan-proof/24-platform-extension-checklist-and-proof-routing
output/app-plan-proof/25-install-and-uninstall-approval-handoff
output/app-plan-proof/26-performance-and-service-health
output/app-plan-proof/27-e2e-and-manual-proof-artifacts
output/app-plan-proof/28-rollout-checklist-and-pr-gate
output/app-plan-proof/29-rust-protocol-evidence-identity-parity
output/app-plan-proof/30-rust-protocol-authority-classifier-parity
output/app-plan-proof/31-journal-sqlite-authority-classifier-storage
output/app-plan-proof/32-live-process-snapshot-source
output/app-plan-proof/33-live-process-journal-sqlite-bridge
output/app-plan-proof/34-service-capture-app-game-live-process-bridge
output/app-plan-proof/35-service-app-game-recurring-freshness
output/app-plan-proof/36-live-foreground-window-source
output/app-plan-proof/37-service-foreground-capture-bridge
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
node scripts/test/app-game-broad-blocking-proof-gates.mjs
node scripts/test/app-game-platform-extension-routing-proof.mjs
node scripts/test/app-game-install-store-handoff-proof.mjs
node scripts/test/app-game-performance-health-proof.mjs
node scripts/test/app-game-plan-rollout-pr-gate.mjs
```

These are scoped proof harnesses. They do not prove broad app blocking support,
polished app catalog UI, install approval, or cross-platform parity.

## Current Gaps

- App-only identity and inventory are not product-complete.
- The current contracts are app/game combined. The app plan may narrow app-only
  vocabulary, but it must reconcile with current `AppGame*` contracts instead
  of creating duplicate truth.
- Shared app/game evidence claim, AI digest reference/classification digest,
  identity, and identity-merge shapes now have Rust protocol parity. Runtime
  identity merge behavior, live adapter identity refs, and portal identity rows
  remain incomplete; staged journal/SQLite projection now stores evidence claim
  and identity rows without claiming live adapters.
- Shared app/game approval authority/action-result, platform authority matrix,
  and classifier result boundary shapes now have Rust protocol parity and
  staged journal/SQLite projection. Existing service app-use/games read-model
  evidence vectors now preserve refs for those staged rows. Runtime classifier
  execution, dedicated service event exposure, policy evaluator consumption,
  portal authority/classifier rows, and adapter execution remain incomplete.
- Windows installed-app inventory needs stronger source-specific proof.
- Windows Store/UWP/AppX identity needs separate proof from Win32 executable
  identity.
- Foreground evidence now has shared app/game contract/protocol/parser proof,
  stored-row sessionization can derive foreground duration, core live
  active-window source proof can emit foreground rows with opaque window/title
  refs, journal/SQLite replay can project foreground-now rows, service app-use
  read models can expose foreground state, and the portal App/Game Sessions
  dashboard labels foreground separately from inventory/running/content claims.
  Bounded service capture can now append optional foreground rows when the
  active-window source is available; dedicated portal foreground source rows and
  subscribed foreground transitions remain incomplete.
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
  evidence, bonus-time approval/audit refs, ask parent/manual-required dry-run
  states, effective budget math, and restart-recovered timer refs. It does not
  yet provide live native app runtime evaluation, service persistence, portal
  budget authoring/preview UI, notification delivery, child request UX, adapter
  execution, or broad installed-app blocking.
- Native app risk detection now has parent-domain contract proof for known
  VPN/proxy, remote desktop, torrent/download, AI chatbot, unknown
  name/publisher/hash, local AI digest, and parent display override candidates.
  Risk candidates carry confidence/source disclosure, no-content claims,
  no-direct-enforcement guards, and risk-app category-proof routing. It does not
  yet provide live OS scanning, live catalog enrichment, service events, portal
  evidence drawer UI, local model quality, or platform enforcement proof.
- Native app AI classifier boundary proof now cross-records the shared app/game
  classifier contract: stored evidence refs, confidence bounds,
  runtime/model/prompt refs, fallback state, and evidence-only policy handoff
  are required, while direct action, duration, and raw scan fields are rejected
  before policy can consume classifier output. Rust protocol parity now mirrors
  that boundary for serialization proof only, and service read-model evidence
  vectors now preserve staged classifier refs. Live provider execution,
  dedicated service events, portal rendering, policy evaluator consumption, and
  adapter enforcement remain gaps.
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
- Native app broad-blocking proof gates now cross-record the shared app/game
  contract proof that manual-required, unavailable, and not-claimed
  block-launch/hide/suspend/shield/process-kill states cannot dispatch adapters
  and must name setup, authority-tier, rollback, audit, and platform proof
  before moving up. It is not AppLocker/App Control, MDM, Endpoint Security,
  Device Owner/Profile Owner, FamilyControls/ManagedSettings, cgroup/systemd,
  or rollback execution proof.
- Native app platform-extension routing now cross-records the shared app/game
  WP25 matrix for every MAC, IOS, ANDROID, and LINUX extension row. It maps
  authority tier, setup state, manual tags, proof packs, and cross-plan handoff,
  but keeps every row manual-required or not-claimed until real platform proof
  exists.
- Native app install/uninstall approval handoff now cross-records the shared
  app/game WP26 matrix for new inventory, installer/updater, store package
  install, purchase signal, uninstall, and tamper candidate rows. Store and
  purchase signals remain context-only, approval refs cite evidence, and
  uninstall/tamper rows route to the tamper feature without adapter or policy
  claims.
- Native app performance-health proof now cross-records the shared app/game
  WP27 matrix for generated inventory, runtime, foreground, journal, replay,
  policy compile, existing dashboard intent, and degraded adapter health
  budgets. It is generated-scale and intent-level proof only; live OS
  throughput, encrypted journal disk/corruption, browser DOM/Playwright render,
  live adapters, install/store approval, and broad blocking remain gaps.
- Native app final rollout/evidence gate proof now cross-records app-plan WP27
  and WP28 from shared app/game WP28 by validating prior proof roots, recording
  E2E/manual scenario routing, no-claim gates, manual-platform proof
  requirements, and PR-ready reporting requirements. It is review-gate proof
  only and does not add live runtime or platform support.
- App category/risk taxonomy, native app risk detection, and native app AI
  classifier boundary now have contract/test proof, but live classifier
  enrichment, portal category/risk/classifier rows, runtime app risk detection,
  local model quality, and platform enforcement remain incomplete.
- Parent app catalog/dashboard has an initial service-backed App/Game Sessions
  surface, but it is not product-complete.
- Broad app blocking remains manual-required outside scoped owned-process proof
  and the focused no-claim/manual-required broad-blocking gate contract.
- Platform-specific authority tiers are now modeled as shared app/game
  parent-domain contract proof and Rust protocol serialization proof, but the
  portal UI and runtime adapter proof are not complete.
- macOS, Linux, Android, iOS, MDM, device-owner, supervised, Endpoint Security,
  AppLocker/App Control, Screen Time, ManagedSettings, cgroups/systemd,
  AppArmor/SELinux, Flatpak, Snap, signing, store, and entitlement claims need
  separate proof before product claims.
- Live store integration, install/purchase approval UI, package-manager/store
  interception, billing entitlement logic, uninstall blocking, and anti-tamper
  behavior remain unproved.
- A real `sysinfo` process snapshot source now feeds native app/game runtime
  record shape in core with opaque executable-path refs and runtime-only
  classification. It now replays through the encrypted journal and SQLite
  read-model path in core and through the service activity-capture journal/store
  path for recurring bounded runtime rows. A real active-window source now feeds
  native app/game foreground record shape in core with opaque window/title refs
  and no content capture, and service capture can append optional foreground
  rows. Staged evidence claim, identity, approval authority/action-result,
  platform authority matrix/rows, and classifier result rows now flow into the
  existing app-use/games service payloads as evidence refs and explicit counts,
  and a dedicated backend app/game boundary read-model event now exposes those
  staged counts and citation refs. Core live Windows shortcut inventory source
  proof now maps bounded Start Menu shortcut scans into inventory-only journal
  rows with hashed source refs, and service activity capture can append those
  inventory-only journal events into the existing encrypted journal/store/read
  model path. Portal source freshness polish, policy evaluation, richer
  process/foreground subscriptions, registry/Store inventory,
  classifier/provider execution, and adapter execution remain unproved.

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

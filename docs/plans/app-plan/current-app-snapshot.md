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
- process observations;
- foreground evidence rows for active-window focus as staged
  contract/protocol/parser proof;
- session summaries;
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
- session summary/report derivation from stored rows;
- staged Windows foreground-window parser proof that can apply foreground
  duration to existing runtime summaries without claiming content;
- scoped Windows app time-limit capability helpers;
- owned-process terminate/time-limit target validation;
- app time-limit tests.

`crates/agent-service` currently has a small `app.rs` module and uses the
shared service/runtime paths for read-model exposure.

## Portal That Exists

`apps/portal` already renders service-backed live activity and policy-preview
state. There is no finished dedicated native app dashboard yet.

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
  but still needs live window capture wiring, journal/SQLite replay, service
  events, portal rows, and explicit no-content UI labels.
- Session duration and daily app rollups need stronger replay proof.
- New/unknown app approval and child request UX are incomplete.
- Risk-app categories need source/confidence and no-content no-claim guards.
- Parent app catalog/dashboard is not product-complete.
- Broad app blocking remains manual-required outside scoped owned-process
  proof.
- Platform-specific authority tiers are not fully modeled in contracts or UI.
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

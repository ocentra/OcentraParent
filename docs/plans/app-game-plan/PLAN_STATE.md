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

## Scope

This folder is the shared native app and native game control plan. It exists because apps and games share the low-level evidence spine, but they do not share product meaning.

## Current ownership interpretation

```text
crates/schema or the owning Rust crate:
  Canonical shared app/game contracts when app/game shapes cross package, crate, app, or plan boundaries.

schema-domain:
  Temporary generated-validation or edge-decoder surface only where TypeScript still needs one during migration.

app-game-domain:
  TypeScript helper/projection and focused validation surface. It consumes Rust-owned/generated contracts and must not become a broad aggregator for policy, enforcement, notification, portal, or production runtime behavior.

app-game-core:
  Child-local Rust app/game observation, sessionization, evidence-event, AI-request, policy-request, and source-readiness runtime boundary.

agent-protocol and agent-service:
  Wire/service transport and read-model boundaries when selected. They are not default owners for every app/game contract.

portal-domain and apps/portal:
  Parent-visible app/game status, dashboard, action-result, and source-readiness projections. They do not observe OS state, classify apps, run timers, or enforce.

AI plan:
  Consumes stored app/game evidence or structured digest refs. AI does not scan the machine or decide enforcement.

Policy/enforcement plans:
  Consume source-ready app/game evidence and parent rules. They own deterministic decisions/actions; app/game owns source truth and handoff readiness.

Notification, child-runtime, LAN, remote, account, data-custody, and setup plans:
  Handoff consumers or adjacent owners only. They must not re-own app/game evidence, source freshness, or adapter authority.
```

## Current coupling risks

```text
- `app-game-domain` currently depends on several sibling domains. Treat those dependencies as migration-sensitive unless they are only approved public helper/contract consumption. Shared shapes should move through `crates/schema` or another neutral Rust-owned boundary.
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

- No concise existing/proved bullet section was detected in the current snapshot.

## Open gaps / missing product runtime

- App/game identity contracts are present, but runtime identity merge behavior and adapter-fed identity refs are not implemented yet.
- Inventory evidence row contracts and Rust inventory-row parity are present, Windows installed-record plus Store/UWP package parser proof exists, staged journal/SQLite replay proof now projects inventory rows, and service activity-surface read models now expose typed inventory state, but live platform crawling and dedicated portal dashboard rows are not implemented yet.
- Runtime evidence contracts and Rust runtime-row parity are present, a staged Windows process runtime parser proof exists, staged journal/SQLite replay proof now projects running-now rows, and service activity-surface read models now expose typed runtime state. Bounded live process capture now refreshes that same service path, but executable metadata crawling, richer subscriptions, and dedicated portal runtime rows are not implemented yet.
- Foreground evidence contracts and Rust foreground-row parity are present, a staged Windows foreground-window parser proof exists, core live active-window source proof now emits foreground rows and journal events with opaque window/title refs, journal/SQLite replay now projects foreground-now rows, the bounded service capture bridge can append optional foreground rows, and service activity-surface read models expose typed foreground state. Dedicated portal foreground rows, subscribed foreground transitions, and content-aware claims are not implemented.
- Rust protocol parity now mirrors the WP01 evidence claim, AI digest reference/classification digest, WP04 identity/identity-merge shapes, the app/game control authority/action-result schemas, the platform authority matrix, and the WP24 parent-domain classifier boundary. WP31 adds staged journal/SQLite storage and read-model projection for evidence claim, identity, authority, action-result, platform authority matrix, and classifier result rows. WP38 carries those staged row refs through existing app-use/games service read-model evidence vectors. Live source subscriptions, classifier execution, dedicated classifier service events, policy runtime, portal authority/classifier rows, and adapter execution remain incomplete.
- Journal and SQLite ingest now covers staged app/game inventory, runtime, foreground, launcher, daily rollup, evidence-claim, identity, approval authority, approval action-result, platform authority matrix, and AI classifier result rows. The service still maps only the established app-use and games activity-surface rows, but those rows now retain staged authority/classifier storage refs in their evidence vectors. The new authority/classifier rows are not yet wired to live source subscriptions, dedicated service events, policy consumers, or portal dashboard rows.
- Portal App/Game Sessions dashboard rows now consume the app-use and games activity-surface DTOs through a shared dashboard intent, but approval, policy, game-budget, live source, and platform-authority surfaces remain incomplete.
- Unknown approval flow now has parent-domain contract proof for weak app/game candidates, child refs, response scopes, expiry, replay state, and manual-required blocks, but live candidate production, notification delivery, service read models, and parent/child approval UI remain incomplete.
- Native game budget policy now has parent-domain contract proof for game budget targets, known-game inclusion, launcher-only exclusion, parent-approved launcher-game candidate inclusion, advisory rating/UGC, multiplayer, and purchase signals, and dry-run-only outcomes. It does not yet provide policy compiler integration, service persistence, portal budget authoring/preview UI, bonus-time integration, notifications, or adapter execution.
- App/game policy target compiler now has parent-domain contract proof for app/game targets, identity/unknown/category/schedule/capability/authority proof, device/local-user/freshness rejection, dry-run-only decisions, and manual-required unproved block-launch. It does not yet provide Rust/service parity, runtime evaluator execution, portal rule authoring/preview UI, timer integration, notifications, rollback, or adapter execution.
- App/game time-budget policy now has parent-domain contract proof for stored app/game session refs, running versus foreground duration modes, schedule evidence, bonus-time approval/audit refs, ask-parent/manual-required dry-run states, effective budget math, and restart-recovered timer refs. It does not yet provide Rust/service parity, runtime evaluator execution, service persistence, portal budget authoring/preview UI, notification delivery, child request UX, adapter execution, or platform timer/rollback execution.
- Child-facing app/game UX now has parent-domain/text-domain contract proof for respectful warning, approval-needed, time-limit, request submitted/approved/ denied, manual-required, and unavailable states with safe copy tokens, evidence refs, child reason/status refs, and no private diagnostics. It does not yet provide live child UI, native overlay rendering, portal preview screenshots, notification delivery, service persistence, Rust/WebSocket parity, adapter execution, or platform shield/block behavior.

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

- Workpacks indexed: 214.
- Workpacks with open checkboxes: 0.
- Workpacks with all detected boxes checked: 54.
- Workpacks with no checkbox status: 160.

### Active/open workpacks

- App control capability guide, schema proposal, and settings inventory are open reference/control-routing workpacks.
- Game control capability guide, schema proposal, and settings inventory are open reference/control-routing workpacks.
- These open rows are not implementation completion claims. Use `WORKPACK_INDEX.md` to choose the exact assigned row and avoid reading the large inventories by default.

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

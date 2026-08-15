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
- **151** have current production source plus expected checked-in tests for their
  bounded scope; **19** are reviewed no-code coordination/proof/reference
  packets; **50** retain a concrete production-code or expected-test gap.
- The authoritative row-by-row source/test result is
  [CODE_AUDIT.md](CODE_AUDIT.md). It overrides historical checkbox, branch,
  removed-package, and ignored-proof wording for Phase 1 status.
- This audit changes ownership/status routing only. It does not claim Phase 2
  focused tests/Enforcer, Phase 3 proof, or release readiness.

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
- These do not erase the 50 Phase 1 gaps or claim cross-platform/provider/child
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

- Workpacks indexed: 220.
- Workpacks with open checkboxes: 0.
- Workpacks with all detected boxes checked: 54.
- Workpacks with no checkbox status: 160.

### Active/open workpacks

- The six app/game capability, schema, and settings guides are reviewed
  no-code reference/control-routing packets. They do not claim product
  implementation or proof completion.
- Fifty implementation/test-writing gaps remain; select them through
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

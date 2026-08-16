# Current Native App Snapshot

Date: 2026-08-15
Code-first detail: [CODE_AUDIT.md](CODE_AUDIT.md)

## Phase 1 totals

- 95/95 graph-imported workpacks reviewed and mapped.
- 80/95 have no remaining source/test-writing gap in their bounded scope.
- 15/95 retain a concrete code or expected-test gap:
  `WP15`, `WP16`, `WP17`, `WP18`, `WP19`, `WP20`, `WP26`, `WP48`,
  `WP49`, `WP61`, `WP62`, `WP63`, `WP64`,
  `WP65`, and `WP102`.
- Focused tests and Enforcer were not rerun for this inventory; proof was not
  regenerated. This is Phase 1 source/test-writing truth only.

## What is real now

- Rust app-only runtime decision/event boundary with negative contract tests.
- Shared Rust app/game evidence, identity, inventory, sessions, authority,
  classifier, policy-readiness, notification-readiness, timer, adapter, and
  parent-surface DTOs.
- Real Windows Start Menu, Store package, registry, process, and active-window
  acquisition code.
- Service startup recurring capture wired to those Windows sources.
- Encrypted journal plus ActivityStore SQLite validation, replay/projection,
  sessions/rollups, source status, service read-model and event paths.
- Scoped Windows owned-process time-limit execution, evidence binding,
  recovery/cancel, result/rollback surfaces, and focused tests.
- Parent-runtime and portal panels for policy readiness, notification
  readiness, platform proof status, adapter dispatch, child transport receipt,
  and timer parent status.
- Tested Rust source-freshness/preview/timer handoff projections through WP101.

## What is not real yet

- Complete installed/running/foreground/session parent inventory UI.
- Durable new/unknown-app approval lifecycle and child/parent interaction.
- Live risk detector/catalog enrichment.
- Current native-app policy compiler and category/risk routing implementation.
- One authoritative session + schedule + bonus/allow-once runtime chain.
- Live child warning/request UI and delivery.
- Notification provider/preference preflight and status producers. WP58
  supplies the canonical local-outbox bridge/store lifecycle; WP59 consumes it
  into scheduler rows/JSONL, and WP60 records ordered metadata-only history
  rows. Durable history/query and live retry/quiet-hours/provider delivery are
  still unclaimed.
- Portal source-freshness rendering/polish.
- Required large-scale performance/load harnesses.
- WP102's bounded service-handoff model/test.
- Broad installed-app blocking or non-Windows hard-control adapters.

## Important no-claim boundary

The WP74-WP101 chain proves typed no-claim projections and handoff requirements.
Searches of non-doc call sites show its builders are consumed by their contract
tests and internal row builders, not by agent-service production composition.
Do not describe that chain as live policy/timer service execution.

## Current implementation order

1. WP18/WP49 policy compiler and category/risk routing.
2. WP16/WP17 durable review and live risk candidates.
3. WP19/WP20 authoritative time-budget runtime and child UX.
4. WP61-WP65 notification provider/preference/delivery status.
5. WP15/WP48/WP63 parent inventory and freshness UI.
6. WP26 performance harnesses.
7. WP102 implement or explicitly merge/retire into WP103.

Only after these Phase 1 gaps close should the plan move through focused Phase
2 validation and Phase 3 clean-checkout proof.

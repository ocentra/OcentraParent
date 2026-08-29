# Current Native App Snapshot

Date: 2026-08-29
Code-first detail: [CODE_AUDIT.md](CODE_AUDIT.md)

## Phase 1 totals

- 95/95 graph-imported workpacks reviewed and mapped.
- 81/95 have no remaining source/test-writing gap in their bounded scope.
- 14/95 retain a concrete code, expected-test, owner, or shared-validation gap:
  `WP06`, `WP16`, `WP17`, `WP18`, `WP19`, `WP20`, `WP25`, `WP26`,
  `WP48`, `WP49`, `WP63`, `WP64`, `WP65`, and `WP102`.
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
- A route-gated parent App inventory/session dashboard for summary,
  inventory/running/foreground state, sessions/rollups, capability, and
  evidence details.
- Tested Rust source-freshness/preview/timer handoff projections through WP101.

## What is not real yet

- Complete Windows installed-app adapter metadata/signature/hash and
  permission-limited ownership.
- Durable new/unknown-app approval lifecycle and child/parent interaction.
- Live risk detector/catalog enrichment.
- Current native-app policy compiler and category/risk routing implementation.
- One authoritative session + schedule + bonus/allow-once runtime chain.
- Live child warning/request UI and delivery.
- A modern install/uninstall approval-handoff owner and production caller.
- Notification provider/preference status producers. WP58
  supplies the canonical local-outbox bridge/store lifecycle; WP59 consumes it
  into scheduler rows/JSONL, and WP60 records ordered metadata-only history
  rows. WP61/WP62 provide bounded preflight source/test contracts. Durable
  history/query and live retry/quiet-hours/provider delivery remain unclaimed.
- Portal source-freshness rendering/polish.
- Required large-scale performance/load harnesses.
- WP102 shared WP101/WP103 route validation.
- Broad installed-app blocking or non-Windows hard-control adapters.

## Important no-claim boundary

The WP74-WP101 chain proves typed no-claim projections and handoff requirements.
Searches of non-doc call sites show its builders are consumed by their contract
tests and internal row builders, not by agent-service production composition.
Do not describe that chain as live policy/timer service execution.

## Current implementation order

1. WP06 Windows installed-app adapter authority and metadata completion.
2. WP18/WP49 policy compiler and category/risk routing.
3. WP16/WP17 durable review and live risk candidates.
4. WP25 modern install/uninstall approval-handoff ownership.
5. WP19/WP20 authoritative time-budget runtime and child UX.
6. WP62-WP65 notification preference/delivery status.
7. WP48/WP63 remaining source-freshness UI.
8. WP26 performance harnesses.
9. WP102 validate the shared WP101/WP103 route without recreating the retired
   parent-domain packet.

Only after these Phase 1 gaps close should the plan move through focused Phase
2 validation and Phase 3 clean-checkout proof.

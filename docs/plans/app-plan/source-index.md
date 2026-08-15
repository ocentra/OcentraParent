# Native App Source Index

Current code-first authority: [CODE_AUDIT.md](CODE_AUDIT.md), audited
2026-08-15. Historical workpack paths and proof roots are routing evidence,
not current implementation truth.

## Product routes

- Owning feature: [App and game control](../../features/app-game-control.md)
- Main expectation: [App and game evidence](../../expectations/app-game-evidence.md)
- Architecture: [App and game evidence sessions](../../architecture/app-game-evidence-sessions.md)
- Policy handoff: [Policy schedules and approvals](../../features/policy-schedules-approvals.md)
- Enforcement handoff: [Enforcement integrity and tamper](../../features/enforcement-integrity-tamper.md)
- Install/purchase handoff: [App install and purchase approval](../../expectations/app-install-purchase-approval.md)
- Workpack chooser: [WORKPACK_INDEX.md](WORKPACK_INDEX.md)

## Current implementation owners

| Boundary | Current tracked owners | What they actually own |
| --- | --- | --- |
| App-only runtime boundary | `crates/app-core` | Native-app observation, evidence, AI/policy request, runtime decision, typed IDs, and tests. |
| Shared app/game contracts and projections | `crates/app-game-core` | Source-freshness, preview, timer-readiness, notification parent-intent, and long handoff projection chains plus contract/generated tests. Most are pure models, not service runtime. |
| Wire contracts | `crates/agent-protocol/src/app_game*.rs` | Evidence/identity/session/inventory, authority, adapter, policy/notification/timer and parent-surface DTOs plus protocol tests. |
| Windows acquisition and local projection | `crates/agent-core/src/activity_store_app_game.rs`, `crates/agent-core/src/activity_store_app_game/` | Real Start Menu, Store manifest, registry, process and foreground sources; journal/SQLite projection; sessionization; source-status rows. |
| Scoped Windows time limit | `crates/agent-core/src/enforcement_app_time_limit.rs`, `crates/agent-service/src/enforcement_timer_api/` | Owned-process timer validation, execution, recovery/cancel, state and focused tests. Not broad installed-app blocking. |
| Service capture/read models | `crates/agent-service/src/activity_capture.rs`, `crates/agent-service/src/activity_capture/`, `crates/agent-service/src/activity_api/app_game*.rs` | Recurring Windows capture, persisted activity rows, policy/notification/platform/timer/adapter read models and events. |
| Canonical generated bridge contracts | `crates/schema/src/app_game*.rs`, `packages/schema-domain/src/*app-game*` | Generated TypeScript schemas and Rust generators for source freshness, policy preview/targets, timer readiness and bridge shapes. |
| Parent runtime projection | `crates/parent-runtime-core/src/parent_ui_bridge/*app_game*`, `crates/parent-runtime-core/src/parent_ui_bridge/live_activity/snapshot/app_game.rs` | Converts service read models into parent UI snapshots without re-owning source truth. |
| Portal rendering | `apps/portal/src/AppGame*`, `apps/portal/src/portal-proof-panels-app-game-renderers.tsx` | Policy, notification, platform, adapter, child-receipt and timer status panels. It does not scan the OS, read SQLite, classify apps, schedule timers or enforce. |

## Removed/stale owners

The following directories have no tracked files in the audited checkout and
must not be presented as live owners:

- `packages/activity-domain`
- `packages/parent-domain`
- `packages/agent-protocol-domain`
- `packages/text-domain`

The prior `scripts/test/app-game-*`, `scripts/test/app-risk*`, and related
native-app proof runners are also absent. Workpacks that still cite those paths
must use their mapped current Rust/test roots from the engineering graph and
[CODE_AUDIT.md](CODE_AUDIT.md). Missing proof runners are a Phase 3 harness
issue unless the workpack's expected test itself is absent, as recorded in the
audit.

## Current source-to-product chain

```text
Windows sources
  -> agent-core typed journal events
  -> encrypted journal + ActivityStore SQLite projection
  -> agent-service app-use/game read models and source-status rows
  -> parent-runtime snapshots
  -> portal status panels
```

The first three hops are real for Windows inventory/process/foreground. The
parent product remains incomplete where [CODE_AUDIT.md](CODE_AUDIT.md) records
missing approval/risk/compiler/child-notification/UI composition.

## Ownership rules

- Do not create a replacement `packages/app-domain` or resurrect legacy
  packages without an explicit workpack/architecture decision.
- Cross-boundary schema authority stays Rust-owned/generated; presentation code
  consumes generated shapes.
- App-only meaning stays in this plan. Shared app/game evidence and generic
  handoff models stay in `app-game-plan`/`app-game-core`.
- Inventory is not runtime; runtime is not foreground; foreground is not
  content; AI evidence is not enforcement authority.
- Platform hard-control claims remain manual-required until the owning platform
  and enforcement plans supply real execution, rollback, and proof.

## Validation routes

Phase 1 uses the exact workpack mappings in
`docs/engineering-graph/code-map.json`. Phase 2 selects focused commands from
[TEST_PROOF_EXPECTATIONS.md](TEST_PROOF_EXPECTATIONS.md) after a workpack is
chosen. Phase 3 writes clean-checkout proof under the current durable proof
policy; ignored `output/` artifacts are never status authority.

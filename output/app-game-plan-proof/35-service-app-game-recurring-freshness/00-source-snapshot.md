# Source Snapshot

- Branch: `codex/app-plan-evidence-control-continuation`
- Base head: `5cf8244ceac6a78b3efbf10f92f52a5578a13f30`
- Workpack: WP35 service app/game recurring freshness
- Hub lock: `crates/agent-service/src/activity_capture.rs`,
  `crates/agent-service/src/activity_capture_tests.rs`,
  `crates/agent-service/README.md`,
  `crates/agent-protocol/src/constants/activity_capture.rs`,
  `crates/agent-protocol/src/constants/activity_store.rs`,
  `crates/agent-protocol/README.md`, app/app-game WP35 docs, and proof roots.

Inspected source:

- `crates/agent-service/src/activity_capture.rs`
- `crates/agent-service/src/activity_capture_tests.rs`
- `crates/agent-protocol/src/constants/activity_capture.rs`
- `docs/features/app-game-control.md`
- `docs/plans/app-game-plan/current-app-game-snapshot.md`
- `docs/plans/app-game-plan/implementation-checklist.md`

Before-state gap:

WP34 proved one bounded service capture could append a live app/game runtime row
into the encrypted journal and ActivityStore. It did not keep the service path
fresh after startup.

Change summary:

The service startup capture path now repeats bounded live process capture on a
protocol-owned cadence. Focused proof records two capture cycles and verifies
two app/game runtime rows remain queryable while foreground remains not-claimed.

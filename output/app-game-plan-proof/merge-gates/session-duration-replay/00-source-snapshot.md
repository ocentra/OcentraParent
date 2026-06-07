# App-game session duration replay gate source snapshot

- Branch: codex/app-game-session-duration-replay-gate-proof-split
- Commit: branch-head-validated-by-harness
- Git status: validated-by-explicit-handoff-status-check

Evidence:
- TypeScript session contracts reject impossible running/foreground/background duration totals.
- Rust sessionization sorts replayed rows before deriving session summaries, so replay order does not mutate duration.
- Journal replay into SQLite produces daily rollup duration values from replayed runtime and foreground rows.
- Duplicate runtime journal replay is guarded so duration and session count do not inflate.
- This proof changes no portal UI, adapter dispatch, policy enforcement, or browser-game path.

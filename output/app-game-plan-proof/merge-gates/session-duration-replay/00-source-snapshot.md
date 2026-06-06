# App-game session duration replay gate source snapshot

- Branch: codex/app-game-inventory-display-gate-proof
- Commit: 67e8c18c84a5a7c16168bc8fd7fe6658ac335e5f
- Git status: clean before proof generation

Evidence:
- TypeScript session contracts reject impossible running/foreground/background duration totals.
- Rust sessionization sorts replayed rows before deriving session summaries, so replay order does not mutate duration.
- Journal replay into SQLite produces daily rollup duration values from replayed runtime and foreground rows.
- Duplicate runtime journal replay is guarded so duration and session count do not inflate.
- This proof changes no portal UI, adapter dispatch, policy enforcement, or browser-game path.

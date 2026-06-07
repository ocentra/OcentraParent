# App-game raw executable path UI leak gate source snapshot

- Branch: codex/app-game-raw-executable-path-ui-leak-gate-proof-split
- Commit: branch-head-validated-by-harness
- Git status: validated-by-explicit-handoff-status-check

Evidence:
- Portal app/game dashboard tests feed raw Windows executable-path-like values into app/game rows.
- The dashboard intent output omits those raw paths and the executablePathRef field.
- The SVG dashboard render source displays labels, state, counts, capability, duration, and evidence refs without executable paths.
- This proof adds no fake activity, adapter dispatch, policy execution, package exports, or browser-game path.

# App-game raw executable path UI leak gate source snapshot

- Branch: codex/app-game-inventory-display-gate-proof
- Commit: 960c5aecd1f7bf2bfd9b8b4ff553155ae1644c07
- Git status: M apps/portal/tests/activity-ui-app-game-dashboard-intent.test.ts
 M docs/features/app-game-control.md
 M docs/plans/app-game-plan/implementation-checklist.md
?? scripts/test/app-game-raw-executable-path-ui-leak-gate-proof.mjs

Evidence:
- Portal app/game dashboard tests feed raw Windows executable-path-like values into app/game rows.
- The dashboard intent output omits those raw paths and the executablePathRef field.
- The SVG dashboard render source displays labels, state, counts, capability, duration, and evidence refs without executable paths.
- This proof adds no fake activity, adapter dispatch, policy execution, package exports, or browser-game path.

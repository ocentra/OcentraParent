# App-game running foreground display gate source snapshot

- Branch: codex/app-game-inventory-display-gate-proof
- Commit: d94b62172622051c714b47259a36d645902c57ec
- Git status: M docs/features/app-game-control.md
 M docs/plans/app-game-plan/implementation-checklist.md
?? scripts/test/app-game-running-foreground-display-gate-proof.mjs

Evidence:
- Portal app/game dashboard intent test includes separate Running and Foreground metric totals.
- The test includes running rows where foregroundRowCount remains 0.
- Core dashboard intent maps runningRowCount and foregroundRowCount into separate dashboard fields.
- Core SVG route renders Running and Foreground counts separately.
- Portal route scaffold E2E assertion expects both RUNNING and FOREGROUND text on App/Game Sessions.

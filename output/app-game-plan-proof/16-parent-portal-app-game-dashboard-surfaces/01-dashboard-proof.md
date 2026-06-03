# Dashboard Proof

The App/Game Sessions route now consumes service-backed app-use and games
read-model rows through a shared dashboard intent.

Implementation proof:

- `app-game-dashboard-intent.ts` maps app-use rows and games rows into a single
  dashboard without mutating or inventing service state.
- App rows use `appName`, `launchCount`, `inventoryRowCount`,
  `runningRowCount`, `foregroundRowCount`, `dailyRollupCount`, `totalMs`, and
  `evidence`.
- Game rows use `displayName`, `sessionCount`, `launcherRowCount`,
  `runningRowCount`, `foregroundRowCount`, `dailyRollupCount`, `totalMs`, and
  `evidence`.
- The dashboard emits separate metrics for app rows, game rows, inventory,
  running, foreground, launcher, unknown review, manual-required, evidence refs,
  and game-budget gap.
- `ParentPortalSvgSurface.tsx` renders the dashboard only for the app/game
  manage context and keeps report, LAN pairing, and generic manage surfaces on
  their existing paths.

UI proof:

- The route displays `APP/GAME READ MODEL DASHBOARD`.
- The route displays `SERVICE ROWS`, `CAPABILITY MATRIX`, and
  `EVIDENCE DRAWER`.
- Browser screenshot proof:
  `output/app-game-plan-proof/16-parent-portal-app-game-dashboard-surfaces/app-game-dashboard-route.png`.
- Long or malicious-looking display names are rendered as React text and
  truncated by `truncateTextForWidth` inside fixed-size SVG row cards.

Known remaining gaps:

- Game budgets are shown as a policy proof gap, not implemented policy.
- Unknown approval and risk-candidate flows remain later workpacks.
- Live source crawling, platform authority proof, and broad blocking remain
  later workpacks.

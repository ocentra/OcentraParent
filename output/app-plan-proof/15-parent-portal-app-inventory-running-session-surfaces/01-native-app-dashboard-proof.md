# Native App Dashboard Proof

Native app-use read-model rows now feed the shared App/Game Sessions dashboard
intent and route surface.

Browser screenshot proof:

- `output/app-plan-proof/15-parent-portal-app-inventory-running-session-surfaces/app-game-dashboard-route.png`

Covered native app states:

- Installed/inventory count from `inventoryRowCount`.
- Running count from `runningRowCount`.
- Foreground count from `foregroundRowCount`.
- Duration from `totalMs`.
- Launch count from `launchCount`.
- Unknown/risk/manual-required state from classification, inventory, runtime,
  foreground, and capability fields.
- Evidence refs from the service-backed row `evidence` array.

Boundaries:

- Inventory is not labeled as usage.
- Running is not labeled as foreground.
- Foreground is not labeled as content.
- Portal does not scan the OS, inspect SQLite, classify apps, run timers, or
  call enforcement adapters.
- Policy decisions, unknown approvals, live adapters, platform authority proof,
  and broad blocking remain later app-plan workpacks.

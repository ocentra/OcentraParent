# Validation Log

Validation completed on branch
`codex/app-game-read-model-service-events` after rebasing onto
`origin/main` at `0c4beb4`.

## Commands

```text
cmd /c npm run --workspace @ocentra-parent/portal test -- activity-ui-intent activity-ui-app-game-dashboard-intent
cmd /c npm run --workspace @ocentra-parent/portal test:e2e
cmd /c npm run --silent lint:schema-boundaries
cmd /c npm run --silent lint --workspace @ocentra-parent/portal
cmd /c npm run --silent type-check --workspace @ocentra-parent/portal
cmd /c npm run --silent format:check
git diff --check
```

## Results

```text
Portal focused tests: 12 files passed, 44 tests passed.
Portal E2E: 2 tests passed against the real Rust agent and portal.
Schema boundary/source shape: passed with advisory source-shape warnings only.
Portal lint: passed.
Portal type-check: passed.
Prettier format check: passed.
Whitespace check: passed.
```

## Browser Proof

Temporary proof stack:

```text
OCENTRA_PARENT_AGENT_PORT=4777
OCENTRA_PARENT_PORTAL_PORT=4778
http://127.0.0.1:4778/#/app-game-sessions
```

The route rendered the dedicated native app and game dashboard from the App Use
and Games read models, including inventory/running/foreground/launcher counts,
evidence references, and the game budgets policy proof gap. Synthetic demo
device names were absent on the route.

Screenshot:

```text
output/app-plan-proof/15-parent-portal-app-inventory-running-session-surfaces/app-game-dashboard-route.png
```

The temporary proof stack was stopped after capture.

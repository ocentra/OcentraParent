# Validation Log

Validation completed on branch `codex/app-game-read-model-service-events`.

## Commands

```text
cmd /c npm run --workspace @ocentra-parent/parent-domain test -- app-game-control-authority app-game-unknown-approval-flow
cmd /c npm run --silent lint --workspace @ocentra-parent/parent-domain
cmd /c npm run --silent type-check --workspace @ocentra-parent/parent-domain
cmd /c npm run --silent lint:schema-boundaries
cmd /c npm run --silent format:check
git diff --check
```

## Results

```text
Focused parent-domain tests: 2 files passed, 10 tests passed.
Parent-domain lint: passed.
Parent-domain type-check: passed.
Schema boundary/source shape: passed with existing advisory warnings only.
Prettier format check: passed.
Whitespace check: passed.
```

## Not Run

```text
Portal E2E: not run because no portal UI or route code changed in this slice.
Rust tests: not run because no Rust protocol, service, or core code changed.
Full npm run validate: not run for this contract-only worker slice.
```

## Source Shape Note

The new `app-game-control-approval-flow.ts` file is below the advisory band.
`app-game-control-authority.ts` was trimmed back below 250 lines after adding
the approval-flow bridge. Remaining source-shape warnings are pre-existing
advisory warnings outside this slice.

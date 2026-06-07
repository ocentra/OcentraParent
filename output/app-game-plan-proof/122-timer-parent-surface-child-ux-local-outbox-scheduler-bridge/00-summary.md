# WP122 Timer Parent-Surface Child UX Local Outbox Scheduler Bridge

## Scope

- Added a parent-domain child UX local outbox scheduler bridge that consumes the
  WP121 child UX local outbox read model and writes existing notification
  scheduler JSONL records for deliverable app/game child UX rows.
- Manual-required and unavailable child UX rows remain unscheduled with blocked
  refs.
- The bridge keeps child delivery, provider delivery, receipt ingestion, retry
  execution, quiet-hours runtime, durable outbox storage, parent notification
  UI, adapter dispatch, platform enforcement, broad blocking, raw private source
  rows, and private diagnostics unclaimed.

## Product Docs

- Updated `docs/features/app-game-control.md`.
- Updated `docs/plans/app-game-plan/implementation-checklist.md`.
- Updated `docs/plans/app-game-plan/workpacks/README.md`.
- Added `docs/plans/app-game-plan/workpacks/122-timer-parent-surface-child-ux-local-outbox-scheduler-bridge.md`.
- Did not update `docs/product-capability-checklist.md` because another lane
  owns production-support checklist churn.

## Validation

- PASS `cmd /c npm run test --workspace @ocentra-parent/parent-domain -- app-game-child-facing-ux-local-outbox-scheduler-bridge`.
- PASS `cmd /c npm run build --workspace @ocentra-parent/parent-domain`.
- PASS focused Prettier check for WP122 source, test, docs, proof, and handoff.
- PASS `cmd /c node scripts/check-no-test-doubles.mjs`.
- PASS `cmd /c node scripts/check-source-shape.mjs` with existing advisory warnings only.
- PASS `git diff --check`.
- PASS `cmd /c npm run lanes:guard`.
- PASS `cmd /c npm run hub:guard`.

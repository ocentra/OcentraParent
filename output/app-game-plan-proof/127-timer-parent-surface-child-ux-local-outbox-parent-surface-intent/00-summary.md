# WP127 Timer Parent-Surface Child UX Local Outbox Parent Surface Intent

## Scope

- Added a parent-domain child UX local outbox parent-surface intent read model.
- Combined WP124 child UX provider-status rows with WP126 child UX
  preference-status rows.
- Parent-surface rows preserve scheduler, outbox, provider, preference,
  quiet-hours, drill-in, audit, and manual proof refs.
- The bridge keeps rendered parent UI, parent preference mutation, provider
  delivery, receipt ingestion, credential runtime, child delivery, retry
  execution, quiet-hours runtime, durable outbox storage, adapter dispatch,
  platform enforcement, broad blocking, raw private source rows, and private
  diagnostics unclaimed.

## Product Docs

- Updated `docs/features/app-game-control.md`.
- Updated `docs/plans/app-game-plan/implementation-checklist.md`.
- Updated `docs/plans/app-game-plan/workpacks/README.md`.
- Added `docs/plans/app-game-plan/workpacks/127-timer-parent-surface-child-ux-local-outbox-parent-surface-intent.md`.
- Did not update `docs/product-capability-checklist.md` because another lane
  owns production-support checklist churn.

## Validation

- PASS `cmd /c npm run test --workspace @ocentra-parent/parent-domain -- app-game-child-facing-ux-local-outbox-parent-surface-intent`.
- PASS `cmd /c npm run build --workspace @ocentra-parent/parent-domain`.
- PASS focused Prettier check for WP127 source, test, docs, proof, and handoff.
- PASS `cmd /c node scripts/check-no-test-doubles.mjs`.
- PASS `cmd /c node scripts/check-source-shape.mjs` with existing advisory warnings only.
- PASS `git diff --check`.
- PASS `cmd /c npm run lanes:guard`.
- PASS `cmd /c npm run hub:guard`.

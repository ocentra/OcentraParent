# WP123 Timer Parent-Surface Child UX Local Outbox Provider Preflight

## Scope

- Added a parent-domain child UX local outbox provider-preflight bridge that
  consumes the WP122 child UX local outbox scheduler read model.
- Scheduled child UX rows become provider-adapter-required preflight rows with
  adapter, credential, and smoke-proof requirements.
- Manual-required and unavailable child UX rows remain blocked before provider
  setup.
- The bridge keeps child delivery, provider delivery, receipt ingestion, retry
  execution, quiet-hours runtime, durable outbox storage, parent notification
  UI, adapter dispatch, platform enforcement, broad blocking, raw private source
  rows, and private diagnostics unclaimed.

## Product Docs

- Updated `docs/features/app-game-control.md`.
- Updated `docs/plans/app-game-plan/implementation-checklist.md`.
- Updated `docs/plans/app-game-plan/workpacks/README.md`.
- Added `docs/plans/app-game-plan/workpacks/123-timer-parent-surface-child-ux-local-outbox-provider-preflight.md`.
- Did not update `docs/product-capability-checklist.md` because another lane
  owns production-support checklist churn.

## Validation

- PASS `cmd /c npm run test --workspace @ocentra-parent/parent-domain -- app-game-child-facing-ux-local-outbox-provider-preflight`.
- PASS `cmd /c npm run build --workspace @ocentra-parent/parent-domain`.
- PASS focused Prettier check for WP123 source, test, docs, proof, and handoff.
- PASS `cmd /c node scripts/check-no-test-doubles.mjs`.
- PASS `cmd /c node scripts/check-source-shape.mjs` with existing advisory warnings only.
- PASS `git diff --check`.
- PASS `cmd /c npm run lanes:guard`.
- PASS `cmd /c npm run hub:guard`.

# WP124 Timer Parent-Surface Child UX Local Outbox Provider Status Handoff

## Scope

- Added a parent-domain child UX provider-status handoff that consumes the WP123
  child UX provider-preflight read model.
- Provider-adapter-required and manual-required preflight rows become
  manual-required V0.8 provider-status boundary entries.
- Unavailable preflight rows become unavailable V0.8 provider-status boundary
  entries.
- The bridge keeps child delivery, provider delivery, receipt ingestion,
  credential runtime, retry execution, quiet-hours runtime, durable outbox
  storage, parent notification UI, adapter dispatch, platform enforcement, broad
  blocking, raw private source rows, and private diagnostics unclaimed.

## Product Docs

- Updated `docs/features/app-game-control.md`.
- Updated `docs/plans/app-game-plan/implementation-checklist.md`.
- Updated `docs/plans/app-game-plan/workpacks/README.md`.
- Added `docs/plans/app-game-plan/workpacks/124-timer-parent-surface-child-ux-local-outbox-provider-status-handoff.md`.
- Did not update `docs/product-capability-checklist.md` because another lane
  owns production-support checklist churn.

## Validation

- PASS `cmd /c npm run test --workspace @ocentra-parent/parent-domain -- app-game-child-facing-ux-local-outbox-provider-status-handoff`.
- PASS `cmd /c npm run build --workspace @ocentra-parent/parent-domain`.
- PASS focused Prettier check for WP124 source, test, docs, proof, and handoff.
- PASS `cmd /c node scripts/check-no-test-doubles.mjs`.
- PASS `cmd /c node scripts/check-source-shape.mjs` with existing advisory warnings only.
- PASS `git diff --check`.
- PASS `cmd /c npm run lanes:guard`.
- PASS `cmd /c npm run hub:guard`.

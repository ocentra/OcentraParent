# WP126 Timer Parent-Surface Child UX Local Outbox Preference Status Handoff

## Scope

- Added a parent-domain child UX preference-status handoff that consumes the
  WP125 child UX preference-preflight read model.
- Parent-preference-required and manual-required preflight rows become manual
  setup V3 notification rule/provider/retry status entries.
- Unavailable preflight rows become disabled/not-sent V3 status entries.
- The bridge keeps parent preference UI/mutation, frequency controls,
  quiet-hours runtime, child delivery, provider delivery, receipt ingestion,
  credential runtime, retry execution, durable outbox storage, parent
  notification UI, adapter dispatch, platform enforcement, broad blocking, raw
  private source rows, and private diagnostics unclaimed.

## Product Docs

- Updated `docs/features/app-game-control.md`.
- Updated `docs/plans/app-game-plan/implementation-checklist.md`.
- Updated `docs/plans/app-game-plan/workpacks/README.md`.
- Added `docs/plans/app-game-plan/workpacks/126-timer-parent-surface-child-ux-local-outbox-preference-status-handoff.md`.
- Did not update `docs/product-capability-checklist.md` because another lane
  owns production-support checklist churn.

## Validation

- PASS `cmd /c npm run test --workspace @ocentra-parent/parent-domain -- app-game-child-facing-ux-local-outbox-preference-status-handoff`.
- PASS `cmd /c npm run build --workspace @ocentra-parent/parent-domain`.
- PASS focused Prettier check for WP126 source, test, docs, proof, and handoff.
- PASS `cmd /c node scripts/check-no-test-doubles.mjs`.
- PASS `cmd /c node scripts/check-source-shape.mjs` with existing advisory warnings only.
- PASS `git diff --check`.
- PASS `cmd /c npm run lanes:guard`.
- PASS `cmd /c npm run hub:guard`.

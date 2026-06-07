# WP131 Timer Parent-Surface Child UX Parent Preference Setup Draft

## Scope

- Added a parent-domain parent preference setup draft read model derived from
  child UX local outbox parent-surface intent rows.
- Preserved parent-safe scheduler, outbox, provider-channel,
  parent-preference, quiet-hours, drill-in, and manual-proof refs.
- Separated draft-ready rows from unavailable-visible rows for later parent UI
  work without rendering UI or mutating preferences.
- Added focused tests over the real child UX handoff, local outbox,
  provider-status, preference-status, parent-surface, and setup-draft contract
  chain.

## No-Claim Boundary

- Parent preference UI, frequency controls, parent preference mutation,
  notification rule mutation, provider delivery, receipt ingestion,
  quiet-hours runtime, child delivery, adapter dispatch, platform enforcement,
  durable outbox storage, raw private source rows, raw target values, and
  private diagnostics remain unclaimed.
- `docs/product-capability-checklist.md` was not touched because another lane
  owns production-support checklist churn.

## Validation

See `10-validation-commands.log`.

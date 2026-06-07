# WP130 Timer Parent-Surface Child UX Parent Action Cards

## Scope

- Added parent action rows to the portal-domain timer parent-surface intent.
- Rendered each child UX parent-surface intent record as an App/Game Sessions
  card backed by the service read model event.
- Kept card contents parent-safe: target meaning, readable manual-action and
  preference-setup states, source/artifact refs, drill-in refs, manual-proof
  refs, and explicit no-claim adapter/child-delivery/platform states.

## No-Claim Boundary

- Parent preference mutation, notification rule mutation, frequency controls,
  provider delivery, receipt ingestion, retry workers, quiet-hours runtime,
  child delivery, adapter dispatch, platform enforcement, durable outbox
  storage, raw private source rows, raw target values, and private diagnostics
  remain unclaimed.
- `docs/product-capability-checklist.md` was not touched because another lane
  owns production-support checklist churn.

## Validation

See `10-validation-commands.log`.

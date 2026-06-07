# WP129 Timer Parent-Surface Child UX Local Outbox Parent Surface Live Records

## Scope

- Added structured child UX local outbox parent-surface intent records to the
  shared timer parent-surface read model.
- Mirrored the records in Rust protocol types and service payload construction.
- Derived parent-surface intent records from replayed action-result rows that
  already carry child reason/status refs, local artifact refs, provider status,
  and preference status.
- Rendered source result ids, parent-surface intent refs, artifact refs, target
  domains, drill-in refs, and manual-proof refs in the App/Game Sessions timer
  parent-surface summary.

## No-Claim Boundary

- Parent notification/preference UI is not rendered.
- Preference mutation, frequency controls, provider delivery, receipt ingestion,
  retry workers, quiet-hours runtime, child delivery, adapter dispatch, platform
  enforcement, durable outbox storage, raw private source rows, raw target
  values, and private diagnostics remain unclaimed.
- `docs/product-capability-checklist.md` was not touched because another lane
  owns production-support checklist churn.

## Validation

See `10-validation-commands.log`.

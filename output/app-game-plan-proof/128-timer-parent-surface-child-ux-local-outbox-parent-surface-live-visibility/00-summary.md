# WP128 Timer Parent-Surface Child UX Local Outbox Parent Surface Live Visibility

## Scope

- Added child UX local outbox parent-surface intent counts and reference ids to
  the shared timer parent-surface read model.
- Mirrored the fields in Rust protocol types and service payload construction.
- Derived parent-surface intent aggregates from replayed action-result rows that
  already carry child reason/status refs and local artifact readiness.
- Rendered the fields in the App/Game Sessions timer parent-surface summary.

## No-Claim Boundary

- Parent notification/preference UI is not rendered.
- Preference mutation, frequency controls, provider delivery, receipt ingestion,
  retry workers, quiet-hours runtime, child delivery, adapter dispatch, platform
  enforcement, durable outbox storage, raw private source rows, and private
  diagnostics remain unclaimed.
- `docs/product-capability-checklist.md` was not touched because another lane owns
  production-support checklist churn.

## Validation

See `10-validation-commands.log`.

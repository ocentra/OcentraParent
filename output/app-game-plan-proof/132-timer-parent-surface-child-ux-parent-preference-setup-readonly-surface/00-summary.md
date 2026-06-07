# WP132 Timer Parent-Surface Child UX Parent Preference Setup Read-Only Surface

## Scope

- Exported the parent-domain child UX parent preference setup draft contract.
- Added read-only parent preference setup rows to the portal-domain timer
  parent-surface intent.
- Rendered setup rows on the App/Game Sessions route as separate cards backed
  by the live service parent-surface event.
- Extended the focused portal test to prove setup cards are service-backed and
  absent service input remains explicit.

## No-Claim Boundary

- Parent preference UI controls, frequency controls, parent preference
  mutation, notification rule mutation, provider delivery, receipt ingestion,
  quiet-hours runtime, child delivery, adapter dispatch, platform enforcement,
  durable outbox storage, raw private source rows, raw target values, and
  private diagnostics remain unclaimed.
- `docs/product-capability-checklist.md` was not touched because another lane
  owns production-support checklist churn.

## Validation

See `10-validation-commands.log`.

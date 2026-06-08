# WP133 Timer Parent-Surface Child UX Parent Preference Setup Service Records

## Scope

- Added dedicated parent preference setup records to the agent-protocol-domain
  timer parent-surface read-model schema.
- Mirrored the setup record shape in Rust protocol and agent-service
  timer-parent-surface payload construction.
- Derived setup records from service action-result rows and parent-surface
  intent records with parent-safe refs.
- Updated App/Game Sessions portal intent rendering to consume service-emitted
  setup records directly.
- Extended focused TypeScript, Rust, and portal tests for setup-record visibility
  and no-claim boundaries.

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

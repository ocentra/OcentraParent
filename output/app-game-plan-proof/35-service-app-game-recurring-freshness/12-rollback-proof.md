# Rollback Proof

No enforcement, blocking, termination, shielding, suspension, uninstall, or
policy action was added in WP35.

Rollback state:

- Stop the service process to stop the recurring capture loop.
- Delete local activity journal/query-store artifacts only as a separate data
  retention/admin operation.
- No adapter cleanup is required because no adapter action executes.

Safe-failure boundary:

If capture fails, the existing activity-capture failure log path records the
reason. Failure does not trigger policy or adapter execution.

# Rollback And Cleanup Proof

Rollback state for the successful owned/current expiry remains
`not-required`.

Parent override cancel proof:

- Normal recovered timer was cancelled and cleared state.
- Dry-run timer was cancelled and cleared state.
- Stale mismatch timer was recovered after rejection, then cancelled and cleared
  state.

Process cleanup:

- The harness stops all child processes in `finally`.
- The expiry child reached `expired/process-terminated` through the scoped
  process adapter.

# Timer Recovery Proof

WP20 adds contract-level restart recovery checks:

- `restart-recovered` timer state requires at least one active timer reference.
- `restart-recovered` timer state requires at least one timer recovery audit
  reference.
- Decisions with recovered timers still need policy preview evidence,
  schedule evidence, stored session refs, and consistent effective budget math.

This does not claim runtime timer restoration, service persistence, platform
timer execution, or adapter rollback. It proves only that a recovered timer
decision cannot be represented without auditable timer and recovery refs.

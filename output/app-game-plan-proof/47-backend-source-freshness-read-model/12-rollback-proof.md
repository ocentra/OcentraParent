# Rollback Proof

Rollback is data-shape-only:

1. Remove `sourceStatusRows` from activity-domain and Rust activity-surface
   read-model row contracts.
2. Remove service projection helper usage from app-use and games read-models.
3. Remove WP47 tests and docs/proof rows.

No timers, process controls, app blocks, platform adapters, or persisted policy
actions are introduced by this slice.

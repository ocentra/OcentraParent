# WP07 Offline Timer Recovery Proof

## Proves

- `policy-schedule.offline-device-timer-recovery`

## Evidence

- `packages/policy-domain/tests/unit/policy-schedule-boundaries.test.ts`
  - `parsePolicyScheduleBoundary: accepts budget status with recovered offline timer state and expiring bonus time`
  - `parsePolicyScheduleBoundary: rejects not-needed offline recovery states that still claim recovered timer state`
- `crates/policy-control-core/tests/unit/policy_preview.rs`
  - `offline_target_state_stays_visible_and_blocks_save`
- `crates/policy-control-core/tests/unit/policy_compiler.rs`
  - `compiler_preserves_wp07_time_boundary_schedule_shapes_verbatim`

## Result

- Offline recovery state is explicit contract data.
- Preview/save remains blocked when the target is offline instead of faking readiness.
- Recovered timer state cannot be claimed without the matching recovery mode.


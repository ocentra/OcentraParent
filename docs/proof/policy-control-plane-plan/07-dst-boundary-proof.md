# WP07 DST Boundary Proof

## Proves

- `policy-schedule.dst-spring-forward`
- `policy-schedule.dst-fall-back`
- `policy-schedule.ambiguous-local-time`
- `policy-schedule.nonexistent-local-time`

## Evidence

- `packages/policy-domain/tests/unit/policy-schedule-boundaries.test.ts`
  - `parsePolicyScheduleBoundary: accepts fall-back overlap boundaries with an explicit occurrence choice`
  - `parsePolicyScheduleBoundary: rejects spring-forward gaps that try to use overlap-only resolution`
- `crates/policy-control-core/tests/unit/policy_conflict.rs`
  - `nonexistent_local_time_stays_explicit_and_blocking`
  - `ambiguous_local_time_stays_explicit_and_blocking`

## Result

- Spring-forward gaps and fall-back overlaps are handled explicitly.
- Ambiguous and nonexistent local times are blocking/manual-required states, not silent success paths.


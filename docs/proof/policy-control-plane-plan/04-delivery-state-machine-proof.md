# WP04 Delivery State Machine Proof

Run id: `019ed32a-fdd2-74b0-bb81-6e152680ac97/2026-06-17T20:17:50Z`

Correlation: `policy-control-plane-plan / WP04 / policy-wp04-delivery-ack-audit / state-machine`

## Validation used

- `npm run test --workspace @ocentra-parent/policy-domain -- tests/unit/policy-compiler.test.ts tests/unit/policy-event.test.ts`
- `cargo test -p ocentra-policy-control-core policy_delivery`
- `cargo test -p ocentra-policy-control-core policy_source`
- `npm run lint:architecture -- --files packages/policy-domain/src/policy.ts packages/policy-domain/src/policy-compiler.ts packages/policy-domain/src/policy-event.ts packages/policy-domain/tests/unit/policy-compiler.test.ts packages/policy-domain/tests/unit/policy-event.test.ts`
- `cargo lint-architecture crates/policy-control-core/src/policy_delivery.rs crates/policy-control-core/src/policy_source.rs crates/policy-control-core/tests/unit/policy_delivery.rs crates/policy-control-core/tests/version-skew/policy_source.rs`

All commands passed on 2026-06-17.

## Owner source surfaces

- `crates/policy-control-core/src/policy_delivery.rs` defines the delivery state enum, transition gate, parent-visible state mapping, active-state gate, reason-code requirements, supersede constraints, rollback reference constraints, and replay handling.
- `crates/policy-control-core/src/policy_source.rs` keeps source-truth lifecycle separate from delivery state and requires acknowledged delivery evidence for active source states.
- `packages/policy-domain/src/policy-compiler.ts` keeps delivery targets and audit references explicit on compiled artifacts.
- `packages/policy-domain/src/policy-event.ts` defines explicit delivery, retry, rollback, audit, and manual-required event kinds.

## Required lifecycle coverage

`policy_delivery.rs` owns the explicit runtime states required by WP04:

- `queued`
- `delivering`
- `delivered`
- `acknowledged`
- `applied`
- `rejected`
- `superseded`
- `rolled-back`
- `degraded`
- `offline`
- `expired-before-delivery`
- `retry-scheduled`
- `partial-domain-apply`
- `blocked-by-permission`
- `blocked-by-capability`
- `manual-required`

## Proof mapping

| WP04 proof id | Current owner evidence |
| --- | --- |
| `policy-delivery.state-machine` | `delivering_state_stays_pending_until_ack_or_apply`, `acknowledged_delivery_stays_pending_and_is_not_active`, `applied_transition_stays_active_when_intermediate_events_arrive_late`, `superseded_transition_requires_newer_policy_version_and_blocks_regressions`, and `policy_delivery_round_trips_explicit_wp04_delivery_states` |
| `policy-delivery.ack-required` | `acknowledged_delivery_stays_pending_and_is_not_active` plus `active_status_requires_acknowledged_delivery_for_every_target` |
| `policy-delivery.parent-visible-state` | `queued_delivery_starts_pending_per_child_device_domain`, `delivering_state_stays_pending_until_ack_or_apply`, `acknowledged_delivery_stays_pending_and_is_not_active`, `offline_delivery_is_degraded_and_requires_reason_code`, `retry_partial_and_expired_transitions_stay_degraded_until_real_delivery_progress`, `blocked_and_manual_required_transitions_require_reason_and_surface_manual_required`, `superseded_transition_requires_newer_policy_version_and_blocks_regressions` |
| `policy-delivery.per-device-domain-status` | `queued_delivery_starts_pending_per_child_device_domain` plus explicit `PolicyDeliveryTarget { child_profile_id, device_id, domain }` ownership in `policy_delivery.rs` |

## Honest boundary

This proof closes the delivery state-machine contract on policy-owned surfaces only. It does not claim portal rendering, shared event transport mechanics, or enforcement runtime execution.

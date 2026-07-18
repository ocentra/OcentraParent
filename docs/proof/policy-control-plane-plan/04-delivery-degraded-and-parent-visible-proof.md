# WP04 Delivery Degraded and Parent-Visible Proof

Run id: `019f773f-d986-7db2-8a0d-2fba41e42bd2/2026-07-18-degraded-parent-visible-refresh`

Correlation: `policy-control-plane-plan / WP04 / policy-wp04-delivery-ack-audit / degraded-parent-visible`

## Validation source

- `cargo test -p ocentra-policy-control-core --test unit --test version-skew`
- `cargo test -p ocentra-child-policy-core --test replay_policy_control_delivery_handoff`
- `cargo test -p ocentra-parent-runtime-core --test unit policy_control_update_flow`

## Proof mapping

| WP04 proof id | Current owner evidence |
| --- | --- |
| `policy-delivery.offline-degraded` | `offline_delivery_is_degraded_and_requires_reason_code` |
| `policy-delivery.retry-safe` | `retry_partial_and_expired_transitions_stay_degraded_until_real_delivery_progress` |
| `policy-delivery.partial-domain-apply` | `retry_partial_and_expired_transitions_stay_degraded_until_real_delivery_progress` |
| `policy-delivery.expired-before-delivery` | `retry_partial_and_expired_transitions_stay_degraded_until_real_delivery_progress` |
| `policy-delivery.permission-loss-blocked` | `blocked_and_manual_required_transitions_require_reason_and_surface_manual_required` |
| `policy-delivery.parent-visible-state` | parent-visible assertions across `queued_delivery_starts_pending_per_child_device_domain`, `delivering_state_stays_pending_until_ack_or_apply`, `acknowledged_delivery_stays_pending_and_is_not_active`, `offline_delivery_is_degraded_and_requires_reason_code`, `retry_partial_and_expired_transitions_stay_degraded_until_real_delivery_progress`, `blocked_and_manual_required_transitions_require_reason_and_surface_manual_required`, and `superseded_before_ack_stays_superseded_and_never_becomes_active` |

## Current Rust owner support

- `crates/policy-control-core/src/policy_delivery.rs` owns delivery states, parent-visible mapping, active-state semantics, and the fail-closed transition-only API.
- `crates/policy-control-core/src/policy_event.rs` owns Rust delivery, retry, rollback, audit, and manual-required event contracts and redacted summaries.
- `crates/child-policy-core/src/policy_control_delivery_handoff.rs` and `crates/parent-runtime-core/src/policy_control_update_flow.rs` keep transition-only runtime seams pending/degraded/manual-required and reject receipt-required promotion.

## No fake-success boundary

Current owner proof shows:

- queued, delivering, delivered, and acknowledged stay `Pending` to parents
- offline, retry-scheduled, expired-before-delivery, and partial-domain-apply stay `Degraded`
- blocked-by-permission, blocked-by-capability, rejected, rolled-back, and manual-required stay `ManualRequired`
- superseded stays `Superseded`
- only receipt-validated `Applied` becomes active; transition-only callers cannot create it

That is the plan-owned no-fake-success contract for WP04.

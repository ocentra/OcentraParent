# WP04 Delivery Degraded and Parent-Visible Proof

Run id: `019ed32a-fdd2-74b0-bb81-6e152680ac97/2026-06-17T20:17:50Z`

Correlation: `policy-control-plane-plan / WP04 / policy-wp04-delivery-ack-audit / degraded-parent-visible`

## Validation source

- `npm run test --workspace @ocentra-parent/policy-domain -- tests/unit/policy-event.test.ts`
- `cargo test -p ocentra-policy-control-core policy_delivery`

## Proof mapping

| WP04 proof id | Current owner evidence |
| --- | --- |
| `policy-delivery.offline-degraded` | `offline_delivery_is_degraded_and_requires_reason_code` |
| `policy-delivery.retry-safe` | `retry_partial_and_expired_transitions_stay_degraded_until_real_delivery_progress` |
| `policy-delivery.partial-domain-apply` | `retry_partial_and_expired_transitions_stay_degraded_until_real_delivery_progress` |
| `policy-delivery.expired-before-delivery` | `retry_partial_and_expired_transitions_stay_degraded_until_real_delivery_progress` |
| `policy-delivery.permission-loss-blocked` | `blocked_and_manual_required_transitions_require_reason_and_surface_manual_required` |
| `policy-delivery.parent-visible-state` | parent-visible assertions across `queued_delivery_starts_pending_per_child_device_domain`, `delivering_state_stays_pending_until_ack_or_apply`, `acknowledged_delivery_stays_pending_and_is_not_active`, `offline_delivery_is_degraded_and_requires_reason_code`, `retry_partial_and_expired_transitions_stay_degraded_until_real_delivery_progress`, `blocked_and_manual_required_transitions_require_reason_and_surface_manual_required`, and `superseded_before_ack_stays_superseded_and_never_becomes_active` |

## TypeScript contract support

- `packages/schema-domain/src/policy-event.ts` keeps delivery event kinds explicit: `policy.delivery.queued`, `policy.delivery.sent`, `policy.delivery.acknowledged`, `policy.delivery.rejected`, `policy.delivery.expired`, `policy.delivery.retry-scheduled`, `policy.domain.applied`, `policy.manual-required`, `policy.rollback.applied`, and `policy.audit.recorded`.
- `policyEventAggregateKey: keeps delivery keys stable and redacted summaries free of private identifiers` proves the TS event boundary keeps delivery identity stable while avoiding private raw identifiers in summarized output.

## No fake-success boundary

Current owner proof shows:

- queued, delivering, delivered, and acknowledged stay `Pending` to parents
- offline, retry-scheduled, expired-before-delivery, and partial-domain-apply stay `Degraded`
- blocked-by-permission, blocked-by-capability, rejected, rolled-back, and manual-required stay `ManualRequired`
- superseded stays `Superseded`
- only `Applied` becomes active

That is the plan-owned no-fake-success contract for WP04.

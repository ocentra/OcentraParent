# WP04 Delivery Degraded and Parent-Visible Proof

Run id: `019f773f-d986-7db2-8a0d-2fba41e42bd2/2026-07-18-degraded-parent-visible-refresh`

Receipt-hydration refresh: `policy-wp04-record-boundary/2026-07-18`

Untrusted-Applied refresh: `policy-wp04-record-boundary/2026-07-18-forged-receipt-rejection`

Correlation: `policy-control-plane-plan / WP04 / policy-wp04-delivery-ack-audit / degraded-parent-visible`

## Validation source

- `cargo test -p ocentra-policy-control-core --test unit --test version-skew`
- `cargo test -p ocentra-child-policy-core --test replay_policy_control_delivery_handoff`
- `cargo test -p ocentra-parent-runtime-core --test unit policy_control_`
- `cargo test -p ocentra-child-notification-core --test observability_policy_control_notification`

## Proof mapping

| WP04 proof id | Current owner evidence |
| --- | --- |
| `policy-delivery.offline-degraded` | `offline_delivery_is_degraded_and_requires_reason_code` |
| `policy-delivery.retry-safe` | `retry_partial_and_expired_transitions_stay_degraded_until_real_delivery_progress` |
| `policy-delivery.partial-domain-apply` | `retry_partial_and_expired_transitions_stay_degraded_until_real_delivery_progress` |
| `policy-delivery.expired-before-delivery` | `retry_partial_and_expired_transitions_stay_degraded_until_real_delivery_progress` |
| `policy-delivery.permission-loss-blocked` | `blocked_and_manual_required_transitions_require_reason_and_surface_manual_required` |
| `policy-delivery.parent-visible-state` | parent-visible assertions across `queued_delivery_starts_pending_per_child_device_domain`, `delivering_state_stays_pending_until_ack_or_apply`, `acknowledged_evidence_cannot_advance_without_trusted_adapter_authority`, `generic_acknowledged_hydration_rejects_matching_public_receipt`, `generic_rolled_back_hydration_rejects_matching_public_receipt`, `offline_delivery_is_degraded_and_requires_reason_code`, `retry_partial_and_expired_transitions_stay_degraded_until_real_delivery_progress`, `blocked_and_manual_required_transitions_require_reason_and_surface_manual_required`, `delivery_handoff_surfaces_receipt_required_states_as_manual_required`, `superseded_before_ack_stays_superseded_and_never_becomes_active`, `applied_state_without_receipt_evidence_fails_closed`, `fully_matching_public_receipt_remains_untrusted_for_applied_hydration`, `schema_v1_receiptless_acknowledged_hydrates_as_unverified_manual_required`, and `legacy_unverified_acknowledged_delivery_requires_manual_parent_action` |

## Current Rust owner support

- `crates/policy-control-core/src/policy_delivery.rs` owns delivery states, parent-visible mapping, active-state semantics, and the fail-closed transition-only API.
- `crates/policy-control-core/src/policy_event.rs` owns Rust delivery, retry, rollback, audit, and manual-required event contracts and redacted summaries.
- `crates/child-policy-core/src/policy_control_delivery_handoff.rs` converts acknowledged and applied requests to typed `ManualRequired`, while `crates/parent-runtime-core/src/policy_control_update_flow.rs` rejects receipt-required promotion through its transition-only seam.

## No fake-success boundary

Current owner proof shows:

- queued, delivering, and delivered stay `Pending` to parents; a future trusted schema-v2 acknowledged record maps to `Pending`, but the current public surface cannot mint one
- offline, retry-scheduled, expired-before-delivery, and partial-domain-apply stay `Degraded`
- blocked-by-permission, blocked-by-capability, rejected, rolled-back, and manual-required stay `ManualRequired`
- superseded stays `Superseded`
- no current public policy path creates active `Applied`: raw receipts are evidence, not authority, and no production execution-authority entry exists until a trusted domain/enforcement adapter and inspectable trace are implemented
- generic/untrusted record deserialization rejects every schema-v2 acknowledged, applied, and rolled-back payload, including caller-built payloads with fully self-consistent receipts; WP04 defines no authenticated persistence capability for rehydrating these states, so their persisted hydration is unsupported and not claimed
- a directly forged `Applied` record without the private stored receipt remains inactive and maps to parent-visible `ManualRequired`
- schema-v1 receiptless acknowledged and rolled-back records preserve their historical state, audit, rollback, and source facts under `LegacySchemaV1Unverified`, but surface parent-visible `ManualRequired` and never become active

That is the plan-owned no-fake-success contract for WP04. Real positive execution remains dependency-blocked.

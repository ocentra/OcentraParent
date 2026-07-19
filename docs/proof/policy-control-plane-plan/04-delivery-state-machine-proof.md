# WP04 Delivery State Machine Proof

Run id: `019f773f-d986-7db2-8a0d-2fba41e42bd2/2026-07-18-policy-receipt-enforcement`

Correlation: `policy-control-plane-plan / WP04 / policy-wp04-delivery-ack-audit / state-machine`

## Validation used

- `cargo test -p ocentra-policy-control-core --test unit --test version-skew`
- `cargo test -p ocentra-child-policy-core --test replay_policy_control_delivery_handoff`
- `cargo test -p ocentra-parent-runtime-core --test unit policy_control_update_flow`
- `cargo clippy -p ocentra-policy-control-core -p ocentra-child-policy-core -p ocentra-parent-runtime-core --all-targets -- -D warnings`
- `cargo fmt --all -- --check`
- `npm run lint:architecture -- --files crates/policy-control-core crates/child-policy-core docs/proof/policy-control-plane-plan/04-delivery-state-machine-proof.md`

All commands passed on 2026-07-18.

## Owner source surfaces

- `crates/policy-control-core/src/policy_delivery.rs` defines the delivery state enum, transition gate, parent-visible state mapping, active-state gate, reason-code requirements, supersede constraints, rollback reference constraints, non-execution replay handling, the caller-owned queue identity contract, an opt-in deterministic identity helper, and the public fail-closed transition-only API.
- `crates/policy-control-core/src/policy_source.rs` keeps source-truth lifecycle separate from delivery state and requires acknowledged delivery evidence for active source states.
- `crates/policy-control-core/src/policy_delivery/adapter_execution.rs` validates typed receipt evidence structurally but exposes no production execution-authority entry that can advance acknowledged, applied, or rolled-back state.
- `crates/child-policy-core/src/policy_control_delivery_handoff.rs` forwards the caller-owned delivery identity without replacement and converts acknowledged or applied requests to typed `ManualRequired` state until a trusted adapter exists; rolled-back requests retain strict rollback validation.
- `crates/parent-runtime-core/src/policy_control_update_flow.rs` also uses the transition-only API and cannot promote child transitions to acknowledged or applied without a real receipt-aware handoff.

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
| `policy-delivery.state-machine` | `delivering_state_stays_pending_until_ack_or_apply`, `acknowledged_evidence_cannot_advance_without_trusted_adapter_authority`, `generic_acknowledged_hydration_rejects_matching_public_receipt`, `applied_receipt_evidence_cannot_advance_without_trusted_adapter_authority`, `generic_rolled_back_hydration_rejects_matching_public_receipt`, `late_intermediate_events_are_stale_after_newer_non_execution_progress`, `superseded_transition_requires_newer_policy_version_and_blocks_regressions`, and `policy_delivery_round_trips_explicit_wp04_delivery_states` |
| `policy-delivery.ack-required` | `generic_acknowledged_hydration_rejects_matching_public_receipt`, `generic_applied_hydration_rejects_fully_matching_forged_receipt`, `generic_rolled_back_hydration_rejects_matching_public_receipt`, `schema_v1_receiptless_applied_is_not_legacy_compatible`, `bare_transition_apis_reject_every_receipt_required_state`, `delivery_handoff_surfaces_receipt_required_states_as_manual_required`, `parent_runtime_policy_control_flow_rejects_receipt_required_child_transitions`, `acknowledged_evidence_cannot_advance_without_trusted_adapter_authority`, and `active_status_requires_acknowledged_delivery_for_every_target` |
| `policy-delivery.parent-visible-state` | `queued_delivery_starts_pending_per_child_device_domain`, `delivering_state_stays_pending_until_ack_or_apply`, `schema_v1_receiptless_acknowledged_hydrates_as_unverified_manual_required`, `offline_delivery_is_degraded_and_requires_reason_code`, `retry_partial_and_expired_transitions_stay_degraded_until_real_delivery_progress`, `blocked_and_manual_required_transitions_require_reason_and_surface_manual_required`, and `superseded_transition_requires_newer_policy_version_and_blocks_regressions` |
| `policy-delivery.per-device-domain-status` | `queued_delivery_starts_pending_per_child_device_domain` plus explicit `PolicyDeliveryTarget { child_profile_id, device_id, domain }` ownership in `policy_delivery.rs` |
| `policy-delivery.identity-compatibility` | `queue_preserves_caller_delivery_id_while_derivation_remains_opt_in`, `queued_delivery_preserves_caller_provided_delivery_id`, and `delivery_queue_starts_pending_per_child_device_domain` prove that the established queue API and production handoff preserve the supplied correlation identity; `policy_delivery_id_is_derived_from_full_scope_and_is_stable` separately proves deterministic opt-in derivation |

## Honest boundary

This proof closes the delivery state-machine contract on policy-owned surfaces only. It proves queue identity compatibility and deterministic identity derivation as separate contracts, and proves that public production seams fail closed rather than fabricating execution authority. WP04 remains runtime-blocked until a trusted domain- or enforcement-owned adapter performs the real side effect, emits the required inspectable execution trace, and supplies non-forgeable authority through a future production entry. It does not claim portal rendering, shared event transport mechanics, or enforcement runtime execution.

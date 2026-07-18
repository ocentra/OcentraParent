# WP04 Delivery Replay and Ordering Proof

Run id: `019f773f-d986-7db2-8a0d-2fba41e42bd2/2026-07-18-policy-replay-ordering-refresh`

Correlation: `policy-control-plane-plan / WP04 / policy-wp04-delivery-ack-audit / replay-ordering`

## Validation source

- `cargo test -p ocentra-policy-control-core --test unit --test version-skew`
- `cargo test -p ocentra-child-policy-core --test replay_policy_control_delivery_handoff`

## Proof mapping

| WP04 proof id | Current owner evidence |
| --- | --- |
| `policy-delivery.idempotent` | `duplicate_and_older_transitions_are_safe_noops` and `delivery_replay_same_sequence_is_duplicate_and_older_sequence_is_stale` |
| `policy-delivery.out-of-order-safe` | `duplicate_and_older_transitions_are_safe_noops` and `applied_transition_stays_active_when_intermediate_events_arrive_late` |
| `policy-delivery.replay-rejected` | `conflicting_same_sequence_replay_is_rejected` in the visible unit owner file and the same-name filtered cargo test result |
| `policy-delivery.superseded-before-ack` | `superseded_before_ack_stays_superseded_and_never_becomes_active` |

## Source-backed constraints

- The public transition-only path returns `Stale` for older non-execution sequence numbers, `Duplicate` for identical non-execution replays at the same sequence, and rejects same-sequence conflicts with an explicit `policy_delivery.sequence` error.
- Acknowledged, applied, and rolled-back transitions use the receipt-aware adapter path; the transition-only path rejects them before they can advance state.
- `transition_allowed` prevents regressions from terminal states such as `Superseded` back to `Delivered`.
- `active_status_requires_acknowledged_delivery_for_every_target` keeps source truth from presenting active policy globally until the delivery side has explicit acknowledgement coverage.

## Negative cases covered

- duplicate delivery does not create duplicate active policy
- stale out-of-order delivery does not overwrite newer policy state
- same-sequence conflicting replay is rejected instead of silently accepted
- supersede-before-ack stays explicit and never promotes the delivery record to active

## Honest boundary

This proof covers delivery-owned ordering, replay, and supersede semantics only. It does not claim broader event-bus durability or cross-process delivery transport behavior.

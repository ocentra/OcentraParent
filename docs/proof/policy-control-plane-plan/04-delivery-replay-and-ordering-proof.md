# WP04 Delivery Replay and Ordering Proof

Run id: `019ed32a-fdd2-74b0-bb81-6e152680ac97/2026-06-17T20:17:50Z`

Correlation: `policy-control-plane-plan / WP04 / policy-wp04-delivery-ack-audit / replay-ordering`

## Validation source

- `cargo test -p ocentra-policy-control-core policy_delivery`
- `cargo test -p ocentra-policy-control-core policy_source`

## Proof mapping

| WP04 proof id | Current owner evidence |
| --- | --- |
| `policy-delivery.idempotent` | `duplicate_and_older_transitions_are_safe_noops` and `delivery_replay_same_sequence_is_duplicate_and_older_sequence_is_stale` |
| `policy-delivery.out-of-order-safe` | `duplicate_and_older_transitions_are_safe_noops` and `applied_transition_stays_active_when_intermediate_events_arrive_late` |
| `policy-delivery.replay-rejected` | `conflicting_same_sequence_replay_is_rejected` in the visible unit owner file and the same-name filtered cargo test result |
| `policy-delivery.superseded-before-ack` | `superseded_before_ack_stays_superseded_and_never_becomes_active` |

## Source-backed constraints

- `apply_policy_delivery_transition` returns `Stale` for older sequence numbers, `Duplicate` for identical replays at the same sequence, and rejects same-sequence conflicts with an explicit `policy_delivery.sequence` error.
- `transition_allowed` prevents regressions from terminal states such as `Superseded` back to `Delivered`.
- `active_status_requires_acknowledged_delivery_for_every_target` keeps source truth from presenting active policy globally until the delivery side has explicit acknowledgement coverage.

## Negative cases covered

- duplicate delivery does not create duplicate active policy
- stale out-of-order delivery does not overwrite newer policy state
- same-sequence conflicting replay is rejected instead of silently accepted
- supersede-before-ack stays explicit and never promotes the delivery record to active

## Honest boundary

This proof covers delivery-owned ordering, replay, and supersede semantics only. It does not claim broader event-bus durability or cross-process delivery transport behavior.

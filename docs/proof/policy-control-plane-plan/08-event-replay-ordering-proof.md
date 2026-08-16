# WP08 Event Replay Ordering Proof

## Proves

- `policy-event.replay-safe`
- `policy-event.out-of-order-safe`

## Evidence

- `packages/policy-domain/tests/unit/policy-event.test.ts`
  - `applyPolicyEventReplay: accepts duplicate and stale replays but rejects conflicting same-sequence writes`
- `crates/policy-control-core/tests/unit/policy_event.rs`
  - `policy_event_replay_tracks_duplicate_stale_and_conflicting_sequences`

## Result

- Duplicate and stale replays are accepted as no-op/replay-safe outcomes.
- Conflicting same-sequence writes are rejected.
- Out-of-order sequences cannot silently overwrite newer state.


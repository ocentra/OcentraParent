# Browser Runtime Action Intent Durable Handoff Proof

This proof carries the named browser action-intent handoff subscriber result into a durable result/read-model row without creating dispatch, browser mutation, child-intervention execution, final policy execution, or enforcement claims.

The row preserves the source handoff event, policy preview id, parent action-intent id, local outbox ref, local handoff ref, durable result ref, durable store ref, read-model ref, and support-status ref. Duplicate request event ids are rejected before projection.

Validation:
- `cargo test -p ocentra-parent-agent-core browser_runtime_action_intent_durable_handoff --quiet`

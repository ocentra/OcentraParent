# Browser Runtime Action Intent Store-Backed Proof

This proof closes the prior store-backed action-intent projection gap for browser evidence rows that have a matching stored policy preview read-model row.

The service-backed browser runtime stream now loads the existing browser evidence read model and the existing policy preview read model from the ActivityStore. Matching policy-preview evidence refs enrich the browser runtime input with a policy preview id, policy decision ref, stable browser action-intent id, dry-run authority, and one pending action-intent candidate.

The projection remains non-mutating: dispatch attempts, adapter execution, child intervention execution, final policy execution, and enforcement execution all stay at zero.

Validation:
- `cargo test -p ocentra-parent-agent-service service_browser_runtime --quiet`

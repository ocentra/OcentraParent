# WP03 Deterministic Output Proof

Run id: `019ed32a-fdd2-74b0-bb81-6e152680ac97/2026-06-17T19:43:50Z`

Correlation: `policy-control-plane-plan / WP03 / policy-wp03-compiler-proof-bundle / deterministic-output`

## Validation source

- `npm run test --workspace @ocentra-parent/policy-domain -- tests/unit/policy-compiler.test.ts tests/unit/policy-schedule-boundaries.test.ts tests/unit/policy-event.test.ts`
- `cargo test -p ocentra-policy-control-core`

## Determinism proof

| Deterministic property | Owner proof |
| --- | --- |
| TS contract accepts a deterministic compiled artifact shape with explicit metadata | `parsePolicyCompiledArtifact: parses deterministic compiled artifacts with explicit no-claim and status metadata` |
| Cross-domain compiler output stays deterministic | `compiler_outputs_stay_deterministic_across_domain_matrix` |
| Notification ask-parent output stays deterministic | `notification_ask_parent_compiler_keeps_review_rules_ready_and_stays_deterministic` |
| Artifact IDs and delivery scope are deterministic and explicit | `compiler_artifact_has_deterministic_id_and_explicit_delivery_scope` |
| Schedule payload survives serialization | `compiled_artifact_serialization_preserves_schedule_payload` |
| WP07 time-boundary payload survives serialization | `compiled_artifact_round_trips_wp07_time_boundary_schedule_payload` |
| Explicit support-matrix payload survives serialization | `compiled_artifact_round_trips_explicit_support_matrix_payload` |

## Source-backed constraints

- `crates/policy-control-core/src/policy_compiler.rs` computes `compiled_artifact_id` from the compiler domain, source document id, and policy version.
- The same source file always copies `audit_reference_ids`, `superseded_by_policy_version`, `rollback_ref`, `schedules`, and validated rules into the output artifact.
- `packages/policy-domain/src/policy-compiler.ts` rejects duplicate audit refs, missing no-claim labels, and mixed supersede-plus-rollback state, preventing structurally ambiguous artifacts from passing as deterministic output.

## No runtime mutation / no silent-drop boundary

- Compiler artifacts carry the full no-claim set exactly once.
- `screen_compiler_keeps_manual_required_and_unsupported_targets_explicit` and `domain_override_serialization_preserves_rule_capability_reason_pairs` prove deterministic output still keeps degraded states visible instead of silently dropping them.

## Honest conclusion

Current owner proof shows deterministic compiler artifacts and serialized payloads for WP03. It does not claim downstream delivery ordering, read-model convergence, or UI rendering determinism.

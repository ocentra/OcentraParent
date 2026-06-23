# WP03 Unsupported and Manual-Required Proof

Run id: `019ed32a-fdd2-74b0-bb81-6e152680ac97/2026-06-17T19:43:50Z`

Correlation: `policy-control-plane-plan / WP03 / policy-wp03-compiler-proof-bundle / unsupported-manual-required`

## Validation source

- `npm run test --workspace @ocentra-parent/policy-domain -- tests/unit/policy-compiler.test.ts tests/unit/policy-schedule-boundaries.test.ts tests/unit/policy-event.test.ts`
- `cargo test -p ocentra-policy-control-core`

## TypeScript contract proof

- `PolicyCompiledArtifactSchema: rejects manual-required or unsupported rules without a reason code`
- `PolicyCompiledArtifactSchema: rejects rules whose capabilityState and status disagree`
- `PolicyEventSchema: keeps rollback linkage and dead-letter/manual-required visibility explicit`
- `resolvePolicyPreviewBudgetBoundaryState: marks manual clock-source preview boundaries as manual-required`

These tests prove the domain contract does not silently coerce unsupported/manual-required states into ready states and does not drop the reason path.

## Rust owner proof

| Behavior | Owner proof |
| --- | --- |
| Screen compiler keeps unsupported and manual-required targets explicit | `screen_compiler_keeps_manual_required_and_unsupported_targets_explicit` |
| Enforcement hints stay manual-required even when AI context is broadly ready | `ai_context_compiler_is_broadly_ready_but_enforcement_hints_stay_manual_required` |
| Support-matrix overrides preserve capability-state and reason-code pairs | `domain_override_serialization_preserves_rule_capability_reason_pairs` |
| Event surface keeps manual-required and dead-letter payloads explicit | `policy_event_manual_required_and_dead_letter_payloads_remain_explicit` |

## Source-backed constraints

- `packages/policy-domain/src/policy-compiler.ts` maps `manual-required` and `unsupported` capability states to matching rule statuses and rejects missing reason codes.
- `crates/policy-control-core/src/policy_compiler.rs` validates support-matrix/domain alignment before emitting compiler rules.
- `packages/policy-domain/src/policy-event.ts` and `crates/policy-control-core/src/policy_event.rs` keep manual-required and unsupported/dead-letter states explicit on the event boundary.

## Honest conclusion

Current owner surfaces prove that unsupported and manual-required outcomes stay explicit through compiler contracts, serialized artifacts, and event payloads. This proof does not claim that later delivery consumers, portal surfaces, or enforcement adapters have handled those states end to end.

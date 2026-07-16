# WP03 Domain Compiler Matrix Proof

Run id: `019ed32a-fdd2-74b0-bb81-6e152680ac97/2026-06-17T19:43:50Z`

Correlation: `policy-control-plane-plan / WP03 / policy-wp03-compiler-proof-bundle / compiler-matrix`

## Validation used

- `npm run test --workspace @ocentra-parent/policy-domain -- tests/unit/policy-compiler.test.ts tests/unit/policy-schedule-boundaries.test.ts tests/unit/policy-event.test.ts`
- `cargo test -p ocentra-policy-control-core`
- `npm run lint:architecture -- --files packages/policy-domain/src/policy.ts packages/policy-domain/src/policy-compiler.ts packages/policy-domain/src/policy-event.ts packages/policy-domain/tests/unit/policy-compiler.test.ts packages/policy-domain/tests/unit/policy-schedule-boundaries.test.ts packages/policy-domain/tests/unit/policy-event.test.ts`
- `cargo lint-architecture crates/policy-control-core/src/policy_source.rs crates/policy-control-core/src/policy_compiler.rs crates/policy-control-core/src/policy_event.rs crates/policy-control-core/tests/unit crates/policy-control-core/tests/version-skew`

All commands passed on this checkout on 2026-06-17.

## Owner source surfaces

- `packages/policy-domain/src/policy-compiler.ts` defines the compiled artifact contract, support-matrix shape, no-claim labels, audit refs, and supersede-versus-rollback exclusivity.
- `crates/policy-control-core/src/policy_compiler.rs` defines `compile_app_game_policy`, `compile_browser_policy`, `compile_network_policy`, `compile_tracking_policy`, `compile_screen_policy`, `compile_ai_policy_context`, `compile_enforcement_policy_hints`, `compile_notification_ask_parent_policy`, deterministic artifact IDs, explicit delivery targets, support-matrix checks, and compiler no-claim labels.
- `crates/policy-control-core/src/policy_source.rs` gates compilation on validated source truth and preserves `audit_reference_ids`, `superseded_by_policy_version`, and `rollback_ref` on compiled artifacts.

## Proof mapping

| WP03 proof id | Current owner evidence |
| --- | --- |
| `policy-compiler.contract-matrix` | `packages/policy-domain/tests/unit/policy-compiler.test.ts` proves the TS contract shape accepts deterministic compiled artifacts with explicit no-claim/status metadata. `crates/policy-control-core/tests/unit/policy_compiler.rs` proves the Rust compiler emits domain artifacts across the owned domain matrix. |
| `policy-compiler.app-game-fixture` | `app_game_compiler_keeps_app_targets_ready_and_browser_targets_explicit` |
| `policy-compiler.browser-fixture` | `browser_compiler_keeps_site_targets_ready_and_app_targets_unsupported` plus the browser-domain sample artifact in `policy-compiler.test.ts` |
| `policy-compiler.network-fixture` | `network_and_tracking_compilers_keep_geofence_and_location_targets_explicit` |
| `policy-compiler.tracking-fixture` | `network_and_tracking_compilers_keep_geofence_and_location_targets_explicit` |
| `policy-compiler.screen-fixture` | `screen_compiler_keeps_manual_required_and_unsupported_targets_explicit` |
| `policy-compiler.ai-context-fixture` | `ai_context_compiler_is_broadly_ready_but_enforcement_hints_stay_manual_required` |
| `policy-compiler.enforcement-handoff-fixture` | `ai_context_compiler_is_broadly_ready_but_enforcement_hints_stay_manual_required` proves the compiler emits explicit manual-required enforcement hints. This does not claim enforcement runtime execution. |
| `policy-compiler.domain-cache-not-truth` | `compiler_rejects_domain_cache_source_documents_as_non_canonical_source_truth` |
| `policy-compiler.rollback-ref-present` | `compiler_artifact_preserves_audit_and_lifecycle_refs_from_source_documents` plus `compile_domain_policy` copying `rollback_ref` from validated source documents |

## Compiler matrix status

| Domain surface | Compile entrypoint | Proved by |
| --- | --- | --- |
| App/game | `compile_app_game_policy` | `app_game_compiler_keeps_app_targets_ready_and_browser_targets_explicit` |
| Browser | `compile_browser_policy` | `browser_compiler_keeps_site_targets_ready_and_app_targets_unsupported` |
| Network | `compile_network_policy` | `network_and_tracking_compilers_keep_geofence_and_location_targets_explicit` |
| Tracking | `compile_tracking_policy` | `network_and_tracking_compilers_keep_geofence_and_location_targets_explicit` |
| Screen | `compile_screen_policy` | `screen_compiler_keeps_manual_required_and_unsupported_targets_explicit` |
| AI context | `compile_ai_policy_context` | `ai_context_compiler_is_broadly_ready_but_enforcement_hints_stay_manual_required` |
| Enforcement hints | `compile_enforcement_policy_hints` | `ai_context_compiler_is_broadly_ready_but_enforcement_hints_stay_manual_required` |
| Notification ask parent | `compile_notification_ask_parent_policy` | `notification_ask_parent_compiler_keeps_review_rules_ready_and_stays_deterministic` |

## No-claim boundary preserved in WP03

- `compile_domain_policy` always attaches compiler no-claim labels for not-source-truth, no runtime mutation, no enforcement claim, no UI delivery claim, and no platform-support claim.
- `PolicyCompiledArtifactSchema` rejects artifacts that omit the full no-claim set, duplicate audit refs, or carry both supersede and rollback refs.

## Honest boundary

This proof closes the owner-side compiler contract bundle only. It does not claim portal authoring, assistant write flows, delivery acknowledgements, or enforcement runtime behavior.

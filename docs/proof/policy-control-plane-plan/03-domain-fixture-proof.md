# WP03 Domain Fixture Proof

Run id: `019ed32a-fdd2-74b0-bb81-6e152680ac97/2026-06-17T19:43:50Z`

Correlation: `policy-control-plane-plan / WP03 / policy-wp03-compiler-proof-bundle / domain-fixtures`

## Validation source

- `npm run test --workspace @ocentra-parent/policy-domain -- tests/unit/policy-compiler.test.ts tests/unit/policy-schedule-boundaries.test.ts tests/unit/policy-event.test.ts`
- `cargo test -p ocentra-policy-control-core`

## Fixture-bearing owner surfaces

- `packages/policy-domain/tests/unit/policy-compiler.test.ts` carries a browser-domain compiled artifact fixture with deterministic IDs, schedule windows, support matrix rows, explicit no-claim labels, audit refs, manual-required/unsupported rules, and rollback metadata.
- `crates/policy-control-core/tests/unit/policy_compiler.rs` carries domain-specific source documents and compiler outputs for the app/game, browser, network, tracking, screen, AI, enforcement-hint, and notification-ask-parent compiler surfaces.
- `crates/policy-control-core/tests/version-skew/policy_compiler.rs` carries round-trip fixtures for schedule payloads, explicit support matrices, status strings, and consumer-version mismatch handling.

## Domain fixture coverage

| Fixture family | Owner proof |
| --- | --- |
| App/game compiled fixture | `app_game_compiler_keeps_app_targets_ready_and_browser_targets_explicit` |
| Browser compiled fixture | `browser_compiler_keeps_site_targets_ready_and_app_targets_unsupported` and `parsePolicyCompiledArtifact: parses deterministic compiled artifacts with explicit no-claim and status metadata` |
| Network compiled fixture | `network_and_tracking_compilers_keep_geofence_and_location_targets_explicit` |
| Tracking compiled fixture | `network_and_tracking_compilers_keep_geofence_and_location_targets_explicit` |
| Screen compiled fixture | `screen_compiler_keeps_manual_required_and_unsupported_targets_explicit` and `screen_compiler_serialization_preserves_status_strings` |
| AI context compiled fixture | `ai_context_compiler_is_broadly_ready_but_enforcement_hints_stay_manual_required` |
| Enforcement-hint compiled fixture | `ai_context_compiler_is_broadly_ready_but_enforcement_hints_stay_manual_required` |
| Notification ask-parent compiled fixture | `notification_ask_parent_compiler_keeps_review_rules_ready_and_stays_deterministic` |

## Fixture integrity that is actually proven

- Schedule payloads are preserved by `compiled_artifact_serialization_preserves_schedule_payload` and `compiled_artifact_round_trips_wp07_time_boundary_schedule_payload`.
- Explicit support-matrix payloads are preserved by `compiled_artifact_round_trips_explicit_support_matrix_payload`.
- Capability reason pairs remain explicit in serialized compiler overrides via `domain_override_serialization_preserves_rule_capability_reason_pairs`.
- `compiler_artifact_has_deterministic_id_and_explicit_delivery_scope` proves compiled fixtures keep a deterministic artifact ID formula and explicit child-profile/device/domain delivery targeting.
- `compiler_artifact_preserves_audit_and_lifecycle_refs_from_source_documents` proves compiled fixtures preserve audit refs plus supersede/rollback lifecycle references from source truth.

## Honest boundary

These fixtures prove owner contract coverage for compiler outputs only. They do not prove delivery, portal rendering, assistant authoring UX, or enforcement execution.

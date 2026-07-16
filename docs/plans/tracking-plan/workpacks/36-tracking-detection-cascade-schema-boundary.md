# WP36 Tracking Detection Cascade Schema Boundary

<!-- agent-capsule -->

> Agent Capsule
> Plan: `tracking-plan`
> Doc: `WP36 Tracking Detection Cascade Schema Boundary`
> Kind: schema-boundary addendum for WP36.
> Read when: WP36 is selected before source edits or proof acceptance.
> Stop rule: use this file only to classify schema ownership; do not broaden into sibling workpacks.
> Proves: schema ownership routing only.
> Does not prove: runtime behavior, platform behavior, provider behavior, production readiness, or product-ready tracking.
> Proof rule: if WP36 source/proof changes, cite this boundary in the proof root and update the selected workpack row.

<!-- /agent-capsule -->

## Central schema rule

WP36 must use canonical cross-boundary tracking schemas from `schema-domain` or an approved neutral protocol/event/evidence boundary.

`tracking-domain` may provide helper/projection/proof adapters only. `tracking-core` may mirror canonical contracts for runtime use only. Neither package is allowed to become the silent canonical owner for public cascade payloads.

## Required proof fields

```text
canonical_schema_owner_state
observation_payload_state
detection_payload_state
analysis_payload_state
policy_handoff_state
intent_payload_state
audit_payload_state
portal_projection_state
correlation_causation_state
idempotency_state
failure_visibility_state
local_schema_rejection_state
no_ai_authority_claim
no_provider_runtime_claim
no_product_ready_claim
no_claim
```

## Acceptance rule

The WP36 proof is incomplete if a public payload, service contract, event payload, portal DTO, policy input, notification/escalation input, custody/export shape, or proof metadata shape exists only as a tracking-local schema.

## No-claim boundary

WP36 proof cannot claim physical-device behavior, provider runtime, production worker behavior, policy authority, portal runtime completion, or product-ready tracking unless those exact tiers are proved by their owning workpacks.

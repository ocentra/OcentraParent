# 05 Capability Status Contract

<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-plan`
> Doc: `05 Capability Status Contract`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR_READY, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Ownership boundary

```text
screen-plan owns local screen capability/status vocabulary and proof-tier refs.
screen-domain owns the public contract surfaces.
portal UX owns rendered presentation only when selected.
platform adapters provide capability inputs but do not own the unified status contract.
```

## Target State

Disabled, unsupported, permission-required, protected-surface, screen-locked, model-unavailable, queue-unavailable, degraded, and ready states are represented.

## Current State

Capability direction exists in docs and partial contracts, but visible product proof is open.

## Required proof fields

The selected proof must name, at minimum:

```text
platform_enum_state
status_enum_state
capture_scope_state
permission_state
protected_surface_state
screen_locked_state
model_status_state
queue_status_state
degraded_reason_state
proof_tier_state
proof_ref_state
portal_visible_state
manual_required_state
ready_state_boundary
no_ai_claim
no_live_view_product_claim
no_claim
```

These are proof-routing fields, not implementation code prescriptions.

## Checklist

- [ ] Define platform enum.
- [ ] Define status enum.
- [ ] Define capture scopes available/unavailable.
- [ ] Define degraded reasons.
- [ ] Define model status.
- [ ] Define queue status.
- [ ] Define proof tier and proof refs.
- [ ] Surface state in portal.

## Proof

- Contract tests.
- Service read-model proof.
- Portal screenshots for unavailable and ready states.

## Failure conditions

- Do not claim ready from missing permission, unsupported platform, protected-surface, model-unavailable, or queue-unavailable states.
- Do not claim AI/product-live-view readiness from screen capability status.
- Do not hide manual-required/degraded states from the portal-visible contract when the selected route touches UI.

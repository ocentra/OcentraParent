# 05 - LocalModelRuntimeStatus Hardening

<!-- agent-capsule -->

> Agent Capsule
> Plan: `ai-plan`
> Doc: `05 - LocalModelRuntimeStatus Hardening`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Target State

Runtime status is product-grade: configured, unconfigured, unavailable, loading,
loaded, generating, cancelling, timed out, degraded, failed, disabled, and last
checked states are all visible and typed.

## Where We Are

Rust service already has local AI runtime status/config/cache/readiness files and
provider proof read models. The states need to be complete and consistently
consumed.

## Checklist

- [ ] Audit current runtime status states.
- [ ] Add missing degraded/unavailable reason codes.
- [ ] Ensure status includes model artifact and provider refs.
- [ ] Expose service read model.
- [ ] Render portal state if UI changes.

## Proof

- Runtime status unit tests.
- Provider proof script output.
- Portal screenshot for any changed runtime UI state.

# 05 - LocalModelRuntimeStatus Hardening

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

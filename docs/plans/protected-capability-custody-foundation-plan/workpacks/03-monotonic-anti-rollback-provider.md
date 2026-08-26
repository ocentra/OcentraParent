# Workpack 03 - Monotonic Anti-Rollback Provider

<!-- agent-capsule -->

> Plan: `protected-capability-custody-foundation-plan`
> Workpack: `03-monotonic-anti-rollback-provider`
> Kind: hardware-backed currentness provider route.
> Proves: ownership and expected source/test boundaries only.
> Does not prove: implementation, hardware authority, tests, proof, READY, or DONE.

<!-- /agent-capsule -->

## Purpose

Separate the core Windows monotonic provider and platform anti-rollback
boundary from the already integrated WP01 mechanics. The provider must derive
current generation from the protected owner/hardware boundary and must not
restore authority from disk, SQLite, rollback state, or caller input.

## Expected production roots

```text
crates/protected-capability-custody-core/src/broker_admission/platform/windows/monotonic.rs
crates/protected-capability-custody-core/src/platform/anti_rollback.rs
```

## Expected test source

```text
crates/protected-capability-custody-core/tests/security/tpm_nv_counter.rs
```

The test obligation covers monotonic increment/currentness, reset, missing or
deleted NV state, stale generation, and fail-closed re-pair. A process-local
counter or test-only provider is not a legal substitute.

## Ownership, dependencies, and state

Protected Custody owns this provider. WP01 remains the foundation and WP02's
owner transaction remains a normal hard prerequisite. WP03 is a blocked
source-order route with expected source/tests, proof, and DONE open; no graph
state or mapped root authorizes runtime success.

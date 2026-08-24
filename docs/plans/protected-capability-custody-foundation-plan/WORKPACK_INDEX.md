# Protected Capability Custody Foundation Plan Workpack Index

<!-- agent-capsule -->

> Plan: `protected-capability-custody-foundation-plan`
> Doc: Protected Capability Custody Foundation Plan Workpack Index
> Kind: workpack selector.
> Proves: local route selection only.
> Does not prove: broker operation, tests, proof, PR readiness, or DONE.

<!-- /agent-capsule -->

Choose exactly one workpack. The source map and planned roots are deliberately
separate: existing core files are live topology, while the broker/client and
test roots are expected but missing.

| Status | Workpack | Source boundary | Required proof tier | Open condition |
| --- | --- | --- | --- | --- |
| validation / implementation-only authorization | [01 Protected Capability Custody Boundary](workpacks/01-protected-capability-custody-foundation.md) | Existing fail-closed core plus the missing isolated Windows broker/client boundary | P0 security/persistence/platform | No broker process, authenticated IPC, external factory/caller, expected tests, proof, or runtime composition exists. |

## Ownership and dependency rules

- The core owns typed custody state, binding validation, path security, storage
  replica checks, and fail-closed transitions.
- A future broker process owns protected OS authority. Its client consumes only
  authenticated typed opaque results.
- Account WP05A is the coordinator consumer. Device Trust WP01 and WP03 are
  owner-participant/ceremony consumers. Their graph edges are implementation
  ordering only; their source, tests, proof, and DONE rows remain open.
- No plan may copy the core's private constructors, mint an opaque handle, or
  replace the broker with an in-process lock/DPAPI/file path.

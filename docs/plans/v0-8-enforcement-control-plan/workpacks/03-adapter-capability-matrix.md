# 03 Adapter Capability Matrix

<!-- agent-capsule -->

> Agent Capsule
> Plan: `v0-8-enforcement-control-plan`
> Doc: `03 Adapter Capability Matrix`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

Sources: [20-step plan](../v0-8-enforcement-control-20-step-plan.md),
[test blueprint](../v0-8-enforcement-control-test-blueprint.md), and
[folder README](../README.md).

## Where We Are

Current proof distinguishes several supported, manual-required, and unavailable
states. The matrix must become the durable truth for all parent-visible claims.

## Where We Want To Be

Every platform, adapter, surface, and action has a proof level and capability
reason that service, portal, docs, and proof JSON agree on.

## Requirement Checklist

- [x] Track platform, adapter kind, surface, action, permission, dependency, and
      proof level.
- [x] Separate implemented, report-only, scaffold, unavailable, degraded, and
      manual-required.
- [x] Add tests that prevent accidental claim upgrades.
- [x] Feed the matrix into service read models and proof output.
- [x] Update feature docs/checklist when a row changes.

## Acceptance And Proof

Proof JSON and parent-visible state match the matrix exactly. Current proof runs:
`node scripts/test/v0-8-supported-adapter-runtime-proof.mjs`,
`node scripts/test/v0-8-cross-platform-enforcement-capability-proof.mjs`, and
`node scripts/test/v0-8-broad-os-adapter-runtime-proof.mjs`.
Rust owns the supported-adapter matrix contracts and consumer parity through
`crates/agent-protocol/src/enforcement_supported_adapter_runtime_proof.rs`,
`crates/agent-protocol/tests/contract/enforcement_supported_adapter_runtime_proof_tests.rs`,
`crates/agent-protocol/src/enforcement_cross_platform_capability_proof.rs`,
`crates/agent-service/src/enforcement_api/enforcement_supported_adapter_runtime_proof_read_model.rs`,
`crates/agent-service/tests/unit/enforcement_supported_adapter_runtime_proof_read_model_tests.rs`,
and `crates/agent-service/src/enforcement_cross_platform_capability_proof_read_model.rs`.
The generated TypeScript DTO surfaces are
`packages/schema-domain/src/v0-8-supported-adapter-runtime-proof.ts`,
`packages/schema-domain/src/v0-8-cross-platform-enforcement-capability-proof.ts`, and
`packages/schema-domain/src/v0-8-broad-os-adapter-runtime-proof.ts`.
Current proof artifacts live under
`output/v0-8-enforcement-control-plan-proof/03-adapter-capability-matrix/`,
`test-results/v0-8-supported-adapter-runtime-proof/`,
`test-results/v0-8-cross-platform-enforcement-capability-proof/`,
`test-results/v0-8-broad-os-adapter-runtime-proof/`, and
`docs/proof/v0-8-enforcement-control-plan/slice-05-adapter-capability-matrix.md`.
The named feature docs already reflected the manual-required, scaffold,
unavailable, degraded, and not-claimed boundaries proved by this slice, so no
additional feature-doc or product-checklist text change was required for this
closure.

## Parallel Ownership Notes

This is a shared guardrail for A. Other lanes may consume these states but must
not define their own capability truth.

## Authenticated Service-Command Boundary Addendum — 2026-08-27

The retained protocol, capability-matrix, and service read-model tests cover
direct typed states and projection invariants. PR #709 withdrew the
unauthenticated dispatcher invocation that was previously used as
service/WebSocket evidence. They do not prove
`run_agent_service` -> `ParentLocalBridgeAdmission` -> `/dev_ws`
authenticated handshake/revalidation -> `command_entry`; authenticated
service-command coverage is **manual-required**. No adapter-ready or release
claim may be inferred from the retained direct tests.

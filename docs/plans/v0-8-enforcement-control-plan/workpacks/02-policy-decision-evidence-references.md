# 02 Policy Decision Evidence References

<!-- agent-capsule -->

> Agent Capsule
> Plan: `v0-8-enforcement-control-plan`
> Doc: `02 Policy Decision Evidence References`
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

Policy preview and action-state proof exist, but every enforceable product state
must keep a visible chain back to rules, schedules, decisions, evidence, and
optional AI references.

## Where We Want To Be

No enforcement action can change device behavior unless it names the parent rule,
policy decision, target, schedule/budget, evidence refs, and validated actor.

## Requirement Checklist

- [x] Require evidence refs for action-capable states.
- [x] Preserve dry-run, observe-only, ask-parent, and unknown outcomes.
- [x] Reject stale, missing, wrong-device, or malformed decision references.
- [x] Show parent-visible reason codes.
- [x] Record all references in audit output.

## Acceptance And Proof

Tests now prove missing or invalid references produce typed rejection or unknown
state, not adapter execution. Current proof runs:
`node scripts/test/v0-8-enforcement-policy-dispatch-proof.mjs`.
Rust owns the dispatch contract and consumer validation through
`crates/agent-protocol/src/constants/v08_enforcement_policy_dispatch.rs`,
`crates/agent-protocol/src/enforcement_policy_dispatch.rs`,
`crates/agent-protocol/tests/unit/enforcement_policy_dispatch_tests.rs`,
`crates/agent-core/src/enforcement_policy_dispatch.rs`,
`crates/agent-core/tests/unit/enforcement_policy_dispatch_tests.rs`,
`crates/agent-service/src/enforcement_policy_dispatch_read_model.rs`, and
`crates/agent-service/tests/unit/enforcement_policy_dispatch_read_model_tests.rs`.
The generated TypeScript DTO surface is
`packages/schema-domain/src/enforcement-policy-dispatch.ts`.
Current proof artifacts live under
`test-results/v0-8-enforcement-policy-dispatch-proof/`,
`output/v0-8-enforcement-control-plan-proof/02-policy-decision-evidence-references/`,
and
`docs/proof/v0-8-enforcement-control-plan/slice-06-policy-decision-evidence-references.md`.
The named feature docs already described the typed policy/evidence/ask-parent
and no-claim boundaries this slice proves, so no additional feature-doc or
product-checklist text change was required for this closure.

## Parallel Ownership Notes

This workpack should move with policy/evidence contract updates. Portal surfaces
consume the chain but do not create it.

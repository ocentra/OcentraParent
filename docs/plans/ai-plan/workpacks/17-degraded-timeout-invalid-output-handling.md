# 17 - Degraded Timeout Invalid-Output Handling

<!-- agent-capsule -->

> Agent Capsule
> Plan: `ai-plan`
> Doc: `17 - Degraded Timeout Invalid-Output Handling`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Target State

AI failure is visible, safe, auditable, and policy-consumable.

## Where We Are

Runtime status and provider states exist, but model failure handling must be
consistent across all AI routes.

## Checklist

- [ ] Define degraded reason codes.
- [ ] Map timeout to degraded/unknown.
- [ ] Map overload to degraded/unknown.
- [ ] Map invalid output to rejected/degraded.
- [ ] Map unavailable model to deterministic fallback.
- [ ] Record failure in journal/read model.

## Proof

- Timeout tests.
- Overload/unavailable tests.
- Invalid-output tests.
- Portal degraded-state screenshot if UI changes.

# 04 - AI Result To Policy Gate

<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-ai-pipeline-plan`
> Doc: `04 - AI Result To Policy Gate`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Ownership boundary

```text
screen-ai-pipeline-plan owns schema-valid AI result to deterministic policy handoff proof.
ai-plan/schema-domain own AI result schema, evidence refs, confidence, and degraded semantics.
policy-control-plane-plan owns policy authority, parent-rule refs, and stricter-rule precedence.
v0-8-enforcement-control-plan owns enforcement execution and adapter authority.
```

## Target State

Only schema-valid AI results reach deterministic parent policy.

## Required proof fields

The selected proof must name, at minimum:

```text
scenario_id
ai_result_state
schema_validation_state
evidence_ref_state
parent_rule_ref_state
confidence_state
degraded_state
invalid_output_rejection_state
policy_handoff_state
policy_decision_state
stricter_rule_state
manual_required_state
action_authority_state
no_direct_ai_policy_claim
no_enforcement_claim
no_product_claim
```

These are proof-routing fields, not implementation code prescriptions.

## Checklist

- [ ] AI result cites evidence refs.
- [ ] AI result cites parent-rule refs.
- [ ] Confidence/degraded state valid.
- [ ] Invalid output rejected before policy.
- [ ] Stricter parent rule wins.

## Proof

- AI result artifact.
- Policy decision artifact.
- Invalid output rejection log.
- Parent-rule conflict proof.
- Block action handoff source artifact: `output/screen-ai-pipeline-proof/block-action-dispatch/00-screen-block-source.json`.

## Failure conditions

- Do not let invalid AI output reach policy.
- Do not let AI write policy or action state directly.
- Do not claim enforcement readiness from a policy handoff artifact.
- Do not claim policy correctness without parent-rule refs, confidence/degraded state, and invalid-output rejection proof.

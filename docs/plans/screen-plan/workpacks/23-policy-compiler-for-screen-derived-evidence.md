# 23 Policy Compiler For Screen Derived Evidence

<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-plan`
> Doc: `23 Policy Compiler For Screen Derived Evidence`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR_READY, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Ownership boundary

```text
screen-plan owns screen-derived evidence refs and summary-to-policy target contracts.
policy-control-plane-plan owns policy authority, parent-rule precedence, and compiler semantics.
v0-8-enforcement-control-plan owns runtime action execution and rollback.
screen-ai-pipeline-plan owns end-to-end AI-to-policy/action proof when selected.
```

## Target State

Visible category/risk targets compile only from validated summaries and parent rules.

## Current State

Screen policy catalog inputs exist. Summary-to-policy proof is open.

## Required proof fields

The selected proof must name, at minimum:

```text
validated_summary_state
screen_evidence_ref_state
category_target_state
risk_target_state
confidence_threshold_state
unknown_low_confidence_state
parent_rule_ref_state
raw_image_rejection_state
raw_ai_text_rejection_state
policy_compile_state
dry_run_state
enforcement_boundary_state
manual_required_state
no_policy_authority_claim
no_enforcement_claim
no_claim
```

These are proof-routing fields, not implementation code prescriptions.

## Checklist

- [ ] Define screen evidence policy targets.
- [ ] Compile category/risk rules.
- [ ] Include confidence threshold.
- [ ] Include unknown/low-confidence behavior.
- [ ] Reject raw image/raw AI text inputs.
- [ ] Add dry-run proof.

## Proof

- Policy compiler tests.
- Dry-run output proof.

## Failure conditions

- Do not compile policy from raw images or raw AI text.
- Do not claim policy authority from screen-local evidence proof.
- Do not claim enforcement execution from dry-run output.
- Do not ignore unknown/low-confidence behavior.

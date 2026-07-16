# 18 Screen Analysis Result Schema

<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-plan`
> Doc: `18 Screen Analysis Result Schema`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR_READY, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Ownership boundary

```text
screen-plan owns the screen-local analysis result shape and redaction/deletion evidence fields.
screen-ai-pipeline-plan owns end-to-end capture-to-AI-to-policy proof.
ai-plan/schema-domain own shared AI model-result contracts when selected.
policy-control-plane-plan owns policy authority and parent-rule semantics.
```

## Target State

Categories, risk signals, text snippets, redaction notes, confidence, uncertainty, evidence refs, image digest, and deletion state are schema-backed.

## Current State

Partial schema direction exists in activity-domain and docs.

## Required proof fields

The selected proof must name, at minimum:

```text
category_state
risk_signal_state
text_snippet_limit_state
redaction_note_state
confidence_state
uncertainty_reason_state
evidence_ref_state
image_digest_state
deletion_state
source_label_state
schema_validation_state
invalid_result_state
policy_authority_state
model_quality_claim_state
no_claim
```

These are proof-routing fields, not implementation code prescriptions.

## Checklist

- [ ] Define visible categories.
- [ ] Define risk signals.
- [ ] Define OCR snippets with limits.
- [ ] Define redaction notes.
- [ ] Define confidence and uncertainty reasons.
- [ ] Define image digest and source refs.
- [ ] Define deletion status.

## Proof

- Contract tests for valid/invalid results.
- Rust protocol conversion tests.

## Failure conditions

- Do not allow raw text overflow or unredacted private text in the result contract.
- Do not claim AI/model quality from schema proof.
- Do not claim policy authority from screen analysis result proof.

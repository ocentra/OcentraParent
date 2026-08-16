# 20 Result Validator And Invalid Output Handling

<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-plan`
> Doc: `20 Result Validator And Invalid Output Handling`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR_READY, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Ownership boundary

```text
screen-plan owns invalid screen-analysis output rejection and invalid/unknown screen state recording.
screen-ai-pipeline-plan owns downstream AI-to-policy path proof.
policy-control-plane-plan owns policy authority and must not receive invalid screen evidence.
```

## Target State

Invalid JSON, missing refs, invalid confidence, unsupported categories, raw text overflow, and malformed deletion state are rejected.

## Current State

Validation direction exists; complete malformed-output proof is open.

## Required proof fields

The selected proof must name, at minimum:

```text
invalid_json_state
missing_source_ref_state
invalid_confidence_state
unsupported_category_state
raw_text_overflow_state
missing_deletion_state
malformed_deletion_state
invalid_unknown_state_recorded
policy_drive_state
service_rejection_state
redaction_state
audit_or_log_state
no_policy_claim
no_enforcement_claim
no_claim
```

These are proof-routing fields, not implementation code prescriptions.

## Checklist

- [ ] Reject invalid JSON.
- [ ] Reject missing source refs.
- [ ] Reject invalid confidence.
- [ ] Reject unsupported categories.
- [ ] Reject raw text overflow.
- [ ] Reject missing/malformed deletion state.
- [ ] Record invalid/unknown state.

## Proof

- Negative contract tests.
- Service tests showing invalid output cannot drive policy.

## Failure conditions

- Do not let invalid or malformed screen analysis output drive policy.
- Do not silently coerce unsupported categories or missing evidence refs.
- Do not log raw invalid payload content without redaction.

# 03 Contract Boundary And Effect Schemas

<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-plan`
> Doc: `03 Contract Boundary And Effect Schemas`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR_READY, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Ownership boundary

```text
screen-plan owns screen settings, capability, queue, capture job, analysis result, deletion/custody, policy evidence ref, and read-model boundary definitions.
screen-domain owns public screen contracts.
schema-domain owns neutral shared contracts when shapes cross feature owners.
screen-ai-pipeline, AI, policy, enforcement, custody, and portal plans consume only typed handoffs when selected.
```

## Target State

Settings, capability, queue, capture job, analysis result, model status, deletion, policy target, and read-model contracts are schema-backed.

## Current State

Partial foundation exists in `packages/activity-domain/src/screen-evidence*.ts` and `crates/agent-protocol/src/screen_evidence.rs`.

## Required proof fields

The selected proof must name, at minimum:

```text
settings_contract_state
capability_contract_state
queue_job_contract_state
capture_job_contract_state
analysis_result_contract_state
model_status_contract_state
deletion_custody_contract_state
policy_evidence_ref_state
read_model_contract_state
schema_domain_promotion_state
malformed_payload_state
raw_runtime_string_state
manual_brand_state
cross_plan_handoff_state
no_claim
```

These are proof-routing fields, not implementation code prescriptions.

## Checklist

- [ ] Reconcile existing schemas with expectation docs.
- [ ] Add missing settings contract.
- [ ] Add missing capability/status contract.
- [ ] Add missing capture queue job contract.
- [ ] Add missing analysis result contract.
- [ ] Add missing deletion/custody contract.
- [ ] Add policy evidence ref contract.
- [ ] Add strict malformed payload tests.

## Proof

- Activity-domain tests.
- Agent-protocol tests.
- No raw app/runtime strings or manual brands.

## Failure conditions

- Do not create feature-owner cross-imports when a shared screen shape belongs in a neutral contract layer.
- Do not claim runtime capture proof from contract tests.
- Do not let malformed payloads or raw string literals become accepted contract states.

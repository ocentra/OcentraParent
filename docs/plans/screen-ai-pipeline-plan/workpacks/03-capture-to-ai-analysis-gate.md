# 03 - Capture To AI Analysis Gate

<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-ai-pipeline-plan`
> Doc: `03 - Capture To AI Analysis Gate`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Ownership boundary

```text
screen-ai-pipeline-plan owns capture-ref to AI-context/router/result integration proof.
screen-plan owns capture/evidence source, protected-surface behavior, and screen custody boundaries.
ai-plan/schema-domain own AI context/result schema, provider/runtime, and degradation semantics.
data-custody-storage-plan owns queue/image retention and deletion behavior.
```

## Target State

Captured evidence flows into OCR, guided VLM, local text model, or deterministic analysis through the AI queue/router.

## Required proof fields

The selected proof must name, at minimum:

```text
scenario_id
capture_ref_state
evidence_digest_state
ai_context_state
provider_route_state
ocr_route_state
vlm_route_state
local_text_route_state
deterministic_route_state
model_runtime_state
ai_result_state
degraded_unknown_state
queue_job_state
queue_deletion_state
redaction_state
policy_eligibility_state
no_policy_authority_claim
no_product_claim
```

These are proof-routing fields, not implementation code prescriptions.

## Checklist

- [ ] Capture ref enters AI context builder.
- [ ] Provider route recorded.
- [ ] OCR runs when text can answer.
- [ ] VLM runs only for guided visual classification.
- [ ] Text model consumes typed context only.
- [ ] Deterministic route skips model when structured evidence is enough.

## Proof

- AI context artifact.
- Route/runtime artifact.
- AI result artifact.
- Degraded/unknown proof where expected.
- `output/screen-ai-pipeline-proof/service-analysis/proof-summary.json` proves the service-owned encrypted queue job, adapter runtime route, `localVision` read-model row, evidence digest, policy eligibility, and queue deletion path for one captured active-window job.
- `output/screen-ai-pipeline-proof/service-native-game-analysis/proof-summary.json` proves a service-owned native foreground active-window capture can flow through the service-owned local adapter analysis runtime into a `localVision` game read-model row, preserving the queue job, capture reason, active-window scope, digest, policy eligibility, and queue deletion path.
- `output/screen-ai-pipeline-proof/ocr-route/proof-summary.json`.
- `output/screen-ai-pipeline-proof/local-text-route/proof-summary.json`.
- `output/screen-ai-pipeline-proof/deterministic-route/proof-summary.json`.
- `output/ai-plan-proof/real-analysis/proof-summary.json`.

## Failure conditions

- Do not claim policy authority from AI analysis proof.
- Do not claim model/provider readiness from deterministic-route proof.
- Do not claim product readiness without queue deletion, redaction, and degraded/unknown proof.
- Do not let VLM run where OCR or deterministic evidence is sufficient without route justification.

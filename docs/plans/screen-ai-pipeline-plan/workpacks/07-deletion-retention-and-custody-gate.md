# 07 - Deletion Retention And Custody Gate

<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-ai-pipeline-plan`
> Doc: `07 - Deletion Retention And Custody Gate`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Ownership boundary

```text
screen-ai-pipeline-plan owns scenario proof for raw image deletion, queue encryption, redaction, and no-remote-upload behavior inside the pipeline.
data-custody-storage-plan owns product retention/export/delete/privacy policy and parent retention controls.
screen-plan owns screen capture/screenshot custody settings.
ai-plan owns AI output custody only through typed result/citation handoffs.
```

## Target State

Raw image custody is explicit. Default behavior deletes temporary images and does not upload raw screenshots remotely.

## Required proof fields

The selected proof must name, at minimum:

```text
scenario_id
queue_encryption_state
raw_path_redaction_state
delete_success_state
delete_after_ttl_state
delete_failure_visible_state
remote_upload_state
retention_mode_state
retention_opt_in_state
unsupported_retention_state
custody_policy_state
expired_record_state
activity_row_state
ai_result_custody_state
proof_artifact_redaction_state
no_raw_remote_upload_claim
no_product_retention_claim
no_claim
```

These are proof-routing fields, not implementation code prescriptions.

## Checklist

- [ ] Queue image encrypted.
- [ ] Raw path redacted outside child agent.
- [ ] Delete after success.
- [ ] Delete after TTL.
- [ ] Delete failure visible.
- [ ] Remote/cloud screenshot upload disabled.
- [ ] Retention requires explicit opt-in if used.

## Proof

- Queue encryption artifact.
- Deletion proof artifact.
- Remote disabled proof.
- Retention non-claim or opt-in proof.
- Current proof artifacts:
  `output/screen-ai-pipeline-proof/service-cadence/proof-summary.json`,
  `output/screen-ai-pipeline-proof/service-analysis/proof-summary.json`,
  `output/screen-ai-pipeline-proof/service-retention-sweeper/proof-summary.json`,
  `output/ai-plan-proof/real-analysis/proof-summary.json`, and
  `output/screen-ai-pipeline-proof/live-operator/proof-summary.json`.
- TTL/delete-failure/retention-mode contract proof: `output/screen-ai-pipeline-proof/deletion-retention-custody/proof-summary.json`.
- The service retention sweeper proof removes an expired encrypted queue record and records an `expiredDeleted` Activity Screen row through the real Rust service/WebSocket path. The retention proof rejects unsupported raw screenshot retention. Production parent UI retention controls remain a non-claim.

## Failure conditions

- Do not claim custody readiness without deletion/TTL/failure-visible proof.
- Do not claim parent retention controls from pipeline deletion proof.
- Do not upload raw screenshots remotely by default.
- Do not expose raw paths outside the child agent/proof boundary.

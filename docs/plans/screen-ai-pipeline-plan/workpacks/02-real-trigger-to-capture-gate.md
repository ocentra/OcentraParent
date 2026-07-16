# 02 - Real Trigger To Capture Gate

<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-ai-pipeline-plan`
> Doc: `02 - Real Trigger To Capture Gate`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Ownership boundary

```text
screen-ai-pipeline-plan owns trigger-to-capture scenario proof and structured skip proof.
screen-plan owns capture primitives, protected-surface behavior, disclosure, and screen settings.
browser/app-game/network/tracking plans own source-trigger truth when their domain trigger is selected.
data-custody-storage-plan owns deletion/retention/export proof for raw image artifacts.
```

## Target State

Real browser, app, game, unknown-process, manual, and cadence triggers produce a real capture job or a real structured-skip proof.

## Required proof fields

The selected proof must name, at minimum:

```text
scenario_id
trigger_type
trigger_source_owner
capture_setting_state
capture_job_state
queue_growth_state
queue_record_state
backpressure_state
disabled_setting_state
structured_skip_state
deleted_image_state
ephemeral_ref_state
classification_boundary_state
platform_state
manual_required_state
no_ai_analysis_claim
no_policy_claim
no_product_claim
```

These are proof-routing fields, not implementation code prescriptions.

## Checklist

- [ ] Browser social/video trigger.
- [ ] Browser education/video trigger.
- [ ] Browser social/feed trigger.
- [ ] Browser game/cloud-game trigger.
- [ ] Native app foreground trigger.
- [ ] Native game/controlled game trigger.
- [ ] Unknown process/app trigger.
- [ ] Timed cadence trigger.
- [ ] Disabled setting prevents new jobs.

## Proof

- Trigger input artifact: `output/screen-ai-pipeline-proof/service-foreground/proof-summary.json` for native foreground and `output/screen-ai-pipeline-proof/service-cadence/proof-summary.json` for cadence.
- Capture job artifact: `output/screen-ai-pipeline-proof/service-foreground/queue-records.json` and `output/screen-ai-pipeline-proof/service-cadence/queue-records.json`.
- Queue proof artifact: foreground proof requires queue growth after a native Notepad foreground action; cadence proof requires three queued timed captures plus pending-queue backpressure.
- Deletion proof artifact: both service proofs require `imageDeletionState: deleted` and sanitized `<ephemeral-screen-queue>` evidence refs.
- Controlled native game proof: `output/screen-ai-pipeline-proof/native-game/03-capture-proof.json`.
- Service native game classification proof: `output/screen-ai-pipeline-proof/service-native-game-analysis/proof-summary.json`. This proves the service foreground runtime can capture a controlled native game-like active window and the service analysis runtime can classify the encrypted queue job as `game`; dedicated installed-game identity detection remains app/game evidence scope.
- Unknown native process proof: `output/screen-ai-pipeline-proof/unknown-native-process/03-capture-proof.json`.
- Disabled no-capture/no-AI proof: `output/screen-ai-pipeline-proof/service-disabled-suppression/proof-summary.json`. The proof starts from one enabled encrypted cadence queue record, then runs cadence, foreground, and analysis service runtimes with the parent disabled setting off and requires no new capture rows, no new queue records, no local vision row, and no pending queue drain.

## Failure conditions

- Do not claim AI analysis from trigger-to-capture proof.
- Do not claim policy/action readiness from capture queue proof.
- Do not claim product trigger readiness from mock-only or placeholder trigger rows.
- Do not close the disabled-setting row unless no new capture rows, no new queue records, no local vision row, and no pending queue drain are proven.

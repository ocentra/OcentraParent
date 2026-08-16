# 09 - Performance Cadence And Backpressure Gate

<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-ai-pipeline-plan`
> Doc: `09 - Performance Cadence And Backpressure Gate`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Target State

Timed cadence and repeated AI analysis are bounded and visible.

## Checklist

- [ ] Short test cadence configured by explicit service opt-in setting.
- [ ] At least three captures recorded.
- [ ] Actual timestamps recorded.
- [ ] Queue backpressure/debounce recorded. The service proof caps pending queue records at three and verifies no fourth row appears after the queue is full.
- [ ] Repeated AI analysis does not flood model runtime. Current service analysis proof uses explicit opt-in, `max_jobs`, `max_ticks`, poll cadence, and adapter timeout to bound service-owned model dispatch, while broader multi-job production VLM stress remains a follow-up proof item.
- [ ] Disable stops future cadence jobs at scheduler/service unit boundary and
      service runtime proof boundary.

## Proof

- `output/screen-ai-pipeline-proof/service-cadence/proof-summary.json`
- `output/screen-ai-pipeline-proof/service-cadence/queue-records.json`
- `output/screen-ai-pipeline-proof/service-cadence/screen-read-model.json`
- `output/screen-ai-pipeline-proof/service-analysis/proof-summary.json`
- `output/screen-ai-pipeline-proof/service-disabled-suppression/proof-summary.json`
- `cargo test -p ocentra-parent-agent-service screen_ai_cadence_runtime -- --nocapture`
- `cargo test -p ocentra-parent-agent-service screen_ai_analysis_runtime -- --nocapture`
- `node --check scripts/test/screen-ai-service-disabled-suppression-proof.mjs`
- `node scripts/test/screen-ai-service-disabled-suppression-proof.mjs`

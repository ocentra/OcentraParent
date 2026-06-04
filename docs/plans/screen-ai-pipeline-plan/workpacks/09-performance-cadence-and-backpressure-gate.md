# 09 - Performance Cadence And Backpressure Gate

## Target State

Timed cadence and repeated AI analysis are bounded and visible.

## Checklist

- [x] Short test cadence configured by explicit service opt-in setting.
- [x] At least three captures recorded.
- [x] Actual timestamps recorded.
- [x] Queue backpressure/debounce recorded. The service proof caps pending queue records at three and verifies no fourth row appears after the queue is full.
- [x] Repeated AI analysis does not flood model runtime. Current service analysis proof uses explicit opt-in, `max_jobs`, `max_ticks`, poll cadence, and adapter timeout to bound service-owned model dispatch, while broader multi-job production VLM stress remains a follow-up proof item.
- [x] Disable stops future cadence jobs at scheduler/service unit boundary and
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

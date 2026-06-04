# 02 - Real Trigger To Capture Gate

## Target State

Real browser, app, game, unknown-process, manual, and cadence triggers produce a
real capture job or a real structured-skip proof.

## Checklist

- [ ] Browser social/video trigger.
- [ ] Browser education/video trigger.
- [ ] Browser social/feed trigger.
- [ ] Browser game/cloud-game trigger.
- [x] Native app foreground trigger.
- [ ] Native game/controlled game trigger.
- [ ] Unknown process/app trigger.
- [x] Timed cadence trigger.
- [ ] Disabled setting prevents new jobs.

## Proof

- Trigger input artifact: `output/screen-ai-pipeline-proof/service-foreground/proof-summary.json`
  for native foreground and `output/screen-ai-pipeline-proof/service-cadence/proof-summary.json`
  for cadence.
- Capture job artifact: `output/screen-ai-pipeline-proof/service-foreground/queue-records.json`
  and `output/screen-ai-pipeline-proof/service-cadence/queue-records.json`.
- Queue proof artifact: foreground proof requires queue growth after a native
  Notepad foreground action; cadence proof requires three queued timed captures
  plus pending-queue backpressure.
- Deletion proof artifact: both service proofs require `imageDeletionState:
deleted` and sanitized `<ephemeral-screen-queue>` evidence refs.

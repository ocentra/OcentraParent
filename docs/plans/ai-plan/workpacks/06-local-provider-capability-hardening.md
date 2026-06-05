# 06 - LocalProviderCapability Hardening

## Target State

Provider capability reports supported tasks, privacy mode, model formats,
resource class, hardware fit, fallback order, and unavailable/degraded reasons.

## Where We Are

Provider scheduler proof exists and now includes one runtime access lane per
physical device. The tracked runtime/scheduler proof artifacts show
`runtimeAccessLaneCount=1`, independent Rust lanes for separate physical
devices, child-safety priority over parent-assistant jobs on the same lane,
queued/degraded/unavailable states, and no duplicate same-device model load.
Provider capability must still become the shared gate for
deterministic/text/OCR/VLM/embedding/remote routes.

## Checklist

- [ ] Type provider capability by task.
- [ ] Include local-only and remote-disabled states.
- [ ] Include resource class and hardware fit.
- [ ] Include fallback order.
- [ ] Add provider route rejection for unsupported tasks.

## Proof

- Provider capability contract tests.
- Local provider runtime/scheduler proof artifacts:
  `output/ai-plan-proof/local-ai-runtime-provider-proof/proof.json` and
  `output/ai-plan-proof/local-ai-provider-scheduler-proof/proof.json`.
- Route selection tests.
- Remote disabled-by-default test.

# 06 - LocalProviderCapability Hardening

<!-- agent-capsule -->

> Agent Capsule
> Plan: `ai-plan`
> Doc: `06 - LocalProviderCapability Hardening`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

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

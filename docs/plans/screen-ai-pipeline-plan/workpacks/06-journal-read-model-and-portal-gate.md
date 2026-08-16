# 06 - Journal Read Model And Portal Gate

<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-ai-pipeline-plan`
> Doc: `06 - Journal Read Model And Portal Gate`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Target State

Journal, SQLite read model, and parent portal show the complete trigger,
capture, AI, policy, action/dry-run, and deletion chain.

## Checklist

- [ ] Journal entry contains trigger ref.
- [ ] Journal entry contains capture ref.
- [ ] Journal entry contains AI result ref.
- [ ] Journal entry contains policy decision ref.
- [ ] Read model replays the chain.
- [ ] Portal screenshot shows explanation.

## Proof

- Journal/read model artifact.
- Portal screenshot.
- Parent explanation artifact.
- Current proof artifact:
  `output/screen-ai-pipeline-proof/portal-chain/proof-summary.json`.
- Current screenshot artifact:
  `output/screen-ai-pipeline-proof/portal-chain/parent-portal-screen-chain.png`.

# 15 - Local Text LLM Execution Dry-Run Adapter

<!-- agent-capsule -->

> Agent Capsule
> Plan: `ai-plan`
> Doc: `15 - Local Text LLM Execution Dry-Run Adapter`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Target State

Local text inference can run in dry-run for safety support with typed input,
bounded output, timeout, cancellation, and invalid-output rejection.

## Where We Are

Generation request/result/runner files exist in Rust service. The dry-run safety
adapter must bind those pieces to AI context and result parsing.

## Checklist

- [ ] Wire context-builder output to generation request.
- [ ] Add timeout and cancellation.
- [ ] Parse output into result candidate.
- [ ] Reject invalid output.
- [ ] Journal dry-run result.
- [ ] Keep enforcement disabled.

## Proof

- Local dry-run proof script.
- Timeout test.
- Invalid output rejection test.
- Dry-run policy integration test.

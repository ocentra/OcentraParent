# 46 - Security Privacy Negative Gates Lane

<!-- agent-capsule -->

> Agent Capsule
> Plan: `ai-plan`
> Doc: `46 - Security Privacy Negative Gates Lane`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Target State

Negative tests prove AI cannot violate custody, direct-scan, direct-enforcement,
remote, screenshot, memory, graph, or prompt-minimization boundaries.

## Where We Are

Expectations define the boundaries. The test suite must enforce them before
runtime behavior expands.

## Checklist

- [ ] AI no direct OS scan test.
- [ ] AI no direct browser/network/screen scan test.
- [ ] AI no direct enforcement test.
- [ ] Remote disabled-by-default test.
- [ ] Raw screenshot API guard test.
- [ ] Unsourced memory/graph rejected tests.
- [ ] Prompt minimization test.
- [ ] Custody label guard test.

## Proof

- Security test suite output.
- No test doubles.
- Validation gate includes negative tests.

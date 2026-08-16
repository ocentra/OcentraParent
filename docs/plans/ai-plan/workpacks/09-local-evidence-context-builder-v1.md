# 09 - Local Evidence Context Builder V1

<!-- agent-capsule -->

> Agent Capsule
> Plan: `ai-plan`
> Doc: `09 - Local Evidence Context Builder V1`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Target State

The context builder assembles the smallest relevant stored evidence window,
parent rules, runtime refs, prompt refs, and evidence-backed memory refs.

## Where We Are

The architecture spec exists and parent-domain context builder contracts exist.
The next proof must use real stored evidence and rules.

## Checklist

- [ ] Build from SQLite/read-model evidence.
- [ ] Include parent rule context.
- [ ] Include runtime/provider refs.
- [ ] Include prompt/template version.
- [ ] Return ready, partial, insufficient, unavailable, or rejected.
- [ ] Reject raw sources and invalid custody.

## Proof

- Stored-evidence integration test.
- Context minimization test.
- Custody rejection test.

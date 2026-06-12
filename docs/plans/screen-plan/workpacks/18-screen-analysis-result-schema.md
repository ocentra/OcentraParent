# 18 Screen Analysis Result Schema

<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-plan`
> Doc: `18 Screen Analysis Result Schema`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Target State

Categories, risk signals, text snippets, redaction notes, confidence, uncertainty, evidence refs, image digest, and deletion state are schema-backed.

## Current State

Partial schema direction exists in activity-domain and docs.

## Checklist

- [ ] Define visible categories.
- [ ] Define risk signals.
- [ ] Define OCR snippets with limits.
- [ ] Define redaction notes.
- [ ] Define confidence and uncertainty reasons.
- [ ] Define image digest and source refs.
- [ ] Define deletion status.

## Proof

- Contract tests for valid/invalid results.
- Rust protocol conversion tests.

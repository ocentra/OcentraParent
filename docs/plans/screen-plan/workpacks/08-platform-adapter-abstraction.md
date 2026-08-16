# 08 Platform Adapter Abstraction

<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-plan`
> Doc: `08 Platform Adapter Abstraction`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Target State

Capture adapter interface exists for Windows, macOS, Linux, Android, iOS, and fake/dev adapters with capability and proof tiers.

## Current State

Platform implementation proof is open.

## Checklist

- [ ] Define adapter ID and platform contract.
- [ ] Define capability probe.
- [ ] Define capture request/result.
- [ ] Define protected/unavailable result.
- [ ] Define proof tier.
- [ ] Ensure fake/dev adapter cannot be product proof.

## Proof

- Adapter contract tests.
- Proof tier mapping in platform deep dive.

# 33 - Browser URL Video AI Lane

<!-- agent-capsule -->

> Agent Capsule
> Plan: `ai-plan`
> Doc: `33 - Browser URL Video AI Lane`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Target State

Managed browser URL, title, metadata, transcript summaries, and video refs feed
AI as typed evidence. AI does not read browser state directly.

## Where We Are

Browser plan owns URL/video AI planning. This workpack links that slice into the
shared AI queue/context/result/policy spine.

## Checklist

- [ ] Consume managed browser evidence only.
- [ ] Add URL/video context mapping.
- [ ] Add deterministic URL/platform classification first.
- [ ] Route ambiguous cases to local text model.
- [ ] Return schema-valid category/support result.
- [ ] Feed parent policy, not direct enforcement.

## Proof

- Browser URL AI dry-run test.
- Video metadata classification test.
- Unmanaged browser exact-URL rejection test.

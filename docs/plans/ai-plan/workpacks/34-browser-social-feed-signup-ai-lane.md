# 34 - Browser Social Feed Signup AI Lane

<!-- agent-capsule -->

> Agent Capsule
> Plan: `ai-plan`
> Doc: `34 - Browser Social Feed Signup AI Lane`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Target State

Social account creation, feed, messaging, upload, livestream, and short-video
signals are classified from typed managed browser evidence and parent rules.

## Where We Are

Browser social planning exists. AI must not infer social content without managed
browser, metadata, OCR, or screen summary proof.

## Checklist

- [ ] Consume typed managed browser/social evidence.
- [ ] Add signup/feed/account route refs.
- [ ] Include parent policy targets.
- [ ] Route screen OCR/VLM only when capture scope allows.
- [ ] Return confidence/degraded states.

## Proof

- Social signup AI dry-run test.
- Feed/shorts degraded state test.
- Policy cannot rely on unsourced social claim.

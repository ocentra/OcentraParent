# 36 - App Game Unknown Classifier Lane

## Target State

Unknown apps and games classify from stored app/game evidence, catalog refs,
launcher refs, session summaries, and optional screen summaries.

## Where We Are

App/game evidence and plans exist. AI must not scan processes or infer duration;
it reads agent-generated summaries and typed evidence.

## Checklist

- [ ] Consume app/game evidence refs.
- [ ] Use deterministic catalog first.
- [ ] Include launcher-only and unknown states.
- [ ] Include session duration summaries from agent evidence.
- [ ] Use screen summary only when approved.
- [ ] Return category/risk evidence with confidence.

## Proof

- Unknown app AI dry-run test.
- Unknown game AI dry-run test.
- Duration-not-model-output test.

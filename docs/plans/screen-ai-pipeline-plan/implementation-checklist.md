# Screen AI Pipeline Implementation Checklist

A checkbox may be marked `[x]` only after the matching artifact exists under:

```text
output/screen-ai-pipeline-proof/
```

## Prerequisite Gates

- [ ] Screen capture proof PR merged or explicitly stacked.
- [ ] AI analysis proof PR merged or explicitly stacked.
- [ ] Pipeline branch contains both prerequisite implementations.
- [ ] Prerequisite commits recorded in proof artifacts.

## Real Trigger Gates

- [ ] Managed browser social/video trigger.
- [ ] Managed browser education/video trigger.
- [ ] Managed browser social/feed trigger.
- [ ] Managed browser game/cloud-game trigger.
- [ ] Native app foreground trigger.
- [ ] Native game or controlled game-window trigger.
- [ ] Unknown process/app trigger.
- [ ] Timed cadence trigger.
- [ ] Disabled setting suppression.
- [ ] Protected/permission-required skip.

## AI Analysis Gates

- [ ] OCR route proof where visible text is enough.
- [ ] Guided VLM route proof where visual classification is needed.
- [ ] Local text model route proof over typed context.
- [ ] Deterministic route proof where structured evidence is enough.
- [ ] Low confidence degrades safely.
- [ ] Invalid output cannot reach policy.

## Policy And Action Gates

- [ ] Observe policy result.
- [ ] Allow policy result.
- [ ] Warn policy result.
- [ ] Ask-parent policy result.
- [ ] Time-limit policy result.
- [ ] Block dry-run or real adapter result.
- [ ] Unknown/manual-required result.
- [ ] AI cannot override stricter parent rule.

## Portal And Proof Gates

- [ ] Journal/read model contains trigger, capture, AI, policy, and deletion refs.
- [ ] Parent portal screenshot shows the full chain.
- [ ] Parent explanation cites evidence and rules.
- [ ] Raw image deletion proof linked.
- [ ] Remote/cloud screenshot upload disabled.
- [ ] Operator live proof completed before product-complete claim.

## Validation

- [ ] Screen focused validation rerun on pipeline branch.
- [ ] AI focused validation rerun on pipeline branch.
- [ ] Pipeline E2E/proof script run.
- [ ] Playwright screenshot proof run.
- [ ] Security negative tests run.
- [ ] Performance/cadence proof run.
- [ ] `git diff --check`.
- [ ] lane/hub guards.
- [ ] `npm run validate` or approved omission.

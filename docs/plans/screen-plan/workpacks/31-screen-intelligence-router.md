# 31 Screen Intelligence Router

<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-plan`
> Doc: `31 Screen Intelligence Router`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Target State

Router chooses the cheapest safe route before capture, OCR, VLM, trusted household mesh provider, or remote redacted-only fallback.

## MVP Boundary

This workpack is required for the capture MVP. It does not select the final OCR/VLM model.

## Checklist

- [ ] Define route enum.
- [ ] Define route input contract.
- [ ] Check browser/app/game/network/session evidence before capture.
- [ ] Route managed browser to structured extraction first.
- [ ] Route native app/game/launcher/unknown process to active-window or selected-window path when allowed.
- [ ] Return manual-required/unavailable when capture is not allowed.
- [ ] Add policy question and sensitivity fields.

## Proof

- Contract tests for route decisions.
- Router examples for browser, native game, launcher, unknown process, and no-screen-needed.

Current proof: `output/screen-plan-proof/31-32-screen-router-structured-extraction/proof-summary.json`.

Non-claims: this is a contract/proof route planner only. It does not claim live managed-browser producer integration,
real DOM/accessibility capture, screenshot capture, portal UI, policy execution, enforcement, or product-complete
pipeline closure.

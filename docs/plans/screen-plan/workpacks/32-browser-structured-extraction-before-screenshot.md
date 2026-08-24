# 32 Browser Structured Extraction Before Screenshot

<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-plan`
> Doc: `32 Browser Structured Extraction Before Screenshot`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Target State

Managed-browser URL/title/meta/DOM/accessibility signals answer first when possible.

## MVP Boundary

This is part of capture MVP because it prevents unnecessary screenshots.

## Checklist

- [x] Define managed browser structured extraction contract.
- [ ] Capture URL/title/meta safely.
- [ ] Capture DOM visible text with strict limits.
- [x] Prefer platform/domain parser before image capture.
- [x] Redact private content and raw DOM overflow.
- [ ] Produce `no_screen_needed` when structured evidence is enough.

Source status: 3/6 rows are implemented as a fail-closed prerequisite. The
current producer emits bounded URL/body identities and redaction/sensitivity
digests, but deliberately exposes no title, metadata, accessibility values, or
visible DOM text because no browser/policy-owned safe-disclosure authority is
mounted. Unknown or protected sensitivity cannot authorize a screenshot or
`no_screen_needed`. The complete function-level path has no executable service
caller, so this is not runtime completion.

## Proof

- Tests showing screenshot is skipped when URL/title/DOM answer policy question.
- Portal/read-model evidence refs show structured source.

Historical proof: `output/screen-plan-proof/31-32-screen-router-structured-extraction/proof-summary.json` covers the
older router contract only. The current source packet has no accepted proof at

Current reviewed topology is bounded to the existing schema/parser/runtime/
router/capture roots listed in this workpack. The composition roots
`crates/agent-service/src/screen_managed_browser_cdp_runtime.rs` and
`crates/agent-service/src/screen_managed_browser_structured_route_composition.rs`,
and the expected focused test/proof roots, remain missing. This is unwired
source with no producer custody claim.
`output/screen-plan-proof/32-browser-structured-extraction-before-screenshot/`.

Non-claims: this is reviewed production source only. It does not claim an
executable managed-browser producer-to-router caller, safe disclosure of DOM or
accessibility content, `no_screen_needed`, focused tests, portal read-model
rendering, live screenshots, policy execution, enforcement, retained proof, or
product-complete pipeline closure.

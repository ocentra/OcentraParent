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

## Canonical source-only refresh (2026-08-18)

The source packet is present at canonical `b4c3a921b` through commits
`cc7b0df6d`, `484ff4d18`, `fdc4ccf29`, and `b4c3a921b`. The changed
`screen-ai-core` router now prefers structured extraction, seals extraction
authority, requires a producer receipt, and rejects unsafe screenshot
fallback. This is implementation evidence only: the existing router test was
not revalidated by this packet, and the real managed-browser producer/caller,
focused extraction/fallback/receipt tests, proof, and DONE remain open.

## Checklist

- [ ] Define managed browser structured extraction contract.
- [ ] Capture URL/title/meta safely.
- [ ] Capture DOM visible text with strict limits.
- [ ] Prefer platform/domain parser before image capture.
- [ ] Redact private content and raw DOM overflow.
- [ ] Produce `no_screen_needed` when structured evidence is enough.

## Proof

- Tests showing screenshot is skipped when URL/title/DOM answer policy question.
- Portal/read-model evidence refs show structured source.

Current proof: `output/screen-plan-proof/31-32-screen-router-structured-extraction/proof-summary.json`.

Non-claims: this is bounded structured-evidence contract proof only. It does not claim real managed-browser DOM or
accessibility producer runtime, portal read-model rendering, live screenshots, policy execution, enforcement, or
product-complete pipeline closure.

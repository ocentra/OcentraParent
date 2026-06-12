# WP159 - App/game malicious metadata safety gate

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP159 - App/game malicious metadata safety gate`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Scope

Close the app/game merge-blocking raw path leakage and malicious metadata
gates for the parent dashboard surface without taking ownership of E-A's
shared renderer lock.

## Implementation

- The existing app/game dashboard fixture keeps a long script-like native app
  label as a manual-required/risk row instead of promoting or dropping it.
- The same fixture proves raw executable path refs remain absent from the
  parent dashboard intent and rendered malicious-label proof.
- The focused portal test renders the label through React SVG text and proves
  script markers are escaped text, not executable markup.
- The same test anchors the proof to the existing shared parent dashboard
  renderer by checking that app/game row labels are rendered through bounded
  text sizing/truncation and that the renderer has no dashboard
  `dangerouslySetInnerHTML` sink.

## No-Claim Boundary

This is a UI safety and negative-proof gate. It does not claim classifier
provider execution, metadata trust, policy enforcement, adapter dispatch,
broad blocking, platform enforcement, or raw private source-row visibility.
The central product capability checklist remains untouched while E-B owns that
lock.

## Validation

See
`output/app-game-plan-proof/159-app-game-malicious-metadata-safety-gate/10-validation-commands.log`.

# WP159 - App/game malicious metadata safety gate

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

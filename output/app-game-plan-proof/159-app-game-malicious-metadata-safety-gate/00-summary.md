# WP159 - App/game malicious metadata safety gate

## Branch

`codex/app-game-control-product-completion`

## Scope

WP159 closes the app/game merge-blocking raw path leakage and malicious
metadata gates for the parent dashboard surface. It proves raw executable path
refs stay out of the parent dashboard intent/rendered malicious-label proof,
and that a long script-like native app display label remains parent-visible
text in the app/game dashboard data path without becoming executable markup or
breaking the bounded SVG label path.

## Runtime Boundary

- The test uses the existing service-backed app-use/games dashboard intent.
- The malicious app label remains manual-required and risk-candidate, keeping
  the parent review state visible.
- Raw executable path refs stay absent from the dashboard intent and rendered
  malicious-label proof.
- React SVG text rendering escapes the script-like label.
- The shared parent dashboard renderer is not edited because E-A owns that
  file; the proof checks the renderer source still uses bounded
  `row.label` text/truncation and no dashboard HTML injection sink.

## No-Claim Boundaries

- No classifier/provider execution.
- No metadata trust upgrade.
- No policy enforcement.
- No adapter dispatch.
- No broad blocking.
- No platform enforcement.
- No raw private executable path or source-row visibility.
- `docs/product-capability-checklist.md` intentionally remains untouched.

## Validation

See `10-validation-commands.log`.

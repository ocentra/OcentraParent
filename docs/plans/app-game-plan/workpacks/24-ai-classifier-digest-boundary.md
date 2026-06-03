# 24 AI Classifier Digest Boundary

## Target State

AI consumes stored app/game evidence or structured digests and returns
classification candidates only.

## Scope

- App/game AI digest refs.
- Unknown app/game classification result.
- Category/risk candidates with source/confidence.
- Model/runtime/prompt-template refs.
- No direct action authority.

## Tests And Proof

- Missing evidence refs rejected.
- Confidence outside `0..1` rejected.
- Block/terminate/hide/suspend/shield fields rejected.
- Duration field in AI output rejected.
- Raw OS scan result in AI output rejected.

## Done Signal

AI can help classify unknown apps/games without scanning the OS or enforcing.

Use the standard checklist in [workpacks README](README.md).

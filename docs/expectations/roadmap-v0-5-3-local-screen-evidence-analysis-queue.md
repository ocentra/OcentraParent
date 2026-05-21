# V0.5.3 Local Screen Evidence Analysis Queue Expectations

This is the milestone-specific expectation file for V0.5.3 in `docs/product-roadmap.md`.

Supporting expectation files: [screen evidence](screen-evidence.md), [capture](capture.md), [evidence storage](evidence-storage.md), [AI](ai.md), [policy](policy.md), [enforcement](enforcement.md), [portal](portal.md), [platforms](platforms.md), and [platform deliverables](platform-deliverables.md).

## Outcome

- Optional local screen evidence is disabled by default and parent controlled.
- Temporary images are encrypted in a local queue, summarized by local OCR/vision, then deleted according to TTL/deletion state.
- Policy consumes schema-valid summaries and evidence refs, not retained screenshots or raw AI text.
- Screen capture, foreground-window targeting, and OCR/vision runtime
  availability are platform capability states before policy or AI can use them.

## Acceptance

- Screen images do not leave the child PC for remote/API AI or Ocentra-hosted processing.
- Queue, summary, confidence, category, source evidence refs, image digest, deletion state, cadence, trigger, and retention settings are typed.
- Portal shows enablement, cadence, triggers, retention/deletion, capability status, and summary state clearly.
- Unsupported or permission-limited platforms report unavailable/degraded screen
  evidence rather than silently falling back to guessed activity.

## Validation

- Run `npm run validate`.
- Include queue encryption tests, schema validation tests, Rust read-model tests, and portal state checks.

# V0.5 Live Activity Portal Expectations

This is the milestone-specific expectation file for V0.5 in `docs/product-roadmap.md`.

Supporting expectation files: [portal](portal.md), [evidence storage](evidence-storage.md), [contracts](contracts.md), and [documentation](documentation.md).

## Outcome

- The local parent visibility surface connects to the real Rust service and shows health, evidence-store, activity, source, and diagnostics state.
- Portal controls send typed intents or queries and never execute child-device work directly.
- Copy/debug output is useful for handoff without exposing secrets or raw private content.

## Acceptance

- One primary result/timeline/table surface updates predictably instead of appending fake cards.
- Empty, loading, stale, degraded, and failure states are visible and not confused with successful data.
- Browser-visible warnings/errors on touched portal routes are treated as product issues.

## Validation

- Run `npm run validate`.
- Include Playwright coverage against the real service path plus browser console checks for touched routes.

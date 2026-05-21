# V0.3 Windows Process And Window Activity Capture Expectations

This is the milestone-specific expectation file for V0.3 in `docs/product-roadmap.md`.

Supporting expectation files: [capture](capture.md), [evidence storage](evidence-storage.md), [contracts](contracts.md), [portal](portal.md), and [platforms](platforms.md).

## Outcome

- Windows process and foreground-window observations are captured as typed evidence without blocking, AI decisions, or content inspection.
- Process/window evidence is journaled, ingested, and queryable before portal display.
- Unsupported, unavailable, access-denied, and degraded states are represented honestly.

## Acceptance

- A real Windows run can observe process/window activity and preserve source, adapter, timestamp, and capability state.
- The system does not claim browser URL, page content, chat content, keystrokes, screenshots, or decrypted traffic from V0.3 evidence.
- Portal rows distinguish observed process/window facts from unknown or unsupported states.

## Validation

- Run `npm run validate`.
- Include focused Rust adapter/mapping tests and a manual Windows local run before claiming parent-visible behavior.

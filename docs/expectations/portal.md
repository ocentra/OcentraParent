# Portal Feature Expectations

Portal features must exercise the real agent path.

## Expected Deliverables

- UI reads typed domain/protocol contracts.
- UI validates agent events through Effect Schema.
- UI uses text/domain packages for display text.
- One clear result area for command output where appropriate.
- Copy/debug affordance for sharing current result.
- Playwright coverage when UI behavior changes.

## Acceptance

- Portal connects to the real local service in tests.
- Playwright proves the visible behavior.
- Command clicks update existing panels instead of appending endless boxes unless the feature is explicitly a log view.
- Logs/history use a table or timeline pattern.
- UI remains usable on common desktop and mobile widths.
- Browser-visible errors and warnings are treated as product issues unless proven harmless and documented.

## Non-Goals

- Do not bypass the Rust service with hardcoded browser state.
- Do not create a polished marketing dashboard before the underlying data path exists.
- Do not show fake activity data as if it came from the child device.

## Done Signal

The portal shows real service data, validates payloads, has useful copy/debug affordances, and has Playwright coverage for the parent-visible behavior.

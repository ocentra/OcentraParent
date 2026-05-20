# Portal Feature Expectations

Portal features must exercise the real agent path.

The portal is a parent-facing control and observability surface. It does not execute child-device work. It sends typed queries, rule updates, approval decisions, and visibility requests to the child-device agent, then renders validated events and read models returned by that agent.

## Expected Deliverables

- UI reads typed domain/protocol contracts.
- UI validates agent events through Effect Schema.
- UI uses text/domain packages for display text.
- One clear result area for intent output where appropriate.
- Copy/debug affordance for sharing current result.
- Explicit rule/query/approval intent contracts for any parent action.
- Playwright coverage when UI behavior changes.

## Acceptance

- Portal connects to the real local service in tests.
- Playwright proves the visible behavior.
- Control clicks update existing panels instead of appending endless boxes unless the feature is explicitly a log view.
- Parent actions are represented as typed intents, not browser-executed work.
- Device-side execution result is visible when a parent action changes rules, approvals, or device state.
- Logs/history use a table or timeline pattern.
- UI remains usable on common desktop and mobile widths.
- Browser-visible errors and warnings are treated as product issues unless proven harmless and documented.

## Non-Goals

- Do not bypass the Rust service with hardcoded browser state.
- Do not run OS commands, capture adapters, AI safety evaluation, policy evaluation, enforcement, timers, or scripts in the portal.
- Do not let portal code become the source of truth for whether a child activity is allowed or blocked.
- Do not create a polished marketing dashboard before the underlying data path exists.
- Do not show fake activity data as if it came from the child device.

## Done Signal

The portal shows real service data, validates payloads, sends only typed parent intents to the device agent, has useful copy/debug affordances, and has Playwright coverage for the parent-visible behavior.

# V0.2 Trusted Local Evidence Store Expectations

This is the milestone-specific expectation file for V0.2 in `docs/product-roadmap.md`.

Supporting expectation files: [data custody](data-custody.md), [evidence storage](evidence-storage.md), [contracts](contracts.md), [portal](portal.md), and [static analysis and security](static-analysis-security.md).

## Outcome

- The child-device agent can write, rotate, replay, ingest, and query trusted local evidence before capture or enforcement exists.
- Encrypted NDJSON remains the source of truth and SQLite remains the local query/index store.
- Portal visibility comes from the real service path, not fake UI state.

## Acceptance

- Journal entries are encrypted, tamper detection works, and rotated segments replay in order.
- SQLite query state rebuilds from journal replay and handles duplicate ingest safely.
- Recent activity and ingest status are visible through typed service responses.

## Validation

- Run `npm run validate`.
- Include journal crypto, replay, query-store, local WebSocket smoke, and portal E2E evidence in handoff.

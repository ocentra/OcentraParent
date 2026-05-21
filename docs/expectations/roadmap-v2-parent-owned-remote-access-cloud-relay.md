# V2 Parent-Owned Remote Access And Cloud Relay Expectations

This is the milestone-specific expectation file for V2 in `docs/product-roadmap.md`.

Supporting expectation files: [data custody](data-custody.md), [cloud](cloud.md), [sync and export](sync-export.md), [LAN pairing](lan-pairing.md), and [static analysis and security](static-analysis-security.md).

## Outcome

- Parent-away-from-home use cases work without making Ocentra the family-data store.
- Cloud services act as account, control-plane, notification, relay, connector-status, and optional stateless compile surfaces.
- Child-device agents validate and execute scoped typed intents locally.

## Acceptance

- Remote health, route status, rule/query/approval intents, parent-owned storage connector status, sync queue, conflict handling, and heartbeat/stale states are typed and auditable.
- Local-first operation continues when cloud is unavailable.
- Ocentra-hosted infrastructure does not retain child activity evidence or generated reports by default.

## Validation

- Run `npm run validate`.
- Include route/auth contract tests, child-agent accepted/rejected intent tests, connector failure tests, and portal/app remote-status coverage.

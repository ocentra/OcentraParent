# WP03 Dispatch Runtime And Lifecycle

Scope: prove local dispatch semantics, subscriber lifecycle, handler reporting, timeout, panic isolation, and observability.

Source rows: `05-implementation-workpacks.md` rows 11-24.

Read next:

- `../05-implementation-workpacks.md` only for rows 11-24
- `../04-tests-proof-and-validation.md`
- `../TEST_PROOF_EXPECTATIONS.md`
- `../PROOF_INDEX.md` only when recording proof paths

Expected outcome:

- Subscriber registry is scoped to an explicit runtime-owned bus.
- Sequential, concurrent, aggregate-ordered, nested publish, fire-and-forget, and publish-and-wait modes have distinct behavior.
- Handler timeout, retry, panic isolation, and wrong-target reporting are deterministic.
- Registrar subscribe/dispose lifecycle and idempotent unsubscribe are proved.
- Metrics/tracing fields are stable enough for logs and diagnostics without leaking payload custody.

Expected tests/proof:

- `eventing.dispatch.sequential.ordering`
- `eventing.dispatch.concurrent.aggregation`
- `eventing.dispatch.aggregate-ordering`
- `eventing.dispatch.no-lock-held-await.audit`
- `eventing.handler.timeout-retry`
- `eventing.handler.panic-isolation`
- `eventing.subscription.lifecycle.idempotent`
- Proof includes command log, timing bounds, panic/timeout negative case, and observability field sample.

Failure conditions:

- Do not use mocks/fakes/spies as the main proof of dispatch behavior.
- Do not hold shared locks across async waits.
- Do not expose hidden global event buses.

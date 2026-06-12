# WP05 Request Response Contracts

Scope: prove local typed request/response behavior without turning eventing into a remote RPC or product-command authority.

Source rows: `05-implementation-workpacks.md` rows 31-35.

Read next:

- `../05-implementation-workpacks.md` rows 31-35 only
- `../02-crate-api-and-code-shape.md`
- `../TEST_PROOF_EXPECTATIONS.md`

Expected outcome:

- Request completion registry is local, bounded, observable, and tied to typed response contracts.
- Timeout, late response, double completion, cancellation/expiry, and durable result-event pattern are specified.
- Request/response does not bypass event custody, validation, or consumer-plan authority.

Expected tests/proof:

- `eventing.request-response.typed-resolution`
- `eventing.request-response.timeout`
- `eventing.request-response.late-response`
- `eventing.request-response.double-completion-negative`
- `eventing.request-response.durable-result-event`
- Proof includes malformed response rejection, expiry boundary, and authority note for product commands.

Failure conditions:

- Do not model remote transport delivery as local request completion.
- Do not let UI or AI complete privileged requests without owning consumer-plan proof.
- Do not carry deferred handles, cleanup callbacks, sockets, tasks, or service pointers inside payloads.

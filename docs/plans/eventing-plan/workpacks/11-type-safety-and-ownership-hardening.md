# WP11 Type Safety And Ownership Hardening

Scope: harden eventing against weak typing, payload mutation, unsafe async ownership, naked strings, and TypeScript/Rust fixture drift.

Source rows: `05-implementation-workpacks.md` rows 63-68.

Read next:

- `../06-type-safety-validation-and-ownership.md`
- `../04-tests-proof-and-validation.md`
- `../../agent/SOURCE_BOUNDARY_FLOW.md`
- `../../agent/TEST_PROOF_DECISION_MATRIX.md` only after local expected tests are selected

Expected outcome:

- Live envelope and stored envelope cannot be confused.
- Request event associated response type is enforced.
- Mutation and interior mutability are rejected unless explicitly proven safe.
- No lock-held-await, borrowed async hazard, or hidden global ownership route remains.
- TypeScript/Rust branded fixtures and schema examples match across boundaries.

Expected tests/proof:

- `eventing.type-safety.live-vs-stored-negative`
- `eventing.request.associated-response-proof`
- `eventing.payload-mutation.source-gate`
- `eventing.no-lock-held-await.audit`
- `eventing.ts-rust.fixture-parity`
- `eventing.no-naked-domain-strings.guard`
- Proof includes static/audit command output, fixture diff result, negative cases, and remaining brand/schema gaps.

Failure conditions:

- Do not accept stringly typed event families or IDs.
- Do not hide mutation behind serde/json escape hatches.
- Do not use TypeScript fixtures as proof of Rust runtime behavior without Rust validation.

Expected proof artifacts:

- `output/eventing-plan-proof/63-type-safety-source-gate/proof-summary.json`
- `output/eventing-plan-proof/66-76-source-safety/proof-summary.json`
- `output/eventing-plan-proof/67-lock-await/proof-summary.json`
- `output/eventing-plan-proof/68-fixture-parity/proof-summary.json`

Current source audit (production source integrated; not closure evidence):

- The accepted source packet at `4aaddb425` keeps `EventEnvelope<E>` bounded by
  `DomainEvent`, makes every live-envelope field private, and exposes immutable
  borrowing accessors plus consuming `into_payload()` from
  `crates/ocentra-eventing/src/envelope/accessors.rs`.
- `StoredEventEnvelope::decode` now reuses the same validation helper after
  payload decoding, revalidating the decoded payload's contract, aggregate key,
  and idempotency key against the stored envelope metadata.
- Reviewed reachable production consumers in `agent-core`, `agent-service`,
  `app-game-core`, `child-runtime`, and `ocentra-eventing` now use the bounded
  accessor/consuming API; an independent review found and repaired three missed
  callers before integration.
- Existing tests still use parts of the old public-field API and do not retain
  the required
  malformed payload, aggregate/idempotency tamper, payload-mutation, and
  lock/await audit negatives for the accepted helper.
- The cited `63`, `66-76`, `67`, and `68` proof roots are absent; unrelated
  policy-control TypeScript checks do not close this Eventing workpack.

WP11 is therefore source-integrated but open. The later test-writing phase must
migrate existing test callers and add the named negative/audit families before
focused execution, retained proof, or any completion claim.

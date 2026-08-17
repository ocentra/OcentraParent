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

Current source audit (implementation-ready; not closure evidence):

- `EventEnvelope<E>` is unconstrained at the struct boundary in
  `crates/ocentra-eventing/src/envelope.rs`; the typed `DomainEvent` bound is
  applied only by selected impls.
- `StoredEventEnvelope::decode` checks the contract but does not revalidate the
  decoded payload's aggregate and idempotency keys against the stored envelope.
- Existing round-trip/version-skew tests do not retain the required malformed
  payload, aggregate/idempotency tamper, payload-mutation, and lock/await audit
  negatives.
- The cited `63`, `66-76`, `67`, and `68` proof roots are absent; unrelated
  policy-control TypeScript checks do not close this Eventing workpack.

WP11 is therefore open and implementation-ready, pending the bounded source
fixes/negative tests and regenerated retained proof.

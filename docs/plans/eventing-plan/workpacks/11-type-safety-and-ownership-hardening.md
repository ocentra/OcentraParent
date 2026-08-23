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

- The accepted source packet through integration commit `fa1230661` keeps
  `EventEnvelope<E>` bounded by
  `DomainEvent`, makes every live-envelope field private, and exposes immutable
  borrowing accessors plus consuming `into_payload()` from
  `crates/ocentra-eventing/src/envelope/accessors.rs`.
- `StoredEventEnvelope::decode` now reuses the same validation helper after
  payload decoding, revalidating the decoded payload's contract, aggregate key,
  and idempotency key against the stored envelope metadata.
- Live `EventEnvelope<E>` deserialization performs the same contract,
  aggregate, and idempotency revalidation instead of trusting serde field
  shape. Pending request entries retain the `RequestEvent` response `TypeId`,
  so a mismatched completion cannot satisfy the request.
- `NdjsonEventJournal` owns event-id/phase idempotent append behavior and the
  generic journal default fails closed when an implementation does not support
  it. Action replay accepts only a private, journal-created,
  non-cloneable/non-serde `ReplayActionReport` and consumes it on dispatch;
  projection replay remains non-authorizing.
- Reviewed reachable production consumers in `agent-core`, `agent-service`,
  `app-game-core`, `child-runtime`, and `ocentra-eventing` now use the bounded
  accessor/consuming API; an independent review found and repaired three missed
  callers before integration.
- The routed WP11 test-source packet is integrated at canonical commit
  `ac5d41322`, updating `crates/ocentra-eventing/tests/unit/envelope.rs`,
  `crates/ocentra-eventing/tests/contract/typed_boundary.rs`, and
  `crates/ocentra-eventing/tests/journal_replay/replay.rs` for the hardened
  accessors/replay API and required negative/audit coverage. The three target
  harnesses compile with `--no-run`; this is compile-only validation, not test
  execution or retained proof.
- Actual test execution, retained proof roots, checklist rows 63-68, and
  completion review remain open. No source/test presence or compile-only result
  may promote WP11 to DONE.
- The cited `63`, `66-76`, `67`, and `68` proof roots are absent; unrelated
  policy-control TypeScript checks do not close this Eventing workpack.

WP11 is therefore in validation: production and routed test source are
integrated, and the three target harnesses compile with `--no-run`. Actual test
execution, retained proof, checklist rows 63-68, and completion review remain
open; no DONE claim is made.

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

Current local completion evidence from the 2026-06-17 regeneration and follow-up validation pass:

- The scoped Rust proof roots now regenerate successfully at:
  - `output/eventing-plan-proof/63-type-safety-source-gate/proof-summary.json`
  - `output/eventing-plan-proof/66-76-source-safety/proof-summary.json`
  - `output/eventing-plan-proof/67-lock-await/proof-summary.json`
  - `output/eventing-plan-proof/68-fixture-parity/proof-summary.json`
- Package-wide TypeScript validation now passes again:
  `npm run type-check --workspace @ocentra-parent/agent-protocol-domain`.
- Focused downstream validation for the hardened package surface now also
  passes:
  `cmd /c npm run test --workspace @ocentra-parent/agent-protocol-domain -- policy-control-audit-redaction.test.ts policy-control-delivery-read-model.test.ts contracts.test.ts`.
- The touched-file architecture gate now passes for
  `src/contracts.ts`, `src/policy-control-audit-redaction.ts`, and
  `src/policy-control-delivery-read-model.ts`.

These proof-summary paths now exist in this checkout, and WP11 is locally
proved in truth for this worktree. The broader plan still remains open because
WP10 household-mesh consumer proof is not yet restored.

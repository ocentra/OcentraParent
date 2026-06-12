# AI Plan � HID Execution Blueprint

## Execution objective

Force AI runtime to stay evidence-only, schema-safe, and authority-neutral with deterministic fallback behavior.

## Slice 01 � Contract and Parser Boundaries

### Acceptance

- AI request/result contracts are schema-valid and cross-language parity is proved.

### Tests

- `ai.contract.schema-negative-decode`
- `ai.output.invariant-regression`

### Proof

- `docs/proof/ai-plan/slice-01-contract-schema.md`

## Slice 02 � Provider and Runtime Routing

### Acceptance

- Provider routing is deterministic and retry/dead-letter/replay-safe.

### Tests

- `ai.runtime.idempotency`
- `ai.rate-limit` and `ai.retry-storm`

### Proof

- `docs/proof/ai-plan/slice-02-provider-routing.md`

## Slice 03 � Safety and Prompt Boundary

### Acceptance

- No prompt injection or raw-screen input can alter safety boundaries.

### Tests

- `ai.prompt-injection.boundary`
- `ai.result-journal.replay-idempotency`

### Proof

- `docs/proof/ai-plan/slice-03-safety-boundary.md`

## Slice 04 � Output and Hallucination Resilience

### Acceptance

- OCR/VLM/classifier output remains bounded and schema-conformant under fixture drift.

### Tests

- `ai.safety-regression.hallucination`
- `ai.output.invariant-regression`

### Proof

- `docs/proof/ai-plan/slice-04-output-invariants.md`

## Workpacks (execution lane)

### Slice-to-workpack binding

- Slice 01: docs/plans/ai-plan/workpacks/01-source-index-and-repo-reconciliation.md
- Slice 02: docs/plans/ai-plan/workpacks/02-current-ai-snapshot-and-gap-map.md
- Slice 03: docs/plans/ai-plan/workpacks/03-contract-boundary-and-effect-schemas.md
- Slice 04: docs/plans/ai-plan/workpacks/04-rust-protocol-parity-for-ai-contracts.md

## PR-ready gate

- No policy-action claim can be checked in until `no-ai-direct-action` is explicitly proven in proof manifest.
- Any output path missing source schema negatives or redaction evidence fails PR gate.

## HID test floor (this plan)

### Required test families for closed slice

- Unit: parser/schema + output invariants
- Integration: provider routing queue and dead-letter
- E2E: policy handoff path with evidence routing
- Security: prompt-injection, hallucination boundary
- Non-functional: timeout/fallback and resource pressure

### Mandatory slice evidence checks

- negative cases documented (at least one per slice)
- rollback/teardown proof recorded
- proof manifest references command output, artifacts, and manual review notes

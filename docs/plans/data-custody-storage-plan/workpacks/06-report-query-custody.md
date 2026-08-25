# Workpack 06: Report Query Custody

Goal: define reports and queries as derived views over governed evidence, not a second data truth.

Context to read:

- `docs/plans/data-custody-storage-plan/EVENT_MODEL.md`
- `docs/plans/data-custody-storage-plan/DATA_CLASSIFICATION.md`
- `docs/features/reports-notifications-sync.md`
- `docs/features/evidence-store-query.md`
- `docs/expectations/notifications.md`
- `docs/expectations/evidence-storage.md`

In scope:

- Report source references, query cursors, pagination, citations, redaction, retention, and export/delete behavior.
- Notification payload boundaries.
- Assistant and report Q&A evidence references.
- Query performance and abuse limits.
- Derived materialization rules, cache invalidation, delete and tombstone effects, and cross-device stale or conflict states.

Out of scope:

- Event transport mechanics owned by `eventing-plan`.
- Portal rendering owned by `portal-ux-household-surfaces-plan`.
- Creating a second uncontrolled report database outside custody rules.

Acceptance:

- Derived views cite allowed source data only.
- Notification payloads are redacted to the chosen custody boundary.
- Query pagination and cursor behavior are stable.
- Stale or conflicting sync state is explicit and claim-safe.

Required query and report states:

- `derivedFresh`
- `derivedStale`
- `partiallyRedacted`
- `deletedSource`
- `syncConflict`
- `cursorExpired`
- `rateLimited`

Expected proof names:

- `data-custody.report.derived-source-matrix`
- `data-custody.report.deleted-expired-no-leak`
- `data-custody.query.cursor-pagination-stability`
- `data-custody.query.rate-limit-abuse`
- `data-custody.notification.payload-allow-deny`
- `data-custody.assistant.allowed-citation-proof`
- `data-custody.report.stale-conflict-state`

Failure conditions:

- Reports duplicate sensitive evidence outside retention control.
- Notifications leak child activity details beyond the chosen custody boundary.
- Assistant or report answers cite evidence the parent role cannot access.
- Query cache returns deleted, expired, wrong-household, or wrong-child data.

## Completion

- Status: production source accepted for the full Rust-owned request/row boundary and generated TypeScript edge; expected tests, focused execution, proof refresh, and downstream runtime consumers remain open. This workpack is not DONE or PR_READY.
- Proof root: `output/data-custody-storage-plan-proof/06-report-query-custody/`
- Canonical owners: `crates/schema`, `crates/storage-custody-core`, and the thin/generated adapter surface in `packages/schema-domain`.

## Source-wave checkpoint (2026-08-17)

- The Rust boundary derives opaque query-source owner evidence from a sealed
  Account-issued authority snapshot and fails closed on the complete
  account/household/member/role/device/child/session/generation/provenance
  binding, session expiry, exact `parent_action.actor.role` matching, bounded
  page size, and requested/source/next cursor plus unique stable-sort/row/source
  continuity. This is snapshot validation only: WP06 does not re-read the
  durable Account repository and does not claim race-safe or
  revocation-linearized current authority.
- Proof construction now requires all seven report/query outcome states and
  returns the private-inner, serialize-only
  `ValidatedReportQueryCustodyProofSnapshot`. The public serde contract proof
  remains an untrusted wire DTO and cannot mint that validated snapshot.
  Producer resolution and proof construction both reject result counts above
  `request.page_size`.
- Rust-owned request/proof templates and their checked-in generated TypeScript
  rules now require requested data classes to be a subset of allowed classes,
  bind every row source class to both scopes, bind row scope/citation arrays and
  authority generation exactly to the request, and enforce
  `rows.length <= request.pageSize`. The TypeScript file remains generated
  output, not a TypeScript-owned authority or runtime.
- Report, notification, portal, and AI runtime consumers remain adjacent-owner work. No shipped report/query caller reaches this shared custody boundary, and no ActivityStore adapter is claimed here.
- No tests were written or run in this source wave. The existing Rust harnesses are stale against the current source API/fixture, and `packages/schema-domain/tests/contract/report-query-custody.test.ts` is an expected but unwritten TypeScript contract test. Focused execution, proof refresh, precommit, CI, and PR remain deferred.

## Routing refresh (2026-08-18, integrated source checkpoints `9462ce44e` and `d3c4b64ca`)

The reviewed source map in `docs/engineering-graph/code-map.json` assigns the
22 integrated production/generated roots to WP06 and retains the prior
module/interface roots for complete WP06 ownership:

- Account authority boundary: `crates/family-identity-core/src/account_identity_authority.rs`, `crates/family-identity-core/src/account_identity_authority_capability.rs`, `crates/family-identity-core/src/account_identity_authority_query_custody.rs`.
- Rust schema and canonical generator inputs: `crates/schema/src/report_query_custody.rs`, `crates/schema/src/report_query_custody/sample_rows.rs`, `crates/schema/src/report_query_custody_ts.rs`, `crates/schema/src/report_query_custody_contracts.template.txt`, `crates/schema/src/report_query_custody_contract_rules.request.template.txt`, `crates/schema/src/report_query_custody_contract_rules.proof.template.txt`.
- Rust custody derivation boundary: `crates/storage-custody-core/Cargo.toml`, `crates/storage-custody-core/src/report_query_custody.rs`, `crates/storage-custody-core/src/report_query_custody_page_size_validate.rs`, `crates/storage-custody-core/src/report_query_custody_proof.rs`, `crates/storage-custody-core/src/report_query_custody_proof_validate.rs`, `crates/storage-custody-core/src/report_query_custody_request_validate.rs`, `crates/storage-custody-core/src/report_query_custody_row.rs`, `crates/storage-custody-core/src/report_query_custody_row_validate.rs`, `crates/storage-custody-core/src/report_query_custody_source.rs`, `crates/storage-custody-core/src/report_query_custody_state_projection.rs`, `crates/storage-custody-core/src/report_query_custody_verified_proof.rs`.
- Generated TypeScript edge: `packages/schema-domain/src/generated-report-query-custody-contract-rules.ts`, `packages/schema-domain/src/generated-report-query-custody-contracts.ts`.

Deferred test/source paths are recorded without a completion claim:

- Stale Rust harnesses: `crates/schema/tests/contract/report_query_custody.rs`, `crates/schema/tests/contract/report_query_custody_generated.rs`, and `crates/storage-custody-core/tests/unit/report_query_custody.rs`.
- Expected but unwritten TypeScript contract test: `packages/schema-domain/tests/contract/report-query-custody.test.ts`.

## Required states represented in source

- `derivedFresh`, `derivedStale`, `partiallyRedacted`, `deletedSource`,
  `syncConflict`, `cursorExpired`, and `rateLimited` are mandatory at proof
  construction and remain individually typed in Rust and the generated edge.
- The existing Rust harnesses have not been migrated to the sealed proof API or
  rerun, and the expected TypeScript contract test is absent. These source
  invariants are therefore not current test or proof acceptance.

## Expected proof artifacts (not regenerated)

- `00-derived-source-matrix-proof.md`
- `01-deleted-expired-not-returned-proof.md`
- `02-query-cursor-pagination-proof.md`
- `03-query-rate-limit-proof.md`
- `04-notification-payload-allow-deny-proof.md`
- `05-portal-cache-custody-proof.md`
- `06-assistant-allowed-citation-proof.md`
- `07-stale-conflict-state-proof.md`
- `16-validation-commands.log`

## Focused validations

- `cargo test -p ocentra-schema --test contract report_query_custody`
- `cargo test -p ocentra-storage-custody-core report_query_custody`
- `cmd /c npm run build --workspace @ocentra-parent/schema-domain`
- `cmd /c npm run test --workspace @ocentra-parent/schema-domain -- tests/contract/report-query-custody.test.ts`
- `cargo lint-architecture crates/schema/src/report_query_custody.rs crates/schema/src/report_query_custody_ts.rs crates/storage-custody-core/src/report_query_custody.rs crates/storage-custody-core/src/report_query_custody_proof_validate.rs crates/storage-custody-core/src/report_query_custody_source.rs crates/storage-custody-core/src/report_query_custody_verified_proof.rs crates/schema/tests/contract/report_query_custody.rs crates/storage-custody-core/tests/unit/report_query_custody.rs`
- `cmd /c npm run lint:architecture -- --files packages/schema-domain/src/generated-report-query-custody-contracts.ts packages/schema-domain/src/generated-report-query-custody-contract-rules.ts packages/schema-domain/tests/contract/report-query-custody.test.ts`

## Adjacent handoffs

- AI and notification runtime owners remain sibling consumers of this shared custody boundary; this packet does not re-own their runtime behavior.
- Portal owners remain downstream consumers; WP06 proves no second truth store or portal-cache mutation claim at the shared contract/runtime layer only.

## No-claim boundary

- No portal rendering claim is made.
- No notification delivery-runtime claim is made.
- No AI answer-runtime claim is made.
- No hosted report/query store claim is made.

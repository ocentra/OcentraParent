# Workpack 01: Custody Source of Truth

Goal: define every data class, its owner, and its source of truth.

Context to read:

- `docs/plans/data-custody-storage-plan/DECISIONS.md`
- `docs/plans/data-custody-storage-plan/DATA_CLASSIFICATION.md`
- `docs/plans/data-custody-storage-plan/RESEARCH_AND_UI_GUIDANCE.md`
- `docs/expectations/data-custody.md`
- `docs/expectations/evidence-storage.md`
- `docs/features/evidence-store-query.md`
- `docs/features/reports-notifications-sync.md`

In scope:

- Data taxonomy for account, household, child profile, device, policy, evidence, reports, notifications, logs, diagnostics, billing references, setup state, AI outputs, screenshots, location, network/app/browser events, provider payloads, and support bundles.
- Custody authority for parent-owned, local-only, parent-cloud-owned, Ocentra relay/transient, Ocentra account metadata, and provider-owned billing identity.
- Sensitive data classes, redaction obligations, and forbidden-hosting boundaries.
- Current truth path for each data class.

Out of scope:

- Key custody.
- Provider selection and sync mechanics.
- Delete/tombstone protocol.
- UI implementation.

Acceptance:

- Every active data class has a source-of-truth row.
- Ocentra-hosted-by-default and must-never-host-by-default classes are explicitly separated.
- Derived data is marked as derived and not mistaken for source truth.
- The forbidden-data list is explicit and claim-safe.

Expected artifacts:

- Data custody matrix.
- Data owner and source-of-truth table.
- Forbidden-data list.
- Claim-safe language list.
- Adjacent-plan handoff notes.

Expected proof names:

- `data-custody.matrix.source-of-truth`
- `data-custody.matrix.no-hosting-default`
- `data-custody.matrix.redaction-rules`

Failure conditions:

- Vague "we store nothing" claims when account, billing, update, or support metadata is required.
- Any plan treating evidence, screenshots, location, policy, or child profile data as generic telemetry.
- A data class missing source-of-truth ownership or encryption/redaction rules.

## Completion

- Status: production source complete for the Rust-owned WP01 contract; the complete expected-test family, current focused execution, proof refresh, and broader readiness remain open.
- Proof root: `output/data-custody-storage-plan-proof/01-custody-source-of-truth/`
- Canonical owner: `crates/schema` for the shared custody source-of-truth contract, generated TS contract surface, and Rust proof tests.
- TS/shared edge note: the Rust-first convergence deliberately removed the old handwritten `custody-boundary.ts` and `data-custody-matrix.ts` adapters. The only current TypeScript edge is the generated, package-exported `packages/schema-domain/src/generated-data-custody-source-of-truth-contracts.ts`; do not restore deleted adapters without a real consumer.

## Implemented contract states

- All `28` active data classes now have an explicit source-of-truth row, owner, default location, and derived-versus-self truth marker.
- Ocentra-hosted-by-default metadata is explicit and limited to `8` classes; `13` classes are explicitly marked `mustNeverBeHostedByDefault`.
- Hosting mode split is explicit and honest: `20` `forbidden`, `6` `allowed-metadata-only`, `1` `short-lived-status-only`, and `1` `public-release-only`.
- Derived classes are explicit and cited rather than promoted to self truth: SQLite/read-model, local AI/policy decisions, generated reports, parent notification history, assistant context, parent-owned storage contents, provider payloads, and support bundles all cite their source classes.
- Raw child evidence rows keep notification exposure at `none`, with reports limited to `allowed-references-only` or `none`.
- Account and provider control-plane separation stays explicit and all WP01 non-claim flags remain false.

These rows describe live production contract source, not complete test proof.
The current Rust contract test covers serde round-trip and generated-file drift.
The expected-test wave must still cover the exact 28-row inventory, unique IDs,
derived-source validity, hosting counts, redaction/notification rules, forbidden
hosting, and every no-claim flag.

## Manual-required and no-claim truth

- `supportDecryptByDefaultClaimed` remains false.
- Provider mode defaults remain explicit and are not implied by the matrix.
- Mobile restore/key custody, delete ergonomics/tombstone propagation, and transfer runtime remain later-workpack or manual-required surfaces.
- No default Ocentra child-activity store, no SQLite truth-layer claim, no provider auto-apply claim, no Ocentra-owned parent-rules claim, no raw-child-evidence-in-notifications claim, and no long-lived hosted reports claim are made.

## Historical proof artifacts

The ignored `output/` root below is not present in a clean checkout and is not
current acceptance for this source.

- `00-data-classification-matrix-proof.md`
- `01-source-of-truth-proof.md`
- `02-no-default-hosted-private-activity-proof.md`
- `03-account-control-plane-separation-proof.md`
- `04-redaction-boundary-proof.md`
- `16-validation-commands.log`
- `data-custody-source-of-truth-proof.json`

## Focused validations

- `cmd /c cargo test -q -p ocentra-schema --test contract data_custody_source_of_truth`
- `cmd /c npm run build --workspace @ocentra-parent/schema-domain`
- `cmd /c npm run lint:architecture -- --files packages/schema-domain/src/generated-data-custody-source-of-truth-contracts.ts`
- `cargo lint-architecture crates/schema/src/data_custody_source_of_truth.rs crates/schema/src/data_custody_source_of_truth crates/schema/src/data_custody_source_of_truth_ts.rs crates/schema/tests/contract/data_custody_source_of_truth.rs`

These commands are the later focused-validation route. They were not run in
the current source/status reconciliation.

## Adjacent handoffs

- WP03/WP08 may now rerun against the normalized generated contract and thin `schema-domain` edge; WP01 no longer carries the prior shared source-of-truth/generated-contract blocker.
- Delete/tombstone, export/import/restore, provider sync/runtime, AI runtime, and portal rendering remain owned by their later workpacks or sibling plans.

## No-claim boundary

- No provider OAuth/upload/delete/retrieval runtime claim is made.
- No transfer runtime claim is made.
- No delete/tombstone or restore/apply runtime claim is made.
- No portal rendering claim is made.
- No LAN claim is made.

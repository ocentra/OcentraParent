# Slice 01: Data Custody Substrate Truth Repair

## Scope

- assigned slice: `data-custody-substrate-truth-repair`
- workpacks touched: `WP03`, `WP04`, `WP05`, `WP06`
- owner boundary: proof-script routing, direct-owner contract wording, `storage-custody-core` correctness, and canonical proof-root honesty
- no-claim boundary: no downstream tracking behavior, device-trust recovery semantics, portal hosted UI truth, or whole-plan completion

## Files Changed

- `scripts/test/parent-owned-sync-export-manifest-proof.mjs`
- `scripts/test/stateless-report-compiler-status-proof.mjs`
- `scripts/test/sync-export-endpoint-contract-proof.mjs`
- `scripts/test/parent-owned-local-export-runtime-proof.mjs`
- `crates/storage-custody-core/tests/unit/custody_decision.rs`
- `crates/storage-custody-core/tests/unit/custody_flow.rs`
- `packages/production-domain/src/parent-owned-sync-export.ts`
- `packages/production-domain/src/stateless-report-compiler-status-values.ts`
- `packages/production-domain/tests/unit/parent-owned-sync-export.test.ts`
- `packages/production-domain/tests/unit/stateless-report-compiler-status.test.ts`

## What Changed

- Rewired stale proof scripts from `parent-domain` owner assumptions to the direct owners in `production-domain` and `endpoint-domain`.
- Repaired the stale endpoint proof test path from `tests/sync-export.test.ts` to `tests/unit/sync-export.test.ts`.
- Repaired `storage-custody-core` Rust tests by importing `DomainEvent` from `ocentra_eventing::envelope`.
- Updated direct-owner known-gap wording so generated proof artifacts no longer falsely describe these contracts as `parent-domain proof`.
- Recorded `parent-owned-local-export-runtime` as an explicit `parent-domain` holdout at the time while still collecting targeted proof for it; that explicit boundary later enabled the focused executor/proof follow-on without widening this substrate slice.

## Validation

| Command | Exit | Result |
| --- | ---: | --- |
| `npm run lint:architecture -- --files scripts/test/parent-owned-sync-export-manifest-proof.mjs scripts/test/stateless-report-compiler-status-proof.mjs scripts/test/sync-export-endpoint-contract-proof.mjs scripts/test/parent-owned-local-export-runtime-proof.mjs packages/production-domain/src/parent-owned-sync-export.ts packages/production-domain/src/stateless-report-compiler-status-values.ts packages/production-domain/tests/unit/parent-owned-sync-export.test.ts packages/production-domain/tests/unit/stateless-report-compiler-status.test.ts` | `0` | pass |
| `cargo lint-architecture crates/storage-custody-core/tests/unit/custody_decision.rs crates/storage-custody-core/tests/unit/custody_flow.rs` | `0` | pass |
| `cargo test -p ocentra-storage-custody-core` | `0` | pass |
| `npm run test --workspace @ocentra-parent/production-domain -- tests/unit/parent-owned-sync-export.test.ts` | `0` | pass |
| `npm run test --workspace @ocentra-parent/production-domain -- tests/unit/stateless-report-compiler-status.test.ts` | `0` | pass |
| `npm run test --workspace @ocentra-parent/endpoint-domain -- tests/unit/sync-export.test.ts` | `0` | pass |
| `node scripts/test/parent-owned-sync-export-manifest-proof.mjs` | `0` | pass |
| `node scripts/test/sync-export-endpoint-contract-proof.mjs` | `0` | pass |
| `node scripts/test/stateless-report-compiler-status-proof.mjs` | `0` | pass |
| `node scripts/test/parent-owned-local-export-runtime-proof.mjs` | `0` | pass |

## Proof Outputs

- `output/data-custody-storage-plan-proof/03-parent-owned-cloud-sync/parent-owned-sync-export-manifest-proof.json`
- `output/data-custody-storage-plan-proof/03-parent-owned-cloud-sync/sync-export-endpoint-contract-proof.json`
- `output/data-custody-storage-plan-proof/03-parent-owned-cloud-sync/16-validation-commands.log`
- `output/data-custody-storage-plan-proof/04-retention-delete-tombstone/16-validation-commands.log`
- `output/data-custody-storage-plan-proof/05-export-import-backup-recovery/parent-owned-local-export-runtime-proof.json`
- `output/data-custody-storage-plan-proof/05-export-import-backup-recovery/16-validation-commands.log`
- `output/data-custody-storage-plan-proof/06-report-query-custody/stateless-report-compiler-status-proof.json`
- `output/data-custody-storage-plan-proof/06-report-query-custody/16-validation-commands.log`

## Surviving Gaps

- The former `packages/parent-domain/src/parent-owned-local-export-runtime.ts` holdout is now backed by a focused executor and proof harness, but this substrate slice still does not claim retention scheduler runtime, parent-visible controls, connector/provider runtime, portal/UI truth, or hosted custody.
- The repaired proofs remain contract proofs. They still do not claim provider runtime, delete executor runtime, report compiler runtime, cloud worker behavior, portal UI, or hosted family/activity custody.
- `WP01`, `WP02`, `WP07`, and `WP08` remain open and are not implied complete by these artifacts.

## Recommended Next Slice

- `data-custody-recovery-bundle-and-handoff-contract`
- reason: the substrate proof roots are now truthful enough that the next dependency can be an explicit recovery/delete-export handoff contract for downstream `device-trust-bootstrap-plan` work, while keeping tracking-specific runtime behavior out of this plan.

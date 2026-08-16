# Slice 02: Recovery Bundle And Handoff Contract

## Scope

- assigned slice: `data-custody-recovery-bundle-and-handoff-contract`
- workpacks touched: `WP03`, `WP05`
- owner boundary: direct-owner custody contract states, sync/export recovery bundle proof shape, delete/export handoff contract, and plan-owned proof reporting
- no-claim boundary: no tracking runtime behavior, no provider runtime, no portal/UI truth, and no downstream `device-trust-bootstrap-plan` runtime or persistence implementation

## Files Changed

- `packages/data-custody-domain/src/custody-boundary.ts`
- `packages/data-custody-domain/tests/unit/custody-boundary.test.ts`
- `packages/production-domain/src/parent-owned-sync-export.ts`
- `packages/production-domain/tests/unit/parent-owned-sync-export.test.ts`
- `scripts/test/parent-owned-sync-export-manifest-proof.mjs`

## What Changed

- Added generic data-custody bundle type/state plus recovery/delete handoff target/state guards so substrate contracts can express preview-only, apply-pending, applied, partial-restore, delete-pending, delete-confirmed, rejected, and manual-required behavior.
- Extended the direct-owner `production-domain` sync/export contract with recovery bundle records covering:
  - setup restore preview handoff
  - device-trust recovery-persistence handoff
  - parent-local delete-runtime handoff
  - wrong-household, wrong-key, corrupt-bundle, manual-required, and partial-restore cases
- Kept preview explicitly non-mutating, source-of-truth preserving, and tombstone preserving in both generic custody and sync/export proof surfaces.
- Kept `packages/parent-domain/src/parent-owned-local-export-runtime.ts` explicit as the then-open runtime holdout instead of pretending this slice finished local delete execution; that follow-on is now locally closed by the focused executor proof on this branch/worktree.
- Updated the direct-owner proof script so the generated proof artifact exposes recovery bundle states, handoff states, and handoff targets directly.

## Validation

| Command | Exit | Result |
| --- | ---: | --- |
| `npm run lint:architecture -- --files scripts/test/parent-owned-sync-export-manifest-proof.mjs packages/data-custody-domain/src/custody-boundary.ts packages/data-custody-domain/tests/unit/custody-boundary.test.ts packages/production-domain/src/parent-owned-sync-export.ts packages/production-domain/tests/unit/parent-owned-sync-export.test.ts` | `0` | pass |
| `npm run test --workspace @ocentra-parent/data-custody-domain -- tests/unit/custody-boundary.test.ts` | `0` | pass |
| `node scripts/test/parent-owned-sync-export-manifest-proof.mjs` | `0` | pass |

## Proof Outputs

- `output/data-custody-storage-plan-proof/03-parent-owned-cloud-sync/parent-owned-sync-export-manifest-proof.json`
- `output/data-custody-storage-plan-proof/05-export-import-backup-recovery/17-recovery-bundle-and-handoff-contract.log`

## Downstream Boundary

- `device-trust-bootstrap-plan`: `locally closed for the owned recovery-persistence boundary`
- reason: this slice provided the substrate contract later consumed by the WP06 recovery-persistence closeout, including negative states and partial restore semantics.
- still not claimed here: this slice itself does not claim encrypted import/apply runtime, delete scheduler runtime, recovery key/device runtime, parent-visible controls, or whole-plan device-trust proof.

## Surviving Gaps

- The formerly downstream local export/delete runtime holdout is now closed by the focused Windows-host executor proof, but that does not expand this slice into a broader product claim.
- The updated proof remains a contract proof. It does not claim encrypted import/apply runtime, SQLite rebuild execution, provider API execution, connector OAuth, portal/UI control, retention scheduler runtime, or parent-visible export/delete controls.
- This slice remains a substrate contract packet; later consumer code/proof can consume it without turning this document into a whole-product claim.

## Recommended Next Slice

- recommended next owner: `coordinator-selected post-device-trust exact slice`
- reason: the storage-side recovery/delete/export handoff contract has already been consumed by the later device-trust WP06 persistence closure on this branch/worktree, so any further work should be chosen from the coordinator queue instead of reopening this substrate packet.

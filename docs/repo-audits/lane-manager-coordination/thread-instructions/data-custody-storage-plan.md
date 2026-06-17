# data-custody-storage-plan Instruction

## Verdict

`partial / substrate-blocking`. Owner surfaces exist, but proof roots/scripts drift, parent-domain holdout, and storage-custody-core test failure block honest closure.

## Assign first

`data-custody-substrate-truth-repair`:

- restore canonical proof roots and direct-owner proof script targets;
- fix `crates/storage-custody-core` unit import failure;
- move proof scripts away from stale parent-domain paths where possible;
- document `parent-owned-local-export-runtime.ts` as holdout or migrate it.

## Then

1. `data-custody-recovery-bundle-and-handoff-contract`.
2. `data-custody-tracking-consumer-integration`.
3. `data-custody-export-delete-idempotency-proof`.

## Coordinate with

- `device-trust-bootstrap-plan` for recovery/key-sealing substrate.
- `tracking-plan` for custody consumer contracts.
- `setup-install-provisioning-plan` for export/sync/setup handoffs.

## Do not

- Do not count tracking or device-trust algorithms as data-custody completion.
- Do not let parent-domain proof shims remain as substrate truth.
- Do not close without explicit delete/export/idempotency/security proof where owned.

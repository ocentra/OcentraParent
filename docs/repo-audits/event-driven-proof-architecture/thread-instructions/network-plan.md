# network-plan Event Architecture Instruction

## Owns

- network flow contracts, evidence parsing/classification, platform claim gates, network read models;
- network runtime evidence events where assigned.

## Must not own

- enforcement action authority;
- browser managed/unmanaged authority;
- screen summary ownership;
- AI runtime or policy dispatch;
- LAN transport truth.

## Required chain

```text
network capture/import
-> network owner parses/classifies evidence
-> network event/read model is recorded
-> policy/enforcement/AI/UI consumers react through typed contracts
```

## Logging/proof

Log capture/import source, parser result, classifier confidence, broad-platform no-claim, event emission, runtime delivery, and portal read-model update.

## Tests

Network-domain owns unit/contract. Rust parser/runtime/service tests should move to crate `tests/`. Cross-domain network -> policy/enforcement/AI proof belongs in service/app proof runners.

## First architecture slice

Finish parent-domain network shim cleanup, create proof roots, then move network Rust tests out of `src` before platform proof expansion.

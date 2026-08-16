# browser-plan Event Architecture Instruction

## Owns

- browser contracts, inventory, managed/unmanaged session state, intervention read models, social/browser/game browser-specific classifiers;
- browser proof scripts and browser platform status.

## Must not own

- enforcement authority;
- AI runtime execution;
- app-game timers;
- portal final product readiness;
- child runtime transport.

## Required chain

```text
browser inventory/session input
-> browser owner classifies managed/unmanaged state
-> browser event/read model records intervention or manual-required state
-> enforcement/policy/AI consumers act through typed contracts
-> portal renders service-backed browser state
```

## Logging/proof

Log browser source, managed-state classification, policy target compile result, intervention decision, no-claim exact URL/content boundary, and service/portal consumption.

## Tests

Browser package tests cover contracts and compilers. Rust protocol/core/service tests must move to crate-level tests for closure. Playwright proof must be service-backed, not screenshot-only.

## First architecture slice

Finish WP01 foundation cleanup, then WP03-WP05 inventory/platform matrix. Delay WP19/WP20 final closure until enforcement contracts are current.

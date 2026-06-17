<!-- agent-capsule -->

> Agent Capsule
> Plan: `logging-domain-parity`
> Doc: `Logging Domain Parity Workpack Index`
> Kind: workpack selector.
> Read when: after NEXT_ACTIONS.md.
> Stop rule: Open exactly one selected workpack.
> Proves: workpack routing only.
> Does not prove: implementation completion or PR readiness.
> Proof rule: Update counts/status only after the corresponding workpack/checklist rows change.

<!-- /agent-capsule -->

# Logging Domain Parity Workpack Index

Use this index to select exactly one workpack.

| Status | Workpack | Boxes | Primary source doc |
| --- | --- | ---: | --- |
| audit-open | [WP01 Current State and Reference Audit](workpacks/01-current-state-and-reference-audit.md) | 0/10 | `00-current-state-and-reference-audit.md` |
| source-present | [WP02 TypeScript Logging Package Parity](workpacks/02-typescript-logging-package-parity.md) | 0/12 | `00-current-state-and-reference-audit.md` |
| source-present | [WP03 Parent Logging Architecture and Routing](workpacks/03-parent-logging-architecture-and-routing.md) | 0/11 | `01-parent-logging-architecture.md` |
| source-present | [WP09 Log Control, Retention, and Bridge Lifecycle](workpacks/09-log-control-retention-bridge-lifecycle.md) | 0/13 | `08-log-control-retention-bridge-lifecycle.md` |
| source-present | [WP04 Rust Logging Core Crate](workpacks/04-rust-logging-core-crate.md) | 0/12 | `02-rust-logging-core-crate.md` |
| source-present | [WP05 Local Validation Evidence](workpacks/05-local-validation-evidence.md) | 0/12 | `03-local-validation-evidence.md` |
| source-present | [WP07 MCP Query Interface](workpacks/07-mcp-query-interface.md) | 0/18 | `06-mcp-query-interface.md` |
| partial-proof | [WP08 Logger Instrumentation and Adoption](workpacks/08-logger-instrumentation-and-adoption.md) | 0/12 | `07-logger-instrumentation-pattern.md` |
| partial-proof | [WP10 Proof Trace Pipeline](workpacks/10-proof-trace-pipeline.md) | 0/13 | `09-proof-trace-pipeline.md` |
| source-present | [WP06 Validation and Enforcement](workpacks/06-validation-and-enforcement.md) | 0/12 | `04-validation-and-enforcement.md` |

Status meanings:

```text
audit-open: audit docs exist, but the named proof root is absent in this checkout
source-present: implementation/tests exist, but the named proof root and checklist closeout are not present in this checkout
partial-proof: focused tests passed, but standalone smoke or broader proof inventory still has visible gaps
```

The boxes column mirrors the current unchecked rows in `CHECKLIST_INDEX.md`, not the historical completion prose still embedded in some workpack closeout sections.

## Selection rules

Current audit note:

```text
Before any workpack in this checkout can be honestly reported done, the plan-level proof inventory claims must be reconciled with the missing output/logging-domain-parity-proof/* and test-results/logging-domain-parity-* roots.
```

Default order:

```text
plan-state reconciliation -> WP10 -> WP03 -> WP06 -> implementation-specific follow-through
```

Allowed parallelism:

```text
WP02 and WP04 can be developed in parallel if package exports and JSON fixtures are coordinated.
WP03 can run after WP02 or in parallel with careful portal/agent route ownership.
WP09 should run after WP02 because it depends on bridge/path helpers.
WP05 must wait until enough WP02/WP04/WP09 storage primitives exist.
WP07 must wait until enough WP02/WP05 query data exists.
WP08 must wait until relevant logger APIs exist, but can run before WP07 smoke if using CLI query proof.
```

## Do not select

Do not create new workpacks unless the existing ten cannot represent the implementation slice.

Do not split into tiny proof-only workpacks.

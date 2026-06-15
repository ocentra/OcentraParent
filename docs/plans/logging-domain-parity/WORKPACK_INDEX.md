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
| done | [WP01 Current State and Reference Audit](workpacks/01-current-state-and-reference-audit.md) | 10/10 | `00-current-state-and-reference-audit.md` |
| done | [WP02 TypeScript Logging Package Parity](workpacks/02-typescript-logging-package-parity.md) | 12/12 | `00-current-state-and-reference-audit.md` |
| done | [WP03 Parent Logging Architecture and Routing](workpacks/03-parent-logging-architecture-and-routing.md) | 11/11 | `01-parent-logging-architecture.md` |
| done | [WP09 Log Control, Retention, and Bridge Lifecycle](workpacks/09-log-control-retention-bridge-lifecycle.md) | 13/13 | `08-log-control-retention-bridge-lifecycle.md` |
| done | [WP04 Rust Logging Core Crate](workpacks/04-rust-logging-core-crate.md) | 12/12 | `02-rust-logging-core-crate.md` |
| done | [WP05 Local Validation Evidence](workpacks/05-local-validation-evidence.md) | 12/12 | `03-local-validation-evidence.md` |
| done | [WP07 MCP Query Interface](workpacks/07-mcp-query-interface.md) | 18/18 | `06-mcp-query-interface.md` |
| done | [WP08 Logger Instrumentation and Adoption](workpacks/08-logger-instrumentation-and-adoption.md) | 12/12 | `07-logger-instrumentation-pattern.md` |
| done | [WP10 Proof Trace Pipeline](workpacks/10-proof-trace-pipeline.md) | 13/13 | `09-proof-trace-pipeline.md` |
| done | [WP06 Validation and Enforcement](workpacks/06-validation-and-enforcement.md) | 12/12 | `04-validation-and-enforcement.md` |

## Selection rules

Default order:

```text
WP01 -> WP02 -> WP03 -> WP09 -> WP04 -> WP05 -> WP07 -> WP08 -> WP10 -> WP06
```

Allowed parallelism:

```text
WP02 and WP04 can be developed in parallel if package exports and JSON fixtures are coordinated.
WP03 can run after WP02 or in parallel with careful portal/agent route ownership.
WP09 should run after WP02 because it depends on bridge/path helpers.
WP05 must wait until enough WP02/WP04/WP09 storage primitives exist.
WP07 must wait until enough WP02/WP05 query data exists.
WP08 must wait until relevant logger APIs exist, but can run before WP07 smoke if using CLI query proof.
WP10 must wait until WP08 has at least one instrumented path and WP09 proof-mode controls exist.
WP06 should be last, after the files it checks exist, including lifecycle, MCP, instrumentation, and proof-trace checks.
```

## Do not select

Do not create new workpacks unless the existing ten cannot represent the implementation slice.

Do not split into tiny proof-only workpacks.

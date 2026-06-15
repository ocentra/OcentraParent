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
| open | [WP01 Current State and Reference Audit](workpacks/01-current-state-and-reference-audit.md) | 0/10 | `00-current-state-and-reference-audit.md` |
| open | [WP02 TypeScript Logging Package Parity](workpacks/02-typescript-logging-package-parity.md) | 0/12 | `00-current-state-and-reference-audit.md` |
| open | [WP03 Parent Logging Architecture and Routing](workpacks/03-parent-logging-architecture-and-routing.md) | 0/11 | `01-parent-logging-architecture.md` |
| open | [WP04 Rust Logging Core Crate](workpacks/04-rust-logging-core-crate.md) | 0/12 | `02-rust-logging-core-crate.md` |
| open | [WP05 Local Validation Evidence](workpacks/05-local-validation-evidence.md) | 0/12 | `03-local-validation-evidence.md` |
| open | [WP07 MCP Query Interface](workpacks/07-mcp-query-interface.md) | 0/18 | `06-mcp-query-interface.md` |
| open | [WP08 Logger Instrumentation and Adoption](workpacks/08-logger-instrumentation-and-adoption.md) | 0/12 | `07-logger-instrumentation-pattern.md` |
| open | [WP06 Validation and Enforcement](workpacks/06-validation-and-enforcement.md) | 0/12 | `04-validation-and-enforcement.md` |

## Selection rules

Default order:

```text
WP01 -> WP02 -> WP03 -> WP04 -> WP05 -> WP07 -> WP08 -> WP06
```

Allowed parallelism:

```text
WP02 and WP04 can be developed in parallel if package exports and JSON fixtures are coordinated.
WP03 can run after WP02 or in parallel with careful portal/agent route ownership.
WP05 must wait until enough WP02/WP04 storage primitives exist.
WP07 must wait until enough WP02/WP05 query data exists.
WP08 must wait until relevant logger APIs exist, but can run before WP07 smoke if using CLI query proof.
WP06 should be last, after the files it checks exist, including MCP and instrumentation checks if WP07/WP08 are in scope.
```

## Do not select

Do not create new workpacks unless the existing eight cannot represent the implementation slice.

Do not split into tiny proof-only workpacks.

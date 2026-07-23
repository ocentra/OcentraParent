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

Use `WORKPACK_FAMILIES.md` only when the selected workpack owner/proof family is unclear. Do not use it as permission to scan multiple workpacks.

| Status | Workpack | Boxes | Primary source doc |
| --- | --- | ---: | --- |
| audit-open | [WP01 Current State and Reference Audit](workpacks/01-current-state-and-reference-audit.md) | 0/10 | `00-current-state-and-reference-audit.md` |
| source-present | [WP02 TypeScript Logging Package Parity](workpacks/02-typescript-logging-package-parity.md) | 0/12 | `00-current-state-and-reference-audit.md` |
| partial-proof | [WP03 Parent Logging Architecture and Routing](workpacks/03-parent-logging-architecture-and-routing.md) | 10/11 | `01-parent-logging-architecture.md` |
| source-present | [WP09 Log Control, Retention, and Bridge Lifecycle](workpacks/09-log-control-retention-bridge-lifecycle.md) | 0/13 | `08-log-control-retention-bridge-lifecycle.md` |
| complete-proven | [WP04 Rust Logging Core Crate](workpacks/04-rust-logging-core-crate.md) | 12/12 | `02-rust-logging-core-crate.md` |
| source-present | [WP05 Local Validation Evidence](workpacks/05-local-validation-evidence.md) | 0/12 | `03-local-validation-evidence.md` |
| partial-proof | [WP07 MCP Query Interface](workpacks/07-mcp-query-interface.md) | 0/18 | `06-mcp-query-interface.md` |
| partial-proof | [WP08 Logger Instrumentation and Adoption](workpacks/08-logger-instrumentation-and-adoption.md) | 8/12 | `07-logger-instrumentation-pattern.md` |
| partial-proof | [WP10 Proof Trace Pipeline](workpacks/10-proof-trace-pipeline.md) | 0/13 | `09-proof-trace-pipeline.md` |
| partial-proof | [WP06 Validation and Enforcement](workpacks/06-validation-and-enforcement.md) | 11/12 | `04-validation-and-enforcement.md` |

Status meanings:

```text
audit-open: audit docs exist, but the named proof root is absent in this checkout
source-present: implementation/tests exist, but the named proof root and checklist closeout are not present in this checkout
partial-proof: focused proof root exists or focused tests passed, but checklist/workpack closeout remains open or broader validation still has visible blockers
complete-proven: every workpack and central checklist row is checked against recorded command evidence; locally generated ignored proof output is reproducible evidence, not a tracked checkout prerequisite
```

The boxes column mirrors the current checklist completion count from `CHECKLIST_INDEX.md`, not the historical completion prose still embedded in some workpack closeout sections.

## Selection rules

Current audit note:

```text
WP04 is complete-proven at 12/12 against source commit `9a1de1600eea800874b12dd8c5c3f5155da069b8`: bounded partial-tail and compacted operation-state recovery, directory-durable compacted commits, indexed compacted-journal lookup, mixed-producer atomic append preservation, atomic repairable markers, exact committed-record verification, complete replay-metadata validation including createdAt custody, hard-link and crash-safe copy fallback, real Windows publication durability, extended UNC normalization, real subprocess custody/conflict coverage, persisted adversarial redaction, a registered 3-test agent-service dev-log target, and normal plus all-features gates in checked-in CI; all five ignored proof artifacts were regenerated against that source tree. WP03, WP06, WP07, WP08, and WP10 retain their documented partial-proof states. WP03 is still only partial-proof because the portal dev-log consumer slice is proved while the agent-service-to-logging-core row remains intentionally open outside this delegated boundary. WP06 is partial-proof because logging-owned proof-inventory checker/query behavior is real, but one root routing check still fails outside this delegated boundary. WP08 is honest partial-proof because its evidence proves the portal dev logger path, the logging-domain source/context storage/query path, and the agent-service startup/dev-log path without claiming repo-wide instrumentation adoption. WP07 and WP10 have proof evidence, but checklist closeout remains intentionally open.
```

Default order:

```text
remaining proof-inventory restoration or claim reduction -> WP03 Rust-side follow-through -> owning-slice dev-log-routing closure
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

Do not use WP07/WP10 proof roots to close unrelated missing roots. Do not use proof roots alone to override unchecked checklist rows or validation blockers.

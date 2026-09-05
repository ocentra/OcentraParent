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
| source-hardened / expected tests open | [WP02 TypeScript Logging Package Parity](workpacks/02-typescript-logging-package-parity.md) | 0/12 | `00-current-state-and-reference-audit.md` |
| partial-proof / route source hardened / expected tests open | [WP03 Parent Logging Architecture and Routing](workpacks/03-parent-logging-architecture-and-routing.md) | 10/11 | `01-parent-logging-architecture.md` |
| source-present | [WP09 Log Control, Retention, and Bridge Lifecycle](workpacks/09-log-control-retention-bridge-lifecycle.md) | 0/13 | `08-log-control-retention-bridge-lifecycle.md` |
| source-present | [WP04 Rust Logging Core Crate](workpacks/04-rust-logging-core-crate.md) | 0/12 | `02-rust-logging-core-crate.md` |
| source-present | [WP05 Local Validation Evidence](workpacks/05-local-validation-evidence.md) | 0/12 | `03-local-validation-evidence.md` |
| partial-proof / query source hardened / expected tests open | [WP07 MCP Query Interface](workpacks/07-mcp-query-interface.md) | 0/18 | `06-mcp-query-interface.md` |
| partial-proof / redaction source hardened / expected tests open | [WP08 Logger Instrumentation and Adoption](workpacks/08-logger-instrumentation-and-adoption.md) | 8/12 | `07-logger-instrumentation-pattern.md` |
| partial-proof | [WP10 Proof Trace Pipeline](workpacks/10-proof-trace-pipeline.md) | 0/13 | `09-proof-trace-pipeline.md` |
| partial-proof | [WP06 Validation and Enforcement](workpacks/06-validation-and-enforcement.md) | 11/12 | `04-validation-and-enforcement.md` |

Status meanings:

```text
audit-open: audit docs exist, but the named proof root is absent in this checkout
source-present: implementation/tests exist, but the named proof root and checklist closeout are not present in this checkout
partial-proof: focused proof root exists or focused tests passed, but checklist/workpack closeout remains open or broader validation still has visible blockers
```

The `720609306` source checkpoint is accepted only for the four bounded source
surfaces above. It changed no tests. Source status must not be promoted to DONE,
proof-current, or focused-validation green until the later expected-test wave
covers redaction edge cases, package exports, real Vite/writer routing, and
query containment/diagnostics.

The boxes column mirrors the current checklist completion count from `CHECKLIST_INDEX.md`, not the historical completion prose still embedded in some workpack closeout sections.

## Selection rules

Current audit note:

```text
WP03, WP06, WP07, WP08, and WP10 now have canonical proof roots in this checkout. WP03 is still only partial-proof because focused validation/proof closeout remains deferred and one root routing check is outside this delegated boundary; source inspection confirms its agent-service-to-logging-core mapping is live through app::health, service_runtime::run_agent_service, and activity_capture. WP06 is partial-proof because logging-owned proof-inventory checker/query behavior is real, but one root routing check still fails outside this delegated boundary. WP08 is honest partial-proof because its canonical root proves the portal dev logger path, the logging-domain source/context storage/query path, and the agent-service startup/dev-log path without claiming repo-wide instrumentation adoption. WP07 and WP10 have proof roots, but checklist closeout remains intentionally open.
```

Accepted source-wave note (2026-08-17): WP02 and WP04 retain
`source-present`; WP03 and WP08 retain `partial-proof`. The accepted source
head `735df89de` records the Rust-owned exact 18-key sensitive-key policy,
generated TypeScript artifact, generated-policy sanitizer consumption, Logger
pre-serialization sanitization, and portal fallback pre-serialization
sanitization. No test, proof, DONE, or external composition claim is implied.

Windows local-artifact owner route (2026-08-29): WP02 is the owner of the
absent `packages/logging-domain/src/local-artifact-mutation-provider.ts` seam.
The existing `src/local-artifact-*.ts`, `src/app-log/appNdjsonWriter.ts`,
`src/test-log/{ingestManifest,logsTree,ndjsonLogFileWriter,ndjsonPaths,ndjsonWriter}.ts`,
and `src/transport/bridgeServer.ts` paths are consumers of that owner. The
dedicated unit and integration provider roots are
`packages/logging-domain/tests/unit/local-artifact-mutation-provider.test.ts`
and
`packages/logging-domain/tests/integration/local-artifact-mutation-provider.test.ts`;
both are currently absent. The route is implementation-authorized only for
that unsatisfied seam after graph regeneration; it does not promote the
source-hardened status, test/proof rows, normal READY, or DONE.

The native expansion of this route (2026-08-29) is also WP02-owned because the
package has no production Node-native binding. The shippable shape is a
package-specific Windows FFI crate consumed by a safe
`crates/logging-core/src/local_artifact_mutation.rs` owner and a long-lived
Rust provider process, with
`packages/logging-domain/src/local-artifact-mutation-provider.ts` as the
TypeScript process/session adapter. The new native roots are
`crates/logging-local-artifact-windows-ffi/{Cargo.toml,src/lib.rs}` and
`crates/logging-local-artifact-provider/{Cargo.toml,src/main.rs}`; workspace
membership (`Cargo.toml`), the logging-core dependency edge, and package
build/runtime resolution (`packages/logging-domain/package.json`) are required
integration inputs. Existing dev/test child-process scripts, the protected
custody FFI crate, and the child-agent MSI are not this provider.

The expected real integration roots are
`crates/logging-core/tests/integration/local_artifact_mutation.rs`,
`crates/logging-local-artifact-windows-ffi/tests/integration/local_artifact_windows.rs`,
`crates/logging-local-artifact-provider/tests/integration/local_artifact_provider.rs`,
and
`packages/logging-domain/tests/integration/local-artifact-mutation-provider.test.ts`
(with the dedicated TypeScript unit root above). These roots are absent; the
route remains implementation-authorized but unsatisfied and does not claim
source, tests, proof, review, READY, or DONE.

Default order:

```text
remaining proof-inventory restoration or claim reduction -> deferred WP03 focused validation/proof -> owning-slice dev-log-routing closure
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

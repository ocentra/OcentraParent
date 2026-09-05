<!-- agent-capsule -->

> Agent Capsule
> Plan: `logging-domain-parity`
> Doc: `WP02 TypeScript Logging Package Parity`
> Kind: assigned implementation workpack.
> Read when: selected by WORKPACK_INDEX.md or explicit assignment.
> Stop rule: Do not open sibling workpacks unless a dependency is named here.
> Proves: TypeScript package parity only after tests/proof pass.
> Does not prove: unrelated Rust logging-core surfaces, local validation wrappers, or production readiness.
> Proof rule: Before DONE, run focused commands and write proof artifacts.

<!-- /agent-capsule -->

# WP02 TypeScript Logging Package Parity

## Purpose

Bring `packages/logging-domain` toward the mature games TypeScript package shape:

```text
test-log
transport
app-log
scripts
exports
scopes
```

This workpack is the first real implementation slice.

## Source inputs

```text
docs/plans/logging-domain-parity/00-current-state-and-reference-audit.md
docs/plans/logging-domain-parity/01-parent-logging-architecture.md
docs/plans/logging-domain-parity/05-codex-continuation-plan.md
ocentra-games/packages/logging-domain/package.json
ocentra-games/packages/logging-domain/src/test-log/**
ocentra-games/packages/logging-domain/src/transport/**
ocentra-games/packages/logging-domain/src/app-log/**
ocentra-games/packages/logging-domain/scripts/**
OcentraParent/packages/logging-domain/package.json
OcentraParent/packages/logging-domain/src/**
```

## Target state

Parent logging-domain has TypeScript parity modules and scripts adapted to parent scopes.

## Accepted source-wave reconciliation (2026-08-17)

The accepted source head `735df89de` establishes the TypeScript edge of the
redaction contract without closing this workpack. Rust owns the canonical
18-entry sensitive-key policy in
`crates/logging-core/src/redaction_policy.rs`; its checked-in generated output
is `packages/logging-domain/src/generated-log-redaction-policy.ts`, and
`packages/logging-domain/src/core/log-redaction.ts` consumes that artifact.
There is no alternate TypeScript-local sensitive-key list or regex policy.

This is source evidence only. The parity test, focused validation, proof-root,
checklist, and completion rows remain deferred and unchecked.

## Windows local-artifact mutation owner route (2026-08-29)

The current Windows failure is at a real package boundary, not at a test-only
fixture. `packages/logging-domain/src/local-artifact-path.ts` currently owns
path containment, canonicalization, directory ownership/currentness, identity
checks, and the explicit mutation-unsupported result used by the local-artifact
append, lock, transaction, retention, bridge, and NDJSON callers. Those callers
must remain consumers of one owner; they must not grow independent filesystem
authority.

The smallest missing production seam is the absent module:

```text
packages/logging-domain/src/local-artifact-mutation-provider.ts
```

WP02 owns this provider contract and its integration with the existing
`local-artifact-path.ts` boundary and callers. The provider is not a caller-
minted callback, path-only helper, boolean capability, mock, or temporary
fallback. It remains an implementation gap until the graph-mapped source exists.

The provider contract is fail-closed and must establish all of the following as
observable outcomes, without prescribing a platform implementation recipe:

- containment and canonical path: every target is proven inside the owned root
  in its canonical namespace; traversal, root replacement, ambiguous path
  forms, and canonicalization failure reject the mutation;
- symlink/reparse handling: links and Windows reparse substitutions in the
  root, ancestors, lock path, and target are rejected or independently proven
  safe before any mutation;
- directory ownership/currentness: the provider proves the expected directory
  identity and ownership, then revalidates it at each mutation boundary;
- atomic create/write/lock/recovery: creation, replacement, append, lock
  acquisition/release, cleanup, and recovery preserve the transaction and
  durability contract; partial, conflicting, or uncertain outcomes fail
  closed;
- capability provenance: only the provider can issue the opaque mutation
  authority after those checks; callers cannot mint, widen, replay, or replace
  that authority.

Required failure classes include containment/canonicalization failure,
symlink/reparse detection, ownership or identity drift, lock conflict, atomic
mutation failure, recovery uncertainty, and unsupported provider state. No
caller may fall back to direct path mutation or convert an unknown result to
success.

The graph route declares no hard predecessor for this package-owned seam.
WP09 remains downstream of WP02 and cannot claim lifecycle completion from this
route. The route authorizes only implementation-phase work for the absent
provider; it does not claim source completion, focused tests, proof, review,
normal READY, or DONE.

### Native Windows adapter route (2026-08-29)

Inspection of the current workspace found no production Node-native binding for
this boundary. Existing TypeScript `child_process` callers launch development,
test, or MCP tools, and the existing Windows FFI crate is restricted to
protected-capability custody. Neither is a local-artifact mutation provider.

The smallest shippable adapter therefore follows the repository's existing
Rust FFI plus bounded process pattern, without introducing N-API or a
path-oriented command helper:

```text
crates/logging-core/src/local_artifact_mutation.rs
  safe logging-core owner for the local-artifact mutation contract
crates/logging-local-artifact-windows-ffi/Cargo.toml
crates/logging-local-artifact-windows-ffi/src/lib.rs
  new package-specific Windows ABI/handle boundary; no raw handle escapes
crates/logging-local-artifact-provider/Cargo.toml
crates/logging-local-artifact-provider/src/main.rs
  new long-lived Rust provider process with a bounded framed protocol
packages/logging-domain/src/local-artifact-mutation-provider.ts
  TypeScript process/session adapter; it performs no direct filesystem mutation
```

The provider process must retain the owner-issued root/session state and return
only bounded mutation outcomes. Its protocol must not accept a caller-minted
capability, target identity, or path authority, and a lost process, malformed
frame, stale identity, or uncertain mutation must fail closed. The native
boundary must provide the canonical containment, reparse/symlink rejection,
directory ownership/currentness, atomic create/write/lock/recovery, and
durability observations required above. The existing protected-custody FFI
crate and child-agent MSI are not substitutes for this owner.

The build integration is part of the same implementation route but is not
present yet: add the two new crates to the workspace in `Cargo.toml`, add the
Windows-only dependency from `crates/logging-core/Cargo.toml` to the dedicated
FFI crate, and extend `packages/logging-domain/package.json` so the adapter
resolves a pinned built provider executable rather than invoking `cargo` in a
shipped runtime. A parent-desktop/release owner must separately stage and
integrity-check that executable; this WP02 route does not claim an installer or
release artifact.

The exact native and adapter route roots (with the new provider/FFI source
roots currently absent) are:

```text
Cargo.toml
crates/logging-core/Cargo.toml
crates/logging-core/src/lib.rs
crates/logging-core/src/local_artifact_mutation.rs
crates/logging-local-artifact-windows-ffi/Cargo.toml
crates/logging-local-artifact-windows-ffi/src/lib.rs
crates/logging-local-artifact-provider/Cargo.toml
crates/logging-local-artifact-provider/src/main.rs
packages/logging-domain/package.json
packages/logging-domain/src/local-artifact-mutation-provider.ts
```

The real Rust integration coverage must exercise the core owner, native
reparse/identity boundary, and provider process against a Windows filesystem;
the real TypeScript integration coverage must exercise the built provider
through the package adapter and then the existing local-artifact consumers.
Those expected roots are:

```text
crates/logging-core/tests/integration/local_artifact_mutation.rs
crates/logging-local-artifact-windows-ffi/tests/integration/local_artifact_windows.rs
crates/logging-local-artifact-provider/tests/integration/local_artifact_provider.rs
packages/logging-domain/tests/unit/local-artifact-mutation-provider.test.ts
packages/logging-domain/tests/integration/local-artifact-mutation-provider.test.ts
```

Until those source, build, and test roots exist and are independently
validated, WP02 remains implementation-authorized but unsatisfied. No source,
test, proof, checklist, review, normal READY, or DONE claim is made by this
route expansion.

Exact production consumer roots for this route are:

```text
packages/logging-domain/src/local-artifact-path.ts
packages/logging-domain/src/local-artifact-append-codec.ts
packages/logging-domain/src/local-artifact-append.ts
packages/logging-domain/src/local-artifact-file.ts
packages/logging-domain/src/local-artifact-lock-owner.ts
packages/logging-domain/src/local-artifact-lock.ts
packages/logging-domain/src/local-artifact-root.ts
packages/logging-domain/src/local-artifact-transaction-cleanup.ts
packages/logging-domain/src/local-artifact-transaction-codec.ts
packages/logging-domain/src/local-artifact-transaction-parsing.ts
packages/logging-domain/src/local-artifact-transaction-prepare.ts
packages/logging-domain/src/local-artifact-transaction-recovery.ts
packages/logging-domain/src/local-artifact-transaction-tree.ts
packages/logging-domain/src/local-artifact-transaction.ts
packages/logging-domain/src/local-artifact-tree.ts
packages/logging-domain/src/app-log/appNdjsonWriter.ts
packages/logging-domain/src/test-log/ingestManifest.ts
packages/logging-domain/src/test-log/logsTree.ts
packages/logging-domain/src/test-log/ndjsonLogFileWriter.ts
packages/logging-domain/src/test-log/ndjsonPaths.ts
packages/logging-domain/src/test-log/ndjsonWriter.ts
packages/logging-domain/src/transport/bridgeServer.ts
```

The real test ownership is the existing package unit and integration trees,
with these dedicated provider tests required and currently absent:

```text
packages/logging-domain/tests/unit/local-artifact-mutation-provider.test.ts
packages/logging-domain/tests/integration/local-artifact-mutation-provider.test.ts
```

Existing consumer tests that must exercise the provider boundary are:

```text
packages/logging-domain/tests/unit/bridge-run-lifecycle.test.ts
packages/logging-domain/tests/unit/bridge-transport.test.ts
packages/logging-domain/tests/unit/ensure-db-script.test.ts
packages/logging-domain/tests/unit/logs-tree.test.ts
packages/logging-domain/tests/unit/ndjson-log-file-writer.test.ts
packages/logging-domain/tests/unit/ndjson-writer.test.ts
packages/logging-domain/tests/unit/retention-cleanup.test.ts
packages/logging-domain/tests/unit/retention-logs-script.test.ts
packages/logging-domain/tests/unit/test-log-duckdb.test.ts
packages/logging-domain/tests/unit/wipe-logs-script.test.ts
packages/logging-domain/tests/unit/wipe-ndjson-scope.test.ts
packages/logging-domain/tests/integration/mcp-query-interface.test.ts
```

The reported 27 Windows test failures remain the current baseline. No source,
test, proof, checklist, or DONE claim is made by this routing correction.

Required module groups:

```text
packages/logging-domain/src/test-log/
packages/logging-domain/src/transport/
packages/logging-domain/src/app-log/
packages/logging-domain/scripts/
```

Required scripts:

```text
bridge
db:ensure
db:rebuild
db:ingest
logs:prepare
test:query
view:ndjson
```

## Scope rules

Required parent scopes:

```text
parent-agent
parent-portal
parent-cloudflare
parent-codex
parent-test
```

Do not hardcode generic parent logging to `cloudflare`.

Cloudflare can exist only as explicit `parent-cloudflare` scope.

## Required proof root

```text
output/logging-domain-parity-proof/02-typescript-logging-package-parity/
```

Required artifacts:

```text
00-package-export-before-after.json
01-typescript-parity-file-map.json
02-scope-defaults-proof.json
03-query-script-smoke.json
16-validation-commands.log
```

The retained artifact runner is:

```text
scripts/test/logging-domain-wp02-parity-proof.mjs
```

It executes the real package build, package tests, and bounded `parent-test`
stats query. It records explicit failed or blocked states and the local-package
no-claim boundary instead of converting unavailable evidence into success.
Its artifact-shape test is
`tests/repo-tooling/logging-domain-wp02-parity-proof.test.mjs`.

## Checklist rows

- [x] `src/test-log` parity modules added/adapted.
- [x] `src/transport` parity modules added/adapted.
- [x] `src/app-log` parity modules added/adapted or explicit deferral recorded.
- [x] `scripts/log-bridge.ts` added.
- [x] DB ensure/rebuild/ingest/query/view scripts added.
- [x] Package exports updated explicitly.
- [x] Existing proof/contract exports preserved.
- [x] Parent scopes added without generic Cloudflare default.
- [x] TypeScript tests added/updated.
- [x] Focused package build/test commands pass.
- [x] Proof root written.
- [x] Workpack completion section filled.

## Expected source changes

Likely files:

```text
Cargo.toml
crates/logging-core/Cargo.toml
crates/logging-core/src/lib.rs
crates/logging-core/src/local_artifact_mutation.rs
crates/logging-local-artifact-windows-ffi/Cargo.toml
crates/logging-local-artifact-windows-ffi/src/lib.rs
crates/logging-local-artifact-provider/Cargo.toml
crates/logging-local-artifact-provider/src/main.rs
packages/logging-domain/package.json
packages/logging-domain/src/local-artifact-mutation-provider.ts
packages/logging-domain/src/test-log/**
packages/logging-domain/src/transport/**
packages/logging-domain/src/app-log/**
packages/logging-domain/scripts/**
packages/logging-domain/tests/**
```

Do not remove existing exports unless migration is documented and tests prove no consumers break.

## Additional validation from continuation note

`05-codex-continuation-plan.md` adds useful TypeScript validation details. Include them in this workpack:

```text
transport serialization and send behavior
invalid bridge payload handling
app/test-log retention behavior
NDJSON serialization shape
DuckDB ingest/query smoke
existing production-safe contract exports still parse
```

Do not treat this as a new workpack. It strengthens WP02 validation.

## Focused commands

```bash
npm run build --workspace @ocentra-parent/logging-domain
npm run test --workspace @ocentra-parent/logging-domain
```

If new query scripts are ready:

```bash
npm run test:query --workspace @ocentra-parent/logging-domain -- stats --scope=parent-test
```

After source and focused tests pass, retain the WP02 proof with:

```bash
npm run proof:logging-domain-wp02 -- --base=<reviewed-base-ref>
```

The proof command is deliberately separate from source validation. Adding the
runner or its tests does not check the proof/checklist rows.

## Manual-required gaps

This workpack does not implement Rust logging-core or validation wrappers. Those belong to WP04/WP05.

## Fill before DONE or PR-ready

```text
Workpack id and branch: WP02 TypeScript Logging Package Parity;
  codex/eventing-wp09-production; reviewed base
  dd65d5d1318db7e7c02d1c6c747b8e5ce9d28c4c.
Touched files: packages/logging-domain package, source, scripts, and tests;
  logging-core, logging-local-artifact-windows-ffi, and
  logging-local-artifact-provider owner/runtime/tests; the dedicated WP02
  proof runner, runner test, and this workpack.
Validation commands and results:
  npm run build --workspace @ocentra-parent/logging-domain -> passed;
  npm run test --workspace @ocentra-parent/logging-domain -> passed;
  npm run test:query --workspace @ocentra-parent/logging-domain --
    stats --scope=parent-test -> passed with the bounded stats schema;
  npm run proof:logging-domain-wp02 -- --base=HEAD -> passed;
  npm run validate:logging -> passed;
  npm run test:logging-evidence -> passed;
  logging provider all-target check, provider integration, Windows FFI
    integration, focused architecture, formatting, and diff checks -> passed.
Proof artifacts:
  output/logging-domain-parity-proof/02-typescript-logging-package-parity/
  contains exactly the five required retained artifacts, all passed.
Product/runtime claims: local TypeScript package parity and its fail-closed
  Windows local-artifact mutation-owner path only. This does not claim
  production telemetry, installer/release custody, or product-wide logging
  readiness.
Known gaps/manual-required states: repository-wide validation, normal
  pre-commit, CI, and promotion remain final integration gates outside this
  workpack's bounded completion claim.
```

## Current audit note

`packages/logging-domain` now contains the TypeScript package surface this
workpack expected, including the `core`, `test-log`, `transport`, and
`app-log` areas plus targeted unit and integration tests. The June 16, 2026
audit also re-verified focused package coverage through
`packages/logging-domain/tests/integration/mcp-query-interface.test.ts`,
`packages/logging-domain/tests/unit/logger.test.ts`, and
`packages/logging-domain/tests/unit/dev-log-fixture.test.ts`.

The canonical lane now contains exactly the five required retained artifacts
under
`output/logging-domain-parity-proof/02-typescript-logging-package-parity/`.
All five record passed bounded evidence, and the retained command log records
successful package build, package test, and `parent-test` query commands. The
durable manifest at
`docs/proof/logging-domain-parity/WP02_TYPESCRIPT_LOGGING_PACKAGE_PARITY_PROOF.md`
records their exact sizes, SHA-256 digests, command results, and no-claim
boundary.

The workpack source, expected tests, focused validation, and proof are current.
Normal graph completion remains blocked only by the central
`CHECKLIST_INDEX.md` rows, which were not changed in this pass because another
active Logging proof-restoration task owns that exact file. The dated missing-
provider and missing-test notes above are historical routing records superseded
by the current completion block and this audit note; they are not current gaps.

## Accepted source and expected-test delta (2026-08-17, superseded)

The reviewed source at `720609306` adds one canonical structured-redaction
policy, explicit package exports, JSON-safe markers for unsupported values,
cycle/reflection failure containment, and native Date/URL/custom-`toJSON`
handling. No test file changed. Before focused execution, add dedicated
redaction coverage for nested secrets, arrays, cycles, unsupported primitives
and objects, throwing getters/proxies/`toJSON`, root/property/index key
semantics, one-call behavior, and package-export import resolution. This is
the historical pre-test state. The required focused test matrix and retained
proof now exist; the no-production-telemetry boundary remains unchanged.

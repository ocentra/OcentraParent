<!-- agent-capsule -->

> Agent Capsule
> Doc: Validation Gates
> Kind: architecture/reference documentation; read only when selected by plan route, source router, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

# Validation Gates

## Root Gate

`npm run validate` is the default local confidence gate.

It runs:

- release version alignment
- pre-AI real evidence proof matrix
- repository secret scan
- dependency security and license policy in CI
- schema-boundary checks
- Rust format and clippy warnings as errors
- package lint
- package type-check
- package tests
- Rust workspace check
- Rust workspace tests
- WebSocket integration smoke
- LAN bind and origin smoke
- portal local smoke
- portal Playwright UI check against the real Rust service

`npm run build` is the portal/package build gate and should pass before any scaffold is considered usable.

## Local Commit Gate

The pre-commit hook is a fast local source gate. It keeps commits from bypassing
formatting, schema/source/test-double guards, hook/tooling tests, Rust
formatting, and Rust workspace compilation.

It intentionally does not run the real-service smoke tests, portal Playwright
E2E, package lint/type-check tasks, TypeScript/Rust unit suites, production
build, or package previews on every local commit. Those remain integration gates
for PR-ready handoff, explicit local confidence runs, and CI.

Use these commands when the heavier gates are needed:

```powershell
npm run test:local
npm run test
npm run precommit:full
npm run validate
npm run ci:local
npm run test:e2e
```

CI adds installer/package reality checks after the root gate:

- Pre-AI proof matrix check for completed runtime claims and platform proof
  status.
- Real portal-to-Rust E2E on Windows, Linux, and macOS runners.

- Windows MSI install, service presence check, and uninstall.
- Linux DEB install, systemd unit check, and remove.
- macOS PKG payload expansion and launchd payload check.
- Android APK emulator install and launch.
- iOS simulator app install and launch.

## Boundary Checks

The scaffold rejects:

- direct Zod source usage
- direct Zod package dependencies
- manual string brands
- naked domain string aliases for identifiers, paths, names, routes, hashes, urls, statuses, and similar domain values
- inline app runtime string literals
- raw `string` annotations in app runtime TypeScript source
- inline Rust service/core string literals
- source workspaces or Rust crates without tests
- mocks, fakes, stubs, spies, MSW, Nock, Sinon, `vi.mock`, `vi.fn`, and equivalent test doubles
- oversized source files, oversized functions, too many classes, or too many exports in one file

String values are allowed inside domain-owned schema/constant/text packages because that is where runtime values become named, validated contracts.

Source size checks use two levels:

- At 80% of the limit, validation prints a warning so the next change can split the file before it becomes a problem.
- Past the hard limit, validation fails.

Documentation is not governed by the same source-code length limits. Long-form docs should still be organized, but the god-file guard is aimed at code ownership and behavior concentration.

## Test Reality

Tests must use real domain contracts, real parsers, real local service processes, and real transport loops. If a dependency is too hard to test without replacement, the production design needs a smaller real boundary rather than a fake test path.

Runtime proof follows
[`docs/expectations/real-evidence-proof.md`](../expectations/real-evidence-proof.md).
Temporary runtime directories, journal paths, SQLite paths, and keys are allowed
as setup. Manually inserted rows, replaced service responses, or portal-local
success state are not valid proof for a completed runtime claim.

## Editor Lint

`eslint.config.js` loads local rules from `eslint-rules/` so editor integrations and package lint tasks fail on the same Ocentra-specific boundaries:

- `ocentra-parent/no-app-string-literals`
- `ocentra-parent/no-runtime-string-types`
- `ocentra-parent/no-naked-domain-string-types`

These rules are tested by `npm run test:tooling`, which is part of `npm run test` and `npm run validate`.

## Rust Gate

Rust validation runs across the entire Cargo workspace.

```powershell
npm run format:rust
npm run lint:rust
npm run validate:rust
```

The service binary must use Tokio's multithreaded runtime and async request handling. Blocking work has to be isolated behind explicit adapters before it enters command handlers.

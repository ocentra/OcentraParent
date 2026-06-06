# WP80 Source-Gated Policy Preview Package Exports

## AI Worker Checklist

- [x] Confirm source docs read: `docs/feature-list.md`,
      `docs/features/app-game-control.md`,
      `docs/expectations/app-game-evidence.md`,
      `docs/expectations/policy.md`, and `packages/parent-domain/README.md`.
- [x] Confirm browser-game scope remains in browser-plan.
- [x] Confirm apps and games share low-level evidence but keep separate product
      meaning.
- [x] Hub lock covers this workpack and exact implementation/docs paths.
- [x] Existing source layout inspected; no parallel app-control or game-control
      truth created.
- [x] Before-state source snapshot recorded in
      `output/app-game-plan-proof/80-source-gated-policy-preview-package-exports/00-source-snapshot.md`.
- [x] Contracts already exist before package exports are exposed.
- [x] Rust/service/portal parity not updated because this is a parent-domain
      package export proof only.
- [x] Tests/proof listed in this workpack are implemented.
- [x] Security/no-claim negative proof captured: package exports do not emit a
      service event, render portal UI, run the evaluator, start or schedule a
      timer, dispatch an adapter, deliver child UX, enforce platform controls,
      or expose raw private source rows.
- [x] Feature/expectation/product-checklist/README update decision recorded.
- [x] Known gaps, deferred items, and no-claim boundaries recorded.

## Scope

Expose the WP76 source-gated policy preview read-model, WP78 timer-handoff, and
WP79 timer-status parent-domain contracts through package subpaths. The proof
builds the package and verifies the public subpaths point to generated JS and
type artifacts.

## Evidence

- `packages/parent-domain/package.json`
- `packages/parent-domain/tests/app-game-source-gated-policy-preview-package-exports.test.ts`
- `scripts/test/app-game-source-gated-policy-preview-package-exports-proof.mjs`
- `test-results/app-game-source-gated-policy-preview-package-exports-proof/proof.json`
- `output/app-game-plan-proof/80-source-gated-policy-preview-package-exports/proof.json`

## No-Claim Boundaries

- No service runtime event.
- No portal UI.
- No policy evaluator runtime.
- No timer runtime.
- No timer scheduling.
- No adapter dispatch.
- No child delivery.
- No platform enforcement.
- No raw private source rows.

Product capability checklist remains unchanged because this work exposes
existing contract surfaces without moving product completion.

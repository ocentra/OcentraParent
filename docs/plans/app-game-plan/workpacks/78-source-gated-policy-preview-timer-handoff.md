# WP78 Source-Gated Policy Preview Timer Handoff

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
      `output/app-game-plan-proof/78-source-gated-policy-preview-timer-handoff/00-source-snapshot.md`.
- [x] Contracts updated first where behavior changes.
- [x] Rust/service/portal parity not updated because this is a parent-domain
      timer-handoff readiness proof only.
- [x] Tests/proof listed in this workpack are implemented.
- [x] Security/no-claim negative proof captured: timer handoff does not start a
      timer, dispatch an adapter, deliver child UX, enforce platform controls,
      or expose raw private source rows.
- [x] Feature/expectation/product-checklist/README update decision recorded.
- [x] Known gaps, deferred items, and no-claim boundaries recorded.

## Scope

Create a parent-domain proof that consumes WP76 source-gated policy preview
read-model rows and classifies which rows are eligible for future timer
sequencing. Preview-ready rows become timer sequencing candidates, while
source-manual and compiler-manual rows stay blocked before timer runtime.

This workpack intentionally avoids `packages/parent-domain/package.json`
because another lane owns that manifest lock.

## Evidence

- `packages/parent-domain/src/app-game-source-gated-policy-preview-timer-handoff.ts`
- `packages/parent-domain/src/app-game-source-gated-policy-preview-timer-handoff-rules.ts`
- `packages/parent-domain/tests/app-game-source-gated-policy-preview-timer-handoff.test.ts`
- `scripts/test/app-game-source-gated-policy-preview-timer-handoff-proof.mjs`
- `test-results/app-game-source-gated-policy-preview-timer-handoff-proof/proof.json`
- `output/app-game-plan-proof/78-source-gated-policy-preview-timer-handoff/proof.json`

## No-Claim Boundaries

- No package manifest export.
- No service runtime event.
- No portal UI.
- No policy evaluator runtime.
- No timer runtime.
- No adapter dispatch.
- No child delivery.
- No platform enforcement.
- No raw private source rows.

# WP80 Source-Gated Policy Preview Package Exports

Cross-recorded from the shared app/game plan.

## Scope

Native app source-gated preview read-model, timer-handoff, and timer-status
contracts are now exposed through parent-domain package subpaths. The proof
builds the package and verifies the generated JS and type artifacts for each
subpath.

## Evidence

- `packages/parent-domain/package.json`
- `packages/parent-domain/tests/app-game-source-gated-policy-preview-package-exports.test.ts`
- `scripts/test/app-game-source-gated-policy-preview-package-exports-proof.mjs`
- `test-results/app-game-source-gated-policy-preview-package-exports-proof/proof.json`
- `output/app-plan-proof/80-source-gated-policy-preview-package-exports/proof.json`

## No-Claim Boundaries

This does not emit service/runtime events, render portal UI, run the policy
evaluator, start or schedule timers, dispatch adapters, deliver child UX, prove
broad app blocking, enforce platform controls, or expose raw private source
rows.

Product capability checklist remains unchanged because no feature status moved.

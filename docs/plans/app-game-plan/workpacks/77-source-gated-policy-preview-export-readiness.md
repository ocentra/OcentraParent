# 77 Source-Gated Policy Preview Export Readiness

## Scope

Prove that the WP76 source-gated policy preview read model has a stable public
surface ready for the next package-manifest sequencing pass without editing the
shared `packages/parent-domain/package.json` lock.

This workpack consumes the WP76 read model, names the exact future package
subpath and symbols, preserves native app/native game row coverage, and keeps
package manifest, service runtime, portal UI, policy evaluator, timer, adapter
dispatch, child delivery, platform enforcement, and raw source-row claims false.

## Implementation

- Add a parent-domain export-readiness contract for the WP76 read model.
- Verify the future package subpath:
  `./app-game-source-gated-policy-preview-read-model`.
- Verify the public symbols required for package export sequencing.
- Keep package manifest mutation deferred while another lane owns
  `packages/parent-domain/package.json`.
- Reuse the WP76 source-gated read model instead of creating a second app/game
  policy-preview truth.

## Proof

- `cmd /c npm run build --workspace @ocentra-parent/parent-domain`
- `cmd /c npm run test --workspace @ocentra-parent/parent-domain -- app-game-source-gated-policy-preview-export-readiness app-game-source-gated-policy-preview-read-model`
- `node scripts/test/app-game-source-gated-policy-preview-export-readiness-proof.mjs`
- `cmd /c npm run format:check`
- `cmd /c npm run lint --workspace @ocentra-parent/parent-domain`
- `git diff --check`
- `cmd /c npm run lanes:guard`
- `cmd /c npm run hub:guard`

Proof artifacts live in:

```text
output/app-game-plan-proof/77-source-gated-policy-preview-export-readiness
```

## No-Claim Boundaries

- The package manifest export remains unmodified in this workpack.
- No service runtime event, portal renderer, policy evaluator runtime, timer,
  adapter dispatch, child delivery, platform enforcement, or raw private
  source-row claim is added.
- WP77 is stacked on WP76 until the source-gated read model lands.

## Product Doc Decision

`docs/product-capability-checklist.md` is unchanged because no feature status
moved. This is a sequencing/readiness proof for the already documented WP76
read model, and the remaining package-manifest edit is deferred to the lane that
owns `packages/parent-domain/package.json`.

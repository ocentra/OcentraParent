# 77 Source-Gated Policy Preview Export Readiness

## Scope

Cross-record the shared app/game WP77 proof for native-app policy preview export
readiness. The proof consumes the WP76 source-gated policy preview read model,
names the future parent-domain package subpath and public symbols, and keeps the
package manifest edit deferred while another lane owns the shared package file.

## Implementation

- Reuse the shared app/game source-gated policy preview read model.
- Prove native app rows remain represented in the export-readiness contract.
- Keep broad app blocking, service runtime, portal UI, policy evaluator, timer,
  adapter dispatch, child delivery, platform enforcement, and raw source-row
  claims false.
- Do not edit `packages/parent-domain/package.json` in this workpack.

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
output/app-plan-proof/77-source-gated-policy-preview-export-readiness
```

## No-Claim Boundaries

- No package export is added until the shared manifest lock clears.
- No service, portal, evaluator, timer, child delivery, adapter, platform
  enforcement, or raw private source-row claim is added.

## Product Doc Decision

`docs/product-capability-checklist.md` is unchanged because no native-app status
moved. The work only proves the future package export surface for already
documented WP76 source-gated preview rows.

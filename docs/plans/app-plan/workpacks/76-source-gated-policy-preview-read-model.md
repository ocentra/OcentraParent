# WP76 Native App Source-Gated Policy Preview Read Model

## Scope

Cross-record the shared app/game WP76 proof for native apps. The read model
projects WP75 source freshness preview gate rows into a redacted parent-domain
shape that future service and portal consumers can render without rechecking
raw source rows or claiming runtime policy execution.

Native games continue to use the same low-level evidence spine with separate
product meaning through the shared app/game target-domain fields.

## Owned Proof

- Shared workpack:
  `docs/plans/app-game-plan/workpacks/76-source-gated-policy-preview-read-model.md`
- Evidence:
  `output/app-plan-proof/76-source-gated-policy-preview-read-model`
- Test harness:
  `scripts/test/app-game-source-gated-policy-preview-read-model-proof.mjs`

## Acceptance

- Native app rows are derived from the source freshness preview gate, not from
  raw private source rows.
- Source-manual-required app rows would remain visible before policy preview
  output is accepted.
- Preview-ready native app rows remain read-only and dry-run-oriented.
- Counts and no-claim flags match the underlying WP75 gate model.

## Non-Goals

- No native app portal authoring or renderer work.
- No service persistence or WebSocket event.
- No package export while another lane owns `packages/parent-domain/package.json`.
- No platform adapter execution, child delivery, or broad app blocking.

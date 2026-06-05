# 27 Screenshot Retention Optional Mode

## Target State

Separate opt-in raw screenshot retention design exists with custody, TTL, disclosure, export, and delete proof.

## Current State

Raw screenshot retention is not default and not product-complete.
`ScreenEvidenceRemoteBoundarySettingSchema` now records the current product
decision as disabled for raw screenshot retention, and
`scripts/test/screen-evidence-settings-retention-proof.mjs` proves the schema
rejects any raw-retention mode outside that disabled state.
The Settings writable-intent proof renders the same disabled retention/live-view
boundary while local screen-summary drafts are enabled.

## Checklist

- [x] Record product decision.
- [x] Keep default `retainRawImage=false`.
- [ ] Define explicit opt-in setting if approved.
- [ ] Define custody and TTL.
- [ ] Define export/delete behavior.
- [ ] Define disclosure/audit.
- [x] Add separate proof.

## Proof

- Feature/checklist update.
- Tests proving retention cannot silently enable.
- `output/screen-plan-proof/remote-retention-boundary/proof-summary.json`.

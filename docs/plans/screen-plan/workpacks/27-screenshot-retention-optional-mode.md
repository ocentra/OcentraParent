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
`ScreenRawScreenshotRetentionOptInSettingSchema` and
`scripts/test/screen-optional-retention-live-preflight-proof.mjs` now define and
prove the separate optional retention preflight contract for disabled,
local-short-TTL, and parent-owned-export modes. The contract requires explicit
parent approval, audit ref, custody state, TTL for enabled modes, delete proof,
delete-on-disable behavior, and schema-forced no raw screenshot remote upload.
This is a contract/preflight proof only; runtime retention enablement, service
persistence, parent UI, platform proof, and privacy/legal approval remain
separate gates.

## Checklist

- [x] Record product decision.
- [x] Keep default `retainRawImage=false`.
- [x] Define explicit opt-in setting if approved.
- [x] Define custody and TTL.
- [x] Define export/delete behavior.
- [x] Define disclosure/audit.
- [x] Add separate proof.

## Proof

- Feature/checklist update.
- Tests proving retention cannot silently enable.
- `output/screen-plan-proof/remote-retention-boundary/proof-summary.json`.
- `output/screen-plan-proof/27-28-optional-retention-live-preflight/proof-summary.json`.

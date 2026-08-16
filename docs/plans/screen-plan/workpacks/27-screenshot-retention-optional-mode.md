# 27 Screenshot Retention Optional Mode

<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-plan`
> Doc: `27 Screenshot Retention Optional Mode`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

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

- [ ] Record product decision.
- [ ] Keep default `retainRawImage=false`.
- [ ] Define explicit opt-in setting if approved.
- [ ] Define custody and TTL.
- [ ] Define export/delete behavior.
- [ ] Define disclosure/audit.
- [ ] Add separate proof.

## Proof

- Feature/checklist update.
- Tests proving retention cannot silently enable.
- `output/screen-plan-proof/remote-retention-boundary/proof-summary.json`.
- `output/screen-plan-proof/27-28-optional-retention-live-preflight/proof-summary.json`.

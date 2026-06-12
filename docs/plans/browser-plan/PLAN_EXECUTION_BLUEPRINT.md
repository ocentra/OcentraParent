# Browser Plan � HID Execution Blueprint

## Execution objective

Make managed browser behavior deterministic with request safety, custody-aware profile states, and audited intervention.

## Slice 01 � Managed Profile and Custody

### Acceptance

- Profile boundaries, redaction, and restart/repair behavior are proven.

### Tests

- `browser.profile.custody-redaction`
- `browser.policy.authz-idempotency`

### Proof

- `docs/proof/browser-plan/slice-01-profile-custody.md`

## Slice 02 � Setting/Schema Safety

### Acceptance

- Settings/state contracts reject invalid values and unknown state transitions.

### Tests

- `browser.setting.schema-boundary`
- `browser.policy.authz-idempotency`

### Proof

- `docs/proof/browser-plan/slice-02-setting-schema.md`

## Slice 03 � Network and Request Security

### Acceptance

- Origin/header/host/redirect/path handling fails closed for attack vectors.

### Tests

- `browser.origin.header-security`
- `browser.security.request-smuggling-desync`

### Proof

- `docs/proof/browser-plan/slice-03-network-security.md`

## Slice 04 � Intervention and Rollback

### Acceptance

- Intervention calls are idempotent and reversible via rollback states.

### Tests

- `browser.policy.authz-idempotency`
- `browser.rate-limit.abuse`

### Proof

- `docs/proof/browser-plan/slice-04-intervention-rollback.md`

## Workpacks (execution lane)

### Slice-to-workpack binding

- Slice 01: docs/plans/browser-plan/workpacks/01-contract-boundary-and-effect-schemas.md
- Slice 02: docs/plans/browser-plan/workpacks/02-source-index-and-doc-reconciliation.md
- Slice 03: docs/plans/browser-plan/workpacks/03-browser-inventory-model.md
- Slice 04: docs/plans/browser-plan/workpacks/04-windows-browser-inventory-adapter.md

## PR-ready gate

- No browser PR claim until audit logs and request security negatives are passed and linked.

## HID test floor (this plan)

### Required test families for closed slice

- Unit: setting/profile schema boundaries
- Integration: intervention and lifecycle handoffs
- E2E: managed/unmanaged runtime and control paths
- Security: origin/header/request security and request-split probes
- Non-functional: rate-limit and retry stability

### Mandatory slice evidence checks

- negative cases documented (at least one per slice)
- rollback/teardown proof recorded
- proof manifest references command output, artifacts, and manual review notes

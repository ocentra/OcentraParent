# 39 Redacted Summary Only Remote Boundary

## Target State

Remote/API path is disabled for raw screenshots and allowed only for parent-approved redacted summaries.

## MVP Boundary

Capture MVP must enforce the default prohibition. Any remote model work belongs to AI/privacy pass.
`ScreenEvidenceRemoteBoundarySettingSchema` now enforces that prohibition:
raw screenshot remote upload is schema-forced false, disabled remote summaries
cannot carry approval refs or destination custody, and the only accepted remote
summary mode requires a parent approval ref plus parent-owned export custody.

## Checklist

- [x] Define remote disabled default.
- [x] Block raw screenshot remote upload.
- [x] Allow only parent-approved redacted summaries when explicitly enabled.
- [x] Record parent approval/audit fields.
- [x] Add negative tests for raw image upload.
- [ ] Add privacy/legal review gate.

## Proof

- Security tests.
- Parent approval/audit proof.
- `output/screen-plan-proof/remote-retention-boundary/proof-summary.json`.

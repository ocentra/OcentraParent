# 39 Redacted Summary Only Remote Boundary

<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-plan`
> Doc: `39 Redacted Summary Only Remote Boundary`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

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

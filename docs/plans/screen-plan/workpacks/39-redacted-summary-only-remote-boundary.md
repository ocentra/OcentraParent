# 39 Redacted Summary Only Remote Boundary

<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-plan`
> Doc: `39 Redacted Summary Only Remote Boundary`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR_READY, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Ownership boundary

```text
screen-plan owns the default prohibition on raw screenshot remote upload and the redacted-summary-only screen boundary.
remote-access-plan owns remote live-view/session/relay authority.
data-custody-storage-plan owns parent-owned export custody, retention, delete, and privacy policy.
ai-plan/privacy work owns remote model behavior if later selected.
```

## Target State

Remote/API path is disabled for raw screenshots and allowed only for parent-approved redacted summaries.

## MVP Boundary

Capture MVP must enforce the default prohibition. Any remote model work belongs to AI/privacy pass.
`ScreenEvidenceRemoteBoundarySettingSchema` now enforces that prohibition: raw screenshot remote upload is schema-forced false, disabled remote summaries cannot carry approval refs or destination custody, and the only accepted remote summary mode requires a parent approval ref plus parent-owned export custody.

## Required proof fields

The selected proof must name, at minimum:

```text
remote_disabled_default_state
raw_screenshot_remote_upload_state
redacted_summary_mode_state
parent_approval_ref_state
audit_ref_state
export_custody_state
destination_custody_state
disabled_summary_state
negative_raw_upload_state
privacy_legal_state
remote_model_boundary_state
remote_access_boundary_state
no_live_view_claim
no_raw_remote_upload_claim
no_claim
```

These are proof-routing fields, not implementation code prescriptions.

## Checklist

- [ ] Define remote disabled default.
- [ ] Block raw screenshot remote upload.
- [ ] Allow only parent-approved redacted summaries when explicitly enabled.
- [ ] Record parent approval/audit fields.
- [ ] Add negative tests for raw image upload.
- [ ] Add privacy/legal review gate.

## Proof

- Security tests.
- Parent approval/audit proof.
- `output/screen-plan-proof/remote-retention-boundary/proof-summary.json`.

## Failure conditions

- Do not allow raw screenshot remote upload.
- Do not allow redacted summary export without parent approval/audit and custody state.
- Do not claim remote live-view, relay, or remote model readiness from this boundary proof.
- Do not mark privacy/legal complete without an explicit approval artifact.

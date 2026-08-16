# 09 Windows Capture Adapter Plan Proof

<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-plan`
> Doc: `09 Windows Capture Adapter Plan Proof`
> Kind: proof reference; read only when validating matching claim.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR_READY, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Ownership boundary

```text
screen-plan owns Windows adapter proof requirements, permission/degraded/protected-surface states, queue write, and deletion proof.
screen-ai-pipeline-plan owns downstream AI/policy product-path proof.
remote-access-plan owns relay-backed remote live-view authority.
```

## Target State

Windows screen source, picker/consent/border, display/window evidence, and protected/degraded states are proved before Windows support is claimed.

## Current State

Windows is the preferred first desktop proof target. Implementation proof is open.

## Required proof fields

The selected proof must name, at minimum:

```text
windows_docs_verified_state
capability_probe_state
picker_consent_state
border_indicator_state
display_source_state
window_source_state
managed_browser_window_state
protected_surface_state
degraded_state
queue_write_state
delete_after_capture_state
log_redaction_state
screenshot_artifact_state
cross_platform_claim_state
live_view_claim_state
no_ai_claim
no_remote_claim
no_claim
```

These are proof-routing fields, not implementation code prescriptions.

## Checklist

- [ ] Verify current Microsoft official screen-source docs.
- [ ] Add Windows capability probe.
- [ ] Prove display source.
- [ ] Prove app/window source.
- [ ] Prove managed browser window source.
- [ ] Prove protected-surface skip/degraded state.
- [ ] Prove queue write and deletion.

## Proof

- `output/screen-plan-proof/windows/`.
- Local Windows logs/screenshots.

## Failure conditions

- Do not claim macOS/Linux/Android/iOS parity from Windows proof.
- Do not claim live view, remote access, or screen-AI pipeline completion from Windows screen-source proof.
- Do not retain raw screenshot material without explicit deletion/retention proof.

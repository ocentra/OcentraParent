# 05 - Policy Action Dry-Run Gate

<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-ai-pipeline-plan`
> Doc: `05 - Policy Action Dry-Run Gate`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Ownership boundary

```text
screen-ai-pipeline-plan owns policy action dry-run and action-boundary proof for the screen-AI path.
policy-control-plane-plan owns policy authority, preview, parent confirmation, ask-parent, and rule precedence.
v0-8-enforcement-control-plan owns adapter execution, rollback, and runtime enforcement authority.
app/browser/network/mobile/domain plans own their respective action adapters when selected.
```

## Target State

Pipeline proves policy actions or dry-run actions without letting AI enforce directly.

## Required proof fields

The selected proof must name, at minimum:

```text
scenario_id
policy_decision_state
action_kind
dry_run_state
observe_state
allow_state
warn_state
ask_parent_state
time_limit_state
block_dry_run_state
adapter_boundary_state
timer_expiry_ref_state
audit_event_state
unknown_manual_required_state
unsupported_adapter_state
enforcement_runtime_state
no_direct_ai_enforcement_claim
no_adapter_execution_claim
no_product_claim
```

These are proof-routing fields, not implementation code prescriptions.

## Checklist

- [ ] Observe action proof.
- [ ] Allow action proof.
- [ ] Warn action proof.
- [ ] Ask-parent action proof.
- [ ] Time-limit action proof with timer/expiry refs.
- [ ] Block dry-run proof and owned-process Windows adapter proof.
- [ ] Unknown/manual-required proof.

## Proof

- Action or dry-run artifact.
- Time-limit adapter artifact: `output/screen-ai-pipeline-proof/action-dispatch/proof-summary.json`.
- Block adapter artifact: `output/screen-ai-pipeline-proof/block-action-dispatch/proof-summary.json`.
- Enforcement-adapter non-claim for browser, category, network/domain, mobile, and broad block paths until those adapters have real proof.
- Audit event artifact.

## Failure conditions

- Do not claim enforcement runtime proof from dry-run proof.
- Do not claim browser/category/network/mobile/broad block adapters until their owning plans prove them.
- Do not let AI bypass policy authority or parent confirmation boundaries.
- Do not claim action completion without audit refs and manual-required/unsupported states.

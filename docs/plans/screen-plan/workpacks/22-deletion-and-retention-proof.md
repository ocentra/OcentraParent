# 22 Deletion And Retention Proof

<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-plan`
> Doc: `22 Deletion And Retention Proof`
> Kind: proof reference; read only when validating matching claim.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR_READY, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Ownership boundary

```text
screen-plan owns screen-local raw image deletion, deletion proof refs, delete-failed visibility, and no-default-raw-retention proof.
data-custody-storage-plan owns product retention/export/delete/privacy policy and parent-owned storage behavior.
portal UX owns rendered deletion state only when selected.
```

## Target State

Delete after success, delete after expiry, delete-failed visible state, deletion proof refs, and no silent long-term raw image behavior are implemented.

## Current State

Retention/deletion is specified but not fully proved.

## Required proof fields

The selected proof must name, at minimum:

```text
queue_before_state
queue_after_state
delete_success_state
delete_after_expiry_state
delete_failed_state
visible_delete_failed_state
deletion_proof_ref_state
raw_retention_state
retention_opt_in_state
remote_upload_state
portal_deletion_state
custody_handoff_state
no_long_term_raw_image_claim
no_product_retention_claim
no_claim
```

These are proof-routing fields, not implementation code prescriptions.

## Checklist

- [ ] Delete raw image after success.
- [ ] Delete raw image after expiry.
- [ ] Record delete-failed state.
- [ ] Record deletion proof ref.
- [ ] Show deletion state in portal.
- [ ] Prove no default long-term raw image retention.

## Proof

- Queue directory before/after proof.
- Store record deletion proof.
- Portal screenshot.

## Failure conditions

- Do not claim deletion readiness without before/after and store-record proof.
- Do not hide delete-failed states.
- Do not claim product retention controls from local deletion proof.
- Do not permit default long-term raw image retention.

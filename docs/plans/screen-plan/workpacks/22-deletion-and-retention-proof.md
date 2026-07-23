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

Independent review invalidated the prior WP22 packet: it exercised a proof
harness rather than the production-safe deletion path. All six outcomes are
open pending retention, durable queue mutation, transactional deletion state,
capture redaction, production projection, and hardened proof custody work.
[`docs/proof/screen-plan/wp22-deletion-retention-proof.md`](../../../proof/screen-plan/wp22-deletion-retention-proof.md).
The portal proof uses the runner's exact-spec environment route, so it runs the
screen-summary proof without fan-out into unrelated E2E suites.

The 2026-07-23 PR 574 review repair closes eight local runtime defects covering
acknowledgement semantics, screen query ordering, whole-job analysis leases,
bounded deletion publication reports, retryable corrupt outbox projection,
malformed lease recovery, expired-queue claim refusal, and tombstone directory
sync. Focused Rust tests and the architecture gate are recorded in the proof
manifest. These repairs do not restore any checklist item below: the workpack
remains open until the complete accepted proof pack satisfies the target state.

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

## WP22 evidence (2026-07-18)

- Real Windows P3 capture run `8HIf9RVzwxgAAAAAAAAAAA` wrote encrypted queue
  custody and confirms the temporary raw PNG existed before encryption and was
  absent afterward. Artifact root:
  `output/screen-plan-proof/22-deletion-and-retention-proof/runtime-capture/`.
- Focused Rust evidence covers encrypted queue removal, expiry/restart-safe
  sweeps, durable deletion-event/read-model projection, delete-failed contract
  serialization, and redaction of raw image payloads.
- The packet is strictly local deletion/custody evidence. It does not claim
  product retention controls, raw-retention opt-in, remote upload, AI quality,
  policy authority, enforcement, or broad portal completion.

## Failure conditions

- Do not claim deletion readiness without before/after and store-record proof.
- Do not hide delete-failed states.
- Do not claim product retention controls from local deletion proof.
- Do not permit default long-term raw image retention.

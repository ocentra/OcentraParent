# Test and Proof Expectations

Status: engineering-spec complete / execution-open.

## Purpose

This file tracks the required execution flow. It does not store proof artifacts.

## Required flow

- [ ] Select one workpack and its external proof path.
- [ ] If the workpack is not WP00, confirm the Cloudflare prerequisite handoff
      or exact blocker first.
- [ ] Map the workpack to its exact required assertion IDs in
      `REQUIRED_TEST_ASSERTION_MATRIX.md`.
- [ ] For a docs-only spec-hardening pass, update route, proof, matrix, and
      status docs without claiming runtime completion.
- [ ] For a runtime pass, write or update the code and tests for that workpack,
      including at least one negative case.
- [ ] Record the exact validation command family or the exact blocker.
- [ ] Record at least one rollback or teardown note for the touched slice.
- [ ] Collect proof in the designated local artifact path.
- [ ] Record the proof pointer outside the plan folder.
- [ ] Sync route, index, queue, and route-gate docs.

## Proof storage

Proof artifacts live in the designated local artifact path for the workpack or
crate, not in this plan folder.

## Failure conditions

- Do not mark DONE or PR_READY until the code, tests, validation, and proof
  flow are complete for the selected runtime slice.
- Do not treat scaffold-only Cloudflare docs as payment runtime proof.
- Do not treat a complete assertion matrix as runtime proof.
- Do not store proof inventories inside the plan folder.

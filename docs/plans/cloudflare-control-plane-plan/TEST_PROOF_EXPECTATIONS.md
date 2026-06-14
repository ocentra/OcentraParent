# Test and Proof Expectations

Status: reset.

## Purpose

This file tracks the required execution flow for the shared Cloudflare module. It does not store proof artifacts.

## Required flow

- [ ] Select one workpack and its proof path under `docs/proof/cloudflare-control-plane-plan/`.
- [ ] Update the module docs, scaffold, or blockers for that workpack.
- [ ] Record real validation commands or exact missing-runtime blockers.
- [ ] Capture at least one negative case and one rollback or teardown note.
- [ ] Keep runtime claims separate from placeholder or scaffold existence.
- [ ] Sync route docs, workpack docs, and proof pointers.
- [ ] Keep payment blocked until WP12 handoff proof exists.

## Proof storage

Proof artifacts live under `docs/proof/cloudflare-control-plane-plan/`, not in this plan folder.

## Failure conditions

- Do not mark DONE or PR_READY until code, tests, validation, and proof flow are complete for the selected slice.
- Do not treat placeholder test files, placeholder scripts, or placeholder wrangler configs as runtime proof.

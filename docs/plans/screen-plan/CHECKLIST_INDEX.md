# Checklist Index

> **Live-code audit (2026-07-17):** [Project Progress Matrix](../../PLAN_CODE_STATUS_MATRIX.md) records current implementation, blockers, dependencies, and next unblocker. Rows remain proof-gated; this audit does not check unsupported work.

Status: reset.

This checklist tracks execution only. Proof artifacts are collected in the designated local artifact path for the workpack or crate, not in this plan folder.

- [ ] Read the plan and route docs.
- [ ] Write or update the code.
- [ ] Write or update the tests.
- [ ] Compile and validate the touched code.
- [ ] Run the tests.
- [ ] Run full crate or package validation.
- [ ] Collect proof artifacts in the designated local artifact path.
- [ ] Record the proof location outside the plan folder.
- [ ] Prepare PR-ready notes.

## WP22 Deletion And Retention Proof

- [x] Delete raw image after success: real Windows capture proof confirms the
  raw temporary PNG is removed after encrypted queue custody.
- [x] Delete raw image after expiry: focused sweeper test passes.
- [x] Record delete-failed state: protocol contract serialization test passes.
- [x] Record deletion proof reference: focused queue and sweeper tests pass.
- [x] Show deletion state in portal: exact-spec Playwright proof passes with
  service-backed desktop and mobile screenshots; the rendered route asserts
  that raw screenshots are absent.
- [x] Prove no default long-term raw image retention: protocol serialization
  excludes raw payloads and the real capture proof uses no raw retention.

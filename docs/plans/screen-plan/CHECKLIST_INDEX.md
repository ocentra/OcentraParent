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

- [ ] Delete raw image after success: prior proof is invalidated by production-path review; the
  raw temporary PNG is removed after encrypted queue custody.
- [ ] Delete raw image after expiry: disabled default sweeper and malformed-expiry retention require repair.
- [ ] Record delete-failed state: contract serialization is not durable transactional deletion proof.
- [ ] Record deletion proof reference: queue/sweeper proof must be production-safe and tamper-resistant.
- [ ] Show deletion state in portal: exact-spec Playwright proof passes with
  service-backed desktop and mobile screenshots; the rendered route asserts
  that raw screenshots are absent.
- [ ] Prove no default long-term raw image retention: protocol serialization
  excludes raw payloads and the real capture proof uses no raw retention.

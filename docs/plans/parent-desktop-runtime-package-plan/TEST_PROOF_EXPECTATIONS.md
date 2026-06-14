# Test and Proof Expectations

Status: reset.

## Purpose

This file tracks the required execution flow. It does not store proof artifacts.

## Required flow

- [ ] Select one workpack and its external proof path.
- [ ] Write or update the code for that workpack.
- [ ] Write or update tests, including at least one negative case.
- [ ] Compile and validate the touched boundary.
- [ ] Run the tests for the touched boundary.
- [ ] Run the full package or crate validation for the touched boundary.
- [ ] Collect proof in the designated local artifact path.
- [ ] Record the proof pointer outside the plan folder.
- [ ] Sync route, index, and route-gate docs.

## Proof storage

Proof artifacts live in the designated local artifact path for the workpack or crate, not in this plan folder.

## Validation anchors

- Docs-only slices: `npm run format:check` and `npm run lint:schema-boundaries`.
- Parent web: `npm run build --workspace @ocentra-parent/portal`, `npm run test --workspace @ocentra-parent/portal`, `npm run test:e2e --workspace @ocentra-parent/portal`.
- Parent mobile proof: `npm run test:parent-mobile-shell-runtime-proof`, `npm run test:parent-mobile-package-source-artifact-proof`, `npm run test:parent-mobile-service-bridge`, `npm run test:parent-mobile-controller-observer-handoff`.
- Parent desktop proof: `npm run test:parent-desktop-release-support-proof`, `npm run dev:desktop`.
- Full readiness: `npm run validate`.

## Failure conditions

- Do not mark DONE or PR_READY until the code, tests, validation, and proof flow are complete.
- Do not store proof inventories inside the plan folder.

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

## Validation anchors

- Docs-only slices: `npm run format:check` and `npm run lint:architecture -- --files docs/plans/child-agent-runtime-distribution-plan docs/PLAN_INDEX.md docs/FEATURE_ROUTE_INDEX.md`
- Windows package: `npm run release:package:windows`
- macOS package: `npm run release:package:macos`
- Linux package: `npm run release:package:linux`
- Android package: `npm run release:package:android`, `npm run test:child-android-protocol-package-lifecycle-proof`
- iOS package: `npm run release:package:ios`
- Full readiness: `npm run validate`

## Failure conditions

- Do not mark DONE or PR_READY until the code, tests, validation, and proof flow are complete.
- Do not store proof inventories inside the plan folder.

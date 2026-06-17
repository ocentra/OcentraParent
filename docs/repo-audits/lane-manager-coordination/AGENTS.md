# Lane Manager Coordination Router

This folder turns the archived per-thread self-assessments into execution instructions for the main Codex lane manager.

## Read order

1. `INDEX.md`
2. `LANE_MANAGER_AUTOPILOT.md`
3. `EXECUTION_DAG.md`
4. `GLOBAL_FIRST_FIXES.md`
5. `thread-instructions/INDEX.md`
6. The instruction file for the thread being dispatched.

## Rules

- Do not treat `per-thread-self-assessments/*` as truth. They are inputs.
- Do not start broad per-plan implementation before global structural fixes are assigned.
- Do not let a thread claim done from generated proof paths, empty folders, scoped validation, or stale docs.
- Do not run multiple threads on the same broad frontage package at the same time unless the owner map says the paths are disjoint.
- Every dispatch must name: files allowed, files forbidden, expected proof command, and the dependency it unblocks.

## Output rule

Every thread must report back with:

- exact files changed;
- exact commands run;
- generated proof root paths if any;
- blocker status;
- next proposed slice;
- whether any claim is local-only, CI-covered, or host-limited.

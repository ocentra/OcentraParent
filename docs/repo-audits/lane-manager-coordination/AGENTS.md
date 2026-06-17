# Lane Manager Coordination Router

This folder turns archived `*-selfaudit.md` files into execution instructions for the main Codex lane manager.

## Read order

1. `INDEX.md`
2. `LANE_MANAGER_AUTOPILOT.md`
3. `DISPATCH_PHASES.md`
4. `EXECUTION_DAG.md`
5. `GLOBAL_FIRST_FIXES.md`
6. `../event-driven-proof-architecture/AGENTS.md`
7. `../event-driven-proof-architecture/thread-instructions/INDEX.md`
8. `thread-instructions/INDEX.md`
9. The lane-manager instruction file and the event-architecture instruction file for the dispatched thread

## Rules

- Treat self-assessments as evidence, not truth.
- Start with global structural fixes before broad per-plan work.
- Use `DISPATCH_PHASES.md` for execution order.
- Use `thread-instructions/*.md` for exact per-thread first slice.
- Use `../event-driven-proof-architecture/thread-instructions/*.md` for boundary, logging, proof-chain, and test-layer requirements.
- Do not run two lanes on the same broad frontage package without path locks.
- Every dispatch must name allowed files, forbidden files, proof commands, event/log chain, and the dependency it unblocks.

## Required report-back

Each dispatched thread must return:

- exact files changed;
- exact commands run;
- generated proof root paths if any;
- emitted/consumed commands, events, requests, or read models;
- log/proof correlation strategy;
- blocker status;
- next proposed slice;
- whether each claim is local-only, CI-covered, or host-limited.

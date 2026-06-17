# Lane Manager Coordination Router

This folder turns archived `*-selfaudit.md` files into execution instructions for the main Codex lane manager.

## Read order

1. `INDEX.md`
2. `LANE_MANAGER_AUTOPILOT.md`
3. `VALIDATION_BUDGET_LADDER.md`
4. `DISPATCH_PHASES.md`
5. `EXECUTION_DAG.md`
6. `GLOBAL_FIRST_FIXES.md`
7. `CODEX_DRY_RUN_RISK_AUDIT.md`
8. `../event-driven-proof-architecture/AGENTS.md`
9. `../event-driven-proof-architecture/thread-instructions/INDEX.md`
10. `thread-instructions/INDEX.md`
11. The thread instruction files for the dispatched plan.

## Rules

- Treat self-assessments as evidence, not truth.
- Start with global structural fixes before broad per-plan work.
- Use `VALIDATION_BUDGET_LADDER.md` before validation.
- Use `DISPATCH_PHASES.md` for execution order.
- Use `thread-instructions/*.md` for exact first slice.
- Use event architecture thread instructions for boundary, logging, proof-chain, and test-layer rules.
- Do not run two lanes on the same broad frontage package without path locks.
- Each dispatch must name allowed files, forbidden files, validation level, event/log chain, and dependency unblocked.

## Required report-back

Each thread returns exact files changed, validation level, proof roots if any, emitted/consumed boundaries, log/proof correlation strategy, blockers, next slice, and local/CI/host-limited status.

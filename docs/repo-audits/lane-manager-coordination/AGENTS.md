# Lane Manager Coordination Router

This folder turns archived `*-selfaudit.md` files into execution instructions for the main Codex lane manager.

## Read order

1. `INDEX.md`
2. `LANE_MANAGER_AUTOPILOT.md`
3. `READ_SCOPE_BUDGET.md`
4. `VALIDATION_BUDGET_LADDER.md`
5. `DISPATCH_PACKET_TEMPLATE.md`
6. `DISPATCH_PHASES.md`
7. `EXECUTION_DAG.md`
8. `GLOBAL_FIRST_FIXES.md`
9. `CODEX_DRY_RUN_RISK_AUDIT.md`
10. `../event-driven-proof-architecture/AGENTS.md`
11. `../event-driven-proof-architecture/thread-instructions/INDEX.md`
12. `thread-instructions/INDEX.md`
13. The thread instruction files for the dispatched plan.

## Rules

- Treat self-assessments as evidence, not truth.
- Start with global structural fixes before broad per-plan work.
- Use `READ_SCOPE_BUDGET.md` before opening more files.
- Use `VALIDATION_BUDGET_LADDER.md` before validation.
- Use `DISPATCH_PACKET_TEMPLATE.md` for every assignment.
- Use `DISPATCH_PHASES.md` for execution order.
- Use `thread-instructions/*.md` for exact first slice.
- Use event architecture thread instructions for boundary, logging, proof-chain, and test-layer rules.
- Do not run two lanes on the same broad frontage package without path locks.
- Each dispatch must name allowed files, forbidden files, read scope, validation level, event/log chain, and dependency unblocked.

## Required report-back

Each thread returns exact files changed, exact files read, validation level, proof roots if any, emitted/consumed boundaries, log/proof correlation strategy, blockers, next slice, and local/CI/host-limited status.

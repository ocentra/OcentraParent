# Lane Manager Instruction

## Read first

- `../LANE_MANAGER_AUTOPILOT.md`
- `../EXECUTION_DAG.md`
- `../GLOBAL_FIRST_FIXES.md`
- `../../per-thread-self-assessments/codex-a-lane-manager-selfaudit.md`

## Immediate assignment

1. Clean archive ambiguity: ensure only canonical `*-selfaudit.md` files are treated as active inputs.
2. Dispatch repo-audit WP01-WP05 before broad plan work.
3. Lock a path ownership table before assigning any shared frontage package work.
4. Dispatch plan threads by `EXECUTION_DAG.md` tier.

## Do not

- Do not let any lane claim done from its self-assessment.
- Do not let more than one lane edit `packages/parent-domain`, `packages/agent-protocol-domain`, `crates/agent-protocol`, or `crates/agent-core` without disjoint path locks.
- Do not merge scoped validation language into repo-wide green wording.

## Required manager output

Maintain a coordinator status table with thread, assigned slice, changed files, commands, proof outputs, verdict, and next dependency.

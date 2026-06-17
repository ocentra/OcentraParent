# Lane Manager Coordination Index

Purpose: coordinate structural cleanup and per-plan follow-up after all canonical `*-selfaudit.md` files landed.

## Inputs

| Input | Use |
| --- | --- |
| `../2026-06-17-structural-truth-audit.md` | Baseline structural audit. |
| `../per-thread-self-assessments/INDEX.md` | Canonical self-assessment file map. |
| `../per-thread-self-assessments/codex-a-lane-manager-selfaudit.md` | Lane-manager meta-assessment. |
| `../per-thread-self-assessments/*-selfaudit.md` | Per-thread self-reports; evidence, not truth. |

## Coordination docs

| Doc | Purpose |
| --- | --- |
| [AGENTS.md](AGENTS.md) | Local route and stop rules. |
| [LANE_MANAGER_AUTOPILOT.md](LANE_MANAGER_AUTOPILOT.md) | Main lane-manager operating instructions. |
| [COORDINATOR_VERDICT_MATRIX.md](COORDINATOR_VERDICT_MATRIX.md) | One-table verdict, first slice, blocker, and dependency tier for every thread. |
| [EXECUTION_DAG.md](EXECUTION_DAG.md) | Dependency order and parallelization gates. |
| [GLOBAL_FIRST_FIXES.md](GLOBAL_FIRST_FIXES.md) | Repo-wide fixes that must start before plan expansion. |
| [thread-instructions/INDEX.md](thread-instructions/INDEX.md) | Per-thread dispatch index. |

## Current phase

`coordination-ready`: assessments are landed; coordinator instructions exist; lane manager should begin with global structural work, then dispatch plan threads by dependency tier.

## Non-negotiable first rule

Before broad per-plan implementation, complete or assign:

1. archive hygiene;
2. test topology inventory;
3. CI/package coverage matrix;
4. architecture policy reconciliation;
5. ownership/orphan/legacy-code map;
6. DRY/common-core candidate map.

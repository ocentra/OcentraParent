# Lane Manager Coordination Index

Purpose: coordinate structural cleanup and per-plan follow-up after all canonical `*-selfaudit.md` files landed.

## Inputs

| Input | Use |
| --- | --- |
| `../2026-06-17-structural-truth-audit.md` | Baseline structural audit. |
| `../WORKPACK_INDEX.md` | Structural audit workpack route. |
| `../per-thread-self-assessments/INDEX.md` | Canonical self-assessment file map. |
| `../per-thread-self-assessments/codex-a-lane-manager-selfaudit.md` | Lane-manager meta-assessment. |
| `../per-thread-self-assessments/*-selfaudit.md` | Per-thread self-reports; evidence, not truth. |

## Coordination docs

| Doc | Purpose |
| --- | --- |
| [AGENTS.md](AGENTS.md) | Local route and stop rules. |
| [LANE_MANAGER_AUTOPILOT.md](LANE_MANAGER_AUTOPILOT.md) | Main lane-manager operating instructions. |
| [READ_SCOPE_BUDGET.md](READ_SCOPE_BUDGET.md) | Read limits so workers do not scan the whole repo. |
| [VALIDATION_BUDGET_LADDER.md](VALIDATION_BUDGET_LADDER.md) | Validation levels so workers do not run broad gates by habit. |
| [DISPATCH_PACKET_TEMPLATE.md](DISPATCH_PACKET_TEMPLATE.md) | Required packet shape for every assignment. |
| [CODEX_DRY_RUN_RISK_AUDIT.md](CODEX_DRY_RUN_RISK_AUDIT.md) | Mental dry-run failure modes and required guardrails. |
| [DISPATCH_PHASES.md](DISPATCH_PHASES.md) | Step-by-step dispatch phases and path locks. |
| [COORDINATOR_VERDICT_MATRIX.md](COORDINATOR_VERDICT_MATRIX.md) | Verdict, first slice, blocker, and tier for every thread. |
| [EXECUTION_DAG.md](EXECUTION_DAG.md) | Dependency order and parallelization gates. |
| [GLOBAL_FIRST_FIXES.md](GLOBAL_FIRST_FIXES.md) | Repo-wide fixes that must start before plan expansion. |
| [thread-instructions/INDEX.md](thread-instructions/INDEX.md) | Per-thread dispatch index. |

## Current phase

`coordination-ready`: assessments are landed; coordinator instructions exist; lane manager should begin with global structural work, then dispatch plan threads by dependency tier.

## First rule

Before broad per-plan implementation, complete or assign:

1. archive hygiene;
2. test topology inventory;
3. CI/package coverage matrix;
4. orphan/legacy/pre-eventing surface inventory;
5. architecture policy reconciliation;
6. ownership drift map;
7. DRY/common-core candidate map.

Every assignment must include a read-scope level, validation level, exact allowed paths, exact forbidden paths, and stop conditions.

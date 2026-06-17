# Repo Audits Index

This folder holds repo-level structural audit routes, event-driven proof architecture rules, and lane-manager coordination docs.

## Start here for structural cleanup

1. [AGENTS.md](AGENTS.md)
2. [2026-06-17 Structural Truth Audit](2026-06-17-structural-truth-audit.md)
3. [NEXT_ACTIONS.md](NEXT_ACTIONS.md)
4. [WORKPACK_INDEX.md](WORKPACK_INDEX.md)
5. One selected workpack under `workpacks/`

## Start here for event-driven proof architecture

1. [event-driven-proof-architecture/AGENTS.md](event-driven-proof-architecture/AGENTS.md)
2. [event-driven-proof-architecture/INDEX.md](event-driven-proof-architecture/INDEX.md)
3. [event-driven-proof-architecture/EVENT_BOUNDARY_STANDARD.md](event-driven-proof-architecture/EVENT_BOUNDARY_STANDARD.md)
4. [event-driven-proof-architecture/LOGGED_PROOF_CHAIN_STANDARD.md](event-driven-proof-architecture/LOGGED_PROOF_CHAIN_STANDARD.md)
5. [event-driven-proof-architecture/thread-instructions/INDEX.md](event-driven-proof-architecture/thread-instructions/INDEX.md)

## Start here for lane-manager coordination

1. [lane-manager-coordination/AGENTS.md](lane-manager-coordination/AGENTS.md)
2. [lane-manager-coordination/INDEX.md](lane-manager-coordination/INDEX.md)
3. [lane-manager-coordination/DISPATCH_PHASES.md](lane-manager-coordination/DISPATCH_PHASES.md)
4. [lane-manager-coordination/thread-instructions/INDEX.md](lane-manager-coordination/thread-instructions/INDEX.md)

## Active docs

| Doc | Purpose |
| --- | --- |
| [AGENTS.md](AGENTS.md) | Local router for structural cleanup work. |
| [2026-06-17 Structural Truth Audit](2026-06-17-structural-truth-audit.md) | Baseline findings for tests, CI, architecture, ownership, and DRY risks. |
| [NEXT_ACTIONS.md](NEXT_ACTIONS.md) | Ordered cleanup queue. |
| [WORKPACK_INDEX.md](WORKPACK_INDEX.md) | Focused workpack selector. |
| [event-driven-proof-architecture/](event-driven-proof-architecture/INDEX.md) | Event-driven boundary, logging/proof-chain, dev UI, and per-thread architecture rules. |
| [lane-manager-coordination/](lane-manager-coordination/INDEX.md) | Dispatch plan for lane manager after self-assessments are archived. |

## Workpacks

| Workpack | Purpose |
| --- | --- |
| [WP01 Test Topology Inventory](workpacks/01-test-topology-inventory.md) | Real tests, empty scaffolds, inline tests, move candidates. |
| [WP02 CI Package Coverage Matrix](workpacks/02-ci-package-coverage-matrix.md) | Crate/package/app command and CI coverage. |
| [WP03 Architecture Policy Reconciliation](workpacks/03-architecture-policy-reconciliation.md) | Re-export/barrel policy truth and cleanup scope. |
| [WP04 Ownership Drift Map](workpacks/04-ownership-drift-map.md) | Misplaced code/tests and broad frontage risks. |
| [WP05 DRY Common-Core Candidate Map](workpacks/05-dry-common-core-candidate-map.md) | Repeated logic and safe extraction candidates. |
| [WP06 Plan Thread Review Gate](workpacks/06-plan-thread-review-gate.md) | Review pasted plan reports against structural truth. |
| [WP07 Orphaned Legacy Surface Inventory](workpacks/07-orphaned-legacy-surface-inventory.md) | Old, weakly-owned, pre-eventing, transitional, and stale proof-wrapper surfaces. |

## Review rule

Do not accept a plan-thread report as complete until source owners, executable tests, proof generators, run evidence, architecture-gate scope, ownership, DRY risks, event-driven boundary, logging/proof-chain evidence, and dependency blockers are checked.

# Repo Audit Next Actions

Status: active structural baseline route.

## Highest-priority order

| Order | Slice | Why first | Exit condition |
| ---: | --- | --- | --- |
| 1 | Test topology inventory | Empty scaffold folders and source-adjacent tests can fake coverage. | Inventory lists real tests, inline tests, empty folders, and missing public-boundary tests per crate/package/app. |
| 2 | CI/package coverage matrix | Plan reports can overclaim CI if jobs do not cover their crate/package. | Matrix maps every crate/package/app to local commands and CI jobs, with gaps explicit. |
| 3 | Architecture policy reconciliation | No-reexport policy conflicts with current crate/package roots. | Decision recorded: cleanup now, staged cleanup, or explicit exceptions; no report may claim repo-wide clean without matching command evidence. |
| 4 | Ownership drift map | Broad packages can hide misplaced implementation. | Map names files/surfaces that are owner, adapter/frontage, or move candidates. |
| 5 | DRY/common-core candidates | Repeated event-chain/runtime decision code can diverge. | Candidate list names duplicated patterns, required tests before extraction, and preferred owner. |
| 6 | Plan-thread review gate | Per-plan reports must be judged against structural truth. | Each pasted report gets a structured verdict using the audit template. |

## Do not start yet

- Broad per-plan implementation cleanup.
- Common-core extraction before tests protect current behavior.
- Repo-wide re-export rewrite without a policy decision.
- Moving source between packages without an ownership map.

## Default next assignment for Codex

Start with `WORKPACK_INDEX.md` row `WP01 Test Topology Inventory`.

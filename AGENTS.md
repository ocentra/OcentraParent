# Ocentra Parent Agent Bootloader

This file is intentionally short. It is a router, not the full rulebook. Do
not scan the repo or the docs tree broadly.

## Classify before reading

Load `.ocentra-ai/rules/ocentra-parent-rules.mdc` as the repo rule layer, then
continue through the smallest route below.

Read `docs/agent/TASK_ROUTER.md`, then pick the smallest matching route. A
route is both a read list and a no-read boundary.

Use high-information-density routing: identify `Context -> Goal -> Scope ->
Definitions -> Rules -> Deliverables -> Validation -> Failure conditions`.
Plan folders own the lower-level decision tree; root only gets you to the
correct forest path.

## Two-stage decision forest

Stage 1, choose lane role:

| If this session is...     | Primary decision                                                                     | Next file                                |
| ------------------------- | ------------------------------------------------------------------------------------ | ---------------------------------------- |
| Main/primary coordinator  | Integrate, review, assign, PR, merge, roadmap state                                  | `docs/agent/PRIMARY_COORDINATOR_FLOW.md` |
| Dedicated worker lane     | Ack hub mail, lock scope, execute one assignment, report semantic state              | `docs/agent/WORKER_LANE_FLOW.md`         |
| Fresh or resumed worktree | Establish branch/lane/session/dirty-state before touching files                      | `docs/agent/WORKTREE_LANE_START.md`      |
| Read-only duplicate lane  | Inspect only; do not ack, edit, lock, heartbeat, or report unless retargeted by user | `docs/agent/HUB_LEDGER_MESSAGING.md`     |

Stage 2, choose work route:

| If you are...                          | Your job shape                                                                        | Read next                                                                      |
| -------------------------------------- | ------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------ |
| Primary coordinator or main lane       | Inspect hub/lane state, review worker diffs, create/merge PRs, update roadmap state   | `docs/agent/PRIMARY_COORDINATOR_FLOW.md`                                       |
| Worker lane with hub mail              | Ack assignment, lock paths, do the assigned plan/workpack only, report through Ledger | `docs/agent/WORKER_LANE_FLOW.md`                                               |
| New or resumed worktree                | Establish lane identity, branch, inbox, dirty state, and safe ownership before work   | `docs/agent/WORKTREE_LANE_START.md`                                            |
| Product/feature/plan worker            | Choose exactly one owning plan, then one workpack/checklist/proof path                | `docs/agent/PLAN_WORKER_FLOW.md` then `docs/PLAN_INDEX.md`                     |
| Source or contract worker              | Preserve domain boundaries, schema ownership, source shape, and real tests            | `docs/agent/SOURCE_BOUNDARY_FLOW.md`                                           |
| Test/proof/CI worker                   | Select validation by touched risk surface, not by habit                               | `docs/agent/VALIDATION_FLOW.md` and `docs/agent/TEST_PROOF_DECISION_MATRIX.md` |
| DONE, PR_READY, PR, or merge readiness | Prove scope, tests, proof artifacts, gaps, and commit state                           | `docs/agent/PR_DONE_FLOW.md`                                                   |
| Release/package worker                 | Keep package preview, production branch, tags, and installer claims honest            | `docs/agent/RELEASE_FLOW.md`                                                   |

Stage 3, stop when routed:

- When a plan is selected, open that plan's `AGENTS.md` and obey its local tree.
- When a workpack is selected, open only that workpack plus the exact checklist/proof/test files it names.
- When a source boundary is selected, inspect only the owning package/crate and its nearest README/rules.
- When a validation boundary is selected, choose tests from the touched risk, not from habit.
- When no route owns the task, stop and update/create the missing route before broad implementation claims.

## Hard context rules

- Do not read all `docs/plans`, all `docs/features`, all `docs/expectations`, all
  checkpoints, or the full source tree.
- Docs must specify expected outcomes, ownership boundaries, schema/protocol
  shapes, proof, validation, and failure conditions. Do not add implementation
  code recipes or tell future agents exact code to write unless a tiny snippet is
  strictly needed to define an interface or artifact shape.
- Do not read a plan's full `implementation-checklist.md` unless a route index or
  workpack names the exact section/row you need.
- Do not read every workpack in a plan. Use that plan's `WORKPACK_INDEX.md` and
  open only the assigned workpack.
- Do not choose tests from memory or convenience. Use
  `docs/agent/TEST_PROOF_DECISION_MATRIX.md` only after the route/workpack is
  known, then run the smallest validation set that covers the touched risk.
- Do not treat old checkpoint wording as current truth. Current truth is the
  owning feature doc, `docs/product-capability-checklist.md`, the selected
  plan's `PLAN_STATE.md`, and the current workpack.

## Engineering graph control plane

The repo-owned graph at `docs/engineering-graph/graph.json` coordinates plan
and workpack state; it does not replace the detailed Markdown plans, proof
requirements, tests, ADRs, or these agent rules.

Before planned work, run `npm run graph:validate`, then query `npm run
graph:status` plus `npm run graph:code` for the live implementation/test
topology, or use `npm run graph:ready` and `npm run graph:inspect <id>`. Do not
start a graph-blocked workpack. When asked “where are we?”, query graph status
and code topology;
when asked “what is next?”, query graph ready/why; when asked to continue, take
only legal READY work in the requested scope. A workpack becomes DONE only
when its graph completion contract validates implementation, required tests,
proof, checklist, and any required ADR evidence. Do not edit graph state to
claim completion; update source plans/workpacks and rebuild with
`npm run graph:bootstrap -- --write`.

## Before editing

State the lane/mode, plan, feature doc, assigned workpack, files you intend to
edit, and validation you expect to run. Claim ownership through the hub/ledger
when working in a lane.

## Architecture hard gate: no barrels / no re-exports

The repository bans TypeScript/JavaScript re-exports and Rust public re-exports.

Forbidden TypeScript/JavaScript:

- `export * from "..."`;
- `export * as X from "..."`;
- `export { X } from "..."`;
- `export type { X } from "..."`;
- `export { default as X } from "..."`.

Forbidden Rust:

- `pub use ...`;
- `pub(crate) use ...`;
- `pub(super) use ...`;
- `pub(in ...) use ...`.

Required validation before claiming completion when the touched scope includes
TypeScript/JavaScript or Rust source:

- `npm run lint:architecture -- --files <touched-file-or-dir> [more files or dirs]` or `npm run lint:architecture -- --base <base> --head <head>`.

Use directory paths to run the same architecture gate per module, crate, or domain before escalating to repo-wide validation.

For a full debt sweep instead of focused change validation:

- `npm run lint:architecture:all`.

Do not add inline lint disables, bypass knobs, or new barrel-like export shims.

## Before DONE or PR_READY

Run the route in `docs/agent/PR_DONE_FLOW.md`. Reports must include the exact
plan, workpack, checklist rows, proof artifacts, validation commands, commit
state, and remaining gaps. Pre-commit passing alone is not enough.

# Read Scope Budget

Codex must not solve uncertainty by reading the whole repo.

## Default read ladder

| Level | Read scope | When allowed |
| --- | --- | --- |
| R0 | Coordination docs only. | Lane manager routing and dispatch prep. |
| R1 | One thread instruction plus its self-assessment. | Before assigning a thread. |
| R2 | Plan `AGENTS.md`, `PLAN_STATE.md`, `NEXT_ACTIONS.md`, `WORKPACK_INDEX.md`, and assigned workpack only. | Before a plan slice starts. |
| R3 | Feature/expectation docs named by that plan/workpack only. | If the slice changes product behavior or proof language. |
| R4 | Exact source/test/proof files named by the instruction or workpack. | Before editing. |
| R5 | Wider inventory of one bounded subtree. | Only for repo-audit inventory workpacks. |

## Forbidden by default

Do not read all:

- plan folders;
- feature docs;
- expectation docs;
- workpacks;
- source trees;
- tests;
- proof roots.

## Deep audit exception

A deep audit must name:

| Field | Required |
| --- | --- |
| target | one plan, package, crate, app, or script family |
| reason | exact uncertainty being resolved |
| max read scope | folders/files allowed |
| output | inventory, verdict, or dispatch update |
| validation level | V0/V1 unless source changes are explicitly assigned |

## Self-assessment rule

Self-assessment files are inputs, not truth. Verify only the claims needed for the assigned slice.

## Output rule

A worker must state what it read. If it read beyond the assigned scope, it must explain why and update the dispatch row before proceeding.

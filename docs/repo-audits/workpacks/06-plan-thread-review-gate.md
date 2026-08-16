# WP06 Plan Thread Review Gate

## Objective

Apply the structural truth audit to each pasted per-plan thread self-assessment.

## Scope

For each report, compare the thread's claims against:

- actual source owners;
- real executable tests;
- empty scaffold folders;
- inline source tests;
- proof generator commands;
- local or CI run evidence;
- architecture gate scope;
- ownership and DRY risks;
- upstream/downstream blockers.

## Review table

| Plan | Claimed status | Actual status | Source owners | Test truth | Proof truth | Gate truth | DRY/owner risk | Verdict | Next slice |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |

## Verdict rules

| Verdict | Use when |
| --- | --- |
| `done` | Code, tests, tracked proof command, run evidence, docs, and dependencies agree. |
| `partial` | Real work exists, but closure still needs tests, proof, owner cleanup, or dependency closure. |
| `false-green` | Report/checklist/docs claim closure but code/tests/proof/gates do not support it. |
| `missing` | Claimed surface has no meaningful implementation or executable proof. |
| `blocked` | Work cannot proceed honestly before an upstream decision/proof lands. |

## Required warning flags

- `empty-test-scaffold`
- `inline-test-move-candidate`
- `proof-generator-missing`
- `proof-run-not-shown`
- `ci-not-covering-surface`
- `architecture-scope-overclaimed`
- `owner-drift`
- `dry-duplication`
- `downstream-before-upstream`
- `host-limited-manual-required`

## Acceptance

- Every pasted report receives a structured verdict.
- No plan is marked done from self-assessment language alone.
- Next slices are ordered so structural blockers close before plan implementation expands.

## Failure conditions

- Reviewing reports in isolation without the structural baseline.
- Accepting old checkmarks, stale `PLAN_STATE.md`, or missing local proof as closure.
- Letting downstream plans widen before account/trust/custody/protocol gates close.

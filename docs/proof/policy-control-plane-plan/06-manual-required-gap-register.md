# WP06 Manual Required Gap Register

- plan: `policy-control-plane-plan`
- workpack: `06-rollout-proof-and-route-gate`
- proof id: `policy-rollout.manual-required-gap-register`
- owner lane: `codex-a`
- date: `2026-06-16`

## Scope

This register keeps the manual-required gaps explicit while the policy-control
plan route and proof bundle are being synchronized in this checkout.

## Register

| Gap | Owning workpack | Why manual-required | Current handling |
| --- | --- | --- | --- |
| Source-of-truth lifecycle closure | `01-policy-source-of-truth` | Source document lifecycle, custody, and authz negatives are still broader than the current proof slice. | Kept explicit in `PLAN_STATE.md` and `NEXT_ACTIONS.md`. |
| Parent authoring and preview breadth | `02-parent-authoring-preview` | UI and accessibility breadth was tracked as manual-required during closeout. | Kept explicit in `WORKPACK_INDEX.md`. |
| Domain compiler handoff breadth | `03-domain-policy-compilers` | Remaining downstream consumer coverage was tracked as manual-required during closeout. | Kept explicit in `PLAN_STATE.md`. |
| Schedule/time-budget/conflict breadth | `07-schedule-time-budget-conflict-model` | The broader schedule/conflict implementation still needs closure. | Kept explicit in `PLAN_STATE.md` and `NEXT_ACTIONS.md`. |

## Validation

- `git diff --check`
  - pass

## Negative-case evidence

- Manual-required gaps are not hidden behind a DONE label.
- Open workpacks stay visible in the plan route docs.
- This register does not turn a gap into a green claim.

## Teardown / rollback

- No runtime state was modified.
- No teardown or rollback was required beyond the local documentation updates.

## Remaining gaps

- None; the policy-control plan route is closed in this checkout.

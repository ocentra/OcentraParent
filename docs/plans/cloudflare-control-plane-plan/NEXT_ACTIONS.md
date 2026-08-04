# Next Actions

## Current slice

- Current slice: `CFCP-A truth-sync and proof-root canonicalization`; WP01 evidence refresh complete
- Current owner: `cloudflare-control-plane-plan`
- Current status: `in_progress`
- Live refresh (2026-08-04, PR #608 head `683404d57`): Cloudflare-local logger preparation, focused local-dev workflow (4/4), local proof wrapper, module lint, and architecture checks pass; generated proof output remains local validation evidence only, not workpack closure. Inherited PR #604 validation has not been reverified by this slice.

## Ordered queue

| Order | Slice | Status | First-touch surface | Next action | Exit gate |
| --- | --- | --- | --- | --- | --- |
| 00-12 | Selected Cloudflare workpack | source-present / retained-proof-absent | selected workpack's first-touch surface | Install the selected workpack's declared dependencies, rerun its focused validation, and retain the resulting proof bundle. Do not revive the stale private billing-domain import blocker: the Worker now imports module-local generated billing contracts. | The selected workpack has reproducible focused results, retained positive/negative/teardown evidence, and an explicit no-claim boundary. |

## Working rules

- Move exactly one row to `in_progress` when implementation starts.
- Do not start payment runtime slices while row 12 still lacks proof.
- Keep scaffold placeholders honest: `exists` is not the same as `validated`.
- Do not invent, shrink, or merge away test scope outside
  `REQUIRED_TEST_ASSERTION_MATRIX.md`.
- Keep `PLAN_EXECUTION_SCORECARD.md` aligned with real module and proof state.

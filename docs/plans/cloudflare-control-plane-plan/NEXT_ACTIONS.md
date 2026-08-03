# Next Actions

## Current slice

- Current slice: `CFCP-A truth-sync and proof-root canonicalization`; WP01 evidence refresh complete
- Current owner: `cloudflare-control-plane-plan`
- Current status: `in_progress`
- Live refresh (2026-08-03, PR #604 head `95cef56e0`): module lint, integration, contract, and architecture checks pass locally; retained proof-root output is still absent, so this is validation evidence only and not workpack closure.

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

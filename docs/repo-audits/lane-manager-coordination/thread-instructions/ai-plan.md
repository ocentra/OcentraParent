# ai-plan Instruction

## Verdict

`partial / false-green-risk`. `packages/ai-domain` is the real owner, but stale `parent-domain` AI wrappers, placeholder test categories, and missing proof roots block closure.

## Assign first

`ai-ownership-and-architecture-cleanup`:

- make `packages/ai-domain` the explicit owner;
- remove or replace stale `packages/parent-domain/src/local-ai*.ts`, `parent-assistant*.ts`, and household AI provider wrappers;
- update consumers to import from owner packages;
- run scoped architecture on touched files.

## Then

1. `ai-test-rebase-into-major-categories`: replace placeholder test folders with real contract/integration/e2e/property/security/load tests where applicable.
2. `local-ai-core-through-journal-and-read-model`: prove runtime, journal, read model, provider scheduler, and degraded/unavailable states.

## Coordinate with

- `screen-plan` and `screen-ai-pipeline-plan` for screen-derived AI claims.
- `portal-ux-household-surfaces-plan` for assistant/parent UI proof.
- `lan-plan` / `eventing-plan` for household AI mesh only after their own gates.

## Do not

- Do not count `parent-domain` wrapper files as AI ownership.
- Do not count empty AI test folders as coverage.
- Do not claim screen-derived AI closure until screen/screen-AI proof roots exist.

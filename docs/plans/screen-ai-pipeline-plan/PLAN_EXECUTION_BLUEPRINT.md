<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-ai-pipeline-plan`
> Doc: `Screen AI Pipeline Execution Blueprint`
> Kind: implementation sequence and handoff protocol.
> Read when: a worker needs exact execution order, DONE rules, or handoff sequencing.
> Stop rule: choose one workpack; do not implement multiple workpacks unless explicitly assigned.
> Proves: execution routing only.
> Does not prove: implementation completion or PR readiness.

<!-- /agent-capsule -->

# Screen AI Pipeline Execution Blueprint

## Current audited entry conditions

- All 10 workpacks are currently open.
- `implementation-checklist.md` currently has 134 unchecked rows and 0 checked rows.
- No retained `output/screen-ai-pipeline-proof/` proof root currently exists in this checkout.
- `docs/proof/screen-ai-pipeline-plan/PLAN_PROOF_MANIFEST.md` is currently missing.
- The scoped architecture gate is currently red on `packages/screen-domain/src/screen-evidence.ts`, `packages/portal-domain/src/contracts.ts`, and `packages/parent-domain/src/local-ai-runtime.ts`.

## Execution rule

Use this loop:

```text
AGENTS.md -> PLAN_STATE.md -> NEXT_ACTIONS.md -> WORKPACK_INDEX.md -> one workpack -> TEST_PROOF_EXPECTATIONS.md -> PROOF_INDEX.md
```

## Proof root

```text
output/screen-ai-pipeline-proof/
```

## Proof routing note

- Use the scenario directories named by the assigned workpack or checklist row, not the workpack filename as a folder name.
- Current docs still mix `proof-summary.json` scenario artifacts with the richer numbered bundle described in `pipeline-proof-matrix.md`; resolve that expectation in the assigned workpack before checking rows.

## Focused commands

```bash
npm run build --workspace @ocentra-parent/screen-domain
npm run test --workspace @ocentra-parent/screen-domain
npm run build --workspace @ocentra-parent/ai-domain
npm run test --workspace @ocentra-parent/ai-domain
cargo test -p ocentra-parent-agent-protocol screen_ai
cargo test -p ocentra-parent-agent-service screen_ai
npm run test --workspace @ocentra-parent/portal -- screen
npm run lint:architecture -- --files packages/screen-domain packages/ai-domain packages/evidence-domain crates/agent-protocol crates/agent-service apps/portal docs/plans/screen-ai-pipeline-plan
```

## DONE rule

One workpack is DONE only after focused commands or explicit blockers are recorded, retained proof artifacts exist under the named scenario directories, and the workpack/checklist rows are updated after the proof exists.

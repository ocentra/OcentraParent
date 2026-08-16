<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `App + Game Plan`
> Kind: short plan entry point.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.

<!-- /agent-capsule -->

# App + Game Plan

This is the short, token-efficient entry point for `app-game-plan`. The original full
README content is preserved at [README_FULL_ORIGINAL.md](README_FULL_ORIGINAL.md) and is not default context.

## Default agent path

1. Read [AGENTS.md](AGENTS.md).
2. Read [PLAN_STATE.md](PLAN_STATE.md).
3. Read [CODE_AUDIT.md](CODE_AUDIT.md) for the current 220-row code/test state.
4. Read [NEXT_ACTIONS.md](NEXT_ACTIONS.md) when starting/resuming.
5. Read [WORKPACK_INDEX.md](WORKPACK_INDEX.md).
6. Open only the assigned workpack.
7. Use [CHECKLIST_INDEX.md](CHECKLIST_INDEX.md) to locate exact checklist sections.
8. Use [PROOF_INDEX.md](PROOF_INDEX.md) only when proof artifacts are needed.

## Current scope

This folder is the shared native app and native game control plan. It exists because apps and games share the low-level evidence spine, but they do not share product meaning.

Current code-first baseline: 220/220 workpacks are mapped; 170 have no Phase
1 source/test-writing gap (151 code+test packets and 19 no-code packets), while
50 remain incomplete. This is not a Phase 2, proof, or release claim.

## Do not default-read

- `implementation-checklist.md` unless the route names exact rows/sections.
- `source-index.md` unless source ownership is unclear.
- all files under `workpacks/`.
- sibling plan folders.
- historical proof/checkpoint docs not named by the workpack.

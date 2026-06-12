<!-- agent-capsule -->

> Agent Capsule
> Doc: DONE, PR_READY, and Pull Request Flow
> Kind: agent flow documentation; read only when selected by root AGENTS or TASK_ROUTER.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# DONE, PR_READY, and Pull Request Flow

Use this before any `DONE`, `PR_READY`, PR creation, PR refresh, or merge-ready
claim.

## No tiny-slice completion claims

Do not claim PR-ready for a tiny unverified slice when the assignment expected a
larger plan/workpack result. The report must prove the assigned workpack or
explicitly state which checklist rows remain open.

## Required report fields

Every `DONE` or PR-ready report must include:

- lane/mode and branch;
- plan folder;
- assigned workpack path;
- checklist rows/sections changed;
- feature doc and product capability checklist row updated, or why not needed;
- files touched;
- proof artifacts and where they are stored;
- test/proof risk rows selected from `TEST_PROOF_DECISION_MATRIX.md`;
- exact validation commands and results;
- commit hash/state and whether the branch is pushed;
- known gaps, risks, skipped heavy checks, and follow-up work.

## Pull request body

The PR body must include detailed scope, touched packages/files, validation,
known gaps/risks, and the roadmap/plan slice completed. Workers may open PRs
when asked, but must not merge PRs or push to `main` unless the user explicitly
asks for that exact action.

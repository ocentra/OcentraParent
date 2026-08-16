<!-- agent-capsule -->

> Agent Capsule
> Doc: Primary Coordinator Flow
> Kind: agent flow documentation; read only when selected by root AGENTS or TASK_ROUTER.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

# Primary Coordinator Flow

Use this only when you are assigning, integrating, reviewing, creating PRs, or
merging lane work.

## Minimum read path

1. Root `AGENTS.md`.
2. `docs/agent/TASK_ROUTER.md`.
3. This file.
4. `docs/agent/HUB_LEDGER_MESSAGING.md`.
5. `docs/architecture/worktree-lanes.md` only when you need lane/wakeup detail.
6. `docs/product-roadmap.md` only when assigning/integrating roadmap work.

## Coordination pass

Before assigning or integrating:

```bash
npm run hub:status
npm run lanes:status
npm run ledger:workers
npm run ledger:tasks
npm run hub:lane-ledger:audit
```

Also check primary/worktree Git status, open PRs/checks, and GitHub Actions state
when relevant. Tell workers to pull/rebase latest `main` before starting.

## Reviewing worker DONE

Do not trust a `DONE` report by itself. Review the branch diff, changed docs,
validation output, proof artifacts, and plan/workpack/checklist updates. Only
create or refresh a PR after local validation is acceptable and the worker branch
is pushed.

## Merge discipline

Merge only after PR CI is green and the reviewed diff is acceptable. After
merge, pull latest `main`, update roadmap/lane/hub state, and tell active
workers to rebase or pull latest `main` before continuing.

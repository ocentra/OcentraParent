<!-- agent-capsule -->

> Agent Capsule
> Doc: Worker Lane Flow
> Kind: agent flow documentation; read only when selected by root AGENTS or TASK_ROUTER.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

# Worker Lane Flow

Use this when you are working as a lane worker (`codex-a`, `codex-b`, etc.) or
when the assignment arrives through hub/Ledger mail.

## Start/resume sequence

Run these from the checkout/worktree you are using before editing:

```bash
npm run lanes:status
npm run lanes:guard
npm run hub:status
npm run hub:guard
npm run hub:inbox
```

Acknowledge the latest relevant hub instruction, then report `STARTED`:

```bash
npm run hub:ack
npm run hub:report -- --summary "STARTED: <short task>" --details "plan=<plan>; workpack=<workpack>; branch=<branch>"
```

Before editing, claim ownership paths:

```bash
npm run hub:lock -- --paths "path/or/package,other/path" --reason "<short scope>"
```

Claims are file-level only: lock exact file paths, not folders or globs, and
keep each claim batch to 10 files or fewer per thread write pass. That cap is
per thread, not per lane. A lane can host many active threads, and each thread
should claim only the files it actively intends to write.

## Read only your route

1. Read `PLAN_WORKER_FLOW.md`.
2. Read `docs/PLAN_INDEX.md`.
3. Read the assigned plan's `AGENTS.md`.
4. Read that plan's `PLAN_STATE.md` and `WORKPACK_INDEX.md`.
5. Read only the assigned workpack and exact referenced checklist section.

Do not read sibling plans, every workpack, or full implementation checklists.

## Progress and finish reports

Use semantic hub reports only: `STARTED`, meaningful progress, `BLOCKED`,
`DONE`, or `PR_READY`. Use heartbeat for liveness instead of overwriting work
state:

```bash
npm run hub:heartbeat -- --state idle --note "waiting for instruction"
```

When done, verify, commit locally on the worker branch, push only when ready for
review, and report exact validation, commit state, touched files, known gaps,
and proof artifacts. Workers do not merge PRs or push to `main` unless the
user explicitly asks for that exact action.

When the report is one of `STARTED`, `BLOCKED`, `PR_READY`, or `DONE`, include
the structured metadata block from `HUB_LEDGER_MESSAGING.md` so the hub can
route the work automatically. Keep the values exact and file-level:

- lane
- threadId
- assignedBy
- plan
- workpack
- worktree
- branch
- scope
- startedAt or blocker / validation / commit as the state requires

Do not send folder claims or globs in the metadata block. Claims remain exact
file paths only, with 10 files or fewer per claim batch.

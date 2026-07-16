<!-- agent-capsule -->

> Agent Capsule
> Doc: Worktree and Lane Start/Resume Flow
> Kind: agent flow documentation; read only when selected by root AGENTS or TASK_ROUTER.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

# Worktree and Lane Start/Resume Flow

Use this when starting a new worktree, resuming an existing lane, changing PCs,
or recovering after a long/overflowed Codex chat.

## Shared state

Live lane, inbox, ownership, heartbeat, and task state belongs in Ocentra
Ledger, not in this product repo. Parent consumes Ocentra Enforcer coordination
through npm wrappers and MCP; the reusable coordination implementation and live
ledger state live outside the product repo.

If using more than one PC, configure a shared `LEDGER_ROOT` or Ledger sync
transport before coordination or lane work. Do not let two PCs actively edit
the same lane at the same time.

## Resume sequence

```bash
npm run lanes:status
npm run lanes:guard
npm run hub:status
npm run hub:guard
npm run hub:inbox
npm run ledger:workers
npm run ledger:tasks
```

If a duplicate live chat already owns the lane, treat your session as read-only
unless the user explicitly retargets that lane. A read-only duplicate may
inspect status and answer questions; it must not ack mail, edit files, claim
paths, heartbeat, or report work.

## Long chat recovery

If a chat gets too long, start a new chat in the same worktree. The Codex hook
should identify the lane, show acknowledged Ledger messages/latest reports, and
avoid repeating completed inbox setup. If hook trust review appears, review and
enable project hooks before relying on automatic Ledger context.

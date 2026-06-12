<!-- agent-capsule -->

> Agent Capsule
> Doc: Hub, Ledger, Mail, and Wakeup Messaging
> Kind: agent flow documentation; read only when selected by root AGENTS or TASK_ROUTER.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

# Hub, Ledger, Mail, and Wakeup Messaging

Use this when you need exact hub/Ledger coordination or lane wakeup protocol.

## Assignment source of truth

Ledger mail/report is the assignment and acknowledgement record. Automation
prompt text is only a wake bridge. Never use automation prompt text as the task
source of truth.

## Worker command flow

```bash
npm run hub:inbox
npm run hub:ack
npm run hub:report -- --summary "STARTED: <task>" --details "plan=<plan>; workpack=<workpack>"
npm run hub:lock -- --paths "<paths>" --reason "<scope>"
npm run hub:report -- --summary "progress: <summary>" --details "validation, blockers, touched files"
npm run hub:report -- --summary "DONE: <summary>" --details "validation, commit, touched files, gaps, proof"
```

## Wakeups

Prefer targeted one-shot Codex wakeups over always-on minute automation. Sender
writes real instruction in Ledger mail, creates/resumes the recipient wakeup,
and recipient pauses it after reading/acking mail. Stable lane wakeups must be
paused, not deleted. Disposable temporary wakeups may delete themselves only
when explicitly temporary.

Use the cheap detector before any automation bridge:

```bash
npm run hub:notify -- --lane <lane> --exit-code
```

If target thread id is unknown or automation tooling is unavailable, report that
limitation in Ledger instead of guessing.

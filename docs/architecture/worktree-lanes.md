# Worktree Lanes

Ocentra Parent uses milestone branches for finished product slices. A feature branch may be pushed for backup, but PR CI is the integration gate when the milestone is ready to merge to `main`.

The lane hub keeps local parallel work explicit. It stores machine-local lane state in:

```text
C:\Users\<you>\.codex\ocentra-parent-worktrees.json
```

This file is not committed because it records local paths, active tasks, owner/thread hints, and temporary branch ownership.

## Lanes

- `primary`: the user's main checkout. Do not repurpose without explicit direction.
- `codex-a`: reusable Codex worktree lane.
- `codex-b`: reusable Codex worktree lane.
- `codex-c`: reusable Codex worktree lane.

Do not create branches for every future roadmap milestone at once. Claim a lane when a milestone is ready to start, because branch bases drift and stacked work should be intentional.

## Commands

Show lane state and live Git status:

```powershell
npm run lanes:status
```

Guard the current checkout before coding or committing:

```powershell
npm run lanes:guard
```

The pre-commit hook runs this guard automatically. If a chat is in the wrong worktree, on the wrong branch, or using an unclaimed lane, the commit fails before validation.

Initialize the lane ledger if it does not exist:

```powershell
npm run lanes:init
```

Claim a lane without creating the worktree yet:

```powershell
npm run lanes:claim -- --lane codex-a --branch "V0.3 Windows Process And Window Activity Capture" --task "V0.3 Windows process/window capture" --owner "codex" --thread "thread-or-chat-label"
```

Claim a lane and create the worktree from `origin/main`:

```powershell
npm run lanes:claim -- --lane codex-a --branch "V0.3 Windows Process And Window Activity Capture" --task "V0.3 Windows process/window capture" --owner "codex" --thread "thread-or-chat-label" --create-worktree
```

Free a lane after merge, park, or handoff:

```powershell
npm run lanes:free -- --lane codex-a --next-action "Reusable after fresh status check."
```

## Branch Naming

Human milestone names are normalized into valid Git refs:

```text
V0.3 Windows Process And Window Activity Capture
-> codex/v0.3-windows-process-and-window-activity-capture
```

## Parallel Work Rule

Parallel lanes should be independent. Good parallel slices are documentation, workflow tooling, policy contracts, and platform research. Dependent product slices should wait for their base PR to merge or should be created as deliberate stacked branches with a clear base.

Before editing in a claimed lane:

1. Run `git status --short --branch` in that lane.
2. Confirm the lane ledger says the lane is yours.
3. Confirm the branch base is the intended branch, usually `origin/main`.
4. Run `npm run lanes:guard` from that worktree.
5. Run focused local validation while coding.
6. Run the full PR gate only when the branch is ready to integrate.

## Owner And Thread Fields

Every active lane should record:

- `owner`: the person or agent responsible for the lane.
- `thread`: a short chat/thread label so parallel Codex chats can identify their own lane.
- `task`: the product or workflow slice being implemented.
- `nextAction`: the next concrete step for anyone who resumes the lane.

When handing work to another chat, update the lane with `npm run lanes:claim -- --force ...` instead of relying on chat history.

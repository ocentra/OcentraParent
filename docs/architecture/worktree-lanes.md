# Worktree Lanes

Ocentra Parent uses milestone branches for finished product slices. A feature branch may be pushed for backup, but PR CI is the integration gate when the milestone is ready to merge to `main`.

The lane hub keeps local parallel work explicit. It stores machine-local lane state in:

```text
C:\Users\<you>\.codex\ocentra-parent-worktrees.json
```

This file is not committed because it records local paths, active tasks, owner/thread hints, and temporary branch ownership.

Cross-chat hub messages, lane reports, and file ownership locks are stored in:

```text
C:\Users\<you>\.codex\ocentra-parent-hub
```

That hub folder is also machine-local. It is the coordination layer between Codex chats opened in different worktree folders.

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

Show cross-chat hub state:

```powershell
npm run hub:status
```

Read the current lane inbox:

```powershell
npm run hub:inbox
```

Watch the current lane inbox for new hub instructions:

```powershell
npm run hub:watch
```

Use `npm run hub:watch -- --interval-ms 5000` to choose a polling interval. Add `--ack` only when the worker is intentionally treating displayed messages as read; otherwise acknowledge manually after reading with `npm run hub:ack`.

Watch worker reports from the primary hub checkout:

```powershell
npm run hub:watch -- --reports --interval-ms 5000
```

Codex lifecycle hooks are configured in `.codex/hooks.json` and execute `npm run --silent hub:hook`, which routes to `scripts/dev/codex-hub-hook.mjs`:

- `SessionStart` and `UserPromptSubmit` add current lane, inbox, lock, and report state to the agent context.
- `SessionStart` and `UserPromptSubmit` record Codex's current `session_id` as the lane's active session, while preserving the human `thread` label.
- `PostToolUse` reminds worker lanes to lock paths when edits create dirty files without hub ownership.
- `Stop` continues worker turns when unread hub messages still need acknowledgement or dirty worker changes need lock/report handling.

Hooks are not a background daemon and do not wake an idle chat on file changes. They make the next turn hub-aware without opening separate watcher consoles. If the primary or a worker chat gets too long, open a new chat in the same worktree. The startup hook will register the new Codex session, show the current lane, latest message, last acknowledged message, locks, and latest report, and explicitly tells the chat not to rerun already acknowledged hub messages. Review/trust project hooks in Codex settings if the app lists them as pending.

Send a hub message to a lane:

```powershell
npm run hub:message -- --lane codex-a --subject "V0.3 scope" --body "Stay inside process/window capture and report touched files."
```

Acknowledge the latest hub message in the current lane:

```powershell
npm run hub:ack
```

Lock files or package roots before editing:

```powershell
npm run hub:lock -- --paths "crates/agent-service,packages/activity-domain" --reason "V0.3 capture implementation"
```

Report progress from a worker lane:

```powershell
npm run hub:report -- --summary "Capture adapter mapped" --details "Touched crates/agent-service. Focused Rust tests pass."
```

Guard the current lane mailbox and file locks:

```powershell
npm run hub:guard
```

The pre-commit hook runs the hub guard automatically. It fails when the lane has an unread hub message or when changed files are outside the lane's hub lock. `primary` may coordinate without a lock, but worker lanes should always lock their intended paths before editing.

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
5. Run `npm run hub:inbox` and acknowledge current instructions with `npm run hub:ack`.
6. Leave `npm run hub:watch -- --interval-ms 5000` running when the primary hub should be able to send follow-up instructions without a manual prompt.
7. Claim file ownership with `npm run hub:lock`.
8. Run focused local validation while coding.
9. Report progress with `npm run hub:report`.
10. Run the full PR gate only when the branch is ready to integrate.

## Owner And Thread Fields

Every active lane should record:

- `owner`: the person or agent responsible for the lane.
- `thread`: a short chat/thread label so parallel Codex chats can identify their own lane.
- `activeSessionId`: the actual current Codex session id, updated automatically by hooks when a chat starts or submits a prompt in that worktree.
- `task`: the product or workflow slice being implemented.
- `nextAction`: the next concrete step for anyone who resumes the lane.

When handing work to another chat in the same lane, open the new chat in that worktree and let the hook register its `session_id`. The primary hub can send a lane message asking a worker to rotate to a fresh chat, but the user still opens the new Codex chat in that worktree; the hook handles identity and already-done message state after the new chat starts. When changing lane ownership, branch, or task scope, update the lane with `npm run lanes:claim -- --force ...` instead of relying on chat history.

## Hub Mailbox Files

Each lane gets:

- `inbox.md`: human-readable hub messages for that lane.
- `status.md`: latest lane report and current file locks.
- `ownership.json`: machine-readable ack, report, and lock state used by `npm run hub:guard`.

Do not hand-edit `ownership.json` unless the mailbox script is broken. Prefer the `npm run hub:*` commands so the Markdown and guard state stay in sync.

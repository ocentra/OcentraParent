# Worktree Lanes

Ocentra Parent uses milestone branches for finished product slices. A feature branch may be pushed for backup, but PR CI is the integration gate when the milestone is ready to merge to `main`.

The repo-owned lane ledger is the portable authority for lane state:

```text
.hub/lane-ledger.json
docs/hub/lane-ledger.md
```

These committed files record lane ids, branches, remote branches, PRs, status, validation, merge state, next action, and cleanup safety. No Codex lane work is complete unless these files are updated, or the report explicitly says why no ledger update was needed.

The lane hub keeps local parallel work explicit. Live operational state is not product code and must not be committed to `OcentraParent`.

The target live hub is OcentraHub, an external append-only event ledger that materializes lane inbox, status, ownership, heartbeat, and report views. See:

```text
docs/hub/ocentra-hub-event-ledger.md
```

During migration, workers may still read the legacy global root:

```text
C:\Users\<you>\.codex\ocentra-parent-hub
```

This legacy root is operational state, not product repo state. Do not copy it into `.hub/state` and do not commit it to `OcentraParent`. Actual worktree folders may still live outside the repo.

Cross-chat hub messages, lane reports, and file ownership locks are stored in:

```text
OcentraHub event log, materialized lane views, or the temporary legacy global root
```

That hub transport is the coordination layer between Codex chats opened in different worktree folders. The higher-level durable lane truth still belongs in `.hub/lane-ledger.json` and `docs/hub/lane-ledger.md`.

Audit local checkouts before cleanup or cross-PC handoff:

```powershell
npm run hub:lane-ledger:audit -- -SearchRoots "E:\","D:\","C:\Users\$env:USERNAME"
```

Sync live hub state before working from another PC:

```powershell
ocentra-hub sync --hub ocentra-parent
```

Until OcentraHub replaces the legacy root, manually ensure each PC has the current legacy hub state before relying on lane reports. Do not use product repo commits as the live mailbox transport.

```powershell
npm run hub:status
```

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

Worker minute heartbeats are standing mailbox checks, not disposable task reminders. A worker should not delete, pause, or replace its per-minute heartbeat just because there is no unread hub mail or active assignment. Routine liveness belongs in the heartbeat log, not in semantic hub reports:

```powershell
npm run hub:heartbeat -- --state alive --note "minute wake"
```

If the lane is idle or parked, use:

```powershell
npm run hub:heartbeat -- --state idle --note "waiting for instruction"
```

Do not overwrite `STARTED`, `BLOCKED`, or `DONE` reports with idle/waiting text. If a lane has an active assignment, the heartbeat should append liveness and the worker should continue useful assigned work, report real progress/`BLOCKED`/`DONE`, or stay quiet.

The primary coordinator can inspect worker liveness without losing report state:

```powershell
npm run hub:heartbeats
```

Heartbeat events are disposable liveness telemetry and may expire or be compacted during migration. They are not semantic lane state. During migration they are materialized in the configured legacy external hub root; after OcentraHub lands they are rebuilt from the external event log.

```text
<legacyHubRoot>/worker-heartbeats.ndjson
<legacyHubRoot>/lanes/<lane>/heartbeat.ndjson
```

Watch worker reports from the primary hub checkout:

```powershell
npm run hub:watch -- --reports --interval-ms 5000
```

Codex lifecycle hooks are configured in `.codex/hooks.json` and execute `npm run --silent hub:hook`, which routes to `scripts/dev/codex-hub-hook.mjs`:

- `SessionStart` and `UserPromptSubmit` add current lane, inbox, lock, report state, and the worker start/idle/reporting protocol to the agent context.
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

Before starting or resuming assigned work, report a short `STARTED` status so the primary coordinator can see that the instruction was accepted. When work is done, verify it, run the lint/tests requested by the hub mail, make a local commit on the worker branch, push that branch when ready for review, and report `DONE` with exact commands, commit state, touched packages/files, known gaps/risks, and detailed scope of what changed. If the user or primary asks the worker to create a PR, the worker may open the PR and include the same detailed scope in the PR body. Workers must not merge PRs or push directly to `main` unless the user explicitly asks for that exact action. Keep routine reports short unless the hub mail asks for detail; `DONE` and PR-ready handoffs are expected to include enough scope for review.

Do not use `hub:report` for per-minute "I am alive" chatter. Use `hub:heartbeat` for that local-only liveness stream so a `DONE` or `BLOCKED` report remains visible until a real work-state report replaces it.

Guard the current lane mailbox and file locks:

```powershell
npm run hub:guard
```

The pre-commit hook runs the hub guard automatically. It fails when the lane has an unread hub message or when changed files are outside the lane's hub lock. `primary` may coordinate without a lock, but worker lanes should always lock their intended paths before editing.

## Primary Coordinator Lifecycle

The primary coordinator is responsible for assignment, review, PR/CI watching, merge timing, and post-merge sync. Before assigning or integrating roadmap work, read:

- `AGENTS.md`
- `.ocentra-ai/rules/ocentra-parent-rules.mdc`
- `docs/architecture/worktree-lanes.md`
- `docs/architecture/primary-coordinator-reminder.md`
- `docs/product-roadmap.md`
- any feature-specific architecture or expectation doc named in the hub assignment

On every coordination pass:

1. Run `npm run hub:status`.
2. Run `npm run lanes:status`.
3. Check `git status --short --branch` in primary and relevant worker worktrees.
4. Check open PRs and CI/check state when branches are pushed.
5. Check latest worker reports before sending new instructions.

When assigning work, tell the worker to fetch/pull or rebase latest `main` first, acknowledge hub mail, report `STARTED`, lock intended paths, and keep routine reports short. The hub message should name the branch, task, relevant docs, validation expectation, that local commits and branch pushes are expected when the scope is ready for review, whether the worker should open a PR, and that `DONE` or PR-ready handoffs need detailed scope.

When a worker reports `DONE`, review the branch before creating or merging anything:

1. Inspect the diff against the intended base.
2. Confirm file locks and touched paths match the assignment.
3. Confirm validation commands and results are credible.
4. Confirm the worker provided detailed scope: what changed, touched packages/files, validation, known gaps/risks, and roadmap slice.
5. Ask the worker for fixes if the diff, tests, docs, or scope are not acceptable.
6. Create or update a PR only after local validation is acceptable and the branch is pushed.

After a PR is open, the primary coordinator watches CI. The PR body must clearly state the detailed scope, validation, known gaps/risks, and roadmap slice completed. If CI fails, route the failure back to the owning worker unless the fix is clearly an integration-only coordinator change. Merge only after CI is green and the reviewed diff is acceptable.

After merging, pull latest `main` in primary, update roadmap/lane/hub state, free or retarget the completed lane, and tell active workers to fetch/rebase latest `main` before continuing. The post-merge hub report must include detailed scope, validation, PR/merge state, known gaps/risks, and the next roadmap action. Do not assign new stacked work from a stale base unless that stacking is intentional and recorded in the hub message.

Merge conflicts should be resolved in the branch that owns the work. A worker resolves conflicts after fetching/rebasing latest `main` in its own worktree and reports the resolution plus validation. Primary resolves conflicts only when it owns the integration branch or the conflict is purely in coordinator-maintained files, and it must keep the worker informed.

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
6. Report `STARTED` before doing the assigned work.
7. Leave `npm run hub:watch -- --interval-ms 5000` running when the primary hub should be able to send follow-up instructions without a manual prompt.
8. Claim file ownership with `npm run hub:lock`.
9. Run focused local validation while coding.
10. Report progress with `npm run hub:report`.
11. When done, verify, run requested lint/tests, make a local commit on the worker branch, push the branch when ready for review, and report `DONE` with validation and commit state. Open a PR only when the user or primary asks for one.
12. Run the full PR gate only when the branch is ready to integrate.

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

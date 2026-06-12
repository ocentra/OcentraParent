<!-- agent-capsule -->

> Agent Capsule
> Doc: Primary Coordinator Reminder
> Kind: architecture/reference documentation; read only when selected by plan route, source router, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

# Primary Coordinator Reminder

This is the full reminder for the primary Ocentra Parent coordinator thread.
The minute heartbeat should stay short and read this file only when a worker is
stale, blocked, done, on the wrong branch, has unread mail after a wake, or when
five minutes or more have passed since the last full coordination pass.

## Role

The primary checkout coordinates and reviews. It does not implement feature
code unless the user explicitly redirects.

The primary coordinator owns:

- worker assignment and retargeting;
- Ledger and lane health checks;
- review of worker `DONE` reports;
- PR creation and CI watching when a branch is ready;
- merge timing after green CI;
- post-merge `main` sync, roadmap status, and next assignments.

## Required Checks

Run these on a full coordination pass:

```powershell
npm run hub:status
npm run hub:heartbeats
npm run lanes:status
git status --short --branch
gh pr list --state open --json number,title,headRefName,isDraft,statusCheckRollup,url
gh run list --limit 5 --json databaseId,headBranch,headSha,status,conclusion,workflowName,displayTitle,createdAt,url
```

Run guards before editing coordination files or making commits:

```powershell
npm run lanes:guard
npm run hub:guard
```

Also check relevant worker worktree `git status --short --branch` when a worker
is stale, blocked, done, or on a branch that does not match Ledger lane state.

## Coordinator Timeline Log

Coordinator timeline entries belong in Ocentra Ledger as `note`, `report`,
`worker.update`, or `task.update` events. Generated views are disposable; the
append-only event streams under `LEDGER_ROOT` are the source of truth.

## Worker Heartbeat Log

Worker liveness is separate from semantic reports. `hub:report` is reserved for
`STARTED`, meaningful progress, `BLOCKED`, and `DONE`. Minute worker wakeups
append local-only heartbeat entries instead:

```powershell
npm run hub:heartbeat -- --state alive --note "minute wake"
npm run hub:heartbeat -- --state idle --note "waiting for instruction"
```

The primary coordinator can inspect the latest heartbeat for each worker:

```powershell
npm run hub:heartbeats
```

Heartbeat state is materialized from Ledger heartbeat events with TTL
semantics.

Use `hub:status` for work state and `hub:heartbeats` for liveness. If a worker
has a fresh heartbeat but its report is `BLOCKED`, it is alive and blocked. If a
worker has a stale heartbeat for two or more minute cycles, treat the worker
thread or heartbeat automation as stale and retarget/open it.

## Active Roadmap Order

Current completed-on-main baseline includes:

- V0.1/V0.2 scaffold, contracts, encrypted journal, SQLite query store;
- V0.3 Windows process/window capture;
- V0.4 Windows network/domain observation foundation;
- V0.5 live activity portal visibility;
- V0.5.1 browser URL/tab evidence research/spec;
- V0.5.1 browser bridge runtime boundary;
- V0.5.1 managed browser launcher/profile runtime status;
- V0.5.2 app/game evidence sessions research/spec;
- V0.5.2 app/game session runtime read model;
- network flow evidence research/spec;
- network flow evidence contract/read-model groundwork;
- network flow evidence runtime read model;
- V0.5.3 local screen evidence spec;
- V0.5.3 local screen evidence queue runtime;
- V0.6 local AI safety decision contracts and context-builder plan;
- V0.7 local AI provider/runtime status command;
- V0.7 local AI evidence context-builder contracts;
- V0.7 local AI dry-run policy evaluator;
- V0.7 context-builder runtime read-path hardening;
- V0.7 portal policy-preview shell;
- V0.7 policy-preview service/API read path with TypeScript protocol contracts
  and Rust service parity;
- V0.7 portal policy-preview read-model wiring with typed service result
  rendering and preview-only enforcement messaging;
- V0.7 parent-rule context preview bridge from context-builder/read-model
  references into the typed service/API response;
- V0.7 local provider/runtime status contract hardening with explicit privacy,
  adapter-boundary, execution-state, and provider-source fields that remain
  unavailable/local-only by default;
- V0.7 portal boundary-field visibility for policy-preview local runtime and
  parent-rule context details;
- V0.7 parent-rule context resolver integration from local rule/read-model
  evidence without enabling enforcement;
- V0.7 local provider adapter probe status with probe/configuration state and
  execution-allowed flags, still no model execution;
- V0.7 network-flow v4 reconciliation with digest payload rollups and direct
  unusual-indicator evidence, still local/read-model only;
- V0.7 parent-rule preview quality/coverage with target aliases and fully
  grounded local parent-rule context matching, still dry-run only;
- hub hook session hardening;
- roadmap runtime order update.

Correct active checkpoint order:

1. Pull and validate current `main` after the V0.7 preview-completion merges.
2. Prepare the manual test pass for local service, portal, evidence read models,
   and LAN development flows across the available PCs.
3. Keep workers parked unless a validation failure or explicit follow-up slice
   needs a branch.
4. V0.8 enforcement adapters only after V0.7 preview decisions are typed,
   evidence-cited, visible, validated on `main`, and the checkpoint has been
   deliberately reviewed.

The V0.7 start gate is now satisfied because browser, app/game, network, and
screen evidence references have all landed on `main` through typed local
contracts/read paths. Do not start enforcement early.

## Current Lane Intent

The previous V0.7 foundation, preview-shell, context-builder read-path,
service/API read-path, portal read-model wiring, parent-rule context bridge,
local provider status hardening, portal boundary visibility, parent-rule
context resolver, local provider adapter probe, network-flow reconciliation,
and parent-rule preview quality batches are merged. Current lane ownership
should stay parked unless validation finds a bug or the user explicitly resumes
implementation.

If Ledger lane state and the live branch disagree, send one targeted hub message and
state which worker chat/worktree needs attention. Do not spam duplicate
messages when unread mail already exists.

## Worker Protocol

Workers must:

- check mailbox every wake;
- acknowledge latest hub mail;
- report `STARTED` before work;
- lock exact paths before edits;
- report `waiting for instruction` only when the lane has no active assignment
  from primary, and do that through `hub:heartbeat`, not `hub:report`;
- keep routine reports short unless hub mail asks for detail;
- verify and run requested lint/tests before `DONE`;
- make local commits on their worker branches after the assigned scope is
  verified;
- push their worker branch when ready for review;
- open a PR when the user or primary explicitly asks for one;
- never merge PRs or push directly to `main` unless the user explicitly asks for
  that exact action;
- never delete per-minute worker heartbeats.

Worker minute heartbeats must not overwrite useful state. If a lane has an
active assignment and there is no unread mail, the worker should append liveness
with `hub:heartbeat`, then continue useful assigned work, report real
progress/`BLOCKED`/`DONE`, or stay quiet. It should not replace `STARTED`,
`BLOCKED`, or `DONE` with `waiting for instruction`.

`DONE` and PR-ready handoffs must include detailed scope:

- what changed;
- touched packages/files;
- validation commands/results;
- commit state;
- known gaps/risks;
- roadmap slice completed;
- PR body outline when relevant.

## Stale Or Blocked Workers

Treat these as action signals:

- `session=-`;
- unread hub mail persists after a worker wake;
- latest report is `waiting for instruction` despite active assignment;
- latest heartbeat is stale for two or more minute cycles;
- live branch does not match Ledger lane state;
- worker reports `BLOCKED`;
- locks overlap or block another active lane.

Actions:

1. Check `hub:status`, `lanes:status`, and the worker worktree Git status.
2. If unread assignment already exists, do not send duplicates; state the worker
   chat/worktree must be opened or retargeted.
3. If a worker is on the wrong branch, send one command-style hub message:
   fetch/pull main, switch/create the assigned branch from `origin/main`, run
   hub/lanes guards, then report `STARTED` or `BLOCKED`.
4. If a worker is blocked on locks, compare path ownership and either tell it to
   wait, retarget to non-overlapping files, or ask the lock owner for a status.

## Review And PR Lifecycle

When a worker reports `DONE`:

1. Inspect branch diff against the intended base.
2. Confirm touched paths match assignment and locks.
3. Confirm validation commands/results.
4. Confirm detailed scope is present.
5. Ask for fixes if scope, tests, or behavior are weak.
6. Create/update PR only after local validation and pushed branch are acceptable.

PR bodies must include detailed scope, validation, known gaps/risks, touched
packages/files, and roadmap slice.

Watch PR CI. If CI fails, route fixes to the owning worker unless the failure is
clearly a coordinator-only integration issue. Merge only after green CI and
acceptable review.

After merge:

1. Pull latest `main` in primary.
2. Update roadmap/lane/hub state.
3. Free or retarget the completed lane.
4. Tell active workers to fetch/rebase latest `main`.
5. Include detailed post-merge scope plus PR/merge state and next roadmap action.

## Conflict Rule

Workers resolve conflicts on their own branches after fetching/rebasing latest
`main`. Primary resolves only integration conflicts it owns and must keep the
worker informed.

## Product Boundaries

Browser URL/tab evidence, app/game evidence sessions, network flow evidence, and
local screen evidence are required pre-AI product bridges. Process/window and
basic network/domain observations are foundation only; they are not proof of
exact active URL/tab, native game duration, decrypted network content, or
complete page semantics.

Ocentra-hosted services are account/control-plane/notification/stateless-compile
surfaces, not the default store for child activity, journals, screenshots,
reports, browser history, or parent rules.

Ocentra provides typed evidence and parent-controlled rules/settings. It must
not hard-code hidden moral judgments as policy.

## Minute Heartbeat Behavior

The minute heartbeat should be small:

- check the last visible state;
- check whether workers are stale, blocked, done, or on the wrong branch;
- read the latest timeline entries when state feels disconnected;
- check `npm run hub:heartbeats` when worker liveness is unclear;
- append a compact timeline snapshot after major state changes or every 15-20
  minutes while active;
- if action is needed, do the smallest targeted coordination action;
- if five minutes or more have passed since the last full pass, run the full
  coordination checks above;
- otherwise stay quiet.

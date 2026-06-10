# Two-PC Codex Sync

Ocentra Parent coordination state must travel through Git, not user-global `.codex`.

Actual worktree folders may still be copied or recreated manually per PC. The coordination state that tells Codex what exists, who owns it, what is blocked, what PR owns it, and what is safe to delete lives in this repo.

## What Syncs Through Git

Committed repo state:

- `.hub/lane-ledger.json`
- `.hub/codex-rules.md`
- `.hub/state/worktree-lanes.json`
- `.hub/state/ocentra-parent-hub`
- `.hub/state/machines/*.json`
- `docs/hub/*.md`

Disposable telemetry:

- `.hub/state/ocentra-parent-hub/worker-heartbeats.ndjson`
- `.hub/state/ocentra-parent-hub/lanes/*/heartbeat.ndjson`

Heartbeat files may be blanked or truncated. They are useful for local freshness checks, but they are not the source of truth.

## Required PC Workflow

Before doing coordination or lane work on either PC:

```powershell
npm run hub:state:sync
npm run hub:status
npm run lanes:status
```

After semantic coordination changes:

```powershell
npm run hub:state:sync -- -Commit
```

Semantic coordination changes include:

- lane claim/free/retarget;
- `STARTED`, `BLOCKED`, `DONE`, or `PR_READY` reports;
- hub messages;
- file locks/unlocks;
- PR state changes;
- merge or close decisions;
- cleanup-safe decisions;
- ledger updates.

Routine heartbeat-only changes do not need a commit.

## Main Branch Rule

`npm run hub:state:sync -- -Commit` stages and commits only repo-owned hub files. It does not stage product files.

The script refuses to push from `main` unless `-Push -AllowMainPush` is explicitly supplied. Normal flow is:

1. Commit migration/tooling changes on a `codex/...` branch and merge through PR.
2. After that, use normal branch/PR flow for product work.
3. Use hub sync commits only for semantic coordination state, not feature code.

## Concurrency Rule

Only one PC may actively own a lane at a time. The lane owner is recorded in:

- `.hub/state/worktree-lanes.json`
- `.hub/lane-ledger.json`
- `.hub/state/machines/<machine-id>.json`

If UP PC wants to work on `codex-b`, DOWN PC must not also edit that same lane. It may coordinate or work on another lane after syncing.

## Conflict Rule

If Git reports conflicts in `.hub/state`, stop and inspect. Do not overwrite the other PC's hub state blindly.

Preferred conflict resolution:

1. Preserve both semantic reports/messages when possible.
2. Keep the latest `ownership.json` lane lock if only one lane owner is legitimate.
3. For heartbeat files, keep either side or blank the file.
4. Update `.hub/lane-ledger.json` and `docs/hub/lane-ledger.md` to reflect the resolved truth.

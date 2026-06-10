# Two-PC Codex Sync

Ocentra Parent live coordination state must not travel through product-repo Git commits. Git is for code, durable hub rules, lane declarations, and reviewed docs. Live mailbox/status/ack/heartbeat/ownership traffic belongs in OcentraHub.

Actual worktree folders may still be copied or recreated manually per PC. Durable state that tells Codex what lane exists, which branch or PR owns it, what is merged, and what is safe to delete lives in this repo. Hot operational state lives outside the repo.

## What Stays In Git

Committed repo state:

- `.hub/lane-ledger.json`
- `.hub/codex-rules.md`
- `.hub/hub.config.json`
- `docs/hub/*.md`

## What Does Not Stay In Git

- `.hub/state/**`
- lane inbox/status/ack files
- heartbeat logs
- watch logs
- mutable ownership files
- machine-local working state

These are generated or operational views. OcentraHub owns their event log and materialization.

## Required PC Workflow

Before doing coordination or lane work on either PC, sync the external hub transport. During migration, verify the configured legacy external hub root is current, then inspect:

```powershell
npm run hub:status
npm run lanes:status
```

After semantic coordination changes, publish them through OcentraHub. Until OcentraHub exists, use the configured legacy external hub root and do not commit live state into `OcentraParent`.

```powershell
ocentra-hub sync --hub ocentra-parent
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

Routine heartbeat-only changes do not need product repo commits.

## Concurrency Rule

Only one PC may actively own a lane at a time. The lane owner is recorded in OcentraHub ownership events and summarized in:

- `.hub/lane-ledger.json`

If UP PC wants to work on `codex-b`, DOWN PC must not also edit that same lane. It may coordinate or work on another lane after syncing.

## Conflict Rule

Ownership conflicts must be represented as events, not overwritten mutable files. If two lanes claim the same path, preserve both claims and resolve with an explicit ownership resolution event.

Preferred conflict resolution:

1. Preserve both semantic reports/messages when possible.
2. Keep both ownership claims until primary decides which lane owns the path.
3. Expire or compact heartbeat events; do not treat them as durable truth.
4. Update `.hub/lane-ledger.json` and `docs/hub/lane-ledger.md` when the durable lane state changes.

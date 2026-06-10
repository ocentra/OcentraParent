# UP PC Resume Handoff

Purpose: preserve the current coordination conversation even if Codex thread history does not transfer.

## Current Migration Goal

Move Ocentra Parent Codex coordination truth into the repo so DOWN PC and UP PC can coordinate through Git. Actual worktree folders may still live outside the repo or be copied manually.

Current branch:

```text
codex/repo-owned-codex-hub-sync
```

Current primary lane:

```text
primary on codex/repo-owned-codex-hub-sync
```

## What Was Changed

- Default lane state moved from `C:\Users\sujan\.codex\ocentra-parent-worktrees.json` to `.hub/state/worktree-lanes.json`.
- Default hub mailbox state moved from `C:\Users\sujan\.codex\ocentra-parent-hub` to `.hub/state/ocentra-parent-hub`.
- Portable lane truth added in `.hub/lane-ledger.json` and `docs/hub/lane-ledger.md`.
- Two-PC sync docs added in `docs/hub/two-pc-sync.md`.
- Sync helper added as `npm run hub:state:sync`.
- Audit helper added as `npm run hub:lane-ledger:audit`.
- Heartbeat files were blanked/truncated because they are disposable liveness telemetry.

## What UP PC Should Do First

From `E:\OcentraParent` on UP PC:

```powershell
git fetch origin
git checkout codex/repo-owned-codex-hub-sync
npm run hub:state:sync
npm run hub:status
npm run lanes:status
```

If the branch is not local:

```powershell
git fetch origin codex/repo-owned-codex-hub-sync
git checkout -b codex/repo-owned-codex-hub-sync origin/codex/repo-owned-codex-hub-sync
```

## Current Coordination State

PR sequencing is paused while the hub migration is being made portable.

Open PR state before migration:

- PR #546 `Network live capture execution proof`: blocked by Windows Real Portal To Rust E2E failure.
- PR #545 `Preserve screen AI full-scope branch`: behind main, previously green before PR548 advanced main.
- PR #544 `Complete D browser runtime proof batch`: behind main, fail-fast failed.
- PR #548 `App-install dispatch executor receipt proof`: merged into main.

Do not send broad sync/rebase waves to workers. When PR sequencing resumes, pick one PR/lane at a time.

## Lane Portability Audit

Active A/B/C/D/E branches are pushed and can be recreated from remote. `codex-c` previously had 259 dirty local entries, but that work was preserved as WIP commit `e3d7253a4` on `origin/codex/app-game-control-product-completion`. That commit is portability-only and not a PR-ready claim. Read `docs/hub/lane-remote-recreation-matrix.md` before trying to recreate or clean up lanes upstairs.

## Thread Rule

Codex chat threads may not transfer reliably across PCs. The repo hub is the durable memory. If this chat is missing upstairs, open a new primary chat in the same checkout and say:

```text
I am on UP PC. Read .hub/state/up-pc-resume-handoff.md and continue the hub migration.
```

The new chat should read this file, then run:

```powershell
npm run hub:state:sync
npm run hub:status
npm run lanes:status
git status --short --branch
```

Then continue from the current branch and PR state.

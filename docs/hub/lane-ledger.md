# Hub Lane Ledger

This page is the human-readable companion to `.hub/lane-ledger.json`. The JSON ledger is the machine-readable repo-owned source of truth. This Markdown file is the review surface for humans moving between PCs.

Operational Codex coordination state defaults to `.hub/state/worktree-lanes.json` and `.hub/state/ocentra-parent-hub`. Actual worktree checkouts may live outside the repo. User-global `.codex` is only a temporary recovery/override location, not normal Ocentra Parent state.

## Mandatory Rule

No Codex lane work is complete unless it updates:

- `.hub/lane-ledger.json`
- `docs/hub/lane-ledger.md`

Workers and primary reports must say which ledger files were updated, or explicitly say no ledger update was needed and why.

## Current Snapshot

Updated: `2026-06-10T00:17:30.000Z`

| Lane      | Work                                             | Branch                                                       |  PR | Status           | Current Meaning                                                                                           | Next Action                                                               |
| --------- | ------------------------------------------------ | ------------------------------------------------------------ | --: | ---------------- | --------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------- |
| `primary` | Primary coordination lane                        | `main`                                                       |   - | `main-merged`    | Primary is on latest main after PR546 merge commit `18d0e15a558b4c43afa250ed9953c9a30648b3e7`.            | Continue one-at-a-time PR integration from latest main.                   |
| `codex-a` | Tracking plan full continuation                  | `codex/tracking-plan-full-continuation-a`                    |   - | `blocked`        | Worker reports waiting for merge wave locks.                                                              | Resume only when selected or explicitly unblocked.                        |
| `codex-b` | Preserve screen AI full-scope branch             | `codex/screen-ai-full-scope-b`                               | 545 | `open-pr`        | PR was green before PR548 advanced main; now behind.                                                      | When selected, rebase, validate, and integrate one-at-a-time.             |
| `codex-c` | Unified native app/game product completion       | `codex/app-game-control-product-completion`                  |   - | `dirty-local`    | Large dirty full-scope app/game work remains local and blocked by shared locks.                           | Do not force micro PRs; preserve full scope until selected/unblocked.     |
| `codex-d` | Complete D browser runtime proof batch           | `codex/d-runtime-ready`                                      | 544 | `blocked`        | PR is behind and fail-fast failed.                                                                        | Keep paused until selected, then route lint/fail-fast fix to D.           |
| `E-A`     | User/live UI lane                                | `codex/portal-theme-toggle-ui-polish`                        | 291 | `stale-preserve` | User-owned UI lane; prior PR291 merged.                                                                   | Primary only acts for sync/FYI or PR/CI/main-safety issues.               |
| `E-B`     | App-install dispatch executor receipt proof      | `codex/e-b-app-install-dispatch-executor-receipt`            | 548 | `main-merged`    | PR548 merged into main and primary pulled main.                                                           | Free/retarget only after local audit confirms no dirty or unpushed work.  |
| `E-C`     | Production support public surface export closure | `codex/e-c-production-support-public-surface-export-closure` | 547 | `main-merged`    | Prior coordinator state says PR547 merged.                                                                | Free/retarget only after local audit confirms no dirty or unpushed work.  |
| `E-D`     | Network live capture execution proof             | `codex/network-live-capture-execution-proof`                 | 546 | `main-merged`    | PR546 merged into main as `18d0e15a558b4c43afa250ed9953c9a30648b3e7`; CI and package previews were green. | Switch or retarget E-D only after local audit; remote branch was deleted. |

## Portable Recovery Procedure

Run this on the PC that has the worktrees before moving or deleting anything:

```powershell
git fetch --all --prune
git worktree list
powershell -ExecutionPolicy Bypass -File scripts/hub/audit-lanes.ps1 -SearchRoots "E:\","D:\","C:\Users\$env:USERNAME"
```

Classify every found checkout as one of:

- `main-merged`
- `open-pr`
- `remote-branch-no-pr`
- `local-unpushed`
- `dirty-local`
- `stale-preserve`
- `blocked`
- `safe-delete`

Then update `.hub/lane-ledger.json` and this file with the audited state before any cleanup.

## Two-PC Sync

When using DOWN PC and UP PC at the same time, Git is the sync transport. Before touching coordination state on either PC:

```powershell
npm run hub:state:sync
```

After semantic hub changes:

```powershell
npm run hub:state:sync -- -Commit
```

Only one PC may actively own a lane at a time. Another PC may coordinate or work a different lane only after syncing.

## Cleanup Rule

`safeToDeleteWorktree` may be set to `true` only when all are true:

- `git status --short` is empty.
- The lane has no commits missing from `origin/main` unless those commits are already merged through a PR.
- The branch has no open PR and no needed remote-only work.
- The lane owner or primary coordinator recorded the cleanup decision in the ledger.

## Heartbeat Rule

Heartbeat files are liveness telemetry, not semantic state. They can be blanked or truncated during repo migration. The semantic coordination state is in:

- `.hub/state/worktree-lanes.json`
- `.hub/state/ocentra-parent-hub/lanes/*/ownership.json`
- `.hub/state/ocentra-parent-hub/lanes/*/inbox.md`
- `.hub/state/ocentra-parent-hub/lanes/*/status.md`
- `.hub/lane-ledger.json`
- `docs/hub/lane-ledger.md`

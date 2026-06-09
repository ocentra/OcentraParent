# Branch Cleanup Audit

Updated: 2026-06-09T15:19:40.351Z

Purpose: record the final Codex branch cleanup and recovery map without relying on local .codex state.

## Final Result

- Remote origin/codex/\* branches were reduced from 466 to 9.
- Remaining remote branches are only active lanes or open PR branches.
- Deleted refs were archived first with GitHub-visible tags.
- No main history was rewritten or changed by this cleanup.

## Cleanup Performed

- Archived 281 exact merged-PR remote branch heads under `archive/codex-branches/*`, then deleted those 281 remote branches.
- Archived 271 matching local exact-merged branch heads under `archive/local-codex-branches/*`, then deleted 270 local branch names.
- Archived 57 local-only branch heads under `archive/local-only-branches/*`, then deleted 56 local-only branch names.
- Archived 176 manual-review remote branch heads under `archive/manual-review-branches/*`, then deleted those 176 remote branches.
- Archived 176 matching local manual-review branch heads under `archive/local-manual-review-branches/*`, then deleted those 176 local branch names.
- Kept `codex/browser-child-intervention-page-ui` locally because it is checked out by `E:/OcentraParentWorktrees/browser-child-intervention-ui/OcentraParent`; its head is preserved by `archive/local-only-branches/codex/browser-child-intervention-page-ui`.

## Remaining Remote Codex Branches

- `origin/codex/app-game-control-product-completion`
- `origin/codex/d-runtime-ready`
- `origin/codex/e-b-app-install-dispatch-executor-receipt`
- `origin/codex/e-c-production-support-public-surface-export-closure`
- `origin/codex/network-live-capture-execution-proof`
- `origin/codex/portal-theme-toggle-ui-polish`
- `origin/codex/repo-owned-codex-hub-sync`
- `origin/codex/screen-ai-full-scope-b`
- `origin/codex/tracking-plan-full-continuation-a`

## Open PRs

| PR   | Branch                                       | State   | Title                                   |
| ---- | -------------------------------------------- | ------- | --------------------------------------- |
| #549 | `codex/repo-owned-codex-hub-sync`            | BLOCKED | Move Codex coordination state into repo |
| #546 | `codex/network-live-capture-execution-proof` | BLOCKED | Network live capture execution proof    |
| #545 | `codex/screen-ai-full-scope-b`               | BEHIND  | Preserve screen AI full-scope branch    |
| #544 | `codex/d-runtime-ready`                      | BEHIND  | Complete D browser runtime proof batch  |

## Remaining Local-Only Branches

- `codex/browser-child-intervention-page-ui`

## Recovery Tags

| Deleted branch family                 | Archive tag namespace                    |
| ------------------------------------- | ---------------------------------------- |
| Exact merged remote branches          | `archive/codex-branches/*`               |
| Matching local exact-merged branches  | `archive/local-codex-branches/*`         |
| Local-only branches                   | `archive/local-only-branches/*`          |
| Manual-review remote branches         | `archive/manual-review-branches/*`       |
| Matching local manual-review branches | `archive/local-manual-review-branches/*` |

## Safety Rules Used

- Archive before deleting any branch ref.
- Keep every active lane branch.
- Keep every open PR branch.
- Do not mutate main.
- Treat deleted branch names as recoverable through their archive tags, not as lost work.

## Next Coordination State

PR sequencing is still paused except for the repo-owned hub migration PR. Active lane branches are clean and remote-backed; after this migration lands, UP PC can recreate lane worktrees from the remote branch matrix and repo-owned hub ledger.

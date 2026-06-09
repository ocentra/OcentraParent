# Lane Remote Recreation Matrix

Updated: 2026-06-09.

Purpose: show whether the active Ocentra Parent lanes can be recreated on UP PC from Git remotes, assuming the same `E:\OcentraParent` and worktree layout can be recreated or copied manually.

## Active Lane Matrix

| Lane      | Thread                                | Branch                                                       | Remote Exists | Local vs Remote                                                               | Dirty Local | Open PR        | Recreate On UP PC                        |
| --------- | ------------------------------------- | ------------------------------------------------------------ | ------------- | ----------------------------------------------------------------------------- | ----------: | -------------- | ---------------------------------------- |
| `codex-a` | `tracking-plan-full-continuation-a`   | `codex/tracking-plan-full-continuation-a`                    | yes           | even with `origin/codex/tracking-plan-full-continuation-a`                    |           0 | none           | yes, from remote                         |
| `codex-b` | `screen-ai-pipeline-b`                | `codex/screen-ai-full-scope-b`                               | yes           | even with `origin/codex/screen-ai-full-scope-b`                               |           0 | #545 `BEHIND`  | yes, from remote                         |
| `codex-c` | `app-game-control-product-completion` | `codex/app-game-control-product-completion`                  | yes           | even with `origin/codex/app-game-control-product-completion`                  |           0 | none           | yes, from remote WIP preservation commit |
| `codex-d` | `browser-social-ui-proof-d`           | `codex/d-runtime-ready`                                      | yes           | even with `origin/codex/d-runtime-ready`                                      |           0 | #544 `BEHIND`  | yes, from remote                         |
| `E-A`     | `E-A`                                 | `codex/portal-theme-toggle-ui-polish`                        | yes           | even with `origin/codex/portal-theme-toggle-ui-polish`                        |           0 | none           | yes, from remote                         |
| `E-B`     | `app-install-purchase-e-b`            | `codex/e-b-app-install-dispatch-executor-receipt`            | yes           | even with `origin/codex/e-b-app-install-dispatch-executor-receipt`            |           0 | merged #548    | yes, from remote                         |
| `E-C`     | `E-C`                                 | `codex/e-c-production-support-public-surface-export-closure` | yes           | even with `origin/codex/e-c-production-support-public-surface-export-closure` |           0 | merged #547    | yes, from remote                         |
| `E-D`     | `eventing-network-e-d`                | `codex/network-live-capture-execution-proof`                 | yes           | even with `origin/codex/network-live-capture-execution-proof`                 |           0 | #546 `BLOCKED` | yes, from remote                         |

## Former Local-Only Work

`codex-c` previously had active local-only work. It has now been preserved as a WIP commit and pushed to the lane branch so the lane can be recreated on UP PC:

```text
lane: codex-c
branch: codex/app-game-control-product-completion
preservation commit: e3d7253a4 WIP preserve codex-c app-game local work
remote: origin/codex/app-game-control-product-completion
dirty entries after preservation: 0
```

This commit is a portability/preservation commit only. It is not a PR-ready or validation-complete claim.

Current result: no active lane in this matrix has local-only dirty files.

## Recreate Commands

After fetching the repo on UP PC, each safe lane can be recreated with:

```powershell
git fetch origin --prune
git worktree add E:\OcentraParentWorktrees\E-D\OcentraParent origin/codex/network-live-capture-execution-proof
```

For named local branches, use:

```powershell
git worktree add -b codex/network-live-capture-execution-proof E:\OcentraParentWorktrees\E-D\OcentraParent origin/codex/network-live-capture-execution-proof
```

Adjust the branch and path per lane.

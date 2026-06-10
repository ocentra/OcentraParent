# Lane Ledger Schema

`.hub/lane-ledger.json` is the repo-owned source of truth for portable Ocentra Parent lane declarations, PR state, validation, merge state, and cleanup safety. Live Codex mailbox, report, heartbeat, ack, and ownership transport does not belong in the product repo. It moves through OcentraHub, or through the configured legacy external hub root during migration. Actual worktree checkouts may live outside the repo.

## Required Top-Level Fields

| Field           | Meaning                                               |
| --------------- | ----------------------------------------------------- |
| `schema`        | Stable schema identifier for tooling and review.      |
| `version`       | Integer schema version.                               |
| `updatedAt`     | ISO timestamp for the last intentional ledger update. |
| `repo`          | GitHub repository owner/name.                         |
| `sourceOfTruth` | Human-readable source-of-truth statement.             |
| `statusEnum`    | Allowed lane status values.                           |
| `lanes`         | Array of lane records.                                |

## Required Lane Fields

| Field                  | Meaning                                                                              |
| ---------------------- | ------------------------------------------------------------------------------------ |
| `laneId`               | Stable lane id, such as `primary`, `codex-a`, `E-D`.                                 |
| `name`                 | Human-readable work name.                                                            |
| `localWorktreePath`    | Machine-local hint only. Use `null` when unknown or not portable.                    |
| `branch`               | Current local branch name.                                                           |
| `remoteBranch`         | Expected remote branch name, usually `origin/<branch>`.                              |
| `pr`                   | Pull request number, or `null` when none exists.                                     |
| `base`                 | Expected base ref, normally `origin/main`.                                           |
| `status`               | One of the allowed status values below.                                              |
| `lastCommit`           | Last known relevant commit SHA, or `null` when not yet audited.                      |
| `mergedToMain`         | `true` only when the lane result is known to be merged into `main`.                  |
| `dirtyLocalFiles`      | Known dirty or locked files. Use an empty array when none are known.                 |
| `lastValidation`       | Commands or CI facts supporting the current state.                                   |
| `nextAction`           | The next coordinator or worker action.                                               |
| `safeToDeleteWorktree` | `true` only after a local audit proves no dirty, unpushed, or unmerged work remains. |

## Status Values

| Status                | Use When                                                                                               |
| --------------------- | ------------------------------------------------------------------------------------------------------ |
| `main-merged`         | The lane output is merged into `main`.                                                                 |
| `open-pr`             | A remote PR exists and is still a live integration candidate.                                          |
| `remote-branch-no-pr` | The branch exists remotely but no open PR is known.                                                    |
| `local-unpushed`      | Local commits exist that are not pushed to a remote tracking branch.                                   |
| `dirty-local`         | The worktree has modified or untracked files.                                                          |
| `stale-preserve`      | The lane should be preserved, but it is not currently an integration candidate.                        |
| `blocked`             | Work or integration is blocked by conflicts, failing checks, locks, missing validation, or user pause. |
| `safe-delete`         | A completed stale lane has been audited and may be removed.                                            |

## Status Precedence

When more than one status could apply, choose the most operationally urgent status:

1. `dirty-local`
2. `local-unpushed`
3. `blocked`
4. `open-pr`
5. `remote-branch-no-pr`
6. `main-merged`
7. `stale-preserve`
8. `safe-delete`

## Completion Rule

No Codex lane work is complete unless it updates both:

- `.hub/lane-ledger.json`
- `docs/hub/lane-ledger.md`

Every `DONE`, `PR_READY`, merge, close, retarget, or cleanup-safe report must include the ledger status and the files updated. If no ledger update is needed, the report must say why.

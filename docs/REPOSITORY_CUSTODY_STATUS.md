# Repository Custody Status

Snapshot: 2026-08-17. This document records where unmerged Ocentra Parent work
is physically and remotely recoverable. It is not a completion or CI claim.

## Protected branch and PR truth

| Ref | Head | Custody state |
| --- | --- | --- |
| `origin/main` | `eb4e66a79` | Protected release integration baseline. |
| `origin/develop` | `4ece51528` | Three commits ahead of `main`; primary checkout tracks this ref. |
| `origin/codex/eventing-wp09-production` | `7dc09c25f` | Clean, pushed source-integration checkpoint; 222 commits ahead of `develop` and 225 ahead of `main`, with neither branch diverged from it. |
| `origin/codex/data-custody-plan-code-wave` | `ec129d668` | Remote-safe six-commit draft rejected for integration: no shipped caller, caller-mintable/stale authority, no actual local custody execution, and five unrelated storage-only commits. |
| `origin/codex/screen-ai-real-authority-source` | `c8eef33cd` | Worker tip is remote-safe. Its accepted semantic delta plus root's dead-constant cleanup is integrated as `7dc09c25f`; the worker worktree is removed. |
| archive refs | `ac9f65bb4`, `405e7fc77` | Protected coverage for historical local/remote tips before branch cleanup. |
| `origin/production` | `683a07cf3` | Historical production ref; not the current source-integration line. |

Open pull requests at this snapshot: **0**. No current source lane is authorized
to bypass review, focused validation, precommit, or CI merely because it is
pushed.

## Registered E-drive worktrees

| Worktree | Branch/head | Current disposition |
| --- | --- | --- |
| `E:/OcentraParent` | `develop` / `4ece51528` | Clean tracked checkout except user-owned untracked `.codex/config.toml`; do not use for parallel feature edits. |
| `E:/OcentraWorktrees/lanes/eventing-wp09-production` | `codex/eventing-wp09-production` / `7dc09c25f` | Root integration/review lane and current remote-safe checkpoint. |
| `E:/OcentraWorktrees/lanes/ai-network-analyzer-source` | `codex/ai-network-analyzer-source` / `7b09e403f` | Active Luna source lane resolving the AI-owned analyzer/prompt/result producer exposed by Network. |
| `E:/OcentraWorktrees/lanes/tracking-runtime-ingress-source` | `codex/tracking-runtime-ingress-source` / `7dc09c25f` | Active Luna source lane mapping and, if authorized, composing WP40 trusted ingress/journal ownership. |
| `E:/OcentraWorktrees/lanes/child-runtime-transport-source` | `codex/child-runtime-transport-source` / `7dc09c25f` | Active Luna source lane for the shipped child/Android event or command transport. |

No Ocentra Parent worktree is authorized on `C:`. Old Codex ledger records are
append-only history and are not proof that a task, process, or checkout is
currently active.

## Consolidation rules

- A lane is integrated only after root reviews the diff and verifies a reachable
  production caller, trusted input, and material effect.
- A clean/pushed branch is recoverable, not accepted. Rejected packets remain
  remote-safe until superseded or deliberately archived.
- Worktrees are removed only after the accepted commits are present on the
  integration ref or the rejected tip is protected remotely, the tree is clean,
  and exact Enforcer claims are released.
- Source waves do not run broad tests, proof, precommit, or CI. Those phases
  begin only after the coherent production-source and expected-test writing
  waves are complete.

## Known non-branch custody

The integration lane retains one intentional stash for the Eventing WP09
durable network-ingestion draft. It is also covered by an archive ref and must
not be dropped until its patch identity is reconciled against accepted source.
The Enforcer ledger still contains historical claims for many deleted
worktrees; verified missing-root claims may be released, while claims attached
to an existing directory require a manual content-custody check first.

Completed no-change Account, Device Trust, and Network audit worktrees were
clean, had no unique commits, and were removed. The rejected Data Custody and
accepted Screen AI worker worktrees were also removed only after their exact
heads matched pushed remote refs.

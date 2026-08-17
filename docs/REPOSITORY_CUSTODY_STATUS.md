# Repository Custody Status

Snapshot: 2026-08-17. This document records where unmerged Ocentra Parent work
is physically and remotely recoverable. It is not a completion or CI claim.

## Protected branch and PR truth

| Ref | Head | Custody state |
| --- | --- | --- |
| `origin/main` | `eb4e66a79` | Protected release integration baseline. |
| `origin/develop` | `4ece51528` | Three commits ahead of `main`; primary checkout tracks this ref. |
| `origin/codex/eventing-wp09-production` | `302d9459c` | Clean, pushed source-integration checkpoint; 220 commits ahead of `develop` and 223 ahead of `main`, with neither branch diverged from it. |
| `origin/codex/data-custody-plan-code-wave` | `4aa8ff0b2` | Remote-safe but rejected for integration pending replacement of caller-minted/self-attested authority and disconnected source with a real runtime owner. |
| archive refs | `ac9f65bb4`, `405e7fc77` | Protected coverage for historical local/remote tips before branch cleanup. |
| `origin/production` | `683a07cf3` | Historical production ref; not the current source-integration line. |

Open pull requests at this snapshot: **0**. No current source lane is authorized
to bypass review, focused validation, precommit, or CI merely because it is
pushed.

## Registered E-drive worktrees

| Worktree | Branch/head | Current disposition |
| --- | --- | --- |
| `E:/OcentraParent` | `develop` / `4ece51528` | Clean tracked checkout except user-owned untracked `.codex/config.toml`; do not use for parallel feature edits. |
| `E:/OcentraWorktrees/lanes/eventing-wp09-production` | `codex/eventing-wp09-production` / `302d9459c` | Root integration/review lane and current remote-safe checkpoint. |
| `E:/OcentraWorktrees/lanes/data-custody-plan-code-wave` | `codex/data-custody-plan-code-wave` / `4aa8ff0b2` | Active repair/review lane; preserve until accepted replacement or archived disposition. |
| `E:/OcentraWorktrees/lanes/account-identity-source-completion` | `codex/account-identity-source-completion` / `302d9459c` | Read-only source audit completed with no edit: durable repository and Cloudflare/session caller are missing. Disposable only after final guard/claim review. |
| `E:/OcentraWorktrees/lanes/device-trust-source-completion` | `codex/device-trust-source-completion` / `302d9459c` | Read-only source audit completed with no edit: shipped platform/passkey issuer is missing. Disposable only after final guard/claim review. |
| `E:/OcentraWorktrees/lanes/network-runtime-source-completion` | `codex/network-runtime-source-completion` / `302d9459c` | Active Luna production-source lane. |
| `E:/OcentraWorktrees/lanes/screen-ai-real-authority-source` | `codex/screen-ai-real-authority-source` / `302d9459c` | Active Luna production-source lane removing synthetic policy authority. |

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

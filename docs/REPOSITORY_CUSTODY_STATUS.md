# Repository Custody Status

Snapshot: 2026-08-17. This document records where unmerged Ocentra Parent work
is physically and remotely recoverable. It is not a completion or CI claim.

## Protected branch and PR truth

| Ref | Head | Custody state |
| --- | --- | --- |
| `origin/main` | `eb4e66a79` | Protected release integration baseline. |
| `origin/develop` | `4ece51528` | Three commits ahead of `main`; primary checkout tracks this ref. |
| `origin/codex/eventing-wp09-production` | `e4272372b` | Clean, pushed source-integration checkpoint; 226 commits ahead of `develop` and 229 ahead of `main`, with neither branch diverged from it. It includes the Screen AI fail-honest boundary, Tracking WP40 ownership map, Enforcement managed-browser fail-honest correction, and the refreshed source/custody matrix. |
| `origin/codex/data-custody-plan-code-wave` | `ec129d668` | Remote-safe six-commit draft rejected for integration: no shipped caller, caller-mintable/stale authority, no actual local custody execution, and five unrelated storage-only commits. |
| `origin/codex/screen-ai-real-authority-source` | `c8eef33cd` | Worker tip is remote-safe. Its accepted semantic delta plus root's dead-constant cleanup has source baseline `7dc09c25f` and remains present in the current integration head; the worker worktree is removed. |
| archive refs | `ac9f65bb4`, `405e7fc77` | Protected coverage for historical local/remote tips before branch cleanup. |
| `origin/production` | `683a07cf3` | Historical production ref; not the current source-integration line. |

Open pull requests at this snapshot: **0**. No current source lane is authorized
to bypass review, focused validation, precommit, or CI merely because it is
pushed.

## Registered E-drive worktrees

| Worktree | Branch/head | Current disposition |
| --- | --- | --- |
| `E:/OcentraParent` | `develop` / `4ece51528` | Clean tracked checkout except user-owned untracked `.codex/config.toml`; do not use for parallel feature edits. |
| `E:/OcentraWorktrees/lanes/eventing-wp09-production` | `codex/eventing-wp09-production` / `e4272372b` | Root integration/review lane and current remote-safe checkpoint. |
| `E:/OcentraWorktrees/lanes/eventing-wp11-source-wave` | `codex/eventing-wp11-source-wave` / `699adfd76` | Pushed Eventing WP11 production-source packet under root review; tests remain intentionally deferred. |
| `E:/OcentraWorktrees/lanes/account-wp02-wp05-source-wave` | `codex/account-wp02-wp05-source-wave` / `8e9a6622a` base | Active Luna production-source lane for implementation-authorized Account WP02-WP05. |
| `E:/OcentraWorktrees/lanes/device-trust-wp02-key-custody-source` | `codex/device-trust-wp02-key-custody-source` / `e4272372b` | Completed no-change source audit: existing Windows custody is real, but shipped ceremony authority/composition is missing; clean worktree pending removal. |
| `E:/OcentraWorktrees/lanes/logging-source-wave-repair` | `codex/logging-source-wave-repair` / `735df89de` | Accepted logging source wave: Rust-owned exact 18-key sensitive-key policy, generated TypeScript artifact, generated-policy sanitizer consumption, and pre-serialization Logger/portal fallback sanitization. Docs/graph reconciliation is source-only; tests, proof, CI, PR/DONE, and external composition remain deferred. |

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
Two Enforcer ledger cleanup passes released 384 stale exact-file claims across
72 verified-missing Ocentra Parent worktree roots on `C:` and `E:`. A fresh
live ledger read then confirmed zero remaining Ocentra Parent claims attached
to missing worktree roots. Other active
claims are not presumed stale: any claim attached to an existing directory or
live worker still requires a content/owner check before release.

Completed no-change Account, Device Trust, Network, child-transport, and child-
enrollment audit worktrees were clean, had no unique commits, and were removed.
The accepted Tracking and Enforcement worker packets were reviewed, integrated,
pushed, and their obsolete worker worktrees/local/remote branches were removed.
The rejected Data Custody and accepted Screen AI worker worktrees were removed
only after their exact heads matched pushed remote refs; those two worker tips
remain remote-safe pending final consolidation decisions.

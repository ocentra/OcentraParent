# Repository Custody Status

Snapshot: 2026-08-17. This document records where unmerged Ocentra Parent work
is physically and remotely recoverable. It is not a completion or CI claim.

## Protected branch and PR truth

| Ref | Head | Custody state |
| --- | --- | --- |
| `origin/main` | `eb4e66a79` | Protected release integration baseline. |
| `origin/develop` | `4ece51528` | Three commits ahead of `main`; primary checkout tracks this ref. |
| `origin/codex/eventing-wp09-production` | `bfb352c90` | Clean, pushed source-integration checkpoint; 230 commits ahead of `develop` and 233 ahead of `main`, with neither branch diverged from it. It also includes Eventing WP11's accepted private-envelope/source-caller migration and synchronized graph/matrix state; expected tests remain deferred. |
| `origin/codex/setup-wp07-source-wave` | `2a1c2a2e6` | Remote-safe packet under repair after independent review rejected hard-coded readiness truth, ID/LAN-derived trust, Start-action LAN misrouting, and erased typed read errors. It is not accepted/integrated. |
| `origin/codex/logging-source-wave` | `bf8a995ab` | Clean, pushed repair packet awaiting independent re-review. It centralizes redaction ownership, fail-closes unsupported values, enforces realpath/symlink containment, and redacts malformed-artifact diagnostics; tests remain deferred and it is not yet integrated. |
| `origin/codex/account-wp02-wp05-source-wave` | `ac03afee3a` | Remote-safe rejected packet. The new public serde records have zero production callers, no durable authority owner, caller-mintable proof/replay/freshness state, and non-monotonic lifecycle transitions. Do not integrate or count it as implementation. |
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
| `E:/OcentraWorktrees/lanes/eventing-wp09-production` | `codex/eventing-wp09-production` / `bfb352c90` | Root integration/review lane and current remote-safe checkpoint. |
| `E:/OcentraWorktrees/lanes/setup-wp07-source-wave` | `codex/setup-wp07-source-wave` / `2a1c2a2e6` plus uncommitted repair | Active Luna repair lane for the four independent-review findings; expected tests remain intentionally deferred. |
| `E:/OcentraWorktrees/lanes/logging-source-wave` | `codex/logging-source-wave` / `bf8a995ab` | Clean pushed repair packet under independent re-review; expected tests remain intentionally deferred. |
| `E:/OcentraWorktrees/lanes/account-wp02-sealed-binding-source` | `codex/account-wp02-sealed-binding-source` / `bfb352c90` base | Active Luna production-source lane for Account WP02's dependency-first sealed current member/role/device binding over WP08 identities. |

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
The accepted Eventing WP11 packet was independently reviewed, integrated,
pushed, and its worker worktree/local/remote branch were removed. The rejected
Account WP02-WP05 tip was verified remote-safe at `ac03afee3a`; its local
worktree/branch were removed and the remote tip remains quarantined for audit.
The Device Trust WP01 follow-up found no legal source packet because the shipped
parent-presence/household authority producer and authenticated platform custody
adapter are absent; it made no changes, released claims, and its clean
worktree/local branch were removed.
The Cloudflare WP06 follow-up likewise made no changes: a verified Firebase
subject cannot legally supply current member/role/device authority, and the
existing WP08 cross-runtime handoff does not expose it. Its clean worktree/local
branch were removed; Account WP02 is now the routed source prerequisite.
The accepted Tracking and Enforcement worker packets were reviewed, integrated,
pushed, and their obsolete worker worktrees/local/remote branches were removed.
The rejected Data Custody and accepted Screen AI worker worktrees were removed
only after their exact heads matched pushed remote refs; those two worker tips
remain remote-safe pending final consolidation decisions.

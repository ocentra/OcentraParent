# Repository Custody Status

Snapshot: 2026-08-17 at accepted Payment source-integration head `4c4d3530c`.
This document
records where unmerged Ocentra Parent work is physically and remotely
recoverable. It is not a completion, test, proof, or CI claim.

## Protected branch and PR truth

| Ref | Head | Custody state |
| --- | --- | --- |
| `origin/main` | `eb4e66a79` | Protected release integration baseline. |
| `origin/develop` | `4ece51528` | Three commits ahead of `main`; the primary checkout tracks this ref. |
| `origin/codex/eventing-wp09-production` | contains source checkpoint `4c4d3530c` | Root integration line now includes independently accepted App/Game WP10, Parent Runtime WP03/WP06, Eventing WP11, Device Trust WP01, and the full Payment production-source wave. Payment matrix/graph/custody refresh follows this source checkpoint; expected tests remain deliberately deferred. |
| `origin/codex/account-wp02-source-wave` | `ce3cf4c96` | Remote-safe Account source packet. It waits for the final accepted Payment repair before reconciliation and full Account-plan source review; it is not yet integrated. |
| `origin/codex/payment-source-wave` | `63305016f` | Clean independently accepted Payment source head. Its complete source range is integrated through root source checkpoint `4c4d3530c`; the branch is retained for the later expected-test wave until supersession/custody cleanup is mechanical. |
| `origin/codex/data-custody-source-wave` | `a7cf4f0fb` plus active local edits | Remote-safe replacement Data Custody checkpoint; private mutation ownership and orphaned-lease recovery are still under active source repair and require a fresh independent review. |
| `origin/codex/device-trust-wp01-source-wave` | `914d06b6a` | Clean independently accepted remaining Device Trust source head. The caller-mintable household-authority path was removed; root integration and graph refresh are next, while tests and real ceremony/platform owners remain open. |
| `origin/codex/account-wp02-wp05-source-wave` | `ac03afee3a` | Quarantined rejected Account packet retained until the accepted Account lane is proven to supersede every useful delta. |
| `origin/codex/data-custody-plan-code-wave` | `ec129d668` | Quarantined rejected/disconnected Data Custody packet retained until the replacement lane is accepted and patch-equivalence is proved. |
| archive refs | `ac9f65bb4`, `405e7fc77` | Protected coverage for historical local/remote tips before branch cleanup. |
| `origin/production` | `683a07cf3` | Historical production ref; not the current source-integration line. |

Open pull requests at this snapshot: **0**. No current source lane is authorized
to bypass independent review, the later expected-test wave, focused validation,
precommit, or CI merely because it is pushed.

## Registered E-drive worktrees

| Worktree | Branch/head | Current disposition |
| --- | --- | --- |
| `E:/OcentraParent` | `develop` / `4ece51528` | Tracked checkout with the user-owned untracked `.codex/config.toml`; do not use for parallel feature edits. |
| `E:/OcentraWorktrees/lanes/eventing-wp09-production` | `codex/eventing-wp09-production` / contains `4c4d3530c` | Root integration/review lane; accepted Payment source is consolidated here and its status/graph refresh is being pushed. |
| `E:/OcentraWorktrees/lanes/account-wp02-source-wave` | `codex/account-wp02-source-wave` / `ce3cf4c96` plus reconciliation edits | Remote-safe Account lane now reconciling the accepted Payment migration/source dependency before full independent Account review. |
| `E:/OcentraWorktrees/lanes/payment-source-wave` | `codex/payment-source-wave` / `63305016f` | Clean accepted Payment source lane; retained for expected-test writing and custody comparison. |
| `E:/OcentraWorktrees/lanes/data-custody-source-wave` | `codex/data-custody-source-wave` / `a7cf4f0fb` plus active edits | Active Luna replacement Data Custody runtime lane. The committed checkpoint is remote-safe; private mutation/recovery repair remains local until coherently committed. |
| `E:/OcentraWorktrees/lanes/device-trust-wp01` | `codex/device-trust-wp01-source-wave` / `914d06b6a` | Clean accepted Device Trust continuation lane. Root integration is next; expected tests and real family/provider/platform ceremony owners remain open. |

No registered Ocentra Parent worktree exists on `C:`. The remaining Codex
worktree folders on `C:` belong to Ocentra Enforcer and Ocentra Games, not this
repository. Old ledger records are append-only history, not evidence of a live
checkout or worker.

## Consolidation rules

- A lane is integrated only after root reviews the full production diff and an
  independent reviewer verifies reachable ownership, trusted inputs, material
  effects, fail-closed behavior, and no P0/P1 source defect.
- A clean/pushed branch is recoverable, not accepted. Rejected packets remain
  remote-safe until an accepted replacement proves every useful delta is
  present or an archive ref covers it.
- A worktree/local/remote feature branch is removed only after accepted commits
  are present on the pushed integration ref, `git cherry` or equivalent proves
  patch custody, the feature tree is clean, and exact Enforcer claims are
  released.
- Source waves do not run tests, builds, proof, precommit, or CI. After all
  coherent production source is written, the program writes the complete
  expected-test wave, then runs focused validation, repo-wide gates, proof,
  precommit, and one coherent PR/CI promotion.
- Actual feature worktrees stay on `E:`. No new Ocentra Parent worktree may be
  created under `C:/Users/sujan/.codex/worktrees`.

## Non-branch custody and completed cleanup

`git stash list` is empty. The previous Eventing WP09 stash was reconciled to a
remote archive ref and a stronger accepted integration commit before it was
dropped. There is no current Ocentra Parent change held only in a stash.

The accepted Eventing WP11, Parent Runtime, App/Game, Setup, Logging, Tracking,
Enforcement, Screen AI, Device Trust WP01, and Payment packets are present on
the integration line. Their obsolete worker worktrees/branches were removed only
after patch custody and claim release, except the Device Trust worktree, which
is deliberately retained and rebased for the rest of that plan's source wave.

The old Account and Data Custody rejected refs remain quarantined remotely.
They will be deleted only after the current accepted replacements are integrated
and mechanically proven to retain every useful delta. The two archive refs stay
until the whole source/test consolidation and promotion cycle is complete.

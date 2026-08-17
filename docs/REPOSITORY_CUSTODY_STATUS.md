# Repository Custody Status

Snapshot: 2026-08-17 at source-integration head `929858eba`. This document
records where unmerged Ocentra Parent work is physically and remotely
recoverable. It is not a completion, test, proof, or CI claim.

## Protected branch and PR truth

| Ref | Head | Custody state |
| --- | --- | --- |
| `origin/main` | `eb4e66a79` | Protected release integration baseline. |
| `origin/develop` | `4ece51528` | Three commits ahead of `main`; the primary checkout tracks this ref. |
| `origin/codex/eventing-wp09-production` | `929858eba` | Clean pushed source-integration line. It includes independently accepted App/Game WP10, Parent Runtime WP03/WP06, Eventing WP11, and Device Trust WP01 source packets plus their matrix/graph truth. Expected tests remain deliberately deferred. |
| `origin/codex/account-wp02-source-wave` | `ce3cf4c96` | Remote-safe Account source packet. It waits for the final accepted Payment repair before reconciliation and full Account-plan source review; it is not yet integrated. |
| `origin/codex/payment-source-wave` | `814f7bac7` | Remote-safe pre-review Payment checkpoint. Six production P1 findings are being repaired locally; this head is rejected for integration until a fresh independent review accepts the repaired packet. |
| `origin/codex/data-custody-source-wave` | `a3bd482b5` | Remote-safe replacement Data Custody checkpoint; the coherent child-runtime/storage effect packet is still under active local source construction and independent review remains required. |
| `origin/codex/device-trust-wp01-source-wave` | `929858eba` | Rebased to the exact integration head after WP01 acceptance. The E-drive lane is being reused for the remaining implementation-authorized Device Trust source workpacks. |
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
| `E:/OcentraWorktrees/lanes/eventing-wp09-production` | `codex/eventing-wp09-production` / `929858eba` | Clean root integration/review lane and current remote-safe source checkpoint. |
| `E:/OcentraWorktrees/lanes/account-wp02-source-wave` | `codex/account-wp02-source-wave` / `ce3cf4c96` | Clean remote-safe Account lane waiting for the final Payment packet before reconciliation and independent review. |
| `E:/OcentraWorktrees/lanes/payment-source-wave` | `codex/payment-source-wave` / `814f7bac7` plus active edits | Active Luna Payment source repair lane. No tests/build/proof/CI; root review is required before integration. |
| `E:/OcentraWorktrees/lanes/data-custody-source-wave` | `codex/data-custody-source-wave` / `a3bd482b5` plus active edits | Active Luna replacement Data Custody runtime lane. The committed checkpoint is remote-safe; the uncommitted runtime packet remains on this E-drive worktree until coherently committed. |
| `E:/OcentraWorktrees/lanes/device-trust-wp01` | `codex/device-trust-wp01-source-wave` / `929858eba` | Reused Device Trust plan lane. WP01 is integrated; source work continues only on implementation-authorized WP02/WP05/WP06/WP07 while WP03/WP04 stay dependency-blocked. |

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
Enforcement, Screen AI, and Device Trust WP01 packets are present on the pushed
integration line. Their obsolete worker worktrees/branches were removed only
after patch custody and claim release, except the Device Trust worktree, which
is deliberately retained and rebased for the rest of that plan's source wave.

The old Account and Data Custody rejected refs remain quarantined remotely.
They will be deleted only after the current accepted replacements are integrated
and mechanically proven to retain every useful delta. The two archive refs stay
until the whole source/test consolidation and promotion cycle is complete.

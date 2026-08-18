# Repository Custody Status

Snapshot: 2026-08-18 after independently reviewed Device Trust WP06, Data
Custody WP05, and Account Identity WP04 source integration at `1101f37f8`.

This document records where unpromoted Ocentra Parent work is physically and
remotely recoverable. It is not a completion, test, proof, CI, or release
claim. Refresh Git ancestry and patch identity before deleting any ref.

## Protected branch and integration truth

| Ref | Head | Custody state |
| --- | --- | --- |
| `origin/main` | `eb4e66a791` | Release baseline. It has no commit missing from consolidation and is 398 commits behind it. |
| `origin/develop` | `4ece515282` | Promotion baseline. It has no commit missing from consolidation and is 395 commits behind it. |
| `origin/codex/eventing-wp09-production` | `1101f37f8` | Clean pushed source-consolidation line. Device Trust WP06, Data Custody WP05, and Account Identity WP04 reviewed production source plus current graph truth are integrated. Expected tests remain deliberately deferred until the source wave is complete. |
| `origin/production` | `683a07cf31` | Historical production ref; not the current integration line. |
| archive refs | `ac9f65bb4a`, `405e7fc77e` | Coverage for historical local/remote tips. Retain through source/test consolidation and promotion. |

Open pull requests at this snapshot: **0**. No source packet is allowed to skip
independent review, the later expected-test wave, focused validation,
precommit, or CI merely because it is pushed.

## Patch-unique remote branch disposition

The classification uses `git cherry origin/codex/eventing-wp09-production
<ref>` plus file/commit review. Raw ahead/behind counts alone are not custody
proof.

| Remote branch | Audited head | Patch state / disposition |
| --- | --- | --- |
| `codex/device-trust-wp01-source-wave` | `914d06b6aa` | Two `git cherry`-unique commits were semantically reviewed and are superseded by stronger integrated Device Trust authority/recovery source. Do not merge the stale branch. |
| `codex/account-wp02-source-wave` | `35edb2830c` | Six `git cherry`-unique commits were semantically reviewed and are superseded or contradict the current sealed authority/runtime boundaries. Do not merge the stale branch. |
| `codex/data-custody-source-consolidation` | `8da579cc70` | Superseded stale alternate; its effect ledger was added and later removed. No direct integration. |
| `codex/data-custody-wp05-source` | `8a92cce1fb` | Two residual patches belong to the old WP05 packet; production meaning is superseded by the repaired source now in consolidation and its docs are stale. No direct integration. |
| `codex/data-custody-source-wave` | `78f01911f1` | Two residual patches remain in a broad stale alternate. Most source meaning is superseded; salvage only if a later narrow review identifies a missing invariant. |
| `codex/account-wp02-wp05-source-wave` | `ac03afee3a` | One residual patch belongs to a rejected/quarantined old identity lifecycle packet. Do not merge. |
| `codex/data-custody-plan-code-wave` | `ec129d6681` | Archive-worthy; no direct integration. |
| `codex/logging-source-wave-repair` | `e0c2d158ab` | Production patches are integrated; remaining unique patch is docs-only. Archive-worthy after custody refresh. |
| `codex/setup-wp07-source-wave` | `09f7c7c960` | Production patches are integrated; remaining unique patch is docs-only. Archive-worthy after custody refresh. |

The following 23 noncanonical source refs have zero patch-unique commits against
`1101f37f8`. They are cleanup candidates only after confirming no open PR and no
unpublished local state; their old tree deltas are ancestry noise, not unique
patch custody:

- `codex/account-cloudflare-authority-routing`
- `codex/account-data-runtime-routing`
- `codex/account-wp02-target-authority`
- `codex/account-wp03-runtime-source`
- `codex/account-wp03-source-wave`
- `codex/account-wp04-source-wave`
- `codex/child-runtime-routing-refresh`
- `codex/child-runtime-source-routing`
- `codex/cloudflare-wp06-authority-source`
- `codex/cloudflare-wp06-runtime-source`
- `codex/data-custody-wp06-source`
- `codex/data-custody-wp08-source`
- `codex/data-custody-wp05-source-repaired`
- `codex/data-wp06-query-source-wave`
- `codex/data-wp06-routing-refresh`
- `codex/data-wp08-p1-source-repair`
- `codex/device-trust-runtime-routing`
- `codex/device-trust-wp02-source-wave`
- `codex/device-trust-wp05-source-wave`
- `codex/device-trust-wp06-source-wave`
- `codex/eventing-wp08-parent-intent-ingress`
- `codex/eventing-wp11-typed-delivery`
- `codex/payment-source-wave`

## Registered E-drive worktrees

There are 23 registered Ocentra Parent worktrees after opening the Screen WP26
source lane. All 23 track an upstream and all 23 HEADs equal it exactly at this
snapshot. Twenty-two are clean; the sole exception is the root checkout's
ignored local Codex configuration recorded below. No worktree contains a
local-only commit.

| Worktree | Branch / audited head | State |
| --- | --- | --- |
| `E:/OcentraParent` | `develop` / `4ece515282` | Only untracked `.codex/config.toml`; do not use for feature edits. |
| `E:/OcentraWorktrees/lanes/eventing-wp09-production` | `codex/eventing-wp09-production` / `1101f37f8` | Clean pushed coordinator integration line. |
| `E:/OcentraWorktrees/lanes/account-wp02-source-wave` | `codex/account-wp04-source-wave` / `1101f37f8` | Clean pushed same-tree alias after Account WP04 integration. |
| `E:/OcentraWorktrees/lanes/data-custody-wp05-source` | `codex/data-custody-wp05-source-repaired` / `f8d0a888a1` | Clean pushed ancestor; repaired WP05 is integrated in consolidation. |
| `E:/OcentraWorktrees/lanes/device-trust-runtime-routing` | `codex/device-trust-wp06-source-wave` / `1b35933194` | Clean pushed; WP06 patches are integrated and this branch is a cleanup candidate. |
| `E:/OcentraWorktrees/lanes/account-cloudflare-authority-routing` | `codex/child-runtime-source-routing` / `c71becbcfd` | Clean pushed, patch-equivalent source custody. |
| `E:/OcentraWorktrees/lanes/account-data-runtime-routing` | `codex/account-wp02-target-authority` / `f6ac50434d` | Clean pushed, patch-equivalent source custody. |
| `E:/OcentraWorktrees/lanes/account-wp03-runtime-source` | `codex/account-wp03-runtime-source` / `59eefd0d23` | Clean pushed, patch-equivalent source custody. |
| `E:/OcentraWorktrees/lanes/account-wp03-source-wave` | `codex/account-wp03-source-wave` / `53b5d195d3` | Clean pushed, patch-equivalent source custody. |
| `E:/OcentraWorktrees/lanes/cloudflare-wp06-authority-source` | `codex/cloudflare-wp06-runtime-source` / `8f50794297` | Clean pushed, patch-equivalent source custody. |
| `E:/OcentraWorktrees/lanes/data-custody-source-consolidation` | `codex/data-custody-source-consolidation` / `8da579cc70` | Clean pushed stale alternate; no direct integration. |
| `E:/OcentraWorktrees/lanes/data-custody-source-wave` | `codex/data-custody-source-wave` / `78f01911f1` | Clean pushed stale alternate; narrow salvage only. |
| `E:/OcentraWorktrees/lanes/data-custody-wp06-source` | `codex/data-custody-wp06-source` / `f5b839efbc` | Clean pushed, patch-equivalent source custody. |
| `E:/OcentraWorktrees/lanes/data-custody-wp08-source` | `codex/data-custody-wp08-source` / `1d63e190c5` | Clean pushed, patch-equivalent source custody. |
| `E:/OcentraWorktrees/lanes/data-wp06-query-source-wave` | `codex/child-runtime-routing-refresh` / `1b6b5a28f6` | Clean pushed, patch-equivalent source custody. |
| `E:/OcentraWorktrees/lanes/data-wp08-p1-source-repair` | `codex/data-wp08-p1-source-repair` / `d77f8f649b` | Clean pushed, patch-equivalent source custody. |
| `E:/OcentraWorktrees/lanes/device-trust-wp01` | `codex/device-trust-wp01-source-wave` / `914d06b6aa` | Clean pushed stale branch; its two residual patches were semantically superseded. |
| `E:/OcentraWorktrees/lanes/eventing-wp08-parent-intent-ingress` | `codex/eventing-wp08-parent-intent-ingress` / `ba0854f0a9` | Clean pushed, patch-equivalent source custody. |
| `E:/OcentraWorktrees/lanes/eventing-wp11-typed-delivery` | `codex/eventing-wp11-typed-delivery` / `8fb261274c` | Clean pushed, patch-equivalent source custody. |
| `E:/OcentraWorktrees/lanes/logging-source-wave-repair` | `codex/logging-source-wave-repair` / `e0c2d158ab` | Clean pushed; production patches integrated, docs-only unique remainder. |
| `E:/OcentraWorktrees/lanes/payment-source-wave` | `codex/payment-source-wave` / `63305016fc` | Clean pushed, patch-equivalent source custody. |
| `E:/OcentraWorktrees/lanes/screen-wp26-source` | `codex/screen-wp26-source` / `1101f37f8` | Clean pushed source lane, newly assigned to the graph-authorized Screen WP26 production packet. |
| `E:/OcentraWorktrees/lanes/setup-wp07-source-wave` | `codex/setup-wp07-source-wave` / `09f7c7c960` | Clean pushed; production patches integrated, docs-only unique remainder. |

No registered Ocentra Parent worktree exists on `C:`. Historical ledger entries
or checkout-like folders without Git metadata are not live worktrees.

## Consolidation and cleanup rules

- Integrate only after full production-diff review and independent P0/P1 source
  acceptance.
- A clean/pushed branch is recoverable, not accepted.
- Never merge a broad stale branch merely because it is ahead; salvage reviewed
  commits or semantics onto current consolidation.
- Delete a feature worktree/branch only after accepted commits are on the
  pushed integration ref, fresh patch identity proves custody, the tree is
  clean, and exact Enforcer claims are released.
- Source waves do not run product tests, builds, proof, precommit, or CI. After
  coherent production source is written, write the complete expected-test
  wave, then run focused validation, repo-wide gates, proof, precommit, and
  coherent plan PR/CI promotion through `develop` to `main`.
- Actual feature worktrees stay on `E:`.

`git stash list` was empty at this snapshot. No known Ocentra Parent change is
held only in a stash.

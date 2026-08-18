# Repository Custody Status

Snapshot: 2026-08-18 after independently reviewed Device Trust WP06 source
integration `57c41b4cf`.

This document records where unpromoted Ocentra Parent work is physically and
remotely recoverable. It is not a completion, test, proof, CI, or release
claim. Refresh Git ancestry and patch identity before deleting any ref.

## Protected branch and integration truth

| Ref | Head | Custody state |
| --- | --- | --- |
| `origin/main` | `eb4e66a791` | Release baseline. It has no commit missing from consolidation and is 379 commits behind it. |
| `origin/develop` | `4ece515282` | Promotion baseline. It has no commit missing from consolidation and is 376 commits behind it. |
| `origin/codex/eventing-wp09-production` | `57c41b4cf` | Clean pushed source-consolidation line. Device Trust WP06 fail-closed recovery source and refreshed graph truth are integrated. Expected tests remain deliberately deferred until the source wave is complete. |
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
| `codex/device-trust-wp06-source-wave` | `1b35933194` | Integrated through `57c41b4cf`; retain until Data WP05 rebases across its recovery overlap, then mechanical cleanup candidate. |
| `codex/data-custody-wp05-source-repaired` | `e91bb3de10` | Current accepted-semantics candidate; rebase and semantic reconciliation on `57c41b4cf` are in progress before integration. |
| `codex/account-wp04-source-wave` | committed base `c483c8d14f` plus preserved dirty source | Unique Account invite/recovery source; finish owner-receipt/digest review, static gates, commit, rebase, independent review, then integrate. |
| `codex/device-trust-wp01-source-wave` | `914d06b6aa` | Two patch-unique safety commits require narrow review; do not merge the broad stale branch wholesale. |
| `codex/account-wp02-source-wave` | `35edb2830c` | Six patch-unique Account/payment/Cloudflare commits require commit-level review after Account WP04 and current Cloudflare truth; do not merge the broad stale branch wholesale. |
| `codex/data-custody-source-consolidation` | `8da579cc70` | Superseded stale alternate; its effect ledger was added and later removed. No direct integration. |
| `codex/data-custody-wp05-source` | `8a92cce1fb` | Superseded by repaired WP05; no direct integration. |
| `codex/data-custody-source-wave` | `78f01911f1` | Mostly patch-equivalent stale tree; salvage only through a narrow later semantic review. |
| `codex/account-wp02-wp05-source-wave` | `ac03afee3a` | Superseded by newer Account authority source; archive after Account review. |
| `codex/data-custody-plan-code-wave` | `ec129d6681` | Archive-worthy; no direct integration. |
| `codex/logging-source-wave-repair` | `e0c2d158ab` | Production patches are integrated; remaining unique patch is docs-only. Archive-worthy after custody refresh. |
| `codex/setup-wp07-source-wave` | `09f7c7c960` | Production patches are integrated; remaining unique patch is docs-only. Archive-worthy after custody refresh. |

The following non-archive source refs had zero patch-unique commits against the
audited consolidation line before the latest Device WP06 integration and are
cleanup candidates only after a fresh post-integration `git cherry`, clean
worktree check, and Enforcer claim release:

- `codex/account-cloudflare-authority-routing`
- `codex/account-data-runtime-routing`
- `codex/account-wp02-target-authority`
- `codex/account-wp03-runtime-source`
- `codex/account-wp03-source-wave`
- `codex/child-runtime-routing-refresh`
- `codex/child-runtime-source-routing`
- `codex/cloudflare-wp06-authority-source`
- `codex/cloudflare-wp06-runtime-source`
- `codex/data-custody-wp06-source`
- `codex/data-custody-wp08-source`
- `codex/data-wp06-query-source-wave`
- `codex/data-wp06-routing-refresh`
- `codex/data-wp08-p1-source-repair`
- `codex/device-trust-runtime-routing`
- `codex/device-trust-wp02-source-wave`
- `codex/device-trust-wp05-source-wave`
- `codex/eventing-wp08-parent-intent-ingress`
- `codex/eventing-wp11-typed-delivery`
- `codex/payment-source-wave`

## Registered E-drive worktrees

There are 22 registered Ocentra Parent worktrees. Twenty are clean and equal to
their upstream ref. The two exceptions are explicitly recorded below.

| Worktree | Branch / audited head | State |
| --- | --- | --- |
| `E:/OcentraParent` | `develop` / `4ece515282` | Only untracked `.codex/config.toml`; do not use for feature edits. |
| `E:/OcentraWorktrees/lanes/eventing-wp09-production` | `codex/eventing-wp09-production` / `57c41b4cf` | Clean pushed coordinator integration line. |
| `E:/OcentraWorktrees/lanes/account-wp02-source-wave` | `codex/account-wp04-source-wave` / `c483c8d14f` | Preserved dirty Account WP04 source packet; 42 status entries at audit time. |
| `E:/OcentraWorktrees/lanes/data-custody-wp05-source` | `codex/data-custody-wp05-source-repaired` / `e91bb3de10` | Clean pushed candidate; rebase/reconciliation in progress. |
| `E:/OcentraWorktrees/lanes/device-trust-runtime-routing` | `codex/device-trust-wp06-source-wave` / `1b35933194` | Clean pushed and integrated; retain temporarily for overlap comparison. |
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
| `E:/OcentraWorktrees/lanes/device-trust-wp01` | `codex/device-trust-wp01-source-wave` / `914d06b6aa` | Clean pushed; two patch-unique commits require narrow review. |
| `E:/OcentraWorktrees/lanes/eventing-wp08-parent-intent-ingress` | `codex/eventing-wp08-parent-intent-ingress` / `ba0854f0a9` | Clean pushed, patch-equivalent source custody. |
| `E:/OcentraWorktrees/lanes/eventing-wp11-typed-delivery` | `codex/eventing-wp11-typed-delivery` / `8fb261274c` | Clean pushed, patch-equivalent source custody. |
| `E:/OcentraWorktrees/lanes/logging-source-wave-repair` | `codex/logging-source-wave-repair` / `e0c2d158ab` | Clean pushed; production patches integrated, docs-only unique remainder. |
| `E:/OcentraWorktrees/lanes/payment-source-wave` | `codex/payment-source-wave` / `63305016fc` | Clean pushed, patch-equivalent source custody. |
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

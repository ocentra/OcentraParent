# Repository Custody Status

Snapshot: 2026-08-18 after the canonical truth refresh at
`b4c3a921b193c58fb3c3f5ec2707415e3063de03`.

The live promotion/custody audit found 32 registered Ocentra Parent worktrees
and zero open pull requests. Thirty-one worktrees are expected clean after this
truth-repair commit; the root checkout retains its ignored local Codex config.
No branch or worktree in this inventory is deletion-authorized by this
document.

This document records where unpromoted Ocentra Parent work is physically and
remotely recoverable. It is not a completion, test, proof, CI, or release
claim. Refresh Git ancestry and patch identity before deleting any ref.

## Protected branch and integration truth

| Ref | Head | Custody state |
| --- | --- | --- |
| `origin/main` | `eb4e66a791` | Protected release baseline. It has zero commits unique versus canonical `b4c3a921b` and is behind canonical; it is not changed by this docs refresh. |
| `origin/develop` | `4ece515282` | Protected promotion baseline. It exists, has zero commits unique versus canonical `b4c3a921b`, and is behind canonical; it is not changed by this docs refresh. |
| `origin/production` | `683a07cf31` | Historical production ref; it has zero commits unique versus canonical and is behind canonical. |
| `origin/codex/eventing-wp09-production` | `b4c3a921b` | Canonical clean pushed source-consolidation line. Reviewed source and graph truth include Screen WP32 structured extraction, Data WP05 Account-authority handoff, Account/Setup WP07 setup-first-run mapping, and Screen WP26/WP33 source-only truth. Expected tests, focused validation, proof, precommit, CI, PR, and DONE remain deliberately open. |
| archive refs | `ac9f65bb4a`, `405e7fc77e` | Coverage for historical local/remote tips. Retain through source/test consolidation and promotion. |

Open pull requests at this snapshot: **0**. No source packet is allowed to skip
independent review, the later expected-test wave, focused validation,
precommit, or CI merely because it is pushed.

Promotion controls were verified as custody facts, not modified here: remote
`develop` exists; `main` and `develop` are protected with required checks
`Format`, `Lint`, `Types`, `Rust Check`, `Full Validation Gate`, and `Package
Preview Gate`; administrators are enforced; force-push and branch deletion are
disabled; and conversation resolution is required. `ci.yml` pull requests
target `main`, `develop`, and `production`. Workflow and branch protection were
not edited by this refresh.

## Active source packets awaiting independent review

These packets are active custody, not integration evidence and not cleanup
candidates. Canonical product-source truth remains
`origin/codex/eventing-wp09-production` at `b4c3a921b` until a packet passes
independent review and is deliberately integrated.

| Branch / worktree | Latest observed state | Review disposition |
| --- | --- | --- |
| `codex/account-wp02-authority-transport` / `E:/OcentraWorktrees/lanes/account-wp02-authority-transport` | The in-flight repair advanced from `c0ec338d0` to clean pushed head `7282020448`; local and upstream equal. | Pending independent review. Do not map, accept, or infer runtime/tests from `7282020448`. |
| `codex/screen-wp32-producer-source` / `E:/OcentraWorktrees/lanes/screen-wp32-producer-source` | Clean pushed head `f992301ca`; local and upstream equal. | Pending independent review. Do not map or accept the producer packet, and do not infer tests, runtime reachability, proof, or DONE. |

## Patch-unique remote branch disposition

The current classification is anchored to canonical
`origin/codex/eventing-wp09-production` at `b4c3a921b`; the predecessor
`1101f37f8` comparison is retained below only as historical custody context.
Raw ahead/behind counts alone are not custody proof.

| Remote branch | Audited head | Patch state / disposition |
| --- | --- | --- |
| `codex/device-trust-wp01-source-wave` | `914d06b6aa` | Two patch-unique commits were semantically reviewed and superseded/rejected because the authority path was forgeable. The canonical reconciled Device Trust source is the integration truth; do not merge the stale branch. |
| `codex/account-wp02-source-wave` | `35edb2830c` | Six patch-unique commits were semantically reviewed and superseded/rejected by the canonical sealed authority/runtime boundaries. Do not merge the stale branch. |
| `codex/data-custody-source-consolidation` | `8da579cc70` | Superseded stale alternate; its effect ledger was added and later removed. No direct integration. |
| `codex/data-custody-wp05-source` | `8a92cce1fb` | Two residual patches belong to the old WP05 packet; production meaning is superseded by the repaired source now in consolidation and its docs are stale. No direct integration. |
| `codex/data-custody-wp05-source` (local-only historical tip) | `bfb85f51` | Fully superseded local-only Data WP05 source branch. No direct integration; retain only as custody history until local/worktree deletion is separately authorized. |
| `codex/data-custody-source-wave` | `78f01911f1` | Two residual patches remain in a broad stale alternate. Most source meaning is superseded; salvage only if a later narrow review identifies a missing invariant. |
| `codex/account-wp02-wp05-source-wave` | `ac03afee3a` | One residual patch belongs to a rejected/quarantined old identity lifecycle packet. Do not merge. |
| `codex/data-custody-plan-code-wave` | `ec129d6681` | Archive-worthy; no direct integration. |
| `codex/logging-source-wave-repair` | `e0c2d158ab` | Production patches are integrated; remaining unique patch is docs-only. Archive-worthy after custody refresh. |
| `codex/setup-wp07-source-wave` | `09f7c7c960` | Production patches are integrated; remaining unique patch is docs-only. Archive-worthy after custody refresh. |

The following list is the predecessor zero-patch inventory measured against
`1101f37f8`, retained for historical custody only. It is not a current
`b4c3a921b` deletion authorization. Re-audit current patch identity, open PRs,
and unpublished local state before deleting any ref; old tree deltas are
ancestry noise, not unique patch custody:

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

There are 32 registered Ocentra Parent worktrees in the live custody count.
Every worktree tracks an upstream and every committed HEAD equaled that
upstream at the audit point. No worktree contains a local-only commit. The root
checkout's ignored local Codex configuration is the only retained dirty state;
the current truth-refresh lane becomes clean when this repair commit is created.

| Worktree | Branch / audited head | State |
| --- | --- | --- |
| `E:/OcentraParent` | `develop` / `4ece515282` | Only untracked `.codex/config.toml`; do not use for feature edits. |
| `E:/OcentraWorktrees/lanes/eventing-wp09-production` | `codex/eventing-wp09-production` / `b4c3a921b1` | Clean pushed canonical product-source integration line. |
| `E:/OcentraWorktrees/lanes/account-wp02-source-wave` | `codex/account-wp04-source-wave` / `1101f37f8` | Clean pushed same-tree alias after Account WP04 integration. |
| `E:/OcentraWorktrees/lanes/data-custody-wp05-source` | `codex/data-custody-wp05-source-repaired` / `f8d0a888a1` | Clean pushed ancestor; repaired WP05 is integrated in consolidation. |
| `E:/OcentraWorktrees/lanes/device-trust-runtime-routing` | `codex/device-trust-wp06-source-wave` / `1b35933194` | Clean pushed; WP06 patches are integrated and this branch is a cleanup candidate. |
| `E:/OcentraWorktrees/lanes/account-cloudflare-authority-routing` | `codex/child-runtime-source-routing` / `c71becbcfd` | Clean pushed, patch-equivalent source custody. |
| `E:/OcentraWorktrees/lanes/account-data-runtime-routing` | `codex/account-wp02-target-authority` / `f6ac50434d` | Clean pushed, patch-equivalent source custody. |
| `E:/OcentraWorktrees/lanes/account-wp02-authority-transport` | `codex/account-wp02-authority-transport` / `7282020448` | Clean pushed repair packet pending independent review. Not integration evidence or a cleanup candidate. |
| `E:/OcentraWorktrees/lanes/account-wp03-runtime-source` | `codex/account-wp03-runtime-source` / `59eefd0d23` | Clean pushed, patch-equivalent source custody. |
| `E:/OcentraWorktrees/lanes/account-wp03-source-wave` | `codex/account-wp03-source-wave` / `53b5d195d3` | Clean pushed, patch-equivalent source custody. |
| `E:/OcentraWorktrees/lanes/account-wp05-routing` | `codex/account-wp05-routing` / `d78758b213` | Clean pushed routing lane; local equals upstream. |
| `E:/OcentraWorktrees/lanes/account-wp05-source` | `codex/account-wp05-source` / `937009bc13` | Clean pushed reviewed source-custody lane; local equals upstream. |
| `E:/OcentraWorktrees/lanes/account-wp07-source` | `codex/account-wp07-source` / `2f3d3051aa` | Clean pushed reviewed source-custody lane; local equals upstream. |
| `E:/OcentraWorktrees/lanes/canonical-truth-refresh` | `codex/canonical-truth-refresh` / current document commit on top of `d7795c188` | Docs/graph review-repair lane; not canonical product-source integration. |
| `E:/OcentraWorktrees/lanes/cloudflare-wp06-authority-source` | `codex/cloudflare-wp06-runtime-source` / `8f50794297` | Clean pushed, patch-equivalent source custody. |
| `E:/OcentraWorktrees/lanes/data-custody-source-consolidation` | `codex/data-custody-source-consolidation` / `8da579cc70` | Clean pushed stale alternate; no direct integration. |
| `E:/OcentraWorktrees/lanes/data-custody-source-wave` | `codex/data-custody-source-wave` / `78f01911f1` | Clean pushed stale alternate; narrow salvage only. |
| `E:/OcentraWorktrees/lanes/data-custody-wp06-source` | `codex/data-custody-wp06-source` / `f5b839efbc` | Clean pushed, patch-equivalent source custody. |
| `E:/OcentraWorktrees/lanes/data-custody-wp08-source` | `codex/data-custody-wp08-source` / `1d63e190c5` | Clean pushed, patch-equivalent source custody. |
| `E:/OcentraWorktrees/lanes/data-wp05-authority-handoff` | `codex/data-wp05-authority-handoff` / `b3c47fd3df` | Clean pushed Account-authority handoff source custody; local equals upstream. |
| `E:/OcentraWorktrees/lanes/data-wp06-query-source-wave` | `codex/child-runtime-routing-refresh` / `1b6b5a28f6` | Clean pushed, patch-equivalent source custody. |
| `E:/OcentraWorktrees/lanes/data-wp08-p1-source-repair` | `codex/data-wp08-p1-source-repair` / `d77f8f649b` | Clean pushed, patch-equivalent source custody. |
| `E:/OcentraWorktrees/lanes/device-trust-wp01` | `codex/device-trust-wp01-source-wave` / `914d06b6aa` | Clean pushed stale branch; its two residual patches were semantically superseded. |
| `E:/OcentraWorktrees/lanes/eventing-wp08-parent-intent-ingress` | `codex/eventing-wp08-parent-intent-ingress` / `ba0854f0a9` | Clean pushed, patch-equivalent source custody. |
| `E:/OcentraWorktrees/lanes/eventing-wp11-typed-delivery` | `codex/eventing-wp11-typed-delivery` / `8fb261274c` | Clean pushed, patch-equivalent source custody. |
| `E:/OcentraWorktrees/lanes/logging-source-wave-repair` | `codex/logging-source-wave-repair` / `e0c2d158ab` | Clean pushed; production patches integrated, docs-only unique remainder. |
| `E:/OcentraWorktrees/lanes/payment-source-wave` | `codex/payment-source-wave` / `63305016fc` | Clean pushed, patch-equivalent source custody. |
| `E:/OcentraWorktrees/lanes/screen-wp26-source` | `codex/screen-wp26-source` / `43649e7b25` | Clean pushed reviewed Screen WP26 source-custody lane; local equals upstream. |
| `E:/OcentraWorktrees/lanes/screen-wp32-producer-source` | `codex/screen-wp32-producer-source` / `f992301ca9` | Clean pushed producer packet pending independent review; not mapped or accepted. |
| `E:/OcentraWorktrees/lanes/screen-wp32-source` | `codex/screen-wp32-source` / `b4c3a921b1` | Clean pushed same-tree alias for the integrated Screen WP32 source-only packet. |
| `E:/OcentraWorktrees/lanes/screen-wp33-source` | `codex/screen-wp33-source` / `0ec240f089` | Clean pushed reviewed Screen WP33 source-custody lane; local equals upstream. |
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

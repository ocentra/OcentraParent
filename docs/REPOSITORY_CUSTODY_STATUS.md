# Repository Custody Status

Snapshot: 2026-08-19 after canonical Cloudflare verifier source, Browser
manual-required source integration at `f80b47c6a`, and this custody/graph truth
refresh. This supersedes the retained 2026-08-18 snapshots below.

This document records where unpromoted Ocentra Parent work is physically and
remotely recoverable. It is not a completion, test, proof, CI, or release
claim. Refresh Git ancestry and patch identity before deleting any ref.

## Protected branch and integration truth

| Ref | Head | Custody state |
| --- | --- | --- |
| `origin/main` | `eb4e66a791` | Release baseline. It has no commit missing from consolidation and is 462 commits behind this snapshot. |
| `origin/develop` | `4ece515282` | Promotion baseline. It has no commit missing from consolidation and is 459 commits behind this snapshot. |
| `origin/codex/eventing-wp09-production` | this commit; parent `f80b47c6a` | Canonical source-consolidation line after Browser source integration and this truth refresh. Account, Device Trust, Data Custody, Cloudflare, Browser, Screen, and accepted Eventing source packets are retained here. Expected tests remain deliberately deferred until the source wave is complete. |
| `origin/production` | `683a07cf31` | Historical production ref; not the current integration line. |
| archive refs | `ac9f65bb4a`, `405e7fc77e` | Coverage for historical local/remote tips. Retain through source/test consolidation and promotion. |

Open pull requests at this snapshot: **0**. No source packet is allowed to skip
independent review, the later expected-test wave, focused validation,
precommit, or CI merely because it is pushed.

## Current remote branch disposition

There are 64 non-archive `origin/codex/*` refs including canonical, plus two
archive coverage refs:

- one canonical consolidation ref;
- 47 noncanonical refs with zero patch-unique commits against canonical;
- 16 noncanonical refs with residual patch-unique history. Some are active
  review/repair custody; others are reviewed net-integrated, rejected, or
  superseded. None may be merged by branch tip.

The 16 residual refs are:

| Remote branch | Current disposition |
| --- | --- |
| `codex/account-wp02-source-wave` | Six residual Account/payment authority commits still require narrow semantic review after the current Account dependencies. Do not merge the stale branch tip. |
| `codex/device-trust-wp01-source-wave` | Two narrow owner-bound entitlement/current-authority commits still require review; do not merge the broad stale branch tip. |
| `codex/account-wp05-cas-source` | Independently rejected as inert/wrong-owner CAS source: no reserve/prepare caller, weaker binding, ephemeral-storage bypass, and ambiguous recovery semantics. Retain only until disposition cleanup. |
| `codex/data-wp05-mount-contract-source` | Two commits are an add-then-revert pair; the reverted seam was caller-mintable and unsafe. Do not cherry-pick. |
| `codex/data-wp08-runtime-source` | Unique production source remains under review; do not merge before Account/Device authority and independent source acceptance. |
| `codex/data-custody-source-consolidation` | Superseded stale alternate; effect ledger was added and later removed. |
| `codex/data-custody-source-wave` | Superseded broad alternate; no reviewed missing production invariant remains. |
| `codex/data-custody-wp05-source` | Superseded old WP05 packet; repaired production source is already canonical. |
| `codex/account-wp02-wp05-source-wave` | Rejected/quarantined old identity lifecycle packet. |
| `codex/data-custody-plan-code-wave` | Obsolete child-runtime routing packet. |
| `codex/logging-source-wave-repair` | Only stale docs remain patch-unique; production source is canonical. |
| `codex/setup-wp07-source-wave` | Only stale docs remain patch-unique; production source is canonical. |
| `codex/canonical-truth-refresh` | Superseded docs-only custody/matrix snapshot; this document and regenerated graph replace it. |
| `codex/cloudflare-wp06-runtime-source-aug19` | Three patch-unique commits are net-integrated in canonical; remaining difference is formatting/history only. Do not replay the tip. |
| `codex/eventing-core-p1-aug19` | Active repair. The reviewed `8bec42487` head is rejected for raw-bus root dispatch, non-transactional descendant cancellation, and two causal callers that create new root buses. A superseding source repair is in progress. |
| `codex/screen-wp32-producer-source` | One residual producer patch requires reconciliation with Browser's now-manual-required, no-Browser-to-Screen source boundary before integration. |

The 47 zero-patch-unique cleanup candidates are:

`account-authority-producer-map`, `account-authority-producer-transport`,
`account-cloudflare-authority-routing`, `account-data-runtime-routing`,
`account-multi-owner-fence-route`, `account-wp02-authority-transport`,
`account-wp02-target-authority`, `account-wp03-runtime-source`,
`account-wp03-source-wave`, `account-wp04-source-wave`,
`account-wp05-cas-routing`, `account-wp05-routing`, `account-wp05-source`,
`account-wp07-source`, `browser-runtime-source-aug19`,
`browser-wp07-wp09-map`, `browser-wp07-wp09-route`,
`child-runtime-routing-refresh`, `child-runtime-source-routing`,
`cloudflare-wp05-source-completion`, `cloudflare-wp06-authority-source`,
`cloudflare-wp06-runtime-map`, `cloudflare-wp06-runtime-route`,
`cloudflare-wp06-runtime-source`, `data-custody-wp05-source-repaired`,
`data-custody-wp06-source`, `data-custody-wp08-source`,
`data-wp05-authority-handoff`, `data-wp05-runtime-composition-routing`,
`data-wp05-source-completion`, `data-wp06-map-aug18`,
`data-wp06-query-source-wave`, `data-wp06-routing-refresh`,
`data-wp06-source-completion`, `data-wp08-p1-source-repair`,
`device-trust-runtime-routing`, `device-trust-wp02-source-wave`,
`device-trust-wp05-source-wave`, `device-trust-wp06-source-wave`,
`eventing-consumer-truth-aug18`, `eventing-wp08-parent-intent-ingress`,
`eventing-wp11-typed-delivery`, `payment-source-wave`, `screen-wp26-source`,
`screen-wp32-source`, `screen-wp33-source`, and `source-map-refresh-aug18`.

Do not delete these refs yet. The user-required cleanup gate is canonical
promotion through `develop` to `main`, followed by fresh open-PR, local-only
commit, worktree-dirty-state, ancestry, patch-identity, and Enforcer-claim
checks.

## Prior patch-unique remote branch disposition at `1101f37f8`

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

## Current registered E-drive worktrees

There are 55 registered Ocentra Parent worktrees. At the pre-commit audit:

- the coordinator worktree contains only this claimed docs/graph refresh;
- `E:/OcentraWorktrees/lanes/eventing-core-p1-aug19` contains the active
  superseding Eventing source repair;
- `E:/OcentraParent` contains only the pre-existing untracked
  `.codex/config.toml`;
- every other registered worktree is clean;
- `codex/account-wp05-owner-fence` and
  `codex/cloudflare-wp06-producer-consumer` have no upstream, but both are
  clean ancestors of canonical with zero patch-unique commits, so neither
  contains unpublished work;
- the only ahead-of-upstream branch is this coordinator line before its final
  push; no other worktree contains a local-only commit;
- no registered Ocentra Parent worktree exists on `C:`.

The exact registered branch set is:

`develop`; `codex/account-authority-producer-map`;
`codex/account-authority-producer-transport`;
`codex/child-runtime-source-routing`; `codex/account-wp02-target-authority`;
`codex/account-multi-owner-fence-route`;
`codex/account-wp02-authority-transport`; `codex/account-wp04-source-wave`;
`codex/account-wp03-runtime-source`; `codex/account-wp03-source-wave`;
`codex/account-wp05-cas-routing`; `codex/account-wp05-cas-source`;
`codex/account-wp05-owner-fence`; `codex/account-wp05-routing`;
`codex/account-wp05-source`; `codex/account-wp07-source`;
`codex/browser-runtime-source-aug19`; `codex/browser-wp07-wp09-map`;
`codex/browser-wp07-wp09-route`; `codex/canonical-truth-refresh`;
`codex/cloudflare-wp05-source-completion`;
`codex/cloudflare-wp06-runtime-source`;
`codex/cloudflare-wp06-producer-consumer`;
`codex/cloudflare-wp06-runtime-map`; `codex/cloudflare-wp06-runtime-route`;
`codex/cloudflare-wp06-runtime-source-aug19`;
`codex/data-custody-source-consolidation`;
`codex/data-custody-source-wave`; `codex/data-custody-wp05-source-repaired`;
`codex/data-custody-wp06-source`; `codex/data-custody-wp08-source`;
`codex/data-wp05-authority-handoff`;
`codex/data-wp05-mount-contract-source`;
`codex/data-wp05-runtime-composition-routing`;
`codex/data-wp05-source-completion`; `codex/data-wp06-map-aug18`;
`codex/child-runtime-routing-refresh`; `codex/data-wp06-source-completion`;
`codex/data-wp08-p1-source-repair`; `codex/data-wp08-runtime-source`;
`codex/device-trust-wp06-source-wave`; `codex/device-trust-wp01-source-wave`;
`codex/eventing-consumer-truth-aug18`; `codex/eventing-core-p1-aug19`;
`codex/eventing-wp08-parent-intent-ingress`;
`codex/eventing-wp09-production`; `codex/eventing-wp11-typed-delivery`;
`codex/logging-source-wave-repair`; `codex/payment-source-wave`;
`codex/screen-wp26-source`; `codex/screen-wp32-producer-source`;
`codex/screen-wp32-source`; `codex/screen-wp33-source`;
`codex/setup-wp07-source-wave`; and `codex/source-map-refresh-aug18`.

## Prior registered-worktree snapshot at `2a50575d2`

There are 39 registered Ocentra Parent worktrees:

- 37 are clean;
- `E:/OcentraParent` has only the pre-existing untracked
  `.codex/config.toml`;
- `E:/OcentraWorktrees/lanes/account-wp05-cas-source` contains the active
  Account WP05 production-source packet and no local commit yet;
- `data-wp05-source-completion` and `eventing-wp11-source-completion` are
  newly opened clean source lanes based on canonical and are not yet pushed to
  branch-specific remotes;
- no worktree contains a forgotten local-only commit;
- no registered Ocentra Parent worktree exists on `C:\`.

The complete current worktree/branch set is:

`develop`;
`codex/child-runtime-source-routing`;
`codex/account-wp02-target-authority`;
`codex/account-wp02-authority-transport`;
`codex/account-wp04-source-wave`;
`codex/account-wp03-runtime-source`;
`codex/account-wp03-source-wave`;
`codex/account-wp05-cas-routing`;
`codex/account-wp05-cas-source`;
`codex/account-wp05-routing`;
`codex/account-wp05-source`;
`codex/account-wp07-source`;
`codex/canonical-truth-refresh`;
`codex/cloudflare-wp06-runtime-source`;
`codex/data-custody-source-consolidation`;
`codex/data-custody-source-wave`;
`codex/data-custody-wp05-source-repaired`;
`codex/data-custody-wp06-source`;
`codex/data-custody-wp08-source`;
`codex/data-wp05-authority-handoff`;
`codex/data-wp05-mount-contract-source`;
`codex/data-wp05-runtime-composition-routing`;
`codex/data-wp05-source-completion`;
`codex/child-runtime-routing-refresh`;
`codex/data-wp08-p1-source-repair`;
`codex/data-wp08-runtime-source`;
`codex/device-trust-wp06-source-wave`;
`codex/device-trust-wp01-source-wave`;
`codex/eventing-wp08-parent-intent-ingress`;
`codex/eventing-wp09-production`;
`codex/eventing-wp11-source-completion`;
`codex/eventing-wp11-typed-delivery`;
`codex/logging-source-wave-repair`;
`codex/payment-source-wave`;
`codex/screen-wp26-source`;
`codex/screen-wp32-producer-source`;
`codex/screen-wp32-source`;
`codex/screen-wp33-source`; and
`codex/setup-wp07-source-wave`.

## Prior registered-worktree snapshot at `1101f37f8`

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

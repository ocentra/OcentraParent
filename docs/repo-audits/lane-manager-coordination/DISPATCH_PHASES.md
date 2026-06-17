# Dispatch Phases

This is the lane-manager execution queue. Use it with `COORDINATOR_VERDICT_MATRIX.md` and `thread-instructions/INDEX.md`.

## Shared path locks

Before dispatching any implementation slice, lock these broad surfaces to one lane or prove disjoint paths:

| Surface | Why locked |
| --- | --- |
| `packages/parent-domain/**` | Many plans report stale shims and re-export wrappers here. |
| `packages/agent-protocol-domain/**` | Tracking, network, policy, app-game, LAN, remote, and AI all touch protocol seams. |
| `crates/agent-protocol/**` | Many Rust protocol tests still live inline and many plan constants overlap. |
| `crates/agent-core/**` | Eventing, network, screen, tracking, enforcement, and app/game all touch runtime helpers. |
| `crates/agent-service/**` | Network, screen, tracking, remote, LAN, and enforcement service paths overlap. |
| `apps/portal/**` and `packages/portal-domain/**` | Portal route truth is downstream of account, policy, LAN, screen, remote, and tracking. |
| `scripts/test/**` | Proof wrappers often point to stale owners and can corrupt plan closure claims. |

## Phase 0: global structural gate

Run or explicitly assign these before broad plan implementation.

| Step | Dispatch | Exit condition |
| ---: | --- | --- |
| 0.1 | lane-manager archive hygiene | Only canonical `*-selfaudit.md` files are active review inputs. |
| 0.2 | repo-audit WP01 | Real tests, empty scaffolds, inline tests, and move candidates inventoried. |
| 0.3 | repo-audit WP02 | Every crate/package/app mapped to local commands and CI jobs. |
| 0.4 | repo-audit WP07 | Orphaned, legacy, pre-eventing, and stale proof-wrapper surfaces inventoried. |
| 0.5 | repo-audit WP03 | Architecture policy state is explicit: clean now, staged cleanup, or exception list. |
| 0.6 | repo-audit WP04/WP05 | Ownership drift and DRY/common-core candidates mapped before refactor. |

## Phase 1: unblock local truth and proof foundations

These may run in parallel only if the path locks are disjoint.

| Order | Thread | Assign | Reason |
| ---: | --- | --- | --- |
| 1 | `tracking-plan` | `S0` closure precondition fix, then `S1` WP33 wrapper migration | `screen-plan` wants tracking truth first; tracking has no predecessor for S0/S1. |
| 2 | `data-custody-storage-plan` | substrate truth repair | Unblocks device-trust key/recovery, setup/custody rows, tracking custody consumers. |
| 3 | `lan-plan` | `B1` proof regeneration | Eventing, portal LAN consumer, and remote route wording should consume refreshed LAN proof. |
| 4 | `cloudflare-control-plane-plan` | `CFCP-C1` queue/dead-letter and negative-path hardening | Required before final payment handoff; local and high value. |
| 5 | `account-identity-family-plan` | WP02-WP05 proof reconciliation | Unblocks setup, portal household UX, remote grants, policy confirmation, payment/session claims. |
| 6 | `logging-domain-parity` | WP03 portal/dev-log closeout, then WP06 checker hardening | Stabilizes proof/debug surfaces before remote and later rollout proof. |
| 7 | `policy-control-plane-plan` | WP06 truth repair, then WP03 proof rebuild, then WP04 bundle | Unblocks portal policy UX and enforcement/policy route claims. |
| 8 | `setup-install-provisioning-plan` | WP06 truth-sync, then WP03 export-surface repair | Needed before distribution/setup handoff closure. |

## Phase 2: infrastructure and owner cleanup

Start after Phase 1 owners have reported their first outputs or blockers.

| Thread | Assign | Gate |
| --- | --- | --- |
| `device-trust-bootstrap-plan` | step-up/QR approval | Do not start key sealing until data-custody substrate is repaired. |
| `eventing-plan` | WP10-A typed household-mesh runtime + crate tests | Prefer after LAN `B1`; final proof must cite current LAN truth. |
| `network-plan` | foundation surface cleanup and proof-root bootstrap | Requires path lock for `packages/parent-domain/src/network*.ts`. |
| `browser-plan` | WP01 foundation cleanup | May run now if browser-domain paths are isolated. |
| `app-game-plan` | truth ownership cleanup | Requires path lock for app-game parent-domain/protocol surfaces. |
| `app-plan` | truth repair only | Do not widen runtime until app-game ownership is settled. |
| `ai-plan` | AI ownership and architecture cleanup | Requires path lock for AI parent-domain wrappers. |
| `payment-subscription-plan` | payment worker/domain proof alpha | Can run local slices; final closure waits Cloudflare handoff. |
| `child-agent-runtime-distribution-plan` | proof-root materializer and test category normalization | Do not close setup/device-trust handoff yet. |
| `parent-desktop-runtime-package-plan` | proof-root + parent web distribution | Setup handoff waits setup-plan producer proof. |

## Phase 3: overlay/UI and downstream product slices

| Thread | Assign | Gate |
| --- | --- | --- |
| `portal-ux-household-surfaces-plan` | start-route + LAN consumer truth | Prefer after LAN `B1`; broad WP02 waits account; policy IA waits policy local proof. |
| `screen-plan` | truth/proof-contract repair | Wait for tracking S0/S1; final closure waits screen-AI/AI/browser where claims remain. |
| `screen-ai-pipeline-plan` | proof/test normalization and architecture cleanup only | Do not run broad proof until screen-plan and AI-plan produce retained prerequisite artifacts. |
| `v0-8-enforcement-control-plan` | proof-router truth | First free duplicate writer claims; app-game readiness/preflight must be sequenced before final WP05/WP13. |
| `remote-access-plan` | hold, then RA-01 contract parity/test repair | Account identity first; LAN before protocol/relay; logging before rollout proof. |

## Phase 4: platform and proof regeneration

Run after the relevant local foundations exist.

| Thread | Platform/proof slice |
| --- | --- |
| `lan-plan` | Android/second-device/Linux/physical proof for rows 15/16/18/19/20. |
| `network-plan` | Windows, Android emulator/device, WSL/Linux proof gates. |
| `tracking-plan` | Android, WSL, portal service-backed proof for WP34-WP39. |
| `screen-plan` | Windows, Android, Linux proof regeneration. |
| `child-agent-runtime-distribution-plan` | Windows/Linux/Android package proof; Apple remains external. |
| `parent-desktop-runtime-package-plan` | Windows desktop, Linux update/rollback, Android parent proof; Apple remains external. |
| `remote-access-plan` | Windows/Android/Linux remote session/degraded/redaction proof after RA-03/RA-04. |

## Phase 5: final closure/handoff

| Thread | Final gate |
| --- | --- |
| `cloudflare-control-plane-plan` | deployment/promotion proof and WP12 payment handoff. |
| `payment-subscription-plan` | consume Cloudflare handoff, then provider/region/store proof and route closeout. |
| `setup-install-provisioning-plan` | final aggregate after account, distribution, LAN, device-trust, custody, policy. |
| `portal-ux-household-surfaces-plan` | final household UX closure after account, policy, LAN, setup, tracking/screen where referenced. |
| `screen-ai-pipeline-plan` | final product path only after screen and AI retained proof artifacts exist. |
| `v0-8-enforcement-control-plan` | final proof after app-game/browser/policy/portal/service proof surfaces exist. |
| `remote-access-plan` | final rollout after account/device/LAN/screen/logging proof exists. |

## Manager stop conditions

Stop a thread and require a new coordinator row if it:

- edits a locked broad surface without a path assignment;
- starts a downstream closure before its upstream artifact exists;
- adds a new broad re-export or barrel to avoid import work;
- counts `.gitkeep`, inline-only Rust tests, stale docs, or generated paths as completion;
- claims platform support without current Windows/Android/Linux/Apple feasibility notes.

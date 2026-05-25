# Current Main Proof Refresh - 2026-05-25

## Scope

This record reconciles the current `main` proof/status checkpoint after PR #90
through PR #96. It is a docs/evidence/status artifact only. It does not add
feature implementation, portal UI work, proof-matrix upgrades, CI/package
artifact evidence, V0.8 OS enforcement behavior, production LAN authentication,
package publishing, or mobile entitlement claims.

The purpose is roadmap-vs-done accuracy:

- V0.7 remains the current product acceptance gate.
- V0.8 and V0.9 proof spines are on `main`, but they are not product-complete.
- PR #96 is workflow/tooling only, not product feature progress.

The authoritative checkpoint runbook remains
`docs/architecture/cross-platform-deliverables-checkpoint.md`. The package and
CI evidence refresh work remains separate from this roadmap reconciliation
record.

## Run Metadata

| Field               | Value                                                                                  |
| ------------------- | -------------------------------------------------------------------------------------- |
| Proof date          | 2026-05-25                                                                             |
| Worktree            | `C:\Users\sujan\.codex\worktrees\ocentra-parent-codex-a\OcentraParent`                 |
| Branch              | `codex/v0.7-current-main-proof-reconciliation`                                         |
| Baseline commit     | `98eaf55b9b8507992cc076fe612e2194de8c90eb`                                             |
| Baseline subject    | `Make precommit gate fast and opt-in full validation (#96)`                            |
| Package/app version | `0.1.1`                                                                                |
| Sensitive data      | No child activity, screenshots, browser history, raw evidence payloads, or secrets.    |
| CI handling         | Not updated here; use the separate current-main CI/package evidence refresh if needed. |

## Source Inputs

- `docs/product-roadmap.md`
- `docs/architecture/cross-platform-deliverables-checkpoint.md`
- `docs/architecture/current-main-proof-refresh-2026-05-24.md`
- `docs/architecture/v0-7-checkpoint-validation-record.md`
- `docs/architecture/v07-cross-platform-proof-gap-tracker.md`
- `docs/architecture/validation-gates.md`
- `docs/expectations/pre-ai-proof-matrix.json`
- PR #90: <https://github.com/ocentra/OcentraParent/pull/90>
- PR #91: <https://github.com/ocentra/OcentraParent/pull/91>
- PR #92: <https://github.com/ocentra/OcentraParent/pull/92>
- PR #93: <https://github.com/ocentra/OcentraParent/pull/93>
- PR #94: <https://github.com/ocentra/OcentraParent/pull/94>
- PR #95: <https://github.com/ocentra/OcentraParent/pull/95>
- PR #96: <https://github.com/ocentra/OcentraParent/pull/96>

## Inspection Commands

| Command                                                                            | Result                                                                     | Proof label   |
| ---------------------------------------------------------------------------------- | -------------------------------------------------------------------------- | ------------- |
| `git fetch origin`                                                                 | Passed; latest `origin/main` fetched before branch creation.               | `implemented` |
| `git switch -c codex/v0.7-current-main-proof-reconciliation origin/main`           | Passed; branch created from `origin/main` at `98eaf55`.                    | `implemented` |
| `cmd /c npm run lanes:status`                                                      | Passed; `codex-a` claimed this branch and B/C stayed on separate lanes.    | `implemented` |
| `cmd /c npm run lanes:guard`                                                       | Passed before edits on `codex-a`.                                          | `implemented` |
| `cmd /c npm run hub:status`                                                        | Passed; latest codex-a message was acknowledged and STARTED was reported.  | `implemented` |
| `cmd /c npm run hub:guard`                                                         | Passed before edits on `codex-a`.                                          | `implemented` |
| `cmd /c npm run hub:lock -- --paths docs/product-roadmap.md,docs/architecture/...` | Passed; intended edit paths are this roadmap and proof-refresh record.     | `implemented` |
| `git log --oneline --first-parent c4e682b..98eaf55`                                | Passed; PR #90 through PR #96 merge order matched the history table below. | `implemented` |
| Seven `gh pr view <number> --json ...` calls for PR #90 through PR #96             | Passed; PR metadata is summarized below.                                   | `implemented` |

## PR90 Through PR96 Reconciliation

| PR  | Merge commit | Main history subject                                  | Roadmap meaning                                                                                                                                                                      |
| --- | ------------ | ----------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| #90 | `bb9b52e`    | `Prove LAN selected-device rejection contracts`       | V0.9 LAN spine proof for selected-route and unselected trusted-device rejection contracts/tests. It does not prove production LAN auth, portal selector UX, or two-device proof.     |
| #91 | `7cedf23`    | `Prove enforcement audit boundary consistency`        | V0.8 enforcement spine proof for audit events carrying unavailable/degraded adapter evidence. It does not prove OS blocking or production enforcement hardening.                     |
| #92 | `872482e`    | `Prove LAN restart registry intent selection`         | V0.9 LAN spine proof for restart-safe trusted registry and selected/unselected control routing in service tests. It does not prove real household LAN, firewall, or router state.    |
| #93 | `3282366`    | `Prove enforcement timer recovery identity`           | V0.8 enforcement spine proof for timer/recovery identity through contracts/protocol/core tests. It does not prove timer delivery, persistence, rollback UX, or OS enforcement.       |
| #94 | `91daf20`    | `Prove LAN discovery privacy surfaces`                | V0.9 LAN spine proof for typed discovery/challenge/status privacy surfaces. It does not prove production LAN authentication, cloud relay, portal selector UX, or router behavior.    |
| #95 | `f9ceb2a`    | `test: prove enforcement approval audit references`   | V0.8 enforcement spine proof for parent approval/override audit references across contract/protocol/core boundaries. It does not prove blocking, portal UI, or anti-tamper behavior. |
| #96 | `98eaf55`    | `Make precommit gate fast and opt-in full validation` | Workflow/tooling only. It changes local pre-commit ergonomics and documents explicit heavy gates; it is not V0.7, V0.8, or V0.9 product feature progress.                            |

## Current Roadmap Position

The current product position is still the V0.7 checkpoint proof gate. The
required pre-AI evidence bridges and V0.7 dry-run policy-preview foundations are
on `main`, but acceptance still depends on the V0.7 proof gate and manual
platform evidence called out in the cross-platform checkpoint runbook.

V0.8 has useful scaffold-real enforcement spine coverage on `main`: typed
contracts, Rust protocol/core parity, unavailable/capability status,
audit-boundary behavior, timer/recovery identity, and parent approval/override
audit references. This is not product-complete enforcement. Do not claim real
blocking, OS adapters, anti-tamper, timer delivery, rollback UX, notification
delivery, or production enforcement hardening from these proof spines.

V0.9 has useful scaffold-real LAN spine coverage on `main`: selected-route and
unselected-device rejection contracts, restart-safe trusted registry intent
selection, typed discovery/challenge/status privacy surfaces, and audit evidence
references. This is not product-complete LAN pairing. Do not claim production
LAN authentication, trusted cross-device control, router/firewall behavior,
cloud relay, portal selector UX, or a full paired/unpaired household flow from
these proof spines.

PR #96 should be treated as validation workflow/tooling. It makes local commits
less expensive by keeping pre-commit to a fast source gate and preserving heavy
validation as explicit local commands and CI responsibilities. Proof records
must distinguish the fast local commit gate from full validation, PR CI,
package-preview evidence, and manual OS/device proof.

## Proof Matrix Handling

Do not update `docs/expectations/pre-ai-proof-matrix.json` from this record. PR
#90 through PR #95 add test-backed V0.8/V0.9 proof spines, and PR #96 changes
workflow/tooling. None of them adds the missing real manual platform evidence
that would justify upgrading proof-matrix rows for Windows privileged behavior,
two-device LAN, package lifecycle, signing, stores, mobile device policy, iOS
entitlements, local AI model execution, or enforcement adapters.

## Explicit Omissions

- No B-owned CI/package evidence file was edited.
- No portal, vendor, C-owned, runtime, service, package script, or workflow file
  was edited in this branch.
- No proof-matrix row was upgraded.
- No package-preview artifact was downloaded or installed locally.
- No local Windows MSI install, elevated service check, reboot, autostart,
  update, uninstall, or data-retention proof was run.
- No macOS host, Android physical device, iOS device, TestFlight, store,
  signing, notarization, or entitlement proof was run.
- No two-device LAN pairing, firewall/router behavior, paired request, failed
  unpaired request, cloud relay, or production auth proof was run.
- No local AI model execution, OS enforcement adapter, real blocking, timer
  delivery, anti-tamper, notification delivery, or rollback UX proof was run.

## Known Gaps And Risks

- The roadmap now names the PR #90 through PR #95 proof spines, but those spines
  remain contract/protocol/core/service proof and do not complete V0.8 or V0.9.
- PR #96 changed local pre-commit expectations; future proof records must name
  whether they ran the fast pre-commit gate, `precommit:full`, `test:local`,
  `validate`, CI, or manual platform proof.
- Current V0.7 acceptance still needs the checkpoint validation gate plus the
  manual Windows, LAN, package lifecycle, Linux, macOS, Android, and iOS proof
  called out in the checkpoint runbook.
- CI/package-preview evidence should stay in the separate evidence refresh lane
  so this reconciliation does not duplicate or conflict with B's proof file.

## Roadmap Slice

V0.7 current-main proof reconciliation after PR #96. This record keeps roadmap
language aligned with `main` at `98eaf55` by separating V0.7 acceptance status,
V0.8/V0.9 proof spines, and PR #96 validation tooling.

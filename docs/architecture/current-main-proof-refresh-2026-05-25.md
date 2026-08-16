<!-- agent-capsule -->

> Agent Capsule
> Doc: Current Main Proof Refresh - 2026-05-25
> Kind: architecture/reference documentation; read only when selected by plan route, source router, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

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

The companion acceptance checklist is
`docs/architecture/v0-7-current-main-acceptance-record-2026-05-25.md`. Read
that record with this one when deciding whether current `main` is accepted or
whether larger V0.8/V0.9 implementation can resume.

## Run Metadata

| Field               | Value                                                                                  |
| ------------------- | -------------------------------------------------------------------------------------- |
| Proof date          | 2026-05-25                                                                             |
| Worktree            | `C:\Users\sujan\.codex\worktrees\ocentra-parent-codex-a\OcentraParent`                 |
| Branch              | `codex/v0.7-current-main-proof-reconciliation`                                         |
| Baseline commit     | `98eaf55b9b8507992cc076fe612e2194de8c90eb`                                             |
| Baseline subject    | `Make precommit gate fast and opt-in full validation (#96)`                            |
| Starting local edit | `ad501a4 docs: reconcile V0.7 current-main proof`                                      |
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
- `docs/architecture/v0-7-current-main-acceptance-record-2026-05-25.md`
- `docs/expectations/pre-ai-proof-matrix.json`
- PR #90: <https://github.com/ocentra/OcentraParent/pull/90>
- PR #91: <https://github.com/ocentra/OcentraParent/pull/91>
- PR #92: <https://github.com/ocentra/OcentraParent/pull/92>
- PR #93: <https://github.com/ocentra/OcentraParent/pull/93>
- PR #94: <https://github.com/ocentra/OcentraParent/pull/94>
- PR #95: <https://github.com/ocentra/OcentraParent/pull/95>
- PR #96: <https://github.com/ocentra/OcentraParent/pull/96>

## Inspection Commands

| Command                                                                            | Result                                                                                 | Proof label   |
| ---------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------- | ------------- |
| `git fetch origin`                                                                 | Passed; latest `origin/main` fetched before branch creation.                           | `implemented` |
| `git switch -c codex/v0.7-current-main-proof-reconciliation origin/main`           | Passed; branch created from `origin/main` at `98eaf55`.                                | `implemented` |
| `cmd /c npm run lanes:status`                                                      | Passed; `codex-a` claimed this branch and B/C stayed on separate lanes.                | `implemented` |
| `cmd /c npm run lanes:guard`                                                       | Passed before edits on `codex-a`.                                                      | `implemented` |
| `cmd /c npm run hub:status`                                                        | Passed; latest codex-a message was acknowledged and STARTED was reported.              | `implemented` |
| `cmd /c npm run hub:guard`                                                         | Passed before edits on `codex-a`.                                                      | `implemented` |
| `cmd /c npm run hub:lock -- --paths docs/product-roadmap.md,docs/architecture/...` | Passed; intended edit paths are this roadmap and proof-refresh record.                 | `implemented` |
| `git log --oneline --first-parent c4e682b..98eaf55`                                | Passed; PR #90 through PR #96 merge order matched the history table below.             | `implemented` |
| Seven `gh pr view <number> --json ...` calls for PR #90 through PR #96             | Passed; PR metadata is summarized below.                                               | `implemented` |
| `cmd /c npm run hub:ack` for `codex-a-msg-20260525T183406410Z-155`                 | Passed; scope correction was acknowledged before expanding this package.               | `implemented` |
| `cmd /c npm run hub:lock -- --paths docs/product-roadmap.md,...acceptance...`      | Passed; expanded lock covered the roadmap, refresh record, and acceptance record only. | `implemented` |

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

### Completed On Main

The completed-on-main record now includes the scaffold and proof slices that can
be reviewed from repository history:

- V0.1 through V0.5 scaffold, local evidence store, Windows process/window and
  network/domain foundations, live portal visibility, browser/app/game/network
  flow/screen evidence contracts and read paths, and package-preview scaffolds.
- V0.6 and V0.7 dry-run local AI policy evaluator foundations, including typed
  context-builder inputs, deterministic dry-run decisions, local runtime status,
  provider probe/unavailable status, parent-rule context preview, and
  evidence-cited policy-preview service/API and portal read-model paths.
- PR #90 through PR #95 proof spines for V0.8/V0.9 contract, protocol, core, and
  service behavior.
- PR #96 validation workflow/tooling changes that make pre-commit fast while
  keeping full validation and package evidence as explicit gates.

Completed on `main` does not mean product-accepted. It means the code or proof
record is merged and reviewable. Acceptance still requires the proof labels
below to be reconciled.

### CI-Mechanical Only

These areas can be supported by CI, proof-matrix checks, contract tests,
real-service smoke tests, and package-preview jobs, but they are not the same as
real OS/device proof:

- shared TypeScript/Rust contracts, schema-boundary checks, Rust checks/tests,
  local service mechanics, and portal-to-Rust transport;
- pre-AI proof matrix validation for already-recorded claims;
- package-preview build/install/launch smoke where hosted runners support it;
- V0.8 enforcement proof spines that prove typed audit/timer/approval reference
  behavior without real OS blocking;
- V0.9 LAN proof spines that prove typed selected-route, registry, discovery,
  challenge/status, privacy-surface, and audit behavior without real household
  pairing.

### Manual-Required Or Not-Yet-Proven

These remain outside the proof produced by PR #90 through PR #96:

- current Windows child-device evidence-preview proof for managed browser,
  foreground app/window, network/domain, app/game duration, and screen queue
  states;
- real parent-to-child LAN proof with one paired request and one failed
  unpaired request across two devices;
- real package lifecycle proof: install, service manager/autostart, reboot,
  update, uninstall, and data-retention behavior;
- Linux WSL/Docker, macOS host, Android physical device, and iOS
  TestFlight/device/entitlement checks;
- V0.8 OS enforcement adapters, real blocking, timer delivery, rollback UX,
  anti-tamper, and notification delivery;
- V0.9 production LAN authentication, router/firewall behavior, portal selector
  UX, cloud relay, and trusted cross-device product control.

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

## Acceptance Gate Before Larger V0.8/V0.9 Work

Larger V0.8/V0.9 implementation should not resume until primary can review the
acceptance package and either mark every item below complete or record an
explicit deferral with owner and reason:

1. Current `main` commit and branch state are recorded and clean.
2. Local focused validation passes: formatting, pre-AI proof, whitespace check,
   lane guard, and hub guard.
3. Full local validation or a deliberate omission note exists for the current
   branch.
4. Current main CI/package-preview evidence is reviewed in the separate
   checkpoint evidence record owned outside this branch.
5. Windows real child-device proof covers the current evidence-preview flow or
   records exact unavailable/degraded states.
6. LAN proof records one paired request and one failed unpaired request through
   real product paths or records why it remains manual-required.
7. Package lifecycle proof records install, autostart/service-manager, reboot,
   update, uninstall, and data-retention behavior where artifacts exist.
8. Linux, macOS, Android, and iOS rows carry current CI, manual, scaffold-only,
   blocked, permission-required, or not-yet-proven labels.
9. The proof matrix and roadmap are updated only for rows backed by concrete
   evidence.
10. V0.8/V0.9 resume scope is split so enforcement and LAN work continues from
    proof-backed boundaries without claiming product completion.

## Branch Validation

This branch ran both focused proof checks and the broader root validation gate
after the expanded acceptance package was drafted:

| Command                            | Result                                                                                                                                                                                                    |
| ---------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `cmd /c npm run format:check`      | Passed; all matched files use Prettier style.                                                                                                                                                             |
| `cmd /c npm run test:pre-ai-proof` | Passed; `11 claims checked across 5 platforms; 7 checkpoint scenarios checked.`                                                                                                                           |
| `cmd /c npm run test:tooling`      | Passed; `79` tooling tests passed.                                                                                                                                                                        |
| `git diff --check`                 | Passed; no whitespace errors.                                                                                                                                                                             |
| `cmd /c npm run lanes:guard`       | Passed for `codex-a` on `codex/v0.7-current-main-proof-reconciliation`.                                                                                                                                   |
| `cmd /c npm run hub:guard`         | Passed with locks limited to the roadmap, current-main refresh, and acceptance record.                                                                                                                    |
| `cmd /c npm run validate`          | Passed; release version, pre-AI proof, schema/source/test-double guards, Turbo lint/type-check/test, Rust validation, local/LAN WebSocket smoke, portal local smoke, and portal Playwright E2E completed. |

`npm run validate` printed existing source-shape advisory warnings and the
existing Vite large-chunk warning; both gates still passed. These warnings do
not upgrade any manual platform proof claim.

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

# App + Game Implementation Checklist

Use `[ ]` for not started, `[~]` for in progress, and `[x]` only after the
required proof pack exists.

## Fill Rules

- Every workpack must name owner/lane, branch/PR/commit, evidence or proof, and
  doc/checklist decision before `DONE`.
- Product docs and checklist rows move only when implementation changes status,
  proof, or gap.
- Browser games remain in browser-plan.
- Native apps and native games use one evidence spine and separate product
  slices.

## Required Proof Pack

Each workpack proof root:

```text
output/app-game-plan-proof/<workpack-id>/
```

Required files:

- [ ] `00-source-snapshot.md`: branch, commit, `git status --short`, current
      lock state, and source files inspected.
- [ ] `01-contract-proof.log`: TypeScript contract tests, decode failures, and
      no-claim negative cases.
- [ ] `02-rust-protocol-proof.log`: Rust protocol parity, serialization, and
      field/enum proof.
- [ ] `03-runtime-evidence.json`: inventory, process, foreground, launcher,
      session, platform, and capability fixture outputs where applicable.
- [ ] `04-journal-sqlite-proof.json`: journal entry refs, replay result, and
      read-model rows where applicable.
- [ ] `05-policy-action-proof.json`: policy input, compiled target, authority
      tier, dry-run/action result, and manual-required state.
- [ ] `06-ui-snapshots/`: parent portal and child-facing screenshots for every
      touched state, or explicit N/A.
- [ ] `07-playwright-ui-proof.log`: Playwright/browser output for UI changes.
- [ ] `08-security-negative-proof.log`: inventory-is-not-use,
      runtime-is-not-foreground, foreground-is-not-content, launcher-is-not-game,
      AI-cannot-enforce, manual-required-cannot-execute, and metadata safety
      proof.
- [ ] `09-manual-platform-proof.md`: OS/device versions, authority tier, setup,
      screenshots/logs, limitations, and follow-up.
- [ ] `10-validation-commands.log`: focused validation plus requested lane/hub
      guards.
- [ ] `11-authority-tier-proof.md`: platform mode, permission/enrollment setup,
      capability state, proof needed to move up.
- [ ] `12-rollback-proof.md`: rollback, cleanup, unblock, unsuspend, unshield,
      and safe-failure proof.

## Base Workpack Checklist

| Step | Workpack                                                                                                             | Status | Owner/Lane | Branch/PR/Commit                           | Evidence Or Proof                                                      | Doc/Checklist Decision                                                        |
| ---- | -------------------------------------------------------------------------------------------------------------------- | ------ | ---------- | ------------------------------------------ | ---------------------------------------------------------------------- | ----------------------------------------------------------------------------- |
| 01   | [Contract boundary and Effect schemas](workpacks/01-contract-boundary-and-effect-schemas.md)                         | [x]    | codex-c    | codex/app-plan-work                        | output/app-game-plan-proof/01-contract-boundary-and-effect-schemas     | Plan checklist only; feature/checklist status unchanged until runtime proof   |
| 02   | [Source index and doc reconciliation](workpacks/02-source-index-and-doc-reconciliation.md)                           | [x]    | codex-c    | codex/app-game-doc-reconciliation          | output/app-game-plan-proof/02-source-index-and-doc-reconciliation      | Routing/docs only; product checklist unchanged because no status moved        |
| 03   | [Current app/game snapshot and gap map](workpacks/03-current-app-game-snapshot-and-gap-map.md)                       | [x]    | codex-c    | codex/app-game-doc-reconciliation          | output/app-game-plan-proof/03-current-app-game-snapshot-and-gap-map    | Snapshot/gap proof only; runtime status unchanged                             |
| 04   | [App/game identity model](workpacks/04-app-game-identity-model.md)                                                   | [x]    | codex-c    | codex/app-game-identity-contracts          | output/app-game-plan-proof/04-app-game-identity-model                  | Contract proof only; runtime identity merge and product status unchanged      |
| 05   | [Inventory evidence model](workpacks/05-inventory-evidence-model.md)                                                 | [x]    | codex-c    | codex/app-game-inventory-evidence-model    | output/app-game-plan-proof/05-inventory-evidence-model                 | Contract proof only; platform inventory adapters and product status unchanged |
| 06   | [Windows installed app/game inventory adapter](workpacks/06-windows-installed-inventory-adapter.md)                  | [x]    | codex-c    | codex/app-game-windows-installed-inventory | output/app-game-plan-proof/06-windows-installed-inventory-adapter      | Adapter/parser proof only; live source crawling and product status unchanged  |
| 07   | [Windows Store/UWP/AppX/MSIX inventory adapter](workpacks/07-windows-store-uwp-appx-inventory-adapter.md)            | [x]    | codex-c    | codex/app-game-windows-store-inventory     | output/app-game-plan-proof/07-windows-store-uwp-appx-inventory-adapter | Parser proof only; live package enumeration and product status unchanged      |
| 08   | [Windows process runtime evidence adapter](workpacks/08-windows-process-runtime-evidence-adapter.md)                 | [x]    | codex-c    | codex/app-game-windows-process-runtime     | output/app-game-plan-proof/08-windows-process-runtime-evidence-adapter | Runtime contract/protocol/parser proof only; live capture/status unchanged    |
| 09   | [Windows foreground app/game evidence adapter](workpacks/09-windows-foreground-evidence-adapter.md)                  | [x]    | codex-c    | codex/app-game-windows-foreground-evidence | output/app-game-plan-proof/09-windows-foreground-evidence-adapter      | Foreground contract/protocol/parser proof only; live capture/status unchanged |
| 10   | [Launcher evidence and game candidate model](workpacks/10-launcher-evidence-and-game-candidate-model.md)             | [ ]    |            |                                            |                                                                        |                                                                               |
| 11   | [Cross-platform authority matrix](workpacks/11-cross-platform-authority-matrix.md)                                   | [ ]    |            |                                            |                                                                        |                                                                               |
| 12   | [App and game category/risk taxonomy](workpacks/12-app-game-category-and-risk-taxonomy.md)                           | [ ]    |            |                                            |                                                                        |                                                                               |
| 13   | [Sessionization and duration engine](workpacks/13-sessionization-and-duration-engine.md)                             | [ ]    |            |                                            |                                                                        |                                                                               |
| 14   | [Journal and SQLite ingest](workpacks/14-journal-and-sqlite-ingest.md)                                               | [ ]    |            |                                            |                                                                        |                                                                               |
| 15   | [Read models and service events](workpacks/15-read-models-and-service-events.md)                                     | [ ]    |            |                                            |                                                                        |                                                                               |
| 16   | [Parent portal app/game dashboard surfaces](workpacks/16-parent-portal-app-game-dashboard-surfaces.md)               | [ ]    |            |                                            |                                                                        |                                                                               |
| 17   | [Unknown app/game approval flow](workpacks/17-unknown-app-game-approval-flow.md)                                     | [ ]    |            |                                            |                                                                        |                                                                               |
| 18   | [Native game budgets and launcher policy](workpacks/18-native-game-budgets-and-launcher-policy.md)                   | [ ]    |            |                                            |                                                                        |                                                                               |
| 19   | [Policy target compiler for app/game rules](workpacks/19-policy-target-compiler-for-app-game-rules.md)               | [ ]    |            |                                            |                                                                        |                                                                               |
| 20   | [Time budget, schedule, and bonus-time integration](workpacks/20-time-budget-schedule-bonus-time-integration.md)     | [ ]    |            |                                            |                                                                        |                                                                               |
| 21   | [Child-facing app/game warning and request UX](workpacks/21-child-facing-warning-and-request-ux.md)                  | [ ]    |            |                                            |                                                                        |                                                                               |
| 22   | [Windows owned-process terminate time-limit proof](workpacks/22-windows-owned-process-terminate-time-limit-proof.md) | [ ]    |            |                                            |                                                                        |                                                                               |
| 23   | [Broad blocking proof gates](workpacks/23-broad-blocking-proof-gates.md)                                             | [ ]    |            |                                            |                                                                        |                                                                               |
| 24   | [AI classifier digest boundary](workpacks/24-ai-classifier-digest-boundary.md)                                       | [ ]    |            |                                            |                                                                        |                                                                               |
| 25   | [Platform extension checklist and proof routing](workpacks/25-platform-extension-checklist-and-proof-routing.md)     | [ ]    |            |                                            |                                                                        |                                                                               |
| 26   | [Install, uninstall, purchase, and store handoffs](workpacks/26-install-uninstall-purchase-store-handoffs.md)        | [ ]    |            |                                            |                                                                        |                                                                               |
| 27   | [Performance and service health](workpacks/27-performance-and-service-health.md)                                     | [ ]    |            |                                            |                                                                        |                                                                               |
| 28   | [E2E, manual proof, rollout, and PR gate](workpacks/28-e2e-manual-proof-rollout-pr-gate.md)                          | [ ]    |            |                                            |                                                                        |                                                                               |

## Merge-Blocking Failure Gates

- [ ] Inventory evidence is displayed as app/game usage.
- [ ] Running evidence is displayed as foreground usage.
- [ ] Foreground evidence is displayed as content knowledge.
- [ ] Launcher evidence is displayed as active game without child-game proof.
- [ ] Unknown process is auto-promoted to known game.
- [ ] AI output directly enforces.
- [ ] Dry-run terminates or blocks app/game.
- [ ] Manual-required action calls an adapter.
- [ ] Android normal mode claims package suspend/hide.
- [ ] iOS claims process scanning/killing.
- [ ] macOS hard block is claimed without MDM/Endpoint/System Extension proof.
- [ ] Linux universal block is claimed without mechanism/distro proof.
- [ ] Session duration changes after journal replay.
- [ ] Portal hides stale, permission-limited, manual-required, or not-claimed
      states.
- [ ] Raw private executable paths leak into parent UI.
- [ ] Malicious app/game metadata causes XSS or layout breakage.

## Platform Extension Checklist

Keep platform items as extension checklist items unless explicitly promoted by a
hub/user assignment.

| Item                                                                                                                                                                                       | Status | Evidence Or Proof |
| ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ------ | ----------------- |
| macOS bundle inventory, LaunchServices, NSWorkspace, Accessibility, code signature, LaunchAgent/LaunchDaemon, PPPC, MDM, Endpoint Security, terminate/quit, hard-block gate                | [ ]    |                   |
| iOS FamilyControls, FamilyActivityPicker, DeviceActivity, ManagedSettings, shield UI, token identity, MDM, supervised restrictions, App Lock, store/signing/entitlement proof              | [ ]    |                   |
| Android package visibility, UsageStats, UsageEvents, Accessibility overlay, Device Owner/Profile Owner, hide/suspend/uninstall-block, lock task, managed configurations, Play policy proof | [ ]    |                   |
| Linux desktop entries, package managers, Flatpak, Snap, AppImage, procfs, cgroups/systemd, X11/Wayland, terminate, AppArmor/SELinux, package/Flatpak/Snap restriction proof                | [ ]    |                   |

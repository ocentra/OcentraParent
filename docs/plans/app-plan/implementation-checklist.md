# Native Apps Implementation Checklist

This is the fill-in checklist for native app implementation work. Future AI
workers must update this file and the matching workpack checklist before
reporting `DONE` or PR-ready.

This checklist tracks app-plan execution only. It does not replace
`docs/product-capability-checklist.md`, and workers must not edit that product
checklist unless a feature row status, proof, or gap actually changes and the
worker holds the correct hub lock.

## Fill Rules

- Keep unchecked items unchecked until code, docs, tests, and proof are present.
- Record lane, branch, PR, commit, or proof path in the notes column when an
  item moves.
- If an item is intentionally deferred, leave it unchecked and write the
  manual-required reason.
- Do not use this file to claim production readiness without proof artifacts.
- Fill the matching `## AI Worker Checklist` inside the workpack file before
  reporting `DONE`.
- Report product-doc updates, or explicitly state why no product-doc update was
  needed.

## Required Proof Pack

Every implementation workpack needs a proof pack before the main workpack row
can be marked complete. Use this root unless the assignment names a stricter
location:

```text
output/app-plan-proof/<workpack-id>/
```

The proof pack must contain or explicitly mark N/A for each applicable item:

- [ ] `00-source-snapshot.md`: git branch, commit, `git status --short`,
      existing source paths inspected, existing behavior, and the before-state gap.
- [ ] `01-contract-proof.log`: TypeScript contract tests, decode failures, and
      schema-boundary/source-shape checks for new or changed contracts.
- [ ] `02-rust-protocol-proof.log`: Rust protocol parity, serialization, and
      invalid-state tests when protocol/service shapes change.
- [ ] `03-runtime-evidence.json`: app inventory, process, foreground, platform,
      authority-tier, or adapter evidence for the workpack.
- [ ] `04-journal-sqlite-proof.json`: journal entry refs, replay result, and
      SQLite/read-model rows when evidence persistence changes.
- [ ] `05-policy-action-proof.json`: policy input, compiled target, authority
      tier, decision, action result, evidence refs, and degraded/manual-required
      labels when policy or enforcement changes.
- [ ] `06-ui-snapshots/`: parent portal and child-facing screenshots for every
      UI-visible state touched by the workpack.
- [ ] `07-playwright-ui-proof.log`: Playwright/browser test output for changed
      portal or child UI, including malicious text escaping and responsive state
      where applicable.
- [ ] `08-security-negative-proof.log`: negative tests proving no content
      capture, no AI direct enforcement, no broad blocking in manual-required
      states, no raw private path/command-line leak, and no platform overclaim.
- [ ] `09-manual-platform-proof.md`: OS/device versions, authority tier, setup,
      commands/UI steps, screenshots/logs, and manual-required labels for real
      platform claims.
- [ ] `10-validation-commands.log`: focused validation plus any requested
      `npm run validate`/`ci:local`/manual command output.

For platform workpacks, add:

- [ ] `11-authority-tier-proof.md`: authority tier, platform mode, enrollment
      state, permission state, and exact capability status.
- [ ] `12-permission-setup-proof.md`: setup commands, OS/device UI steps,
      screenshots/logs, and degraded/manual-required fallback.
- [ ] `13-rollback-proof.md`: rollback, cleanup, unblock, unsuspend, unshield,
      allowlist restore, or policy removal proof.

## UI Snapshot Gates

When a workpack touches portal, child-facing UI, policy authoring, read models,
dashboards, or status surfaces, workers must capture screenshots before marking
the workpack complete.

- [ ] Parent portal snapshot for normal/supported state.
- [ ] Parent portal snapshot for stale/degraded/manual-required state.
- [ ] Parent portal snapshot for unknown/new app state when in scope.
- [ ] Parent portal snapshot for risk app state when in scope.
- [ ] Policy authoring/preview snapshot when rules, catalog, or compiler UI is
      in scope.
- [ ] Child warning/block/approval snapshot when child UX is in scope.
- [ ] Responsive/narrow viewport snapshot where required.
- [ ] Malicious/long text snapshot for app names, publishers, paths, titles,
      category labels, or AI labels.
- [ ] Explicit `ui-not-applicable.md` when the workpack has no UI surface.

## Evidence Quality Gates

- [ ] Raw fixture/evidence is stored with redacted sensitive values, not just a
      prose summary.
- [ ] Every action proof includes evidence refs and policy decision refs.
- [ ] Every stale/degraded/manual-required state is represented in contracts,
      runtime/read model, and UI where applicable.
- [ ] Every platform limitation is represented as observe-only,
      permission-required, managed-device-required, admin/root-required,
      system-extension-required, supervised-device-required, manual-required, or
      not-claimed until real platform proof exists.
- [ ] Every authority-tier claim names setup, permission, adapter, and proof.
- [ ] Every failed, skipped, manual, or deferred test has a reason and follow-up
      owner recorded.

## Minimum Serious MVP Test Set

Do not mark the native app plan or an implementation slice product-complete
unless the relevant parts of this suite exist or are explicitly N/A with a
reason.

- [ ] Unit: app identity.
- [ ] Unit: inventory evidence.
- [ ] Unit: runtime evidence.
- [ ] Unit: foreground evidence.
- [ ] Unit: session model.
- [ ] Unit: category taxonomy.
- [ ] Unit: policy target compiler.
- [ ] Unit: authority tier.
- [ ] Unit: AI no-direct-enforcement.
- [ ] Integration: Windows inventory fixtures.
- [ ] Integration: Windows process fixtures.
- [ ] Integration: Windows foreground fixtures.
- [ ] Integration: identity merge.
- [ ] Integration: sessionization.
- [ ] Integration: journal/SQLite replay.
- [ ] Integration: policy dry-run.
- [ ] Integration: unknown app approval.
- [ ] Integration: risk app detection.
- [ ] Contract: `NativeAppIdentity`.
- [ ] Contract: `NativeAppInventoryEvidence`.
- [ ] Contract: `NativeAppRuntimeEvidence`.
- [ ] Contract: `NativeAppSessionSummary`.
- [ ] Contract: `NativeAppPolicyDecision`.
- [ ] Contract: `NativeAppEnforcementResult`.
- [ ] Contract: `NewAppApprovalRequest`.
- [ ] Contract: `AppCapabilityStatus`.
- [ ] Security: weak evidence no-upgrade.
- [ ] Security: manual-required guard.
- [ ] Security: platform authority guard.
- [ ] Security: path redaction.
- [ ] Security: malicious metadata escaping.
- [ ] Security: stale evidence rejection.
- [ ] E2E: Windows app inventory to portal.
- [ ] E2E: Windows runtime session.
- [ ] E2E: foreground duration.
- [ ] E2E: unknown app approval.
- [ ] E2E: risk app detection.
- [ ] E2E: time-limit dry-run.
- [ ] E2E: owned-process enforcement where already scoped.
- [ ] E2E: broad block manual-required.
- [ ] Playwright: app dashboard.
- [ ] Playwright: inventory details.
- [ ] Playwright: running/foreground states.
- [ ] Playwright: evidence drawer.
- [ ] Playwright: unknown approval.
- [ ] Playwright: risk categories.
- [ ] Playwright: policy authoring.
- [ ] Playwright: platform matrix.
- [ ] Playwright: manual-required labels.

## Fixture And Manual Test Gates

- [ ] App inventory fixtures include Windows registry, Windows Start Menu,
      Windows Store packages, macOS app bundles, Linux desktop entries, Linux
      Flatpak apps, Android packages, and iOS activity tokens.
- [ ] Runtime fixtures include Windows process snapshots, Windows process
      start/exit, Windows foreground windows, macOS NSWorkspace running apps,
      Linux procfs snapshots, Android usage events, and iOS DeviceActivity
      events.
- [ ] Session fixtures cover foreground, background-only, process exit, stale
      gap, replayed, and unknown process sessions.
- [ ] Policy fixtures cover observe, warn, ask unknown, time limit,
      block-vpn-manual-required, Android Device Owner required, iOS
      ManagedSettings required, and macOS Endpoint Security required states.
- [ ] UI fixtures cover empty dashboard, mixed dashboard, unknown approval, VPN
      risk, manual-required, platform matrix, and malicious metadata states.
- [ ] Manual tests are tagged with every applicable requirement:
      `@manual`, `@requires-windows`, `@requires-macos`, `@requires-linux`,
      `@requires-android-device-owner`, `@requires-ios-familycontrols`,
      `@requires-mdm`, `@requires-endpoint-security`, `@requires-applocker`,
      and `@requires-app-control`.
- [ ] No ignored or manual test is left without reason.

## Merge-Blocking Failure Gates

Block `DONE`, PR-ready, or merge if any of these are true:

- [ ] Inventory evidence is displayed as app usage.
- [ ] Running evidence is displayed as foreground usage.
- [ ] Foreground evidence is displayed as content knowledge.
- [ ] AI output can directly enforce.
- [ ] Dry-run terminates or blocks app.
- [ ] Manual-required action calls an adapter.
- [ ] Android normal mode claims package suspend/hide.
- [ ] iOS claims process scanning/killing.
- [ ] macOS hard block is claimed without entitlement/profile proof.
- [ ] Linux universal block is claimed without mechanism/distro proof.
- [ ] Session duration changes after journal replay.
- [ ] Portal hides stale, permission-limited, manual-required, or not-claimed
      states.
- [ ] Raw private executable paths leak into parent UI.
- [ ] Malicious app metadata causes XSS or layout breakage.

## Main Execution Gates

- [ ] Source docs read: folder README, source index, current snapshot, full
      scope plan, platform deep dive, test blueprint, UI/UX guide, and the
      assigned workpack.
- [ ] Hub lock covers the workpack file and exact implementation/docs paths.
- [ ] Existing source layout inspected before editing; no parallel app-control
      truth created.
- [ ] TypeScript Effect Schema contracts land before Rust/service/portal
      consumers.
- [ ] Rust protocol parity exists for new protocol-facing contracts.
- [ ] Journal/read-model/storage behavior exists before portal or policy claims
      depend on it.
- [ ] Portal UI renders capability, degraded, stale, unsupported, and
      manual-required states honestly.
- [ ] Inventory-only proof never triggers strict action.
- [ ] Running-only proof never claims foreground.
- [ ] Foreground proof never claims content.
- [ ] AI classification is evidence, not authority.
- [ ] Parent policy is the enforcement authority.
- [ ] Required proof pack exists with logs, JSON, screenshots, or explicit N/A
      reasons for every applicable gate.
- [ ] Feature docs, expectation docs, module READMEs, and product capability
      checklist decisions are recorded.
- [ ] `DONE` report includes workpack, touched paths, validation, proof, known
      gaps, and documentation changes.

## Base Workpack Checklist

Use `[ ]` for not started, `[~]` for in progress, and `[x]` only after the
required proof pack exists.

| Step | Workpack                                                                                                                     | Status | Owner/Lane | Branch/PR/Commit                           | Evidence Or Proof                                                             | Doc/Checklist Decision                                                                                                                                                                                                                                                                                                                                                                   |
| ---- | ---------------------------------------------------------------------------------------------------------------------------- | ------ | ---------- | ------------------------------------------ | ----------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 01   | [Contract boundary and Effect schemas](workpacks/01-contract-boundary-and-effect-schemas.md)                                 | [x]    | codex-c    | codex/app-plan-proof-reconciliation        | output/app-plan-proof/01-contract-boundary-and-effect-schemas                 | Covered by shared app/game WP01 proof; product checklist unchanged because this is contract/proof reconciliation only                                                                                                                                                                                                                                                                    |
| 02   | [Source index and doc reconciliation](workpacks/02-source-index-and-doc-reconciliation.md)                                   | [x]    | codex-c    | codex/app-plan-proof-reconciliation        | output/app-plan-proof/02-source-index-and-doc-reconciliation                  | Covered by shared app/game WP02 proof; product checklist unchanged because this is routing/docs reconciliation only                                                                                                                                                                                                                                                                      |
| 03   | [Current app snapshot and gap map](workpacks/03-current-app-snapshot-and-gap-map.md)                                         | [x]    | codex-c    | codex/app-plan-proof-reconciliation        | output/app-plan-proof/03-current-app-snapshot-and-gap-map                     | Covered by shared app/game WP03 proof; runtime/product gaps remain recorded                                                                                                                                                                                                                                                                                                              |
| 04   | [App identity model](workpacks/04-app-identity-model.md)                                                                     | [x]    | codex-c    | codex/app-plan-proof-reconciliation        | output/app-plan-proof/04-app-identity-model                                   | Covered by shared app/game WP04 proof; product checklist unchanged until runtime identity merge/product UI proof exists                                                                                                                                                                                                                                                                  |
| 05   | [Installed app inventory model](workpacks/05-installed-app-inventory-model.md)                                               | [x]    | codex-c    | codex/app-plan-proof-reconciliation        | output/app-plan-proof/05-installed-app-inventory-model                        | Covered by shared app/game WP05 proof; product checklist unchanged until live inventory/service/portal proof exists                                                                                                                                                                                                                                                                      |
| 06   | [Windows installed app inventory adapter](workpacks/06-windows-installed-app-inventory-adapter.md)                           | [x]    | codex-c    | codex/app-plan-proof-reconciliation        | output/app-plan-proof/06-windows-installed-app-inventory-adapter              | Covered by shared app/game WP06 proof; parser proof only, live source crawling/product status unchanged                                                                                                                                                                                                                                                                                  |
| 07   | [Windows Store UWP AppX inventory adapter](workpacks/07-windows-store-uwp-appx-inventory-adapter.md)                         | [x]    | codex-c    | codex/app-plan-proof-reconciliation        | output/app-plan-proof/07-windows-store-uwp-appx-inventory-adapter             | Covered by shared app/game WP07 proof; parser proof only, live package enumeration/product status unchanged                                                                                                                                                                                                                                                                              |
| 08   | [Windows process runtime evidence adapter](workpacks/08-windows-process-runtime-evidence-adapter.md)                         | [x]    | codex-c    | codex/app-plan-proof-reconciliation        | output/app-plan-proof/08-windows-process-runtime-evidence-adapter             | Covered by shared app/game WP08 proof; parser proof only, live capture/service/portal status unchanged                                                                                                                                                                                                                                                                                   |
| 09   | [Windows foreground app evidence adapter](workpacks/09-windows-foreground-app-evidence-adapter.md)                           | [x]    | codex-c    | codex/app-game-windows-foreground-evidence | output/app-plan-proof/09-windows-foreground-app-evidence-adapter              | Covered by shared app/game WP09 proof; product checklist unchanged because live capture/service/portal status did not move                                                                                                                                                                                                                                                               |
| 10   | [Cross-platform authority matrix](workpacks/10-cross-platform-authority-matrix.md)                                           | [x]    | codex-c    | codex/app-game-authority-matrix            | output/app-plan-proof/10-cross-platform-authority-matrix                      | Covered by shared app/game WP11 proof; contract/test proof only, runtime platform adapters remain manual-required                                                                                                                                                                                                                                                                        |
| 11   | [App category and risk taxonomy](workpacks/11-app-category-and-risk-taxonomy.md)                                             | [x]    | codex-c    | codex/app-game-category-risk-taxonomy      | output/app-plan-proof/11-app-category-and-risk-taxonomy                       | Covered by shared app/game WP12 proof; contract/test proof only, no live classifier/policy/UI/enforcement status moved                                                                                                                                                                                                                                                                   |
| 12   | [App sessionization and duration engine](workpacks/12-app-sessionization-and-duration-engine.md)                             | [x]    | codex-c    | codex/app-game-sessionization-duration     | output/app-plan-proof/12-app-sessionization-and-duration-engine               | Covered by shared app/game WP13 proof; product checklist proof strengthened but status remains in progress pending journal-file ingest, service events, portal rows, policy, and live platform proof                                                                                                                                                                                     |
| 13   | [Journal and SQLite app ingest](workpacks/13-journal-and-sqlite-app-ingest.md)                                               | [x]    | codex-c    | codex/app-game-journal-sqlite-ingest       | output/app-plan-proof/13-journal-and-sqlite-app-ingest                        | Covered by shared app/game WP14 proof; Rust encrypted-journal replay plus SQLite projection proof only, product checklist unchanged because service events, portal, policy, and live platform proof remain open                                                                                                                                                                          |
| 14   | [App read models and service events](workpacks/14-app-read-models-and-service-events.md)                                     | [x]    | codex-c    | codex/app-game-read-model-service-events   | output/app-plan-proof/14-app-read-models-and-service-events                   | Native app-use activity read-model rows now consume the shared app-game service projection; product checklist not edited because `codex-a` holds that lock and app dashboard, policy/approval, live source, and platform proof remain gaps                                                                                                                                               |
| 15   | [Parent portal app inventory running session surfaces](workpacks/15-parent-portal-app-inventory-running-session-surfaces.md) | [x]    | codex-c    | codex/app-game-read-model-service-events   | output/app-plan-proof/15-parent-portal-app-inventory-running-session-surfaces | Native app-use rows now feed the shared App/Game Sessions dashboard with separate inventory, running, foreground, unknown/risk/manual-required capability, duration, and evidence counts; product checklist unchanged because live adapters, unknown approvals, policy integration, and platform proof remain gaps                                                                       |
| 16   | [New app and unknown app approval flow](workpacks/16-new-app-and-unknown-app-approval-flow.md)                               | [x]    | codex-c    | codex/app-game-read-model-service-events   | output/app-plan-proof/16-new-app-and-unknown-app-approval-flow                | Covered by shared app/game WP17 proof; app approval contracts now distinguish new inventory apps, unknown runtime apps, portable/installer candidates, expiry, audit-backed replay state, child status/reason refs, and manual-required block outcomes; product checklist unchanged because live adapters, approval UI, service storage, and platform proof remain gaps                  |
| 17   | [Risk app detection](workpacks/17-risk-app-detection.md)                                                                     | [ ]    |            |                                            |                                                                               |                                                                                                                                                                                                                                                                                                                                                                                          |
| 18   | [Policy target compiler for app rules](workpacks/18-policy-target-compiler-for-app-rules.md)                                 | [x]    | codex-c    | codex/app-game-read-model-service-events   | output/app-plan-proof/18-policy-target-compiler-for-app-rules                 | Cross-recorded from shared app/game compiler proof: app targets require identity, unknown-state, category, schedule, capability, authority, device/local-user freshness, and evidence proof before dry-run decisions; unproved block-launch compiles to manual-required; WP17 risk app detection remains open and product checklist unchanged because runtime/UI/enforcement remain gaps |
| 19   | [Time budget schedule bonus-time integration](workpacks/19-time-budget-schedule-bonus-time-integration.md)                   | [x]    | codex-c    | codex/app-game-read-model-service-events   | output/app-plan-proof/19-time-budget-schedule-bonus-time-integration          | Cross-recorded from shared app/game WP20: native app time-budget decisions now consume stored session refs, schedule evidence, bonus approval/audit refs, ask-parent/manual-required dry-run states, effective budget math, and timer recovery refs; WP17 risk detection remains open and product checklist unchanged because runtime/UI/service/adapter execution remain gaps           |
| 20   | [Child-facing app warning block request UX](workpacks/20-child-facing-app-warning-block-request-ux.md)                       | [ ]    |            |                                            |                                                                               |                                                                                                                                                                                                                                                                                                                                                                                          |
| 21   | [Windows owned-process terminate time-limit proof](workpacks/21-windows-owned-process-terminate-time-limit-proof.md)         | [ ]    |            |                                            |                                                                               |                                                                                                                                                                                                                                                                                                                                                                                          |
| 22   | [Broad blocking proof gates](workpacks/22-broad-blocking-proof-gates.md)                                                     | [ ]    |            |                                            |                                                                               |                                                                                                                                                                                                                                                                                                                                                                                          |
| 23   | [App AI classifier digest boundary](workpacks/23-app-ai-classifier-digest-boundary.md)                                       | [ ]    |            |                                            |                                                                               |                                                                                                                                                                                                                                                                                                                                                                                          |
| 24   | [Platform extension checklist and proof routing](workpacks/24-platform-extension-checklist-and-proof-routing.md)             | [ ]    |            |                                            |                                                                               |                                                                                                                                                                                                                                                                                                                                                                                          |
| 25   | [Install and uninstall approval handoff](workpacks/25-install-and-uninstall-approval-handoff.md)                             | [ ]    |            |                                            |                                                                               |                                                                                                                                                                                                                                                                                                                                                                                          |
| 26   | [Performance and service health](workpacks/26-performance-and-service-health.md)                                             | [ ]    |            |                                            |                                                                               |                                                                                                                                                                                                                                                                                                                                                                                          |
| 27   | [E2E and manual proof artifacts](workpacks/27-e2e-and-manual-proof-artifacts.md)                                             | [ ]    |            |                                            |                                                                               |                                                                                                                                                                                                                                                                                                                                                                                          |
| 28   | [Rollout checklist and PR gate](workpacks/28-rollout-checklist-and-pr-gate.md)                                               | [ ]    |            |                                            |                                                                               |                                                                                                                                                                                                                                                                                                                                                                                          |

## Platform Extension Checklist

Keep these as extension checklist items, not base MVP workpacks, unless a hub
assignment explicitly promotes one into implementation.

| Step       | Extension Item                                        | Status | Owner/Lane | Evidence Or Proof | Notes |
| ---------- | ----------------------------------------------------- | ------ | ---------- | ----------------- | ----- |
| MAC-01     | App bundle inventory adapter                          | [ ]    |            |                   |       |
| MAC-02     | LaunchServices/Spotlight inventory adapter            | [ ]    |            |                   |       |
| MAC-03     | NSWorkspace running app adapter                       | [ ]    |            |                   |       |
| MAC-04     | Accessibility foreground/window adapter               | [ ]    |            |                   |       |
| MAC-05     | Code signature/team-id identity adapter               | [ ]    |            |                   |       |
| MAC-06     | LaunchAgent/LaunchDaemon service model                | [ ]    |            |                   |       |
| MAC-07     | PPPC profile for Accessibility/Automation permissions | [ ]    |            |                   |       |
| MAC-08     | MDM installed-app query proof                         | [ ]    |            |                   |       |
| MAC-09     | Parental Controls payload proof                       | [ ]    |            |                   |       |
| MAC-10     | Endpoint Security feasibility/proof                   | [ ]    |            |                   |       |
| MAC-11     | macOS terminate/quit adapter                          | [ ]    |            |                   |       |
| MAC-12     | macOS hard-block manual proof gate                    | [ ]    |            |                   |       |
| IOS-01     | FamilyControls authorization UX                       | [ ]    |            |                   |       |
| IOS-02     | FamilyActivityPicker token selection                  | [ ]    |            |                   |       |
| IOS-03     | DeviceActivity schedule/threshold monitor             | [ ]    |            |                   |       |
| IOS-04     | ManagedSettings app/category shield adapter           | [ ]    |            |                   |       |
| IOS-05     | Shield UI and child request flow                      | [ ]    |            |                   |       |
| IOS-06     | Token-based app identity model                        | [ ]    |            |                   |       |
| IOS-07     | iOS MDM installed-app query mode                      | [ ]    |            |                   |       |
| IOS-08     | Supervised-device restriction matrix                  | [ ]    |            |                   |       |
| IOS-09     | App Lock / Single App Mode proof                      | [ ]    |            |                   |       |
| IOS-10     | iOS exception/approval flow                           | [ ]    |            |                   |       |
| IOS-11     | iOS manual-required fallback labels                   | [ ]    |            |                   |       |
| IOS-12     | iOS store/signing/entitlement proof                   | [ ]    |            |                   |       |
| ANDROID-01 | Package inventory and visibility contract             | [ ]    |            |                   |       |
| ANDROID-02 | UsageStats permission and usage summary adapter       | [ ]    |            |                   |       |
| ANDROID-03 | UsageEvents foreground/session adapter                | [ ]    |            |                   |       |
| ANDROID-04 | Accessibility-assisted foreground/overlay adapter     | [ ]    |            |                   |       |
| ANDROID-05 | VpnService/DNS relation handoff to network plan       | [ ]    |            |                   |       |
| ANDROID-06 | Device Owner provisioning flow                        | [ ]    |            |                   |       |
| ANDROID-07 | Profile Owner / managed profile flow                  | [ ]    |            |                   |       |
| ANDROID-08 | setApplicationHidden adapter proof                    | [ ]    |            |                   |       |
| ANDROID-09 | setPackagesSuspended adapter proof                    | [ ]    |            |                   |       |
| ANDROID-10 | setUninstallBlocked proof                             | [ ]    |            |                   |       |
| ANDROID-11 | Lock Task / allowlist mode proof                      | [ ]    |            |                   |       |
| ANDROID-12 | Managed configurations for app restrictions           | [ ]    |            |                   |       |
| ANDROID-13 | Play policy/signing/store compliance proof            | [ ]    |            |                   |       |
| ANDROID-14 | Android child request/approval UX                     | [ ]    |            |                   |       |
| LINUX-01   | Desktop entry inventory adapter                       | [ ]    |            |                   |       |
| LINUX-02   | dpkg/rpm/pacman package inventory adapter             | [ ]    |            |                   |       |
| LINUX-03   | Flatpak inventory adapter                             | [ ]    |            |                   |       |
| LINUX-04   | Snap inventory adapter                                | [ ]    |            |                   |       |
| LINUX-05   | AppImage bounded scan adapter                         | [ ]    |            |                   |       |
| LINUX-06   | procfs runtime adapter                                | [ ]    |            |                   |       |
| LINUX-07   | cgroup/systemd identity adapter                       | [ ]    |            |                   |       |
| LINUX-08   | X11 foreground adapter                                | [ ]    |            |                   |       |
| LINUX-09   | Wayland compositor capability matrix                  | [ ]    |            |                   |       |
| LINUX-10   | Linux terminate adapter                               | [ ]    |            |                   |       |
| LINUX-11   | cgroup/systemd scope enforcement proof                | [ ]    |            |                   |       |
| LINUX-12   | AppArmor/SELinux manual proof                         | [ ]    |            |                   |       |
| LINUX-13   | Package-manager restriction proof                     | [ ]    |            |                   |       |
| LINUX-14   | Flatpak/Snap restriction proof                        | [ ]    |            |                   |       |

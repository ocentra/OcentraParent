# Pasted Content Coverage Audit

This audit records the read-through of the app-plan pasted content. The source
attachments were consolidated into repo-owned plan docs instead of copied as
unmanaged notes.

## Attachment Map

| Attachment                             | Source Theme                                           | Covered By                                                                                                                                                                                                                                                                                                                   | Coverage Notes                                                                                                                                                                                                       |
| -------------------------------------- | ------------------------------------------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `44abd211-2cf4-441e-8195-e17804297311` | V0.5 Native Apps Full Scope Plan                       | [README](README.md), [source index](source-index.md), [current snapshot](current-app-snapshot.md), [full scope plan](v0-5-native-apps-full-scope-plan.md), [test blueprint](v0-5-native-apps-test-blueprint.md), [implementation checklist](implementation-checklist.md), [workpacks](workpacks/)                            | Covered. The proposed 28 base workpacks were normalized to current repo ownership and existing `AppGame*` contract paths.                                                                                            |
| `5ccf5397-260d-4aef-884f-d79ec2b73138` | Platform Authority Matrix And Cross-Platform Deep Dive | [platform deep dive](v0-5-native-apps-platform-deep-dive.md), [implementation checklist](implementation-checklist.md), [full scope plan](v0-5-native-apps-full-scope-plan.md), [workpack 10](workpacks/10-cross-platform-authority-matrix.md), [workpack 24](workpacks/24-platform-extension-checklist-and-proof-routing.md) | Covered. The platform-specific MAC/IOS/ANDROID/LINUX items are tracked as extension checklists, not base MVP workpacks.                                                                                              |
| `355abd55-7081-484c-b29f-1b57e48d7cf3` | V0.5 Native Apps Test Blueprint                        | [test blueprint](v0-5-native-apps-test-blueprint.md), [implementation checklist](implementation-checklist.md), [workpack 27](workpacks/27-e2e-and-manual-proof-artifacts.md), [workpack 28](workpacks/28-rollout-checklist-and-pr-gate.md)                                                                                   | Covered. The lightweight test skeleton was replaced with Doc 2's evidence invariants, recommended test layout, contract/security/platform proof matrices, fixtures, CI gates, merge blockers, and final quality bar. |

## Coverage Checklist

- [ ] Native app inventory is evidence, not current use.
- [ ] Running process evidence proves process use, not foreground use.
- [ ] Foreground evidence proves active use, not content.
- [ ] AI classification is evidence, not authority.
- [ ] Parent policy decides actions.
- [ ] Enforcement requires platform adapter proof and authority-tier proof.
- [ ] App-specific scope is separated from browser games/web apps and
      game-specific native game/launcher product work.
- [ ] Existing app/game feature docs remain source-of-truth inputs.
- [ ] Current code ownership is mapped to existing `packages/activity-domain`,
      `packages/parent-domain`, `crates/agent-protocol`, `crates/agent-core`,
      `crates/agent-service`, and `apps/portal` paths.
- [ ] Current app/game contracts and scoped owned-process time-limit proof are
      recorded as existing state, not blank-state work.
- [ ] App identity uses layered fields and does not rely on display name alone.
- [ ] Inventory sources include Windows registry, Start Menu, AppX/UWP, known
      paths, executable metadata, macOS bundles, Linux desktop/package sources,
      Android packages, iOS tokens/MDM, and parent catalog.
- [ ] Runtime sources include process snapshots, process start/exit,
      foreground window, UsageStats/UsageEvents, DeviceActivity, accessibility,
      and managed-device state.
- [ ] App session summaries remain derived read models backed by evidence refs.
- [ ] App categories and risk labels are source/confidence-bearing policy
      inputs, not hidden decisions.
- [ ] New/unknown app approval is represented as a policy/approval flow where
      adapter proof exists and report/ask/manual-required where it does not.
- [ ] Risk app types include VPN/proxy, remote desktop, torrent/download,
      installer/updater, AI/chatbot, messaging/social/video, and unknown risk.
- [ ] Windows enforcement is separated into observe, warn, owned-process
      terminate/time-limit, AppLocker/App Control proof, and strict allowlist
      proof.
- [ ] macOS is represented through app bundles, LaunchServices/Spotlight,
      NSWorkspace, Accessibility, code signature, LaunchAgent/LaunchDaemon,
      PPPC, MDM, Parental Controls payloads, System Extensions, and Endpoint
      Security proof paths.
- [ ] iOS/iPadOS is represented through FamilyControls, FamilyActivityPicker,
      DeviceActivity, ManagedSettings, MDM installed-app query, supervised
      restrictions, App Lock, and store/signing/entitlement proof paths.
- [ ] Android is represented through package visibility, UsageStats,
      UsageEvents, Accessibility, Device Owner/Profile Owner/DPC,
      `setApplicationHidden`, `setPackagesSuspended`, `setUninstallBlocked`,
      managed configurations, and lock task mode proof paths.
- [ ] Linux is represented through desktop entries, package managers, Flatpak,
      Snap, AppImage, procfs, cgroups/systemd, X11/Wayland matrices,
      AppArmor/SELinux, and distro-specific proof paths.
- [ ] Platform-specific items are extension checklists like browser-plan
      enhancement checklists, not base MVP workpacks.
- [ ] Proof expectations include contract, Rust, fixture, security, persistence,
      UI, manual platform, performance, and rollout gates.
- [ ] The test blueprint names required unit, integration, contract, security,
      platform/manual, E2E, Playwright, performance, and CI gates.
- [ ] The test blueprint records the minimum serious MVP test set and the
      merge-blocking failures that prevent `DONE`, PR-ready, or merge claims.
- [ ] Required fixtures are listed for inventory, runtime, sessions, policy, and
      UI states.
- [ ] Manual tests require explicit platform tags such as Windows, macOS, Linux,
      Android Device Owner, iOS FamilyControls, MDM, Endpoint Security,
      AppLocker, and App Control.
- [ ] Platform-specific no-claim gates are recorded for Windows, macOS, Linux,
      Android, and iOS/iPadOS.
- [ ] Product copy must not use bare "unsupported" as the final platform claim;
      it must name observe-only, permission-required, managed-device-required,
      admin/root-required, system-extension-required, supervised-device-required,
      manual-required, or not-claimed with proof needed to move up.

## Consolidation Decisions

- The pasted 28-step base split remains the base native app workpack sequence.
- The platform-specific MAC/IOS/ANDROID/LINUX lists are kept as platform
  extension checklists in the main checklist and platform deep dive.
- The detailed Doc 2 test blueprint replaces the earlier lightweight blueprint
  instead of becoming a parallel note.
- The implementation checklist now mirrors the stricter proof pack and
  merge-blocking gates, including platform workpack authority, permission setup,
  and rollback proof files.
- All 28 workpacks were expanded after Doc 2 integration so each file now
  mirrors the browser-plan workpack style: source inputs, current/target state,
  scope, touched paths, tests/proof, repeated AI worker checklist, and
  manual-required gaps.
- The app plan does not split game work into a new `game-plan` yet. It records
  that native game-specific work is adjacent and should become a separate plan
  when the user asks for that slice.
- The app plan does not move generated app-control inventories. It links them as
  source inputs.
- External platform research context is retained as planning context; product
  claims still require repo proof.

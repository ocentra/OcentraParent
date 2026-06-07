# 05 Cross-Platform Inventory Matrix

## Where We Are

The architecture docs define platform posture, but runtime/browser support is
Windows-first. macOS, Linux, Android, iOS, Safari, Firefox, and mobile browser
support remain platform-specific or manual-required.

2026-06-02 codex-d progress: `activity-domain` now exposes a typed browser
platform inventory matrix. It keeps Windows Edge/Chrome entries host-observed,
macOS/Linux Chromium candidates manual-required, Firefox/Safari/WebKit and
external mobile browsers unsupported until platform adapters prove them, and
Android owned-browser-shell support manual-required. The matrix rejects
non-Windows managed exact-URL or known-active claims. This is contract proof
only; it does not add managed launch, bridge custody, exact URL, active-tab,
macOS/Linux/mobile adapters, or manual platform evidence.

2026-06-04 codex-d progress: `scripts/test/browser-platform-inventory-matrix-proof.mjs`
now replays the matrix as a proof gate. It validates row uniqueness, platform
coverage, manual-required proof requirements, unsupported exact-URL labels,
Windows-only host-observed/fixture-backed status, and iOS unsupported boundaries, then writes
`test-results/browser-platform-inventory-matrix-proof/proof.json` and
`output/browser-plan-proof/05-cross-platform-inventory-matrix/11-proof-gate-manifest.md`.
This is still a no-claim gate; it does not add real macOS/Linux/mobile adapters
or product checklist status.

2026-06-06 codex-d continuation: the proof pack now includes
`output/browser-plan-proof/05-cross-platform-inventory-matrix/09-manual-platform-proof.md`.
The repeatable proof gate is green with that artifact present, but the row
remains partial because macOS, Linux, Android, and iOS still require real
platform/device artifacts before any product support upgrade.

2026-06-07 codex-d continuation: `scripts/test/browser-platform-android-host-proof.mjs`
now captures real Android emulator host evidence for WP05. The proof can start
the configured headless Android emulator through the local SDK, queries only
known public browser package ids, records Chrome package visibility and the
default VIEW handler as redacted refs, captures UI-tree and logcat hashes, and
writes `test-results/browser-platform-android-host-proof/proof.json` plus
`output/browser-plan-proof/05-cross-platform-inventory-matrix/11-android-host-device-proof.json`.
The proof also hashes the Android agent manifest, Gradle application id, and
`ChildAndroidPrivilegedCapabilityProof` source to prove the current Android
package is an agent wrapper, not an owned browser shell: no WebView, VIEW
handler, AccessibilityService, VpnService, DeviceAdminReceiver, UsageStats
permission, or owned browser package id is declared.
The screenshot path remains explicitly unused when the headless emulator
returns a black screencap. The matrix gate reads that artifact and reports
`android-browser-package-visibility-proof`, but WP05 remains partial: the
Ocentra owned browser shell is not installed, and exact URL, active-tab,
device-owner policy, VPN/DNS browser proof, UsageStats/Accessibility route
proof, enforcement, and product checklist upgrade remain unclaimed.

2026-06-07 codex-d continuation: `scripts/test/browser-platform-linux-host-proof.mjs`
now captures real WSL Ubuntu package/PATH/desktop-entry boundary evidence for
WP05. The proof queries only known public Linux browser commands, Debian package
names, and desktop-entry globs, then writes
`test-results/browser-platform-linux-host-proof/proof.json` plus
`output/browser-plan-proof/05-cross-platform-inventory-matrix/12-linux-host-package-proof.json`.
The current host has WSL available but no known Chrome/Chromium/Firefox/Edge
command, package, or desktop-entry proof. This is useful negative evidence, but
not Linux desktop browser support: managed profile, desktop adapter, exact URL,
active tab, Snap/Flatpak, and enforcement remain unclaimed.

2026-06-07 codex-d continuation: `scripts/test/browser-platform-windows-host-proof.mjs`
now captures real Windows host browser inventory boundary evidence for WP05.
The proof queries known public Edge/Chrome/Firefox executable locations, Windows
App Paths registry keys, and HTTP/HTTPS URL association user-choice keys, stores
only redacted hash refs plus normalized known-browser-family labels for
recognized handler ProgIds, and writes
`test-results/browser-platform-windows-host-proof/proof.json` plus
`output/browser-plan-proof/05-cross-platform-inventory-matrix/13-windows-host-browser-proof.json`.
The matrix gate reads that artifact and reports host-observed Windows rows, but
managed launch, bridge custody, exact URL, active tab, managed profile repair,
and enforcement remain unclaimed.

## Where We Want To Be

The browser inventory/read model can show platform-specific capability states
without claiming desktop CDP where it does not apply.

## Scope

- macOS app bundle inventory and CDP candidate browsers.
- Linux desktop entries, packages, PATH, Snap/Flatpak, and CDP candidates.
- Firefox WebDriver BiDi or managed extension later-adapter state.
- Safari/WebKit platform-specific state.
- Android owned browser shell, VPN/DNS, UsageStats, Accessibility, Device Owner,
  managed profile, managed configurations.
- iOS FamilyControls, ManagedSettings, Safari extension, and manual-required
  states.

## Touched Paths

- `packages/parent-domain/src/browser-control-*.ts`
- `packages/activity-domain/src/browser*.ts`
- `scripts/test/browser-platform-inventory-matrix-proof.mjs`
- `scripts/test/browser-platform-android-host-proof.mjs`
- `scripts/test/browser-platform-linux-host-proof.mjs`
- `scripts/test/browser-platform-windows-host-proof.mjs`
- `test-results/browser-platform-inventory-matrix-proof/`
- `test-results/browser-platform-android-host-proof/`
- `test-results/browser-platform-linux-host-proof/`
- `test-results/browser-platform-windows-host-proof/`
- `output/browser-plan-proof/05-cross-platform-inventory-matrix/`
- platform README/docs when implementation starts.

## Tests And Proof

- Unit tests for matrix derivation.
- Fixture tests for platform states.
- `node scripts/test/browser-platform-inventory-matrix-proof.mjs`
- `node scripts/test/browser-platform-windows-host-proof.mjs`
- `node scripts/test/browser-platform-android-host-proof.mjs`
- `node scripts/test/browser-platform-linux-host-proof.mjs`
- Manual platform proof tables when platform work starts.

## AI Worker Checklist

Fill this before reporting `DONE` or PR-ready:

- [x] Confirm source docs read: [folder README](../README.md), [source index](../source-index.md), [current snapshot](../current-browser-snapshot.md), [full scope plan](../v0-5-managed-browser-full-scope-plan.md), [test blueprint](../v0-5-managed-browser-test-blueprint.md), [UI/UX guide](../ui-ux-requirements-guide.md), [main checklist](../implementation-checklist.md), and this workpack.
- [x] Check enhancement overlap: URL/video AI intelligence, social platform/account/feed gating, and browser games/cloud gaming.
- [x] Hub lock covers this workpack and exact implementation/docs paths.
- [x] Existing source layout inspected; no parallel browser truth created.
- [x] Before-state source snapshot recorded in `output/browser-plan-proof/05-cross-platform-inventory-matrix/00-source-snapshot.md`.
- [x] Contracts updated first where this workpack changes behavior.
- [x] Rust/service/portal parity updated only after contracts exist; no Rust/service/portal surface changed in this contract-only slice.
- [x] Raw evidence artifacts captured or marked N/A: this slice is a platform support matrix contract and has no bridge/CDP, journal, SQLite, policy, or action runtime evidence.
- [x] Tests/proof listed in this workpack are implemented for matrix derivation, dishonest-state rejection, and the repeatable proof gate; Windows host browser executable proof and default URL handler association boundary evidence, Android emulator package-visibility proof, and WSL Linux package/PATH boundary proof are present, while managed launch/bridge custody/exact URL/active tab, live macOS/Linux desktop/owned-shell Android/iOS fixtures, and manual proof remain manual-required.
- [x] Validation command outputs saved in the proof pack and summarized in [main checklist](../implementation-checklist.md).
- [x] UI snapshots captured for every touched parent portal, child UX, block/warn, policy authoring, or dashboard state; no UI changed, so `ui-not-applicable.md` records why.
- [x] Security/no-claim negative proof captured: non-Windows entries reject managed exact-URL and known-active claims; iOS remains unsupported; unsupported entries cannot keep exact URL available.
- [x] Manual platform proof captured for real browser/OS claims; Android emulator browser package/default-handler visibility, UI-tree/logcat hash evidence, and source-backed no-owned-shell/no-WebView/no-privileged-browser-adapter boundary evidence are captured in `11-android-host-device-proof.json`, WSL Linux package/PATH boundary evidence is captured in `12-linux-host-package-proof.json`, Windows host browser executable proof and default URL handler association boundary evidence are captured in `13-windows-host-browser-proof.json`, and `09-manual-platform-proof.md` records the remaining manual-required boundaries.
- [x] Evidence/proof artifact paths recorded in [main checklist](../implementation-checklist.md), including `test-results/browser-platform-inventory-matrix-proof/proof.json` and `output/browser-plan-proof/05-cross-platform-inventory-matrix/11-proof-gate-manifest.md`.
- [x] Feature/expectation/product-checklist/README update decision recorded in [main checklist](../implementation-checklist.md).
- [x] Known gaps, deferred items, and no-claim boundaries recorded before `DONE`.

## Manual-Required Gaps

No browser product claim until real platform proof exists for the exact
capability being upgraded.
Windows host browser executable proof and default URL handler association
boundary evidence are now captured, but managed launch, bridge custody, exact
URL, active-tab, managed profile repair, and enforcement remain unclaimed.
Android emulator package visibility and source-backed Android agent boundary
evidence are now proved, but the owned browser shell, exact URL, active-tab,
device-owner policy, VPN/DNS browser proof, UsageStats/Accessibility route
proof, and enforcement remain unclaimed.
WSL Linux package/PATH evidence is now proved, but no Chrome/Chromium/Firefox/Edge
command, package, desktop entry, desktop adapter, exact URL, active-tab, Snap,
Flatpak, or enforcement claim is upgraded.
Remaining work requires real macOS app bundle inventory, Linux desktop entry and
package inventory, Android owned browser shell/device-policy proof, and iOS
FamilyControls/ManagedSettings/Safari-extension proof before this workpack can
be marked complete.

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
`android-browser-package-visibility-proof`, but WP05 remains partial: exact URL,
active-tab, device-owner policy, VPN/DNS browser proof, UsageStats/Accessibility
route proof, enforcement, and product checklist upgrade remain unclaimed.

2026-06-08 codex-d continuation: `scripts/test/browser-platform-android-host-proof.mjs`
now supports an explicit physical Android target through `ANDROID_SERIAL`. The
proof records that the requested raw serial was not persisted, filters out other
attached Android targets, and stores only a redacted device ref plus non-secret
ADB device metadata. On the available Samsung Galaxy S9 target it observed
product `star2qltecs`, model `SM_G965W`, boot completion, known browser package
visibility, default VIEW handler query evidence, UI-tree hash evidence, and
logcat hash evidence in
`test-results/browser-platform-android-host-proof/proof.json` and
`output/browser-plan-proof/05-cross-platform-inventory-matrix/11-android-host-device-proof.json`.
This upgrades physical-device package/default-handler visibility evidence only.
It does not claim owned shell custody on the physical device, Device Owner,
Browser Role, exact URL policy, known active tab, VPN/DNS browser proof,
UsageStats/Accessibility route proof, final policy execution, browser blocking,
Play signing, release readiness, or broad content-filter enforcement.

2026-06-07 codex-d continuation: `scripts/test/browser-platform-android-owned-shell-proof.mjs`
adds the first real owned Android browser shell proof for WP05. The proof builds
the separate `platforms/android/agent/browser-shell` APK, launches the configured
headless Android emulator through the local SDK, installs
`com.ocentra.parent.browser`, opens a local proof page through the shell's
`VIEW`/`BROWSABLE` handler, and records UIAutomator evidence that the shell
loaded the proof page. It writes
`test-results/browser-platform-android-owned-shell-proof/proof.json` and
`output/browser-plan-proof/05-cross-platform-inventory-matrix/15-android-owned-browser-shell-proof.json`.
The artifact stores only APK/source/UI-tree hashes and redacted device/URL refs;
it does not persist raw URLs, raw page content, raw intent resolution, raw
package lists, or raw UI trees. Headless emulator screenshots remain unpersisted
when unusable. The proof explicitly rejects managed exact-URL policy, known
active-tab proof, Device Owner policy, VPN/DNS browser proof, UsageStats route
proof, Accessibility route proof, and enforcement claims.

2026-06-07 codex-d continuation: the Android owned browser shell proof now
creates a disposable AOSP ATD emulator through `avdmanager`, installs the same
owned shell APK, and observes `dpm set-device-owner` / `dpm list-owners`
Device Owner enrollment for
`com.ocentra.parent.browser/ca.ocentra.parent.browser.OcentraOwnedBrowserDeviceAdminReceiver`.
The proof now also observes the Device Owner app configuring persistent
HTTP/HTTPS browser routing policy through `DevicePolicyManager` while rendering
that state in the owned shell UI. It deletes the temporary AVD after the run and
stores only hashes and redacted refs for ADB, emulator, AVD manager, Device
Policy Manager output, APK/source, device, URL, resolver output, and UI tree
evidence. This upgrades only source-backed DeviceAdmin metadata, proof-launched
emulator Device Owner enrollment, and proof-launched emulator policy mutation.
It does not claim exact URL policy, known active-tab policy, implicit browser
routing enforcement, VPN/DNS browser proof, UsageStats route proof,
Accessibility route proof, final policy execution, browser blocking, broad
content-filter enforcement, physical-device behavior, Play signing, or release
readiness.

2026-06-07 codex-d continuation: the Android owned browser shell proof now
also exercises the Android Browser Role route on the disposable proof emulator.
The proof first records that Device Owner persistent preferred activity does
not itself produce a browser implicit-route resolver match, then applies the
browser role through the host shell, observes Android resolving the same
`ACTION_VIEW`/`DEFAULT`/`BROWSABLE` HTTP proof URL to
`com.ocentra.parent.browser`, and observes the owned shell loading the proof
page from an implicit VIEW launch. This upgrades only proof-launched emulator
browser-role implicit routing for the owned shell. It does not claim silent
Device Owner default-browser assignment, exact URL policy, known active-tab
policy, physical-device behavior, final policy execution, browser blocking,
broad content-filter enforcement, Play signing, or release readiness.

2026-06-07 codex-d continuation: `scripts/test/browser-platform-linux-host-proof.mjs`
now captures real WSL Ubuntu package/PATH/desktop-entry boundary evidence plus
a positive headless Linux browser launch proof for WP05. The proof queries only
known public Linux browser commands, Debian package names, and desktop-entry
globs, launches the observed Linux browser against a local proof document,
captures DOM hash evidence and a screenshot, then writes
`test-results/browser-platform-linux-host-proof/proof.json` plus
`output/browser-plan-proof/05-cross-platform-inventory-matrix/12-linux-host-package-proof.json`
and
`output/browser-plan-proof/05-cross-platform-inventory-matrix/12-linux-headless-browser-screenshot.png`.
The current host has WSL available and Google Chrome installed/launchable, so
the Linux Chrome row is host-observed for install and launch only. Linux desktop
adapter integration, managed profile, exact URL, active tab, Snap/Flatpak, and
enforcement remain unclaimed.

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

2026-06-07 codex-d continuation: `scripts/test/browser-platform-windows-managed-cdp-proof.mjs`
now launches a real Windows Chromium-family browser through an Ocentra-owned
temporary managed profile and loopback CDP endpoint. The proof opens a local
proof page, verifies `/json/version` and `/json/list`, observes the exact
managed URL through CDP, captures a screenshot through the CDP WebSocket, cleans
up the temporary profile, and writes
`test-results/browser-platform-windows-managed-cdp-proof/proof.json`,
`output/browser-plan-proof/05-cross-platform-inventory-matrix/14-windows-managed-cdp-proof.json`,
and
`output/browser-plan-proof/05-cross-platform-inventory-matrix/14-windows-managed-cdp-screenshot.png`.
This upgrades only Windows Ocentra-launched managed browser launch/bridge
custody/exact managed URL proof for the local proof page. It does not claim
exact active-tab enforcement, final policy execution, browser blocking,
non-Windows managed CDP support, raw path persistence, raw CDP payload
persistence, or raw page content capture.

## Where We Want To Be

The browser inventory/read model can show platform-specific capability states
without claiming desktop CDP where it does not apply.

## Scope

- macOS app bundle inventory and CDP candidate browsers.
- Linux desktop entries, packages, PATH, Snap/Flatpak, and CDP candidates.
- Firefox WebDriver BiDi or managed extension later-adapter state.
- Safari/WebKit platform-specific state.
- Android Device Owner, VPN/DNS, UsageStats, Accessibility, managed profile,
  managed configurations, exact active-tab policy, and enforcement.
- iOS FamilyControls, ManagedSettings, Safari extension, and manual-required
  states.

## Touched Paths

- `packages/parent-domain/src/browser-control-*.ts`
- `packages/activity-domain/src/browser*.ts`
- `scripts/test/browser-platform-inventory-matrix-proof.mjs`
- `scripts/test/browser-platform-android-host-proof.mjs`
- `scripts/test/browser-platform-android-owned-shell-proof.mjs`
- `scripts/test/browser-platform-linux-host-proof.mjs`
- `scripts/test/browser-platform-windows-host-proof.mjs`
- `scripts/test/browser-platform-windows-managed-cdp-proof.mjs`
- `test-results/browser-platform-inventory-matrix-proof/`
- `test-results/browser-platform-android-host-proof/`
- `test-results/browser-platform-android-owned-shell-proof/`
- `test-results/browser-platform-linux-host-proof/`
- `test-results/browser-platform-windows-host-proof/`
- `test-results/browser-platform-windows-managed-cdp-proof/`
- `output/browser-plan-proof/05-cross-platform-inventory-matrix/`
- platform README/docs when implementation starts.

## Tests And Proof

- Unit tests for matrix derivation.
- Fixture tests for platform states.
- `node scripts/test/browser-platform-inventory-matrix-proof.mjs`
- `node scripts/test/browser-platform-windows-host-proof.mjs`
- `node scripts/test/browser-platform-windows-managed-cdp-proof.mjs`
- `node scripts/test/browser-platform-android-host-proof.mjs`
- `node scripts/test/browser-platform-android-owned-shell-proof.mjs`
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
- [x] Tests/proof listed in this workpack are implemented for matrix derivation, dishonest-state rejection, and the repeatable proof gate; Windows host browser executable proof and default URL handler association boundary evidence, Windows Ocentra-launched managed CDP proof for an exact local proof URL, Android emulator package-visibility proof, physical Samsung Galaxy S9 package/default-handler visibility proof, Android owned browser shell build/install/launch proof, Android proof-created AVD Device Owner enrollment proof, Android proof-created AVD persistent browser routing policy mutation proof, Android proof-created emulator browser-role implicit routing proof, and WSL Linux package/PATH/desktop-entry/headless-launch proof are present, while exact active-tab enforcement, final policy execution, browser blocking, live macOS/iOS fixtures, and manual proof remain manual-required.
- [x] Validation command outputs saved in the proof pack and summarized in [main checklist](../implementation-checklist.md).
- [x] UI snapshots captured for every touched parent portal, child UX, block/warn, policy authoring, or dashboard state; no UI changed, so `ui-not-applicable.md` records why.
- [x] Security/no-claim negative proof captured: non-Windows entries reject managed exact-URL and known-active claims; iOS remains unsupported; unsupported entries cannot keep exact URL available.
- [x] Manual platform proof captured for real browser/OS claims; Android emulator and physical Samsung Galaxy S9 browser package/default-handler visibility, UI-tree/logcat hash evidence, and source-backed no-privileged-browser-adapter boundary evidence are captured in `11-android-host-device-proof.json`, Android owned browser shell build/install/launch, local proof-page UI evidence, proof-created AVD Device Owner enrollment evidence, proof-created AVD persistent browser routing policy mutation evidence, and proof-created emulator browser-role implicit routing evidence are captured in `15-android-owned-browser-shell-proof.json`, WSL Linux package/PATH/desktop-entry/headless-launch evidence is captured in `12-linux-host-package-proof.json` and `12-linux-headless-browser-screenshot.png`, Windows host browser executable proof and default URL handler association boundary evidence are captured in `13-windows-host-browser-proof.json`, Windows managed CDP exact-local-URL proof and screenshot evidence are captured in `14-windows-managed-cdp-proof.json` and `14-windows-managed-cdp-screenshot.png`, and `09-manual-platform-proof.md` records the remaining manual-required boundaries.
- [x] Evidence/proof artifact paths recorded in [main checklist](../implementation-checklist.md), including `test-results/browser-platform-inventory-matrix-proof/proof.json` and `output/browser-plan-proof/05-cross-platform-inventory-matrix/11-proof-gate-manifest.md`.
- [x] Feature/expectation/product-checklist/README update decision recorded in [main checklist](../implementation-checklist.md).
- [x] Known gaps, deferred items, and no-claim boundaries recorded before `DONE`.

## Manual-Required Gaps

No browser product claim until real platform proof exists for the exact
capability being upgraded.
Windows host browser executable proof and default URL handler association
boundary evidence are now captured. Windows managed CDP proof also launches an
Ocentra-owned temporary managed browser profile, observes the exact local proof
URL, and captures a CDP screenshot, but exact active-tab enforcement, final
policy execution, browser blocking, managed profile repair, and non-Windows
managed CDP support remain unclaimed.
Android emulator and physical Samsung Galaxy S9 package visibility,
source-backed Android agent boundary evidence, owned browser shell
build/install/launch proof, proof-created AVD Device Owner enrollment plus
persistent browser routing policy mutation proof, and proof-created emulator
browser-role implicit routing proof are now proved. Exact URL policy,
active-tab policy, silent Device Owner default-browser assignment, physical
owned-shell custody, physical Device Owner/Browser Role behavior, VPN/DNS
browser proof, UsageStats/Accessibility route proof, final policy execution,
browser blocking, and broad content-filter enforcement remain unclaimed.
WSL Linux package/PATH/desktop-entry evidence and a headless Google Chrome
launch/screenshot are now proved, but Linux desktop adapter integration,
managed profile, exact URL, active-tab, Snap, Flatpak, and enforcement remain
unclaimed.
Remaining work requires real macOS app bundle inventory and iOS
FamilyControls/ManagedSettings/Safari-extension proof before this workpack can
be marked complete.

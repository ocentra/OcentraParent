# WP05 Manual Platform Proof Boundary

Generated: 2026-06-08

This artifact records the manual platform boundary for the cross-platform
browser inventory matrix. It does not claim live non-Windows browser inventory,
managed exact URL evidence, known-active tab evidence, browser content capture,
blocking, rollback, or enforcement.

| Platform | Current proof state | Evidence boundary |
| --- | --- | --- |
| Windows | Host browser inventory proof present; managed CDP local proof present | `13-windows-host-browser-proof.json` records known Edge/Chrome/Firefox executable/App Paths visibility and queried HTTP/HTTPS URL-association-key boundary refs as redacted hashes. `14-windows-managed-cdp-proof.json` records an Ocentra-launched managed browser profile reaching the exact local proof URL with a CDP screenshot. It does not claim active-tab enforcement, final policy execution, browser blocking, or non-Windows support. |
| macOS | Manual-required | No macOS app bundle scan, Chrome/Edge managed profile launch, Safari/WebKit adapter, or CDP/BiDi platform proof is captured in this artifact. |
| Linux | WSL package/PATH proof plus headless browser launch proof present; desktop adapter proof manual-required | `12-linux-host-package-proof.json` records WSL Ubuntu availability, known browser command/package/desktop-entry queries, and a real headless Google Chrome launch with screenshot. It does not claim a Linux desktop session, managed profile, Snap/Flatpak proof, exact URL proof, known-active tab proof, Firefox BiDi, or enforcement. |
| Android | Physical device package/default-handler visibility proof present; physical owned-shell install/activity-start boundary observed; Device Owner/browser-role proof remains emulator-scoped | `11-android-host-device-proof.json` records the physical Wi-Fi ADB target as a redacted Samsung Galaxy S9 SM-G965W/star2qltecs device ref, queries known browser package ids only, records Chrome/Edge/Samsung Internet visibility, captures a screenshot/UI-tree/logcat hashes, and records a redacted default VIEW handler ref. `15-android-owned-browser-shell-proof.json` records proof-created AVD Device Owner enrollment and browser-role routing policy mutation for an owned browser shell path. It also records the physical Samsung Galaxy S9 installed the owned browser shell APK and started the explicit owned-shell activity, but the visible page remained behind keyguard/black-capture state; no physical Device Owner, browser-role routing, exact URL policy, known-active tab proof, VPN/DNS browser proof, UsageStats/Accessibility route proof, Play signing, broad browser enforcement, content filtering, or final policy execution is claimed. |
| iOS | Unsupported/manual-required future boundary | No FamilyControls authorization, ManagedSettings token selection, Safari extension, WebKit browser app, or device proof is captured in this artifact. |

Required before product upgrade:

- Real host/device artifacts for the specific platform being upgraded.
- Adapter proof that separates inventory discovery from exact URL/tab evidence.
- Managed profile or owned shell custody proof before exact URL or active-tab claims.
- Manual rollback and failure evidence before blocking or enforcement claims.

Checklist decision: WP05 remains a no-claim matrix proof unless and until those
platform artifacts exist. Product checklist status is not upgraded by this file.

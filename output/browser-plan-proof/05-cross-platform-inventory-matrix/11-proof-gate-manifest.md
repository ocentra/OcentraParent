# WP05 Platform Inventory Matrix Proof Gate

Generated: 2026-06-07T11:12:58.361Z

Rows checked: 12
Host-observed rows: 2
Fixture-backed rows: 0
Manual-required rows: 4
Unsupported rows: 6
Product claimed: false
Android host proof: android-browser-package-visibility-proof
Android owned shell proof: android-owned-browser-shell-build-install-launch-device-owner-proof
Linux host proof: linux-wsl-package-inventory-boundary-proof
Windows host proof: windows-host-browser-inventory-boundary-proof
Windows managed CDP proof: windows-managed-cdp-exact-url-proof

| Platform | Browser | Product | Proof State | Exact URL | Active Tab | Reason |
| --- | --- | --- | --- | --- | --- | --- |
| windows | edge | Microsoft Edge | host-observed | unavailable | unavailable | windows-managed-edge-candidate |
| windows | chrome | Google Chrome | host-observed | unavailable | unavailable | windows-managed-chrome-candidate |
| macos | chrome | Google Chrome | manual-required | manual-required | manual-required | macos-chrome-cdp-candidate-manual-required |
| macos | unknown | Safari | unsupported | unsupported | unsupported | macos-safari-webkit-later-adapter |
| linux | chrome | Google Chrome | manual-required | manual-required | manual-required | linux-chrome-cdp-candidate-manual-required |
| linux | unknown-chromium | Chromium | manual-required | manual-required | manual-required | linux-chromium-cdp-candidate-manual-required |
| linux | firefox | Mozilla Firefox | unsupported | unsupported | unsupported | linux-firefox-bidi-later-adapter |
| android | unknown-chromium | Android owned browser shell | manual-required | manual-required | manual-required | android-owned-browser-shell-manual-required |
| android | chrome | Android Chrome | unsupported | unsupported | unsupported | android-external-chrome-device-policy-required |
| android | firefox | Android Firefox | unsupported | unsupported | unsupported | android-firefox-later-adapter |
| ios | unknown | iOS Safari | unsupported | unsupported | unsupported | ios-safari-familycontrols-manual-required |
| ios | unknown-chromium | iOS browser app | unsupported | unsupported | unsupported | ios-browser-app-webkit-policy-boundary |

No product checklist upgrade is claimed.
Non-Windows managed exact URL and known-active tab support remain manual-required or unsupported until separate real platform proof exists.
Android emulator package-visibility proof is present; exact URL, active tab, device-owner policy, and enforcement remain unclaimed.
Android owned browser shell build/install/launch proof plus proof-launched emulator Device Owner enrollment evidence is present, but exact URL policy, known active tab, Device Owner policy mutation, VPN/DNS, UsageStats, Accessibility, and enforcement remain unclaimed.
Linux WSL package/PATH/desktop-entry boundary proof is present, but Linux desktop browser adapter, managed profile, exact URL, active tab, and enforcement remain unclaimed.
Windows host browser executable proof and default URL handler association boundary evidence are present, but managed launch, bridge custody, exact URL, active tab, and enforcement remain unclaimed.
Windows managed CDP proof is present for an Ocentra-launched managed browser profile reaching the exact local proof URL and capturing a CDP screenshot, but active-tab enforcement, final policy execution, browser blocking, and non-Windows support remain unclaimed.

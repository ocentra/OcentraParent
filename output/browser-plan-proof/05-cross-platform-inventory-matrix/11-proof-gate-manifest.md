# WP05 Platform Inventory Matrix Proof Gate

Generated: 2026-06-07T01:15:15.105Z

Rows checked: 12
Fixture-backed rows: 2
Manual-required rows: 4
Unsupported rows: 6
Product claimed: false
Android host proof: android-browser-package-visibility-proof
Linux host proof: linux-wsl-package-inventory-boundary-proof

| Platform | Browser | Product | Proof State | Exact URL | Active Tab | Reason |
| --- | --- | --- | --- | --- | --- | --- |
| windows | edge | Microsoft Edge | fixture-backed | unavailable | unavailable | windows-managed-edge-candidate |
| windows | chrome | Google Chrome | fixture-backed | unavailable | unavailable | windows-managed-chrome-candidate |
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
Android emulator package-visibility proof is present, but owned browser shell custody, exact URL, active tab, device-owner policy, and enforcement remain unclaimed.
Linux WSL package/PATH/desktop-entry boundary proof is present, but Linux desktop browser adapter, managed profile, exact URL, active tab, and enforcement remain unclaimed.

# WP05 Manual Platform Proof Boundary

Generated: 2026-06-06

This artifact records the manual platform boundary for the cross-platform
browser inventory matrix. It does not claim live non-Windows browser inventory,
managed exact URL evidence, known-active tab evidence, browser content capture,
blocking, rollback, or enforcement.

| Platform | Current proof state | Evidence boundary |
| --- | --- | --- |
| Windows | Fixture-backed in WP05; live Windows inventory proof is owned by WP04 | Windows rows remain inventory/candidate rows only until managed launch, bridge custody, exact URL, and active-tab proof exist. |
| macOS | Manual-required | No macOS app bundle scan, Chrome/Edge managed profile launch, Safari/WebKit adapter, or CDP/BiDi platform proof is captured in this artifact. |
| Linux | Manual-required | No Linux desktop entry, package manager, Snap/Flatpak, PATH, Chrome/Chromium launch, or Firefox BiDi platform proof is captured in this artifact. |
| Android | Package visibility proof present; owned-shell proof manual-required | `11-android-host-device-proof.json` records one booted Android emulator inspected through `adb`, known browser package ids queried only, Chrome package visibility, and a redacted default VIEW handler ref. It does not claim an Ocentra owned browser shell, exact URL proof, known-active tab proof, device-owner policy, VPN/DNS browser proof, UsageStats/Accessibility route proof, managed configuration, UI capture, or enforcement. |
| iOS | Unsupported/manual-required future boundary | No FamilyControls authorization, ManagedSettings token selection, Safari extension, WebKit browser app, or device proof is captured in this artifact. |

Required before product upgrade:

- Real host/device artifacts for the specific platform being upgraded.
- Adapter proof that separates inventory discovery from exact URL/tab evidence.
- Managed profile or owned shell custody proof before exact URL or active-tab claims.
- Manual rollback and failure evidence before blocking or enforcement claims.

Checklist decision: WP05 remains a no-claim matrix proof unless and until those
platform artifacts exist. Product checklist status is not upgraded by this file.

# Platform Capability Matrix

`packages/parent-domain/src/capabilities.ts` is the typed source of truth for platform claims. It is intentionally conservative: a platform can be present in CI without being marketed as fully supported.

## Current Status

- Windows: supported headless service, localhost WebSocket control, signed auto-update scaffold; LAN control is a pairing-gated preview scaffold.
- Linux: preview `.deb` and systemd service scaffold.
- macOS: preview `.pkg` and launchd service scaffold.
- Android: preview debug APK and foreground service scaffold.
- iOS: preview simulator app scaffold.

## Not Claimed Yet

- Android device-owner policy.
- iOS Family Controls entitlement.
- Google Play, Apple App Store, or Mac App Store distribution.
- Windows Authenticode signing.
- macOS Developer ID signing and notarization.
- Non-Windows updater installers.

Those items should move from `planned` to `preview-scaffold` or `supported` only when code, CI, secrets, and tests all exist.

# Android Platform

Android package scaffold and future Android child-agent/parent-mobile proof
area.

## Owns

- Android package mechanics.
- Android foreground/service scaffold.
- Android proof artifacts for install, launch, permissions, and lifecycle.
- Future Android-specific child-agent and parent-mobile wrappers.

## Must Not Own

- Generic product contracts.
- Claims of desktop-level monitoring or control without Android API proof.
- Device Owner, Accessibility, VPN/DNS, UsageStats, notification, or managed
  profile claims before each capability is proved separately.

## Required Product Proof Before Claiming Android Child-Agent Support

- APK install and launch on emulator and real device where needed.
- Foreground service lifecycle.
- Notification permission and persistent status behavior.
- UsageStats or equivalent app-activity evidence.
- AccessibilityService behavior if used.
- VPN/DNS filter behavior if used.
- Device Owner or managed-profile behavior if required for enforcement.
- LAN/WebSocket transport to the Rust/service path or approved mobile runtime.
- Play signing/store policy status before distribution claims.

## Connected Docs

- [Android/iOS platform expectations](../../docs/expectations/platforms.md)
- [Mobile agents expectations](../../docs/expectations/roadmap-v6-mobile-agents.md)
- [Android proof checkpoint](../../docs/checkpoints/child-android-device-proof-artifact-gate-2026-06-01.md)

## Gaps To Fill

- Child-agent runtime parity is manual-required.
- Store/signing and policy proof are incomplete.
- Android parent app and Android child agent must stay separate in docs and UI.

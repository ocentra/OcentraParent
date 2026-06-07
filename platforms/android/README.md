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

## Current Proof

- `mobile-child-agent-capability-proof` aggregates Android child-agent package,
  service, storage/protocol, permission, privileged, and device-gate proof rows.
- Current aggregate state is scaffold/manual-required/not-implemented: foreground
  service runtime, notifications, UsageStats, Accessibility, VPN/DNS, Device
  Owner, managed profile, Play signing, device proof, and external transport are
  not promoted to product support.
- `parent-mobile-service-bridge-proof` and
  `parent-mobile-controller-observer-handoff-proof` cover Android parent mobile
  separately from Android child-agent support. The parent-mobile proof exposes
  local-service unavailable, LAN-service degraded, cloud-relay not-implemented,
  parent-cache stale, parent-owned-storage offline, observer read-only, and
  controller-takeover manual-required states without claiming Device Owner,
  foreground child-agent runtime, Play signing, store release, or phone-local AI.
- `test:tracking-plan-android-emulator-proof` covers tracking-plan Android
  emulator package mechanics: debug APK build, install, launch, foreground
  service observation, UI tree, screenshot, logcat, battery dump, and
  connectivity dump. It records foreground location, background permission,
  foreground-service-backed background sample storage, app-owned local geofence
  enter/exit rows, `LocationManager.addProximityAlert` registration metadata,
  separate Android proximity-alert broadcast transition counters, active
  geofence-limit representation, and Android status-gap bridge proof while
  keeping Android system geofence delivery unclaimed unless those separate
  system counters are nonzero. Dwell, notification delivery, physical-device,
  Device Owner, and managed-profile behavior are not claimed/manual-required.
- `browser-platform-android-owned-shell-proof` covers the browser-plan WP05
  owned-browser-shell package boundary. It builds the separate
  `platforms/android/agent/browser-shell` APK, creates a disposable AOSP ATD
  emulator, installs `com.ocentra.parent.browser`, launches a local proof page
  through a `VIEW`/`BROWSABLE` WebView shell, observes Device Owner enrollment
  for the shell's `DeviceAdminReceiver`, observes the Device Owner app
  configuring persistent HTTP/HTTPS browser routing policy, deletes the
  temporary AVD, and records redacted UI-tree, APK, source, Device Policy
  Manager, and device refs. It does not claim exact URL policy, known active tab,
  implicit routing enforcement, VPN/DNS, UsageStats, Accessibility, browser
  blocking, broad content-filter enforcement, physical-device behavior, Play
  signing, or release readiness.

## Gaps To Fill

- Child-agent runtime parity is manual-required.
- Store/signing and policy proof are incomplete.
- Android parent app and Android child agent must stay separate in docs and UI.

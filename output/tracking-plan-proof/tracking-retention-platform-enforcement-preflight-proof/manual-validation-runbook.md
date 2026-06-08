# Tracking Retention Platform Enforcement Preflight Manual Runbook

- generatedAt: 2026-06-08T11:50:00.000Z
- status: manual_required
- This runbook is not product-ready proof. It names the required platform evidence still missing.

## android-device-policy

Acceptance criteria:
- Android device-owner or profile-owner retention policy write is observed on an enrolled child device.
- A retained location or geofence event remains queryable after the configured local retention boundary.
- The artifact includes the command, policy target, retained record id, timestamp, and platform status result.

Manual commands:
- cmd /c npm run android:device:retention -- --enrolled-child --capture-artifact tracking-retention/platform-runtime-retention-enforcement.json
- adb shell dumpsys device_policy
- adb shell dumpsys activity service ocentra

Required artifacts:
- tracking-retention/platform-runtime-retention-enforcement/android-device-policy-write.json
- tracking-retention/platform-runtime-retention-enforcement/android-retained-record-observation.json

## ios-family-controls

Acceptance criteria:
- iOS Screen Time or Family Controls retention path is exercised on an entitled child device.
- A retained location or geofence event remains queryable after the configured local retention boundary.
- The artifact includes entitlement state, policy target, retained record id, timestamp, and platform status result.

Manual commands:
- xcrun simctl list devices
- xcodebuild -scheme OcentraParentChild -destination generic/platform=iOS archive
- manual: capture entitled iOS child-device retention artifact tracking-retention/platform-runtime-retention-enforcement.json

Required artifacts:
- tracking-retention/platform-runtime-retention-enforcement/ios-family-controls-profile.json
- tracking-retention/platform-runtime-retention-enforcement/ios-retained-record-observation.json

## desktop-managed-policy

Acceptance criteria:
- Desktop managed-policy retention write is exercised through the real local service path.
- A retained location or geofence record remains queryable after the configured local retention boundary.
- The artifact includes the command, storage target, retained record id, timestamp, and service status result.

Manual commands:
- cmd /c npm run dev:agent
- cmd /c npm run test:local -- --tracking-retention-platform-enforcement
- manual: capture desktop managed-policy retention artifact tracking-retention/platform-runtime-retention-enforcement.json

Required artifacts:
- tracking-retention/platform-runtime-retention-enforcement/desktop-managed-policy-write.json
- tracking-retention/platform-runtime-retention-enforcement/desktop-retained-record-observation.json

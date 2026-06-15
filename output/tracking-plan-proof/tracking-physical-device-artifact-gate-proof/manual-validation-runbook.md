# Tracking Physical Device Manual Validation Runbook

This runbook names the physical-device artifacts required before Android or iOS tracking behavior can be claimed. File presence alone does not approve behavior; a reviewer must inspect artifact contents and keep authority, provider, production, and product-ready gates separate.

## android

- proofRoot: output/tracking-plan-proof/android-background-geofence
- status: manual-required
- missingArtifacts: 10
- physicalDeviceStatusObserved: true
- supportingStatusProofRef: test-results/tracking-android-physical-device-runtime-proof/proof.json
- supportingStatusArtifacts: 20

Acceptance criteria:
- Record a real child Android device run, not an emulator-only run.
- Capture foreground and background permission state before and after the geofence scenario.
- Show system geofence or background delivery evidence with timestamps, not only app-owned local listener rows.
- Keep product-ready false until parent-visible UI, authority, provider, and production rows are separately complete.

Manual validation commands:
- adb devices -l
- adb shell dumpsys package com.ocentra.parent.child | findstr ACCESS_BACKGROUND_LOCATION
- adb logcat -d | findstr OcentraTracking
- node scripts/test/tracking-physical-device-artifact-gate-proof.mjs

Required artifacts:
- 00-run-metadata.json
- 01-device-metadata.json
- 02-permission-state.json
- 03-geofence-definition.json
- 04-location-events.ndjson
- 05-geofence-transitions.ndjson
- 06-alert-decision.json
- 07-parent-ui-screenshot.png
- 08-logcat.txt
- 09-result-summary.md

Acceptance notes:
- Required android physical artifacts: 10.
- Artifact-set-present only means the required files exist in the proof root.
- Physical-device behavior remains unclaimed until a reviewer accepts the artifact contents.
- Product claims stay false for authority, provider delivery, production runtime, and product readiness in this gate.

Supporting status artifacts:
- test-results/tracking-android-physical-device-runtime-proof/00-device.json
- test-results/tracking-android-physical-device-runtime-proof/01-adb-connect.txt
- test-results/tracking-android-physical-device-runtime-proof/02-adb-install.txt
- test-results/tracking-android-physical-device-runtime-proof/03-prepare-device-for-launch.txt
- test-results/tracking-android-physical-device-runtime-proof/03-launch-activity.txt
- test-results/tracking-android-physical-device-runtime-proof/03-start-service.txt
- test-results/tracking-android-physical-device-runtime-proof/04-service-dump.txt
- test-results/tracking-android-physical-device-runtime-proof/05-activity-dump.txt
- test-results/tracking-android-physical-device-runtime-proof/06-window-dump.txt
- test-results/tracking-android-physical-device-runtime-proof/07-battery.txt
- test-results/tracking-android-physical-device-runtime-proof/08-connectivity.txt
- test-results/tracking-android-physical-device-runtime-proof/09-ui.xml
- test-results/tracking-android-physical-device-runtime-proof/10-screen.png
- test-results/tracking-android-physical-device-runtime-proof/11-logcat.txt
- test-results/tracking-android-physical-device-runtime-proof/12-package-dump.txt
- test-results/tracking-android-physical-device-runtime-proof/13-permission-state.json
- test-results/tracking-android-physical-device-runtime-proof/14-background-location-sample-prefs.xml
- test-results/tracking-android-physical-device-runtime-proof/15-geofence-transition-prefs.xml
- test-results/tracking-android-physical-device-runtime-proof/16-physical-route-observation.txt
- test-results/tracking-android-physical-device-runtime-proof/17-location-manager-state.txt

## ios

- proofRoot: output/tracking-plan-proof/ios-region-monitoring
- status: manual-required
- missingArtifacts: 10
- physicalDeviceStatusObserved: false
- supportingStatusProofRef: output/tracking-plan-proof/ios-region-monitoring/status-support-not-collected
- supportingStatusArtifacts: 0

Acceptance criteria:
- Record a real child iOS device run, not simulator-only proof.
- Capture Core Location authorization, Always/background state, and region monitoring registration evidence.
- Show region/significant-change/visit delivery evidence with timestamps and parent alert decision refs.
- Keep product-ready false until entitlement/review, authority, provider, and production rows are separately complete.

Manual validation commands:
- xcrun xctrace list devices
- xcodebuild test -scheme OcentraParentChildTracking -destination id=<physical-device-udid>
- xcrun simctl is not accepted for this row; attach physical-device logs instead
- node scripts/test/tracking-physical-device-artifact-gate-proof.mjs

Required artifacts:
- 00-run-metadata.json
- 01-device-metadata.json
- 02-authorization-state.json
- 03-region-definition.json
- 04-location-events.ndjson
- 05-region-transitions.ndjson
- 06-alert-decision.json
- 07-screenshots
- 08-xcode-test-log.txt
- 09-result-summary.md

Acceptance notes:
- Required ios physical artifacts: 10.
- Artifact-set-present only means the required files exist in the proof root.
- Physical-device behavior remains unclaimed until a reviewer accepts the artifact contents.
- Product claims stay false for authority, provider delivery, production runtime, and product readiness in this gate.


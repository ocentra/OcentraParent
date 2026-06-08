# Tracking Physical Device Evidence Review Runbook

Use this after the required Android/iOS physical-device artifact files exist. This runbook is a content review gate, not a product-claim approval gate.

## android

- proofRoot: output/tracking-plan-proof/android-background-geofence
- status: artifact-missing
- artifactSetComplete: false
- physicalDeviceStatusObserved: true
- supportingStatusProofRef: test-results/tracking-android-physical-device-runtime-proof/proof.json
- supportingStatusArtifacts: 13
- contentAccepted: false

Review criteria:
- Record a real child Android device run, not an emulator-only run.
- Capture foreground and background permission state before and after the geofence scenario.
- Show system geofence or background delivery evidence with timestamps, not only app-owned local listener rows.
- Keep product-ready false until parent-visible UI, authority, provider, and production rows are separately complete.

Commands to reproduce or inspect:
- adb devices -l
- adb shell dumpsys package com.ocentra.parent.child | findstr ACCESS_BACKGROUND_LOCATION
- adb logcat -d | findstr OcentraTracking
- node scripts/test/tracking-physical-device-artifact-gate-proof.mjs

Review notes:
- Required android physical artifacts: 10.
- Artifact-set-present only means the required files exist in the proof root.
- Physical-device behavior remains unclaimed until a reviewer accepts the artifact contents.
- Product claims stay false for authority, provider delivery, production runtime, and product readiness in this gate.

Supporting status artifacts:
- test-results/tracking-android-physical-device-runtime-proof/00-device.json
- test-results/tracking-android-physical-device-runtime-proof/01-adb-connect.txt
- test-results/tracking-android-physical-device-runtime-proof/02-adb-install.txt
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

## ios

- proofRoot: output/tracking-plan-proof/ios-region-monitoring
- status: artifact-missing
- artifactSetComplete: false
- physicalDeviceStatusObserved: false
- supportingStatusProofRef: output/tracking-plan-proof/ios-region-monitoring/status-support-not-collected
- supportingStatusArtifacts: 0
- contentAccepted: false

Review criteria:
- Record a real child iOS device run, not simulator-only proof.
- Capture Core Location authorization, Always/background state, and region monitoring registration evidence.
- Show region/significant-change/visit delivery evidence with timestamps and parent alert decision refs.
- Keep product-ready false until entitlement/review, authority, provider, and production rows are separately complete.

Commands to reproduce or inspect:
- xcrun xctrace list devices
- xcodebuild test -scheme OcentraParentChildTracking -destination id=<physical-device-udid>
- xcrun simctl is not accepted for this row; attach physical-device logs instead
- node scripts/test/tracking-physical-device-artifact-gate-proof.mjs

Review notes:
- Required ios physical artifacts: 10.
- Artifact-set-present only means the required files exist in the proof root.
- Physical-device behavior remains unclaimed until a reviewer accepts the artifact contents.
- Product claims stay false for authority, provider delivery, production runtime, and product readiness in this gate.


# Tracking Android Emulator Artifact Inventory Source Snapshot

- generatedAt: 2026-06-08T10:00:00.000Z
- commit: 23430858cbbbaac5ec9b646b0234c30fd9a23ef0
- requiredProofTier: P4_PHYSICAL_DEVICE
- currentProofTier: P3_LOCAL_DEV_MACHINE
- status: android-emulator-local-artifacts-present-physical-device-required
- sourceAndroidEmulatorProofRef: test-results/tracking-plan-android-emulator-proof/proof.json
- requiredArtifactCount: 12
- presentArtifactCount: 12
- missingArtifactCount: 0
- permissionUiArtifactCount: 3
- runtimeArtifactCount: 8
- localGeofenceTransitionCount: 4
- localGeofenceDwellCount: 10
- systemProximityTransitionCount: 0
- does not prove Android physical-device background behavior, Android system geofence delivery, authority enrollment, production runtime, or product readiness
- proof module: packages/parent-domain/src/tracking-android-emulator-artifact-inventory-proof.ts
- proof tests: packages/parent-domain/tests/tracking-android-emulator-artifact-inventory-proof.test.ts
- proof harness: scripts/test/tracking-android-emulator-artifact-inventory-proof.mjs

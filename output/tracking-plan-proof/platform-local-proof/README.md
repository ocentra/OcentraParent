# Tracking Platform Local Proof

Generated: 2026-06-04T03:47:06.027Z

Status: proved

This proof records the current local pre-device platform pass.

- WSL replay runs Rust tracking read-model tests inside Ubuntu 22.04.
- Android local proof uses the local Android SDK/AVD to build, install, launch, inspect UI, inspect service state, and collect logcat/screenshot artifacts when an emulator is available.
- iOS simulator proof remains manual-required on this Windows lane.

This proof does not claim Android/iOS location, background tracking, geofence delivery, physical-device behavior, authority-enrolled control, full UI accessibility, or production readiness.

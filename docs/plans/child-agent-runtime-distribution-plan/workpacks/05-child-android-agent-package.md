# Workpack 05 - Child Android Agent Package

<!-- agent-capsule -->

> Agent Capsule
> Plan: `child-agent-runtime-distribution-plan`
> Workpack: `05-child-android-agent-package`
> Kind: workpack route and proof contract.

<!-- /agent-capsule -->

Purpose: define the child Android package, install proof, and device-owner gap proof boundary.

## Owns

- Android package and install state
- child-agent lifecycle on Android
- managed-profile or device-owner gap truth
- uninstall and removal behavior on Android

## Must prove

- the package installs as a child-agent artifact on Android
- platform authority limits are explicit for managed-profile or device-owner claims
- install and launch states are honest under the chosen Android mode
- manual-required states are recorded when the platform cannot prove a capability
- no device-owner claim is made without platform evidence

## Failure conditions

- device-owner or managed-profile support is claimed without proof
- install success is treated as full authority
- Android gaps are hidden behind generic mobile wording
- parent-client distribution claims leak into this slice

## Execution truth

Status: production code drafted / test-deferred.

The Android package now owns a `ca.ocentra.child.agent` identity, a child
activity/foreground-service entrypoint, and an app-private `child-runtime/`
composition directory. Existing parent-package Android capability adapters are
deliberately retained behind the child shell. A Rust-owned JNI bridge now
initializes and queries the existing `ocentra-child-runtime` service through a
typed native boundary. The Gradle package task uses `cargo-ndk` to stage the
bridge for configured ABIs (arm64-v8a by default); missing cargo-ndk, missing
ABI output, and failed native loading remain manual-required. External
transport, device authority, install/runtime lifecycle, and store readiness
remain open.

## Code-drafted boundary

- The package identity and launcher now target the child agent.
- `ChildAgentCompositionService` owns the Android lifecycle entrypoint.
- `crates/child-runtime-android-bridge` owns the JNI exports for native start,
  readiness, domain-flow count, last error, and stop; it delegates startup to
  `ChildAgentService::initialize_with_paths`, including its durable recovery
  gate.
- `ChildAgentComposition` loads that library when present and maps native
  readiness into typed Java health. Missing library, startup failure, query
  failure, recovery pending, and revoked trust remain explicit non-ready
  states; bridge failure is `RUST_RUNTIME_MANUAL_REQUIRED`.
- Existing parent-package capability/proof adapters remain explicitly declared
  as legacy platform components behind the child shell; their proof route
  remains deferred and does not define child runtime readiness.
- The JNI bridge is a local composition boundary only. It does not expose
  network transport, package/install authority, Device Owner, managed profile,
  or Android store/signing claims.
- `platforms/android/agent/app/build.gradle` owns the cargo-ndk staging hook;
  native packaging and ABI/device validation remain deferred.

Tests, validation, proof, device-owner/managed-profile evidence, and release
readiness are deferred to the later global validation phase.

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

## Live source truth

Status: source partial; implementation correction and all test/validation/proof gates remain open.

The Android app uses `ca.ocentra.child.agent`, a child activity, a foreground composition service, app-private durable paths, a Rust JNI bridge, and a cargo-ndk staging hook. These are real package/runtime source boundaries.

The JNI bridge calls `ChildAgentService::initialize_with_paths(ChildAgentServicePaths::from_root(...))`, which supplies no `ChildAgentTrustBindingSource`. Startup therefore remains fail-closed/manual-required. Java exposes local Binder health only and `ChildAgentComposition` reports transport as `NOT_IMPLEMENTED`; no authenticated product ingress exists. Device Owner, managed profile, store authority, and platform removal integration are also absent.

## Required production source outcome

- consume WP10's reviewed current-trust startup and authenticated ingress/health boundary through JNI without copying or minting trust;
- keep foreground lifecycle, native load/ABI failures, and restart/stop states observable;
- own Android device-owner/managed-profile/manual-required state and removal callback boundaries explicitly;
- preserve app-private custody and canonical child identity.

Implementation dependency: Child WP10 reviewed implementation. Normal READY/DONE remains strict.

## Expected test-source gap

- correct the existing bridge test that expects `Ready` without a trust source;
- prove missing/stale/revoked trust stays non-ready and current trust can reach ready;
- cover JNI load/start/query/stop, foreground restart, authenticated ingress, and health;
- cover ABI packaging, install/launch/remove, device-owner/managed-profile, store, and manual-required states on appropriate targets;
- reject parent identity and fake transport/device authority.

Historical Android protocol/capability proof and debug APK artifacts do not establish trusted startup, transport, Device Owner, managed profile, store, removal, or release completion.

# iOS Platform

iOS simulator/package scaffold and future iOS parent-mobile/child-agent proof
area.

## Owns

- iOS package mechanics.
- Simulator launch proof.
- Future iOS entitlement, signing, TestFlight, and approved API proof records.
- Future iOS parent-mobile and child-agent wrappers where Apple APIs allow them.
- iOS parent mobile scaffold app under `platforms/ios/OcentraParentMobile`.

## Must Not Own

- Claims of background monitoring or enforcement without approved API proof.
- Family Controls, DeviceActivity, Managed Settings, Network Extension,
  notification, or background claims before entitlement/device proof.
- Generic contracts that belong in TypeScript domain packages.

## Required Product Proof Before Claiming iOS Child-Agent Support

- Apple developer signing/provisioning.
- Family Controls entitlement where needed.
- DeviceActivity and Managed Settings proof where screen/app control is claimed.
- Network Extension proof where network filtering is claimed.
- Notification/background behavior proof.
- TestFlight or real-device install proof.
- Parent-visible unavailable/degraded states for capabilities Apple does not
  allow.

## Connected Docs

- [iOS platform expectations](../../docs/expectations/platforms.md)
- [Mobile agents expectations](../../docs/expectations/roadmap-v6-mobile-agents.md)
- [iOS entitlement checkpoint](../../docs/checkpoints/child-ios-entitlement-capability-proof-2026-05-31.md)

## Current Proof

- `mobile-child-agent-capability-proof` aggregates iOS child-agent simulator,
  entitlement, signing, TestFlight/device, App Store, and external-transport
  proof rows.
- Current aggregate state is simulator/manual-required/planned: Family Controls,
  DeviceActivity, Screen Time, Network Extension, notifications/background,
  signing, TestFlight, App Store, device proof, and external transport are not
  promoted to product support.
- `parent-mobile-service-bridge-proof` and
  `parent-mobile-controller-observer-handoff-proof` cover iOS parent mobile
  separately from iOS child-agent support. The parent-mobile proof keeps iOS as
  controller-candidate/manual-required, LAN/provider routing unavailable,
  cloud-relay not-implemented, parent-cache stale, parent-owned-storage offline,
  local model execution disabled, and signing/TestFlight/device proof
  unclaimed.
- `release:package:parent-ios` builds the separate
  `ca.ocentra.parent.mobile` parent mobile simulator app. Its package preview is
  real build/install/launch evidence for the parent app scaffold only; it is not
  controller authority, background execution, signing, TestFlight, App Store, or
  child-agent parity proof.

## Gaps To Fill

- Current support is scaffold/simulator proof, not child-agent parity.
- Entitlement and real-device proof are manual-required.
- Parent iOS app and child iOS agent must remain separate source, package, and
  product claims.

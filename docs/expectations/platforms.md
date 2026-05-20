# Platform Expectations

Platform claims must match real OS capabilities. Scaffolded support is not the same as product support.

## Windows

- First production-grade agent target.
- Service, MSI, process/window capture, network observation, local policy, and enforcement are expected here first.
- Windows-specific adapters must stay behind platform boundaries.

## macOS

- Scaffold and package preview are useful early.
- Capture/enforcement claims require real permission/API proof.
- Do not assume Windows service behavior maps to launchd behavior without tests.

## Linux

- Useful for CI, package proof, and future desktop support.
- Do not assume Windows capture adapters apply.
- Service-manager package behavior must be tested separately.

## Android

- SQLite is the expected local query store.
- Use platform-approved foreground/device-management capabilities.
- Do not claim desktop-level control unless device-owner policy or equivalent is actually implemented.

## iOS

- Most restrictive target.
- Use Apple-approved capabilities and entitlements only.
- Do not claim background monitoring or enforcement beyond proven APIs.

## Web

- Parent portal and control surface only.
- Does not run the child-device agent.
- Does not run child-device AI, policy evaluation, enforcement, timers, capture adapters, or scripts.
- Talks to local, LAN, or cloud-routed agents through typed service contracts.

## Done Signal

Every platform-facing feature says exactly which platform behavior is implemented, which behavior is scaffold-only, and what validation proves the claim.

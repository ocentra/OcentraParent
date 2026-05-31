# Child Android Permission Package Capability Proof

Date: 2026-05-31

Roadmap slice: Child Android permission and package capability proof.

## Scope

- Adds a typed `@ocentra-parent/parent-domain` read model for Child Android permission/package capability proof.
- Adds `ChildAndroidPermissionCapabilityProof` to the Android native wrapper package.
- Proves manifest-declared foreground service and notification permissions, package-local permission proof Bundle wiring, debug APK/checksum output, and app-private storage path scaffold.
- Records UsageStats as settings-grant-required, POST_NOTIFICATIONS as manual-runtime-required, accessibility and VPN/DNS as not implemented, and device-owner plus managed profile as blocked until device/enrollment proof exists.

## Proof Command

```powershell
npm run test:child-android-permission-capability-proof
```

Expected artifact:

```text
test-results/child-android-permission-capability-proof/proof.json
```

## What This Proves

- Android permission/package bridge constants compile into the debug APK.
- `MainActivity` and `OcentraParentAgentService` surface the package-local permission proof bundle.
- Parent-domain accepts the honest permission/package read model.
- Parent-domain rejects dishonest upgrades for automatic notification grants, UsageStats availability, accessibility/VPN declarations, device-owner support, and install lifecycle proof.
- Debug APK and SHA-256 checksum artifacts are produced by the repo package script.

## Non-Claims

- No emulator or physical-device runtime proof.
- No Android child enforcement parity.
- No notification runtime grant or delivery proof.
- No UsageStats settings grant or observation.
- No AccessibilityService, VPN/DNS, device-owner, or managed-profile behavior.
- No install, update, background, reboot, or uninstall lifecycle behavior.
- No LAN/WebSocket child-agent permission transport.

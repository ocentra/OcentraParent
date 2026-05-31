# Child Android Protocol Package Lifecycle Proof

Date: 2026-05-31

Roadmap slice: Child Android native wrapper protocol bridge and package lifecycle capability proof.

## Scope

- Adds a typed `@ocentra-parent/parent-domain` read model for Child Android lifecycle proof.
- Ties the read model to the Android native wrapper through `ChildAndroidLifecycleProof`.
- Proves debug APK build and checksum mechanics through `npm run release:package:android`.
- Records capability-specific Android truth for foreground service, notification permission, local storage, typed protocol bridge, UsageStats, accessibility, VPN/DNS, device owner, managed profile, package lifecycle, and store distribution.

## Proof Command

```powershell
npm run test:child-android-protocol-package-lifecycle-proof
```

Expected artifact:

```text
test-results/child-android-protocol-package-lifecycle-proof/proof.json
```

## What This Proves

- Android package-local lifecycle bridge constants compile into the debug APK.
- Launcher activity starts the foreground service scaffold.
- Foreground service and notification permissions are declared in the manifest.
- Debug APK and SHA-256 checksum artifacts are produced by the repo package script.
- Parent-domain rejects dishonest upgrades for external transport, device-owner behavior, notification runtime grant, and install/update lifecycle.

## Non-Claims

- No Android enforcement parity.
- No device-owner, accessibility, VPN/DNS, UsageStats, managed-profile, emulator, or physical-device behavior.
- No install, update, background, reboot, or uninstall lifecycle proof.
- No Google Play signing or store distribution.
- No LAN/WebSocket child-agent protocol transport from the Android package.

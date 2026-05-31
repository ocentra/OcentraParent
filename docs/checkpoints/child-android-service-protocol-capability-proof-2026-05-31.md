# Child Android Service Protocol Capability Proof

Date: 2026-05-31

Roadmap slice: Child Android foreground service/status protocol capability proof.

## Scope

- Adds a typed `@ocentra-parent/parent-domain` read model for Child Android service/protocol proof.
- Adds `ChildAndroidServiceProtocolProof` to the Android native wrapper package.
- Proves foreground service status, storage bridge reference, status export fields, and capability labels compile into the debug APK.
- Records UsageStats as permission-required, accessibility and VPN/DNS as unavailable, and device-owner plus managed profile as blocked until device/enrollment proof exists.

## Proof Command

```powershell
npm run test:child-android-service-protocol-capability-proof
```

Expected artifact:

```text
test-results/child-android-service-protocol-capability-proof/proof.json
```

## What This Proves

- Android service/protocol bridge constants compile into the debug APK.
- `MainActivity` and `OcentraParentAgentService` surface the package-local service proof bundle.
- Parent-domain accepts the honest service/protocol read model.
- Parent-domain rejects dishonest upgrades for external transport, remote status export, UsageStats implementation, and device-owner availability.
- Debug APK and SHA-256 checksum artifacts are produced by the repo package script.

## Non-Claims

- No emulator or physical-device foreground service runtime proof.
- No Android child enforcement parity.
- No UsageStats permission grant or observation.
- No AccessibilityService, VPN/DNS, device-owner, or managed-profile behavior.
- No remote status export, hosted child activity upload, or LAN/WebSocket child-agent service transport.

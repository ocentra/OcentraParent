# Child iOS Entitlement Capability Proof - 2026-05-31

## Scope

Worker D added a child iOS entitlement/package capability proof that keeps the current iOS target honest as a simulator/package scaffold. The proof covers the Xcode app target, bundle identifier, Info.plist, status surface, simulator package script, typed parent-domain read model, and pre-AI proof matrix entry.

## Proof Command

```powershell
npm run test:child-ios-entitlement-capability-proof
```

The command builds TypeScript contracts, runs the focused parent-domain iOS entitlement proof test, inspects the iOS package scaffold source, validates the proof matrix wiring, and writes:

```text
test-results/child-ios-entitlement-capability-proof/proof.json
```

## Proved States

- `ca.ocentra.parent.agent` remains the iOS bundle identifier in the Xcode project.
- `AgentStatusViewController` exposes status labels for the iOS scaffold and the manual-required capability states.
- The iOS Info.plist exists as a basic app plist and does not declare background modes, Family Controls, DeviceActivity, or Network Extension capability claims.
- The simulator package script still targets `iphonesimulator` with `CODE_SIGNING_ALLOWED=NO`.
- The typed parent-domain read model rejects upgraded entitlement, signing, TestFlight, device, or external transport claims without artifacts.

## Non-Claims And Manual Requirements

- No Family Controls entitlement approval, DeviceActivity schedule, Screen Time authorization, Network Extension filtering, notification grant/delivery, or background execution behavior is claimed.
- No Apple signing, provisioning profile, entitlement file, TestFlight install, App Store distribution, physical-device install, or device runtime behavior is claimed.
- No child-agent parity or external LAN/WebSocket iOS transport is claimed.

## Manual Proof Checklist

- Archive the generated proof JSON with the commit SHA.
- Run the iOS simulator package build on a Mac and archive xcodebuild output plus generated package checksums before upgrading simulator build status.
- Record Apple signing team, provisioning profile, and entitlement approval state before upgrading Family Controls, DeviceActivity, Screen Time, Network Extension, notifications, or background execution.
- Run TestFlight or physical-device install proof and archive device behavior before upgrading TestFlight, App Store, or device support states.
- Add a real child iOS runtime bridge before claiming child-agent parity or external transport.

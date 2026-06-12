<!-- agent-capsule -->

> Agent Capsule
> Doc: Child iOS Entitlement Capability Proof - 2026-05-31
> Kind: historical checkpoint/proof documentation; read only when CHECKPOINT_INDEX or PROOF_INDEX names it.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

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

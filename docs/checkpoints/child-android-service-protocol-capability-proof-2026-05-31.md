<!-- agent-capsule -->

> Agent Capsule
> Doc: Child Android Service Protocol Capability Proof
> Kind: historical checkpoint/proof documentation; read only when CHECKPOINT_INDEX or PROOF_INDEX names it.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

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

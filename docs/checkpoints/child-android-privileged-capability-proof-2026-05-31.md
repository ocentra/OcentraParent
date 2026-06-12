<!-- agent-capsule -->

> Agent Capsule
> Doc: Child Android Privileged Capability Proof
> Kind: historical checkpoint/proof documentation; read only when CHECKPOINT_INDEX or PROOF_INDEX names it.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

# Child Android Privileged Capability Proof

Date: 2026-05-31

Roadmap slice: Child Android privileged capability proof.

## Scope

- Adds a typed `@ocentra-parent/parent-domain` read model for Child Android privileged capability proof.
- Adds `ChildAndroidPrivilegedCapabilityProof` to the Android native wrapper package.
- Proves package-local privileged status Bundle wiring in `MainActivity` and `OcentraParentAgentService`.
- Records UsageStats as settings/manual-device proof, Accessibility and VPN/DNS as not declared or not implemented, device-owner and managed profile as blocked, physical-device behavior as device-proof-required, and external child-agent transport as not implemented.

## Proof Command

```powershell
npm run test:child-android-privileged-capability-proof
```

Expected artifact:

```text
test-results/child-android-privileged-capability-proof/proof.json
```

## What This Proves

- Android privileged capability bridge constants compile into the debug APK.
- `MainActivity` and `OcentraParentAgentService` surface the package-local privileged capability proof bundle.
- Parent-domain accepts the honest privileged capability read model.
- Parent-domain rejects dishonest upgrades for UsageStats availability, Accessibility/VPN/DNS implementation, device-owner or managed-profile enrollment, physical-device proof, and external child-agent transport.
- Debug APK and SHA-256 checksum artifacts are produced by the repo package script.

## Non-Claims

- No emulator or physical-device runtime proof.
- No Android child enforcement parity.
- No UsageStats settings grant or observation.
- No AccessibilityService declaration, user grant, or behavior.
- No VpnService, DNS adapter, or filtering behavior.
- No device-owner or managed-profile enrollment or policy action.
- No LAN/WebSocket child-agent privileged transport.

## Manual Upgrade Checklist

- Archive `test-results/child-android-privileged-capability-proof/proof.json` with the commit SHA.
- Install the APK on an emulator or physical device and record package install output before upgrading device proof.
- Record UsageStats settings grant and observed usage events before upgrading UsageStats support.
- Record AccessibilityService declaration, user grant, and behavior before upgrading accessibility support.
- Record VpnService/DNS adapter declaration, user grant, and filtering behavior before upgrading VPN/DNS support.
- Record device-owner or managed-profile enrollment artifacts before upgrading policy states.
- Record child-agent parity and external transport artifacts before claiming remote child privileged control.

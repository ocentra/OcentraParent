<!-- agent-capsule -->

> Agent Capsule
> Doc: Child Android Permission Package Capability Proof
> Kind: historical checkpoint/proof documentation; read only when CHECKPOINT_INDEX or PROOF_INDEX names it.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

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

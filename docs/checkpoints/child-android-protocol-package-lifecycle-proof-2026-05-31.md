<!-- agent-capsule -->

> Agent Capsule
> Doc: Child Android Protocol Package Lifecycle Proof
> Kind: historical checkpoint/proof documentation; read only when CHECKPOINT_INDEX or PROOF_INDEX names it.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

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

<!-- agent-capsule -->

> Agent Capsule
> Doc: Child Android Storage Protocol Capability Proof
> Kind: historical checkpoint/proof documentation; read only when CHECKPOINT_INDEX or PROOF_INDEX names it.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

# Child Android Storage Protocol Capability Proof

Date: 2026-05-31

Roadmap slice: Child Android package-local storage/protocol capability proof.

## Scope

- Adds a typed `@ocentra-parent/parent-domain` read model for Child Android storage/protocol proof.
- Adds `ChildAndroidStorageProtocolProof` to the Android native wrapper package.
- Proves storage protocol command/event constants compile into the debug APK.
- Records storage surfaces separately: app-private files, encrypted evidence journal, SQLite query store, parent-owned export, Ocentra-hosted child activity storage, and protocol snapshot.
- Keeps hosted child activity storage not-default and unimplemented.

## Proof Command

```powershell
npm run test:child-android-storage-protocol-capability-proof
```

Expected artifact:

```text
test-results/child-android-storage-protocol-capability-proof/proof.json
```

## What This Proves

- Android package-local storage bridge constants compile into the debug APK.
- Parent-domain accepts the honest storage/protocol read model.
- Parent-domain rejects dishonest upgrades for external storage transport, durable encrypted journal, hosted default storage, and app-private files presented as durable evidence storage.
- Debug APK and SHA-256 checksum artifacts are produced by the repo package script.

## Non-Claims

- No Android storage persistence proof on emulator or physical device.
- No encrypted evidence journal runtime behavior.
- No SQLite query-store runtime behavior.
- No parent-owned export runtime behavior.
- No Ocentra-hosted default child activity storage.
- No LAN/WebSocket child-agent storage transport from the Android package.
- No Android child enforcement parity.

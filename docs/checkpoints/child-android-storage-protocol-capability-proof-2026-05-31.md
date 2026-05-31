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

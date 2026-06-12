<!-- agent-capsule -->

> Agent Capsule
> Doc: Platform Capability Matrix
> Kind: architecture/reference documentation; read only when selected by plan route, source router, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# Platform Capability Matrix

`packages/parent-domain/src/capabilities.ts` is the typed source of truth for platform claims. It is intentionally conservative: a platform can be present in CI without being marketed as fully supported.

## Current Status

- Windows: supported headless service, localhost WebSocket control, signed auto-update scaffold; LAN control is a pairing-gated preview scaffold.
- Linux: preview `.deb` and systemd service scaffold.
- macOS: preview `.pkg` and launchd service scaffold.
- Android: preview debug APK and foreground service scaffold.
- iOS: preview simulator app scaffold.

## Not Claimed Yet

- Android device-owner policy.
- iOS Family Controls entitlement.
- Google Play, Apple App Store, or Mac App Store distribution.
- Windows Authenticode signing.
- macOS Developer ID signing and notarization.
- Non-Windows updater installers.

Those items should move from `planned` to `preview-scaffold` or `supported` only when code, CI, secrets, and tests all exist.

# Child Agent Artifact Matrix

<!-- agent-capsule -->

> Agent Capsule
> Plan: `child-agent-runtime-distribution-plan`
> Doc: `CHILD_AGENT_ARTIFACT_MATRIX.md`
> Kind: plan reference document.

<!-- /agent-capsule -->

| Artifact | Committed source | Source/reachability gap | Expected test gap | What it must not claim |
| --- | --- | --- | --- | --- |
| Child Windows package | Child binary, builder, MSI, and WinSW child values. | No shipped trust source, authenticated ingress, external health, or fully canonical child source identity. | Child-labelled elevated lifecycle/respawn plus trusted startup/health. | Parent distribution, setup ownership, or static-declaration respawn. |
| Child macOS package | Child binary/package paths and launchd child values. | Parent-labelled plist source; no trust source/ingress/health/signing/notarization completion. | Real-host launchd/signing/restart/remove/health. | Notarization, lifecycle, or parent-client parity from package output. |
| Child Linux package | Child `.deb`, binary/path, and systemd child values. | Parent-labelled unit source; no trust source/ingress/health/fail-closed lifecycle/signing-feed completion. | Baseline-host health/crash/restart/remove/cleanup and identity. | Generic distro, signed feed, or respawn readiness. |
| Child Android package | Canonical child id, foreground service, JNI bridge, and native staging hook. | JNI omits trust source; Binder health is local; transport/device-owner/removal integration is absent. | Trust-currentness, JNI/foreground/ingress/health/removal/device-authority. | Debug APK, Device Owner, managed profile, store, or transport completion. |
| Child iOS capability package | Capability/limit contracts, canonical `OcentraChildAgent` project/product/scheme/app identity, `ca.ocentra.child.agent` bundle id, and child-named simulator artifact source. | Smoke/workflow inputs, Apple signing/provisioning, physical-device launch, TestFlight/App Store ownership, and runtime parity remain open. | Child-identity build/smoke checks and simulator/device capability-limit coverage. | Daemon, background-service, recovery, supervision, signing, store, or parent parity. |
| Parent-authorized removal | Durable revocation/reauthorization state and audit boundary. | No production Account-authority caller, platform cleanup callback, or cleanup receipt. | Authority/replay/restart and platform cleanup/idempotency. | Stealth persistence, child self-authorization, or platform cleanup from revocation state alone. |
| Child updater/handoff | Windows signed-manifest/hash/installer and typed setup/device-trust projection. | Projection is unused; no durable handoff delivery/replay, scheduler integration, or non-Windows update owner. | Handoff/update/install/restart plus platform signing/store outcomes. | Setup, trust, installed health, or cross-platform completion. |

## Matrix rules

- Proof is collected per artifact, not per folder.
- Source/runtime reachability is reviewed before proof is regenerated.
- Parent client distribution is separate.
- Mobile rows must show manual-required gaps honestly.
- Service respawn and uninstall resistance are platform-specific claims.

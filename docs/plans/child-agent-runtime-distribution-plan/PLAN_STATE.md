# Child Agent Runtime Distribution Plan State

Status: live-code review complete; production source is partial and no workpack is release-complete. Package shells, durable child-runtime custody, bounded in-process ingress, Android JNI composition, removal state, the Windows updater, typed setup/device-trust handoff projection, and canonical iOS child source identity exist. Shipped startup authority, authenticated external ingress, externally reachable health, platform lifecycle completion, removal callbacks, live handoff/update composition, iOS test/workflow/signing/device/store closure, and the executable aggregate release gate remain missing.

Program phase: source routing only. Tests, builds, proof regeneration, precommit, CI, PR, and completion claims are deferred. Existing proof is historical review input, not authority for the current source state.

Reviewed iOS source packet: `c71becbcfd4f07eb98a118f10dbf261320f6b54e`, integrated into the source consolidation branch before the expected-test wave.

## Code-backed workpack state

| Workpack | Committed production source | Production source still required | Expected test source still required | Runtime/caller reachability | Source order |
| --- | --- | --- | --- | --- | --- |
| WP01 | Route and ownership boundary only; no product source is expected. | Keep route ownership aligned with the executable graph. | Route/index consistency checks only. | Not a runtime workpack. | First, route-only. |
| WP02 | Windows builder, MSI authoring, WinSW definitions, child service binary, and child artifact/service values exist. | Canonical child-owned source identities plus trusted startup, authenticated ingress, and external health composition are not shipped. | Child startup/readiness tests and child-labelled elevated install/start/stop/restart/uninstall/respawn tests are missing. | Installed binary starts without a trust-binding source and has no production command or health client. | After WP10 implementation source. |
| WP03 | macOS builder and launchd payload target the child binary and child runtime paths. | Canonical child-owned plist/source identity, signing/notarization inputs, trusted startup, health, and lifecycle completion remain missing. | Real-host launchd, signing/notarization, restart, disable, uninstall, and health tests are missing; existing smoke inputs remain legacy-labelled. | Launchd can start the binary, but the binary remains trust-manual-required and externally unreachable. | After WP10; may run beside WP02/WP04/WP05. |
| WP04 | Linux builder, child `.deb` values, child systemd unit contents, and child service paths exist. | Canonical child-owned unit/source identity, fail-closed service lifecycle, trusted startup, external health, signing/feed ownership, and cleanup completion remain missing. | Child-labelled package smoke plus real-host service health, crash/restart, disable/remove, distro, and cleanup tests are missing. | systemd can start the binary, but no trusted binding, command transport, or health endpoint is composed. | After WP10; may run beside WP02/WP03/WP05. |
| WP05 | `ca.ocentra.child.agent`, the foreground composition service, JNI bridge, app-private custody, and native staging hook exist. | JNI startup does not supply Device Trust authority; authenticated ingress, usable external health, device-owner/managed-profile authority, and platform removal integration remain missing. | Bridge tests must cover manual-required-without-trust and current-trust startup; Android instrumentation/lifecycle/ingress/removal tests are missing. | Binder health is local and transport is explicitly `NOT_IMPLEMENTED`; the bridge currently starts without a trust source. | After WP10. |
| WP06 | Rust capability/limit contracts plus canonical `OcentraChildAgent` project/product/scheme/app, `ca.ocentra.child.agent`, and child-named simulator release source exist. | Smoke/workflow consumers, Apple signing/provisioning, physical-device launch, TestFlight/App Store authority, and retained proof remain open. | Child-identity build/smoke checks and simulator/device capability-limit tests are missing. | Capability-only; no daemon, persistent service, or runtime parity exists. | Source packet reviewed; expected-test wave next. |
| WP07 | Windows/macOS/Linux manager declarations contain restart policy for the child binary. | A live cross-platform lifecycle state boundary, health-aware supervision, bounded restart/backoff truth, deliberate-stop handling, and platform callbacks are missing. | Per-platform kill/reboot/manager-restart/disable/teardown/loop-guard tests are missing. | Static manager declarations exist; no production observer consumes service health or proves respawn. | After WP02-WP06 and WP10 implementation source. |
| WP08 | Durable removal state, current-authority token construction, identity checks, revocation/reauthorization, tamper evidence, and readiness gating exist. | No production authority caller invokes the removal API; platform package/device cleanup callbacks and durable cleanup receipts are missing. | Authority mismatch/replay/restart tests and real platform cleanup callback/idempotency tests are missing. | Public removal APIs have no production caller outside `child-runtime`; platforms remain manual-required. | After Account WP08 authority source, WP10, and WP07. |
| WP09 | Windows manifest/signature/hash/installer source and a typed platform matrix exist. | Updater scheduling/production handoff composition is absent and macOS/Linux/Android/iOS signing/store/update owners remain manual or missing. | Updater install/update/handoff/restart tests and platform-specific signing/store/device-owner tests are missing; current updater coverage is only partial. | The updater CLI can execute the Windows update path, but it never consumes WP10's handoff projection. | After WP02-WP06 and WP10 implementation source. |
| WP10 | Shared request/response schema and a pure updater outcome projection exist. | No setup producer, durable delivery/replay owner, live updater consumer, trusted child startup source, authenticated ingress adapter, or external health endpoint is composed. | Trust-source/currentness, startup recovery, authenticated ingress, health, handoff replay/expiry, updater callback, and crash/restart integration tests are missing. | `consume_setup_device_trust_handoff` has no production caller; desktop and Android startup construct paths with no trust source. | First runtime source packet, gated by reviewed Device Trust WP01 implementation; never depend back on Setup WP07. |
| WP11 | Documentation/proof aggregation exists. | No executable child-plan aggregate source gate joins canonical identity, startup, lifecycle, removal, updater/handoff, and platform truth. | Aggregate negative-fixture and release-blocker tests are missing. | No production or release workflow consumes a single executable WP11 result. | Last, after WP01-WP10 implementation source. |

The compact plan/workpack routing counts are routing state only. Mapped files, contracts, proof folders, or checked boxes do not mean the production intent is complete.

## Dependency and ownership interpretation

```text
Device Trust WP01
  -> Child WP10 trusted startup / authenticated ingress / health / handoff consumption
     -> Child WP02, WP03, WP04, WP05 platform packages
        -> Child WP07 lifecycle supervision
           -> Child WP08 platform removal callbacks

Child WP06 canonical iOS child identity
  -> Child WP07 and WP09

Account WP08 current household authority
  -> Child WP08 production removal caller

Child WP02-WP06 + WP10
  -> Child WP09 updater/signing/store/device-owner completion

Child WP01-WP10
  -> Child WP11 executable aggregate gate
```

- `setup-install-provisioning-plan` owns the setup producer and UI journey. Its WP07 already consumes Child WP10, so Child WP10 must not add a reverse hard dependency and create a cycle.
- `device-trust-bootstrap-plan` WP01 owns current child-device trust material. Child WP10 owns the shipped adapter that consumes that authority at child startup.
- `account-identity-family-plan` WP08 owns current verified household authority. Child WP08 consumes it and must not mint or infer parent authorization.
- This plan owns child artifacts and installed child runtime distribution. Parent-client proof cannot close any child package or runtime row.

## First coherent source packets

1. WP06: correct the actual iOS product/project/bundle/release identity to the child boundary while preserving capability-only limits.
2. WP10: compose the current Device Trust source into child startup, provide authenticated ingress and external health ownership, and connect typed setup/update handoff consumption without taking Setup ownership.
3. WP02-WP05: finish each platform package against the WP10 runtime edge; keep platform work disjoint.
4. WP07, then WP08 and WP09: add lifecycle observation/callback ownership, authorized removal cleanup, and updater/platform completion.
5. WP11: add the executable aggregate gate only after every preceding production source packet exists.

Normal graph `READY`/`DONE` remains strict. An implementation-only edge may authorize a later source packet only after its dependency's reviewed implementation roots exist; it does not satisfy tests, proof, checklist, CI, review, or merge gates.

## Current coupling risks

- Static package scripts and manager declarations are not installed runtime reachability.
- `ChildAgentService::initialize()` and the Android bridge both construct paths without a trust-binding source, so readiness stays fail-closed/manual-required.
- `ChildAgentIngress` is an in-process queue, not an authenticated product transport.
- Health is a Rust/Android local API, not a shipped external health endpoint.
- Removal authority types are sound only at the boundary; no production caller or OS cleanup callback currently consumes them.
- The setup/device-trust projection is pure and unused by the updater CLI.
- iOS and several smoke/workflow paths still use parent identities; old proof cannot upgrade those production gaps.

## Proof interpretation

```text
Package build is not install readiness.
Install proof is not trusted startup or service health.
Service health is not authenticated ingress.
Manager restart declarations are not respawn proof.
Revocation state is not platform cleanup.
Updater manifest validation is not live setup/device-trust handoff consumption.
Capability contracts are not a canonical child application identity.
WP11 documentation is not an executable aggregate release gate.
```

## HID execution guard

- Follow `PLAN_EXECUTION_BLUEPRINT.md`, then `WORKPACK_INDEX.md`, then `NEXT_ACTIONS.md`.
- Select only a graph-legal source packet and claim its exact files before editing.
- Do not mark any workpack complete from mapped files, checklist deltas, or historical proof.
- After the source wave, add the expected test source, then run focused validation, then regenerate proof, then run precommit/CI/PR gates.

## HID execution blueprint

Continue execution from: [PLAN_EXECUTION_BLUEPRINT.md](PLAN_EXECUTION_BLUEPRINT.md).
Update this plan only via the blueprint and matching workpack checklist.

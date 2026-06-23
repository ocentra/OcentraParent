<!-- agent-capsule -->

> Agent Capsule
> Plan: `child-agent-runtime-distribution-plan`
> Doc: `Child Runtime Distribution Workpack Families`
> Kind: owner-path classifier for selected workpacks.
> Read when: only after `WORKPACK_INDEX.md` selects or names a workpack and the owner path is unclear.
> Stop rule: classify the selected workpack only; do not use this file as permission to scan every workpack in the family.
> Proves: routing and owner-path classification only.
> Does not prove: package readiness, runtime readiness, platform support, setup readiness, or PR readiness.
> Proof rule: if this file changes route/status claims, update `AGENTS.md`, `PLAN_STATE.md`, and any affected selected workpack route.

<!-- /agent-capsule -->

# Child Runtime Distribution Workpack Families

Use this file to classify a selected workpack before opening source. This plan proves child artifact distribution boundaries. It does not prove parent client distribution, setup journey completion, policy/enforcement behavior, AI behavior, portal display, LAN transport, or broad child runtime feature readiness.

## Scope, route, and separation family

```text
Workpacks:
WP01 Child Agent Scope And Route Boundary

Owners:
docs/plans/child-agent-runtime-distribution-plan
schema-domain for shared handoff/package/trust shapes
child-runtime-domain for package-boundary metadata only

Rule:
Scope proof must separate child artifacts from parent client artifacts, setup journey state, device trust state, and runtime behavior. Route cleanup is not package or runtime proof.
```

## Platform package artifact family

```text
Workpacks:
WP02 Child Windows Service Package
WP03 Child macOS Service Package
WP04 Child Linux Service Package
WP05 Child Android Agent Package
WP06 Child iOS Capability Package

Owners:
scripts/release for artifact build/checksum/package output
schema-domain for shared artifact/signing/platform-capability shapes
platform-specific package scripts and manifests when selected

Rule:
Artifact build proves only that the selected package artifact and metadata were produced. It does not prove install, launch, service health, respawn, uninstall, transport, setup readiness, policy, enforcement, or store distribution unless the selected proof root contains that exact evidence.
```

## Runtime and service lifecycle family

```text
Workpacks:
WP02 Windows service lifecycle slices
WP03 macOS launchd/service slices
WP04 Linux service-manager slices
WP05 Android foreground-service/package lifecycle slices
WP06 iOS lifecycle/capability slices

Owners:
child local-service/runtime owners for runtime behavior
agent-service/agent-protocol only when service/protocol proof is selected
this plan for package-to-runtime distribution proof boundaries

Rule:
Package installation is not service readiness. Runtime lifecycle proof must state installed state, service manager or platform lifecycle state, start/stop/restart behavior, health signal, degraded state, and artifact/build provenance.
```

## Mobile capability, device-owner, managed-profile, and supervision family

```text
Workpacks:
WP05 Child Android Agent Package
WP06 Child iOS Capability Package
WP09 Child Signing Store Device Owner Matrix

Owners:
Android/iOS platform proof scripts and manifests when selected
schema-domain for capability/device-owner/managed-profile/supervision states
device-trust-bootstrap-plan when trusted-device material is involved

Rule:
Android debug APK proof is not device-owner or managed-profile proof. iOS simulator/provisioning proof is not background-service or supervision parity. Keep privileged/mobile capabilities manual-required until real device/platform artifacts prove them.
```

## Managed respawn and supervision family

```text
Workpacks:
WP07 Child Managed Service Respawn
platform-specific parts of WP02-WP06

Owners:
platform service manager proof for Windows/macOS/Linux where supported
Android/iOS platform capability proof where lifecycle restrictions apply
child local-service/runtime owners for runtime health semantics

Rule:
Respawn proof must name platform service manager, restart policy, backoff/loop guard, crash/stop/reboot behavior, degraded state, and limits. Respawn is not uninstall resistance, setup completion, or runtime feature readiness.
```

## Parent-authorized uninstall and revocation family

```text
Workpacks:
WP08 Child Parent Authorized Uninstall
platform-specific uninstall slices in WP02-WP06

Owners:
this plan for child package removal/revocation proof
account-identity-family-plan for actor/household/role authority when selected
device-trust-bootstrap-plan for trust revocation material when selected
platform package manager proof when selected

Rule:
Parent-authorized uninstall is an explicit custody/removal/revocation flow. It is not hidden persistence. Proof must show authorized actor, target device/artifact, revocation state, cleanup/removal result, residual-state note, and manual-required platform gaps.
```

## Signing, store, and artifact custody family

```text
Workpacks:
WP09 Child Signing Store Device Owner Matrix
WP11 Proof CI Release Gate
platform package workpacks when signing state is selected

Owners:
scripts/release for produced artifacts/checksums/SBOMs when selected
release/signing infrastructure owners when signing is selected
schema-domain for artifact/signing/store/device-owner matrix shapes

Rule:
Checksum/SBOM/signing proof is not install, runtime, service health, store approval, device-owner enrollment, or policy capability proof. Each artifact row must state unsigned/debug/signed/notarized/store/manual-required explicitly.
```

## Setup-device-trust handoff family

```text
Workpacks:
WP10 Setup Device Trust Handoff
WP01 route boundary when setup/trust scope is disputed

Owners:
setup-install-provisioning-plan for setup journey and UI flow
device-trust-bootstrap-plan for trusted-device bootstrap and sealed/local trust
this plan for typed setup-to-child-install distribution handoff consumption
schema-domain for shared setup-trust-handoff shapes

Rule:
Setup handoff must be typed request/response state, not a loose UI transition. Setup success is not package readiness unless artifact, platform, install, runtime, and trust handoff proof exist for the selected claim.
```

## Proof, CI, and release gate family

```text
Workpacks:
WP11 Proof CI Release Gate
selected platform package workpacks when aggregating proof

Owners:
selected proof roots under output/child-agent-runtime-distribution-plan-proof/<workpack>/
CI/release docs only when release status changes

Rule:
No PR_READY from package scripts alone, parent client proof, stale legacy proof paths, empty proof directories, debug/mobile scaffold proof, or manual-required rows. WP11 can aggregate only proof roots that already contain structured artifacts and no-claim boundaries.
```

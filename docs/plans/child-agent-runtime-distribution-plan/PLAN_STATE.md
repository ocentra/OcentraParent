# Child Agent Runtime Distribution Plan State

Status: production code drafted for the child executable boundary and WP02/WP03/WP04 desktop package retargeting; tests, validation, and proof are intentionally deferred to the later global phase. WP05 remains a separate Android runtime gap, WP06 remains capability-only, WP08 remains contract-only, WP09 remains matrix/release alignment, WP10 remains a typed handoff, and WP11 remains proof-gate work.

Global phase boundary: this pass changes production/core code only. No tests, proof artifacts, Enforcer validation scans/checks, precommit, CI, or PR claims are made from this lane.

Research status: aligned against the current repo child-service/runtime surface, the existing package scripts for Windows/macOS/Linux/Android/iOS distribution, and the separated parent-client runtime distribution route. The parent client plan now owns the parent artifact boundary; this plan owns the child artifact boundary.

## Current ownership interpretation

```text
crates/schema or the owning Rust crate:
  Canonical shared child package, child runtime, platform capability, device-owner, managed-profile, supervision, artifact, signing, setup-trust-handoff, and release-gate shapes when they cross package, crate, app, or plan boundaries.

schema-domain:
  Temporary thin/generated validation and edge-decoder surface only where TypeScript still consumes Rust-owned contracts during migration.

child-runtime-domain:
  Child runtime package-boundary metadata/helper surface. Shared child runtime contracts live in Rust-owned schema surfaces.

scripts/release:
  Artifact build/checksum/signing-package proof only for the selected platform.

agent-protocol and agent-service:
  Runtime/protocol proof only when the selected workpack names child runtime, service health, package lifecycle protocol, or service-manager proof.

setup-install-provisioning-plan:
  Setup journey owner. This plan consumes typed setup-to-child-install handoff state only.

device-trust-bootstrap-plan:
  Trusted-device bootstrap, local sealed trust, and device trust material owner.

parent-client-runtime-distribution-plan:
  Parent client artifact/distribution owner. Parent proof cannot close child artifact rows.

Policy, enforcement, AI, portal, notification, LAN, remote, account, payment, and data-custody plans:
  Adjacent sibling owners or handoff consumers. They must not re-own child package/distribution truth.
```

## Current coupling risks

```text
- Package scripts exist, but package-script presence is not install/runtime/readiness proof.
- Android debug APK proof remains package-local proof unless real device-owner, managed-profile, runtime, transport, or store artifacts are produced.
- iOS simulator/provisioning proof remains capability/provisioning proof unless real supervision/background-service limits are proven.
- Setup-device-trust handoff is not setup journey completion and cannot close package rows alone.
- Parent client proof cannot close child package, child runtime, child service, respawn, or uninstall/revocation rows.
- Stale legacy proof paths still conflict with the current output proof-root route and must not be used for status bumps.
```

## Current proof interpretation

```text
Package build is not install readiness.
Install proof is not service health.
Service health is not respawn proof.
Respawn proof is not uninstall/revocation proof.
Uninstall resistance is not hidden persistence.
Checksum/signing/SBOM proof is not store approval or platform lifecycle proof.
Setup handoff proof is not setup journey completion.
Device-owner, managed-profile, supervision, and privileged mobile states remain manual-required until platform artifacts prove them.
WP11 can aggregate only proof roots with structured artifacts and no-claim boundaries.
```

Evidence from the repo:

- `release:package:windows`, `release:package:linux`, `release:package:macos`, `release:package:android`, and `release:package:ios` already exist as child distribution anchors.
- `test:child-android-protocol-package-lifecycle-proof` already exists as a child proof anchor.
- The existing managed-service proof route is retained for the later validation phase; this pass does not run or rely on it.
- `docs/features/child-agent-local-service.md` describes the child runtime/service boundary that distribution must package honestly.
- The repo already differentiates child-service/runtime ownership from parent-client packaging, but the plan naming still needs the child route.

Current child direction:

- Windows production package code now targets the child executable and child service identity; MSI lifecycle, elevated-host execution, and respawn proof remain deferred.
- macOS production package code now targets the child executable and child launchd identity; signing, install/runtime, and restart proof remain deferred.
- Linux production package code now targets the child executable, child `.deb` identity, and child systemd paths; package lifecycle, distro, service-health, and crash-recovery proof remain deferred.
- Android rows now prove package state, explicit `debug-apk-sideload` mode, and manual-required install/launch/removal plus device-owner/managed-profile gaps honestly; real device/runtime/store artifacts remain open.
- WP09 now has a real Rust-owned shared contract under `crates/schema/src/child_signing_store_device_owner_matrix.rs`, a checked-in generated TS contract under `packages/schema-domain/src/generated/child-signing-store-device-owner-matrix-contracts.ts`, and a thin schema-domain adapter/proof pack under `output/child-agent-runtime-distribution-plan-proof/09-child-signing-store-device-owner-matrix/`.
- WP06 now has a real Rust-owned shared contract under `crates/schema/src/child_ios_entitlement_capability_proof.rs`, a checked-in generated TS contract under `packages/schema-domain/src/generated-child-ios-entitlement-capability-proof-contracts.ts`, a thin schema-domain adapter at `packages/schema-domain/src/child-ios-entitlement-capability-proof.ts`, and a real proof pack under `output/child-agent-runtime-distribution-plan-proof/06-ios-entitlement-capability-proof/`; its focused Rust contract/build/proof/runner/architecture/type-check validations are now green while keeping capability-only, provisioning-limit, supervision-limit, and no-daemon/no-parity boundaries explicit.
- Managed respawn configuration is drafted for Windows WinSW, macOS launchd, and Linux systemd; kill/reboot/service-manager restart behavior remains unvalidated. Android remains manual-required; iOS remains unsupported.
- Parent-authorized uninstall now has its own contract/read-model proof slice with explicit parent-authorization, revocation-audit, teardown, and residual-state boundaries; unsupported or manual-required platform uninstall control remains visible rather than implied.
- Setup-device-trust handoff now has a real Rust-owned contract proof pack under `output/child-agent-runtime-distribution-plan-proof/10-setup-device-trust-handoff/`, with explicit request/response refs, external artifact pointer, route-sync rows, and no-claim boundaries that keep setup/trust/package/runtime states separate.
- WP01 now has a real scope-and-route proof pack under `output/child-agent-runtime-distribution-plan-proof/01-child-agent-scope-and-route-boundary/`, with explicit Rust-first ownership, historical parent-client compatibility note, and no-claim boundaries between package build, install, runtime health, respawn, uninstall/revocation, setup trust, and release readiness.
- WP11 now has a real aggregate proof gate under `output/child-agent-runtime-distribution-plan-proof/11-proof-ci-release-gate/`, and it intentionally records that the child plan is not PR-ready or release-ready while WP02 remains blocked on exact local lifecycle proof gaps.

Open gaps:

- WP02 now has a real proof pack under `output/child-agent-runtime-distribution-plan-proof/02-child-windows-service-package/`, and the latest rerun at `test-results/windows-package-lifecycle-proof/2026-06-28T20-18-36-351Z/proof.json` kept install/start/stop/restart/uninstall/respawn execution honestly blocked and manual-required on a non-elevated host.
- WP04 is production-code drafted under the child `.deb` + `systemd` package boundary; package lifecycle, distro, service-health, and crash-recovery validation remain deferred.
- WP03 is production-code drafted under the child launchd package boundary; install/runtime/restart, signing, notarization, and uninstall validation remain deferred.
- WP08 now has a real proof pack under `output/child-agent-runtime-distribution-plan-proof/08-child-parent-authorized-uninstall/`, and it intentionally limits claims to contract/read-model truth plus explicit no-claim boundaries rather than platform uninstall-control parity.
- WP05 now has a real proof pack under `output/child-agent-runtime-distribution-plan-proof/05-child-android-agent-package/`, but it intentionally stops at package-only proof and manual-required device/runtime/authority states.
- WP07 is production-manager configuration drafted under the child desktop identity; live installed runtime health and respawn validation remain deferred.
- Proof docs still disagree between the `output/child-agent-runtime-distribution-plan-proof/...` route and stale legacy proof-path references.

## HID execution guard

- Follow `PLAN_EXECUTION_BLUEPRINT.md`, then `WORKPACK_INDEX.md`, then `NEXT_ACTIONS.md`.
- Do not mark this plan complete from checklist deltas alone.
- Before any checked update, attach a real test run log or explicit blocker under the selected `output/child-agent-runtime-distribution-plan-proof/<workpack-file-stem>/` root.
- Failure rule: no PR-ready claim until package lifecycle, tamper/uninstall, and setup-device-trust handoff proofs are present for the assigned slice.

## HID execution blueprint

Continue execution from: [PLAN_EXECUTION_BLUEPRINT.md](PLAN_EXECUTION_BLUEPRINT.md).
Update this plan only via the blueprint and matching workpack checklist.

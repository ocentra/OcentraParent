# Child Agent Runtime Distribution Plan State

Status: production code drafted for the child service composition boundary, durable startup recovery, typed readiness, bounded desktop ingress, WP02/WP03/WP04 desktop package retargeting, the WP05 Android child composition, Rust/JNI bridge, and cargo-ndk packaging hook, the WP08 durable parent-authorized revocation boundary, and the WP10 package/update consumer projection; tests, validation, and proof are intentionally deferred to the later global phase. Android ABI/device packaging validation and transport remain explicit downstream gaps. WP06 remains capability-only, WP09 remains matrix/release alignment, and WP11 remains proof-gate work.

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

- The child runtime now owns a durable composition service with explicit journal/tombstone paths, startup recovery, typed readiness, and a bounded observed-event ingress API; network transport is not implemented here.
- The child runtime now owns a durable parent-authorized revocation boundary under `removal-state.json`; only a bound, existing verified household-authority contract can construct its action-specific removal token, revoked trust is surfaced as typed readiness, ingress is closed, and package/device removal remains manual-required where OS authority is external.
- Windows MSI service/file removal now preserves the child ProgramData custody root for parent/admin audit review; Linux/macOS hooks retain the same manual platform-removal boundary, and Android remains manual-required.
- The deferred Windows lifecycle harness is stale against this custody-preserving boundary (it expects ProgramData absence and retains legacy parent labels); no proof claim is made until that proof-only route is reconciled.
- Windows production package code now targets the child composition executable and child service identity; MSI lifecycle, elevated-host execution, and respawn proof remain deferred.
- macOS production package code now targets the child composition executable and child launchd identity; signing, install/runtime, and restart proof remain deferred.
- Linux production package code now targets the child composition executable, child `.deb` identity, and child systemd paths; package lifecycle, distro, service-health, and crash-recovery proof remain deferred.
- Android production code now drafts the `ca.ocentra.child.agent` identity, child-owned composition foreground service, app-private composition paths, `crates/child-runtime-android-bridge` JNI startup/readiness projection over `ChildAgentService`, and a cargo-ndk Gradle staging hook; missing tool/ABI/library, load/start/query failures, recovery pending, and revoked trust remain explicit non-ready states while legacy parent-package capability adapters stay behind the child shell. External transport, install/runtime, device-owner/managed-profile, and store validation remain deferred.
- WP09 now has production Windows child updater/package enforcement under `crates/agent-updater/src/manifest.rs`, `hash.rs`, `update.rs`, and the Windows release scripts: child-only identity, canonical signed payload verification, strict SHA-256/artifact validation, random updater-owned temporary custody, external-key-only normal signing, and fail-closed bootstrap verifier/key requirements. The Rust-owned platform matrix and its proof remain deferred.
- Deferred updater contract fixtures still use legacy parent service/package identities and must be reconciled before validation; the bootstrap requires externally provisioned verifier executable and public key.
- WP06 now has a real Rust-owned shared contract under `crates/schema/src/child_ios_entitlement_capability_proof.rs`, a checked-in generated TS contract under `packages/schema-domain/src/generated-child-ios-entitlement-capability-proof-contracts.ts`, a thin schema-domain adapter at `packages/schema-domain/src/child-ios-entitlement-capability-proof.ts`, and a real proof pack under `output/child-agent-runtime-distribution-plan-proof/06-ios-entitlement-capability-proof/`; its focused Rust contract/build/proof/runner/architecture/type-check validations are now green while keeping capability-only, provisioning-limit, supervision-limit, and no-daemon/no-parity boundaries explicit.
- Managed respawn configuration is drafted for Windows WinSW, macOS launchd, and Linux systemd; kill/reboot/service-manager restart behavior remains unvalidated. Android remains manual-required; iOS remains unsupported.
- WP08 now has a production child-service revocation/audit boundary plus its existing contract/read-model slice; platform uninstall/device-owner cleanup, tests, validation, and proof remain deferred and unsupported/manual-required control remains visible rather than implied.
- Setup-device-trust handoff now has a real Rust-owned contract proof pack under `output/child-agent-runtime-distribution-plan-proof/10-setup-device-trust-handoff/`, with explicit request/response refs, external artifact pointer, route-sync rows, and no-claim boundaries that keep setup/trust/package/runtime states separate.
- WP10 production code now drafts `crates/agent-updater/src/handoff.rs` as the typed consumer of the setup-owned response plus updater outcome. It preserves response identity/artifact/platform/no-claim state, fails closed on manual/expired/inconsistent setup-trust fields, and exposes current, dry-run, completed, reboot-required, and failed package-update outcomes without claiming install, setup, trust, service health, transport, or runtime readiness. No setup producer or live transport wiring exists yet.
- WP01 now has a real scope-and-route proof pack under `output/child-agent-runtime-distribution-plan-proof/01-child-agent-scope-and-route-boundary/`, with explicit Rust-first ownership, historical parent-client compatibility note, and no-claim boundaries between package build, install, runtime health, respawn, uninstall/revocation, setup trust, and release readiness.
- WP11 now has a real aggregate proof gate under `output/child-agent-runtime-distribution-plan-proof/11-proof-ci-release-gate/`, and it intentionally records that the child plan is not PR-ready or release-ready while WP02 remains blocked on exact local lifecycle proof gaps.

Open gaps:

- WP02 now has a real proof pack under `output/child-agent-runtime-distribution-plan-proof/02-child-windows-service-package/`, and the latest rerun at `test-results/windows-package-lifecycle-proof/2026-06-28T20-18-36-351Z/proof.json` kept install/start/stop/restart/uninstall/respawn execution honestly blocked and manual-required on a non-elevated host.
- WP04 is production-code drafted under the child `.deb` + `systemd` package boundary; package lifecycle, distro, service-health, and crash-recovery validation remain deferred.
- WP03 is production-code drafted under the child launchd package boundary; install/runtime/restart, signing, notarization, and uninstall validation remain deferred.
- WP08 has production revocation/audit code drafted under `crates/child-runtime/src/removal.rs` and `service.rs`; contract/read-model proof, package/device cleanup artifacts, tests, and validation remain deferred, with no platform uninstall-control parity claim.
- WP05 is production-code drafted under the child Android identity, local composition boundary, Rust/JNI bridge, and cargo-ndk packaging hook; ABI/device packaging validation, package/install/runtime, transport, device-authority, store validation, tests, and proof remain deferred.
- WP07 is production-manager configuration drafted under the child desktop identity; live installed runtime health and respawn validation remain deferred.
- WP10 package/update consumer wiring is production-code drafted under `crates/agent-updater`; setup-producer integration, install callback, transport, tests, validation, and proof remain deferred.
- Proof docs still disagree between the `output/child-agent-runtime-distribution-plan-proof/...` route and stale legacy proof-path references.

## HID execution guard

- Follow `PLAN_EXECUTION_BLUEPRINT.md`, then `WORKPACK_INDEX.md`, then `NEXT_ACTIONS.md`.
- Do not mark this plan complete from checklist deltas alone.
- Before any checked update, attach a real test run log or explicit blocker under the selected `output/child-agent-runtime-distribution-plan-proof/<workpack-file-stem>/` root.
- Failure rule: no PR-ready claim until package lifecycle, tamper/uninstall, and setup-device-trust handoff proofs are present for the assigned slice.

## HID execution blueprint

Continue execution from: [PLAN_EXECUTION_BLUEPRINT.md](PLAN_EXECUTION_BLUEPRINT.md).
Update this plan only via the blueprint and matching workpack checklist.

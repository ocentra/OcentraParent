# Child Agent Runtime Distribution Plan State

Status: canonical child runtime distribution scope documented; implementation and proof remain open.

Research status: aligned against the current repo child-service/runtime surface, the existing package scripts for Windows/macOS/Linux/Android/iOS distribution, and the separated parent-client runtime distribution route. The parent client plan now owns the parent artifact boundary; this plan owns the child artifact boundary.

## Current ownership interpretation

```text
schema-domain:
  Canonical shared child package, child runtime, platform capability, device-owner, managed-profile, supervision, artifact, signing, setup-trust-handoff, and release-gate shapes when they cross package, crate, app, or plan boundaries.

child-runtime-domain:
  Child runtime package-boundary metadata/helper surface. Shared child runtime contracts live in schema-domain.

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
- `docs/features/child-agent-local-service.md` describes the child runtime/service boundary that distribution must package honestly.
- The repo already differentiates child-service/runtime ownership from parent-client packaging, but the plan naming still needs the child route.

Current child direction:

- Windows, macOS, and Linux rows must prove package/service lifecycle behavior separately.
- Android rows must prove package state, device install state, and device-owner or managed-profile gaps honestly.
- iOS rows must prove distribution/provisioning state honestly and keep service-limit gaps visible.
- Managed respawn and parent-authorized uninstall need their own platform-specific proof.
- Setup-device-trust handoff must stay separate from package distribution.

Open gaps:

- Artifact matrix and signing/device-owner claims need explicit per-platform rows.
- Tamper/uninstall and managed respawn proof still need separate slices.
- iOS capability and manual-required state still need explicit wording.
- WP01 and WP10 remain open; no proof artifacts exist under their declared `output/child-agent-runtime-distribution-plan-proof/...` roots.
- Proof docs still disagree between the `output/child-agent-runtime-distribution-plan-proof/...` route and stale legacy proof-path references.
- Android and iOS proof runners still reference missing parent-domain test ownership paths instead of the existing `packages/child-runtime-domain/tests/unit/...` tests.
- Focused child proof commands still fail before slice validation when unrelated workspace `build:contracts` dependencies break.

## HID execution guard

- Follow `PLAN_EXECUTION_BLUEPRINT.md`, then `WORKPACK_INDEX.md`, then `NEXT_ACTIONS.md`.
- Do not mark this plan complete from checklist deltas alone.
- Before any checked update, attach a real test run log or explicit blocker under the selected `output/child-agent-runtime-distribution-plan-proof/<workpack-file-stem>/` root.
- Failure rule: no PR-ready claim until package lifecycle, tamper/uninstall, and setup-device-trust handoff proofs are present for the assigned slice.

## HID execution blueprint

Continue execution from: [PLAN_EXECUTION_BLUEPRINT.md](PLAN_EXECUTION_BLUEPRINT.md).
Update this plan only via the blueprint and matching workpack checklist.

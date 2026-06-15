# Child Agent Runtime Distribution Plan State

Status: canonical child runtime distribution scope documented; implementation and proof remain open.

Research status: aligned against the current repo child-service/runtime surface, the existing package scripts for Windows/macOS/Linux/Android/iOS distribution, and the separated parent-client runtime distribution route. The parent client plan now owns the parent artifact boundary; this plan owns the child artifact boundary.

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

- Canonical child plan route still needs to be reflected in the global indexes.
- Artifact matrix and signing/device-owner claims need explicit per-platform rows.
- Tamper/uninstall and managed respawn proof still need separate slices.
- iOS capability and manual-required state still need explicit wording.

## HID execution guard

- Follow `PLAN_EXECUTION_BLUEPRINT.md`, then `WORKPACK_INDEX.md`, then `NEXT_ACTIONS.md`.
- Do not mark this plan complete from checklist deltas alone.
- Before any checked update, attach a real test run log or explicit blocker from the assigned boundary and a proof manifest under `docs/proof/child-agent-runtime-distribution-plan/`.
- Failure rule: no PR-ready claim until package lifecycle, tamper/uninstall, and setup-device-trust handoff proofs are present for the assigned slice.

## HID execution blueprint

Continue execution from: [PLAN_EXECUTION_BLUEPRINT.md](PLAN_EXECUTION_BLUEPRINT.md).
Update this plan only via the blueprint and matching workpack checklist.

# Parent Client Runtime Distribution Plan State

Status: canonical parent client distribution scope documented; implementation and proof remain open.

Research status: aligned against the current repo parent client surfaces, the existing desktop/mobile proof scripts, and the runtime-distribution guidance in the pasted apply set. The historical `parent-desktop-runtime-package-plan` path is retained for compatibility only.

## Current ownership interpretation

```text
apps/portal:
  Parent web portal source/projection surface and web distribution target when selected.

portal-domain:
  Public portal contracts/projections when selected.

parent-domain:
  Parent client package/handoff contracts only where public exports exist and the selected workpack names them.

scripts/dev:
  Parent desktop dev launch helpers and local launch proof anchors.

scripts/release:
  Build/package/proof helpers for selected parent desktop/mobile artifacts.

setup-install-provisioning-plan:
  Setup journey, install readiness, first-run state, and setup-side handoff owner.

child-agent-runtime-distribution-plan:
  Child package/runtime, child package lifecycle, tamper/uninstall, and child-specific artifact owner.

device-trust-bootstrap-plan:
  Trusted-device bootstrap, local sealed trust, and parent presence/approval owner.

account-identity-family-plan, payment-subscription-plan, policy-control-plane-plan, remote-access-plan, and data-custody-storage-plan:
  Sibling owners for account, payment, policy, remote access, and custody behavior.
```

## Current coupling risks

```text
- The folder path still says desktop, but the canonical scope is parent client distribution.
- WP01 text says proof was recorded, while WORKPACK_INDEX still marks WP01 open; keep open until proof root, checklist, and state are aligned.
- Web build proof is not production account portal readiness.
- Desktop launch smoke is not desktop product readiness.
- Mobile scaffold or source artifact proof is not Android/iOS platform support.
- Package artifact proof is not setup completion.
- Route bridge proof is not child-agent runtime authority.
- Signing, notarization, store, update, rollback, SBOM, and launch claims must remain per artifact/platform.
```

## Current proof interpretation

```text
output/parent-client-runtime-distribution-plan-proof/<workpack>/ is the canonical proof root.
docs/proof/parent-desktop-runtime-package-plan/ is compatibility-only for old references and should not become the active proof root.
All WP01-WP11 rows remain open until proof artifacts and checklist rows align.
Manual-required states are expected for unsupported or unavailable platform/store proof.
```

Evidence from the repo:

- `apps/portal` is the parent web portal surface.
- `dev:desktop` and `dev:desktop:lan` already exist for the parent desktop shell.
- `release:package:parent-android` and `release:package:parent-ios` already exist for parent mobile packaging.
- `test:parent-mobile-shell-runtime-proof`, `test:parent-mobile-package-source-artifact-proof`, `test:parent-desktop-release-support-proof`, `test:parent-mobile-service-bridge`, and `test:parent-mobile-controller-observer-handoff` already exist as proof anchors.
- The repo already differentiates parent client surfaces from child runtime work, but the plan naming still read as desktop-only before this correction.
- The plan-local route docs now keep canonical parent-client scope, route bridge, setup handoff, and child-runtime exclusions separate.

Current parent direction:

- Parent web portal is a distribution target with build, route, auth, and cache proof.
- Parent desktop shell/package needs explicit artifact, signing, update, rollback, and launch proof.
- Parent Android and iOS packages are manual-required until device and store proof exists.
- Parent client route bridge contracts must be separate from setup journey and child runtime claims.
- Child agent runtime/package distribution belongs to `child-agent-runtime-distribution-plan`; this plan may only reference its handoff boundary.

Open gaps:

- Parent client artifact matrix is missing from the old desktop-only plan.
- Signing/store/notarization states are not explicit per artifact.
- Update/rollback model is not explicit per artifact.
- Setup handoff contracts are not explicit in a single source.
- Child-agent distribution proof is owned by `child-agent-runtime-distribution-plan`, not this plan.
- WP01 status needs reconciliation between workpack text, proof artifacts, and `WORKPACK_INDEX.md`.

## HID execution guard

- Follow `PLAN_EXECUTION_BLUEPRINT.md`, then `WORKPACK_INDEX.md`, then `NEXT_ACTIONS.md`.
- Use `WORKPACK_FAMILIES.md` only when owner/proof family is unclear.
- Do not mark this plan complete from checklist deltas alone.
- Before any checked update, attach a real test run log or explicit blocker from the assigned boundary and a proof manifest under `output/parent-client-runtime-distribution-plan-proof/<workpack>/`.
- Failure rule: no PR-ready claim until replay/idempotency, authZ/replay, and rollback/teardown proofs are present for the assigned slice.

## HID execution blueprint

Continue execution from: [PLAN_EXECUTION_BLUEPRINT.md](PLAN_EXECUTION_BLUEPRINT.md).
Update this plan only via the blueprint and matching workpack checklist.

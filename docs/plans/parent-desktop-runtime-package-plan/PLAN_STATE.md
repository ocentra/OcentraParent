# Parent Client Runtime Distribution Plan State

Status: canonical parent client distribution scope documented; implementation and proof remain open.

Research status: aligned against the current repo parent client surfaces, the existing desktop/mobile proof scripts, and the runtime-distribution guidance in the pasted apply set. The historical `parent-desktop-runtime-package-plan` path is retained for compatibility only.

Evidence from the repo:

- `apps/portal` is the parent web portal surface.
- `dev:desktop` and `dev:desktop:lan` already exist for the parent desktop shell.
- `release:package:parent-android` and `release:package:parent-ios` already exist for parent mobile packaging.
- `test:parent-mobile-shell-runtime-proof`, `test:parent-mobile-package-source-artifact-proof`, `test:parent-desktop-release-support-proof`, `test:parent-mobile-service-bridge`, and `test:parent-mobile-controller-observer-handoff` already exist as proof anchors.
- The repo already differentiates parent client surfaces from child runtime work, but the plan naming still read as desktop-only before this correction.

Current parent direction:

- Parent web portal is a distribution target with build, route, auth, and cache proof.
- Parent desktop shell/package needs explicit artifact, signing, update, rollback, and launch proof.
- Parent Android and iOS packages are manual-required until device and store proof exists.
- Parent client route bridge contracts must be separate from setup journey and child runtime claims.
- Child agent runtime/package distribution now belongs to a separate plan.

Open gaps:

- Canonical name correction still needs to be reflected in the plan index and route docs.
- Parent client artifact matrix is missing from the old desktop-only plan.
- Signing/store/notarization states are not explicit per artifact.
- Update/rollback model is not explicit per artifact.
- Setup handoff contracts are not explicit in a single source.
- Child-agent distribution still needs its own plan and proof root.

## HID execution guard

- Follow `PLAN_EXECUTION_BLUEPRINT.md`, then `WORKPACK_INDEX.md`, then `NEXT_ACTIONS.md`.
- Do not mark this plan complete from checklist deltas alone.
- Before any checked update, attach a real test run log or explicit blocker from the assigned boundary and a proof manifest under `docs/proof/parent-desktop-runtime-package-plan/` for compatibility.
- Failure rule: no PR-ready claim until replay/idempotency, authZ/replay, and rollback/teardown proofs are present for the assigned slice.

## HID execution blueprint

Continue execution from: [PLAN_EXECUTION_BLUEPRINT.md](PLAN_EXECUTION_BLUEPRINT.md).
Update this plan only via the blueprint and matching workpack checklist.

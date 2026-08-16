<!-- agent-capsule -->

> Agent Capsule
> Plan: `parent-client-runtime-distribution-plan`
> Doc: `Parent Client Runtime Distribution Checklist Index`
> Kind: exact checklist router.
> Read when: a selected workpack references checklist rows.
> Stop rule: do not scan unrelated checklist rows.
> Proves: checklist routing only.
> Does not prove: implementation completion.
> Proof rule: a checkbox can be checked only after proof artifacts and focused command results exist.

<!-- /agent-capsule -->

# Parent Client Runtime Distribution Checklist Index

> **Live-code audit (2026-07-17):** [Project Progress Matrix](../../PLAN_CODE_STATUS_MATRIX.md) records current implementation, blockers, dependencies, and next unblocker. Rows remain proof-gated; this audit does not check unsupported work.

## Fill rules

- Leave a checkbox unchecked until proof exists.
- Every checked row must cite one or more proof artifacts from `PROOF_INDEX.md`.
- Every proof item must list exact commands run, pass/fail/blocker, and no-claim boundaries.
- Do not mark a platform or artifact ready from scaffold, launch smoke, or package-preview alone.
- Do not mark PR_READY until WP11 aggregates proof from all required earlier workpacks.

## WP01 Parent Client Scope And Route Boundary

- [x] Canonical parent-client scope documented. Proof: `output/parent-client-runtime-distribution-plan-proof/01-parent-client-scope-and-route-boundary/00-scope-summary.md`.
- [x] Historical desktop-only path compatibility documented. Proof: `output/parent-client-runtime-distribution-plan-proof/01-parent-client-scope-and-route-boundary/00-scope-summary.md`.
- [x] Child agent runtime excluded. Proof: `output/parent-client-runtime-distribution-plan-proof/01-parent-client-scope-and-route-boundary/01-negative-case-proof.md`.
- [x] Setup-install handoff documented. Proof: `output/parent-client-runtime-distribution-plan-proof/01-parent-client-scope-and-route-boundary/00-scope-summary.md`, `output/parent-client-runtime-distribution-plan-proof/01-parent-client-scope-and-route-boundary/02-manual-required-gap-register.md`.
- [x] Portal shell UX handoff documented. Proof: `output/parent-client-runtime-distribution-plan-proof/01-parent-client-scope-and-route-boundary/00-scope-summary.md`.
- [x] No product-readiness overclaim recorded. Proof: `output/parent-client-runtime-distribution-plan-proof/01-parent-client-scope-and-route-boundary/01-negative-case-proof.md`.
- [x] Required proof artifacts written. Proof: `output/parent-client-runtime-distribution-plan-proof/01-parent-client-scope-and-route-boundary/00-scope-summary.md`, `output/parent-client-runtime-distribution-plan-proof/01-parent-client-scope-and-route-boundary/01-negative-case-proof.md`, `output/parent-client-runtime-distribution-plan-proof/01-parent-client-scope-and-route-boundary/02-manual-required-gap-register.md`, `output/parent-client-runtime-distribution-plan-proof/01-parent-client-scope-and-route-boundary/16-validation-commands.log`.
- [x] Focused commands pass or blocker recorded. Proof: `output/parent-client-runtime-distribution-plan-proof/01-parent-client-scope-and-route-boundary/16-validation-commands.log`.
- [x] Workpack completion section filled. Proof: `output/parent-client-runtime-distribution-plan-proof/01-parent-client-scope-and-route-boundary/00-scope-summary.md`.
- [x] PLAN_STATE updated if state changed. Proof: `output/parent-client-runtime-distribution-plan-proof/01-parent-client-scope-and-route-boundary/00-scope-summary.md`, `output/parent-client-runtime-distribution-plan-proof/01-parent-client-scope-and-route-boundary/16-validation-commands.log`.

## WP02 Parent Web Portal Distribution

- [x] Parent web build path defined. Proof: `output/parent-client-runtime-distribution-plan-proof/02-parent-web-portal-distribution/00-scope-summary.md`, `output/parent-client-runtime-distribution-plan-proof/02-parent-web-portal-distribution/16-validation-commands.log`.
- [x] Route ownership documented. Proof: `output/parent-client-runtime-distribution-plan-proof/02-parent-web-portal-distribution/00-scope-summary.md`.
- [x] Auth/cache/env separation documented. Proof: `output/parent-client-runtime-distribution-plan-proof/02-parent-web-portal-distribution/00-scope-summary.md`, `output/parent-client-runtime-distribution-plan-proof/02-parent-web-portal-distribution/01-negative-case-proof.md`.
- [x] Hosted route does not claim child-agent execution. Proof: `output/parent-client-runtime-distribution-plan-proof/02-parent-web-portal-distribution/00-scope-summary.md`, `output/parent-client-runtime-distribution-plan-proof/02-parent-web-portal-distribution/02-manual-required-gap-register.md`.
- [x] Build/test proof exists or blocker recorded. Proof: `output/parent-client-runtime-distribution-plan-proof/02-parent-web-portal-distribution/16-validation-commands.log`.
- [x] Deployment/preview blocker recorded if missing. Proof: `output/parent-client-runtime-distribution-plan-proof/02-parent-web-portal-distribution/02-manual-required-gap-register.md`.
- [x] Required proof artifacts written. Proof: `output/parent-client-runtime-distribution-plan-proof/02-parent-web-portal-distribution/00-scope-summary.md`, `output/parent-client-runtime-distribution-plan-proof/02-parent-web-portal-distribution/01-negative-case-proof.md`, `output/parent-client-runtime-distribution-plan-proof/02-parent-web-portal-distribution/02-manual-required-gap-register.md`, `output/parent-client-runtime-distribution-plan-proof/02-parent-web-portal-distribution/16-validation-commands.log`.
- [x] Workpack completion section filled. Proof: `output/parent-client-runtime-distribution-plan-proof/02-parent-web-portal-distribution/00-scope-summary.md`.
- [x] No production web-ready overclaim made. Proof: `output/parent-client-runtime-distribution-plan-proof/02-parent-web-portal-distribution/01-negative-case-proof.md`, `output/parent-client-runtime-distribution-plan-proof/02-parent-web-portal-distribution/02-manual-required-gap-register.md`.
- [x] Setup handoff remains separate. Proof: `output/parent-client-runtime-distribution-plan-proof/02-parent-web-portal-distribution/02-manual-required-gap-register.md`.
- [x] Parent data/source labels preserved. Proof: `output/parent-client-runtime-distribution-plan-proof/02-parent-web-portal-distribution/00-scope-summary.md`.
- [x] Route sync updated if state changed. Proof: `output/parent-client-runtime-distribution-plan-proof/02-parent-web-portal-distribution/00-scope-summary.md`, `output/parent-client-runtime-distribution-plan-proof/02-parent-web-portal-distribution/16-validation-commands.log`.

## WP03 Parent Desktop Shell Package

- [ ] Desktop shell package boundary defined.
- [ ] Local service bridge boundary defined.
- [ ] Launch smoke proof exists or blocker recorded.
- [ ] Installer/package artifact proof exists or blocker recorded.
- [ ] Signing/notarization state is explicit.
- [ ] Update/rollback handoff is explicit.
- [ ] Required proof artifacts written.
- [ ] Workpack completion section filled.
- [ ] No desktop-ready overclaim from launch smoke.
- [ ] Setup completion is not claimed.
- [ ] Manual-required gaps visible.
- [ ] Focused commands pass or blocker recorded.

## WP04 Parent Android Package

- [ ] Android parent package boundary defined.
- [ ] Build/install state recorded.
- [ ] Store/manual-required state recorded.
- [ ] Device proof exists or blocker recorded.
- [ ] Parent app and child agent claims separated.
- [ ] Required proof artifacts written.
- [ ] Workpack completion section filled.
- [ ] No Android parity overclaim from scaffold.
- [ ] Platform/store limitations visible.
- [ ] Setup handoff recorded.
- [ ] Focused commands pass or blocker recorded.
- [ ] PLAN_STATE updated if state changed.

## WP05 Parent iOS Package

- [ ] iOS parent package boundary defined.
- [ ] Simulator/device/TestFlight/App Store states separated.
- [ ] Signing/provisioning state recorded.
- [ ] Device proof exists or blocker recorded.
- [ ] Parent app and child agent claims separated.
- [ ] Required proof artifacts written.
- [ ] Workpack completion section filled.
- [ ] No iOS parity overclaim from scaffold.
- [ ] Platform/store limitations visible.
- [ ] Setup handoff recorded.
- [ ] Focused commands pass or blocker recorded.
- [ ] PLAN_STATE updated if state changed.

## WP06 Parent Local Service Route Bridge

- [x] Parent client route bridge boundary defined. Proof: `output/parent-client-runtime-distribution-plan-proof/06-parent-local-service-route-bridge/00-scope-summary.md`.
- [x] Local-service authority remains service-owned. Proof: `output/parent-client-runtime-distribution-plan-proof/06-parent-local-service-route-bridge/00-scope-summary.md`, `output/parent-client-runtime-distribution-plan-proof/06-parent-local-service-route-bridge/01-negative-case-proof.md`.
- [x] Portal route does not execute child-device work. Proof: `output/parent-client-runtime-distribution-plan-proof/06-parent-local-service-route-bridge/00-scope-summary.md`, `output/parent-client-runtime-distribution-plan-proof/06-parent-local-service-route-bridge/01-negative-case-proof.md`.
- [x] Request/response contract proof exists or blocker recorded. Proof: `output/parent-client-runtime-distribution-plan-proof/06-parent-local-service-route-bridge/00-scope-summary.md`, `output/parent-client-runtime-distribution-plan-proof/06-parent-local-service-route-bridge/16-validation-commands.log`.
- [x] Failure/degraded states visible. Proof: `output/parent-client-runtime-distribution-plan-proof/06-parent-local-service-route-bridge/01-negative-case-proof.md`, `output/parent-client-runtime-distribution-plan-proof/06-parent-local-service-route-bridge/16-validation-commands.log`.
- [x] Required proof artifacts written. Proof: `output/parent-client-runtime-distribution-plan-proof/06-parent-local-service-route-bridge/00-scope-summary.md`, `output/parent-client-runtime-distribution-plan-proof/06-parent-local-service-route-bridge/01-negative-case-proof.md`, `output/parent-client-runtime-distribution-plan-proof/06-parent-local-service-route-bridge/02-manual-required-gap-register.md`, `output/parent-client-runtime-distribution-plan-proof/06-parent-local-service-route-bridge/16-validation-commands.log`.
- [x] Workpack completion section filled. Proof: `docs/plans/parent-desktop-runtime-package-plan/workpacks/06-parent-local-service-route-bridge.md`.
- [x] No setup-complete claim made. Proof: `output/parent-client-runtime-distribution-plan-proof/06-parent-local-service-route-bridge/00-scope-summary.md`, `output/parent-client-runtime-distribution-plan-proof/06-parent-local-service-route-bridge/02-manual-required-gap-register.md`.
- [x] No child-agent runtime claim made. Proof: `output/parent-client-runtime-distribution-plan-proof/06-parent-local-service-route-bridge/00-scope-summary.md`, `output/parent-client-runtime-distribution-plan-proof/06-parent-local-service-route-bridge/02-manual-required-gap-register.md`.
- [x] Focused commands pass or blocker recorded. Proof: `output/parent-client-runtime-distribution-plan-proof/06-parent-local-service-route-bridge/16-validation-commands.log`.
- [x] Route sync updated if state changed. Proof: `docs/plans/parent-desktop-runtime-package-plan/PLAN_STATE.md`, `output/parent-client-runtime-distribution-plan-proof/06-parent-local-service-route-bridge/16-validation-commands.log`.
- [x] Manual-required gaps visible. Proof: `output/parent-client-runtime-distribution-plan-proof/06-parent-local-service-route-bridge/02-manual-required-gap-register.md`.

## WP07 Parent Client Signing Store Matrix

- [ ] Signing matrix defined by artifact/platform.
- [ ] Store/notarization matrix defined.
- [ ] Manual-required states defined.
- [ ] Unsupported states defined.
- [ ] Signed/unsigned distinction visible.
- [ ] Required proof artifacts written.
- [ ] Workpack completion section filled.
- [ ] No signing/store-ready overclaim made.
- [ ] Focused commands pass or blocker recorded.
- [ ] Release handoff recorded.
- [ ] Platform limitations visible.
- [ ] PLAN_STATE updated if state changed.

## WP08 Parent Client Update Rollback

- [x] Update channel state defined. Proof: `output/parent-client-runtime-distribution-plan-proof/08-parent-client-update-rollback/00-scope-summary.md`, `test-results/parent-desktop-release-support-proof/proof.json`.
- [x] Rollback path defined. Proof: `output/parent-client-runtime-distribution-plan-proof/08-parent-client-update-rollback/00-scope-summary.md`, `output/parent-client-runtime-distribution-plan-proof/08-parent-client-update-rollback/01-negative-case-proof.md`, `test-results/parent-desktop-release-support-proof/proof.json`.
- [x] Checksum/integrity proof exists or blocker recorded. Proof: `output/parent-client-runtime-distribution-plan-proof/08-parent-client-update-rollback/00-scope-summary.md`, `test-results/parent-desktop-release-support-proof/proof.json`.
- [x] SBOM proof exists or blocker recorded. Proof: `output/parent-client-runtime-distribution-plan-proof/08-parent-client-update-rollback/02-manual-required-gap-register.md`, `output/parent-client-runtime-distribution-plan-proof/08-parent-client-update-rollback/16-validation-commands.log`.
- [x] Failed update state visible. Proof: `output/parent-client-runtime-distribution-plan-proof/08-parent-client-update-rollback/01-negative-case-proof.md`, `test-results/parent-desktop-release-support-proof/proof.json`.
- [x] Required proof artifacts written. Proof: `output/parent-client-runtime-distribution-plan-proof/08-parent-client-update-rollback/00-scope-summary.md`, `output/parent-client-runtime-distribution-plan-proof/08-parent-client-update-rollback/01-negative-case-proof.md`, `output/parent-client-runtime-distribution-plan-proof/08-parent-client-update-rollback/02-manual-required-gap-register.md`, `output/parent-client-runtime-distribution-plan-proof/08-parent-client-update-rollback/16-validation-commands.log`.
- [x] Workpack completion section filled. Proof: `docs/plans/parent-desktop-runtime-package-plan/workpacks/08-parent-client-update-rollback.md`.
- [x] No update-ready claim without rollback proof. Proof: `output/parent-client-runtime-distribution-plan-proof/08-parent-client-update-rollback/00-scope-summary.md`, `output/parent-client-runtime-distribution-plan-proof/08-parent-client-update-rollback/01-negative-case-proof.md`.
- [x] Focused commands pass or blocker recorded. Proof: `output/parent-client-runtime-distribution-plan-proof/08-parent-client-update-rollback/16-validation-commands.log`.
- [x] Manual-required gaps visible. Proof: `output/parent-client-runtime-distribution-plan-proof/08-parent-client-update-rollback/02-manual-required-gap-register.md`, `test-results/parent-desktop-release-support-proof/proof.json`.
- [x] Setup handoff remains separate. Proof: `output/parent-client-runtime-distribution-plan-proof/08-parent-client-update-rollback/00-scope-summary.md`, `output/parent-client-runtime-distribution-plan-proof/08-parent-client-update-rollback/02-manual-required-gap-register.md`.
- [x] PLAN_STATE updated if state changed. Proof: `docs/plans/parent-desktop-runtime-package-plan/PLAN_STATE.md`, `output/parent-client-runtime-distribution-plan-proof/08-parent-client-update-rollback/16-validation-commands.log`.

## WP09 Parent Client Launch Smoke Matrix

- [x] Launch smoke matrix defined by artifact/platform. Proof: `output/parent-client-runtime-distribution-plan-proof/09-parent-client-launch-smoke-matrix/00-scope-summary.md`.
- [x] Web launch proof exists or blocker recorded. Proof: `output/parent-client-runtime-distribution-plan-proof/09-parent-client-launch-smoke-matrix/00-scope-summary.md`, `output/parent-client-runtime-distribution-plan-proof/09-parent-client-launch-smoke-matrix/03-web-launch-smoke.log`, `output/parent-client-runtime-distribution-plan-proof/09-parent-client-launch-smoke-matrix/16-validation-commands.log`.
- [x] Desktop launch proof exists or blocker recorded. Proof: `output/parent-client-runtime-distribution-plan-proof/09-parent-client-launch-smoke-matrix/00-scope-summary.md`, `output/parent-client-runtime-distribution-plan-proof/09-parent-client-launch-smoke-matrix/04-desktop-launch-smoke.log`, `output/parent-client-runtime-distribution-plan-proof/09-parent-client-launch-smoke-matrix/16-validation-commands.log`.
- [x] Android launch proof exists or blocker recorded. Proof: `output/parent-client-runtime-distribution-plan-proof/09-parent-client-launch-smoke-matrix/00-scope-summary.md`, `output/parent-client-runtime-distribution-plan-proof/09-parent-client-launch-smoke-matrix/05-parent-mobile-launch-smoke.log`, `output/parent-client-runtime-distribution-plan-proof/09-parent-client-launch-smoke-matrix/16-validation-commands.log`.
- [x] iOS launch proof exists or blocker recorded. Proof: `output/parent-client-runtime-distribution-plan-proof/09-parent-client-launch-smoke-matrix/00-scope-summary.md`, `output/parent-client-runtime-distribution-plan-proof/09-parent-client-launch-smoke-matrix/05-parent-mobile-launch-smoke.log`, `output/parent-client-runtime-distribution-plan-proof/09-parent-client-launch-smoke-matrix/16-validation-commands.log`.
- [x] Manual-required gaps visible. Proof: `output/parent-client-runtime-distribution-plan-proof/09-parent-client-launch-smoke-matrix/00-scope-summary.md`, `output/parent-client-runtime-distribution-plan-proof/09-parent-client-launch-smoke-matrix/02-manual-required-gap-register.md`.
- [x] Required proof artifacts written. Proof: `output/parent-client-runtime-distribution-plan-proof/09-parent-client-launch-smoke-matrix/00-scope-summary.md`, `output/parent-client-runtime-distribution-plan-proof/09-parent-client-launch-smoke-matrix/01-negative-case-proof.md`, `output/parent-client-runtime-distribution-plan-proof/09-parent-client-launch-smoke-matrix/02-manual-required-gap-register.md`, `output/parent-client-runtime-distribution-plan-proof/09-parent-client-launch-smoke-matrix/03-web-launch-smoke.log`, `output/parent-client-runtime-distribution-plan-proof/09-parent-client-launch-smoke-matrix/04-desktop-launch-smoke.log`, `output/parent-client-runtime-distribution-plan-proof/09-parent-client-launch-smoke-matrix/05-parent-mobile-launch-smoke.log`, `output/parent-client-runtime-distribution-plan-proof/09-parent-client-launch-smoke-matrix/16-validation-commands.log`.
- [x] Workpack completion section filled. Proof: `docs/plans/parent-desktop-runtime-package-plan/workpacks/09-parent-client-launch-smoke-matrix.md`.
- [x] Launch smoke is not product-readiness claim. Proof: `output/parent-client-runtime-distribution-plan-proof/09-parent-client-launch-smoke-matrix/00-scope-summary.md`, `output/parent-client-runtime-distribution-plan-proof/09-parent-client-launch-smoke-matrix/01-negative-case-proof.md`.
- [x] Focused commands pass or blocker recorded. Proof: `output/parent-client-runtime-distribution-plan-proof/09-parent-client-launch-smoke-matrix/16-validation-commands.log`.
- [x] Route sync updated if state changed. Proof: `docs/plans/parent-desktop-runtime-package-plan/WORKPACK_INDEX.md`, `output/parent-client-runtime-distribution-plan-proof/09-parent-client-launch-smoke-matrix/16-validation-commands.log`.
- [x] PLAN_STATE updated if state changed. Proof: `docs/plans/parent-desktop-runtime-package-plan/PLAN_STATE.md`, `output/parent-client-runtime-distribution-plan-proof/09-parent-client-launch-smoke-matrix/16-validation-commands.log`.

## WP10 Setup Handoff Contracts

- [ ] Setup request contract defined.
- [ ] Setup response/readiness contract defined.
- [ ] Parent install state mapped to setup-install plan.
- [ ] Setup plan does not own package proof.
- [ ] Package plan does not own setup completion.
- [ ] Required proof artifacts written.
- [ ] Workpack completion section filled.
- [ ] Focused commands pass or blocker recorded.
- [ ] Adjacent handoff recorded.
- [ ] No setup-ready overclaim made.

## WP11 Proof CI Release Gate

- [ ] Prior proof roots consumed or blockers recorded.
- [ ] CI proof matrix written.
- [ ] Release gate proof written.
- [ ] Platform manual-required gap register written.
- [ ] Product-status wording safe.
- [ ] Route/index sync proof written.
- [ ] Required proof artifacts written.
- [ ] Focused commands pass or blocker recorded.
- [ ] PLAN_STATE and WORKPACK_INDEX reflect actual state.
- [ ] No PR_READY claim without required proof roots.
- [ ] Setup-install handoff state recorded.
- [ ] Child runtime distribution excluded.
- [ ] Manual-required gaps visible.
- [ ] Release blockers explicit.

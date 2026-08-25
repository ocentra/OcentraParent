<!-- agent-capsule -->

> Agent Capsule
> Plan: `setup-install-provisioning-plan`
> Doc: `Setup Install Provisioning Plan Checklist Index`
> Kind: exact checklist router.
> Read when: a selected workpack references checklist rows.
> Stop rule: do not scan unrelated checklist rows.
> Proves: checklist routing only.
> Does not prove: implementation completion.
> Proof rule: a checkbox can be checked only after proof artifacts and focused command results exist.

<!-- /agent-capsule -->

# Setup Install Provisioning Plan Checklist Index

> **Live-code audit (2026-07-17):** [Project Progress Matrix](../../PLAN_CODE_STATUS_MATRIX.md) records current implementation, blockers, dependencies, and next unblocker. Rows remain proof-gated; this audit does not check unsupported work.

## Fill rules

- Leave a checkbox unchecked until proof exists.
- Every checked row must cite one or more proof artifacts from `PROOF_INDEX.md`.
- Every proof item must list exact commands run, pass/fail/blocker, and no-claim boundaries.
- Do not mark setup complete from website-only, installer-only, UI-only, or pairing-only proof.
- Do not mark PR_READY until WP06 aggregates proof from all required earlier workpacks.

## WP01 Family Web Info Site

- [x] Public route map defined. `output/setup-install-provisioning-plan-proof/01-family-web-info-site/00-public-site-route-map-proof.md`
- [x] Public pages no-private-activity boundary documented. `output/setup-install-provisioning-plan-proof/01-family-web-info-site/01-no-private-activity-data-proof.md`
- [x] Data collection matrix defined. `output/setup-install-provisioning-plan-proof/01-family-web-info-site/02-data-collection-matrix.md`
- [x] Privacy copy no-overclaim proof exists. `output/setup-install-provisioning-plan-proof/01-family-web-info-site/03-privacy-copy-no-overclaim-proof.md`
- [x] Download/register/support/privacy/status links defined. `output/setup-install-provisioning-plan-proof/01-family-web-info-site/00-public-site-route-map-proof.md`
- [x] Cloudflare Pages or Workers deploy shape selected or blocker recorded. `output/setup-install-provisioning-plan-proof/01-family-web-info-site/05-deploy-preview-proof-or-blocker.md`
- [x] Custom domain/manual-required state recorded if no deployment proof exists. `output/setup-install-provisioning-plan-proof/01-family-web-info-site/05-deploy-preview-proof-or-blocker.md`
- [x] Basic link/accessibility proof exists or blocker recorded. `output/setup-install-provisioning-plan-proof/01-family-web-info-site/04-link-accessibility-proof.md`
- [x] Required proof artifacts written. `output/setup-install-provisioning-plan-proof/01-family-web-info-site/00-public-site-route-map-proof.md`, `output/setup-install-provisioning-plan-proof/01-family-web-info-site/01-no-private-activity-data-proof.md`, `output/setup-install-provisioning-plan-proof/01-family-web-info-site/02-data-collection-matrix.md`, `output/setup-install-provisioning-plan-proof/01-family-web-info-site/03-privacy-copy-no-overclaim-proof.md`, `output/setup-install-provisioning-plan-proof/01-family-web-info-site/04-link-accessibility-proof.md`, `output/setup-install-provisioning-plan-proof/01-family-web-info-site/05-deploy-preview-proof-or-blocker.md`, `output/setup-install-provisioning-plan-proof/01-family-web-info-site/16-validation-commands.log`
- [x] Focused commands pass or blocker recorded. `output/setup-install-provisioning-plan-proof/01-family-web-info-site/16-validation-commands.log`
- [x] Workpack completion section filled. `docs/plans/setup-install-provisioning-plan/workpacks/01-family-web-info-site.md`
- [x] PLAN_STATE updated if state changed. `docs/plans/setup-install-provisioning-plan/PLAN_STATE.md`

## WP02 Registration Login Entry

- [x] Register/login/logout/invite/resume/recovery route map defined. `output/setup-install-provisioning-plan-proof/02-registration-login-entry/00-registration-route-state-proof.md`
- [x] Account identity handoff contract defined. `output/setup-install-provisioning-plan-proof/02-registration-login-entry/01-auth-handoff-contract-proof.md`
- [x] Expired invite state visible. `output/setup-install-provisioning-plan-proof/02-registration-login-entry/02-invite-negative-state-proof.md`
- [x] Revoked invite state visible. `output/setup-install-provisioning-plan-proof/02-registration-login-entry/02-invite-negative-state-proof.md`
- [x] Wrong-household invite state visible. `output/setup-install-provisioning-plan-proof/02-registration-login-entry/02-invite-negative-state-proof.md`
- [x] Provider unavailable state visible. `output/setup-install-provisioning-plan-proof/02-registration-login-entry/05-provider-unavailable-state-proof.md`
- [x] No private profile/device data before household authority. `output/setup-install-provisioning-plan-proof/02-registration-login-entry/03-no-sensitive-data-before-household-proof.md`
- [x] Recovery link/state visible. `output/setup-install-provisioning-plan-proof/02-registration-login-entry/00-registration-route-state-proof.md`, `output/setup-install-provisioning-plan-proof/02-registration-login-entry/04-registration-ui-state-proof.md`
- [x] Redacted logging proof or blocker recorded. `output/setup-install-provisioning-plan-proof/02-registration-login-entry/01-auth-handoff-contract-proof.md`
- [x] Required proof artifacts written. `output/setup-install-provisioning-plan-proof/02-registration-login-entry/00-registration-route-state-proof.md`, `output/setup-install-provisioning-plan-proof/02-registration-login-entry/01-auth-handoff-contract-proof.md`, `output/setup-install-provisioning-plan-proof/02-registration-login-entry/02-invite-negative-state-proof.md`, `output/setup-install-provisioning-plan-proof/02-registration-login-entry/03-no-sensitive-data-before-household-proof.md`, `output/setup-install-provisioning-plan-proof/02-registration-login-entry/04-registration-ui-state-proof.md`, `output/setup-install-provisioning-plan-proof/02-registration-login-entry/05-provider-unavailable-state-proof.md`, `output/setup-install-provisioning-plan-proof/02-registration-login-entry/16-validation-commands.log`
- [x] Focused commands pass or blocker recorded. `output/setup-install-provisioning-plan-proof/02-registration-login-entry/16-validation-commands.log`
- [x] Workpack completion section filled. `docs/plans/setup-install-provisioning-plan/workpacks/02-registration-login-entry.md`

## WP03 Parent Install Journey

- [x] Parent bootstrap code state machine defined. `output/setup-install-provisioning-plan-proof/03-parent-install-journey/00-parent-bootstrap-code-state-proof.md`
- [x] Parent platform matrix defined. `output/setup-install-provisioning-plan-proof/03-parent-install-journey/01-parent-platform-matrix-proof.md`
- [x] Download/version/integrity display expectations defined. `output/setup-install-provisioning-plan-proof/03-parent-install-journey/02-download-integrity-proof.md`
- [x] Unsupported platform state visible. `output/setup-install-provisioning-plan-proof/03-parent-install-journey/03-unsupported-platform-proof.md`
- [x] Manual-required state visible. `output/setup-install-provisioning-plan-proof/03-parent-install-journey/03-unsupported-platform-proof.md`, `output/setup-install-provisioning-plan-proof/03-parent-install-journey/05-parent-install-ui-proof.md`
- [x] Update-required state visible. `output/setup-install-provisioning-plan-proof/03-parent-install-journey/04-update-rollback-handoff-proof.md`, `output/setup-install-provisioning-plan-proof/03-parent-install-journey/05-parent-install-ui-proof.md`
- [x] Runtime distribution handoff proof exists or blocker recorded. `output/setup-install-provisioning-plan-proof/03-parent-install-journey/04-update-rollback-handoff-proof.md`
- [x] Website download button cannot imply package readiness. `output/setup-install-provisioning-plan-proof/03-parent-install-journey/02-download-integrity-proof.md`
- [x] Required proof artifacts written. `output/setup-install-provisioning-plan-proof/03-parent-install-journey/00-parent-bootstrap-code-state-proof.md`, `output/setup-install-provisioning-plan-proof/03-parent-install-journey/01-parent-platform-matrix-proof.md`, `output/setup-install-provisioning-plan-proof/03-parent-install-journey/02-download-integrity-proof.md`, `output/setup-install-provisioning-plan-proof/03-parent-install-journey/03-unsupported-platform-proof.md`, `output/setup-install-provisioning-plan-proof/03-parent-install-journey/04-update-rollback-handoff-proof.md`, `output/setup-install-provisioning-plan-proof/03-parent-install-journey/05-parent-install-ui-proof.md`, `output/setup-install-provisioning-plan-proof/03-parent-install-journey/16-validation-commands.log`
- [x] Focused commands pass or blocker recorded. `output/setup-install-provisioning-plan-proof/03-parent-install-journey/16-validation-commands.log`
- [x] Workpack completion section filled. `docs/plans/setup-install-provisioning-plan/workpacks/03-parent-install-journey.md`
- [x] PLAN_STATE updated if state changed. `docs/plans/setup-install-provisioning-plan/PLAN_STATE.md`
- [x] No product-ready installer claim made without package proof. `output/setup-install-provisioning-plan-proof/03-parent-install-journey/02-download-integrity-proof.md`, `output/setup-install-provisioning-plan-proof/03-parent-install-journey/04-update-rollback-handoff-proof.md`

## WP04 Child Install Permission Journey

- [x] Child bootstrap code state machine defined. `output/setup-install-provisioning-plan-proof/04-child-install-permission-journey/00-child-bootstrap-code-state-proof.md`
- [x] Child platform matrix defined. `output/setup-install-provisioning-plan-proof/04-child-install-permission-journey/01-child-platform-matrix-proof.md`
- [x] Permission matrix defined. `output/setup-install-provisioning-plan-proof/04-child-install-permission-journey/02-permission-matrix-proof.md`
- [x] Installed/running/permissioned/paired/trusted/policy-ready states separated. `output/setup-install-provisioning-plan-proof/04-child-install-permission-journey/02-permission-matrix-proof.md`
- [x] Missing permission degraded state visible. `output/setup-install-provisioning-plan-proof/04-child-install-permission-journey/03-missing-permission-degraded-proof.md`
- [x] Disclosure state visible. `output/setup-install-provisioning-plan-proof/04-child-install-permission-journey/04-child-disclosure-proof.md`
- [x] Reinstall recovery state visible. `output/setup-install-provisioning-plan-proof/04-child-install-permission-journey/05-reinstall-recovery-proof.md`
- [x] Runtime/package/platform owner handoffs recorded. `output/setup-install-provisioning-plan-proof/04-child-install-permission-journey/04-child-disclosure-proof.md`, `output/setup-install-provisioning-plan-proof/04-child-install-permission-journey/05-reinstall-recovery-proof.md`
- [x] Unsupported/manual-required states visible. `output/setup-install-provisioning-plan-proof/04-child-install-permission-journey/06-child-install-ui-proof.md`
- [x] Required proof artifacts written. `output/setup-install-provisioning-plan-proof/04-child-install-permission-journey/00-child-bootstrap-code-state-proof.md`, `output/setup-install-provisioning-plan-proof/04-child-install-permission-journey/01-child-platform-matrix-proof.md`, `output/setup-install-provisioning-plan-proof/04-child-install-permission-journey/02-permission-matrix-proof.md`, `output/setup-install-provisioning-plan-proof/04-child-install-permission-journey/03-missing-permission-degraded-proof.md`, `output/setup-install-provisioning-plan-proof/04-child-install-permission-journey/04-child-disclosure-proof.md`, `output/setup-install-provisioning-plan-proof/04-child-install-permission-journey/05-reinstall-recovery-proof.md`, `output/setup-install-provisioning-plan-proof/04-child-install-permission-journey/06-child-install-ui-proof.md`, `output/setup-install-provisioning-plan-proof/04-child-install-permission-journey/16-validation-commands.log`
- [x] Focused commands pass or blocker recorded. `output/setup-install-provisioning-plan-proof/04-child-install-permission-journey/16-validation-commands.log`
- [x] Workpack completion section filled. `docs/plans/setup-install-provisioning-plan/workpacks/04-child-install-permission-journey.md`
- [x] No ready claim made from installed-process proof alone. `output/setup-install-provisioning-plan-proof/04-child-install-permission-journey/02-permission-matrix-proof.md`, `output/setup-install-provisioning-plan-proof/04-child-install-permission-journey/03-missing-permission-degraded-proof.md`
- [x] No sibling owner implementation absorbed. `output/setup-install-provisioning-plan-proof/04-child-install-permission-journey/04-child-disclosure-proof.md`, `output/setup-install-provisioning-plan-proof/04-child-install-permission-journey/05-reinstall-recovery-proof.md`

## WP05 Pairing Readiness Recovery

- [x] Pairing lifecycle state machine defined. `output/setup-install-provisioning-plan-proof/05-pairing-readiness-recovery/05-pairing-state-machine-proof.md`
- [x] Readiness matrix defined. `output/setup-install-provisioning-plan-proof/05-pairing-readiness-recovery/05-readiness-matrix-proof.md`
- [x] Wrong-household/wrong-device state covered. `output/setup-install-provisioning-plan-proof/05-pairing-readiness-recovery/05-pairing-negative-proof.md`
- [x] Stale/revoked/offline state covered. `output/setup-install-provisioning-plan-proof/05-pairing-readiness-recovery/05-pairing-negative-proof.md`
- [x] No fake ready state proof exists. `output/setup-install-provisioning-plan-proof/05-pairing-readiness-recovery/05-no-fake-ready-state-proof.md`
- [x] Offline device degraded state visible. `output/setup-install-provisioning-plan-proof/05-pairing-readiness-recovery/05-pairing-negative-proof.md`, `output/setup-install-provisioning-plan-proof/05-pairing-readiness-recovery/05-readiness-matrix-proof.md`
- [x] Permission missing degraded state visible. `output/setup-install-provisioning-plan-proof/05-pairing-readiness-recovery/05-readiness-matrix-proof.md`, `output/setup-install-provisioning-plan-proof/05-pairing-readiness-recovery/05-no-fake-ready-state-proof.md`
- [x] Policy baseline missing state visible. `output/setup-install-provisioning-plan-proof/05-pairing-readiness-recovery/05-readiness-matrix-proof.md`, `output/setup-install-provisioning-plan-proof/05-pairing-readiness-recovery/05-no-fake-ready-state-proof.md`
- [x] Data custody unavailable state visible. `output/setup-install-provisioning-plan-proof/05-pairing-readiness-recovery/05-readiness-matrix-proof.md`, `output/setup-install-provisioning-plan-proof/05-pairing-readiness-recovery/05-no-fake-ready-state-proof.md`
- [x] Lost-parent-device recovery state visible. `output/setup-install-provisioning-plan-proof/05-pairing-readiness-recovery/05-recovery-flow-proof.md`
- [x] Reinstall recovery state visible. `output/setup-install-provisioning-plan-proof/05-pairing-readiness-recovery/05-recovery-flow-proof.md`, `output/setup-install-provisioning-plan-proof/05-pairing-readiness-recovery/no-fake-ready-after-install-proof.md`
- [x] Redacted setup/pairing log proof or blocker recorded. `output/setup-install-provisioning-plan-proof/05-pairing-readiness-recovery/05-redacted-pairing-log-proof.md`, `output/setup-install-provisioning-plan-proof/05-pairing-readiness-recovery/redacted-bootstrap-logs-proof.md`
- [x] Workpack completion section filled. `docs/plans/setup-install-provisioning-plan/workpacks/05-pairing-readiness-recovery.md`

## WP07 First-Run Setup UI And State Machine

Current production/test overlay (2026-08-17):

- [x] Reachable Rust owner renders a fail-closed 15-row manual-required authority matrix without invoking the evaluator or action planner. Source: `crates/parent-runtime-core/src/setup_first_run.rs`.
- [x] LAN selected-device, paired, and reachability values remain non-authoritative observations; Start reads are separated from LAN command routing and discovery scans fail closed.
- [ ] Bind real typed account, package, child-runtime, device-trust/pairing, permission, custody, policy, and recovery owner inputs.
- [ ] Implement the actual first-run transition/action model and readiness-driven completion guard.
- [ ] Rewrite `snapshot_and_dispatch_tests.rs`, portal-domain setup panel tests, portal route-panel tests, and setup E2E fixtures for the accepted source shape.
- [ ] Run focused tests and regenerate current retained proof after source and expected-test writing are complete.

- [x] End-to-end first-run state machine defined. `output/setup-install-provisioning-plan-proof/07-first-run-setup-ui-and-state-machine/00-first-run-state-machine-proof.md`
- [x] Welcome/sign-in/household screens mapped. `output/setup-install-provisioning-plan-proof/07-first-run-setup-ui-and-state-machine/01-first-run-ui-screen-map.md`
- [x] Parent install screen/state mapped. `output/setup-install-provisioning-plan-proof/07-first-run-setup-ui-and-state-machine/01-first-run-ui-screen-map.md`
- [x] Child profile/install/pair screens mapped. `output/setup-install-provisioning-plan-proof/07-first-run-setup-ui-and-state-machine/01-first-run-ui-screen-map.md`
- [x] Readiness/recovery/complete/blocked/manual-required screens mapped. `output/setup-install-provisioning-plan-proof/07-first-run-setup-ui-and-state-machine/01-first-run-ui-screen-map.md`
- [x] Empty/error/degraded UI proof exists or blocker recorded. `output/setup-install-provisioning-plan-proof/07-first-run-setup-ui-and-state-machine/02-empty-error-degraded-ui-proof.md`
- [x] Manual-required state visible. `output/setup-install-provisioning-plan-proof/07-first-run-setup-ui-and-state-machine/03-manual-required-visible-proof.md`
- [x] Adjacent handoff visible. `output/setup-install-provisioning-plan-proof/07-first-run-setup-ui-and-state-machine/04-adjacent-handoff-visible-proof.md`
- [x] Source/custody labels visible. `output/setup-install-provisioning-plan-proof/07-first-run-setup-ui-and-state-machine/06-source-custody-label-proof.md`
- [x] No fake ready state proof exists. `output/setup-install-provisioning-plan-proof/07-first-run-setup-ui-and-state-machine/05-no-fake-ready-state-proof.md`
- [x] Portal tests or exact missing test blocker recorded. `output/setup-install-provisioning-plan-proof/07-first-run-setup-ui-and-state-machine/16-validation-commands.log`
- [x] Required proof artifacts written. `output/setup-install-provisioning-plan-proof/07-first-run-setup-ui-and-state-machine/00-first-run-state-machine-proof.md`, `output/setup-install-provisioning-plan-proof/07-first-run-setup-ui-and-state-machine/01-first-run-ui-screen-map.md`, `output/setup-install-provisioning-plan-proof/07-first-run-setup-ui-and-state-machine/02-empty-error-degraded-ui-proof.md`, `output/setup-install-provisioning-plan-proof/07-first-run-setup-ui-and-state-machine/03-manual-required-visible-proof.md`, `output/setup-install-provisioning-plan-proof/07-first-run-setup-ui-and-state-machine/04-adjacent-handoff-visible-proof.md`, `output/setup-install-provisioning-plan-proof/07-first-run-setup-ui-and-state-machine/05-no-fake-ready-state-proof.md`, `output/setup-install-provisioning-plan-proof/07-first-run-setup-ui-and-state-machine/06-source-custody-label-proof.md`, `output/setup-install-provisioning-plan-proof/07-first-run-setup-ui-and-state-machine/16-validation-commands.log`
- [x] Workpack completion section filled. `docs/plans/setup-install-provisioning-plan/workpacks/07-first-run-setup-ui-and-state-machine.md`
- [x] PLAN_STATE updated if state changed. `docs/plans/setup-install-provisioning-plan/PLAN_STATE.md`

## WP06 Rollout Proof And Route Gate

- [x] WP01 proof root consumed or blocker recorded. `output/setup-install-provisioning-plan-proof/01-family-web-info-site/`
- [x] WP02 proof root consumed or blocker recorded. `output/setup-install-provisioning-plan-proof/02-registration-login-entry/`
- [x] WP03 proof root consumed or blocker recorded. `output/setup-install-provisioning-plan-proof/03-parent-install-journey/`
- [x] WP04 proof root consumed or blocker recorded. `output/setup-install-provisioning-plan-proof/04-child-install-permission-journey/`
- [x] WP05 proof root consumed or blocker recorded. `output/setup-install-provisioning-plan-proof/05-pairing-readiness-recovery/`
- [x] WP07 proof root consumed or blocker recorded. `output/setup-install-provisioning-plan-proof/07-first-run-setup-ui-and-state-machine/`
- [x] Rollout proof pack written. `output/setup-install-provisioning-plan-proof/06-rollout-proof-and-route-gate/00-rollout-proof-pack.md`
- [x] Route/index sync proof written. `output/setup-install-provisioning-plan-proof/06-rollout-proof-and-route-gate/01-route-sync-proof.md`
- [x] Platform readiness matrix written. `output/setup-install-provisioning-plan-proof/06-rollout-proof-and-route-gate/02-platform-readiness-matrix.md`
- [x] Public/private boundary proof written. `output/setup-install-provisioning-plan-proof/06-rollout-proof-and-route-gate/03-public-private-boundary-proof.md`
- [x] Manual-required gap register written. `output/setup-install-provisioning-plan-proof/06-rollout-proof-and-route-gate/04-manual-required-gap-register.md`
- [x] Product-status wording proof written. `output/setup-install-provisioning-plan-proof/06-rollout-proof-and-route-gate/05-product-status-safe-wording-proof.md`
- [x] Focused validation commands pass or blockers recorded. `output/setup-install-provisioning-plan-proof/06-rollout-proof-and-route-gate/16-validation-commands.log`
- [x] PLAN_STATE and WORKPACK_INDEX reflect actual state. `docs/plans/setup-install-provisioning-plan/PLAN_STATE.md`, `docs/plans/setup-install-provisioning-plan/WORKPACK_INDEX.md`
- [x] No PR_READY claim without required proof roots. `docs/plans/setup-install-provisioning-plan/workpacks/06-rollout-proof-and-route-gate.md`

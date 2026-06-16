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

## Fill rules

- Leave a checkbox unchecked until proof exists.
- Every checked row must cite one or more proof artifacts from `PROOF_INDEX.md`.
- Every proof item must list exact commands run, pass/fail/blocker, and no-claim boundaries.
- Do not mark setup complete from website-only, installer-only, UI-only, or pairing-only proof.
- Do not mark PR_READY until WP06 aggregates proof from all required earlier workpacks.

## WP01 Family Web Info Site

- [ ] Public route map defined.
- [ ] Public pages no-private-activity boundary documented.
- [ ] Data collection matrix defined.
- [ ] Privacy copy no-overclaim proof exists.
- [ ] Download/register/support/privacy/status links defined.
- [ ] Cloudflare Pages or Workers deploy shape selected or blocker recorded.
- [ ] Custom domain/manual-required state recorded if no deployment proof exists.
- [ ] Basic link/accessibility proof exists or blocker recorded.
- [ ] Required proof artifacts written.
- [ ] Focused commands pass or blocker recorded.
- [ ] Workpack completion section filled.
- [ ] PLAN_STATE updated if state changed.

## WP02 Registration Login Entry

- [ ] Register/login/logout/invite/resume/recovery route map defined.
- [ ] Account identity handoff contract defined.
- [ ] Expired invite state visible.
- [ ] Revoked invite state visible.
- [ ] Wrong-household invite state visible.
- [ ] Provider unavailable state visible.
- [ ] No private profile/device data before household authority.
- [ ] Recovery link/state visible.
- [ ] Redacted logging proof or blocker recorded.
- [ ] Required proof artifacts written.
- [ ] Focused commands pass or blocker recorded.
- [ ] Workpack completion section filled.

## WP03 Parent Install Journey

- [ ] Parent bootstrap code state machine defined.
- [ ] Parent platform matrix defined.
- [ ] Download/version/integrity display expectations defined.
- [ ] Unsupported platform state visible.
- [ ] Manual-required state visible.
- [ ] Update-required state visible.
- [ ] Runtime distribution handoff proof exists or blocker recorded.
- [ ] Website download button cannot imply package readiness.
- [ ] Required proof artifacts written.
- [ ] Focused commands pass or blocker recorded.
- [ ] Workpack completion section filled.
- [ ] PLAN_STATE updated if state changed.
- [ ] No product-ready installer claim made without package proof.

## WP04 Child Install Permission Journey

- [ ] Child bootstrap code state machine defined.
- [ ] Child platform matrix defined.
- [ ] Permission matrix defined.
- [ ] Installed/running/permissioned/paired/trusted/policy-ready states separated.
- [ ] Missing permission degraded state visible.
- [ ] Disclosure state visible.
- [ ] Reinstall recovery state visible.
- [ ] Runtime/package/platform owner handoffs recorded.
- [ ] Unsupported/manual-required states visible.
- [ ] Required proof artifacts written.
- [ ] Focused commands pass or blocker recorded.
- [ ] Workpack completion section filled.
- [ ] No ready claim made from installed-process proof alone.
- [ ] No sibling owner implementation absorbed.

## WP05 Pairing Readiness Recovery

- [ ] Pairing lifecycle state machine defined.
- [ ] Readiness matrix defined.
- [ ] Wrong-household/wrong-device state covered.
- [ ] Stale/revoked/offline state covered.
- [ ] No fake ready state proof exists.
- [ ] Offline device degraded state visible.
- [ ] Permission missing degraded state visible.
- [ ] Policy baseline missing state visible.
- [ ] Data custody unavailable state visible.
- [ ] Lost-parent-device recovery state visible.
- [ ] Reinstall recovery state visible.
- [ ] Redacted setup/pairing log proof or blocker recorded.
- [ ] Workpack completion section filled.

## WP07 First-Run Setup UI And State Machine

- [ ] End-to-end first-run state machine defined.
- [ ] Welcome/sign-in/household screens mapped.
- [ ] Parent install screen/state mapped.
- [ ] Child profile/install/pair screens mapped.
- [ ] Readiness/recovery/complete/blocked/manual-required screens mapped.
- [ ] Empty/error/degraded UI proof exists or blocker recorded.
- [ ] Manual-required state visible.
- [ ] Adjacent handoff visible.
- [ ] Source/custody labels visible.
- [ ] No fake ready state proof exists.
- [ ] Portal tests or exact missing test blocker recorded.
- [ ] Required proof artifacts written.
- [ ] Workpack completion section filled.
- [ ] PLAN_STATE updated if state changed.

## WP06 Rollout Proof And Route Gate

- [ ] WP01 proof root consumed or blocker recorded.
- [ ] WP02 proof root consumed or blocker recorded.
- [ ] WP03 proof root consumed or blocker recorded.
- [ ] WP04 proof root consumed or blocker recorded.
- [ ] WP05 proof root consumed or blocker recorded.
- [ ] WP07 proof root consumed or blocker recorded.
- [ ] Rollout proof pack written.
- [ ] Route/index sync proof written.
- [ ] Platform readiness matrix written.
- [ ] Public/private boundary proof written.
- [ ] Manual-required gap register written.
- [ ] Product-status wording proof written.
- [ ] Focused validation commands pass or blockers recorded.
- [ ] PLAN_STATE and WORKPACK_INDEX reflect actual state.
- [ ] No PR_READY claim without required proof roots.

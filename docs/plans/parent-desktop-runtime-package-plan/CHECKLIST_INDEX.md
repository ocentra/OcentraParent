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

## Fill rules

- Leave a checkbox unchecked until proof exists.
- Every checked row must cite one or more proof artifacts from `PROOF_INDEX.md`.
- Every proof item must list exact commands run, pass/fail/blocker, and no-claim boundaries.
- Do not mark a platform or artifact ready from scaffold, launch smoke, or package-preview alone.
- Do not mark PR_READY until WP11 aggregates proof from all required earlier workpacks.

## WP01 Parent Client Scope And Route Boundary

- [ ] Canonical parent-client scope documented.
- [ ] Historical desktop-only path compatibility documented.
- [ ] Child agent runtime excluded.
- [ ] Setup-install handoff documented.
- [ ] Portal shell UX handoff documented.
- [ ] No product-readiness overclaim recorded.
- [ ] Required proof artifacts written.
- [ ] Focused commands pass or blocker recorded.
- [ ] Workpack completion section filled.
- [ ] PLAN_STATE updated if state changed.

## WP02 Parent Web Portal Distribution

- [ ] Parent web build path defined.
- [ ] Route ownership documented.
- [ ] Auth/cache/env separation documented.
- [ ] Hosted route does not claim child-agent execution.
- [ ] Build/test proof exists or blocker recorded.
- [ ] Deployment/preview blocker recorded if missing.
- [ ] Required proof artifacts written.
- [ ] Workpack completion section filled.
- [ ] No production web-ready overclaim made.
- [ ] Setup handoff remains separate.
- [ ] Parent data/source labels preserved.
- [ ] Route sync updated if state changed.

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

- [ ] Parent client route bridge boundary defined.
- [ ] Local-service authority remains service-owned.
- [ ] Portal route does not execute child-device work.
- [ ] Request/response contract proof exists or blocker recorded.
- [ ] Failure/degraded states visible.
- [ ] Required proof artifacts written.
- [ ] Workpack completion section filled.
- [ ] No setup-complete claim made.
- [ ] No child-agent runtime claim made.
- [ ] Focused commands pass or blocker recorded.
- [ ] Route sync updated if state changed.
- [ ] Manual-required gaps visible.

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

- [ ] Update channel state defined.
- [ ] Rollback path defined.
- [ ] Checksum/integrity proof exists or blocker recorded.
- [ ] SBOM proof exists or blocker recorded.
- [ ] Failed update state visible.
- [ ] Required proof artifacts written.
- [ ] Workpack completion section filled.
- [ ] No update-ready claim without rollback proof.
- [ ] Focused commands pass or blocker recorded.
- [ ] Manual-required gaps visible.
- [ ] Setup handoff remains separate.
- [ ] PLAN_STATE updated if state changed.

## WP09 Parent Client Launch Smoke Matrix

- [ ] Launch smoke matrix defined by artifact/platform.
- [ ] Web launch proof exists or blocker recorded.
- [ ] Desktop launch proof exists or blocker recorded.
- [ ] Android launch proof exists or blocker recorded.
- [ ] iOS launch proof exists or blocker recorded.
- [ ] Manual-required gaps visible.
- [ ] Required proof artifacts written.
- [ ] Workpack completion section filled.
- [ ] Launch smoke is not product-readiness claim.
- [ ] Focused commands pass or blocker recorded.
- [ ] Route sync updated if state changed.
- [ ] PLAN_STATE updated if state changed.

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

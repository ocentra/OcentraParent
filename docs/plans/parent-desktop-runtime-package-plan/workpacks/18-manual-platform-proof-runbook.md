# 18 Manual Platform Proof Runbook

<!-- agent-capsule -->

> Agent Capsule
> Plan: `parent-desktop-runtime-package-plan`
> Doc: `18 Manual Platform Proof Runbook`
> Kind: proof reference; read only when validating matching claim.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

Sources: [20-step plan](../parent-desktop-runtime-package-20-step-plan.md),
[test blueprint](../parent-desktop-runtime-package-test-blueprint.md),
[requirements guide](../runtime-package-requirements-guide.md), and
[folder README](../README.md).

## Where We Are

CI can prove repeatable mechanics. Real hosts/devices are required for signing,
stores, OS permissions, and package install behavior.

## Where We Want To Be

Manual proof records name commit, platform, package/app version, command or UI
action, permission state, logs/screenshots, and observed result.

## Requirement Checklist

- [ ] Define manual proof artifacts per platform.
- [ ] Include distinct parent/child and package/runtime claims.
- [ ] Record signing/store/entitlement gaps.
- [ ] Keep artifacts out of source unless intentionally tracked.
- [ ] Update proof matrix after manual checks.

## Acceptance And Proof

Manual proof requirements are explicit before any production claim is made.

Current proof: manual runbook rows require host/device, command or UI action,
permissions, package version, logs/screenshots/proof JSON, and known gaps for
parent desktop, parent mobile, child desktop, child Android, child iOS, relay,
signing, store, and support.

## Parallel Ownership Notes

The user/manual tester may execute this gate after branch/CI readiness.

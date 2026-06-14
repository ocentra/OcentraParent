# 16 Release Branch Boundary

<!-- agent-capsule -->

> Agent Capsule
> Plan: `parent-desktop-runtime-package-plan`
> Doc: `16 Release Branch Boundary`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
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

`main` is a CI and package-preview branch. Production release publishing is not
automatic.

## Where We Want To Be

Release docs, workflows, and reports preserve preview versus production
promotion boundaries.

## Requirement Checklist

- [ ] Keep production publishing out of `main`.
- [ ] Label PR/merge as CI/package preview events.
- [ ] Document explicit production promotion.
- [ ] Avoid release notes that claim store/signing without proof.
- [ ] Review workflow changes carefully.

## Acceptance And Proof

Merging a package branch to `main` cannot silently publish production releases.

Current proof: `scripts/test/parent-desktop-release-support-proof.test.mjs`
checks that package preview artifacts live in the preview workflow and that
production publishing remains on the `production` branch with required secrets.

## Parallel Ownership Notes

Primary controls merge timing after CI is green.

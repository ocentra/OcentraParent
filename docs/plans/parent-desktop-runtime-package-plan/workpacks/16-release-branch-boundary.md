# 16 Release Branch Boundary

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

- [x] Keep production publishing out of `main`.
- [x] Label PR/merge as CI/package preview events.
- [x] Document explicit production promotion.
- [x] Avoid release notes that claim store/signing without proof.
- [x] Review workflow changes carefully.

## Acceptance And Proof

Merging a package branch to `main` cannot silently publish production releases.

Current proof: `scripts/test/parent-desktop-release-support-proof.test.mjs`
checks that package preview artifacts live in the preview workflow and that
production publishing remains on the `production` branch with required secrets.

## Parallel Ownership Notes

Primary controls merge timing after CI is green.

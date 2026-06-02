# 10 Signing, Notarization, And Store Claims

Sources: [20-step plan](../parent-desktop-runtime-package-20-step-plan.md),
[test blueprint](../parent-desktop-runtime-package-test-blueprint.md),
[requirements guide](../runtime-package-requirements-guide.md), and
[folder README](../README.md).

## Where We Are

Signing, notarization, Play signing, TestFlight/App Store, and store publishing
are not product-proved.

## Where We Want To Be

Every signing/store state is explicit: unavailable, credential-missing,
manual-required, preview-only, or proved.

## Requirement Checklist

- [ ] Add matrix rows for signing/store states.
- [ ] Keep credentials out of source.
- [ ] Record manual proof requirements.
- [ ] Avoid store distribution claims before artifacts exist.
- [ ] Update release docs/checklist when status changes.

## Acceptance And Proof

No branch report can claim store/signing readiness without named artifacts.

## Parallel Ownership Notes

This workpack is a release-truth guardrail.

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

- [x] Add matrix rows for signing/store states.
- [x] Keep credentials out of source.
- [x] Record manual proof requirements.
- [x] Avoid store distribution claims before artifacts exist.
- [x] Update release docs/checklist when status changes.

## Acceptance And Proof

No branch report can claim store/signing readiness without named artifacts.

Current proof: Windows signing, macOS notarization, Google Play, TestFlight,
and App Store states remain manual-required. The contract rejects implemented
signing/store states without real credential/artifact proof.

## Parallel Ownership Notes

This workpack is a release-truth guardrail.

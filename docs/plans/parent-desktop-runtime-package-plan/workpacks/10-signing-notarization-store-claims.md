# 10 Signing, Notarization, And Store Claims

<!-- agent-capsule -->

> Agent Capsule
> Plan: `parent-desktop-runtime-package-plan`
> Doc: `10 Signing, Notarization, And Store Claims`
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

Current proof: Windows signing, macOS notarization, Google Play, TestFlight,
and App Store states remain manual-required. The contract rejects implemented
signing/store states without real credential/artifact proof.

## Parallel Ownership Notes

This workpack is a release-truth guardrail.

# 12 Privacy And Release Docs

<!-- agent-capsule -->

> Agent Capsule
> Plan: `parent-desktop-runtime-package-plan`
> Doc: `12 Privacy And Release Docs`
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

Package/runtime proof can affect public-facing product claims. Docs must keep
preview versus production clear.

## Where We Want To Be

Release and privacy docs say what is packaged, what is local, what is hosted,
what is signed, what is preview-only, and what remains unavailable.

## Requirement Checklist

- [ ] Update feature docs when status changes.
- [ ] Keep release wording honest.
- [ ] Label child activity custody.
- [ ] Document package preview boundaries.
- [ ] Record manual proof gaps.

## Acceptance And Proof

Docs match package/proof state at PR-ready handoff.

Current proof: feature and expectation docs now state that package previews,
support diagnostics, signing, stores, mobile, relay, and rollback are not
production claims until real proof exists.

## Parallel Ownership Notes

Primary reviews product claim language before merge.

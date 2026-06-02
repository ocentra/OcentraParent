# 12 Privacy And Release Docs

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

- [x] Update feature docs when status changes.
- [x] Keep release wording honest.
- [x] Label child activity custody.
- [x] Document package preview boundaries.
- [x] Record manual proof gaps.

## Acceptance And Proof

Docs match package/proof state at PR-ready handoff.

Current proof: feature and expectation docs now state that package previews,
support diagnostics, signing, stores, mobile, relay, and rollback are not
production claims until real proof exists.

## Parallel Ownership Notes

Primary reviews product claim language before merge.

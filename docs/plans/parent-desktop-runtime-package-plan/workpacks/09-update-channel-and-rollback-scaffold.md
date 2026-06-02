# 09 Update Channel And Rollback Scaffold

Sources: [20-step plan](../parent-desktop-runtime-package-20-step-plan.md),
[test blueprint](../parent-desktop-runtime-package-test-blueprint.md),
[requirements guide](../runtime-package-requirements-guide.md), and
[folder README](../README.md).

## Where We Are

Updater scaffolding exists, but production update signing and rollback behavior
are not fully proved.

## Where We Want To Be

Update state distinguishes scaffold, preview, signature-required, rollback
available, rollback unavailable, and production boundary.

## Requirement Checklist

- [x] Represent update channel and rollback states.
- [x] Keep unsigned preview labels.
- [x] Reject production claims without signing proof.
- [x] Add tests for matrix/proof output.
- [x] Update release docs when status changes.

## Acceptance And Proof

Parents/support can tell whether update behavior is production, preview, or
manual-required.

Current proof: scaffold, unsigned-preview, signature-required, production
promotion, and rollback-unavailable states are parsed and tested. Rollback
available is rejected for unsigned preview or unpromoted production state.

## Parallel Ownership Notes

Do not bypass update signature rules in production code.

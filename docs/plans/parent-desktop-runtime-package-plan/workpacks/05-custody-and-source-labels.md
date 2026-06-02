# 05 Custody And Source Labels

Sources: [20-step plan](../parent-desktop-runtime-package-20-step-plan.md),
[test blueprint](../parent-desktop-runtime-package-test-blueprint.md),
[requirements guide](../runtime-package-requirements-guide.md), and
[folder README](../README.md).

## Where We Are

Platform docs require clear data source and custody labels. Package surfaces
must not imply hosted child activity storage.

## Where We Want To Be

Desktop states label live local, LAN, relay, parent cache, parent-owned storage,
Ocentra-hosted non-activity metadata, or unavailable.

## Requirement Checklist

- [ ] Include source labels in command/proof output.
- [ ] Keep activity custody local/parent-owned by default.
- [ ] Label relay/cache unavailable states.
- [ ] Avoid hosted child data claims.
- [ ] Update docs if custody state changes.

## Acceptance And Proof

Package/runtime output makes custody source visible to parent surfaces and
support diagnostics.

## Parallel Ownership Notes

C renders these labels; D proves package/runtime availability.

# 11 Support Diagnostics And Redaction

Sources: [20-step plan](../parent-desktop-runtime-package-20-step-plan.md),
[test blueprint](../parent-desktop-runtime-package-test-blueprint.md),
[requirements guide](../runtime-package-requirements-guide.md), and
[folder README](../README.md).

## Where We Are

Support diagnostics are needed for package/manual testing but must avoid private
child data leakage.

## Where We Want To Be

Support output includes version, commit, platform, package state, route state,
service state, and non-private diagnostic ids with redaction.

## Requirement Checklist

- [ ] Define allowed support fields.
- [ ] Redact secrets, tokens, raw journals, SQLite data, and child activity.
- [ ] Include package/service/source state.
- [ ] Test redaction.
- [ ] Document support use.

## Acceptance And Proof

Support output is useful for troubleshooting and safe to share in reports.

## Parallel Ownership Notes

Reports/export features are separate from support diagnostics.

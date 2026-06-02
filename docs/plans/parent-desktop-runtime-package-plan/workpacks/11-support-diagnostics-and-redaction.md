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

- [x] Define allowed support fields.
- [x] Redact secrets, tokens, raw journals, SQLite data, and child activity.
- [x] Include package/service/source state.
- [x] Test redaction.
- [x] Document support use.

## Acceptance And Proof

Support output is useful for troubleshooting and safe to share in reports.

Current proof: support diagnostics require version, commit, platform, package,
service, route, capability, and degraded-state fields. The proof rejects
unredacted sensitive values and the release helper strips forbidden diagnostic
fields from support output.

## Parallel Ownership Notes

Reports/export features are separate from support diagnostics.

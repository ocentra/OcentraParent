# 07 Unmanaged Browser Fallback

Sources: [20-step plan](../v0-8-enforcement-control-20-step-plan.md),
[test blueprint](../v0-8-enforcement-control-test-blueprint.md), and
[folder README](../README.md).

## Where We Are

Unmanaged browser states can be shown as possible bypass and process-only
fallback, not exact page proof. The V0.8 browser/enforcement timer recovery
proof adds schema-backed unmanaged fallback rows for process identity required,
report-only, warn-child, parent-review, terminate-process, relaunch-managed
manual-required, degraded, and unavailable states while rejecting exact
URL/tab/title/content claim upgrades.

## Where We Want To Be

Unmanaged browser handling is honest: detect, warn, report possible bypass, or
perform scoped process action where proved.

## Requirement Checklist

- [x] Never attach exact URL, title, tab, or content to unmanaged process state.
- [x] Require explicit process identity for process fallback.
- [x] Label possible bypass and manual-required exact URL gaps.
- [x] Keep managed/unmanaged states separate in UI and proof JSON.
- [x] Add tests for unsupported and degraded states.

## Acceptance And Proof

Unmanaged browser tests fail if exact URL action is inferred from process or
network metadata. Current proof lives in
`packages/parent-domain/tests/v0-8-browser-enforcement-timer-recovery-proof.test.ts`
and
`scripts/test/windows-managed-unmanaged-browser-enforcement-proof.mjs`; latest
evidence rows include report-only, parent-review, terminate-process,
relaunch-managed manual-required, degraded, unavailable, and exact
URL/tab/title/content not-claimed states.

## Parallel Ownership Notes

This workpack protects product truth. It should be reviewed anytime browser
copy, policy targets, or proof labels change.

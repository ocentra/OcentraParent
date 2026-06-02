# 07 Unmanaged Browser Fallback

Sources: [20-step plan](../v0-8-enforcement-control-20-step-plan.md),
[test blueprint](../v0-8-enforcement-control-test-blueprint.md), and
[folder README](../README.md).

## Where We Are

Unmanaged browser states can be shown as possible bypass and process-only
fallback, not exact page proof.

## Where We Want To Be

Unmanaged browser handling is honest: detect, warn, report possible bypass, or
perform scoped process action where proved.

## Requirement Checklist

- [ ] Never attach exact URL, title, tab, or content to unmanaged process state.
- [ ] Require explicit process identity for process fallback.
- [ ] Label possible bypass and manual-required exact URL gaps.
- [ ] Keep managed/unmanaged states separate in UI and proof JSON.
- [ ] Add tests for unsupported and degraded states.

## Acceptance And Proof

Unmanaged browser tests fail if exact URL action is inferred from process or
network metadata.

## Parallel Ownership Notes

This workpack protects product truth. It should be reviewed anytime browser
copy, policy targets, or proof labels change.

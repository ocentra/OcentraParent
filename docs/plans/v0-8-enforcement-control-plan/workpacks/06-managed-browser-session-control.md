# 06 Managed Browser Session Control

Sources: [20-step plan](../v0-8-enforcement-control-20-step-plan.md),
[test blueprint](../v0-8-enforcement-control-test-blueprint.md), and
[folder README](../README.md).

## Where We Are

Managed browser status and manual-required exact URL states are represented, but
managed browser enforcement is not product-complete.

## Where We Want To Be

Managed browser actions apply only to an Ocentra-managed profile/session with a
validated bridge and service-owned session id.

## Requirement Checklist

- [ ] Reject unmanaged or stale bridge/session ids.
- [ ] Separate managed session intervention from exact URL action.
- [ ] Show unsupported, degraded, bridge-unavailable, and manual-required states.
- [ ] Avoid page body, form, cookie, token, or decrypted content claims.
- [ ] Add browser evidence refs to action/audit output.

## Acceptance And Proof

Service and UI proof distinguish managed profile/session state from unmanaged
browser process detection.

## Parallel Ownership Notes

Do not collapse browser control into app/process control. Exact URL claims need
their own proof.

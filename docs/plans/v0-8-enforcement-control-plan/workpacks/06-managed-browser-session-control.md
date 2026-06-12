# 06 Managed Browser Session Control

<!-- agent-capsule -->

> Agent Capsule
> Plan: `v0-8-enforcement-control-plan`
> Doc: `06 Managed Browser Session Control`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

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

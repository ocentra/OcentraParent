# 16 Tamper/Uninstall Non-Claim Design

<!-- agent-capsule -->

> Agent Capsule
> Plan: `v0-8-enforcement-control-plan`
> Doc: `16 Tamper/Uninstall Non-Claim Design`
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

Tamper/uninstall protection is not product-proved. The roadmap tracks it, but
no stealth or persistence behavior is approved.

## Where We Want To Be

The product has an explicit design for install health, permission loss, removal,
alerts, support/admin removal, and platform-specific proof before hardening.

## Requirement Checklist

- [ ] Document allowed and forbidden behavior.
- [ ] Define parent-visible removed, stopped, permission-denied, and unsupported
      states.
- [ ] Keep removal/support paths documented.
- [ ] Require security review before hardening.
- [ ] Keep proof output manual-required until real platform artifacts exist.

## Acceptance And Proof

No code or docs claim anti-tamper protection until the design and proof gate are
complete.

## Parallel Ownership Notes

This is a product/security boundary. Do not implement stealth or privilege
behavior in a worker lane.

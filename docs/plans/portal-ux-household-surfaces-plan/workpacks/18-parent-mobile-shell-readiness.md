# 18 Parent Mobile Shell Readiness

<!-- agent-capsule -->

> Agent Capsule
> Plan: `portal-ux-household-surfaces-plan`
> Doc: `18 Parent Mobile Shell Readiness`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

Sources: [20-step plan](../portal-ux-household-surfaces-20-step-plan.md), [test blueprint](../portal-ux-household-surfaces-test-blueprint.md), [UI/UX guide](../ui-ux-requirements-guide.md), and [folder README](../README.md).

## Ownership boundary

```text
portal UX owns responsive, touch-friendly, small-width parent portal presentation patterns.
parent-client-runtime-distribution-plan owns parent mobile Android/iOS package/runtime distribution proof.
child-agent-runtime-distribution-plan owns child mobile agent/runtime proof.
```

## Where We Are

Parent mobile is scaffold/proof-first. The portal should prepare reusable patterns without claiming mobile child-agent parity.

Installable parent mobile Android/iOS package-preview targets now exist as separate scaffold apps from child-agent mobile previews.

## Where We Want To Be

Source-state, selected-device, policy, activity, approval, and report patterns can map to parent mobile shells later.

## Required proof fields

The selected proof must name, at minimum:

```text
route
viewport
responsive_state
touch_target_state
source_label_state
authority_label_state
desktop_assumption_state
parent_mobile_package_boundary_state
child_agent_mobile_boundary_state
manual_required_state
no_claim
```

These are proof-routing fields, not implementation code prescriptions.

## Requirement Checklist

- [ ] Keep layouts responsive and touch-friendly where practical.
- [ ] Preserve source/authority labels on small widths.
- [ ] Avoid desktop-only assumptions in reusable components.
- [ ] Label mobile runtime gaps honestly.
- [ ] Track parent mobile Android/iOS package-preview CI targets separately from child-agent mobile previews.
- [ ] Test narrow widths for key workflows.

## Acceptance And Proof

Responsive portal proof supports future parent mobile UX without claiming child mobile support.

Proof must distinguish responsive portal layout, parent mobile shell/package boundaries, and child-agent mobile boundaries.

## Failure conditions

- Do not claim parent mobile runtime readiness from responsive web proof.
- Do not claim child mobile support from parent mobile shell proof.
- Do not hide manual-required mobile/platform gaps.
- Do not omit source/authority labels on narrow widths.

## Parallel Ownership Notes

D owns parent desktop/package runtime. Mobile runtime proof remains separate.

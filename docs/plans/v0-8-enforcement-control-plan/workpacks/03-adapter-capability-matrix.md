# 03 Adapter Capability Matrix

<!-- agent-capsule -->

> Agent Capsule
> Plan: `v0-8-enforcement-control-plan`
> Doc: `03 Adapter Capability Matrix`
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

Current proof distinguishes several supported, manual-required, and unavailable
states. The matrix must become the durable truth for all parent-visible claims.

## Where We Want To Be

Every platform, adapter, surface, and action has a proof level and capability
reason that service, portal, docs, and proof JSON agree on.

## Requirement Checklist

- [ ] Track platform, adapter kind, surface, action, permission, dependency, and
      proof level.
- [ ] Separate implemented, report-only, scaffold, unavailable, degraded, and
      manual-required.
- [ ] Add tests that prevent accidental claim upgrades.
- [ ] Feed the matrix into service read models and proof output.
- [ ] Update feature docs/checklist when a row changes.

## Acceptance And Proof

Proof JSON and parent-visible state match the matrix exactly.

## Parallel Ownership Notes

This is a shared guardrail for A. Other lanes may consume these states but must
not define their own capability truth.

# 10 NetBIOS, LLMNR, And Reverse DNS

<!-- agent-capsule -->

> Agent Capsule
> Plan: `lan-plan`
> Doc: `10 NetBIOS, LLMNR, And Reverse DNS`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

Sources: [20-step plan](../v0-9-lan-discovery-20-step-plan.md),
[test blueprint](../v0-9-lan-discovery-test-blueprint.md),
[UI/UX guide](../ui-ux-requirements-guide.md), and
[folder README](../README.md).

## Where We Are

Windows-heavy name enrichment is still planned discovery work. Existing proof
must not treat hostnames or reverse DNS as owner, child profile, or permanent
identity.

## Where We Want To Be

NetBIOS, LLMNR, and reverse DNS provide optional name evidence. They improve
display and classification but cannot confirm child identity or automatic child
assignment.

## Requirement Checklist

- [ ] Normalize NetBIOS, LLMNR, and reverse DNS output as name evidence.
- [ ] Track source, confidence, first-seen, last-seen, and interface.
- [ ] Reject malformed, oversized, and unsafe display values.
- [ ] Keep same-hostname-only matches below auto-merge threshold.
- [ ] Add regression fixtures for duplicate names and long names.

## Acceptance And Proof

- Parser tests cover Windows hostname cases, missing values, duplicate
  hostnames, malformed packets, and invalid text.
- Merge tests prove hostname-only matches never auto-merge.
- UI tests prove hostile or long names do not execute script or break layout.

## Parallel Ownership Notes

Treat this as enrichment only. Do not let a name-source worker add assignment or
confirmation behavior.

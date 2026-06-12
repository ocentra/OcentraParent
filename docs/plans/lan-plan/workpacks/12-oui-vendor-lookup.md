# 12 OUI And Vendor Lookup

<!-- agent-capsule -->

> Agent Capsule
> Plan: `lan-plan`
> Doc: `12 OUI And Vendor Lookup`
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

Current LAN rows may expose MAC/interface data where the host OS provides it,
but production vendor and randomized-MAC confidence logic still needs a focused
model and tests.

## Where We Want To Be

MAC vendor lookup enriches device records while staying honest. Known vendors
can improve classification. Locally administered, randomized, multicast, or
malformed MACs reduce confidence or reject identity use.

## Requirement Checklist

- [ ] Add normalized MAC parsing and validation.
- [ ] Add OUI/vendor lookup with unknown-prefix behavior.
- [ ] Detect locally administered/randomized candidate MACs.
- [ ] Reject multicast and malformed MACs as identity keys.
- [ ] Keep vendor evidence separate from owner, OS, or child identity.

## Acceptance And Proof

- Unit tests cover known vendor prefixes, unknown prefix, lowercase/uppercase,
  malformed MAC, locally administered MAC, and multicast MAC.
- Merge tests reduce confidence for randomized MAC evidence.
- UI/read-model tests expose randomized/private MAC warning when relevant.

## Parallel Ownership Notes

Vendor data can be maintained separately from merge and classification, but its
confidence output must be consumed by both.

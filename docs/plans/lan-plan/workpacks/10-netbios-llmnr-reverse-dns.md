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

The local Rust slice is implemented in `lan-core`: Linux reverse DNS,
NetBIOS, and LLMNR all normalize into weak name-only evidence, reject unsafe
display values, and stay non-controlling in the read model. Existing truth
still must not treat hostnames or reverse DNS as owner, child profile,
automatic child assignment, or permanent identity.

## Where We Want To Be

NetBIOS, LLMNR, and reverse DNS provide optional name evidence. They improve
display and classification but cannot confirm child identity or automatic child
assignment.

## Requirement Checklist

- [x] Normalize NetBIOS, LLMNR, and reverse DNS output as name evidence.
- [x] Track source, confidence, first-seen, last-seen, and interface.
- [x] Reject malformed, oversized, and unsafe display values.
- [x] Keep same-hostname-only matches below auto-merge threshold.
- [x] Add regression fixtures for duplicate names and long names.

## Acceptance And Proof

- Local Rust tests cover reverse-DNS parsing, duplicate hostnames staying
  separate by MAC, hostname-only hints staying below previous-scan trust,
  passive LLMNR and NetBIOS packet parsing, and unsafe or oversized names
  being rejected.
- Read-model tests keep WP10 sources partial and name-only; hostname evidence
  remains weak and non-controlling rather than confirming a child or elevating
  route authority.
- UI/script/layout hardening is not claimed by this Rust-only packet. Any
  browser presentation proof remains outside W10's Rust ownership.

## Parallel Ownership Notes

Treat this as enrichment only. Do not let a name-source worker add assignment or
confirmation behavior.

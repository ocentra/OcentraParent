# 10 NetBIOS, LLMNR, And Reverse DNS

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

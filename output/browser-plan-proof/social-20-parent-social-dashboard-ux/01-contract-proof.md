# SOCIAL-20 Contract Proof

The parent social dashboard UX snapshot requires section contracts for:

- account approval queue;
- feed/video gates;
- native app capability;
- connector boundaries;
- decision memory;
- manual-required gaps.

Each section carries a stable action, status, severity, sort order, evidence
refs, and reason codes. Account and feed/video sections can be ready for review.
Native, connector, and gap sections remain manual-required. Decision memory is
contract-only.

The focused Vitest suite accepts an honest six-panel snapshot and rejects
missing sections, rendered UI/runtime claims, and unsupported section status or
action upgrades.

The text-domain suite accepts schema-backed copy tokens for the social review
title, section labels, status labels, and action labels. The exact-copy checks
prove manual-required and contract-only wording remains visible instead of
implying connector authorization, rendered UI, or enforcement.

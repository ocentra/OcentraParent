# @ocentra-parent/activity-domain

Thin package boundary for activity-domain metadata and focused proof coverage.

## Public Surface

- `./package-info` via [src/package-info.ts](C:\Users\sujan\.codex\worktrees\ocentra-parent-codex-a\OcentraParent\packages\activity-domain\src\package-info.ts)

## What Stays Here

- package identity and boundary metadata
- focused unit/contract proofs that exercise canonical activity-related schemas

## What Does Not Stay Here

- shared activity capture, journal, query, activity-surface, family-aggregation, or screen VLM schema ownership
- cross-package schema/value/id/read-model contract definitions that already live in `@ocentra-parent/schema-domain/*`
- runtime orchestration, policy authority, transport, portal UI, or enforcement logic

## Current Contract Shape

The live tests in this package import canonical schemas directly from `@ocentra-parent/schema-domain/*`. This package no longer publishes local schema wrapper leaves for:

- activity capture
- activity query
- activity journal
- activity surface
- activity family aggregation
- screen VLM journal/read-model projection

## Validation Intent

Keep validation focused on:

- building this package
- proving the local tests still parse the canonical central schemas
- architecture lint for this package boundary

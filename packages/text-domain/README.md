# @ocentra-parent/text-domain

Thin package boundary for text-domain metadata and focused proof coverage.

## Public Surface

- `./package-info` via [src/package-info.ts](C:\Users\sujan.codex\worktrees\ocentra-parent-codex-a\OcentraParent\packages\text-domain\src\package-info.ts)

## What Stays Here

- package identity metadata
- focused unit proofs that exercise canonical text contracts in `@ocentra-parent/schema-domain`

## What Does Not Stay Here

- shared display text token ownership
- cross-package text schemas, decode helpers, or token catalogs
- local pass-through wrappers for browser, portal, or social text contracts once a canonical schema-domain owner exists

## Current Contract Shape

The live tests in this package import canonical text contracts directly from `@ocentra-parent/schema-domain/*`. This package no longer publishes local schema wrapper leaves for:

- browser child UX text
- browser parent explanation text
- portal dev text
- social child approval block UX text
- social dashboard UX text

## Validation Intent

Keep validation focused on:

- building this package
- proving the local tests still parse or resolve the canonical central text contracts
- architecture lint for this package boundary

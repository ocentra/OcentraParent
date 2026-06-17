# Repo Audit Agent Router

This folder is the structural cleanup route for OcentraParent. Use it before per-plan implementation when the assignment mentions test topology, CI coverage, architecture gates, source ownership, repeated code, or false-green proof.

## Route order

1. Read this file.
2. Read `INDEX.md`.
3. Read `2026-06-17-structural-truth-audit.md`.
4. Read `NEXT_ACTIONS.md`.
5. Read `WORKPACK_INDEX.md`.
6. Select exactly one workpack.
7. Open only the selected workpack and exact files it names.

## Stop rule

Do not scan all plans, all source, all tests, or all docs unless the selected workpack explicitly requires an inventory. Prefer generated inventories and narrow path lists over manual wandering.

## Structural cleanup rule

Structural work comes before plan closure. Do not accept any per-plan `done`, `complete`, or `PR_READY` claim if the claim relies on:

- empty test scaffold folders;
- inline source tests that should be public-boundary tests;
- generated proof paths without tracked generator commands;
- scoped architecture passes presented as repo-wide clean;
- broad frontage packages owning narrow-domain behavior;
- duplicated runtime/event code that should be common or explicitly isolated.

## Completion rule

A structural workpack is complete only when it updates the audit docs, the relevant inventory/matrix, and any generator/check script needed to keep the finding from recurring.

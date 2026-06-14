# Tracking Agent Driver — Workpack Execution Rules

## Purpose

This document tells Codex how to execute tracking work without scanning the entire plan or making proof-only changes.

## Branch setup

Use the assigned branch:

```text
codex/tracking-plan-full-continuation-a
```

Before editing:

```bash
git fetch origin codex/tracking-plan-full-continuation-a
git checkout codex/tracking-plan-full-continuation-a
git pull --ff-only origin codex/tracking-plan-full-continuation-a
```

## Required read order

Read in this order only:

```text
1. docs/plans/tracking-plan/README.md
2. docs/plans/tracking-plan/PLAN_STATE.md
3. docs/plans/tracking-plan/NEXT_ACTIONS.md
4. docs/plans/tracking-plan/WORKPACK_INDEX.md
5. docs/plans/tracking-plan/agent-driver/00-current-state-audit.md
6. this file
7. the one assigned workpack
8. exact checklist/proof rows named by that workpack
```

Do not open every workpack.

Do not open `README_FULL_ORIGINAL.md` unless the assigned workpack explicitly requires historical context.

Do not scan the full implementation checklist. Use exact rows only.

## Assignment rule

Every implementation run needs exactly one assigned workpack.

Required pre-edit note:

```text
Assigned workpack:
Reason selected:
Open checklist rows targeted:
Expected source files:
Expected tests:
Expected proof root:
Manual-required/no-claim boundaries:
```

If the task is broad, split it into one workpack first.

## Implementation-first rule

Use this order:

```text
PLAN -> CODE -> TEST -> RUN/FIX -> PROOF -> DOC
```

Meaning:

```text
1. Read plan/workpack.
2. Modify real source code or tests.
3. Run focused tests or record a real blocker.
4. Generate proof only after tests or blocker evidence exists.
5. Update workpack/checklist/docs last.
```

Do not start by updating proof JSON, checklist status, or README claims.

## Proof-only exception

Proof-only work is allowed only when the assigned workpack says it is proof routing, artifact inventory, or checklist reconciliation.

If proof-only, the pre-edit note must say:

```text
This is proof-routing-only because:
Source behavior already exists at:
Validation command proving it:
Proof artifact to refresh:
```

Otherwise proof-only changes are rejected.

## Source change requirement

For implementation work, at least one of these must change:

```text
packages/tracking-domain/src/**
packages/agent-protocol-domain/src/**
crates/tracking-core/src/**
crates/agent-protocol/src/**
crates/agent-service/src/**
apps/portal/src/**
packages/portal-domain/src/**
relevant test files
```

If no real source or test changes are needed, the work is not an implementation slice.

## Test requirement

Each slice must name and run focused tests.

Prefer narrow commands first:

```bash
npm run test --workspace @ocentra-parent/tracking-domain -- <test-file>
cargo test -p ocentra-tracking-core <test-name>
cargo test -p ocentra-parent-agent-protocol <test-name>
cargo test -p ocentra-parent-agent-service <test-name>
```

Then run broader validation only when the focused slice passes.

## Proof update rule

After tests pass, update the assigned proof root only.

Example:

```text
output/tracking-plan-proof/<workpack-id>/
test-results/<proof-mode>/
```

Do not scatter proof updates across unrelated workpack roots.

## Checklist update rule

A checkbox may be marked checked only when:

```text
1. the assigned workpack acceptance row is filled,
2. focused commands are listed,
3. proof artifacts exist,
4. no-claim boundaries are stated,
5. matching implementation-checklist row is updated if required.
```

Do not mark product-ready or broad DONE from a local proof.

## No-claim boundary rule

Tracking claims are proof-tiered.

Do not claim these unless assigned proof artifacts exist:

```text
physical Android behavior
iOS background/region behavior
authority-enrolled device behavior
provider delivery behavior
production worker behavior
product-ready tracking
```

Local emulator, simulator, hosted UI, and fixture proof must remain named as such.

## Stop rules

Stop and report instead of guessing when:

```text
- assigned workpack is missing or ambiguous
- source owner is unclear between TS/Rust/service/portal
- proof artifact path does not exist and workpack expects it
- command requires a physical device or OS not available in the current environment
- checklist row would require a product-ready claim
- changes would touch sibling workpacks
```

## Completion report format

Use this exact structure:

```text
Assigned workpack:
Source behavior added:
Files changed:
Tests changed:
Commands run:
Proof artifacts:
Checklist/docs updated:
No-claim boundaries preserved:
Remaining gaps:
```

If `Source behavior added` is empty, do not call it implementation complete.

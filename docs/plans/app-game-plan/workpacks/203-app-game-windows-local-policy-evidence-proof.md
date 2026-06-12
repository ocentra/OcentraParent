# WP203 App/Game Windows Local Policy Evidence Proof

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP203 App/Game Windows Local Policy Evidence Proof`
> Kind: proof reference; read only when validating matching claim.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

## Scope

Sample Windows local AppLocker and App Control policy evidence as parent-safe
counts and booleans before any broad app/game blocking claim.

This moves Windows broad-blocking work beyond static manual-gate preflight by
checking the local AppIDSvc service, AppLocker local policy readability, and
Device Guard/App Control state without storing raw policy XML, executable
paths, publisher rules, or private policy details.

## Implementation

- Added
  `packages/parent-domain/src/app-game-windows-local-policy-evidence-proof.ts`.
- Added
  `packages/parent-domain/tests/app-game-windows-local-policy-evidence-proof.test.ts`.
- Added `scripts/test/app-game-windows-local-policy-evidence-proof.mjs`.

## Validation

Focused validation for this workpack:

```powershell
cmd /c npm run test --workspace @ocentra-parent/parent-domain -- app-game-windows-local-policy-evidence-proof
cmd /c node scripts/test/app-game-windows-local-policy-evidence-proof.mjs
```

## Proof

- `test-results/app-game-windows-local-policy-evidence-proof/proof.json`
- `output/app-game-plan-proof/203-app-game-windows-local-policy-evidence-proof/proof.json`

## Boundaries

Proved:

- Windows local AppLocker/App Control policy state can be sampled as
  parent-safe counts and booleans.
- Raw AppLocker policy XML, executable paths, publisher rules, and private
  policy details are not stored.
- Broad app/game blocking remains blocked until enforce proof, system-app
  allowlist proof, rollback proof, audit custody proof, adapter dispatch proof,
  and child-device delivery proof exist.

Not proved:

- Windows broad installed-app launch blocking execution.
- System-app allowlist execution.
- Rollback execution or audit custody.
- Adapter dispatch, platform enforcement, provider delivery, child-device
  delivery, raw executable path custody, or raw policy XML custody.

# WP160 - App/game evidence boundary safety gates

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP160 - App/game evidence boundary safety gates`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Scope

Close the app/game merge-blocking evidence-boundary gates for the parent
dashboard surface in one batch.

## Implementation

- Portal activity UI tests now include dedicated app/game rows for each
  evidence-boundary failure mode.
- Inventory-only rows keep inventory count without running, foreground, or
  duration upgrades.
- Running-only rows keep running count without foreground or duration upgrades.
- Foreground rows expose foreground count and evidence refs without leaking
  private foreground content strings.
- Launcher rows stay launcher-only and do not become active native game proof.
- Unknown process rows stay review candidates and are not auto-promoted to
  known games.

## No-Claim Boundary

This is a portal dashboard negative-proof gate. It does not claim new source
adapters, classifier provider execution, policy enforcement, adapter dispatch,
broad blocking, platform enforcement, child-game proof, or private content
visibility. The central product capability checklist remains untouched while
E-B owns that lock.

## Validation

See
`output/app-game-plan-proof/160-app-game-evidence-boundary-safety-gates/10-validation-commands.log`.

# WP160 - App/game evidence boundary safety gates

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

# WP161 - App/game enforcement platform no-claim safety gates

## Branch

`codex/app-game-control-product-completion`

## Scope

WP161 closes the remaining app/game merge-blocking no-claim gates in one
parent-domain batch: AI output is not enforcement, dry-run is not block/kill,
manual-required is not adapter execution, Android normal mode is not
hide/suspend authority, iOS is not process scan/kill authority, and macOS hard
block requires privileged proof.

## Runtime Boundary

- Parent-domain authority tests cover action-result status and capability
  consistency.
- Parent-domain platform authority tests cover Android, iOS, and macOS hard
  control proof gates.
- Existing AI classifier boundary tests cover direct-action rejection and
  evidence-only handoff.
- One platform authority rule was hardened so supported iOS terminate-process,
  block-launch, and allowlist claims are rejected.

## No-Claim Boundaries

- No provider delivery execution.
- No external receipt ingestion.
- No adapter dispatch.
- No broad blocking.
- No platform enforcement.
- No raw private row access.
- No raw target value exposure.
- No private diagnostics.
- `docs/product-capability-checklist.md` intentionally remains untouched.

## Validation

See `10-validation-commands.log`.

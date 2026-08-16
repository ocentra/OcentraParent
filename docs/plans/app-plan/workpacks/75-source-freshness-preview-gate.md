# WP75 Native App Source Freshness Preview Gate

## Scope

Cross-record the shared app/game WP75 proof for native apps. The source
freshness preview gate validates that native app policy previews only become
preview-ready when the source freshness readiness row has already accepted
fresh inventory, runtime, and foreground evidence.

Native games use the same low-level evidence spine, but separate product meaning
is preserved through the shared app/game target-domain checks.

## Owned Proof

- Shared workpack:
  `docs/plans/app-game-plan/workpacks/75-source-freshness-preview-gate.md`
- Evidence:
  `output/app-plan-proof/75-source-freshness-preview-gate`
- Test harness:
  `scripts/test/app-game-source-freshness-preview-gate-proof.mjs`

## Acceptance

- Native app preview gate rows cite source freshness readiness and source
  evidence refs.
- Stale, missing, manual-required, unavailable, or not-claimed source freshness
  blocks preview before compiled policy output is accepted.
- Policy preview rows remain dry-run-only and do not claim evaluator runtime,
  timers, adapter dispatch, child delivery, broad app blocking, or platform
  enforcement.

## Non-Goals

- No native app portal authoring UI.
- No service persistence or WebSocket event.
- No package export while another lane owns `packages/parent-domain/package.json`.
- No platform adapter execution or broad app blocking.

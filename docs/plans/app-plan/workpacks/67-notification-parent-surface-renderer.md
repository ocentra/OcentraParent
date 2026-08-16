# 67. Notification Parent Surface Renderer

Cross-recorded from shared app/game WP67.

## Goal

Render native app notification parent-surface intent rows in the Portal
App/Game Sessions route without claiming notification delivery, parent
preference mutation, or runtime service production behavior.

## Scope

- Reuse the shared WP66 parent-surface intent read model.
- Render redacted native app history/preference setup status, drill-in refs,
  scheduler/outbox refs, provider/preference status, quiet-hours status, and
  manual-proof requirements.
- Preserve an explicit missing-service empty state.
- Keep native app and native game meanings separate while sharing the same
  low-level app/game notification evidence spine.

## Non-Goals

- Parent preference mutation, frequency controls, or quiet-hours editor
  behavior.
- Provider delivery, credentials, webhooks, receipts, cloud routing, child
  delivery, production retry/quiet-hours workers, durable production storage,
  policy evaluator execution, broad app blocking, mobile UI, or platform
  support.

## Code-pass status

The existing parent-runtime notification panel now consumes the joined
parent-surface intent rows emitted in the agent-service notification-readiness
payload. The existing Portal App/Game Sessions renderer displays redacted
surface, provider, preference, quiet-hours, drill-in, audit, and manual-proof
metadata. Older payloads without a joined intent retain the typed provider/
preference boundary fallback; no live provider or parent mutation is claimed.
The workpack remains unvalidated with tests/proof/checklist deferred.

## Proof

- `apps/portal/src/AppGameNotificationParentSurfaceRoutePanel.tsx`
- `apps/portal/src/app-game-notification-parent-surface-panel.ts`
- `apps/portal/tests/app-game-notification-parent-surface-panel.test.ts`
- `scripts/test/app-game-notification-parent-surface-ui-proof.mjs`
- `test-results/app-game-notification-parent-surface-ui-proof/proof.json`
- `output/app-plan-proof/67-notification-parent-surface-renderer/`
- `output/app-game-plan-proof/67-notification-parent-surface-renderer/`

## Validation

- [ ] Schema-backed native app parent-surface intent rows render route-level
      status and manual-proof metadata.
- [ ] Missing service input stays explicit instead of inventing rows.
- [ ] No live service event, parent preference mutation, provider delivery,
      receipt ingestion, credentials, production runtime, child delivery, broad
      app blocking, mobile UI, or platform support is claimed.
- [ ] Product checklist unchanged because this proof does not move feature
      status and runtime/provider/platform gaps remain.

# 21 Child-Facing App/Game Warning And Request UX

## Target State

Child-facing UX explains warnings, limits, and approval requests without shame,
raw diagnostics, or hidden enforcement behavior.

## Scope

- App limited by family rules.
- New app needs approval.
- Game time almost finished.
- Game or app blocked/manual-required state.
- Ask parent for more time.
- Permission/unavailable state.

## Tests And Proof

- Text is parent-rule based, not AI-blame based.
- Child UI does not leak private paths or diagnostics.
- Ask-parent request carries evidence refs and child reason refs.
- Manual-required/unavailable states are honest.

## Done Signal

Child UX is respectful, actionable, and consistent with actual capability state.

Use the standard checklist in [workpacks README](README.md).

## Completion - 2026-06-03

- Owner: `codex-c`
- Branch: `codex/app-game-read-model-service-events`
- Proof root:
  `output/app-game-plan-proof/21-child-facing-warning-and-request-ux/`
- Native app cross-record:
  `output/app-plan-proof/20-child-facing-app-warning-block-request-ux/`

Completed:

- Added `packages/parent-domain/src/app-game-child-facing-ux-rules.ts` and
  `packages/parent-domain/src/app-game-child-facing-ux.ts`.
- Added `packages/text-domain/src/app-game-child-ux-text.ts`.
- Added focused parent-domain and text-domain tests for child warning,
  approval-needed, time-limit, request submitted/approved/denied,
  manual-required, and unavailable states.
- Proved request actions require approval, evidence, child reason, and child
  status refs; manual-required/unavailable states cannot claim adapter action
  refs; child copy avoids blame, AI attribution, private paths, executable
  names, and parent-only diagnostics.

Deferred:

- Live child UI, native overlay rendering, portal preview screenshots,
  notification delivery, service persistence, Rust/WebSocket parity, adapter
  execution, and platform-specific child shield/block behavior remain later
  workpacks.

# 21 Child-Facing App/Game Warning And Request UX

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `21 Child-Facing App/Game Warning And Request UX`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

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

- [x] Text is parent-rule based, not AI-blame based.
- [x] Child UI contract cannot accept private paths, arbitrary copy, or diagnostics.
- [x] Ask-parent request carries evidence refs and child reason/status refs.
- [x] Manual-required/unavailable states are honest.

## Done Signal

Child UX is respectful, actionable, and consistent with actual capability state.

Use the standard checklist in [workpacks README](README.md).

## Current Status - Phase 1/2 Complete; Phase 3 Open

The 2026-08-15 code audit found that the historical TypeScript owners and tests
below are no longer tracked. The current Rust owner is now
`ocentra-app-game-core`: commit `c9bb2e153` adds a controlled-token child UX
contract over the WP17 approval lifecycle and WP51 runtime decisions. It covers
limited, approval-needed, time-warning, request submitted/approved/denied,
manual-required, and unavailable states without accepting arbitrary copy,
diagnostics, private paths, or adapter-action claims.

Verified on that code checkpoint:

- focused child UX contract tests: 5/5 passed;
- full `ocentra-app-game-core` contract target: 79/79 passed;
- full crate unit target: 10/10 passed;
- Clippy with `-D warnings`, focused Enforcer, Rust formatting, hub guard, and
  pre-commit passed.

Live child rendering/overlay, durable delivery/outbox, notifications, service
persistence, screenshots, platform shield behavior, adapter execution, retained
proof, and whole-plan gates remain open. Therefore the workpack is in validation,
not DONE.

## Historical Contract Completion - 2026-06-03

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

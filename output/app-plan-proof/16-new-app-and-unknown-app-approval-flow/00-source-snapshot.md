# Source Snapshot

Date: 2026-06-03

Branch: `codex/app-game-read-model-service-events`

Scope:

- Native app-plan WP16 new app and unknown app approval flow.
- Shared app/game WP17 contract implementation is reused instead of creating
  parallel app-only approval truth.

Source docs read:

- `docs/feature-list.md`
- `docs/features/app-game-control.md`
- `docs/features/policy-schedules-approvals.md`
- `docs/expectations/app-game-evidence.md`
- `docs/expectations/policy.md`
- `docs/expectations/enforcement.md`
- `docs/expectations/app-install-purchase-approval.md`
- `docs/plans/app-plan/README.md`
- `docs/plans/app-plan/source-index.md`
- `docs/plans/app-plan/current-app-snapshot.md`
- `docs/plans/app-plan/implementation-checklist.md`
- `docs/plans/app-plan/workpacks/16-new-app-and-unknown-app-approval-flow.md`
- `docs/plans/app-game-plan/source-index.md`
- `docs/plans/app-game-plan/workpacks/17-unknown-app-game-approval-flow.md`
- targeted native app full-scope, platform, test, and UI guide sections for
  unknown, approval, manual-required, and child request states.

Before state:

- Native app plan proof mirrored app/game evidence through WP15.
- New/unknown native app approval was still an unproved product gap.

Changed source:

- `packages/parent-domain/src/app-game-control-approval-flow.ts`
- `packages/parent-domain/src/app-game-control-authority.ts`
- `packages/parent-domain/src/app-game-control-authority-rules.ts`
- `packages/parent-domain/tests/app-game-unknown-approval-flow.test.ts`
- `docs/plans/app-plan/current-app-snapshot.md`
- `docs/plans/app-plan/implementation-checklist.md`
- `docs/plans/app-plan/source-index.md`
- `docs/plans/app-plan/workpacks/16-new-app-and-unknown-app-approval-flow.md`

No source changed:

- No Rust protocol, service, portal, notification, or platform adapter code was
  changed in this contract-only slice.
- Native app approval UI and live candidate production remain later work.

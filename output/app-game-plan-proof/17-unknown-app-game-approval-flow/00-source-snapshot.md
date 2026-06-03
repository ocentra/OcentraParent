# Source Snapshot

Date: 2026-06-03

Branch: `codex/app-game-read-model-service-events`

Scope:

- App-game WP17 unknown app/game approval flow.
- Native app-plan WP16 mirror for new and unknown native app approval states.

Source docs read:

- `docs/feature-list.md`
- `docs/features/app-game-control.md`
- `docs/features/policy-schedules-approvals.md`
- `docs/expectations/app-game-evidence.md`
- `docs/expectations/policy.md`
- `docs/expectations/enforcement.md`
- `docs/expectations/app-install-purchase-approval.md`
- `docs/plans/app-game-plan/README.md`
- `docs/plans/app-game-plan/source-index.md`
- `docs/plans/app-game-plan/current-app-game-snapshot.md`
- `docs/plans/app-game-plan/implementation-checklist.md`
- `docs/plans/app-game-plan/workpacks/17-unknown-app-game-approval-flow.md`
- `docs/plans/app-plan/README.md`
- `docs/plans/app-plan/source-index.md`
- `docs/plans/app-plan/current-app-snapshot.md`
- `docs/plans/app-plan/implementation-checklist.md`
- `docs/plans/app-plan/workpacks/16-new-app-and-unknown-app-approval-flow.md`
- targeted native app full-scope, platform, test, and UI guide sections for
  unknown, approval, manual-required, and child request states.

Before state:

- App/game approval request, decision, and action-result contracts existed.
- Unknown/new app-game candidates did not have a dedicated contract layer for
  candidate kind/source, child status/reason refs, parent response scope,
  expiry, replay state, and safe unanswered fallback.
- The docs tracked unknown approval as an open product gap.

Changed source:

- `packages/parent-domain/src/app-game-control-approval-flow.ts`
- `packages/parent-domain/src/app-game-control-authority.ts`
- `packages/parent-domain/src/app-game-control-authority-rules.ts`
- `packages/parent-domain/tests/app-game-unknown-approval-flow.test.ts`
- `docs/features/app-game-control.md`
- `docs/plans/app-game-plan/current-app-game-snapshot.md`
- `docs/plans/app-game-plan/implementation-checklist.md`
- `docs/plans/app-game-plan/source-index.md`
- `docs/plans/app-game-plan/workpacks/17-unknown-app-game-approval-flow.md`
- `docs/plans/app-plan/current-app-snapshot.md`
- `docs/plans/app-plan/implementation-checklist.md`
- `docs/plans/app-plan/source-index.md`
- `docs/plans/app-plan/workpacks/16-new-app-and-unknown-app-approval-flow.md`

No source changed:

- No Rust protocol, service, portal, notification, or platform adapter code was
  changed in this contract-only slice.
- `packages/parent-domain/README.md` was not changed because that path was
  already locked by another active lane.

# 22 Broad Blocking Proof Gates

Sources: [platform deep dive](../v0-5-native-apps-platform-deep-dive.md),
[test blueprint](../v0-5-native-apps-test-blueprint.md), and
`docs/expectations/enforcement.md`.

## Where We Are

Broad app blocking remains manual-required unless platform-specific authority,
adapter, action, rollback, and UI proof exists. Scoped owned-process terminate
proof does not prove broad launch blocking.

## Where We Want To Be

Every broad-block claim has a named platform mechanism, authority tier, setup
proof, adapter proof, rollback proof, audit proof, and UI state. If proof is
missing, policy returns manual_required or not-claimed and no adapter runs.

## Scope

- Windows AppLocker and App Control proof gates.
- macOS MDM/profile/Endpoint Security/System Extension proof gates.
- Linux distro/session/mechanism-specific cgroup/systemd/AppArmor/SELinux proof
  gates.
- Android Device Owner/Profile Owner/delegated hide/suspend proof gates.
- iOS FamilyControls/ManagedSettings/MDM/App Lock proof gates.
- Rollback, allowlist restore, unblock, unsuspend, unshield, and audit proof.

## Touched Paths

- `packages/parent-domain/src/enforcement-policy-dispatch.ts`
- `packages/activity-domain/src/app-game*.ts`
- platform adapter paths when assigned.
- `docs/plans/app-plan/v0-5-native-apps-platform-deep-dive.md`
- `output/app-plan-proof/<platform>/<capability>/`

## Tests And Proof

- Manual-required state cannot call adapter.
- Android normal mode cannot hide or suspend.
- iOS cannot claim process scanning/killing.
- macOS hard block cannot claim Endpoint Security deny without entitlement/lab
  proof.
- Linux block claim names distro/session/mechanism.
- Windows AppLocker audit proof is not enforce proof.

## AI Worker Checklist

Fill this before reporting `DONE` or PR-ready:

- [ ] Confirm source docs read: [folder README](../README.md), [source index](../source-index.md), [current snapshot](../current-app-snapshot.md), [full scope plan](../v0-5-native-apps-full-scope-plan.md), [platform deep dive](../v0-5-native-apps-platform-deep-dive.md), [test blueprint](../v0-5-native-apps-test-blueprint.md), [UI/UX guide](../ui-ux-requirements-guide.md), [main checklist](../implementation-checklist.md), and this workpack.
- [ ] Confirm this is native/installed-app scope, not browser pages, browser games, or game-specific product semantics unless the source docs explicitly route that handoff.
- [ ] Hub lock covers this workpack and exact implementation/docs paths.
- [ ] Existing app/game source layout inspected; no parallel app-control truth created.
- [ ] Before-state source snapshot recorded in `output/app-plan-proof/22-broad-blocking-proof-gates/00-source-snapshot.md`.
- [ ] Contracts updated first where this workpack changes behavior:
      `packages/parent-domain/src/app-game-broad-blocking-proof-gates.ts`,
      `packages/parent-domain/src/app-game-broad-blocking-proof-gate-data.ts`,
      and `packages/parent-domain/src/app-game-broad-blocking-proof-gate-rules.ts`.
- [ ] Rust/service/portal parity N/A for this slice because no protocol, service, or UI payload changed.
- [ ] Raw evidence artifact captured: `output/app-plan-proof/22-broad-blocking-proof-gates/03-runtime-evidence.json`.
- [ ] Tests/proof listed in this workpack and [test blueprint](../v0-5-native-apps-test-blueprint.md) are implemented as no-claim/manual-required proof.
- [ ] Required fixtures are N/A except manual-required state; manual-required and unavailable gates are covered by the contract matrix.
- [ ] Validation command outputs saved in the proof pack and summarized in [main checklist](../implementation-checklist.md).
- [ ] UI snapshots N/A; `output/app-plan-proof/22-broad-blocking-proof-gates/06-ui-snapshots/ui-not-applicable.md` records why.
- [ ] Security/no-claim negative proof captured in `output/app-plan-proof/22-broad-blocking-proof-gates/08-security-negative-proof.log`.
- [ ] Manual platform proof N/A for support claims; `09-manual-platform-proof.md` records that no claim stronger than manual-required/unavailable/not-claimed moved up.
- [ ] Platform limitations use manual-required, unavailable, and not-claimed language with exact proof needed to move up.
- [ ] Evidence/proof artifact paths recorded in [main checklist](../implementation-checklist.md).
- [ ] Feature/checklist update decision recorded: feature doc and app/app-game checklists updated; product capability checklist unchanged because product status/support did not move and the file is locked by another lane.
- [ ] Known gaps, deferred items, and no-claim boundaries recorded before `DONE`.

## Manual-Required Gaps

If broad-block proof is missing, report manual-required, unavailable, degraded,
or not-claimed. Do not imply completion.

## Completion - 2026-06-03

- Owner/lane: `codex-c`.
- Branch: `codex/app-game-read-model-service-events`.
- Proof root: `output/app-plan-proof/22-broad-blocking-proof-gates/`.
- Shared proof root: `output/app-game-plan-proof/23-broad-blocking-proof-gates/`.
- Contract: `packages/parent-domain/src/app-game-broad-blocking-proof-gates.ts`.
- Matrix data: `packages/parent-domain/src/app-game-broad-blocking-proof-gate-data.ts`.
- Gate rules: `packages/parent-domain/src/app-game-broad-blocking-proof-gate-rules.ts`.
- Test: `packages/parent-domain/tests/app-game-broad-blocking-proof-gates.test.ts`.
- Harness: `node scripts/test/app-game-broad-blocking-proof-gates.mjs`.

This is native app no-claim/manual-required proof only. Broad app blocking
support remains unimplemented until platform setup, authority-tier, rollback,
audit, and platform-specific proof artifacts exist.

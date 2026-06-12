# 23 Broad Blocking Proof Gates

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `23 Broad Blocking Proof Gates`
> Kind: proof reference; read only when validating matching claim.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Target State

Broad app/game blocking, launch prevention, allowlists, hide/suspend/shield, and
strict mode remain manual-required until platform-specific proof exists.

## Scope

- Windows AppLocker/App Control.
- macOS Endpoint Security/MDM/System Extension.
- Linux cgroup/systemd/AppArmor/SELinux/package restrictions.
- Android Device Owner/Profile Owner/DPC.
- iOS FamilyControls/ManagedSettings/supervised/MDM.
- Rollback and safe system-app allow proof.

## Tests And Proof

- Manual-required action cannot call adapter.
- Unsupported/unavailable action cannot call adapter.
- Platform proof names setup, authority tier, rollback, and audit state.
- UI exposes manual-required reason.

## Done Signal

No broad blocking claim exists without platform proof and rollback evidence.

Use the standard checklist in [workpacks README](README.md).

## Completion - 2026-06-03

- Owner/lane: `codex-c`.
- Branch: `codex/app-game-read-model-service-events`.
- Proof root: `output/app-game-plan-proof/23-broad-blocking-proof-gates/`.
- Contract: `packages/parent-domain/src/app-game-broad-blocking-proof-gates.ts`.
- Matrix data: `packages/parent-domain/src/app-game-broad-blocking-proof-gate-data.ts`.
- Gate rules: `packages/parent-domain/src/app-game-broad-blocking-proof-gate-rules.ts`.
- Test: `packages/parent-domain/tests/app-game-broad-blocking-proof-gates.test.ts`.
- Harness: `node scripts/test/app-game-broad-blocking-proof-gates.mjs`.

Proof captured:

- Manual-required app/game broad blocking gates cannot dispatch adapters.
- Unavailable broad blocking gates cannot dispatch adapters.
- Windows AppLocker audit-only evidence is not AppLocker enforce proof.
- Platform proof requirements name setup, authority tier, rollback, audit, and
  platform-specific proof.
- Android normal mode hide/suspend and iOS process killing remain unclaimed
  before platform proof.
- Parent-visible manual-required reasons are explicit instead of generic
  unsupported copy.

No product status moved: this is no-claim/manual-required contract proof only,
not AppLocker/App Control, MDM, Endpoint Security, Device Owner/Profile Owner,
FamilyControls/ManagedSettings, cgroup/systemd, AppArmor/SELinux, package
restriction, portal UI, service-event, or rollback execution proof.

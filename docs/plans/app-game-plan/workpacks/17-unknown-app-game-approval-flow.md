# 17 Unknown App/Game Approval Flow

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `17 Unknown App/Game Approval Flow`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Target State

New, unknown, portable, installer, launcher-game candidate, and game-like
executables can request parent approval or remain report-only/manual-required
where proof is missing.

## Scope

- Approval request contracts.
- Evidence refs and child reason/status refs.
- Parent responses: allow once, allow this app/game, allow category, ask child
  why, block if supported, report only.
- Expiry, denial, override, and audit states.

## Tests And Proof

- [ ] New inventory app creates candidate.
- [ ] Unknown process/new app approval request carries evidence and child
      status refs.
- [ ] Unknown game-like executable stays unknown/possible game through safe
      observe-only/manual-required fallback.
- [ ] Parent approval includes evidence refs.
- [ ] Block remains manual-required without adapter proof.
- [ ] Approval survives restart where storage exists through replayable/replayed
      persistence state plus audit refs.

## Current Status - Phase 1 Active

The 2026-08-15 code audit found that the historical TypeScript contract owner
named below was removed. Current Rust protocol code retains string-shaped
approval request/decision DTOs and serialization tests, but it does not yet
produce unknown candidates or own a durable approval lifecycle with expiry,
idempotent replay, and fail-closed transition tests.

This workpack is therefore active for a bounded Rust-owned implementation in
`ocentra-app-game-core`. The slice must add typed candidate production and a
durable journal/replay state machine covering allow, deny, ask-child,
report-only, unsupported-block/manual-required, expiry, override, stale or
mismatched evidence, and duplicate/replayed transitions. It does not claim a
service command, portal or child UI, notification delivery, or platform adapter
execution.

## Historical Contract Slice - 2026-06-03

- Owner/lane: codex-c.
- Branch: `codex/app-game-read-model-service-events`.
- Proof: `output/app-game-plan-proof/17-unknown-app-game-approval-flow`.
- Source:
  - `packages/parent-domain/src/app-game-control-approval-flow.ts`
  - `packages/parent-domain/src/app-game-control-authority.ts`
  - `packages/parent-domain/src/app-game-control-authority-rules.ts`
  - `packages/parent-domain/tests/app-game-unknown-approval-flow.test.ts`
- Product-doc decision: `docs/features/app-game-control.md` gap/current-state
  text updated; `docs/product-capability-checklist.md` unchanged because this is
  contract proof only and live candidate production, parent/child approval UI,
  service persistence/read models, notifications, and platform adapter execution
  remain incomplete.

## Done Signal

Unknown apps and games can be handled without auto-promoting weak evidence or
pretending unsupported blocks exist.

Use the standard checklist in [workpacks README](README.md).

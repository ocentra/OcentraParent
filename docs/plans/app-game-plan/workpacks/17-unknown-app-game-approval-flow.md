# 17 Unknown App/Game Approval Flow

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

- [x] New inventory app creates candidate.
- [x] Unknown process/new app approval request carries evidence and child
      status refs.
- [x] Unknown game-like executable stays unknown/possible game through safe
      observe-only/manual-required fallback.
- [x] Parent approval includes evidence refs.
- [x] Block remains manual-required without adapter proof.
- [x] Approval survives restart where storage exists through replayable/replayed
      persistence state plus audit refs.

## Completed Contract Slice - 2026-06-03

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

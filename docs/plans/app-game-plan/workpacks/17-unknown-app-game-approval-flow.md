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

- [x] New inventory app creates candidate.
- [x] Unknown process/new app approval request carries evidence and child
      status refs.
- [x] Unknown game-like executable stays unknown/possible game through safe
      observe-only/manual-required fallback.
- [x] Parent approval includes evidence refs.
- [x] Block remains manual-required without adapter proof.
- [x] Approval survives restart where storage exists through replayable/replayed
      persistence state plus audit refs.

## Current Status - Phase 1 and Phase 2 Complete; Phase 3 Open

The Rust-owned lifecycle landed at `fd536480b`. It produces typed unknown app,
process, portable, installer, launcher-game, and game-like candidates; validates
evidence, device, local-user, category, child-status, actor, audit, expiry, and
override refs; and persists request, parent-response, and expiry transitions in
the synchronized Eventing NDJSON journal.

Focused validation is green:

- `cargo test -p ocentra-app-game-core --test contract` (`63 passed`)
- `cargo clippy -p ocentra-app-game-core --all-targets -- -D warnings`
- focused Enforcer `architecture-policy`, `source-shape`, `required-tests`,
  `no-test-doubles`, `no-naked-domain-strings`, `validation-bypass`, and
  `reexports`

The six WP17 tests cover candidate evidence and weak-game classification,
restart/replay and exact idempotency, conflicting transition rejection,
unsupported-block/manual-required behavior, expiry and late response rejection,
ask-child follow-up refs, category refs, and override refs. Adapter dispatch is
always `not-dispatched` in this owner.

Phase 3 retained proof and plan-level precommit/CI remain open. A service
composition owner, portal/child UI, notification delivery, and platform adapter
execution remain outside this bounded workpack slice.

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

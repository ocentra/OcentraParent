# 12 App And Game Category/Risk Taxonomy

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `12 App And Game Category/Risk Taxonomy`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Target State

Native app categories, native game categories, and risk candidates are
source/confidence-bearing policy inputs, not hidden decisions.

## Scope

- Native app categories: school, productivity, social, video, music, AI, VPN,
  remote desktop, torrent/download, developer, system, unknown, and related.
- Native game categories: educational, casual, puzzle, strategy, shooter,
  horror, casino-like, sports, racing, simulation, sandbox, RPG, MMO, launcher
  only, unknown game candidate, and related.
- Game context signals: rating, age label, multiplayer, UGC, chat/voice,
  purchase capability.

## Tests And Proof

- Known catalog category includes source/confidence.
- Unknown name-like category remains candidate.
- Risk label does not become enforcement decision.
- Parent label can override display only.

## Done Signal

Policy and UI can use category/risk rows without treating them as source truth or
adapter authority.

Use the standard checklist in [workpacks README](README.md).

## Completion Note - 2026-06-03

- Owner/lane: codex-c.
- Branch: `codex/app-game-category-risk-taxonomy`.
- Proof pack:
  `output/app-game-plan-proof/12-app-game-category-and-risk-taxonomy/`.
- Contracts:
  `packages/activity-domain/src/app-game-category-risk-primitives.ts` and
  `packages/activity-domain/src/app-game-category-risk.ts`.
- Tests: `packages/activity-domain/tests/app-game-category-risk.test.ts`.
- Scope proved: source/confidence/evidence-backed native app categories, native
  game categories, risk candidates, game context signals, parent display
  overrides, AI digest refs, duplicate candidate rejection, and
  no-direct-enforcement guards.
- Not proved: live classifier/catalog enrichment, Rust/service/read-model
  parity, portal UI, policy compiler routing, or adapter authority.

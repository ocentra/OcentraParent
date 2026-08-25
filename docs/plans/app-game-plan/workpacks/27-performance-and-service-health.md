# 27 Performance And Service Health

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `27 Performance And Service Health`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Target State

Inventory, runtime polling, journaling, replay, policy, and portal rendering stay
bounded at realistic app/game scale.

## Scope

- Inventory scan bounds.
- Runtime polling cadence.
- Journal/SQLite write volume.
- Session replay cost.
- Policy compile cost.
- Portal row virtualization or pagination where needed.
- Health/degraded status.

## Tests And Proof

- Render 500 app/game rows without layout failure.
- Compile 1,000 app/game rules within budget.
- Replay large evidence fixture within budget.
- Degraded state appears when adapters fail or data is stale.

## Done Signal

The app/game subsystem has performance limits, health states, and proof for
large-enough parent households.

## Current reviewed topology and no-claim boundary

The canonical head contains generic app/game observation, journal, and
read-model roots only. It does not contain the performance-health source,
rules, proof helper, or expected test listed by this workpack. Those absent
`packages/parent-domain` roots remain an explicit gap; the generic activity
roots do not establish performance-health ownership or completion.

The historical completion text below is retained as history only and is not
current source/test truth.

## Historical completion update - 2026-06-03

Proof pack:

- `output/app-game-plan-proof/27-performance-and-service-health`
- Cross-recorded native app proof:
  `output/app-plan-proof/26-performance-and-service-health`

Implemented proof:

- `packages/parent-domain/src/app-game-performance-health.ts`
- `packages/parent-domain/src/app-game-performance-health-rules.ts`
- `packages/parent-domain/src/app-game-performance-health-proof.ts`
- `packages/parent-domain/tests/app-game-performance-health.test.ts`
- `scripts/test/app-game-performance-health-proof.mjs`

The proof matrix records contract-backed budget rows for inventory scan bounds,
runtime polling bounds, foreground debounce bounds, journal write volume,
session replay cost, policy compile cost, portal render bounds, and degraded
adapter health state. The generated smoke covers 1,000 inventory rows, 500
runtime rows, 500 foreground transitions, 10,000 journal records, 100,000
replay observations, 1,000 policy compile parses, and 500 existing App/Game
Sessions dashboard intent rows.

Portal proof is scoped to the existing dashboard intent helper because
`apps/portal` is locked by `codex-d`. No portal source was edited and no
browser DOM, Playwright, or screenshot render proof is claimed.

## AI Worker Checklist

- [ ] Source docs read: folder README, source index, current snapshot, app/game
      shared evidence spine, native apps product slice, native games product
      slice, platform deep dive, UI guide, test blueprint, main checklist, and
      this workpack.
- [ ] Hub lock covers the parent-domain performance health contracts, focused
      test, proof script, generated proof roots, and docs changed by this
      workpack.
- [ ] Existing app/game source layout inspected; the proof extends
      `parent-domain` instead of creating a parallel performance truth.
- [ ] Before-state source snapshot recorded in the proof pack.
- [ ] Contracts were added before any proof or doc completion claim.
- [ ] Rust/service/portal parity is explicitly not changed by this row; portal
      coverage is limited to generated intent smoke against existing source.
- [ ] Raw proof artifacts record generated inventory, runtime, foreground,
      journal, replay, policy compile, portal intent, and degraded health rows.
- [ ] Tests and proof listed in this workpack are implemented or recorded as
      manual-required/no-claim where live platform proof is needed.
- [ ] Validation command outputs are saved in the proof pack and summarized in
      the main checklist.
- [ ] UI snapshots are not applicable because no UI source changed;
      `ui-not-applicable.md` records the reason.
- [ ] Security/no-claim proof records that live adapter execution and live
      platform throughput are not claimed.
- [ ] Manual platform proof is recorded as not applicable for this contract and
      generated-scale row; real host proof remains a gap.
- [ ] Feature, plan checklist, source-index, and snapshot doc decisions are
      recorded. Product capability checklist is unchanged because no live
      product status moved.
- [ ] Known gaps, deferred items, and no-claim boundaries are recorded before
      `DONE`.

## Manual-Required Gaps

- Live OS inventory scan throughput.
- Live process/runtime and foreground polling throughput.
- Real encrypted journal disk throughput and corruption/recovery proof.
- Browser DOM, Playwright, screenshot, and layout proof for 500 rendered rows.
- Live platform adapters, store APIs, approval UI, broad blocking, and
  cross-platform runtime support.

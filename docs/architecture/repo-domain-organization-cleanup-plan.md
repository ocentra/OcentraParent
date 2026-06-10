# Repo Domain Organization Cleanup Plan

This is the durable TODO for the repo-wide organization pass discussed during
the June 2026 merge wave. The goal is not prettier folders or micro-PR churn.
The goal is to make Ocentra Parent's domain, protocol, runtime, UI, proof, and
documentation boundaries canonical enough that tracking, LAN, network,
app/game, browser, AI, screen, social, and production-support work can scale
without duplicate truth.

## Trigger

Run this cleanup after the current PR merge wave is stable enough that lanes are
not rebasing every few minutes. Tracking is the first concrete slice because it
already exposes most failure modes:

- TypeScript domain contracts in `packages/activity-domain` and
  `packages/parent-domain`;
- protocol mirror shapes in `packages/agent-protocol-domain`;
- Rust wire/service/runtime code in `crates/agent-protocol`,
  `crates/agent-core`, and `crates/agent-service`;
- portal read models and route rendering in `apps/portal`;
- proof scripts under `scripts/test`;
- proof docs under `docs/plans/tracking-plan`.

## Non-Goals

- Do not open small cleanup PRs that only move names around.
- Do not default to writing code. Start with a repo scan, classify what exists,
  and decide what can move safely before touching implementation files.
- Do not rename files without reducing duplicate canonical truth.
- Do not change product claims or mark feature work complete from this plan.
- Do not merge tracking work until the full A/tracking branch has a meaningful
  organized implementation/proof chunk.
- Do not make AI a tracking-owned implementation detail. AI stays an external
  evidence boundary unless a separate AI plan owns the runtime change.

## Current Scan Snapshot

The first scan found repeated tracking/status/read-model/proof concerns across:

- `packages/parent-domain/src/tracking-*`;
- `packages/activity-domain/src/*tracking*`, `*location*`, `*geofence*`, and
  browser/activity read-model files;
- `packages/agent-protocol-domain/src/*tracking*`, network, LAN, and activity
  protocol files;
- `crates/agent-protocol/src`, `crates/agent-core/src`, and
  `crates/agent-service/src/activity_api`;
- `apps/portal/src/tracking-status-panel.ts`,
  `apps/portal/src/TrackingStatusRoutePanel.tsx`, live activity state, and
  hosted proof helpers;
- `scripts/test/tracking-*` and
  `docs/plans/tracking-plan/workpacks/*`.

That is expected for a product that grew by proof slices. The cleanup should
now classify each duplicate-looking shape as one of:

| Classification     | Meaning                                     | Action                                                                   |
| ------------------ | ------------------------------------------- | ------------------------------------------------------------------------ |
| canonical contract | the single source of product/protocol truth | move or keep in the owning domain package/crate and export it            |
| read-model mirror  | UI-facing projection of canonical data      | name it as a projection and test it against canonical contracts          |
| runtime adapter    | platform/service implementation detail      | move reusable logic into `agent-core`, keep transport in `agent-service` |
| proof fixture      | generated or static evidence input          | keep under proof scripts/output, but parse with canonical contracts      |
| stale duplicate    | same shape, same purpose, different name    | remove after proving consumers use the canonical source                  |

## Target Boundaries

| Layer                                   | Owns                                                                                                    | Must Not Own                                             |
| --------------------------------------- | ------------------------------------------------------------------------------------------------------- | -------------------------------------------------------- |
| `@ocentra-parent/schema-domain`         | Effect Schema helpers and brand/decode helpers                                                          | feature-specific product meaning                         |
| `@ocentra-parent/activity-domain`       | child activity evidence, tracking/location/geofence observations, activity read models                  | parent policy authoring, portal DOM, Rust transport      |
| `@ocentra-parent/parent-domain`         | parent-authored policy, family/device product contracts, notification/action intent, product proof rows | local service transport, portal DOM, raw OS adapter code |
| `@ocentra-parent/agent-protocol-domain` | WebSocket command/event envelopes shared by portal and Rust                                             | UI projection shapes that are not protocol               |
| `@ocentra-parent/portal-domain`         | route ids, panel ids, DOM constants, dev command descriptors                                            | product contracts, protocol envelopes, runtime strings   |
| `crates/agent-protocol`                 | Rust wire constants and serde protocol structs                                                          | service storage/runtime behavior                         |
| `crates/agent-core`                     | reusable domain/runtime logic, evaluators, compilers, read-model transformations                        | WebSocket transport, HTTP handlers, portal UI            |
| `crates/agent-service`                  | transport boundary, ActivityStore integration, service dispatch, platform adapter wiring                | duplicate protocol constants or domain rules             |
| `apps/portal`                           | UI consumption, rendering, local route state, screenshots                                               | canonical product/protocol truth                         |
| `scripts/test`                          | proof runners and generated artifacts                                                                   | new product truth not backed by contracts                |

## Canonicalization Rules

1. If two runtimes agree on a value, centralize it before reuse.
2. Event ids, route ids, command names, policy ids, status names, schema
   fields, proof row kinds, and protocol shapes cannot live as app-local
   strings.
3. TypeScript runtime validation uses Effect Schema brands and decode helpers.
   No raw `string` annotations for domain values where a branded type exists.
4. Rust-facing protocol shapes live in `crates/agent-protocol` and are mirrored
   from explicit TypeScript contracts, not invented in service code.
5. Reusable Rust logic goes into `agent-core`. Service code only owns the
   transport/service boundary.
6. Portal and child app surfaces consume domain/protocol contracts. They do not
   define canonical tracking, LAN, network, browser, app/game, screen, or AI
   schema truth.
7. Tests validate canonical contracts and real boundaries. They must not keep
   lookalike fixture shapes that drift from the exported contract.
8. Proof scripts may assemble evidence, but they must parse and emit through
   canonical contracts or explicitly mark generated-only proof shape.

## Tracking First Slice

A owns the first organized tracking proof on
`codex/tracking-plan-full-continuation-a`. The branch should not ask for PR or
main merge until the tracking organization slice is meaningful and validated.

The first tracking pass is an architecture and movement plan before it is a code
edit. A should read this file, scan the repo, and produce a movement map that
says what stays, what moves, what is deleted as duplicate truth, what becomes a
projection, and which docs/checklists will be updated. Only then should A move
implementation, tests, or proof files.

Tracking cleanup order:

1. Inventory current tracking shapes:
   - `rg -n "tracking|Tracking|Geofence|Location|ReadModel|agent.activity" packages crates apps/portal scripts/test docs/plans/tracking-plan`
   - classify every duplicate-looking type, status, event, command, route, and
     proof row using the table above.
2. Produce a movement map before coding:
   - source path;
   - proposed target path;
   - owner package/crate/plan;
   - reason for the move;
   - consumers that must be updated;
   - validation command that proves the move;
   - docs/checklists that must change.
3. Pick canonical TypeScript owners:
   - activity evidence and observations in `packages/activity-domain`;
   - parent policy/action/proof-readiness contracts in `packages/parent-domain`;
   - command/event envelopes in `packages/agent-protocol-domain`;
   - route/panel ids in `packages/portal-domain`;
   - display text tokens in `packages/text-domain`.
4. Move reusable Rust logic:
   - tracking read-model transformations, policy evaluation helpers, and
     domain/runtime logic to `crates/agent-core`;
   - command/event constants and serde structs to `crates/agent-protocol`;
   - WebSocket/ActivityStore/service dispatch to `crates/agent-service`.
5. Move tests to the owner that proves the boundary:
   - package contract tests under the owning package's `tests/`;
   - Rust public API tests under the owning crate's `tests/` where the public
     API supports it;
   - service integration tests under `crates/agent-service/tests/` or the
     existing service test module only when they prove transport/storage;
   - portal rendering tests under `apps/portal/tests/` only when they prove UI
     projection from canonical contracts;
   - no central test file should keep feature-specific truth if the feature has
     an owning package/crate.
6. Move proof assets to the owner that generates or consumes them:
   - feature proof docs stay under the owning plan folder, for tracking:
     `docs/plans/tracking-plan/`;
   - generated proof output stays under the feature proof root, for tracking:
     `output/tracking-plan-proof/<workpack-id>/`;
   - package-owned proof fixtures should live near the package/crate test
     boundary when they are static inputs;
   - proof scripts should either move beside the owning package/crate test
     harness or stay in `scripts/test` only when they orchestrate multiple
     packages/crates;
   - every move must update proof paths in the implementation checklist,
     workpack docs, feature docs, and test commands.
7. Collapse portal-local truth:
   - `apps/portal` can keep view models, render helpers, and DOM assembly;
   - anything named like a canonical status, command, route, or proof field
     must come from a domain/protocol package.
8. Make tests prove the boundary:
   - TypeScript parser/brand tests for canonical contracts;
   - Rust crate tests under crate-level `tests/` where public APIs support it;
   - service-boundary tests for real transport/storage;
   - portal tests for rendering projections from canonical contracts.
9. Update tracking docs:
   - this plan;
   - `docs/plans/tracking-plan/README.md`;
   - `docs/plans/tracking-plan/implementation-checklist.md`;
   - any moved workpack docs or proof path references under
     `docs/plans/tracking-plan/workpacks/`;
   - owning feature/checklist docs only when status, proof, or gaps actually
     move.

## Cross-Feature Rollout

After tracking proves the pattern, repeat the same cleanup for:

1. LAN pairing and multi-device control;
2. network/domain control;
3. app/game control;
4. browser/web control;
5. local AI safety evaluator and parent assistant actions;
6. screen evidence analysis;
7. social/video control;
8. production distribution/support.

Each rollout must identify the canonical domain package, Rust crate boundary,
portal projection boundary, proof runner, and docs owner before editing code.

When a rollout moves files, it must update the existing plan folder for that
feature. For example, tracking updates `docs/plans/tracking-plan`, browser
updates `docs/plans/browser-plan`, app/game updates
`docs/plans/app-game-plan`, screen updates `docs/plans/screen-plan`, and so on.
The move is not complete if old plan docs still point at obsolete proof paths,
test paths, or ownership boundaries.

## Coordination TODO

This cleanup depends on durable coordination being portable without turning hot mailbox state into product-repo churn:

- lane state remains in `.hub/lane-ledger.json`,
  `docs/hub/lane-ledger.md`, and the external OcentraHub event ledger;
- future docs/hub-only updates should use the fast required-check path added by
  PR #551 once it lands;
- no lane should keep important cleanup state only in machine-local Codex memory;
- live mailbox, heartbeat, report, ack, and ownership traffic must move through
  OcentraHub, with the legacy external hub root used only as a migration bridge;
- no branch should be considered safe to delete until the hub ledger and remote
  branch/PR state prove it.

## Done Criteria

The organization pass is not done until:

- the branch contains a movement map reviewed against this plan before broad
  code movement;
- a repo scan lists canonical owners for tracking event ids, command names,
  route ids, status names, schema fields, policy ids, and proof row kinds;
- duplicate tracking shapes are either removed, renamed as projections, or
  explicitly documented as generated proof fixtures;
- tests and proof inputs are moved under the owning feature/package/crate where
  that ownership is clear;
- generated proof output and proof docs stay under the owning plan/proof roots,
  with all path references updated;
- existing plan folders and workpack docs are updated when files/proofs/tests
  move;
- TypeScript and Rust boundaries both consume the canonical contracts;
- portal and proof scripts consume canonical contracts instead of local
  lookalikes;
- docs record what moved, what stayed, what remains duplicated, and why;
- A validates the branch and reports the exact proof/test commands before PR
  readiness.

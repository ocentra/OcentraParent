<!-- agent-capsule -->

> Agent Capsule
> Doc: Repo Domain Organization Cleanup Plan
> Kind: architecture/reference documentation; read only when selected by plan route, source router, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# Repo Domain Organization Cleanup Plan

This is the durable TODO for the repo-wide organization pass discussed during
the June 2026 merge wave. The goal is not prettier folders or micro-PR churn.
The goal is to make Ocentra Parent's domain, protocol, runtime, UI, proof, and
documentation boundaries canonical enough that tracking, LAN, network,
app/game, browser, AI, screen, social, and production-support work can scale
without duplicate truth.

## Current Authority

This cleanup plan predates the Rust-first parent architecture decision. Use
`docs/agent/RUST_FIRST_PARENT_ARCHITECTURE.md` as the current authority when
this file or older plan docs say TypeScript, `schema-domain`,
`agent-protocol-domain`, WebSocket, or Vite owns product truth.

Interpret old TypeScript cleanup language as migration/debt inventory only.
Current target ownership is Rust-first: `crates/schema` and the owning Rust
domain/runtime crates own contracts, route snapshots, actions, read models,
projections, and business logic. TypeScript may keep presentation helpers,
generated bridge DTO consumers, thin host/dev adapters, and temporary edge
decoders until Rust replacements are live and consumed.

## Trigger

Run this cleanup after the current PR merge wave is stable enough that lanes are
not rebasing every few minutes. Tracking is the first concrete slice because it
already exposes most failure modes:

- scattered TypeScript contract ownership across `packages/activity-domain`,
  `packages/parent-domain`, and `packages/agent-protocol-domain`;
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
| canonical contract | the single source of product/protocol truth | move it to `crates/schema` or the owning Rust domain/runtime crate; keep TS mirrors only as generated output, temporary edge decoders, or migration debt |
| read-model mirror  | UI-facing projection of canonical data      | name it as a projection and test it against canonical contracts          |
| runtime adapter    | platform/service implementation detail      | move reusable logic into `agent-core`, keep transport in `agent-service` |
| proof fixture      | generated or static evidence input          | keep under proof scripts/output, but parse with canonical contracts      |
| stale duplicate    | same shape, same purpose, different name    | remove after proving consumers use the canonical source                  |

## Target Boundaries

| Layer                                   | Owns                                                                                                    | Must Not Own                                             |
| --------------------------------------- | ------------------------------------------------------------------------------------------------------- | -------------------------------------------------------- |
| `crates/schema`                         | canonical cross-boundary Rust DTOs, route snapshots, action/result shapes, generated TypeScript bridge artifacts, and encoded-shape tests | domain behavior that belongs in a narrower Rust crate    |
| Rust domain/runtime crates              | product logic, projections, read models, policy decisions, state machines, and domain-local schemas     | duplicate cross-boundary DTO truth                       |
| `crates/parent-runtime-core`            | parent app facade, UI action dispatch into Rust, and route snapshot assembly                            | TS presentation rendering                                |
| `crates/agent-protocol`                 | Rust parent/child or service wire constants and structs where transport-specific                        | TS-first contract authority                              |
| `crates/agent-core`                     | reusable domain/runtime logic, evaluators, compilers, read-model transformations                        | WebSocket transport, HTTP handlers, portal UI            |
| `crates/agent-service`                  | transport boundary, ActivityStore integration, service dispatch, platform adapter wiring                | duplicate protocol constants or domain rules             |
| `apps/portal`                           | TSX/CSS/assets, generated bridge DTO consumption, thin host/dev adapters, local visual state            | canonical product/protocol truth                         |
| `@ocentra-parent/schema-domain`         | temporary edge decoders or generated validation adapters during migration                               | product schema authority or business logic               |
| `@ocentra-parent/activity-domain`       | transitional presentation/projection helpers only where not product truth                               | shared schema authority, runtime behavior, Rust transport |
| `@ocentra-parent/parent-domain`         | transitional helpers until Rust replacement owns parent/family/device behavior                          | product policy authority or route snapshots              |
| `@ocentra-parent/agent-protocol-domain` | transitional dev/protocol adapters until Rust/generated bridge replaces consumers                       | shared schema authority or product protocol truth        |
| `@ocentra-parent/portal-domain`         | pure presentation helpers, DOM ids, and dev descriptors only                                            | product contracts, protocol envelopes, runtime strings   |
| `scripts/test`                          | proof runners and generated artifacts                                                                   | new product truth not backed by contracts                |

## Canonicalization Rules

1. Every shared product schema, route snapshot, action/result shape, read-model
   DTO, branded value, and exported contract starts in `crates/schema` or the
   owning Rust domain/runtime crate.
2. Event ids, route ids, command names, policy ids, status names, schema
   fields, proof row kinds, and protocol shapes cannot live as app-local
   strings or peer-owned TS schema lookalikes.
3. TypeScript runtime validation is allowed only at untrusted TS edges or
   generated validation edges. It must not become business truth.
4. TypeScript domain packages may keep temporary edge decoders, presentation
   helpers, or adapters during migration, but shared product schemas must not be
   copied, exported, or re-owned there.
5. Rust-facing protocol shapes live in `crates/schema`, the owning Rust domain
   crate, or `crates/agent-protocol` for transport-specific wire concerns.
6. Generated TypeScript and any temporary TS mirror must preserve Rust encoded
   field names, discriminants, nullability, enum values, and version semantics.
   Drift coverage is required through Rust serialization, generated artifact,
   fixture, or equivalent parity tests.
7. Reusable Rust logic goes into `agent-core`. Service code only owns the
   transport/service boundary.
8. Portal and child app surfaces consume canonical contracts. They do not
   define canonical tracking, LAN, network, browser, app/game, screen, or AI
   schema truth.
9. Tests validate canonical contracts and real boundaries. They must not keep
   lookalike fixture shapes that drift from the exported contract.
10. Proof scripts may assemble evidence, but they must parse and emit through
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
3. Pick Rust canonical owners first:
   - cross-boundary DTOs, route snapshots, actions, and read models in
     `crates/schema`;
   - tracking, location, geofence, expected-place, and nearby-place behavior in
     the owning Rust tracking/runtime crate;
   - parent policy/action/proof-readiness behavior in the owning Rust
     policy/parent runtime crate;
   - command/event envelope transport shapes in the Rust protocol/runtime
     boundary when they are not schema-wide DTOs.
   TypeScript packages then consume generated DTOs, temporary edge decoders, or
   pure presentation helpers without becoming canonical owners.
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

## Repo-Owned Coordination TODO

This cleanup depends on coordination staying repo-owned:

- lane state is externalized through Ocentra Ledger under `LEDGER_ROOT`; the
  product repo tracks only the `tools/ocentra-ledger` submodule pointer and
  Ledger wrapper scripts;
- future docs/hub-only updates should use the fast required-check path added by
  PR #551 once it lands;
- no lane should keep important cleanup state only in `.codex`;
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

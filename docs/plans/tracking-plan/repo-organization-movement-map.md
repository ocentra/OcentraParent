# Repo Organization Movement Map

> **Historical checkpoint:** the branch heads and movement state below are not
> current repository truth. The 2026-08-15 code-first reconciliation is in
> `source-index.md`, `current-tracking-snapshot.md`, `CODE_AUDIT.md`, and the
> reviewed Tracking entries in `docs/engineering-graph/code-map.json`.

This map is the required pre-code inventory for the repo organization pass.
It covers whole-repo feature/domain/crate/test ownership and then identifies the
tracking-first implementation slice. Tracking is the first slice, not the final
scope. This is not a completion claim.

## Branch State

- Branch: `codex/tracking-plan-full-continuation-a`.
- Latest checked main: `origin/main` at `f93fe8d1d`.
- Previous pushed branch head before this checkpoint: `945109bbd`.
- Sync state: `origin/main` is an ancestor of this branch; no rebase is needed
  before the first organization chunk.
- Current organization checkpoint:
  - `docs/plans/tracking-plan/README.md`;
  - `docs/plans/tracking-plan/repo-organization-goal.md`;
  - `docs/plans/tracking-plan/repo-organization-movement-map.md`.
  - Rust tracking read-model payload projection is moving from
    `crates/agent-service` to `crates/agent-protocol`.
  - Rust tracking retention default write request construction is moving from
    service websocket test/support code to `crates/agent-protocol`.
  - `packages/parent-domain/tests/tracking*.ts` is moving into
    `packages/parent-domain/tests/tracking/`.
  - Selected tracking proof scripts are being rewired to consume canonical
    `agent-protocol-domain` and `portal-domain` command, event, payload, and
    route exports instead of repeating those identities locally.
  - Tracking evidence-drawer and retention-settings hosted UI proof intent is
    moving from `apps/portal` into `packages/portal-domain`; portal app code
    remains the DOM/React renderer.
  - App/game and social portal panel intent re-export shims in `apps/portal/src`
    are being removed so route panels and tests import canonical intent models
    directly from `packages/portal-domain`.
  - Safe portal unit tests are being grouped by feature folder
    (`logging`, `local-ai`, `diagnostics`, `activity`, `portal`, `screen`)
    without changing test behavior.
  - Screen summary portal intent now consumes the canonical
    `ActivityScreenReadModelSchema` from `packages/activity-domain`; the local
    portal-domain lookalike read-model parser/type copy was removed.
  - Browser intervention, network flow, and policy preview protocol payload
    adapters are moving from `apps/portal/src` to
    `packages/agent-protocol-domain/src`; portal now consumes those adapters
    from the protocol-domain package.

## Canonical Ownership

These are repo-wide ownership rules, not tracking-only rules:

| Layer                                           | Canonical owner                                              | Consumer rule                                                                                                       |
| ----------------------------------------------- | ------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------- |
| Effect Schema helpers, brands, decode helpers   | `packages/schema-domain`                                     | All TypeScript domain packages import shared brand/decode helpers.                                                  |
| API paths, endpoint ids, headers, query brands  | `packages/endpoint-domain`                                   | Apps/services import endpoint truth; no app-local endpoint strings.                                                 |
| WebSocket command/event envelopes               | `packages/agent-protocol-domain` and `crates/agent-protocol` | Portal, scripts, service, and Rust tests consume these contracts.                                                   |
| Portal routes, DOM ids, dev command descriptors | `packages/portal-domain`                                     | Portal source and E2E tests consume route/selector constants.                                                       |
| Display text tokens                             | `packages/text-domain`                                       | UI consumes text tokens; UI source does not invent runtime text truth.                                              |
| Operational log contracts                       | `packages/logging-domain` and `crates/agent-protocol`        | Service/core/app code emit shared log shapes.                                                                       |
| Parent/family/device product contracts          | `packages/parent-domain`                                     | Portal/child/proofs consume product contracts; no UI-local policy truth.                                            |
| Activity/evidence/read-model contracts          | `packages/activity-domain`                                   | Tracking, browser, app/game, network, screen, and reports consume shared evidence contracts where possible.         |
| Rust wire/protocol structs/constants            | `crates/agent-protocol`                                      | `agent-service` and `agent-core` consume, not duplicate.                                                            |
| Rust reusable runtime/domain logic              | `crates/agent-core`                                          | Service uses core; core does not import service.                                                                    |
| Rust transport/service orchestration            | `crates/agent-service`                                       | Service parses/dispatches and talks to storage/platform adapters; it does not own canonical protocol/domain shapes. |
| Shared eventing spine                           | `crates/ocentra-eventing`                                    | Feature runtimes consume shared eventing instead of private buses.                                                  |
| Shared network evidence spine                   | `crates/ocentra-network-evidence`                            | Network/browser/app/game evidence should reuse where applicable.                                                    |

## Repo-Wide Feature Map

Each feature needs an owned home across docs, TypeScript domain contracts, Rust
modules/crates, UI projections, tests, and proof orchestration. Existing homes
should be reused unless they create duplicate truth.

| Feature/plan                          | TS domain owner                                                                                                  | Rust owner                                                                                    | UI owner                                    | Tests/proofs owner                           | Organization rule                                                                                               |
| ------------------------------------- | ---------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------- | ------------------------------------------- | -------------------------------------------- | --------------------------------------------------------------------------------------------------------------- |
| Family setup and device roles         | `packages/parent-domain`, `packages/portal-domain`                                                               | `crates/agent-protocol`, `crates/agent-core` when service-backed                              | `apps/portal` route projections             | package tests; portal tests; plan docs       | Shared household/profile/device ids live in domain packages, not route code.                                    |
| Child agent local service             | `packages/agent-protocol-domain`, `packages/activity-domain`                                                     | `crates/agent-protocol`, `crates/agent-core`, `crates/agent-service`                          | portal service status projections           | Rust crate tests; real service smoke         | Service owns transport only; protocol/core own reusable truth.                                                  |
| Evidence store and query              | `packages/activity-domain`                                                                                       | `crates/agent-core` ActivityStore/read-model code; protocol structs in `agent-protocol`       | portal evidence projections                 | Rust integration tests; package parser tests | Query/read-model schemas must not be redefined per feature.                                                     |
| Browser and web control               | `packages/parent-domain`, `packages/activity-domain`, `packages/agent-protocol-domain`                           | `agent-core` browser logic; `agent-service` transport; protocol constants in `agent-protocol` | portal browser policy/proof surfaces        | browser plan scripts/tests                   | Browser ids/events/policy states centralize before portal/proof reuse.                                          |
| App and game control                  | `packages/parent-domain`, `packages/activity-domain`, `packages/agent-protocol-domain`                           | `agent-core` app/game domain; `agent-service` platform dispatch                               | portal app/game catalog/control projections | app-game plan scripts/tests                  | App/game catalog, action, authority, timer, notification, and receipt shapes must not fork between UI and Rust. |
| Network and domain control            | `packages/activity-domain`, `packages/agent-protocol-domain`, `packages/parent-domain`                           | `crates/ocentra-network-evidence`, `agent-core`, `agent-service`                              | portal network evidence/rule projections    | network plan tests/proofs                    | Reuse network evidence spine; do not duplicate flow/parser/risk shapes in tracking/browser.                     |
| Screen evidence analysis              | `packages/activity-domain`, `packages/parent-domain`                                                             | `crates/screen-capture-adapter`, `agent-core`, `agent-service`                                | portal screen evidence projections          | screen plan proofs/tests                     | Screen capture/AI retention/evidence contracts stay separate from tracking and AI model quality.                |
| Screen visibility and live view       | `packages/agent-protocol-domain`, `packages/parent-domain`                                                       | service/event subscription modules; protocol in `agent-protocol`                              | portal live-view projections                | screen/live-view tests                       | Live-view transport is not screen-evidence contract truth.                                                      |
| Social and video control              | `packages/parent-domain`, `packages/activity-domain`                                                             | `agent-protocol`, `agent-core`, `agent-service` as runtime grows                              | portal social/video projections             | social/video plan tests                      | Social/video event/read-model shapes must not be copied from screen/browser by name drift.                      |
| Location, geofence, and device status | `packages/activity-domain`, `packages/parent-domain`, `packages/agent-protocol-domain`, `packages/portal-domain` | `agent-protocol`, `agent-core`, `agent-service`                                               | portal tracking projections                 | tracking plan tests/proofs                   | Tracking is first implementation slice; AI stays boundary-only.                                                 |
| Policy, schedules, and approvals      | `packages/parent-domain`                                                                                         | policy compiler/evaluator in `agent-core`; protocol in `agent-protocol`                       | portal policy projections                   | policy/parent-domain tests                   | Policy authority stays parent-domain/core, not AI/portal/proofs.                                                |
| App install and purchase approval     | `packages/parent-domain`, future store/domain package if needed                                                  | platform adapters through service/core boundaries                                             | portal approval projections                 | install/purchase plan tests                  | Store/provider metadata must be canonical before UI/runtime reuse.                                              |
| Local AI safety evaluator             | AI-owned domain/package boundaries plus `packages/parent-domain` evidence inputs                                 | AI runtime/provider code outside tracking; protocol in `agent-protocol`                       | portal AI status/projection                 | AI plan tests                                | AI model/provider/prompt quality belongs to AI lane, not tracking.                                              |
| Parent assistant actions              | `packages/parent-domain`, AI/action contracts                                                                    | assistant runtime in `agent-core`/service boundary                                            | portal assistant UI                         | assistant/action tests                       | Assistant can request actions; policy/domain authority remains canonical.                                       |
| Enforcement, integrity, and tamper    | `packages/parent-domain`, `packages/agent-protocol-domain`                                                       | `agent-core` policy dispatch; service/platform adapters                                       | portal enforcement status                   | enforcement tests/proofs                     | Enforcement adapter states must be distinct from policy decisions and UI claims.                                |
| Reports, notifications, and sync      | `packages/parent-domain`, `packages/activity-domain`, `packages/agent-protocol-domain`                           | core read-model/report/notification logic; service provider boundary                          | portal report/notification projections      | reports/notification tests                   | Notification/report receipts consume canonical evidence refs; provider delivery is separate.                    |
| Remote, LAN, and mobile platforms     | `packages/portal-domain`, `packages/agent-protocol-domain`, `packages/parent-domain`                             | LAN/mobile modules in protocol/core/service; platform adapters isolated                       | portal LAN/mobile projections               | LAN/mobile tests/proofs                      | LAN pairing cannot imply location; mobile platform claims require real platform proof.                          |
| Production distribution and support   | release/support domain docs and package manifests                                                                | updater/service packaging crates                                                              | portal support/status surfaces              | release/package smoke tests                  | Installer/store/signing claims require platform-specific proof.                                                 |

## Whole-Repo DRY Workstreams

| Workstream                      | Target                                                    | First action                                                                                                              |
| ------------------------------- | --------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------- |
| Single-source contract manifest | `scripts/check-single-source-contracts.json`              | Inventory owner identities per feature before adding guard entries.                                                       |
| TS domain contract reuse        | `packages/*-domain`                                       | Replace proof/script/UI literal checks with imports from owner packages.                                                  |
| Rust protocol canonicalization  | `crates/agent-protocol`                                   | Move protocol payload projection out of service when it is pure protocol/log shape.                                       |
| Rust runtime/service split      | `crates/agent-core`, `crates/agent-service`               | Keep reusable runtime logic in core, dispatch/transport in service.                                                       |
| Test folder organization        | package/crate `tests/` plus source-adjacent private tests | Move only tests with public API boundaries; do not expose internals just to move tests.                                   |
| Proof orchestration             | `scripts/test/<feature>/` later                           | First dedupe canonical imports; move folders as a dedicated chunk after scripts are stable.                               |
| Plan docs                       | `docs/plans/<feature-plan>/`                              | Update only owning plan/docs when paths or proof ownership move.                                                          |
| Portal proof intent models      | `packages/portal-domain/src/<feature>*`                   | Keep proof data builders and detail labels in portal-domain; keep DOM/React rendering in `apps/portal`.                   |
| App/social portal intent shims  | `packages/portal-domain/src/<feature>*`                   | Delete app-local re-export mirror files after route panels/tests consume portal-domain directly.                          |
| Portal read-model projections   | `packages/activity-domain` and `packages/portal-domain`   | Portal-domain may format canonical read models, but must parse/accept owner schemas instead of cloning read-model shapes. |
| Protocol read-model adapters    | `packages/agent-protocol-domain`                          | Protocol log/event payload adapters belong beside protocol contracts; portal consumes them as UI state inputs.            |

## Tracking-First Ownership

| Concern                                                                      | Canonical owner                                                                                                                                    | Current state                                                                                       |
| ---------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------- |
| Tracking evidence, observations, geofence, read-model TS contracts           | `packages/activity-domain/src/tracking*.ts`                                                                                                        | Existing owner. Keep.                                                                               |
| Parent-authored tracking policy, family/device product contracts, proof rows | `packages/parent-domain/src/tracking*.ts`                                                                                                          | Existing owner. Keep; split only when reducing proof-family duplication.                            |
| WebSocket command/event contracts                                            | `packages/agent-protocol-domain/src/contracts.ts`, `packages/agent-protocol-domain/src/agent-message-codec.ts`                                     | Existing TypeScript owner. Portal consumes protocol-domain codecs instead of owning serialization.  |
| WebSocket defaults and payload field names                                   | `packages/agent-protocol-domain/src/defaults.ts`                                                                                                   | Existing TypeScript owner. Keep.                                                                    |
| Portal route ids, DOM ids, proof selectors, route hash                       | `packages/portal-domain/src/routes.ts`, `packages/portal-domain/src/contracts.ts`, `packages/portal-domain/src/tracking-status-proof-artifacts.ts` | Existing owner. Reuse in tests/scripts instead of repeating strings.                                |
| Portal rendering/projections                                                 | `packages/portal-domain/src/live-activity-state.ts`, `packages/portal-domain/src/*tracking*`, `apps/portal/src/*tracking*`                         | Domain package owns projections; app files consume/render only and must not define canonical truth. |
| Rust tracking protocol structs/constants                                     | `crates/agent-protocol/src/tracking*.rs`, `crates/agent-protocol/src/constants/*tracking*`, `crates/agent-protocol/src/transport.rs`               | Existing owner. Keep and expand only when Rust crosses boundary.                                    |
| Rust tracking runtime/read-model/mutation logic                              | `crates/agent-core/src/tracking/`                                                                                                                  | Existing owner. Keep.                                                                               |
| Rust tracking transport/service orchestration                                | `crates/agent-service/src/activity_api.rs`, `crates/agent-service/src/websocket/`                                                                  | Existing owner for dispatch only. Reduce protocol-shape logic here.                                 |
| Generated proof output                                                       | `output/tracking-plan-proof/**`, `test-results/tracking*/**`                                                                                       | Not product source truth. Do not commit churn as organization proof.                                |
| Proof orchestration scripts                                                  | `scripts/test/tracking*.mjs`                                                                                                                       | Cross-package orchestration. Keep initially; dedupe literals and consider folder grouping later.    |

## Empty Or Deferred Boundaries

| Path                                                                      | Decision                                                                                                                                                                                            |
| ------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `crates/tracking-core/`                                                   | Empty directory, not an active crate. Do not move code here without a deliberate crate-design decision. Current live Rust tracking ownership remains `agent-core`/`agent-protocol`/`agent-service`. |
| `crates/agent-service/src/*tracking*_tests.rs` and inline websocket tests | Keep source-adjacent for now because they exercise private service seams and test-only handlers. Moving would force public API expansion or weaken coverage.                                        |
| `scripts/test/tracking-android-physical-device-runtime-proof.mjs`         | Avoid in this pass unless physical-device proof is explicitly in scope; it uses ADB/device artifacts and writes proof output.                                                                       |
| `output/**` and `test-results/**`                                         | Do not run proof scripts just to reorganize. These roots stay generated/local unless a later validation step intentionally regenerates them.                                                        |

## First Safe Code Movement Candidates

| Priority | Source                                                                                                                                     | Proposed target                                                                                                                          | Reason                                                                                                              | Consumers to update                                       | Validation                                                                                                                             |
| -------- | ------------------------------------------------------------------------------------------------------------------------------------------ | ---------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------- |
| 1        | `crates/agent-service/src/tracking_read_model_payload.rs` and `tracking_read_model_payload_tests.rs`                                       | `crates/agent-protocol/src/tracking_read_model_payload.rs` and protocol tests/module export                                              | Payload flattening is protocol/log-field projection, not service orchestration. It already uses protocol constants. | `activity_api.rs`, protocol exports/tests.                | Done in this checkpoint. Validated with protocol/service/core tracking Rust tests.                                                     |
| 2        | `default_write_request()` inside `crates/agent-service/src/websocket/tracking_retention_settings_write.rs`                                 | `crates/agent-protocol/src/tracking_retention_settings_write_command.rs`                                                                 | It constructs protocol/default fixture shape, not transport behavior.                                               | Service websocket tests and protocol serialization tests. | Done in this checkpoint. Validated with protocol/service tracking Rust tests.                                                          |
| 3        | Literal protocol/route matrices in `scripts/test/tracking-plan-service-read-model-proof.mjs` and `tracking-plan-service-data-ui-proof.mjs` | Dynamic imports from `@ocentra-parent/agent-protocol-domain/contracts` and `@ocentra-parent/portal-domain/contracts` after package build | Scripts repeated canonical command/event/payload/route identity.                                                    | Script imports and proof assertions only.                 | In progress for this checkpoint. Run the touched proof scripts, package lint/type-check where relevant, and schema-boundary lint.      |
| 4        | Literal protocol/route matrices in `scripts/test/tracking-plan-hosted-ui-proof.mjs`                                                        | Imports from `@ocentra-parent/agent-protocol-domain` and `@ocentra-parent/portal-domain`                                                 | Hosted UI proof repeats route/protocol identities and has broader dev-server coupling.                              | Script imports and proof assertions only.                 | Defer until the hosted proof chunk because it starts local service/portal flows.                                                       |
| 5        | `apps/portal/e2e/tracking-hosted-ui-proof.spec.ts` route hash and proof selector literals                                                  | Imports or helper from `@ocentra-parent/portal-domain`                                                                                   | E2E should consume canonical route/DOM selector truth.                                                              | E2E imports and selector construction.                    | `cmd /c npm run test:e2e --workspace @ocentra-parent/portal -- tracking-hosted-ui-proof` or existing hosted proof command when needed. |
| 6        | `apps/portal/src/tracking-evidence-drawer-hosted-ui-proof.ts` and proof-model parts of `tracking-retention-settings-hosted-ui-proof.ts`    | `packages/portal-domain/src/tracking-evidence-drawer-hosted-ui-proof.ts` and `tracking-retention-settings-hosted-ui-proof.ts`            | Proof intent is domain data, not renderer behavior.                                                                 | Portal tracking panel imports and tests.                  | In progress for this checkpoint. Validate portal-domain build/tests plus portal tracking-status-panel tests.                           |

## Later Organization Candidates

| Candidate                                                                                                      | Reason to defer                                                                                                                    |
| -------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------- |
| Move `packages/parent-domain/tests/tracking*.ts` into `packages/parent-domain/tests/tracking/`                 | Done in this checkpoint: 75 tracking tests moved with mechanical import rewiring and focused parent-domain tracking test pass.     |
| Move `scripts/test/tracking*.mjs` into `scripts/test/tracking/`                                                | Requires package script/import path updates across many proof commands. Do as a dedicated chunk.                                   |
| Split `apps/portal/tests/tracking-status-panel.test.ts` and `apps/portal/e2e/tracking-hosted-ui-proof.spec.ts` | Both are large and mix fixtures/assertions/screenshots. Split when making a portal-proof chunk, not during Rust protocol movement. |
| Shared test fixture builders for tracking read models                                                          | Useful DRY step, but must avoid test-only fake truth. Prefer exported sample builders from owning domain packages.                 |
| Expand `scripts/check-single-source-contracts.json`                                                            | Needed for repo-wide DRY enforcement, but only after selected owner contracts are inventoried and stable.                          |
| Move remaining portal tracking proof model files                                                               | Do in feature-owned batches after retention/evidence drawer split is validated; several files also touch React route-panel shape.  |
| Remove app/game and social portal intent re-export shims                                                       | Done in this checkpoint: app/game and social route panels/tests now import directly from `@ocentra-parent/portal-domain`.          |
| Group safe portal unit tests by feature folder                                                                 | Done in this checkpoint for logging, local AI, diagnostics, activity, portal, and screen tests with import-depth rewiring only.    |
| Remove portal-domain screen summary read-model duplicate                                                       | Done in this checkpoint: `screen-summary-panel` now parses `ActivityScreenReadModelSchema` and tests use a canonical fixture.      |
| Move protocol read-model adapters out of portal app                                                            | Done in this checkpoint for browser intervention, network flow, and policy preview payload-to-read-model adapters.                 |

## Non-Move Decisions

- Keep `packages/activity-domain/tests/tracking*.ts` beside
  `activity-domain`; they prove canonical activity/tracking contracts.
- Keep `packages/agent-protocol-domain/tests/tracking*.ts` beside
  `agent-protocol-domain`; they prove TypeScript WebSocket contract boundaries.
- Keep `crates/agent-core/tests/tracking_*.rs`; they already prove public
  crate behavior through integration tests.
- Keep private Rust service tests source-adjacent until a real public service
  test boundary exists.
- Keep portal UI files in `apps/portal/src` as projections only; move canonical
  ids/contracts to domain packages, not into portal helpers.

## First Implementation Chunk

The first substantial organization chunk is:

1. Move Rust tracking read-model payload projection from `agent-service` to
   `agent-protocol`. Done.
2. Move protocol/default retention write request construction out of the service
   websocket handler. Done.
3. Move parent-domain tracking tests into
   `packages/parent-domain/tests/tracking/`. Done.
4. Replace obvious protocol/route literals in the tracking service read-model
   and service-data UI proof scripts with canonical imports. In progress for
   the next checkpoint; hosted UI proof remains deferred because it starts
   broader local service/portal flows.
5. Move tracking evidence-drawer proof intent and retention-settings proof model
   construction into `packages/portal-domain`, leaving app-local DOM/React
   renderers in `apps/portal`.
6. Update this movement map with what actually moved and what was deferred.
7. Run focused Rust/package validation.
8. Commit and push the meaningful chunk. Do not open a PR.

## Validation Plan

Focused validation for first chunk:

```powershell
cargo fmt --all --check
cargo test -p ocentra-parent-agent-protocol tracking
cargo test -p ocentra-parent-agent-core tracking
cargo test -p ocentra-parent-agent-service tracking
cargo check -p ocentra-parent-agent-service -p ocentra-parent-agent-protocol
cmd /c npm run lint:schema-boundaries
$env:OCENTRA_PARENT_DOMAIN_TEST_SKIP_PROOF_CHAIN='1'; cmd /c npm run test --workspace @ocentra-parent/parent-domain -- tracking; Remove-Item Env:OCENTRA_PARENT_DOMAIN_TEST_SKIP_PROOF_CHAIN
```

Before handoff or PR-ready-style claim:

```powershell
cmd /c npm run hub:guard
cmd /c npm run codeql:local:changed -- --reuse-db
cmd /c npm run validate
```

## Known Gaps After This Map

- Tracking is still not fully organized until the selected movement chunks are
  executed and validated.
- Product-ready tracking remains false.
- Android/iOS/macOS/Linux physical/runtime claims remain manual/proof-tier
  gated.
- AI remains a tracking boundary dependency only.
- Single-source contract guard expansion remains a repo-wide TODO after the
  first selected identities are stabilized.
- The selected tracking proof scripts still need focused validation after the
  current canonical-import rewiring.
- Remaining tracking portal proof-model files still live in `apps/portal/src`
  and should move in feature-owned chunks after this evidence/retention split
  is validated.

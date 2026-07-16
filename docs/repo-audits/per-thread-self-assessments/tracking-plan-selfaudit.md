# tracking-plan

## Normalized Header

- plan/thread name: `tracking-plan`
- source thread label: `tracking-plan`
- source thread id: `019ed32e-ee64-7131-92ef-e11c7a039e70`
- worktree: `C:\Users\sujan\.codex\worktrees\ocentra-parent-codex-a\OcentraParent`
- branch: `codex/tracking-plan-full-continuation-a`
- claimed status: audit/proposal complete; not done; `WP33` false-green; `WP34-WP39` open; current closure proof red
- claimed source files/crates/packages: `packages/tracking-domain`, `packages/agent-protocol-domain`, `packages/parent-domain` shadow wrappers, `crates/agent-protocol`, `crates/parent-runtime-core`, `crates/tracking-core`, `crates/agent-core`, `crates/agent-service`, `apps/portal`, `scripts/test/tracking-*.mjs`
- claimed tests: `packages/tracking-domain/tests/{unit,contract}`, `crates/agent-protocol/tests/contract`, `crates/parent-runtime-core/tests/unit`, `crates/agent-core/tests/unit`, `apps/portal/tests/tracking-status-panel.test.ts`, `apps/portal/e2e/tracking-hosted-ui-proof.spec.ts`, plus src-adjacent tracking tests still in `crates/agent-service/src`
- claimed proof commands/artifacts: `node scripts/test/tracking-android-system-geofence-blocker-proof.mjs` green; `node scripts/test/tracking-product-readiness-closure-proof.mjs` red; `output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/44-android-system-geofence-blocker-proof.json` present; `22/24/26/42` notification artifacts missing; closure/gap-map proofs missing
- claimed blockers: local schema crash in `packages/agent-protocol-domain/src/network-runtime-events.ts`; stale `parent-domain` tracking proof wrappers; TS architecture failure in `packages/tracking-domain/src/tracking-control-catalog-data.ts`; Rust re-export gate failure in `crates/tracking-core/src/lib.rs` and `crates/agent-protocol/src/tracking/mod.rs`; possible shared-eventing/sibling-runtime dependencies later
- claimed next actions: fix closure precondition import/schema crash; migrate remaining `WP33` notification/provider wrappers to `tracking-domain`; clear architecture debt; then execute `WP34-WP39` in ordered runtime/event/UI slices
- obvious missing evidence fields: fresh `WP33` closure proof set; fresh `WP34-WP39` proof roots; dedicated tracking `crates/agent-service/tests` integration surface; full Windows/Android/WSL proof for final claims; Apple-host-only proof remains external
- archival note: self-report only; unverified input for structural review

## Raw Self-Report

**COMPLETION_ARCHITECTURE_REPORT**

Tracking-plan is not closeable from docs or old proof alone. The real path is: recover the broken `WP33` proof chain from current owners, clear current architecture/runtime import failures, then finish `WP34-WP39` as real code across `tracking-domain`, `agent-protocol-domain`, `agent-protocol`, `parent-runtime-core`, `tracking-core`, `agent-core`, `agent-service`, and `apps/portal`, with scoped Windows/Android/WSL proof and honest Apple-host-only boundaries.

**Plan closure definition**

"Actually done" for this plan means all of this is true at once:
- `WP33` reruns cleanly from current canonical tracking owners and regenerates real closure artifacts, including `tracking-product-readiness-closure-proof` and `tracking-source-reconciliation-gap-map-proof`.
- `WP34-WP39` are implemented in owning code surfaces, not just described in docs.
- required tests are real and placed under proper category surfaces where applicable; no fake-green proof-only closure claims.
- `output/tracking-plan-proof/` and `test-results/` contain honest current artifacts, and `docs/proof/tracking-plan/slice-*.md` manifests exist with commands, failures, negatives, and manual-required notes.
- touched TS/Rust architecture gates pass.
- Windows/Android/WSL proof is collected where relevant; Apple-only proof is explicitly left as external-host work, not silently implied.

**Exact docs / source / tests / proof read**

Directly re-opened in this checkpoint:
- `docs/plans/tracking-plan/AGENTS.md`
- `docs/plans/tracking-plan/PLAN_STATE.md`
- `docs/plans/tracking-plan/WORKPACK_INDEX.md`
- `docs/plans/tracking-plan/NEXT_ACTIONS.md`
- `docs/plans/tracking-plan/TEST_PROOF_EXPECTATIONS.md`
- `docs/plans/tracking-plan/source-index.md`
- `docs/plans/tracking-plan/event-driven-runtime-test-matrix.md`
- `docs/plans/tracking-plan/workpacks/34-tracking-event-contracts-and-protocol-constants.md`
- `docs/plans/tracking-plan/workpacks/35-parent-tracking-config-command-event-flow.md`
- `docs/plans/tracking-plan/workpacks/36-tracking-detection-cascade-event-flow.md`
- `docs/plans/tracking-plan/workpacks/37-tracking-event-journal-replay-and-projection.md`
- `docs/plans/tracking-plan/workpacks/38-tracking-notification-and-escalation-event-flow.md`
- `docs/plans/tracking-plan/workpacks/39-tracking-portal-event-read-model-proof.md`

Referenced by search/row lookup in this checkpoint:
- `docs/plans/tracking-plan/current-tracking-snapshot.md`
- `docs/plans/tracking-plan/implementation-checklist.md` rows `26` and `33`

Directly re-opened code/tests:
- `scripts/test/tracking-android-system-geofence-blocker-proof.mjs`
- `scripts/test/tracking-provider-notification-proof.mjs`
- `scripts/test/tracking-notification-receipt-boundary-proof.mjs`
- `scripts/test/tracking-notification-preference-preflight-proof.mjs`
- `scripts/test/tracking-notification-parent-surface-history-proof.mjs`
- `scripts/test/tracking-notification-local-outbox-readiness-proof.mjs`
- `scripts/test/tracking-product-readiness-closure-proof.mjs`
- `packages/tracking-domain/src/tracking-notification-receipt-boundary-proof.ts`
- `packages/tracking-domain/tests/contract/tracking-notification-receipt-boundary-proof.test.ts`
- `packages/tracking-domain/tests/contract/tracking-product-readiness-closure-proof.test.ts`
- `packages/tracking-domain/tests/unit/tracking-event-contracts.test.ts`
- `packages/agent-protocol-domain/src/network-runtime-events.ts`
- `packages/agent-protocol-domain/src/tracking-retention-settings-write-command.ts`
- `crates/agent-protocol/src/constants/tracking_config_update.rs`
- `crates/parent-runtime-core/src/tracking_config_update_flow.rs`
- `crates/agent-protocol/Cargo.toml`
- `crates/parent-runtime-core/Cargo.toml`
- `crates/tracking-core/Cargo.toml`
- `crates/agent-core/Cargo.toml`
- `crates/agent-service/Cargo.toml`

Directly inspected proof/artifact surfaces:
- `output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/`
- existence of `22`, `24`, `26`, `42`, `44` under that root
- existence of `test-results/tracking-product-readiness-closure-proof/proof.json`
- existence of `test-results/tracking-notification-receipt-boundary-proof/proof.json`

**Current truth**

| State | Exact surface | Current truth |
|---|---|---|
| done | `scripts/test/tracking-android-system-geofence-blocker-proof.mjs` | already migrated off `@ocentra-parent/parent-domain`; architecture passed and `44-android-system-geofence-blocker-proof.json` now exists |
| partial | `packages/tracking-domain/src/*` and `packages/tracking-domain/tests/contract/*` for provider/notification proofs | real sources and contract tests exist for `tracking-provider-notification-proof`, `tracking-notification-receipt-boundary-proof`, `tracking-notification-preference-preflight-proof`, `tracking-notification-parent-surface-history-proof`, `tracking-notification-local-outbox-readiness-proof`, `tracking-notification-preference-status-handoff` |
| partial | `WP34-WP35` foundations | real config/event groundwork exists in `packages/agent-protocol-domain/src/tracking-retention-settings-write-command.ts`, `crates/agent-protocol/src/tracking/config_update_event.rs`, `crates/parent-runtime-core/src/tracking_config_update_flow.rs`, but workpacks are not proved complete |
| false-green | `docs/plans/tracking-plan/workpacks/33-*.md`, `WORKPACK_INDEX.md`, `proof-summary.json` | WP33 shows checked/summary state while required artifacts `22`, `24`, `26`, `42`, closure proof, and gap-map proof are missing |
| false-green | `packages/parent-domain/src/tracking-*.ts` and `scripts/test/tracking-*.mjs` still pointing there | these are migration/shadow surfaces; they look authoritative but many wrappers still build/test `@ocentra-parent/parent-domain` instead of `tracking-domain` |
| missing | `node scripts/test/tracking-product-readiness-closure-proof.mjs` output | current rerun fails before artifact accounting with an `Effect Schema` construction crash from `packages/agent-protocol-domain/src/network-runtime-events.ts`, imported by `packages/agent-protocol-domain/src/tracking-retention-settings-write-command.ts` |
| missing | `WP36-WP39` runtime/journal/portal proof | no current evidence that detection cascade, replay/projection, notification/escalation runtime, and portal event-projection UI are end-to-end complete |

**Code surface and ownership**

- Canonical TS tracking owner: `packages/tracking-domain`
- TS protocol/command mirror: `packages/agent-protocol-domain`
- TS migration/shadow surface that should stop driving proof truth: `packages/parent-domain`
- Rust protocol/constants: `crates/agent-protocol`
- Rust parent config event flow: `crates/parent-runtime-core`
- Rust tracking runtime/state/projection logic: `crates/tracking-core`
- Rust read-model and store consumers: `crates/agent-core`
- Rust service/WebSocket/API transport: `crates/agent-service`
- Portal rendering and Playwright/UI proof: `apps/portal`

**Test surface inventory**

- Healthy category layout already present:
  - `packages/tracking-domain/tests/unit`
  - `packages/tracking-domain/tests/contract`
  - `crates/agent-protocol/tests/contract`
  - `crates/parent-runtime-core/tests/unit`
  - `crates/agent-core/tests/unit`
- Portal tracking tests are split, not fully normalized:
  - `apps/portal/tests/tracking-status-panel.test.ts`
  - `apps/portal/e2e/tracking-hosted-ui-proof.spec.ts`
- Src-adjacent tracking tests still exist:
  - `crates/agent-service/src/tracking_read_model_service_tests.rs`
  - inline `mod tests` in `crates/agent-service/src/websocket/tracking_retention_settings_write.rs`
- I did not find empty tracking-only test folders in the inspected owners.
- For strict final closure, these still need work:
  - add real tracking integration tests under `crates/agent-service/tests/` instead of counting `src`-adjacent tests as the final shape
  - add tracking-specific replay/idempotency/invariant/property/security/authz/load/chaos coverage for `WP34-WP39`; the matrix requires them and the current tracking test surface does not yet show dedicated suites for those categories
  - for `WP39`, stop relying on hosted-only optics; add service-backed UI/Playwright flows tied to event/read-model state

**Proof inventory**

- Canonical proof roots:
  - `output/tracking-plan-proof/`
  - `test-results/`
- Real current proof from this lane:
  - `output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/44-android-system-geofence-blocker-proof.json`
  - `output/tracking-plan-proof/09-android-background-location-and-geofence-adapter/26-android-system-geofence-blocker-proof.json`
  - `test-results/tracking-android-system-geofence-blocker-proof/proof.json`
- Stale / false-green:
  - `output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/proof-summary.json`
  - `output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/65-claim-audit-proof.json`
  - older hosted/local inventory files in that root are not enough to close WP33 while core notification/closure artifacts are absent
- Missing right now:
  - `output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/22-notification-receipt-boundary-proof.json`
  - `24-notification-preference-preflight-proof.json`
  - `26-notification-parent-surface-history-proof.json`
  - `42-notification-local-outbox-readiness-proof.json`
  - `test-results/tracking-product-readiness-closure-proof/proof.json`
  - current rerun evidence for `WP34`, `WP35`, `WP36`, `WP37`, `WP38`, `WP39` proof roots

**Scoped validation inventory**

| Status | Command | Current result |
|---|---|---|
| pass | `cmd /c npm run lint:architecture -- --files scripts/test/tracking-android-system-geofence-blocker-proof.mjs` | green |
| pass | `node scripts/test/tracking-android-system-geofence-blocker-proof.mjs` | green; regenerated `44-android-system-geofence-blocker-proof.json` |
| fail | `node scripts/test/tracking-product-readiness-closure-proof.mjs` | fails early inside `packages/agent-protocol-domain/src/network-runtime-events.ts` with `TypeError: Cannot read properties of undefined (reading 'ast')` via `tracking-retention-settings-write-command.ts` |
| fail | `cmd /c npm run lint:architecture -- --files packages/tracking-domain/src/tracking-control-catalog-data.ts` | fails on forbidden `prettier-ignore` at lines `64` and `406` |
| fail | `cargo lint-architecture crates/tracking-core/src/lib.rs crates/agent-protocol/src/tracking/mod.rs` | fails on Rust public re-export ban in both files |
| unrun | remaining wrapper migrations | `tracking-provider-notification-proof`, `tracking-notification-receipt-boundary-proof`, `tracking-notification-preference-preflight-proof`, `tracking-notification-parent-surface-history-proof`, `tracking-notification-local-outbox-readiness-proof`, `tracking-notification-preference-status-handoff-proof` |
| unrun | focused runtime/event suites | targeted `cargo test` for `tracking_config_update_events`, `tracking_config_update_flow`, `tracking_read_model`, portal event/read-model UI, Android emulator/device proofs, WSL proofs |

**Dependency graph / blocker taxonomy**

| Bucket | Dependency | What it blocks | Exact reason |
|---|---|---|---|
| `local-now` | `packages/agent-protocol-domain/src/network-runtime-events.ts`, `packages/agent-protocol-domain/src/tracking-retention-settings-write-command.ts` | current closure-proof rerun | schema construction crash prevents `tracking-product-readiness-closure-proof` from even reaching artifact accounting |
| `local-now` | remaining WP33 wrapper scripts under `scripts/test/` | honest WP33 closure | current wrappers still build/test `@ocentra-parent/parent-domain` even though real `tracking-domain` owners exist |
| `local-now` | `packages/tracking-domain/src/tracking-control-catalog-data.ts`, `crates/tracking-core/src/lib.rs`, `crates/agent-protocol/src/tracking/mod.rs` | any honest completion claim touching runtime/event surfaces | architecture gates are red now |
| `needs-coordinator-sequencing` | `docs/plans/eventing-plan` / any active owner of `crates/ocentra-eventing` | `WP34-WP37` if shared event/journal/replay API changes are needed | tracking must consume shared eventing; it cannot fork a tracking-local bus/journal/replay layer |
| `needs-sibling-plan-contract` | notification/provider/production runtime owners for artifacts `51`, `52`, `58`, `62`, `63` if those are not built inside this lane | final product-readiness closure | real provider delivery, webhook receipt, production worker runtime, and full handoff artifacts may belong to adjacent notification/production lanes rather than tracking docs |
| `host-platform-limited` | real iOS/macOS host/device proof | only Apple-native runtime claims | Windows host can support docs/accounting, but real Apple simulator/device/runtime proof needs Apple hardware/host |

**Platform feasibility**

- Windows host now:
  - feasible and present: `node`, `cargo`, PowerShell, `adb`, `wsl`
  - locally absent: `docker`
- Android:
  - feasible here; `adb` is installed at `C:\Users\sujan\AppData\Local\Android\Sdk\platform-tools\adb.exe`
  - emulator and attached Samsung-device proof should be treated as executable work, not blockers
- Linux:
  - feasible here through `wsl.exe`
  - Docker is a local tool gap, not a host limitation
- Apple-only:
  - real iOS/macOS simulator/device/runtime proof

**No-hand-wave execution plan**

| Slice | Files / domains to touch | Validation to run | Proof to collect | Exit criteria |
|---|---|---|---|---|
| `S0 closure precondition fix` | `packages/agent-protocol-domain/src/network-runtime-events.ts`, `packages/agent-protocol-domain/src/tracking-retention-settings-write-command.ts`, possibly the exact import chain in `packages/tracking-domain/src/tracking-product-readiness-closure-proof.ts` | targeted `npm run test --workspace @ocentra-parent/tracking-domain -- tracking-product-readiness-closure-proof`; focused architecture on touched TS files | none until import/test path is stable again | closure proof test imports cleanly and the script reaches proof-artifact accounting instead of crashing |
| `S1 WP33 wrapper migration` | `scripts/test/tracking-provider-notification-proof.mjs`, `tracking-notification-receipt-boundary-proof.mjs`, `tracking-notification-preference-preflight-proof.mjs`, `tracking-notification-parent-surface-history-proof.mjs`, `tracking-notification-local-outbox-readiness-proof.mjs`, `tracking-notification-preference-status-handoff-proof.mjs` | focused architecture on those scripts; targeted `@ocentra-parent/tracking-domain` contract tests; each wrapper script rerun | regenerate `22`, `24`, `26`, `42`, `54`, provider proof, then rerun closure and gap-map | all notification/provider artifacts are regenerated from `tracking-domain`, not `parent-domain` |
| `S2 architecture debt cleanup` | `packages/tracking-domain/src/tracking-control-catalog-data.ts`, `crates/tracking-core/src/lib.rs`, `crates/agent-protocol/src/tracking/mod.rs`, direct callers/imports | `npm run lint:architecture -- --files ...`; `cargo lint-architecture ...`; affected unit/contract tests | updated proof notes under WP01/WP33 if status changes | tracking TS/Rust architecture gates are green on touched files |
| `S3 WP34-WP35 event/config completion` | `packages/tracking-domain/src/tracking-event-contracts*`, `packages/agent-protocol-domain/src/tracking-retention-settings-write-command.ts`, `crates/agent-protocol/src/tracking/config_update_event.rs`, `crates/parent-runtime-core/src/tracking_config_update_flow.rs`, `crates/agent-service/src/websocket/tracking_retention_settings_write.rs`, related tests | targeted `npm` unit/contract tests; `cargo test -p ocentra-parent-agent-protocol tracking_config_update_events`; `cargo test -p ocentra-parent-runtime-core tracking_config_update_flow` | `output/tracking-plan-proof/34-tracking-event-contracts/` and `35-parent-tracking-config-event-flow/` | event constants, schemas, config intent flow, idempotency, and portal read-model updates are real and proved |
| `S4 WP36-WP38 runtime chain` | `crates/tracking-core/src/{runtime_flow,expected_place,nearby_place,alerting,temporary_live,missing_device,...}`, `crates/agent-core/src/tracking/mod.rs`, `crates/agent-service` tracking transport/read-model seams | targeted `cargo test -p ocentra-tracking-core`; `cargo test -p ocentra-parent-agent-core tracking_read_model`; add replay/idempotency/security/property/load tests where applicable | `output/tracking-plan-proof/36-tracking-detection-cascade-event-flow/`, `37-tracking-event-journal-replay-projection/`, `38-tracking-notification-escalation-event-flow/` | evidence-to-policy-to-notification/escalation chain exists, replay is projection-only, and degraded/manual-required states are explicit |
| `S5 WP39 portal + service-backed UI` | `apps/portal/tests/tracking-status-panel.test.ts`, `apps/portal/e2e/tracking-hosted-ui-proof.spec.ts`, new portal test surfaces under major categories, service read-model endpoints, possibly move/replace src-adjacent service tests | focused portal unit/e2e/Playwright commands; service-backed scenario validation | `output/tracking-plan-proof/39-tracking-portal-event-read-model-proof/` plus screenshots/accessibility artifacts | portal renders service/event projection only; no hosted-only proof is counted as full product runtime proof |
| `S6 final closure + manifests` | `scripts/test/tracking-plan-pre-device-proof.mjs`, `tracking-product-readiness-closure-proof.mjs`, `tracking-source-reconciliation-gap-map-proof.mjs`, `docs/proof/tracking-plan/slice-*.md`, plan rows/docs that move status | rerun closure/gap-map scripts and the minimum commands from each touched slice | fresh closure proof, gap-map proof, HID manifests, updated workpack/checklist references | plan truth is current, proof roots are honest, and remaining Apple/sibling-plan boundaries are explicit rather than implied |

**First coordinator ask**

No predecessor plan is required before the immediate next slice. I should continue locally with `S0` then `S1`.

The first sequencing-sensitive dependency is `eventing-plan` only when `WP34-WP37` need shared `crates/ocentra-eventing` changes. If another active thread already owns that crate, I need you to confirm that ownership before I widen into shared journal/replay/event-taxonomy edits.

**Strict done bar**

Before you can ever mark this plan done, all of this must be true:
- `WP33`, `WP34`, `WP35`, `WP36`, `WP37`, `WP38`, and `WP39` each have current code, current tests, current proof, and updated plan rows.
- `tracking-product-readiness-closure-proof` and `tracking-source-reconciliation-gap-map-proof` both rerun green from current owners.
- no required artifact in `output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/` is missing while `proof-summary.json` still claims readiness.
- no tracking completion claim depends on `packages/parent-domain` shadow wrappers being treated as canonical source.
- touched TS/Rust architecture gates are green.
- tracking event/runtime coverage includes the applicable contract, unit, integration, e2e/Playwright, replay/idempotency, security/authz, invariant/property, and load/chaos categories required by the matrix.
- Windows/Android/WSL proof is real where relevant; Apple-only work is explicitly externalized.
- `docs/proof/tracking-plan/slice-*.md` exists with command logs, negative cases, skipped/manual-required notes, and artifact paths.

**COORDINATOR_DECISION_REQUEST**

- recommended next slice: `S0 closure precondition fix`, immediately followed by `S1 WP33 wrapper migration`
- recommended predecessor plans: none for `S0/S1`; after that, coordinate `eventing-plan` only if shared `crates/ocentra-eventing` changes are already owned elsewhere; coordinate notification/production owners only if final provider/runtime artifacts are owned outside tracking
- estimated risk: high
- estimated proof difficulty: very high
- continue or pause: continue immediately through `local-now` slices; pause only before widening into shared eventing or sibling-owned provider/production runtime contracts

## Optional Addendum

- Earlier audit passes found that `@ocentra-parent/parent-domain` is not just a stale proof owner; it is also broken on this branch from unrelated billing/production cross-plan errors. Any remaining tracking proof wrappers that still depend on `@ocentra-parent/parent-domain` are therefore doubly non-authoritative: wrong owner and not rerunnable from current branch truth.
- Earlier closure-chain validation also established an underlying missing-artifact frontier beneath the current `agent-protocol-domain` schema crash. After `S0` fixes the import/schema failure, the next concrete closure dependency remains the missing `output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/22-notification-receipt-boundary-proof.json`, followed by the rest of the notification/provider artifact chain (`24`, `26`, `42`, `54`, provider proof, then closure/gap-map).
- Earlier audit passes also confirmed that `tracking-claim-audit-proof.mjs` is no longer the active blocker. It reruns from `tracking-domain`; the current blockers are upstream proof restoration, the schema/import crash, stale wrapper migration, and the still-open runtime/event workpacks `WP34-WP39`.

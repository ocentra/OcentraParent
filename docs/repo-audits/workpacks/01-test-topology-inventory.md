# WP01 Test Topology Inventory

## Objective

Build the repo-level test topology inventory before accepting any plan completion claim.

## Scope

Inspect:

- `crates/*/src/**/*.rs`
- `crates/*/tests/**`
- `packages/*/tests/**`
- `apps/*/tests/**`
- `apps/*/e2e/**`
- `scripts/test/**`

## Required classification

| Class | Meaning |
| --- | --- |
| real test | Executable test file with assertions and command path. |
| empty scaffold | `.gitkeep` or empty folder with no executable tests. |
| inline private seam | `#[cfg(test)]` test justified by private helper or service wiring. |
| inline move candidate | `#[cfg(test)]` test that exercises public behavior and should move to crate-level `tests/`. |
| proof script | Script that generates proof artifacts under ignored local/CI output paths. |

## Output table

Create or update an inventory table with:

| Surface | Real tests | Empty scaffolds | Inline src tests | Move candidates | Proof scripts | Notes |
| --- | ---: | ---: | ---: | ---: | ---: | --- |

## Acceptance

- Empty scaffold folders are listed explicitly.
- Inline Rust tests are classified, not just counted.
- Packages/crates with only minimal smoke tests are marked weak.
- No plan report may count a test category until this inventory confirms executable tests exist.

## Failure conditions

- Counting folder names as tests.
- Counting generated proof artifacts as tests.
- Treating `scripts/check-required-tests.mjs` as full coverage proof.

## Inventory method

- Rust real-test count: `.rs` files under `crates/*/tests/**` that contain explicit test indicators (`#[test]`, `#[tokio::test]`, `assert!`, `assert_eq!`, `assert_ne!`, `proptest!`, `rstest`, or `insta::`).
- TS/JS real-test count: `*.test.*` or `*.spec.*` files under `packages/*/tests/**`, `apps/*/tests/**`, and `apps/*/e2e/**`.
- Empty scaffold count: directories under `tests/**` or `e2e/**` whose subtree contains no real test file and no non-`.gitkeep` code file.
- Inline Rust count: `src/**/*.rs` files containing `#[cfg(test)]`.
- Proof script count: script files under `scripts/test/**`. These are proof generators, not product tests.
- The original stop condition was valid. The coordinator corrected the packet to allow read-only evidence inspection of `crates/agent-core/**`, `crates/agent-protocol/**`, and `crates/agent-service/**` for WP01 classification only.

## Inventory table

| Surface | Real tests | Empty scaffolds | Inline src tests | Move candidates | Proof scripts | Notes |
| --- | ---: | ---: | ---: | ---: | ---: | --- |
| `apps/local-api` | 2 | 0 | 0 | 0 | 0 | Minimal smoke only. |
| `apps/parent-desktop` | 0 | 0 | 0 | 0 | 0 | No executable tests under `tests/` or `e2e/`. |
| `apps/portal` | 41 | 0 | 0 | 0 | 0 | `*.ts` helpers in `e2e/` were excluded; only `*.test.*` / `*.spec.*` counted. |
| `crates/agent-core` | 1 | 57 | 17 | 7 | 0 | Weak crate-level surface; inline-heavy. |
| `crates/agent-protocol` | 15 | 56 | 4 | 3 | 0 | Protocol contracts still partly hidden in `src/`. |
| `crates/agent-service` | 1 | 63 | 37 | 14 | 0 | Weak crate-level surface; inline-heavy. |
| `crates/agent-updater` | 2 | 58 | 3 | 0 | 0 | Thin crate-level coverage; inline tests read as private seam. |
| `crates/app-core` | 3 | 60 | 0 | 0 | 0 | Shared crate scaffold taxonomy dominates. |
| `crates/app-game-core` | 3 | 60 | 0 | 0 | 0 | Shared crate scaffold taxonomy dominates. |
| `crates/billing-core` | 3 | 32 | 0 | 0 | 0 | Shared crate scaffold taxonomy dominates. |
| `crates/browser-core` | 4 | 61 | 0 | 0 | 0 | Shared crate scaffold taxonomy dominates. |
| `crates/child-ai-core` | 2 | 64 | 0 | 0 | 0 | Minimal smoke only; scaffold-heavy. |
| `crates/child-enforcement-core` | 1 | 60 | 0 | 0 | 0 | Minimal smoke only; scaffold-heavy. |
| `crates/child-notification-core` | 3 | 60 | 0 | 0 | 0 | Shared crate scaffold taxonomy dominates. |
| `crates/child-policy-core` | 5 | 59 | 0 | 0 | 0 | Shared crate scaffold taxonomy dominates. |
| `crates/child-runtime` | 6 | 56 | 0 | 0 | 0 | Real integration coverage exists, but scaffold debt remains large. |
| `crates/entitlement-core` | 2 | 59 | 0 | 0 | 0 | Minimal smoke only; scaffold-heavy. |
| `crates/family-identity-core` | 4 | 59 | 0 | 0 | 0 | Shared crate scaffold taxonomy dominates. |
| `crates/lan-core` | 2 | 57 | 0 | 0 | 0 | Minimal smoke only; scaffold-heavy. |
| `crates/logging-core` | 1 | 1 | 0 | 0 | 0 | Single executable test file plus empty fixture scaffold. |
| `crates/network-core` | 2 | 61 | 0 | 0 | 0 | Minimal smoke only; scaffold-heavy. |
| `crates/ocentra-eventing` | 22 | 0 | 0 | 0 | 0 | Only crate in this slice with no empty scaffold debt. |
| `crates/ocentra-evidence` | 1 | 58 | 0 | 0 | 0 | Minimal smoke only; scaffold-heavy. |
| `crates/ocentra-network-evidence` | 2 | 58 | 1 | 1 | 0 | Public crate root still hides one inline move candidate. |
| `crates/parent-runtime-core` | 5 | 58 | 0 | 0 | 0 | Shared crate scaffold taxonomy dominates. |
| `crates/policy-control-core` | 16 | 54 | 0 | 0 | 0 | Real coverage exists, scaffold debt remains large. |
| `crates/provisioning-core` | 2 | 58 | 0 | 0 | 0 | Minimal smoke only; scaffold-heavy. |
| `crates/remote-access-core` | 2 | 59 | 0 | 0 | 0 | Minimal smoke only; scaffold-heavy. |
| `crates/screen-ai-core` | 1 | 55 | 0 | 0 | 0 | Minimal smoke only; scaffold-heavy. |
| `crates/screen-capture-adapter` | 2 | 60 | 1 | 1 | 0 | Public crate root still hides one inline move candidate. |
| `crates/screen-core` | 3 | 60 | 0 | 0 | 0 | Shared crate scaffold taxonomy dominates. |
| `crates/screen-live-view-core` | 4 | 59 | 0 | 0 | 0 | Shared crate scaffold taxonomy dominates. |
| `crates/storage-custody-core` | 2 | 55 | 0 | 0 | 0 | Minimal smoke only; scaffold-heavy. |
| `crates/tracking-core` | 25 | 48 | 0 | 0 | 0 | Strongest crate-level test surface after `ocentra-eventing`; scaffold debt still large. |
| `packages/activity-domain` | 7 | 31 | 0 | 0 | 0 | Shared package scaffold taxonomy dominates. |
| `packages/agent-protocol-domain` | 46 | 32 | 0 | 0 | 0 | Strong package test surface, scaffold debt remains broad. |
| `packages/ai-domain` | 34 | 32 | 0 | 0 | 0 | Strong package test surface, scaffold debt remains broad. |
| `packages/app-game-domain` | 168 | 32 | 0 | 0 | 0 | Largest package test surface in this slice. |
| `packages/billing-domain` | 13 | 32 | 0 | 0 | 0 | Shared package scaffold taxonomy dominates. |
| `packages/browser-domain` | 101 | 32 | 0 | 0 | 0 | Strong package test surface, scaffold debt remains broad. |
| `packages/capability-domain` | 2 | 32 | 0 | 0 | 0 | Minimal smoke only. |
| `packages/child-runtime-domain` | 14 | 32 | 0 | 0 | 0 | Shared package scaffold taxonomy dominates. |
| `packages/data-custody-domain` | 3 | 35 | 0 | 0 | 0 | Thin real coverage with extra observability-only empty folders. |
| `packages/endpoint-domain` | 5 | 32 | 0 | 0 | 0 | Shared package scaffold taxonomy dominates. |
| `packages/enforcement-domain` | 22 | 32 | 0 | 0 | 0 | Real package coverage exists, scaffold debt remains broad. |
| `packages/event-domain` | 3 | 32 | 0 | 0 | 0 | Thin real coverage. |
| `packages/evidence-domain` | 4 | 32 | 0 | 0 | 0 | Shared package scaffold taxonomy dominates. |
| `packages/family-domain` | 8 | 32 | 0 | 0 | 0 | Shared package scaffold taxonomy dominates. |
| `packages/lan-domain` | 18 | 32 | 0 | 0 | 0 | Real package coverage exists, scaffold debt remains broad. |
| `packages/logging-domain` | 41 | 31 | 0 | 0 | 0 | Strong package test surface, scaffold debt remains broad. |
| `packages/network-domain` | 4 | 32 | 0 | 0 | 0 | Shared package scaffold taxonomy dominates. |
| `packages/notification-domain` | 6 | 32 | 0 | 0 | 0 | Shared package scaffold taxonomy dominates. |
| `packages/parent-domain` | 16 | 32 | 0 | 0 | 0 | Real package coverage exists, scaffold debt remains broad. |
| `packages/policy-domain` | 8 | 32 | 0 | 0 | 0 | Shared package scaffold taxonomy dominates. |
| `packages/portal-domain` | 26 | 32 | 0 | 0 | 0 | Real package coverage exists, scaffold debt remains broad. |
| `packages/production-domain` | 28 | 32 | 0 | 0 | 0 | Real package coverage exists, scaffold debt remains broad. |
| `packages/remote-access-domain` | 2 | 35 | 0 | 0 | 0 | Minimal smoke only with extra observability-only empty folders. |
| `packages/schema-domain` | 2 | 32 | 0 | 0 | 0 | Minimal smoke only. |
| `packages/screen-domain` | 37 | 32 | 0 | 0 | 0 | Strong package test surface, scaffold debt remains broad. |
| `packages/setup-domain` | 6 | 35 | 0 | 0 | 0 | Extra observability-only empty folders. |
| `packages/text-domain` | 7 | 32 | 0 | 0 | 0 | Shared package scaffold taxonomy dominates. |
| `packages/tracking-domain` | 80 | 31 | 0 | 0 | 0 | Strong package test surface, scaffold debt remains broad. |
| `scripts/test` | 0 | 0 | 0 | 0 | 820 | Proof-script family only; not counted as tests. |

## Empty scaffold folders

The empty-folder problem is mostly a repeated taxonomy skeleton rather than isolated missing directories. WP01 records the shared relative paths here because fully expanded per-surface paths would be hundreds of duplicates and would hide the real topology problem.

### Shared crate scaffold taxonomy

These explicitly confirmed relative directories recur empty across 28-29 crates and are therefore structural debt, not plan-specific proof:

- `tests/ai-safety`
- `tests/ai-prompt-injection-hallucination-regression-output-invariant`
- `tests/ai-temperature-safety-boundary`
- `tests/alerting-error-budget`
- `tests/api-fuzzing`
- `tests/authn`
- `tests/authz`
- `tests/canary-rollback-validation`
- `tests/chaos`
- `tests/chaos-partial-outage-slow-dependency-retry-storm`
- `tests/clock-skew`
- `tests/clock-skew-expiry-dst`
- `tests/concurrency`
- `tests/concurrency-race-idempotency-ordering`
- `tests/consumer-driven`
- `tests/differential`
- `tests/e2e`
- `tests/flaky-detection-mutation-score-ci-dependency-kill`
- `tests/header-injection-request-splitting-open-redirect`
- `tests/human-misuse`
- `tests/integration`
- `tests/invariant`
- `tests/load`
- `tests/load-spike-soak-memory-fd-connection-exhaustion`
- `tests/migration`
- `tests/migration-rollback-backward-compatibility-schema-drift`
- `tests/monitoring`
- `tests/mutation`
- `tests/observability/alerting`
- `tests/observability/logging-metrics-tracing`
- `tests/property-based`
- `tests/quality`
- `tests/quality/flaky-detection-mutation-score-ci-dependency-kill`
- `tests/release`
- `tests/release/canary-rollback`
- `tests/security`
- `tests/security/authn`
- `tests/security/authz`
- `tests/security/cors-origin-headers-host-redirect-url`
- `tests/security/fuzz`
- `tests/security/header-injection-request-splitting-open-redirect`
- `tests/security/rate-limit-abuse-dos`
- `tests/security/replay`

Crate outliers also exist and are empty exactly where named, including:

- `crates/child-notification-core/tests/alert-firing`
- `crates/agent-service/tests/api-fuzz`
- `crates/agent-service/tests/cache-poisoning`
- `crates/network-core/tests/cache-poisoning`
- `crates/agent-service/tests/cors-origin`
- `crates/parent-runtime-core/tests/cors-origin`
- `crates/agent-service/tests/desync`
- `crates/network-core/tests/desync`
- `crates/logging-core/tests/fixtures`

### Shared package scaffold taxonomy

These explicitly confirmed relative directories recur empty across 28 packages and define the dominant package-side scaffold debt:

- `tests/clock-skew`
- `tests/concurrency`
- `tests/consumer-driven`
- `tests/differential`
- `tests/e2e`
- `tests/human-misuse`
- `tests/invariant`
- `tests/load`
- `tests/migration`
- `tests/monitoring`
- `tests/mutation`
- `tests/observability`
- `tests/observability/alerting`
- `tests/observability/logging-metrics-tracing`
- `tests/property-based`
- `tests/quality`
- `tests/quality/flaky-detection-mutation-score-ci-kill`
- `tests/release`
- `tests/release/canary-rollback`
- `tests/security`
- `tests/security/authn`
- `tests/security/authz`
- `tests/security/cors-origin-headers-host-redirect-url`
- `tests/security/fuzz`
- `tests/security/header-injection-request-splitting-open-redirect`
- `tests/security/rate-limit-abuse-dos`
- `tests/security/replay`
- `tests/security/smuggling-desync-cache-poisoning`

Package outliers also exist and are empty exactly where named, including:

- `packages/data-custody-domain/tests/observability/logging`
- `packages/data-custody-domain/tests/observability/metrics`
- `packages/data-custody-domain/tests/observability/tracing`
- `packages/remote-access-domain/tests/observability/logging`
- `packages/remote-access-domain/tests/observability/metrics`
- `packages/remote-access-domain/tests/observability/tracing`
- `packages/setup-domain/tests/observability/logging`
- `packages/setup-domain/tests/observability/metrics`
- `packages/setup-domain/tests/observability/tracing`

## Inline Rust classification

WP01 classifies every inline-heavy crate the structural audit called out. Files listed below are move candidates because they expose public protocol, read-model, API, websocket, settings, or status behavior. Inline files not listed below remain inventory-classified as `inline private seam` for now because they are queueing, transport wiring, config, scheduler, persistence, snapshot, fixture, or other internal runtime support.

### Move candidates by file

`crates/agent-core`

- `crates/agent-core/src/activity_store_app_game/app_game_journal_sqlite_ingest.rs`
- `crates/agent-core/src/activity_store_app_game.rs`
- `crates/agent-core/src/activity_store_memory_graph_index_query.rs`
- `crates/agent-core/src/browser_event_runtime/action_handoff_child_status.rs`
- `crates/agent-core/src/browser_event_runtime/topology.rs`
- `crates/agent-core/src/browser_event_runtime.rs`
- `crates/agent-core/src/network_event_runtime.rs`

`crates/agent-protocol`

- `crates/agent-protocol/src/activity.rs`
- `crates/agent-protocol/src/lan_pairing/device_roles.rs`
- `crates/agent-protocol/src/lan_pairing.rs`

`crates/agent-service`

- `crates/agent-service/src/activity_api/app_game_adapter_dispatch_result_payload.rs`
- `crates/agent-service/src/activity_api/app_game_adapter_host_capabilities.rs`
- `crates/agent-service/src/activity_api.rs`
- `crates/agent-service/src/activity_surface_read_models.rs`
- `crates/agent-service/src/browser_evidence_payload.rs`
- `crates/agent-service/src/browser_runtime_status.rs`
- `crates/agent-service/src/enforcement_api.rs`
- `crates/agent-service/src/lan_network_inventory.rs`
- `crates/agent-service/src/lan_pairing.rs`
- `crates/agent-service/src/lan_pairing_status/selection.rs`
- `crates/agent-service/src/network.rs`
- `crates/agent-service/src/screen_settings_runtime.rs`
- `crates/agent-service/src/websocket/tracking_retention_settings_write.rs`
- `crates/agent-service/src/websocket.rs`

`crates/ocentra-network-evidence`

- `crates/ocentra-network-evidence/src/lib.rs`

`crates/screen-capture-adapter`

- `crates/screen-capture-adapter/src/lib.rs`

### Inline private seam buckets

- `crates/agent-core`: queueing, remote-delivery transport/custody/store helpers, and fixture-transport internals remain inline-private-seam.
- `crates/agent-protocol`: `src/lib.rs` remains an inline aggregator seam rather than a direct move candidate.
- `crates/agent-service`: `activity_capture/*`, `activity_surface_store.rs`, `browser_policy_runtime.rs`, `event_builder.rs`, `lan_pairing_runtime_state*`, `local_ai_*`, `parent_assistant_api/thread_store.rs`, `screen_ai_*`, `snapshot.rs`, and related runtime internals remain inline-private-seam.
- `crates/agent-updater`: `args.rs`, `crypto.rs`, and `hash.rs` remain inline-private-seam.

## Weak surfaces

These surfaces currently have only minimal executable coverage relative to their scaffold or runtime breadth and should not be treated as meaningfully complete by later plan reports:

- `apps/parent-desktop`: 0 real tests.
- `apps/local-api`: 2 real tests.
- `crates/agent-core`: 1 real crate-level test file, 57 empty scaffolds, 17 inline files.
- `crates/agent-service`: 1 real crate-level test file, 63 empty scaffolds, 37 inline files.
- `crates/agent-updater`: 2 real tests, 58 empty scaffolds, 3 inline files.
- `crates/child-ai-core`: 2 real tests, 64 empty scaffolds.
- `crates/child-enforcement-core`: 1 real test, 60 empty scaffolds.
- `crates/entitlement-core`: 2 real tests, 59 empty scaffolds.
- `crates/lan-core`: 2 real tests, 57 empty scaffolds.
- `crates/logging-core`: 1 real test, 1 empty scaffold.
- `crates/network-core`: 2 real tests, 61 empty scaffolds.
- `crates/ocentra-evidence`: 1 real test, 58 empty scaffolds.
- `crates/ocentra-network-evidence`: 2 real tests, 58 empty scaffolds, 1 inline file.
- `crates/provisioning-core`: 2 real tests, 58 empty scaffolds.
- `crates/remote-access-core`: 2 real tests, 59 empty scaffolds.
- `crates/screen-ai-core`: 1 real test, 55 empty scaffolds.
- `crates/screen-capture-adapter`: 2 real tests, 60 empty scaffolds, 1 inline file.
- `crates/storage-custody-core`: 2 real tests, 55 empty scaffolds.
- `packages/capability-domain`: 2 real tests, 32 empty scaffolds.
- `packages/data-custody-domain`: 3 real tests, 35 empty scaffolds.
- `packages/event-domain`: 3 real tests, 32 empty scaffolds.
- `packages/remote-access-domain`: 2 real tests, 35 empty scaffolds.
- `packages/schema-domain`: 2 real tests, 32 empty scaffolds.

## Command log / evidence inputs

- `git branch --show-current`
- `npm run hub:lock -- --paths "docs/repo-audits/workpacks/01-test-topology-inventory.md" --reason "WP01 corrected packet inventory update only"`
- `Get-Content docs/repo-audits/AGENTS.md`
- `Get-Content docs/repo-audits/INDEX.md`
- `Get-Content docs/repo-audits/WORKPACK_INDEX.md`
- `Get-Content docs/repo-audits/2026-06-17-structural-truth-audit.md`
- `Get-Content docs/repo-audits/workpacks/01-test-topology-inventory.md`
- `Get-Content docs/repo-audits/lane-manager-coordination/READ_SCOPE_BUDGET.md`
- `Get-Content docs/repo-audits/lane-manager-coordination/VALIDATION_BUDGET_LADDER.md`
- `rg -n "agent-core|agent-protocol|agent-service|inline" docs/repo-audits/2026-06-17-structural-truth-audit.md docs/repo-audits/workpacks/01-test-topology-inventory.md`
- Node inventory pass over `crates/*/src/**/*.rs`, `crates/*/tests/**`, `packages/*/tests/**`, `apps/*/tests/**`, `apps/*/e2e/**`, and `scripts/test/**` using the method above to emit per-surface counts, grouped empty scaffold paths, weak surfaces, and inline-file inventories.
- Targeted file reads for move-candidate confirmation:
  - `crates/agent-core/src/activity_store_app_game/app_game_journal_sqlite_ingest.rs`
  - `crates/agent-core/src/activity_store_app_game.rs`
  - `crates/agent-core/src/activity_store_memory_graph_index_query.rs`
  - `crates/agent-core/src/browser_event_runtime/action_handoff_child_status.rs`
  - `crates/agent-core/src/browser_event_runtime/topology.rs`
  - `crates/agent-core/src/browser_event_runtime.rs`
  - `crates/agent-core/src/network_event_runtime.rs`
  - `crates/agent-protocol/src/activity.rs`
  - `crates/agent-protocol/src/lan_pairing/device_roles.rs`
  - `crates/agent-protocol/src/lan_pairing.rs`
  - `crates/agent-service/src/activity_api.rs`
  - `crates/agent-service/src/activity_api/app_game_adapter_dispatch_result_payload.rs`
  - `crates/agent-service/src/activity_api/app_game_adapter_host_capabilities.rs`
  - `crates/agent-service/src/activity_surface_read_models.rs`
  - `crates/agent-service/src/browser_evidence_payload.rs`
  - `crates/agent-service/src/browser_runtime_status.rs`
  - `crates/agent-service/src/enforcement_api.rs`
  - `crates/agent-service/src/lan_network_inventory.rs`
  - `crates/agent-service/src/lan_pairing.rs`
  - `crates/agent-service/src/lan_pairing_status/selection.rs`
  - `crates/agent-service/src/network.rs`
  - `crates/agent-service/src/screen_settings_runtime.rs`
  - `crates/agent-service/src/websocket/tracking_retention_settings_write.rs`
  - `crates/agent-service/src/websocket.rs`
  - `crates/ocentra-network-evidence/src/lib.rs`
  - `crates/screen-capture-adapter/src/lib.rs`

# 25 Rollout Checklist And PR Gate

<!-- agent-capsule -->

> Agent Capsule
> Plan: `lan-plan`
> Doc: `25 Rollout Checklist And PR Gate`
> Kind: assigned active workpack; read only when this exact workpack is selected.
> Read when: Only when this exact workpack is explicitly selected from `WORKPACK_INDEX.md`.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack's own proof rows and tests support the claim.
> Proves: only this workpack's current rollout-gate boundary and progress explicitly recorded here.
> Does not prove: current PR readiness for sibling workpacks or broad LAN completion.
> Proof rule: Rewrite any stale TS-first gate text before using this file for execution or release claims.

<!-- /agent-capsule -->

Sources: [folder README](../README.md), [feature doc](../../../features/family-setup-device-roles.md),
[family setup expectations](../../../expectations/family-setup.md),
[LAN pairing expectations](../../../expectations/lan-pairing.md),
[PR/DONE flow](../../../agent/PR_DONE_FLOW.md).

## Active scope status

This workpack is part of the authoritative `01-25` LAN execution model. It is
active and still open.

Historical TS-first gate text from older copies of this draft is stale. Current
direction for this workpack is:

- Rust-owned shared schemas, protocol/runtime parity, and host-bridge snapshots
  remain the contract source of truth.
- TS stays pure UI/presentation only.
- UI compile or browser checks may still exist as presentation sanity only, but
  they do not create contract ownership, runtime truth, read-model truth, or
  proof closure.

## Where We Are

This workpack describes a later rollout gate that only makes sense after the
earlier household/setup follow-on workpacks are actually closed with real
proof.

## PR Gate For Active Scope

All of the following would need to be true before any broad PR-ready claim:

- earlier active household/setup workpacks explicitly closed or reduced to
  explicit manual-required/harness blockers with current proof
- Rust-owned schema, protocol, runtime, and bridge tests green
- supporting UI presentation checks green for the surfaces that actually exist
  and already consume honest Rust proof roots
- no false product claims about protection, readiness, or physical household LAN
  proof

Current verified rollout truth on 2026-06-28:

- Focused LAN runtime validations are green in this lane:
  `cargo test -p ocentra-parent-agent-service lan_pairing -- --nocapture`,
  `cargo test -p ocentra-parent-runtime-core policy_network_route_load_keeps_host_bridge_surface_and_attaches_lan_read_model -- --nocapture`,
  `cargo test -p ocentra-parent-runtime-core parent_subscription_event_keeps_lan_diagnostics_and_history_surface_intact -- --nocapture`,
  `cargo test -p ocentra-parent-runtime-core parent_subscription_event_preserves_explicit_lan_history_state_labels -- --nocapture`,
  `cargo test -p ocentra-parent-runtime-core lan_agent_command_requested_for_devices_route_forwards_signed_child_observe_payload_and_replay_fields -- --nocapture`,
  `cargo test -p ocentra-parent-runtime-core lan_scan_action_returns_bounded_error_when_response_times_out -- --nocapture`,
  `cargo test -p ocentra-parent-runtime-core product_bridge_actions_return_route_snapshots_without_invented_overlay_data -- --nocapture`,
  `$env:OCENTRA_PARENT_PORTAL_PLAYWRIGHT_SPEC='portal-ui.spec.ts'; node scripts/test/portal-playwright-runner.mjs`,
  `cargo lint-architecture crates/agent-service/tests/unit/lan_pairing.rs crates/agent-service/tests/unit/lan_pairing_household_device_spine.rs crates/parent-runtime-core/tests/unit/parent_ui_bridge/snapshot_and_dispatch_tests.rs`,
  and
  `npm run lint:architecture -- --files apps/portal/e2e/portal-ui.spec.ts apps/portal/e2e/portal-route-scaffold-assertions.ts`.
- The 2026-08-15 code-first audit invalidates the broader wrapper claim:
  `scripts/test/v0-9-lan-source-matrix-plan-completion.mjs` is absent from the
  current repository. `build:contracts` history does not substitute for that
  missing aggregate verifier.
- The current Windows portal Playwright rerun is green in this lane on
  2026-06-28:
  `$env:OCENTRA_PARENT_PORTAL_PLAYWRIGHT_SPEC='portal-ui.spec.ts'; node scripts/test/portal-playwright-runner.mjs`.
- W14 is locally complete; W18, W19, W23, and W25 are reduced to honest
  manual/open rows rather than stale checklist drift.
- Broad PR-ready/DONE still cannot be claimed while physical multi-device,
  router/firewall, signed-artifact, restart, and other manual-required LAN
  topology proof remain open.
- The accepted Windows LAN portal rerun is fresh supporting evidence from
  2026-06-28 rather than historical-only proof.

Current verified LAN replay-consumer truth on 2026-07-19:

- The tracked WP25 proof source is
  `docs/proof/lan-plan/25-rollout-checklist-and-pr-gate/`. Generated rerun
  output remains untracked and does not replace this source-of-record proof.

- The parent-owned Rust agent-service client loads the canonical
  `AgentLanRuntimeEventChainStreamGet` response and converts validated discovery
  rows into existing `ParentRouteEventSnapshot` values for Rust-owned
  `ParentSubscriptionEvent` construction. This does not by itself prove that a
  real Tauri emitter delivered the batch to the portal listener.
- The independently loaded status and replay histories must agree on state,
  latest event ID, and latest observed time. A mutation between those two reads
  rejects the replay batch instead of combining different history versions.
- Replay order is preserved. Duplicate IDs, stale or out-of-order chains,
  broken references, inconsistent metadata, producer-incompatible
  history-state/material-row combinations, and malformed payloads reject the
  whole replay batch. Empty history accepts canonical metadata-only
  `interface-changed`, `scan-started`, and `scan-finished` rows but rejects
  material discovery rows; ready history requires at least one material row;
  manual-required history cannot carry rows. Unavailable takes precedence
  before row inspection, while degraded and agent-offline precedence also
  remains explicit. The last live LAN status events remain authoritative on an
  ID collision.
- Replay rejection does not erase the status snapshot, so explicit
  `agent-offline` and `manual-required` read-model states remain visible. The
  bridge emits one host-owned warning event with a unique safe event ID, valid
  RFC3339 timestamp, fixed service-to-portal peers and roles, warning severity,
  and no rejected input identifiers or payload.
- The Tauri host delivery decision now emits a new replay event ID even when
  the route snapshot is unchanged and suppresses already delivered IDs. That
  exact decision seam has focused crate-level coverage. A real `AppHandle`
  emit observed by the portal listener is still open.
- The isolated portal state edge accepts a later live status event without
  applying its timestamp ceiling to replay rows, fails closed on nonempty replay
  rows with missing, invalid, or mismatched snapshot metadata, and preserves the
  identical newest 128 rows when a 129-plus full history is repeated.
- Focused protocol and agent-service stream tests, the parent runtime
  diagnostics/history/replay integration group, the desktop host-decision test,
  and the isolated portal-state edge test are green. Exact commands and counts,
  including later scoped gates, are recorded in the current proof root. These
  are separate automated seams and do not constitute backend-to-Tauri-emitter-
  to-portal-listener proof or manual runtime proof.

Accordingly, WP25 is Phase 1 incomplete until WP16 adds the integrated
backend-to-`AppHandle`-to-listener regression and WP20 restores or replaces its
missing aggregate verifier programs. Physical/manual topology evidence remains
a later Phase 3 gate.
- WP25 remains `partial/code-test gap`. The existing seam coverage does not close the
  Tauri-emitter-to-portal-listener gap, physical multi-device, router/firewall,
  signed-artifact, restart, or manual topology evidence rows and does not
  support broad PR-ready or DONE.

## Automated Gates For Active Scope

| Check | Direction |
| --- | --- |
| Shared schema / bridge drift | focused Rust-owned schema and generated bridge drift checks pass |
| Rust compile | owning protocol/service/runtime crates compile cleanly |
| Rust tests | owning contract/runtime test groups pass |
| UI compile | pure UI compile passes without becoming contract authority |
| Portal browser checks | real presentation-only UI flows pass where the route exists and only after the consumed Rust proof roots are already honest |
| Proof scripts | only current, non-stale proof scripts are accepted |

Only real organized test folders/groups count in these gates. Inline
source-owned tests, placeholder directories, `.gitkeep` trees, fake coverage,
and mock-only readiness do not satisfy rollout closure.

## Proof Artifact Checklist For Active Scope

- rollout truth exists at
  `docs/proof/lan-plan/25-rollout-checklist-and-pr-gate/01-rollout-gate-truth.md`
- rollout validation history exists at
  `docs/proof/lan-plan/25-rollout-checklist-and-pr-gate/16-validation-commands.log`
- generated rerun output stays untracked under `output/` or in CI artifacts
- earlier packet proof roots exist and match the current Rust-owned direction
- UI screenshot/snapshot proof exists only for real surfaces
- any UI/browser artifact is attached as supporting presentation evidence, not
  as LAN proof authority
- manual-required proof remains open where physical two-device LAN evidence is
  still missing

## Security no-claim checks

- no portal surface claims a device is protected before the Rust-owned
  capability/runtime status proves it
- observer-style roles cannot issue write commands
- revoked and wrong-device command paths reject cleanly and are audited
- deferred delivery or notification paths remain explicitly unclaimed

## Feature doc checklist updates

Only move feature or product checklist rows when real proof artifacts exist.
Do not convert a draft rollout gate into product readiness by documentation
alone.

## Manual-required gaps

Two-device physical LAN proof remains manual-required before any broad
multi-device household readiness claim. Automated proof can be complete while
that row stays open.

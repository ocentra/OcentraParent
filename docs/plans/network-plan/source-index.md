# Source Index

<!-- agent-capsule -->

> Agent Capsule
> Plan: `network-plan`
> Doc: `Source Index`
> Kind: source ownership index; read only when source ownership is unclear.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not inspect broad source from here; use only the named package/crate path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.

<!-- /agent-capsule -->

This index records the source files and product docs that own network evidence
and network/domain control work. Use it before implementation so a worker does
not invent a second truth.

## Product Sources

| Source                                                                                         | Ownership                                                                                                              |
| ---------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------- |
| `docs/feature-list.md`                                                                         | Lists Network and domain control as the owning feature area.                                                           |
| `docs/features/network-domain-control.md`                                                      | Owns network/domain product outcome, current state, gap, checklist, and next AI instructions.                          |
| `docs/expectations/network-flow-evidence.md`                                                   | Owns metadata-only evidence scope, AI/policy digest boundary, and no decrypted-content claims.                         |
| `docs/expectations/policy.md`                                                                  | Owns parent-authored network/domain/category/VPN/proxy targets and typed policy decisions.                             |
| `docs/expectations/enforcement.md`                                                             | Owns enforcement authority, adapter result, audit, rollback, and dry-run boundaries.                                   |
| `docs/expectations/ai.md`                                                                      | Owns AI input boundaries for network summaries and unusual-traffic digests.                                            |
| `docs/expectations/platforms.md`                                                               | Owns platform claim rules and manual proof requirements.                                                               |
| `docs/architecture/network-flow-evidence-capture.md`                                           | Owns the current Windows-first observation architecture and attribution truth ladder.                                  |
| `docs/plans/network-plan/workpacks/network-control-capability-guide.md`                        | Design input for network capabilities, proof limits, and policy/control groups.                                        |
| `docs/plans/network-plan/workpacks/network-control-schema-proposal.md`                         | Schema design input for network control settings.                                                                      |
| `docs/plans/network-plan/workpacks/network-control-settings-inventory.md`                      | Generated network control inventory; not implementation proof.                                                         |
| `docs/plans/eventing-plan/README.md`                                                           | Owns the reusable Rust eventing plan that network Workpack 10 must consume before network-specific bus routing exists. |
| `docs/plans/v0-8-enforcement-control-plan/workpacks/08-network-domain-report-only-boundary.md` | Existing V0.8 report-only/manual-required network-domain enforcement boundary.                                         |

## Imported DOCX V2 Inputs

These local DOCX inputs were reviewed on 2026-06-03 and reconciled into this
plan package. They are planning inputs, not implementation proof.

| Source                                                 | Covered Plan Area                                                                                                                                                          |
| ------------------------------------------------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `E:\Download\doc1_network_plan_v2.docx`                | Event-driven network intervention, packet tooling, Zeek/Suricata/Snort analysis inputs, typed topics, AI audit, risk budgets, cascade reactions, and proof-gated controls. |
| `E:\Download\doc2_network_tests_blueprint_v2.docx`     | PCAP fixture families, parser/analyzer tests, event bus resilience, detection and cascade E2E tests, performance metrics, UI proof, and proof dossier contents.            |
| `E:\Download\doc3_network_checklist_workpacks_v2.docx` | Workpack additions for capture logging, classification, event streaming, AI audit, cascade controls, portal UX, hardening, compliance, deployment, and support.            |

## Current Code Foundation

| Source                                                         | Current Role                                                                              |
| -------------------------------------------------------------- | ----------------------------------------------------------------------------------------- |
| `crates/network-core/src/network_runtime.rs`                   | Rust network runtime/domain decisions and replay-safe network behavior.                  |
| `crates/agent-protocol/src/network_flow.rs`                    | Canonical Rust protocol payload, A-D grade, policy-action, and event contract shapes.    |
| `crates/agent-protocol/tests/contract/network_eventing_contract.rs` | Payload mutation, version-skew, and shared-eventing contract tests.                  |
| `crates/agent-protocol/src/constants/network_flow.rs`          | Rust protocol constants for network flow boundaries.                                      |
| `crates/agent-core/src/network_capture*.rs`                    | Current network capture adapter foundation.                                               |
| `crates/agent-core/src/activity_store_network_flow*.rs`        | Network flow journal/SQLite storage foundation.                                           |
| `crates/agent-core/src/network_event_runtime*.rs`              | Runtime spine, handler receipt, queue/replay, no-enforcement proof, and journal-state boundary. |
| `crates/agent-service/src/service_runtime.rs`                  | Service-lifetime startup owner for the shared network runtime spine.                         |
| `crates/agent-service/src/network_runtime_delivery.rs`          | Shared-spine network delivery path and durable-journal readiness projection.                  |
| `crates/agent-service/src/network_runtime_stream_payload.rs`   | Shared-spine runtime stream path and durable-journal readiness projection.                   |
| `crates/agent-service/src/network_flow_digest*.rs`             | Service digest rollups and unusual indicators.                                            |
| `crates/agent-service/src/activity_network_flow_payload.rs`    | Service payload boundary for activity network flow.                                       |
| `crates/ocentra-network-evidence/src/*`                        | Parser, classifier, cascade, adapter-gate, performance, and platform-claim proof logic.  |
| `apps/portal/src/use-portal-network-activity-refresh.ts`       | Real portal network refresh routing.                                                      |
| `apps/portal/tests/live-activity-network-flow.test.ts`         | Portal network flow read-model, drawer, and no-claim behavior tests.                      |
| `apps/portal/e2e/network-evidence-drawer-proof.spec.ts`        | Portal service-backed network drawer e2e proof route.                                     |
| `scripts/test/v0-8-browser-domain-adapter-proof.mjs`           | Existing browser/domain adapter proof harness input; does not prove broad host filtering. |

## Existing Plan Neighbors

Network work must coordinate with:

- `docs/plans/browser-plan/README.md` for exact URL/tab and managed browser evidence.
- `docs/plans/app-game-plan/README.md` for foreground process/app/game/session confirmation.
- `docs/plans/screen-plan/README.md` for opt-in screen summaries when network evidence is ambiguous.
- `docs/plans/ai-plan/README.md` for local AI runtime and summary consumption.
- `docs/plans/v0-8-enforcement-control-plan/README.md` for adapter authority and audit.
- `docs/plans/lan-plan/README.md` for LAN device discovery and trusted route state.

## Ocentra Games Event Bus Reference

The Ocentra Games event bus is reference material for the reusable Rust
eventing plan. Network must not copy the TypeScript package directly or create
a private network-only bus. The implementation order is reusable Rust bus first,
then Parent network event contracts and handlers.

Reference sources in `E:\ocentra-games`:

| Source                                                           | Reusable Semantics                                                                                                                      |
| ---------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------- |
| `packages/eventing-domain/src/core/EventBus.ts`                  | Typed subscribe/publish, sync and async subscribers, queueing, retry, TTL, in-flight republish guard, and singleton/test injection.     |
| `packages/eventing-domain/src/core/EventRegistrar.ts`            | Registrar-owned subscriptions and dispose/unsubscribe lifecycle.                                                                        |
| `packages/eventing-domain/src/core/EventArgsBase.ts`             | Event id, timestamp, unique id, republish flag, target handler marker, and dispose hook.                                                |
| `packages/eventing-domain/src/core/OperationResult.ts`           | Success/failure result wrapper with attempts and error message.                                                                         |
| `packages/eventing-domain/src/core/OperationDeferred.ts`         | Request/response deferred completion carried on event payloads.                                                                         |
| `packages/eventing-domain/src/core/createEventRegistrar.ts`      | Small registrar factory for scoped handler ownership.                                                                                   |
| `packages/eventing-domain/src/testing/createTestEventBus.ts`     | Isolated event bus for deterministic tests.                                                                                             |
| `packages/eventing-domain/src/events/assets/UploadAssetEvent.ts` | Event class carrying command data plus deferred response.                                                                               |
| `packages/eventing-domain/src/events/assets/SyncToR2Event.ts`    | Fire-and-complete event pattern with deferred completion.                                                                               |
| `src/adapters/network/NetworkRouter.ts`                          | Consumer handler pattern: subscribe to event contracts, execute side effect, resolve deferred result, and honor target handler markers. |
| `src/lib/eventing/README.md`                                     | App-layer React/service wiring separated from the eventing-domain runtime.                                                              |

Ocentra Parent must not copy Ocentra Games event names directly into app or
runtime code. Parent event identifiers should live in protocol/domain constants
before runtime code consumes them, with tests for collision, routing, deferred
completion or result events, queue expiry, and no adapter execution in dry-run
or manual-required states. Vite/TypeScript UI code cannot publish business
events or adapter commands.

## Platform References To Verify During Workpacks

Workers must verify current official docs before claiming platform-specific
implementation behavior. This source index names candidate platform families;
it does not prove them.

| Platform      | Candidate Path To Verify                                                                                               |
| ------------- | ---------------------------------------------------------------------------------------------------------------------- |
| Windows       | IP Helper, ETW, DNS client events/cache, Windows Firewall, Windows Filtering Platform, Npcap/libpcap where used.       |
| Android       | VpnService, DNS/private DNS behavior, Device Owner or managed-profile policy where used.                               |
| macOS         | Network Extension, packet tunnel, content filter, DNS proxy, configuration profile/MDM requirements.                   |
| iOS           | Network Extension, Family Controls or Screen Time interaction if relevant, supervised/MDM or entitlement requirements. |
| Linux         | libpcap, nftables, eBPF/cgroup hooks, TUN/TAP, distro/service-manager assumptions.                                     |
| Router/import | DHCP/router logs, DNS resolver logs, source device mapping, NAT ambiguity, custody limits.                             |

## Not Product Proof

- A plan file.
- A type definition without parser/storage/runtime proof.
- A PCAP fixture without live adapter proof.
- Hosted CI on a platform without real OS permission/device artifacts.
- Browser or portal screenshots without service-backed network evidence.
- AI classification without stored evidence refs.
- DNS/domain rules without adapter result, audit, and rollback/unavailable proof.
- A generated network-control catalog or settings inventory by itself.
- A proof script path without a current committed proof bundle or explicit blocker note.

# Current Network Snapshot

<!-- agent-capsule -->

> Agent Capsule
> Plan: `network-plan`
> Doc: `Current Network Snapshot`
> Kind: current snapshot; read for status/gap claims.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

## Current Product State

Network/domain control has useful foundation but is not product-complete.
Current Ocentra truth is:

- network flow summaries and unusual-indicator reconciliation exist in proof
  form;
- TypeScript and Rust protocol/read-model foundations exist for network flow
  evidence, including row10 remote-delivery status and network runtime event
  parity;
- deterministic PCAP/parser/analyzer/signature, AI audit, risk-budget,
  policy-preview, retention/read-model, platform/manual-required, adapter
  capability/status, action-result, performance, and security proof artifacts
  exist under `output/network-plan-proof/` and `test-results/network-*-proof/`;
- bounded live-capture execution is modeled as a separate row13b proof: driver
  execution needs proof-ready row13 refs plus invocation, interface, permission,
  bounded-window, clean-stop, custody, retention/delete/export, metadata-only
  sanitization, and private-traffic-exclusion refs; metadata snapshots do not
  substitute for Npcap/libpcap capture;
- Android physical target identity is now a separate read-only proof for the
  named Galaxy S9 target: ADB connect, `adb devices -l`, and getprop
  observations must match expected product/model/device/release/ABI refs before
  the target can satisfy physical-device evidence. This does not execute
  VpnService, packet capture, packet blocking, app correlation, or adapter
  authority;
- Windows Firewall bounded lab execution is now a separate row38a proof: an
  apply-ready row38 adapter proof can be paired with an Ocentra lab rule name,
  an RFC 5737 TEST-NET target, administrator permission, and
  apply/verify-present/rollback/verify-removed command evidence before it can
  report executed-and-rolled-back. Without host/admin/command evidence it stays
  manual-required or unavailable. This is not production enforcement and does
  not claim persistent host filtering;
- portal visibility exists for service-backed network read models, including
  network evidence drawer status for activity rows and adapter/platform
  capability state;
- policy can reference stored flow evidence in dry-run/report-only contexts;
- host network/domain filtering remains manual-required unless a real OS
  adapter proof exists;
- exact URL/content/page/video/message/search claims cannot come from network
  metadata alone.

## Existing Foundation

| Area                           | Existing Evidence                                                                                                                   | Status                                   |
| ------------------------------ | ----------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------- |
| Feature doc                    | `docs/features/network-domain-control.md`                                                                                           | Existing.                                |
| Expectation docs               | `docs/expectations/network-flow-evidence.md`, `docs/expectations/policy.md`, `docs/expectations/enforcement.md`                     | Existing.                                |
| Architecture                   | `docs/architecture/network-flow-evidence-capture.md`                                                                                | Existing Windows-first observation plan. |
| Settings/catalog docs          | `docs/network-control-capability-guide.md`, `docs/network-control-schema-proposal.md`, `docs/network-control-settings-inventory.md` | Existing design inputs.                  |
| TypeScript contracts           | `packages/activity-domain/src/network-flow.ts`, `packages/agent-protocol-domain/src/network-runtime-events.ts`                      | Contract and runtime-event proof exists. |
| Rust protocol                  | `crates/agent-protocol/src/network_flow.rs`, `crates/agent-protocol/src/network_flow_events.rs`                                     | Protocol parity proof exists.            |
| Rust capture/storage           | `crates/agent-core/src/network_capture*.rs`, `crates/agent-core/src/activity_store_network_flow*.rs`                                | Deterministic proof foundation.          |
| Rust service digest/read model | `crates/agent-service/src/network_flow_digest*.rs`, `crates/agent-service/src/activity_network_flow_payload.rs`                     | Service read-model proof exists.         |
| Portal read model              | `apps/portal/src/network-flow-read-model.ts`, `apps/portal/src/live-network-flow-panel.ts`                                          | Service-backed proof visibility exists.  |
| Enforcement boundary           | `docs/plans/v0-8-enforcement-control-plan/workpacks/08-network-domain-report-only-boundary.md`                                      | Manual-required/report-only boundary.    |

## Known Gaps

- Production live packet capture driver support and live raw artifact creation.
- Router/log import implementation proof.
- Live broker/family-hub transport, provider delivery, child-device delivery,
  remote acknowledgement handling, and remote delete/export propagation.
- Local AI model execution or remote provider execution.
- Full policy engine execution and notification provider delivery.
- Live host DNS/WFP/VPN/NetworkExtension/Linux adapter mutation, packet
  blocking, process termination execution, and host filtering. Windows Firewall
  has only a bounded reversible TEST-NET lab execution proof; production
  enforcement and persistent policy-driven firewall rules remain open.
- Physical-device proof beyond the named Android target, Device Owner or other
  authority-enrolled proof, and any platform adapter execution proof where a
  platform claim needs it.
- Parent-facing rule UX and broader risk-budget/performance/platform UI beyond
  the current service-backed network drawer.
- Production SLO validation, external audit or penetration-test execution,
  deployment execution, and full support-material authoring.

## Product Boundary

Default network evidence path:

```text
network observation
  -> typed network evidence
  -> encrypted journal
  -> SQLite query store
  -> network summaries and unusual indicators
  -> local AI summaries or deterministic policy refs
  -> dry-run or manual-required enforcement handoff
  -> parent-visible evidence and audit
```

Future strict intervention path:

```text
stored network evidence
  -> parent-authored rule
  -> typed policy decision
  -> adapter capability check
  -> platform adapter action
  -> adapter result
  -> audit event
  -> rollback, expiry, unavailable, or manual-required state
```

Network observation and network intervention remain separate. Observing a flow
does not prove the product can block it.

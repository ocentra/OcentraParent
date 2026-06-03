# Current Network Snapshot

## Current Product State

Network/domain control has useful foundation but is not product-complete.
Current Ocentra truth is:

- network flow summaries and unusual-indicator reconciliation exist in proof
  form;
- TypeScript and Rust protocol/read-model foundations exist for network flow
  evidence;
- portal visibility exists for service-backed network read models;
- policy can reference stored flow evidence in dry-run/report-only contexts;
- host network/domain filtering remains manual-required unless a real OS
  adapter proof exists;
- exact URL/content/page/video/message/search claims cannot come from network
  metadata alone.

## Existing Foundation

| Area | Existing Evidence | Status |
| --- | --- | --- |
| Feature doc | `docs/features/network-domain-control.md` | Existing. |
| Expectation docs | `docs/expectations/network-flow-evidence.md`, `docs/expectations/policy.md`, `docs/expectations/enforcement.md` | Existing. |
| Architecture | `docs/architecture/network-flow-evidence-capture.md` | Existing Windows-first observation plan. |
| Settings/catalog docs | `docs/network-control-capability-guide.md`, `docs/network-control-schema-proposal.md`, `docs/network-control-settings-inventory.md` | Existing design inputs. |
| TypeScript contracts | `packages/activity-domain/src/network-flow.ts` | Partial foundation. |
| Rust protocol | `crates/agent-protocol/src/network_flow.rs` | Partial foundation. |
| Rust capture/storage | `crates/agent-core/src/network_capture*.rs`, `crates/agent-core/src/activity_store_network_flow*.rs` | Partial foundation. |
| Rust service digest/read model | `crates/agent-service/src/network_flow_digest*.rs`, `crates/agent-service/src/activity_network_flow_payload.rs` | Partial foundation. |
| Portal read model | `apps/portal/src/network-flow-read-model.ts`, `apps/portal/src/live-network-flow-panel.ts` | Partial foundation. |
| Enforcement boundary | `docs/plans/v0-8-enforcement-control-plan/workpacks/08-network-domain-report-only-boundary.md` | Manual-required/report-only boundary. |

## Known Gaps

- Source reconciliation into implementation contracts, runtime modules, and
  proof artifacts.
- PCAP replay harness and fixture taxonomy, including safe baseline,
  suspicious, and edge/high-concurrency fixture families.
- Packet-tool comparison and analyzer proof for tcpdump, dumpcap, TShark,
  Wireshark, Zeek-style logs, Suricata alerts, and Snort-compatible rules where
  used.
- DNS query/response parsing proof.
- TLS SNI and HTTP Host parsing proof where visible.
- QUIC/HTTP3 limitation and DoH/DoT detection proof.
- Flow aggregation/sessionization proof.
- Domain normalization and category classifier proof.
- VPN/proxy/Tor/tunnel classifier proof.
- Network evidence grade model.
- Network-triggered evidence cascade model.
- Cross-slice network evidence bundle model.
- Network event bus contract that consumes the reusable Rust eventing plan, with
  Ocentra Games eventing semantics used only as reference material.
- Event topic namespace, publisher/subscriber auth, backpressure, retention,
  filtering, and broker/family-hub decision proof.
- AI detection model proof over fixture summaries and signature alerts, without
  raw PCAP, payloads, or decrypted content.
- AI audit narrative proof with parent-readable recommendations and cited
  evidence refs.
- Household risk budget, age/profile threshold, and cascade reaction model.
- Parent UI evidence drawer and limitation messaging.
- DNS proxy/block/redirect adapter proof.
- Windows Firewall adapter proof.
- Windows WFP observation/intervention proof.
- Android VpnService proof.
- Apple Network Extension proof.
- Linux nftables/eBPF/TUN proof.
- Router/log import proof.
- Physical device and authority-enrolled proof where needed.
- Throughput, packet-to-detection latency, resource impact, and high-concurrency
  benchmark proof.
- Security, privacy, retention, delete/export, support, and staged rollout proof.

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

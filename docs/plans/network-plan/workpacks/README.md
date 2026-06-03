# Network Plan Workpacks

Each workpack should be implemented as a narrow, proof-backed slice. A workpack
can be split further when source-shape, platform proof, or validation needs it.

## Workpack Matrix

| Id | Workpack | Required Proof Tier | Primary Proof |
| --- | --- | --- | --- |
| 01 | Source index and repo reconciliation | P0 | Docs source index and current repo file map. |
| 02 | Current network snapshot and gap map | P0 | Current-state doc reconciled with feature/checklist status. |
| 03 | Contract boundary and Effect schemas | P0 | TypeScript schema tests. |
| 04 | Rust protocol parity for network contracts | P0 | Rust serialization/parity tests after TS contracts. |
| 05 | NetworkFlowEvidence contract | P0 | Valid/invalid contract tests. |
| 06 | NetworkDomainEvidence contract | P0 | DNS/SNI/HTTP host/IP-only state tests. |
| 07 | NetworkActivityClassification contract | P0 | Classification enum/confidence/uncertainty tests. |
| 08 | NetworkEvidenceGrade model | P0 | A/B/C/D mapping tests. |
| 09 | NetworkPolicyAction and capability contract | P0 | Dry-run/manual-required/action validation tests. |
| 10 | NetworkActivityEvent contracts and reusable Rust eventing consumption | P0/P2 | Reusable Rust eventing plan implemented first; network-specific event contracts, routing tests, deferred/result-event tests, queue/TTL tests, and no-adapter-call guards consume the shared bus instead of creating a private network bus. |
| 11 | Rust crate evaluation | P0 | Decision record with official docs and package/license/security notes. |
| 12 | PCAP file replay harness | P1 | Replay command, fixtures, expected JSON outputs. |
| 13 | Live pcap/Npcap/libpcap capture adapter | P3 | Local host capture proof with driver/permission state. |
| 14 | Packet parser | P1 | Fixture parser tests for Ethernet/IP/TCP/UDP/ICMP. |
| 15 | DNS query/response parser | P1 | DNS fixture tests and malformed packet tests. |
| 16 | TLS ClientHello/SNI parser | P1 | SNI-visible and SNI-hidden fixture tests. |
| 17 | HTTP Host parser | P1 | Plain HTTP host parser tests and HTTPS no-claim tests. |
| 18 | QUIC/HTTP3 limitation detector | P1 | QUIC unknown/limited-visibility tests. |
| 19 | DoH/DoT detector | P1 | Candidate detector tests with no visited-domain claim. |
| 20 | Flow aggregation/sessionization | P1 | 5-tuple, reverse direction, timeout, byte/count tests. |
| 21 | Domain normalization and public suffix model | P1 | Public suffix and malformed domain tests. |
| 22 | Domain/category intelligence database | P1 | Source/custody/freshness tests and update policy. |
| 23 | Social/video/game/cloud-gaming classifier | P1 | Domain, CDN, process, browser confirmation tests. |
| 24 | VPN/proxy/Tor/tunnel classifier | P1 | Indicator tests and hidden-destination negative tests. |
| 25 | Remote desktop/torrent/download classifier | P1 | Candidate tests and uncertainty states. |
| 26 | Process/app correlation model | P3 | Real or replay-backed process attribution proof. |
| 27 | Managed browser correlation bridge | P3 | Exact URL only from managed browser evidence proof. |
| 28 | Unmanaged browser correlation | P3 | Process candidate proof without exact URL claim. |
| 29 | App/game foreground/session correlation | P3 | Stored app/game evidence refs and launcher-only guards. |
| 30 | Screen summary trigger integration | P3 | Screen-plan opt-in and skipped/manual-required proof. |
| 31 | Evidence cascade router | P1 | Ordered next-check tests and weak-hint no-block tests. |
| 32 | Cross-slice evidence bundle builder | P1 | Bundle proof-level and must-not-claim tests. |
| 33 | Network-triggered local AI queue | P3 | AI receives only summaries/evidence refs. |
| 34 | Evidence-grade policy mapping | P3 | Policy decisions by grade with parent rule refs. |
| 35 | Parent notification candidate mapping | P3 | Alert candidates with no provider-delivery claim unless proved. |
| 36 | Parent UI network evidence drawer | P3 | Service-backed Playwright proof. |
| 37 | DNS proxy/block/redirect adapter | P3 | DNS apply/result/rollback/audit proof. |
| 38 | Windows Firewall adapter | P3 | Firewall apply/result/rollback/audit proof. |
| 39 | Windows WFP research/proof gate | P5 | Signed/permissioned WFP proof or manual-required state. |
| 40 | Android VpnService adapter/proof gate | P4/P5 | Physical-device VpnService proof; Device Owner proof if claimed. |
| 41 | Apple Network Extension adapter/proof gate | P4/P5 | Entitlement/device/supervision proof or manual-required state. |
| 42 | Linux nftables/eBPF/TUN adapter/proof gate | P4/P5 | Distro-specific adapter proof with rollback/audit. |
| 43 | Zeek-style structured log generator and analyzer comparison proof | P1 | Fixture PCAP to connection/DNS/HTTP/TLS summaries with TShark/Wireshark comparison artifacts. |
| 44 | Suricata/Snort-compatible signature alert ingestion proof | P1 | Signature alert fixtures, typed analyzer alert records, false-positive guards, and no-adapter-call proof. |
| 45 | Event topic namespace, publisher SDK, subscriber filtering, backpressure, retention, and broker/family-hub decision proof | P0/P6 | Domain-owned event constants, local-first bus proof, broker decision record, and auth/encryption/retention/replay/delete requirements if broker-backed. |
| 46 | AI detection model fixture evaluation and drift/precision proof | P3 | Deterministic summary/alert fixtures, precision/recall/confusion matrix, drift notes, and raw-PCAP rejection proof. |
| 47 | AI audit narrative and recommendation proof | P3 | Parent-readable narrative fixtures with cited evidence refs, uncertainty, and unsupported-claim rejection proof. |
| 48 | Household risk budget and cascade threshold model | P3 | Age/profile/household policy thresholds, prior-event behavior, action mapping, and audit proof. |
| 49 | Performance, latency, resource, and high-concurrency benchmark proof | P3/P6 | Packet-to-detection latency, event throughput, CPU/memory/disk/queue metrics, and high-concurrency fixture results. |
| 50 | Security, privacy, compliance, deployment, support, and staged rollout proof | P6 | Threat model, retention/delete/export, key/secrets handling, support playbooks, training, rollout gates, and known-gap signoff. |

## Required Workpack Record

Each workpack should include:

```text
status:
requiredProofTier:
currentProofTier:
artifactPath:
docsRead:
touchedFiles:
validation:
knownGaps:
manualRequiredReason:
nextAction:
```

## Workpack 10 Reuse Target

Workpack 10 should start from the reusable Rust eventing plan instead of
designing a new bus from scratch.

Primary dependency:

- [Reusable Rust Eventing Plan](../../eventing-plan/README.md)

Reusable Ocentra Games semantics to preserve through the Rust eventing plan:

- class-backed event contracts with canonical static event type identifiers;
- `EventArgsBase` fields for timestamp, unique id, republish flag, target
  handler, and disposal;
- `EventBus` subscribe, subscribeAsync, unsubscribe, publish, publishAsync, and
  clear;
- `EventRegistrar` scoped subscription ownership;
- `OperationResult` and `OperationDeferred` request/response behavior;
- queueing, retry, TTL, max queue, timeout, in-flight duplicate guard, and
  isolated test bus.

Rust reusable-bus requirements before network consumes it:

- the generic eventing crate is shared by parent/controller and child-agent Rust
  runtimes;
- event identifiers live in protocol constants or enums before Parent runtime
  consumes them;
- envelopes carry event id, source evidence refs, custody/source state, target
  handler, retry state, and typed payload;
- async dispatch uses Tokio-friendly channels or traits;
- request/response uses oneshot completion with typed result;
- queue/retry/TTL/in-flight guards match the TypeScript behavior where the
  service needs it;
- dry-run, manual-required, and unavailable states cannot call adapters through
  the bus.
- Vite/TypeScript UI cannot publish business events or adapter commands.

## Reporting Rule

Worker `DONE` reports must include:

- exact branch and commit state;
- touched packages/files;
- validation commands and results;
- proof artifact paths;
- feature doc and checklist row updated, or why no update was needed;
- known gaps/risks;
- whether the work changed status or only added plan/proof coverage.

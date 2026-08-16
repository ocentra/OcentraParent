# Network Plan

This folder is the single working plan location for child-device network
evidence, domain observation, DNS and flow classification, process/app/browser
correlation, network-triggered cross-slice evidence cascade, network policy
handoff, DNS/firewall/VPN/WFP/NetworkExtension intervention paths, proof
artifacts, and parent UI.

This is an end-state plan. Work can land in small slices, but the requirements
stay explicit and proof-driven.

## Source Docs

- `docs/features/network-domain-control.md`
- `docs/expectations/network-flow-evidence.md`
- `docs/expectations/policy.md`
- `docs/expectations/enforcement.md`
- `docs/expectations/ai.md`
- `docs/expectations/platforms.md`
- `docs/architecture/network-flow-evidence-capture.md`
- `docs/network-control-capability-guide.md`
- `docs/network-control-schema-proposal.md`
- `docs/network-control-settings-inventory.md`

Imported planning inputs:

- `E:\Download\doc1_network_plan_v2.docx`
- `E:\Download\doc2_network_tests_blueprint_v2.docx`
- `E:\Download\doc3_network_checklist_workpacks_v2.docx`

## Core Rule

```text
Network is the tripwire.
Browser, app/game, and screen evidence are confirmation sensors.
AI is the structured analyzer.
Policy is the authority.
Adapters are the hands.
Audit is the proof.
```

Network evidence can prove destinations, flows, DNS, timing, volume, protocol,
ports, interface/source state, and sometimes process or app correlation. Network
evidence alone must not claim exact HTTPS URLs, exact videos/posts, private
messages, search queries, page content, screen activity, decrypted content, or
that the child saw a specific item.

## Non-Negotiable Boundaries

```text
No HTTPS MITM by default.
No decrypted payload capture by default.
No raw packet dumps sent to AI.
No remote/cloud upload of child network evidence by default.
No exact URL, video, post, message, search, or page-content claim from network-only evidence.
No enforcement action without a parent-authored rule, evidence refs, a typed policy decision, adapter capability proof, audit event, and rollback/unavailable state.
```

## LAN Discovery Versus Network Evidence

LAN discovery answers:

- what devices exist on the local network;
- which discovered devices are paired, trusted, ignored, or unknown;
- which child agents are reachable.

Network evidence answers:

- what traffic is leaving or entering a child device;
- which domains, IPs, protocols, ports, and processes are involved where
  observable;
- whether traffic suggests social, video, game, VPN, proxy, bypass, remote
  desktop, torrent, update, or unknown activity;
- what evidence slices should be triggered next;
- whether a network-level intervention is possible and proved.

## Where We Are

- Network/domain control is already a first-class feature doc.
- Network flow expectations already define metadata-only evidence and no
  decrypted-content claims.
- The architecture doc already defines a Windows-first observation path through
  endpoint snapshots, DNS/domain attribution, journal/SQLite ingest, local AI
  references, and portal read models.
- TypeScript, Rust protocol, Rust core, Rust service, and portal read-model
  foundation exists for network flow evidence and digests.
- Network control settings inventory exists as design input.
- Network/domain blocking remains manual-required/report-only unless a real OS
  adapter proof exists.

## Where We Want To Be

Ocentra needs a full network subsystem that can:

- observe metadata through proof-gated adapters;
- replay PCAP fixtures for deterministic parser and classification proof;
- use tcpdump, dumpcap, TShark, Wireshark, Zeek, Suricata, and Snort as
  fixture, analyzer, comparison, and alert inputs where each tool path is
  explicitly proved;
- normalize DNS/domain/IP/protocol/process/app/browser evidence;
- classify social, video, game, cloud gaming, VPN/proxy/tunnel, DoH/DoT, Tor,
  remote desktop, torrent, update, school/productivity, and unknown traffic;
- route observation, summary, signature alert, detection, audit, and control
  requests through a typed local event bus before considering broker-backed
  family-hub or relay deployments;
- run anomaly and supervised detection only over structured summaries, labels,
  signature alerts, and evidence refs;
- generate parent-readable AI audit narratives without giving AI direct
  enforcement authority;
- evaluate household risk budgets and age/profile thresholds before any ask,
  warn, monitor, limit, or block action;
- assign an explicit evidence grade to every network claim;
- trigger browser, app/game, screen, memory, and local AI checks only when the
  network evidence cannot honestly answer the question alone;
- build cross-slice evidence bundles with explicit must-not-claim fields;
- feed policy with typed evidence refs and parent rules;
- execute DNS, firewall, WFP, Android VpnService, Apple Network Extension, and
  Linux nftables/eBPF/TUN paths only when adapter proof exists;
- surface capability, degraded, manual-required, and unavailable states in the
  parent UI;
- write proof artifacts for every claimed platform capability.

## Plan Files

- [Source Index](source-index.md)
- [Current Network Snapshot](current-network-snapshot.md)
- [Network Evidence And Intervention Full Scope Plan](01-network-evidence-and-intervention-full-scope-plan.md)
- [Network Tests, Proof, And Validation Blueprint](02-network-tests-proof-and-validation-blueprint.md)
- [Network Implementation Checklist And Workpacks](03-network-implementation-checklist-and-workpacks.md)
- [Network Plan Implementation Checklist](implementation-checklist.md)
- [UI/UX Requirements Guide](ui-ux-requirements-guide.md)
- [Pasted Content Coverage Audit](pasted-content-coverage-audit.md)
- [Workpacks](workpacks/README.md)

## Eventing Prerequisite

Network work must not create a private network-only bus. Workpack 10 depends on
the reusable Rust eventing plan:

- [Reusable Rust Eventing Plan](../eventing-plan/README.md)

The shared Rust bus is intended for both parent/controller and child-agent Rust
runtimes. The Vite/TypeScript portal surface remains view/input only and cannot
own network evidence, AI, policy, cascade, enforcement, or audit logic.

## Workpack Checklist

| Step | Workpack                                     | Target State                                                                                                                                           |
| ---- | -------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------ |
| 01   | Source index and repo reconciliation         | Existing feature, expectation, architecture, settings, code, and proof sources are indexed.                                                            |
| 02   | Current network snapshot and gap map         | Current observation/read-model proof and remaining intervention gaps are explicit.                                                                     |
| 03   | Contract boundary and Effect schemas         | Network evidence, domain evidence, classification, grade, action, and cascade contracts are schema-backed.                                             |
| 04   | Rust protocol parity                         | Rust-facing shapes mirror TypeScript contracts after TS tests exist.                                                                                   |
| 05   | PCAP replay and analyzer harness             | Deterministic replay supports parser, Zeek-style log, Suricata/Snort alert, and classifier proof without live adapters.                                |
| 06   | DNS and domain evidence                      | DNS query/response, cache, SNI, HTTP host, reverse lookup, and IP-only states remain separate and graded.                                              |
| 07   | Flow aggregation                             | Flows, sessions, byte/count summaries, and stale/ambiguous states are repeatable and test-backed.                                                      |
| 08   | Classification, detection, and AI audit      | Social/video/game/VPN/proxy/tunnel classifications, anomaly detections, signature alerts, and audit narratives cite evidence and preserve uncertainty. |
| 09   | Network-triggered cascade                    | Browser, app/game, screen, memory, AI, risk-budget, and policy checks are ordered by cheapest safe proof.                                              |
| 10   | Policy and enforcement handoff               | Parent rules and typed decisions gate every adapter action.                                                                                            |
| 11   | Parent UI                                    | Dashboard, timeline, evidence drawer, audit narrative, risk budget, cascade view, policy status, and platform matrix are service-backed.               |
| 12   | Platform, hardening, and release proof packs | Windows, Android, macOS, iOS, Linux, router/import, privacy, security, support, and rollout claims each require their own proof artifacts.             |

## Proof Root

Every implementation workpack should store proof under:

```text
output/network-plan-proof/<workpack-id>/
```

Platform proof should use:

```text
output/network-plan-proof/<platform>/<capability>/
```

## Final Quality Bar

Network is product-credible only when:

```text
Parent can see what was observed and how it was observed.
Every network claim has an evidence grade.
Network-only evidence never becomes exact URL/content proof.
Browser/app/game/screen confirmation is triggered when needed.
AI consumes only typed summaries and evidence refs.
Policy consumes parent-authored rules and evidence refs.
Adapters execute only after typed policy decisions and capability proof.
Manual-required and unavailable states are visible.
Platform proof artifacts exist before platform claims are upgraded.
Audit shows the rule, evidence, decision, adapter result, rollback/unavailable state, and known uncertainty.
Operational proof covers throughput, latency, resource impact, retention, delete/export, incident/support workflow, and staged rollout readiness.
```

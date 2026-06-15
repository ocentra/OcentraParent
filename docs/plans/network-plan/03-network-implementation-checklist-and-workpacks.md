# Network Implementation Checklist And Workpacks

<!-- agent-capsule -->

> Agent Capsule
> Plan: `network-plan`
> Doc: `Network Implementation Checklist And Workpacks`
> Kind: plan reference document; read only when routed by AGENTS, DOC_INDEX, or workpack.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

## Implementation Rule

No checkbox may be marked complete unless proof artifacts exist.

```text
P0 contract proof is not runtime proof.
P1 PCAP replay proof is not live capture proof.
P2 hosted CI proof is not privileged OS proof.
P3 local desktop proof is not mobile or MDM proof.
P4 physical device proof is not authority-enrolled proof.
P5 authority proof is required for strong platform control.
```

## Proof Tiers

```ts
type NetworkProofTier =
  | 'P0_CONTRACT'
  | 'P1_FIXTURE_PCAP_REPLAY'
  | 'P2_HOSTED_CI'
  | 'P3_LOCAL_DESKTOP'
  | 'P4_PHYSICAL_DEVICE'
  | 'P5_AUTHORITY_ENROLLED_DEVICE'
  | 'P6_PRODUCTION_PILOT';
```

Every workpack must state:

```text
requiredProofTier
currentProofTier
status
artifactPath
manualRequiredReason
```

## Workpacks

### Foundation

1. Source index and repo reconciliation.
2. Current network snapshot and gap map.
3. Contract boundary and Effect schemas.
4. Rust protocol parity for network contracts.
5. NetworkFlowEvidence contract.
6. NetworkDomainEvidence contract.
7. NetworkActivityClassification contract.
8. NetworkEvidenceGrade model.
9. NetworkPolicyAction and capability contract.
10. NetworkActivityEvent contracts and handlers that consume the reusable typed
    Rust eventing plan instead of creating a network-only bus.

### Passive Capture And Parsing

11. Rust crate and tooling evaluation: pcap, packet parsing, DNS parsing,
    public suffix, tcpdump, dumpcap, TShark, Wireshark, Zeek, Suricata, and
    Snort-compatible rules.
12. PCAP file replay harness.
13. Live pcap/Npcap/libpcap capture adapter.
14. Packet parser: Ethernet/IP/TCP/UDP/ICMP.
15. DNS query/response parser.
16. TLS ClientHello/SNI parser where visible.
17. HTTP Host parser for plaintext HTTP.
18. QUIC/HTTP3 limitation detector.
19. DoH/DoT detector.
20. Flow aggregation/sessionization.

### Classification And Correlation

21. Domain normalization and public suffix model.
22. Domain/category intelligence database.
23. Social/video/game/cloud-gaming classifier.
24. VPN/proxy/Tor/tunnel classifier.
25. Remote desktop/torrent/download classifier.
26. Process/app correlation model.
27. Managed browser correlation bridge.
28. Unmanaged browser correlation.
29. App/game foreground/session correlation.
30. Screen summary trigger integration.

### Cross-Slice Cascade

31. Evidence cascade router.
32. Cross-slice evidence bundle builder.
33. Network-triggered local AI queue.
34. Evidence-grade policy mapping.
35. Parent notification candidate mapping.
36. Parent UI network evidence drawer.

### Intervention

37. DNS proxy/block/redirect adapter.
38. Windows Firewall adapter.
39. Windows WFP research/proof gate.
40. Android VpnService adapter/proof gate.
41. Apple Network Extension adapter/proof gate.
42. Linux nftables/eBPF/TUN adapter/proof gate.

### Detection, Hardening, And Rollout

43. Zeek-style structured log generator and analyzer comparison proof.
44. Suricata/Snort-compatible signature alert ingestion proof.
45. Event topic namespace, publisher SDK, subscriber filtering, backpressure,
    retention, and broker/family-hub decision proof.
46. AI detection model fixture evaluation and drift/precision proof.
47. AI audit narrative and recommendation proof.
48. Household risk budget and cascade threshold model.
49. Performance, latency, resource, and high-concurrency benchmark proof.
50. Security, privacy, compliance, deployment, support, and staged rollout
    proof.

## Done State Per Workpack

A workpack is done only when applicable proof exists for:

- TypeScript contracts;
- parser or domain tests;
- Rust protocol parity if Rust crosses the boundary;
- journal/SQLite proof if evidence-facing;
- service read-model proof if portal-facing;
- local AI/policy reference proof if AI/policy-facing;
- adapter result and rollback/unavailable proof if intervention-facing;
- live PCAP/raw-capture artifact proof if live capture is used:
  encryption-at-rest, quota rotation, retention/delete/export behavior,
  private-family-traffic exclusion where possible, and manual-required state
  when custody cannot be proved;
- event-bus auth, encryption, backpressure, replay, deletion, and retention
  proof if broker-backed routing is used;
- analyzer alert proof if Zeek, Suricata, Snort-compatible, or similar inputs
  are used;
- AI detection and AI audit proof if detection, narrative, or recommendation
  claims are made;
- risk budget and threshold proof if household or child-profile risk budgets
  influence cascade output;
- performance and resource proof if real-time or near-real-time claims are made;
- support, deployment, and rollback proof if production rollout claims are made;
- security negative tests for overclaim or unsafe execution;
- UI proof if parent-facing;
- manual platform proof if a platform claim is made;
- documentation naming what is possible, hard, unproved, and not claimed.

## Main Network Gates

- [ ] Network plan folder exists.
- [ ] Source index exists.
- [ ] Current snapshot exists.
- [ ] Full-scope plan exists.
- [ ] Test/proof blueprint exists.
- [ ] Implementation checklist/workpacks exist.
- [ ] NetworkFlowEvidence is schema-backed for full end-state fields.
- [ ] NetworkDomainEvidence is schema-backed.
- [ ] NetworkActivityClassification is schema-backed.
- [ ] Evidence grade model exists in source.
- [ ] Network event contracts and reusable eventing projections exist in source.
- [ ] Reusable Rust eventing crate exists before Rust service modules depend on
      bus dispatch.
- [ ] Reusable Rust eventing proof includes validated newtypes, typed live
      envelopes, associated request responses, immutable payloads, and
      no lock-held-await source audit.
- [ ] Network-specific event contracts consume the reusable eventing crate
      rather than creating a network-only bus.
- [ ] Network does not define `NetworkEventBus`, a network dispatch registry,
      a network queue, network retry machinery, or a network request registry.
- [ ] Event topic namespace, publisher, subscriber, backpressure, retention, and
      broker/family-hub decision contracts exist where used.
- [ ] Network-triggered cascade model exists in source.
- [ ] Cross-slice bundle model exists in source.
- [ ] PCAP replay harness exists.
- [ ] Safe, suspicious, and edge PCAP fixture families exist with expected
      outputs.
- [ ] DNS parser exists.
- [ ] Flow aggregator exists.
- [ ] Zeek-style summary generator proof exists before structured analyzer-log
      claims.
- [ ] Suricata/Snort-compatible alert ingestion proof exists before signature
      alert claims.
- [ ] Domain classifier exists.
- [ ] AI detection proof exists over structured summaries and evidence refs.
- [ ] AI audit narrative proof exists with no unsupported exact-content claims.
- [ ] Risk budget and cascade threshold model exists before risk budgets affect
      policy outputs.
- [ ] No exact URL from network-only guard exists.
- [ ] No exact video from network-only guard exists.
- [ ] No message content from network guard exists.
- [ ] AI raw PCAP forbidden guard exists.
- [ ] Performance, latency, resource, and high-concurrency benchmarks exist
      before real-time response claims.
- [ ] Security/privacy/compliance and deployment/support proof exists before
      production rollout claims.
- [ ] DNS proxy proof exists before DNS block claim.
- [ ] Windows Firewall proof exists before Windows firewall block claim.
- [ ] WFP remains manual-required until proof exists.
- [ ] Android VpnService remains manual-required until physical-device proof exists.
- [ ] Apple Network Extension remains manual-required until entitlement/device proof exists.
- [ ] Linux enforcement remains mechanism-specific.

## Merge-Blocking Failures

- network-only evidence claims exact URL;
- network-only evidence claims exact video;
- network-only evidence claims private messages;
- network-only evidence claims search query;
- IP-only becomes domain proof;
- CDN flow becomes exact platform/content proof;
- DoH candidate becomes exact visited domain;
- VPN tunnel becomes hidden destination proof;
- raw PCAP is sent to AI;
- decrypted payload is captured by default;
- HTTPS MITM is enabled by default;
- Grade C evidence blocks without explicit strict parent policy and adapter proof;
- manual-required action calls adapter;
- dry-run blocks traffic;
- remote/cloud upload of child network evidence happens by default;
- signature-only alert causes an adapter command without policy and adapter
  proof;
- broker-backed event routing bypasses local-first custody, retention, deletion,
  auth, or encryption proof;
- AI audit narrative recommends an unsupported exact-content or enforcement
  claim.

## Codex Worker Instruction

```text
Use docs/plans/network-plan as the single full-scope network evidence and intervention plan.

Do not water down the plan.
Do not drop hard features.
If a feature is possible but hard, document authority tier, platform path, proof artifacts, and workpacks.
If a feature cannot be claimed from network-only evidence, encode that as must-not-claim and security tests.
Do not say unsupported unless the authority/proof path is also documented.

Rules:
- Network is a tripwire and intervention layer, not a content oracle.
- Network evidence must be graded A/B/C/D.
- Network event routing should reuse the Ocentra Games eventing-domain semantics
  through the reusable typed Rust eventing plan instead of inventing unrelated
  or network-only bus behavior.
- Network live handlers should receive typed events, not `serde_json::Value`,
  and network event payloads should remain immutable facts.
- Network event routing is local-first; Kafka, RabbitMQ, EventBridge, Pub/Sub,
  or similar broker-backed routing is future scale scope and needs explicit
  custody, auth, encryption, retention, replay, deletion, and performance proof.
- Zeek-style logs and Suricata/Snort-compatible alerts are evidence inputs, not
  policy authority.
- Network-triggered cascade must query browser, app/game, screen, memory, and local AI as needed.
- Risk budgets and cascade thresholds need child profile, household policy,
  prior event, evidence-grade, and audit proof before they influence actions.
- Managed browser evidence is required for exact URL/video claims unless another proved source exists.
- Screen evidence requires screen-plan opt-in.
- AI consumes summaries/evidence refs only, never raw PCAP or decrypted payloads.
- No HTTPS MITM by default.
- DNS proxy is the first intervention path.
- Windows Firewall is the second Windows intervention path.
- WFP, Android VpnService, Apple Network Extension, and Linux nftables/eBPF/TUN are full planned paths with proof gates.
```

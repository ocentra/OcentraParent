# Network Evidence And Intervention Full Scope Plan

<!-- agent-capsule -->

> Agent Capsule
> Plan: `network-plan`
> Doc: `Network Evidence And Intervention Full Scope Plan`
> Kind: plan reference document; read only when routed by AGENTS, DOC_INDEX, or workpack.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

## Product Objective

Ocentra should become a network-aware family safety system, not just a browser,
app, or screen observer.

The child-device agent should detect network metadata in near real time and use
that metadata to trigger the correct evidence cascade.

Example:

```text
network detects video-platform candidate traffic
  -> classify video_platform_candidate
  -> check managed browser for exact URL evidence
  -> check app/game foreground session
  -> if unmanaged or ambiguous, request screen summary only when enabled
  -> if exact URL/video evidence exists, queue local AI analysis
  -> build cross-slice evidence bundle
  -> policy decides allow, warn, ask, limit, block, or unknown
  -> adapter executes only when proof and authority exist
  -> audit the evidence, decision, action, result, and uncertainty
```

## Claim Boundaries

### Network Can Strongly Prove

```text
DNS query or response was observed.
A flow existed between local and remote endpoints.
Traffic volume, duration, packet counts, or byte counts where adapter-supported.
Protocol, port, direction, and timing.
Destination domain where DNS/SNI/HTTP host/browser correlation proves it.
VPN/proxy/tunnel/DoH/DoT candidate where indicators exist.
Process/app correlation where the platform adapter proves it.
An Ocentra-controlled DNS/domain/IP/flow action happened where the adapter result proves it.
```

### Network Can Sometimes Prove

```text
TLS SNI hostname when visible.
HTTP Host for plaintext HTTP.
Process-to-flow mapping.
App/package-to-flow mapping.
QUIC/HTTP3 limitation hints.
Router-level source-device mapping.
Cloud gaming or video streaming pattern candidates.
```

### Network Cannot Honestly Prove By Itself

```text
Exact HTTPS URL path.
Exact YouTube video ID.
Exact Instagram reel/post.
Private messages.
Search query.
Video meaning.
Game scene/content.
Chat contents.
Account creation.
Whether content is educational or entertainment.
Whether the child personally saw the content.
```

### Network Can Trigger Proof Escalation

```text
Network-only social/video/game candidate
  -> trigger browser, app/game, screen, memory, and local AI checks as needed.

Network-only VPN/proxy/DoH candidate
  -> trigger app/process/policy confirmation.

Network-only unknown high-volume traffic
  -> trigger app/process/screen/AI confirmation.
```

## Technical Layers

### Event Bus Layer

The network subsystem should route internal work through a typed event bus
instead of direct module calls between observation, classification, cascade,
policy, intervention, and audit.

Network must consume the reusable Rust eventing plan first:

- [Reusable Rust Eventing Plan](../eventing-plan/README.md)

Use the Ocentra Games `@ocentra/eventing-domain` package as a semantics
reference through that Rust plan, not as a network-local implementation. The
reusable semantics are:

- event classes with canonical event type identifiers;
- event args with timestamp, unique id, republish flag, optional target handler,
  and dispose lifecycle;
- sync and async subscribers;
- scoped registrar ownership with dispose/unsubscribe;
- publish and publish-async result wrapping;
- request/response events through deferred completion;
- queued events when handlers are not registered yet;
- retry, TTL, queue capacity, and async timeout behavior;
- in-flight duplicate guard unless an event is explicitly republishable;
- isolated test bus.

Ocentra Parent should use the shared Rust eventing crate for parent/controller
and child-agent runtimes. The Vite/TypeScript portal surface can send typed
intents and render read models, but it must not own network business logic or
publish enforcement events. Network should consume the Rust eventing
implementation through:

- protocol-owned event type constants or enums;
- network event payload structs carried by typed `ocentra-eventing`
  `EventEnvelope<E>` values;
- `EventContext<E>` handlers and reusable eventing publisher handles;
- reusable publish reports equivalent to success, failure, handled, queued,
  expired, or dropped;
- reusable local request/response handling through `RequestEvent::Response`;
- reusable queue/retry/TTL/in-flight behavior;
- test bus with deterministic queue draining.

Network may define `Network*` event payload structs, event type constants, and
read-model projections only. It must not define `NetworkEventBus`, a network
dispatch registry, a network queue, network retry machinery, or a network
request registry.

Candidate network events:

- `NetworkFlowObserved`;
- `NetworkDomainObserved`;
- `NetworkFlowAggregated`;
- `NetworkActivityClassified`;
- `NetworkCascadeRequested`;
- `NetworkEvidenceBundleBuilt`;
- `NetworkPolicyDecisionRequested`;
- `NetworkInterventionRequested`;
- `NetworkActionResultObserved`;
- `NetworkAuditEventWritten`.

The bus is an orchestration boundary, not authority. Events may request work,
but policy decisions and adapters still need evidence refs, parent rules,
capability proof, and audit.

Network topic families should be represented by domain-owned constants and Rust
protocol constants, not raw strings in app or service code. The planning topic
namespace is:

- network flow observations;
- network flow summaries;
- signature or analyzer alerts;
- AI detections;
- AI audit reports;
- policy and cascade requests;
- control command requests;
- adapter action results.

The default Ocentra Parent path is local-first and should use an in-process,
embedded, or local-service bus before any broker is introduced. Kafka,
RabbitMQ, EventBridge, Pub/Sub, or a similar broker is a P6 family-hub, relay,
or production-scale decision only. Any broker-backed path must prove custody,
publisher/subscriber authentication, encryption, retention, replay, deletion,
and child-device performance impact before it becomes product scope.

### Observe And Capture

- endpoint snapshot adapters;
- DNS query/response observation;
- PCAP replay for fixture proof;
- live pcap/Npcap/libpcap capture where platform proof exists;
- ETW or WFP on Windows where contracts can represent loss, privilege, decode,
  degraded, and unavailable states;
- Android VpnService, Apple Network Extension, Linux nftables/eBPF/TUN, and
  router/import paths only after platform-specific proof.

### Packet Tooling And Network Security Monitoring

The plan includes packet tooling and network security monitoring inputs, but
each tool has a narrow role:

- tcpdump and dumpcap capture bounded fixture PCAPs and manual proof artifacts;
- TShark and Wireshark compare decoder output and support forensic inspection;
- Zeek-style analyzers can convert packets into structured connection, DNS,
  HTTP, TLS, and SSL logs;
- Suricata can produce IDS/IPS-style signature alerts and application-layer
  detections;
- Snort-compatible rules can inform signature compatibility and fixture tests.

These tools do not decide policy by themselves. A Zeek log, Suricata alert, or
Snort-compatible hit can publish analyzer evidence, but enforcement still needs
typed evidence refs, a parent-authored rule or emergency policy state, a policy
decision, adapter capability proof, and audit. Wireshark is inspection tooling,
not a product enforcement mechanism.

### Parse And Aggregate

- Ethernet/IP/TCP/UDP/ICMP parsing where raw packet sources are used;
- DNS parser;
- TLS ClientHello/SNI parser where visible;
- HTTP Host parser for plaintext HTTP;
- QUIC/HTTP3 limitation detector;
- DoH/DoT detector;
- flow/session aggregator;
- byte/count/duration summaries;
- stale, ambiguous, encrypted, unavailable, and IP-only states.

### Classify And Correlate

- domain normalization and public suffix handling;
- domain/category intelligence;
- social/video/game/cloud-gaming classification;
- VPN/proxy/Tor/tunnel classification;
- remote desktop/torrent/download/update classification;
- process/app/browser correlation;
- managed browser confirmation;
- app/game foreground/session confirmation;
- screen summary trigger integration when enabled.

### Detection And AI Audit

Network detection has three input families:

- deterministic classifiers, such as known domain, protocol, category, VPN,
  proxy, tunnel, remote desktop, torrent, update, or cloud-gaming indicators;
- analyzer or signature alerts from Zeek-style summaries, Suricata, or
  Snort-compatible rule output;
- AI detection over structured summaries, labeled fixtures, signature alerts,
  and evidence refs.

AI detection can use anomaly features such as duration, bytes, packet count,
protocol mix, domain/process frequency, new destinations, and repeated failure
patterns. Supervised classifiers may categorize traffic as benign, suspicious,
malicious, update, game, streaming, school/productivity, or unknown where
training and proof exist.

The AI audit service consumes detections and evidence refs, correlates them with
profile, process, browser, app/game, screen, and policy context, then publishes
parent-readable narratives and recommended next actions. AI audit is advisory.
It must not receive raw PCAP, decrypted payloads, private messages, search
queries, or exact content claims that were not proved by another source.

### Intervene

- DNS proxy, block, or redirect;
- Windows Firewall;
- Windows WFP;
- Android VpnService;
- Apple Network Extension;
- Linux nftables/eBPF/TUN;
- managed browser intervention;
- app/game process intervention;
- router/import or manual-required paths.

Control commands should be event-bus requests and adapter-specific actions
should publish results back to the bus. Commands are invalid unless they cite
the evidence bundle, policy decision, adapter capability state, expiry or
rollback state, and audit target.

## Evidence Grades

Every network claim must carry an explicit grade.

```ts
type NetworkEvidenceGrade = 'A_STRONG' | 'B_PROBABLE' | 'C_WEAK_HINT' | 'D_UNUSABLE';
```

Grade A examples:

- Ocentra DNS proxy saw and blocked a domain.
- Managed browser confirmed exact URL.
- Android VpnService saw app/package plus flow/domain.
- Windows WFP saw process plus flow/domain.
- Windows Firewall rule blocked a known target with adapter result proof.

Grade B examples:

- DNS query to a known platform domain.
- TLS SNI visible.
- Flow to a known platform domain/IP.
- Long-lived video/game flow pattern.
- VPN/proxy/DoH/DoT candidate.

Grade C examples:

- IP-only flow.
- Shared CDN.
- Reverse DNS only.
- Router-only log.
- QUIC unknown endpoint.
- Encrypted DNS hiding destination.

Grade D examples:

- Missing device identity.
- Missing timestamp.
- Corrupted capture.
- Ambiguous NAT/router-only attribution.
- No domain, process, or context.

Policy behavior must change by grade:

```text
A_STRONG -> block, warn, ask, or limit if parent rule matches.
B_PROBABLE -> trigger cascade; warn, ask, or strict-category handling only if parent configured it.
C_WEAK_HINT -> observe or trigger more evidence; do not block by default.
D_UNUSABLE -> diagnostics only.
```

## Network-Triggered Cascade

```ts
type NetworkTriggeredCascade = {
  cascadeId: string;
  sourceNetworkEventRef: string;
  createdAt: string;
  triggerKind:
    | 'social_media_candidate'
    | 'video_streaming_candidate'
    | 'gaming_candidate'
    | 'cloud_gaming_candidate'
    | 'vpn_proxy_tunnel_candidate'
    | 'adult_gambling_candidate'
    | 'dns_bypass_candidate'
    | 'blocked_domain_attempt'
    | 'unknown_high_volume'
    | 'policy_restricted_domain';
  evidenceGrade: NetworkEvidenceGrade;
  confidence: number;
  nextChecks: Array<
    | 'check_managed_browser_tabs'
    | 'check_managed_browser_current_url'
    | 'check_unmanaged_browser_process'
    | 'check_foreground_app'
    | 'check_app_game_session'
    | 'check_screen_summary'
    | 'check_dns_policy'
    | 'check_memory_cache'
    | 'run_local_ai'
    | 'ask_parent'
  >;
  privacyMode:
    | 'network_only'
    | 'browser_structured_only'
    | 'managed_browser_capture_if_needed'
    | 'active_window_screen_if_enabled'
    | 'screen_manual_required';
  status: 'planned' | 'running' | 'completed' | 'degraded' | 'manual_required' | 'failed';
};
```

Cascade priority:

```text
1. Managed browser structured evidence.
2. App/game foreground/session evidence.
3. Unmanaged browser/process evidence.
4. Evidence-backed memory/cache.
5. Screen summary only if enabled and needed.
6. Local AI only after typed evidence bundle exists.
```

## Risk Budget And Cascade Reaction

The cascade engine should evaluate network detections against child profile,
age band, household rules, risk budgets, prior occurrences, time of day, and
recent cross-slice evidence before recommending an action.

Supported reaction classes:

- ignore when the event is known-safe, expected, or below threshold;
- monitor when the evidence is weak but worth retaining;
- ask parent when risk is plausible and the next action needs human judgment;
- warn child when the policy allows a child-visible nudge;
- limit or block only when evidence grade, parent rule, and adapter proof allow
  the specific action.

Known malicious signature hits may raise urgency, but signature-only evidence
does not automatically terminate processes or block traffic. Immediate block or
process termination is allowed only when the product has an explicit strict
policy state, a parent-authored rule or documented emergency rule, adapter
capability proof, rollback/unavailable behavior, and audit.

Cross-slice risk budgets are planned but proof-gated. Network risk may affect
screen-time, app/game, browser, ask-parent, or allowance-style budgets only
after those policy contracts and UI states exist. Safe or compliant behavior may
lower risk budget pressure only when the product can explain and audit that
calculation.

Safe or compliant behavior may reduce risk pressure only through a
parent-authored rule with cap, expiry, audit reason, and UI explanation. Network
evidence must not automatically grant extra privileges, allowance, or time
without policy proof.

## Cross-Slice Network Evidence Bundle

```ts
type CrossSliceNetworkEvidenceBundle = {
  bundleId: string;
  createdAt: string;
  childProfileRef: string;
  deviceRef: string;
  networkEvidenceRefs: string[];
  browserEvidenceRefs: string[];
  appGameEvidenceRefs: string[];
  screenEvidenceRefs: string[];
  locationEvidenceRefs: string[];
  deviceStatusEvidenceRefs: string[];
  aiAnalysisRefs: string[];
  memoryRefs: string[];
  policyRefs: string[];
  proofLevel:
    | 'network_only'
    | 'network_plus_process'
    | 'network_plus_managed_browser'
    | 'network_plus_app_game'
    | 'network_plus_screen'
    | 'cross_confirmed';
  claim:
    | 'social_platform_use'
    | 'video_platform_use'
    | 'exact_url_known'
    | 'exact_video_candidate'
    | 'game_activity'
    | 'cloud_gaming_activity'
    | 'vpn_proxy_candidate'
    | 'dns_bypass_candidate'
    | 'unknown';
  confidence: number;
  uncertaintyReasonCodes: string[];
  mustNotClaim: string[];
};
```

`locationEvidenceRefs` and `deviceStatusEvidenceRefs` are optional cross-slice
confirmation inputs only when those feature areas have their own proof. Network
must not infer location from traffic.

Example `mustNotClaim` values:

```text
exact video ID unknown
message content unknown
search query unknown
visible content unknown
process attribution unknown
```

## Core Contract Families

Implementation should add or extend Effect Schema contracts before runtime code
consumes these concepts:

- `NetworkFlowEvidence`
- `NetworkDomainEvidence`
- `NetworkActivityClassification`
- `NetworkEvidenceGrade`
- `NetworkPolicyAction`
- `NetworkActivityEvent`
- `NetworkEventingCapability`
- `NetworkEventDeliveryProjection`
- `NetworkEventTopicNamespace`
- `NetworkAnalyzerAlert`
- `NetworkDetectionResult`
- `NetworkAiAuditReport`
- `NetworkRiskBudgetState`
- `NetworkCascadeReaction`
- `NetworkTriggeredCascade`
- `CrossSliceNetworkEvidenceBundle`
- `NetworkAdapterCapabilityStatus`
- `NetworkProofTier`
- `NetworkActionResult`
- `NetworkInterventionAuditEvent`

Rust protocol parity is required only after the TypeScript contracts and tests
exist and only for shapes the Rust service sends, receives, stores, journals,
or exposes to the portal.

## Platform Plan

Windows:

- P1 PCAP replay and deterministic parser proof.
- P2 hosted CI for contracts and shared parser paths.
- P3 local Windows endpoint/DNS/Npcap proof where available.
- P3 DNS proxy proof.
- P3 Windows Firewall proof.
- P4 real child-device Windows traffic proof.
- P5 WFP observation/intervention proof.

Android:

- P1 contracts and fixture proof.
- P2 emulator proof where useful.
- P4 physical Android VpnService proof.
- P4 app/package correlation proof where possible.
- P5 Device Owner or managed-profile proof where required.

macOS:

- P1 contracts and fixture proof.
- P2 hosted macOS CI where possible.
- P3 local macOS observation proof.
- P3 DNS proxy proof.
- P5 Network Extension or managed-device proof.

iOS:

- P1 contracts.
- P2 simulator/package proof where possible.
- P4 physical iOS app proof.
- P5 entitlement, Network Extension, supervised-device, or MDM proof where used.

Linux:

- P1 PCAP replay.
- P2 Linux CI.
- P3 real Linux pcap/libpcap proof.
- P3 DNS proxy proof.
- P4 distro-specific nftables proof.
- P5 eBPF/cgroup/TUN proof where implemented.

No universal Linux, Android, iOS, or macOS claim exists until platform-specific
proof names the exact host/device, permission state, command/UI action,
artifact path, and known gaps.

## End-State Capabilities

The final network system should support:

- packet capture and PCAP replay;
- bounded PCAP fixture generation with capture-source metadata;
- packet decoder comparison against TShark or Wireshark output;
- Zeek-style structured log generation for connection, DNS, HTTP, TLS, and SSL
  evidence where supported;
- Suricata and Snort-compatible signature alert ingestion where proved;
- typed event topic namespaces and local-first bus routing;
- DNS query/response extraction;
- TLS SNI extraction where visible;
- HTTP Host extraction for plaintext HTTP;
- QUIC/HTTP3 limitation detection;
- DoH/DoT detection;
- VPN/proxy/tunnel detection;
- flow aggregation and bandwidth/session summaries;
- domain normalization and category classification;
- process/app/browser correlation;
- app/game and screen trigger integration;
- network-triggered local AI queue;
- anomaly and supervised detection over summaries and evidence refs;
- parent-readable AI audit reports with cited evidence refs;
- household risk budgets and cascade thresholds;
- DNS proxy/block/redirect;
- Windows Firewall;
- Windows WFP;
- Android VpnService;
- Apple Network Extension;
- Linux nftables/eBPF/TUN;
- router log/import support;
- A/B/C/D evidence grades;
- parent UI evidence drawer;
- policy dry-run and enforcement handoff;
- retention/delete/export;
- event-bus auth, encryption, retention, replay, deletion, and backpressure proof
  where broker-backed routing is used;
- throughput, latency, resource, and high-concurrency benchmarks;
- security, privacy, compliance, deployment, support, and staged rollout proof;
- local-first custody.

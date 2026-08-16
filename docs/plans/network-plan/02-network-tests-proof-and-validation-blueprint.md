# Network Tests, Proof, And Validation Blueprint

<!-- agent-capsule -->

> Agent Capsule
> Plan: `network-plan`
> Doc: `Network Tests, Proof, And Validation Blueprint`
> Kind: test blueprint reference; read only when local expectations route here.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

## Core Test Principle

Network tests must prove parsing and classification, but they must also prove
that Ocentra does not overclaim.

Every network result must answer:

```text
What did we observe?
What can it prove?
What can it not prove?
What evidence grade is it?
What should be triggered next?
What policy action is allowed?
What adapter proof exists?
```

## Test Folder Shape

```text
tests/network/
  unit/
    network_flow_evidence.test.ts
    network_domain_evidence.test.ts
    network_activity_classification.test.ts
    network_evidence_grade.test.ts
    dns_packet_parser.test.ts
    tls_sni_parser.test.ts
    quic_limitation_detector.test.ts
    doh_dot_detector.test.ts
    flow_aggregator.test.ts
    zeek_style_log_generator.test.ts
    signature_alert_contract.test.ts
    domain_normalizer.test.ts
    domain_category_classifier.test.ts
    network_detection_result.test.ts
    network_ai_audit_report.test.ts
    network_risk_budget.test.ts
    cascade_router.test.ts
    cross_slice_bundle.test.ts
    network_policy_action.test.ts
    network_eventing_contract_projection.test.ts
  integration/
    pcap_replay_dns.test.ts
    pcap_replay_tls_sni.test.ts
    pcap_replay_quic_unknown.test.ts
    dns_proxy_policy.test.ts
    network_to_browser_cascade.test.ts
    network_to_app_game_cascade.test.ts
    network_to_screen_cascade.test.ts
    network_to_ai_queue.test.ts
    reusable_eventing_network_cascade_dispatch.test.ts
    reusable_eventing_network_resilience.test.ts
    zeek_summary_to_event_bus.test.ts
    signature_alert_to_detection.test.ts
    detection_to_cascade_control.test.ts
    network_policy_compile.test.ts
    journal_sqlite_network_ingest.test.ts
  security/
    no_exact_url_from_network_only.test.ts
    no_exact_video_from_network_only.test.ts
    no_message_content_from_network.test.ts
    no_search_query_from_encrypted_traffic.test.ts
    no_decrypted_payload_capture.test.ts
    cdn_ambiguity_guard.test.ts
    ip_only_weak_hint_guard.test.ts
    dry_run_no_block.test.ts
    manual_required_no_adapter_call.test.ts
    raw_pcap_not_sent_to_ai.test.ts
    manual_required_event_no_adapter_call.test.ts
    signature_only_no_adapter_call.test.ts
  platform/
    windows_npcap_capture.manual.test.ts
    windows_dns_proxy.manual.test.ts
    windows_firewall_rule.manual.test.ts
    windows_wfp.manual.test.ts
    android_vpnservice.manual.test.ts
    ios_network_extension.manual.test.ts
    macos_network_extension.manual.test.ts
    linux_nftables.manual.test.ts
    linux_ebpf.manual.test.ts
  performance/
    network_event_throughput.bench.ts
    packet_to_detection_latency.bench.ts
    high_concurrency_flows.bench.ts
    agent_resource_impact.bench.ts
  e2e/
    youtube_managed_browser_cascade.e2e.ts
    youtube_unmanaged_browser_cascade.e2e.ts
    roblox_game_cascade.e2e.ts
    steam_launcher_only_cascade.e2e.ts
    vpn_proxy_alert.e2e.ts
    dns_block_policy.e2e.ts
    doh_bypass_candidate.e2e.ts
    cloud_gaming_cascade.e2e.ts
```

Portal/UI tests should live with the existing portal test pattern unless the
repo later centralizes E2E folders.

## Required PCAP Fixtures

The fixture library must cover three families:

- safe baseline traffic: DNS, HTTP, HTTPS/TLS handshakes, QUIC, video
  streaming, gaming, school/productivity, system updates, IoT chatter, LAN
  discovery, multicast, and broadcast;
- suspicious traffic: port scans, DNS tunneling, botnet command-and-control
  candidates, malware download candidates, phishing page candidates, DDoS
  floods, lateral-movement candidates, Tor, VPN, proxy, and DoH/DoT bypass;
- edge traffic: 1000+ concurrent connections, IPv6 and extension-header cases,
  ICMP, unusual or legacy protocols, shared CDN ambiguity, source NAT
  ambiguity, and corrupted or partial captures.

```text
tests/network/fixtures/pcap/
  dns-youtube.pcap
  dns-googlevideo.pcap
  dns-instagram.pcap
  dns-facebook.pcap
  dns-tiktok.pcap
  dns-discord.pcap
  dns-roblox.pcap
  dns-steam.pcap
  dns-xbox-cloud.pcap
  dns-geforce-now.pcap
  tls-sni-youtube.pcap
  tls-sni-instagram.pcap
  quic-googlevideo-ip-only.pcap
  doh-cloudflare.pcap
  dot-quad9.pcap
  tor-bootstrap-candidate.pcap
  vpn-openvpn-candidate.pcap
  remote-desktop-candidate.pcap
  torrent-candidate.pcap
  normal-school-site.pcap
  system-update-windows.pcap
  shared-cdn-cloudflare.pcap
  ambiguous-akamai-fastly.pcap
```

Each fixture must include:

```text
README.md
capture-source.md
expected-flow-evidence.json
expected-domain-evidence.json
expected-zeek-style-logs.json
expected-signature-alerts.json
expected-classification.json
expected-detection-result.json
expected-evidence-grade.json
must-not-claim.json
optional-tshark-comparison.txt
optional-wireshark-inspection.md
```

Fixture captures must be local, bounded, legal to keep in the repo or stored in
an approved artifact path, and free of private child/family traffic.

## Unit Tests

Capture adapter:

- enumerate interfaces without assuming elevated permission;
- report permission, driver, degraded, unavailable, and child-device-scope
  states;
- start and stop bounded captures cleanly;
- rotate PCAP or raw-capture artifacts by quota when that storage mode is used;
- fail closed when capture is not authorized.

DNS parser:

- parse A, AAAA, CNAME, NXDOMAIN, and multiple-answer responses;
- reject malformed packets as invalid;
- extract query name and resolver IP;
- normalize domains without inventing process ownership.

Flow aggregator:

- merge same 5-tuple flows;
- merge reverse direction correctly;
- close flows on idle timeout;
- count bytes and packets;
- preserve protocol;
- keep NAT/router-only source as weak or ambiguous.

Zeek-style log generator:

- produce deterministic connection, DNS, HTTP, TLS, and SSL summary rows from
  replay fixtures where the source evidence supports them;
- preserve unknown, encrypted, missing, and ambiguous states instead of filling
  in guesses;
- match expected rows against approved fixture outputs and decoder comparisons.

Signature alert ingestion:

- ingest Suricata-style alert rows and Snort-compatible rule hits into typed
  analyzer alert records;
- carry signature id, rule source, severity, timestamp, flow ref, and evidence
  custody;
- publish alert events without bypassing detection, policy, or adapter gates;
- prove known false-positive fixtures remain non-enforcing.

Evidence grade:

- DNS query to known domain maps to Grade B unless stronger proof exists;
- DNS proxy blocked domain maps to Grade A;
- managed browser confirmed URL maps to Grade A for exact URL claim;
- IP-only CDN maps to Grade C;
- reverse DNS only maps to Grade C;
- missing device maps to Grade D.

Cascade router:

- social candidate checks managed browser and app/process first;
- video candidate checks browser before screen;
- gaming candidate checks app/game before screen;
- VPN candidate checks app/process/policy before screen;
- CDN ambiguity requires confirmation;
- weak hints do not block by default.

Event bus:

- event type identifiers are unique;
- sync subscribers run and mark the event handled;
- async subscribers run in parallel when awaited;
- request/response events resolve deferred results once;
- events without subscribers queue and later drain when a handler registers;
- queued events expire by TTL;
- max retry and max queue limits drop with audit/log state;
- in-flight duplicate publish is rejected unless republishable;
- target handler markers prevent the wrong adapter from handling a request;
- publisher and subscriber auth/encryption/retention metadata is required when
  the route is broker-backed;
- backpressure, offsets, filtering, retry, and replay behavior is deterministic;
- test bus isolates subscribers and queues between tests.

AI detection and audit:

- use deterministic fixture summaries, labels, analyzer alerts, and evidence
  refs as inputs;
- do not use mocks, fakes, spies, or replaced services;
- output risk level, confidence, evidence refs, uncertainty codes, and
  recommended next checks;
- generate parent-readable audit narratives without unsupported exact content
  claims;
- prove raw PCAP, decrypted payload, private message, search query, and exact
  URL inputs are rejected unless another proved evidence source provides them.

## Integration Tests

Raw capture to summaries:

```text
bounded PCAP fixture
  -> parser extracts tuples, timestamps, bytes, protocol, and interface metadata
  -> Zeek-style summaries produce connection/DNS/HTTP/TLS rows where supported
  -> event bus publishes flow summary events
  -> journal/query store records evidence refs
```

Signature alert to detection:

```text
suspicious PCAP fixture
  -> Suricata/Snort-compatible alert fixture is ingested
  -> analyzer alert event is published
  -> detection service correlates alert with flow summary
  -> detection event carries risk, confidence, uncertainty, and evidence refs
  -> no adapter command is published by the alert alone
```

Detection to cascade to control:

```text
high-risk detection
  -> cascade evaluates child profile, household rule, risk budget, prior events, and proof tier
  -> policy decision produces ask/warn/monitor/limit/block/manual-required
  -> control command is published only when adapter proof allows it
  -> adapter result or manual-required state is audited
```

Event bus resilience:

```text
publisher flood, subscriber delay, queue overflow, broker restart if broker-backed
  -> backpressure, retry, TTL, filtering, offsets, replay, and dropped-event audit are deterministic
  -> no policy or adapter command is lost silently
```

YouTube managed browser:

```text
PCAP DNS youtube
network classification video_candidate
managed browser URL confirms exact watch URL
local AI URL/video analysis is queued only from exact URL evidence
bundle proofLevel network_plus_managed_browser
policy can decide exact URL/video if parent rule matches
```

YouTube unmanaged browser:

```text
PCAP DNS youtube
Chrome process found but not managed
screen summary requested only if enabled
bundle says unmanaged_browser_video_candidate
policy can ask parent or require managed browser
no exact URL claim unless another evidence path proves it
```

Roblox game:

```text
PCAP DNS roblox
foreground Roblox process or app/game session exists
bundle proofLevel network_plus_app_game
policy can count game time from app/game evidence, not network alone
```

Steam launcher only:

```text
PCAP DNS steam
Steam launcher foreground
no child game process/session
bundle claim launcher_only
no game budget unless parent configured launcher budget
```

VPN/proxy:

```text
PCAP VPN/DoH candidate
app/process check requested
policy bypass rule considered
urgent parent alert candidate allowed
block only if adapter proof exists
```

## Merge-Blocking Negative Tests

- network-only YouTube evidence cannot claim exact video;
- googlevideo CDN cannot claim educational or entertainment content;
- Instagram CDN cannot claim reel or post;
- Discord domain cannot claim message content;
- DoH candidate cannot claim visited domain;
- VPN tunnel cannot claim hidden destination;
- IP-only flow cannot claim a domain;
- router-only log cannot claim local process;
- hidden SNI cannot be invented;
- AI cannot receive raw PCAP;
- AI cannot receive decrypted payload;
- Grade C evidence cannot block by default;
- signature-only analyzer alert cannot block by default;
- `manual_required` cannot call an adapter;
- dry-run cannot block traffic;
- event-bus delivery cannot bypass policy or adapter capability checks.

## Platform Proof Packs

Each platform proof must output:

```text
output/network-plan-proof/<platform>/<capability>/
  00-run-metadata.json
  01-device-metadata.json
  02-permission-or-driver-state.json
  03-input-scenario.md
  04-raw-capture-or-log-ref.md
  05-network-flow-evidence.json
  06-domain-evidence.json
  07-classification.json
  08-cascade-plan.json
  09-policy-decision.json
  10-action-result.json
  11-ui-screenshots/
  12-validation-commands.log
  13-known-gaps.md
```

Windows proof must distinguish:

- endpoint snapshot proof;
- DNS evidence proof;
- Npcap/pcap proof where used;
- DNS proxy proof;
- Windows Firewall proof;
- WFP proof;
- process correlation proved versus unknown.

Android proof must distinguish:

- VpnService consent;
- virtual interface active;
- outgoing packet observation;
- DNS/domain classification;
- domain block proof;
- package/app correlation proof where possible;
- Device Owner or lockdown proof where claimed.

macOS/iOS proof must distinguish:

- entitlement state;
- permission/consent state;
- packet tunnel/content filter/DNS proxy proof where available;
- supervised or managed-device requirements where claimed.

Linux proof must distinguish:

- distro and kernel assumptions;
- libpcap proof;
- DNS proxy proof;
- nftables proof;
- eBPF/cgroup/TUN proof where implemented.

## Performance And Proof Metrics

Every detection and control proof should record:

- fixture count and scenario type;
- packet count, flow count, and event count;
- packet-to-summary latency;
- packet-to-detection latency;
- detection-to-cascade latency;
- cascade-to-command latency when a command is allowed;
- CPU, memory, disk, queue depth, and dropped-event counts;
- precision, recall, false positives, false negatives, and confusion matrix
  where labeled fixtures exist;
- parent audit clarity evaluation for AI narratives;
- compliance statement proving unsupported claims were not made.

High-risk response goals may use sub-second targets only when the target adapter,
proof tier, and configured policy explicitly allow them. The test must record
when a path is dry-run, manual-required, unsupported, unavailable, or degraded.

Proof dossiers should preserve full event histories from observation through
audit so a reviewer can trace evidence id, topic, handler, policy decision,
adapter result, and UI proof without relying on logs alone.

## Broker Delivery Proof

Broker-backed routes must declare delivery semantics as one of:

- at-least-once;
- at-most-once;
- local idempotency/queue duplicate-safety only, unless a live broker proof
  establishes stronger semantics.

Tests must prove duplicate detection, idempotency keys, replay behavior,
dropped-event audit, and that no adapter action executes twice from duplicate
delivery. Do not claim generic exactly-once delivery unless the exact broker,
configuration, producer, consumer, offset, transaction, and idempotency proof
supports it.

## Playwright UI Tests

Required parent UI surfaces:

- network dashboard;
- current activity;
- domain timeline;
- flow evidence drawer;
- evidence grade badge;
- cascade plan view;
- cross-slice evidence bundle view;
- AI audit narrative view;
- risk budget and threshold view;
- event history and proof metric view;
- policy decision view;
- block event view;
- manual-required capability state;
- platform matrix;
- network limitation explanation.

UI must show these states when applicable:

- exact URL unknown;
- exact video unknown;
- message content unknown;
- process unknown;
- domain from DNS;
- domain from SNI;
- IP-only;
- CDN ambiguous;
- VPN/proxy candidate;
- requires confirmation;
- adapter unavailable;
- dry-run only;
- manual-required.

## Validation Gates

Focused docs-only validation:

```powershell
git diff --check
```

Contract implementation gate:

```powershell
npm run test:local
```

PR-ready gate unless scoped otherwise:

```powershell
npm run validate
```

Platform proof gates must also include the exact manual commands, screenshots,
device metadata, and known-gap files named above.

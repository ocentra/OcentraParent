<!-- agent-capsule -->

> Agent Capsule
> Doc: Network Control Capability Guide
> Kind: repo documentation; read only when routed by root AGENTS, docs indexes, feature route, source router, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# Network Control Capability Guide

Status: product capability guide for future portal UI and parent guidance.

This document explains what Ocentra Parent can and cannot know or control from
network and domain evidence. It is meant to feed later Policy and Network UI
work, especially parent-facing guide sections where a parent chooses between
observation, domain controls, network enforcement, local-network exceptions,
bandwidth and time budgets, suspicious-indicator reports, and stricter platform
setup.

This is not a moral policy document. The product should expose real capability
boundaries and let the parent choose the household rule posture. The important
engineering rule is that the UI must not imply exact URL, content, app intent, or
network blocking that the child-device agent cannot prove through the selected
platform adapter.

## Core Terms

### Network Flow Evidence

Network flow evidence is metadata about communication observed by the
child-device agent or a platform network adapter.

Possible fields include:

- local IP and port;
- remote IP and port;
- transport protocol;
- TCP state;
- process id and process identity where available;
- DNS/domain attribution where available;
- interface, route, LAN/WAN, VPN, proxy, or tunnel indicators;
- timestamps, connection counts, duration, and bytes sent/received where the
  adapter can prove them;
- evidence id, source id, capability state, custody state, and retention state.

Network flow evidence is not decrypted content. It does not prove page text,
chat content, search terms, active browser tabs, full HTTPS URLs, or user intent.

### Domain Attribution

Domain attribution means the system can associate a flow or summary with a
domain candidate or known domain. The source may be observed DNS traffic, a DNS
client cache entry, reverse DNS, a managed resolver, a parent rule target, or a
managed browser URL that was deliberately joined to a network observation.

The source and confidence matter. A DNS cache entry is weaker than an observed
resolver event. A host-level DNS event is weaker than a per-process resolver
event. A parent-entered domain rule is not evidence that the child visited that
domain.

### Process Attribution

Process attribution means the system can connect a network endpoint or flow to a
process identity, such as process id, executable reference, publisher,
signature/hash reference, and user/session where available.

Process attribution can be strong on some desktop adapters and weak or
unavailable on routers, VPNs, DNS-only controls, and some mobile platforms.
Process attribution does not prove which browser tab, app screen, or user action
caused the flow.

### Network Control

Network control is any action that changes network behavior:

- allow, block, or rate-limit traffic by process, IP, port, protocol, domain, or
  category where supported;
- force or configure DNS, proxy, VPN, firewall, WFP, packet filter, or router
  policy;
- terminate a process after a network policy decision;
- warn, ask parent, or report instead of blocking;
- keep local-network exceptions for printers, LAN pairing, parental devices, and
  trusted home services.

Network control must run through the child-device agent or an approved platform
adapter. Portal UI is an authoring and reporting surface only.

### Exact URL Evidence

Exact URL evidence means URL path, query, active tab URL, browser page title,
request URL, or download source URL. Network/domain evidence alone does not prove
this. Exact URL evidence belongs to a managed browser boundary or another
explicit platform/browser integration.

### Local Network Exception

A local network exception is a rule that permits or observes traffic inside a
trusted local scope even when broader internet controls are strict.

Examples:

- loopback service ports used by the child-device agent;
- LAN pairing ports between parent controller and child agent;
- local printer, NAS, DNS resolver, router, or media device;
- school/home subnet where the parent explicitly allows discovery;
- multicast and broadcast protocols that are needed for device discovery.

Local exceptions need explicit scope. A broad `private-network` allow rule can
hide unwanted lateral movement if it is not auditable.

## The Main Capability Truth

Network controls are useful for domain, destination, process, volume, protocol,
and suspicious-indicator policy. They are not the same as browser-native URL
control.

Ocentra can honestly claim network visibility only for metadata the child-device
agent observes, normalizes, journals, and exposes through typed read models. It
can honestly claim network enforcement only after an adapter changes network
behavior on a real target platform and records an auditable result.

Network/domain controls can support:

- domain allow/block rules;
- IP, port, protocol, and process rules;
- VPN/proxy/tunnel indicators;
- LAN versus internet classification;
- bandwidth and connection-count summaries;
- unusual new destination reporting;
- local AI/policy digests with evidence ids;
- app/browser correlation when the flow also has process or managed-browser
  evidence.

They cannot reliably prove:

- exact URL path or query in normal HTTPS;
- page title or active tab;
- page body, chat content, search terms, form values, cookies, tokens, or
  credentials;
- specific video or post within a CDN-backed service;
- which tab caused a network request;
- per-process DNS attribution when the source is host-level DNS cache only;
- child intent or safety classification without a separate policy/AI contract.

Unknown, encrypted, ambiguous, stale, and unsupported states are product states.
They must not be hidden or upgraded into stronger claims.

## Capability Matrix

| Capability                        | What can be possible                                                 | Required layer                                     | Important limit                                                           |
| --------------------------------- | -------------------------------------------------------------------- | -------------------------------------------------- | ------------------------------------------------------------------------- |
| Detect remote IP                  | Yes on most endpoint, firewall, VPN, router, or flow adapters        | Endpoint/flow observation                          | IP alone may be CDN/shared, NATed, private, or anycast.                   |
| Detect remote port                | Yes where endpoint/packet metadata is exposed                        | Endpoint/flow observation                          | Port does not prove application semantics.                                |
| Detect protocol                   | TCP/UDP/IP protocol usually; app protocol sometimes                  | Endpoint/flow observation, DPI if approved         | QUIC over UDP/443 can hide higher-level HTTP details.                     |
| Attribute process                 | Often on Windows endpoint adapters; varies elsewhere                 | OS endpoint/process adapter                        | Router/DNS-only data usually cannot identify the local process.           |
| Attribute domain                  | Sometimes through DNS events/cache or managed resolver               | DNS adapter, resolver, browser join, proxy         | DoH, DoT, VPNs, ECH, CDNs, and cache ambiguity reduce confidence.         |
| Attribute exact URL               | No from network alone                                                | Managed browser, explicit URL filter, proxy        | Normal HTTPS hides path/query from passive network observers.             |
| Block IP                          | Often possible                                                       | Firewall, WFP, VPN, packet filter, router          | Shared IP/CDN can overblock unrelated services.                           |
| Block domain                      | Possible through DNS, proxy, browser, VPN, WFP, router               | Domain resolver/control layer                      | DoH/DoT, hard-coded IPs, CDNs, and cached connections can bypass or blur. |
| Block exact URL                   | Possible only with browser/proxy/URL filter proof                    | Managed browser, explicit URL filter, HTTP proxy   | Not a generic firewall claim. HTTPS interception is out of current scope. |
| Block process network             | Possible on some OS adapters                                         | WFP/firewall/app control/VPN with app binding      | Requires platform proof and robust process identity.                      |
| Block protocol or port            | Often possible                                                       | Firewall, WFP, packet filter, router               | Can break legitimate services and may not classify encrypted app traffic. |
| Bandwidth budget                  | Possible where byte counters are reliable                            | Flow counters, firewall/VPN/router counters        | DNS-only and endpoint snapshots may not provide byte counts.              |
| Time budget                       | Possible as flow-active, process-active, or foreground-app time      | Flow/process/browser/session timers                | Network-active time is not the same as active child attention.            |
| Detect VPN/proxy/tunnel           | Possible as indicator                                                | Adapter/interface/route/process/flow heuristics    | Indicator does not prove the tunneled destination or content.             |
| Force all traffic through adapter | Possible on some platforms                                           | Always-on VPN, WFP, router, MDM/profile            | Requires privileges, setup, and bypass proof.                             |
| Router-level control              | Possible in managed router scenarios                                 | Router API/DNS/firewall                            | Cannot usually see local process or active tab.                           |
| Cloud relay control               | Possible only for Ocentra protocol traffic or parent-authorized path | Relay/control-plane contract                       | Cannot control arbitrary child internet traffic by itself.                |
| LAN exception                     | Possible                                                             | Route/interface/subnet/service policy              | Too broad an exception can hide unwanted local traffic.                   |
| Suspicious indicator report       | Possible                                                             | Stored network digest plus deterministic/AI labels | Indicator must cite evidence and keep uncertainty.                        |
| Audit enforcement result          | Required for strict actions                                          | Journaled policy decision plus adapter result      | A rule value alone is not proof that traffic was blocked.                 |

## Network Visibility: What Is Possible

### DNS, Domain, IP, Port, And Protocol

The child-device agent can use layered evidence:

- endpoint snapshots for local and remote IP/port/protocol;
- TCP and UDP owner PID tables where supported;
- DNS client cache or observed resolver events;
- managed resolver logs where Ocentra controls the resolver path;
- firewall, VPN, WFP, packet filter, router, or proxy observations;
- managed browser URL evidence when there is an explicit join to flow evidence.

The evidence contract should keep these states separate:

- `domain-known`;
- `domain-candidate`;
- `domain-ambiguous`;
- `ip-only`;
- `dns-unavailable`;
- `dns-stale`;
- `encrypted-content-unavailable`;
- `process-attributed`;
- `process-unknown`;
- `adapter-unavailable`;
- `adapter-permission-required`.

The portal should show the state rather than flattening it into one "site"
column.

### Process And App Attribution

Process attribution is valuable for answering:

- Which process opened network connections?
- Which destination did this process contact?
- Did an unmanaged browser or unknown app create traffic?
- Did a known app suddenly contact a new destination?
- Did a child-controlled process use VPN/proxy/tunnel-like behavior?

Process attribution is not enough for:

- exact browser URL;
- active tab;
- page title;
- user intent;
- category enforcement without a parent-authored rule.

For browser traffic, app/browser correlation should prefer:

1. managed browser evidence for exact URL/tab state;
2. process-attributed network flow for lower-layer destination evidence;
3. DNS/domain candidates with confidence and ambiguity;
4. IP-only summaries when no domain evidence exists.

### LAN Versus Internet

LAN classification can use IP ranges, route/interface hints, gateway state, DNS
suffixes, mDNS/SSDP/multicast indicators, and known local service registrations.

The UI should distinguish:

- loopback;
- local agent service;
- LAN parent controller;
- local subnet;
- private address through VPN;
- public internet;
- unknown route;
- captive portal or public Wi-Fi;
- router/cloud relay metadata.

LAN allowance is not automatically safe. Parent-facing controls should allow
specific local service exceptions before broad subnet exceptions.

### Bandwidth And Time Budgets

Network budgets can be useful, but they need precise definitions:

- flow byte budget: bytes sent/received by matching flows;
- connection-count budget: number of connections or new destinations;
- network-active time: time a matching flow was active;
- foreground-correlated time: network-active time while a process/browser/app was
  foreground;
- schedule budget: whether a rule is active during a local time window.

Budget enforcement must say what it counts. A video CDN byte budget is not the
same as screen time. A background updater can consume bandwidth while the child
is not actively using the app.

### Suspicious Indicators

Suspicious indicators are report inputs, not hidden enforcement by themselves.

Useful indicators include:

- new destination for child/device/process;
- high-volume unknown process;
- repeated connection failures;
- DNS mismatch or excessive DNS churn;
- DNS unavailable while traffic continues;
- direct IP traffic to public internet;
- VPN, proxy, Tor-like, tunnel, MASQUE-like, or unknown adapter indicator;
- unusual port or protocol for a child device;
- domain/IP reputation category when the category source is explicit;
- LAN scan-like pattern;
- traffic from an unmanaged browser or unsupported app;
- traffic during blocked schedule or after budget exhaustion.

Each indicator should carry evidence ids, source, confidence, and whether it was
deterministic or AI-assisted. AI cannot invent flows, destinations, bytes,
process ownership, or decrypted content.

## Network Control Layers

### DNS And Managed Resolver

DNS control can allow, block, redirect, or classify domain lookups.

Strengths:

- parent-friendly domain rules;
- broad device or profile coverage when DNS path is controlled;
- useful reporting for domains and categories;
- works even when the app is not a browser if it uses the managed resolver.

Limits:

- DNS answers can be cached before policy changes;
- several domains can resolve to one IP, and one domain can resolve to many IPs;
- apps can use hard-coded IPs;
- apps can use DNS-over-HTTPS or DNS-over-TLS unless the platform routes or
  blocks those paths;
- DNS control does not see HTTPS path/query;
- DNS control usually cannot identify a local process unless an endpoint adapter
  joins it.

### Firewall, WFP, And Packet Filter

Host firewall, Windows Filtering Platform, Linux packet filters, and similar
platform adapters can control traffic by IP, port, protocol, interface, app, user
or process where the platform exposes those fields.

Strengths:

- strong local enforcement on supported platforms;
- can block non-browser apps;
- can enforce LAN/internet route and port rules;
- can produce auditable adapter results.

Limits:

- may need admin rights, service installation, signed drivers, system
  extensions, MDM, or entitlement approval;
- can overblock shared infrastructure;
- does not decrypt HTTPS;
- may be bypassed by other privileged network layers if not properly installed;
- must be performance-tested and rollback-capable.

### VPN Or Tunnel Adapter

A VPN-style adapter can route traffic through a local or remote policy engine.

Strengths:

- can cover many apps on mobile and desktop;
- can centralize DNS/domain policy;
- can implement always-on or lockdown modes on platforms that support them;
- can provide flow counters and route indicators.

Limits:

- requires visible setup and platform permission;
- can conflict with school/work VPNs;
- may not reveal original process or exact URL;
- must not export child activity to Ocentra-hosted infrastructure by default;
- always-on/lockdown claims require platform proof.

### Proxy

A proxy can mediate HTTP and HTTPS CONNECT destinations. It may support richer
domain/category rules than a firewall.

Strengths:

- explicit domain allow/block;
- central policy for apps that honor proxy settings;
- potential full URL control for plain HTTP or explicitly managed browser/proxy
  integrations.

Limits:

- many apps ignore system proxy settings;
- HTTPS path/query remains hidden without TLS interception, which is out of
  current Ocentra Parent scope;
- certificates, trust, and privacy risk are high if interception is introduced
  later;
- QUIC and direct sockets can bypass unless separately controlled.

### Router

Router-level control can cover a device even when the child-device agent is not
active.

Strengths:

- home-wide DNS, IP, port, or device rules;
- useful for IoT and guest devices;
- can enforce local network access and internet schedules.

Limits:

- usually cannot see local process, active tab, foreground app, or child profile;
- device identity can be ambiguous with MAC randomization, NAT, VPN, or shared
  devices;
- remote/off-home traffic is not covered;
- router vendor APIs vary widely;
- router proof is separate from Windows endpoint proof.

### Cloud Relay

Cloud relay can route Ocentra protocol messages between parent and child devices
when local/LAN access is unavailable. It is not a generic network firewall.

Cloud relay may help with:

- remote rule update delivery;
- parent approval requests;
- report query routing;
- device reachability metadata;
- stateless report compilation from parent-authorized sources.

Cloud relay must not claim:

- control of arbitrary child internet traffic;
- storage of child network evidence by default;
- exact network observations unless the child agent uploaded parent-authorized
  typed summaries under an explicit custody setting.

## Modern Network Limits

### HTTPS, DoH, QUIC, ECH, And CDNs

Normal HTTPS hides URL path, query, request bodies, and response bodies from a
passive network observer. DNS-over-HTTPS carries DNS queries through HTTPS.
QUIC is UDP-based and encrypted after its protected handshake. Encrypted Client
Hello encrypts more of the TLS ClientHello. CDN infrastructure can put many
unrelated services behind shared IPs and hostnames.

Practical product impact:

- exact URL rules require managed browser, explicit URL filter, or proxy proof;
- domain rules must tolerate ambiguous or unavailable domain evidence;
- IP block rules should warn about shared infrastructure risk;
- DoH/DoT policy needs a specific resolver-control or network-blocking posture;
- QUIC blocking can force TCP fallback for some browsers/sites, but that is a
  compatibility decision and not a content-inspection feature;
- ECH reduces SNI-based classification, so DNS/resolver or browser evidence
  becomes more important;
- CDN-backed sites need domain/category evidence, not IP-only overclaiming.

### VPN, Proxy, Tor, And Tunnels

VPNs, proxies, Tor-like tools, and tunnels can hide the final destination from
the local network observer. Ocentra can still record the tunnel app/process,
server IP/domain where visible, adapter state, protocol, port, duration, and
volume where available.

The UI should say "VPN/proxy/tunnel indicator" or "traffic appears tunneled"
unless the adapter proves a stronger claim. It should not claim to know the
inside destination or content of a tunnel.

### Private Relay And Platform Privacy Features

Platform privacy relays, browser secure DNS settings, MAC address randomization,
IPv6 privacy addresses, and private browsing modes can reduce attribution.

These are not errors by themselves. They are capability states that can interact
with parent policy:

- allow;
- observe;
- warn;
- ask parent;
- require managed browser/network path;
- block only where a platform adapter proves support.

## Domain Blocking Versus Exact URL Blocking

Domain blocking can answer:

- allow or block `example.invalid`;
- allow or block subdomains;
- block a category when the category source is explicit;
- block a known resolver, VPN gateway, proxy, or tunnel host;
- block an app from making network connections to a domain/IP/port.

Exact URL blocking can answer:

- allow or block a specific page path;
- handle query strings;
- classify a specific video page;
- block a browser download source URL.

Exact URL blocking is not a generic network claim. It requires a managed browser,
explicit URL filter, HTTP proxy path, or another platform-specific proof. When
only network/domain evidence exists, the UI should offer domain rules and mark
exact URL rules as requiring a managed browser or explicit URL-filter capability.

## Reports, Custody, Retention, And Audit

Network reports should be derived from stored evidence, not portal state or AI
guesses.

Recommended report surfaces:

- recent flows;
- top processes;
- top domains;
- top IPs;
- top ports/protocols;
- new destinations;
- bandwidth summaries;
- time-window summaries;
- VPN/proxy/tunnel indicators;
- LAN exceptions used;
- blocked/allowed/warned/asked decisions;
- enforcement failures and unavailable states;
- source/custody labels.

Custody labels should distinguish:

- live local child agent;
- live LAN child agent;
- child-device encrypted journal;
- child-device SQLite query store;
- parent-device cache;
- parent-owned export;
- parent-authorized relay;
- Ocentra-hosted non-activity metadata;
- unavailable.

Retention should be explicit:

- raw flow evidence retention;
- domain summary retention;
- bandwidth summary retention;
- policy/audit retention;
- exported report retention;
- deletion and expiry behavior;
- whether redacted summaries survive raw evidence deletion.

Audit is required for strict actions:

- policy version;
- rule id;
- evidence id;
- adapter id and capability state;
- action requested;
- action result;
- rollback or expiry state;
- parent approval or override reference;
- custody label;
- timestamp and source.

## Platform Capability Notes

### Windows

Windows is the strongest first target for Ocentra Parent network control.

Likely capability layers:

- IP Helper endpoint snapshots for TCP/UDP owner PID state;
- DNS client cache or DNS event observation;
- ETW for network event streams if loss/decode/privilege states are typed;
- Windows Firewall for IP, port, protocol, service, and application rules;
- Windows Filtering Platform for future observation and enforcement adapters;
- process/window evidence for app/browser correlation;
- managed browser evidence for exact URL/tab state.

Windows caveats:

- WFP, ETW, firewall, and service paths may require admin rights and careful
  installer/service setup;
- broad domain/network blocking is still manual-required until a real adapter
  proof exists;
- endpoint snapshots can miss short-lived flows and may not provide bytes;
- DNS cache is host-level unless a stronger source proves per-process
  attribution;
- product claims should follow real host proof, not contract presence.

### macOS

macOS requires separate proof.

Possible layers:

- Network Extension content filter or app proxy paths where entitled and
  approved;
- configuration profiles or MDM for stronger managed-device cases;
- process observation with platform permissions;
- browser-managed evidence for exact URL/tab state;
- DNS/proxy/VPN settings when allowed by setup.

Caveats:

- Network Extension, System Extension, TCC, MDM, signing, notarization, and App
  Store review affect what is shippable;
- do not assume Windows WFP or process-control semantics map to macOS;
- consumer child-agent claims must stay behind Apple-approved capabilities.

### Linux

Linux can support strong network control in managed distributions, but parity
depends on distro, desktop, privilege model, packaging, and firewall stack.

Possible layers:

- nftables/netfilter, iptables compatibility, or distro firewall managers;
- process and socket inspection through procfs/netlink where permitted;
- DNS/proxy/VPN controls;
- managed browser evidence for exact URL/tab state.

Caveats:

- desktop foreground and app identity vary;
- privilege and service installation differ by distro;
- router/server Linux is not the same as a child desktop agent;
- claims need distro-specific validation.

### Android

Android network control depends heavily on app role.

Possible layers:

- VpnService for an app-owned VPN path;
- always-on VPN and lockdown when user, device owner, or profile owner setup
  permits;
- DevicePolicyManager controls for device-owner or profile-owner deployments;
- managed DNS, app restrictions, or package lifecycle controls where the device
  management posture permits;
- Usage Stats, accessibility, or browser/app-specific integrations for
  foreground/app context when explicitly approved.

Limits:

- a normal app cannot broadly firewall every other app without a VPN-style or
  device-management boundary;
- exact URL in arbitrary mobile browsers is not generally reliable from network
  metadata;
- per-app VPN and always-on behavior need device proof and UX setup;
- school/work VPN conflicts need explicit handling.

### iOS And iPadOS

iOS and iPadOS are constrained and require Apple-approved paths.

Possible layers:

- Screen Time frameworks: Family Controls, Managed Settings, Device Activity;
- web domain shielding through managed settings where allowed;
- Network Extension content filter or URL filter paths where entitlement and
  deployment permit;
- MDM/supervision for stronger managed-device content filtering;
- app/category/domain tokens rather than raw browser history in Screen Time
  flows.

Limits:

- third-party apps do not get general packet inspection or arbitrary exact URL
  telemetry from other apps;
- entitlements, review, supervision, and deployment model determine capability;
- web domain shielding is not the same as full browser history capture;
- parent iOS app claims and child iOS agent claims must stay separate.

### Router And Home Network

Router control can be valuable later, but it is a different product surface from
endpoint control.

Possible layers:

- DHCP/device identity;
- DNS resolver policy;
- firewall/IP/port rules;
- bandwidth/time schedules;
- LAN device grouping;
- parent-owned router API integration.

Limits:

- weak process/child attribution;
- vendor-specific APIs;
- MAC randomization and device sharing;
- no off-home coverage;
- no exact URL path/query without explicit proxy/filter integration.

### Cloud Relay

Cloud relay is a remote access and routing feature, not child traffic
inspection. It should be scoped to typed Ocentra messages, parent-authorized
report reads, approvals, and device reachability.

It must preserve the repo's local-first rule: Ocentra-hosted infrastructure does
not store child network activity by default.

## Policy Modes To Represent Later In UI

### Observe Network Activity

What it means:

- record and summarize network metadata;
- show process, destination, protocol, port, DNS/domain, volume, and capability
  states where available;
- classify suspicious indicators in report-only mode.

Does not provide:

- network blocking;
- exact URL rules;
- decrypted content;
- guaranteed process attribution on every platform.

### Domain Rules

What it means:

- allow, warn, ask, limit, or block domains and subdomains where a domain-control
  layer is available;
- use managed browser evidence for exact domain/origin when available;
- use DNS/network attribution with confidence when browser evidence is absent.

Does not guarantee:

- full URL path/query control;
- CDN-safe IP blocking;
- process attribution from router or DNS-only data.

### IP, Port, And Protocol Rules

What it means:

- allow or block remote IPs, CIDRs, local/remote ports, and transport protocols
  through firewall/WFP/VPN/router/packet-filter adapters.

Risk:

- can break legitimate infrastructure;
- can miss app semantics;
- can be too broad for CDN-backed services.

### VPN, Proxy, And Tunnel Handling

What it means:

- observe, warn, ask, block, or require approval for VPN/proxy/tunnel indicators;
- optionally require managed network path where supported.

Does not mean:

- Ocentra knows tunneled destinations or content.

### Bandwidth And Time Budgets

What it means:

- apply budgets to flow bytes, connection counts, network-active duration, or
  foreground-correlated duration according to the selected evidence source.

Must show:

- counted evidence type;
- reset window;
- degraded behavior when counters are missing;
- whether background traffic counts.

### Local Network Exceptions

What it means:

- allow specific local services, subnets, protocols, or Ocentra pairing traffic
  while internet rules remain strict.

Must show:

- exact exception scope;
- last used evidence;
- risk of broad private-network allow rules.

### Strict Network Enforcement

What it means:

- child-device agent applies platform network control after a typed policy
  decision references stored evidence or a parent-authored target.

Requires:

- adapter capability proof;
- policy decision;
- audit event;
- rollback or expiry path;
- visible unsupported/degraded state.

## Current Ocentra Parent Posture

Current repository direction already models this split:

- Network flow evidence is local-first metadata, not decrypted content.
- Browser URL/tab evidence remains the exact URL source.
- Policy can consume stored network summaries and unusual indicators only after
  they are journaled and queryable.
- Enforcement is scaffold/protocol/audit work unless a real platform adapter
  proof exists.
- Windows is first, but broad network/domain blocking remains manual-required
  until real OS adapter proof exists.
- LAN and cloud relay are typed control/report paths, not default hosted child
  evidence stores.
- Portal UI authors rules and shows capability states. It does not run capture,
  policy evaluation, enforcement, timers, OS commands, or scripts.

Relevant local docs:

- [`docs/architecture/network-flow-evidence-capture.md`](../../../architecture/network-flow-evidence-capture.md)
- [`docs/expectations/network-flow-evidence.md`](../../../expectations/network-flow-evidence.md)
- [`docs/expectations/policy.md`](../../../expectations/policy.md)
- [`docs/expectations/enforcement.md`](../../../expectations/enforcement.md)
- [`docs/expectations/data-custody.md`](../../../expectations/data-custody.md)
- [`docs/product-roadmap.md`](../../../product-roadmap.md)
- [`docs/managed-unmanaged-browser.md`](../../../plans/browser-plan/workpacks/managed-unmanaged-browser.md)

## Future UI Rules

The Network UI should eventually make these distinctions visible:

- Show exact URL controls only when managed browser or explicit URL-filter
  capability is available.
- Show domain rules as domain evidence, not exact URL evidence.
- Show IP-only, domain-ambiguous, DNS-unavailable, encrypted-content-unavailable,
  process-unknown, adapter-unavailable, and stale states directly.
- Keep process, domain, IP, port, protocol, VPN/proxy/tunnel, LAN exception, and
  bandwidth/time budget rules as separate target types.
- Keep LAN exceptions visible beside strict rules.
- Show capability status beside each strict action: ready, unsupported,
  permission-required, adapter-missing, proof-missing, degraded, monitor-only,
  manual-required, or unavailable.
- Require proof for enforcement claims: parent rule, evidence reference, policy
  decision, adapter action, adapter result, audit row, and rollback/expiry state.
- Keep custody labels close to reports, AI summaries, exports, and parent
  assistant surfaces.

The parent should be able to choose policy posture with informed tradeoffs:

- observe only;
- domain rules;
- IP/port/protocol rules;
- VPN/proxy/tunnel handling;
- bandwidth and network-active time budgets;
- local network exceptions;
- strict network enforcement where proven;
- managed browser for exact URL rules;
- router or cloud relay options only where separately configured and proven.

## Source References

External capability references:

- [Windows Filtering Platform](https://learn.microsoft.com/en-us/windows/win32/fwp/about-windows-filtering-platform)
- [Windows Firewall overview](https://learn.microsoft.com/en-us/windows/security/operating-system-security/network-security/windows-firewall/)
- [GetExtendedTcpTable](https://learn.microsoft.com/en-us/windows/win32/api/iphlpapi/nf-iphlpapi-getextendedtcptable)
- [GetExtendedUdpTable](https://learn.microsoft.com/en-us/windows/win32/api/iphlpapi/nf-iphlpapi-getextendedudptable)
- [Get-DnsClientCache](https://learn.microsoft.com/en-us/powershell/module/dnsclient/get-dnsclientcache?view=windowsserver2025-ps)
- [RFC 8484: DNS Queries over HTTPS](https://www.rfc-editor.org/info/rfc8484)
- [RFC 9000: QUIC](https://www.rfc-editor.org/info/rfc9000)
- [RFC 9849: TLS Encrypted Client Hello](https://www.rfc-editor.org/info/rfc9849)
- [Android VpnService](https://developer.android.com/reference/android/net/VpnService)
- [Android DevicePolicyManager](https://developer.android.com/reference/android/app/admin/DevicePolicyManager)
- [Apple Screen Time frameworks](https://developer.apple.com/documentation/ScreenTimeAPIDocumentation)
- [Apple Network Extension content filter providers](https://developer.apple.com/documentation/networkextension/content-filter-providers)
- [Apple content filtering deployment guide](https://support.apple.com/guide/deployment/filter-content-for-apple-devices-dep1129ff8d2/web)
- [nftables project documentation](https://www.nftables.org/documentation/)

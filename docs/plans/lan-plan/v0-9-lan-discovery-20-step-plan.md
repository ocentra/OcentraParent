# V0.9 LAN Discovery 20-Step Plan

<!-- agent-capsule -->

> Agent Capsule
> Plan: `lan-plan`
> Doc: `V0.9 LAN Discovery 20-Step Plan`
> Kind: plan reference document; read only when routed by AGENTS, DOC_INDEX, or workpack.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

This plan turns the LAN discovery notes into a concrete Ocentra Parent subsystem
plan. It keeps LAN discovery separate from browser control, app blocking, screen
time, AI classification, and remote desktop. The rule for this slice is:

```text
LAN scan discovers. Child agent confirms. Parent assigns.
```

This is a plan document only. It does not claim production household LAN
readiness, mobile parity, remote/cloud relay, broad OS blocking, or physical
two-device proof until the matching artifacts exist.

Companion requirement doc: [V0.9 LAN Discovery Test Blueprint](v0-9-lan-discovery-test-blueprint.md).
Follow this document for the build sequence and the companion blueprint for
required tests, fixtures, proof gates, and quality bars.

## Product Boundary

- Owning feature: Remote, LAN, and mobile platforms.
- Secondary feature overlap: Child agent local service, only for the installed
  Rust child-agent hello, heartbeat, inventory, and capability reporting.
- Main expectations: LAN pairing and platform claims.
- Product goal: find household LAN devices, merge evidence into stable device
  records, identify Ocentra child agents when present, and expose trusted
  device state that parent portal surfaces can consume.
- Non-goals: full port scanning, HTTPS interception, packet-content
  inspection, browser URL tracking, AI ownership guesses, router login
  scraping, remote desktop, cloud relay, automatic child assignment, ICMP-only
  reachability truth, and platform claims made from MAC vendor alone.

## 20-Step Plan

1. Establish the LAN discovery contract boundary.
   Define the canonical LAN contracts at the shared Rust boundary: discovery
   source, evidence, interface map, device record, merge score,
   classification, install-eligibility state, child-agent hello, heartbeat,
   assignment, ignore, rename, route state, and proof summary. Keep external
   input as unknown until parsed at the boundary, and keep UI rendering
   downstream of typed Rust-backed service snapshots rather than owning LAN
   truth.

2. Define the durable evidence model.
   Represent every observation as evidence with source, timestamp, confidence,
   and expiry. The core record must support IPs, MACs, hostnames, services,
   vendor data, randomized/private MAC suspicion, DHCP or advertisement
   fingerprints, device type guess, install-eligibility guess, child-agent
   identity, role badges, first seen, last seen, and last confirmed by agent.
   IP must never be the primary identity, and every visible classification or
   installability claim must carry reasons or remain unknown/manual-required.

3. Build active interface detection.
   Detect active network interfaces with name, description, index, MAC, local
   IPs, default gateway, DNS server, DHCP server, broadcast address, IPv4
   subnet, IPv6 prefixes, up/down state, loopback state, and best-effort Wi-Fi
   signal. Exclude loopback, disconnected, VPN, Docker, Hyper-V, WSL, and
   link-local-only interfaces by default, while keeping an advanced manual
   interface selection path.

4. Add OS neighbor table ingestion.
   Read fast local neighbor state before any active scan. Windows should use
   GetIpNetTable2, Linux should start with /proc/net/arp and ip neigh, and macOS
   can begin with arp -a parsing before native adapters are added. When a
   router/controller lease table or ARP/NDP table is accessible through an
   allowed integration, ingest it as additional evidence rather than a
   privileged override. This creates early IP, MAC, interface, and
   neighbor-state evidence for both IPv4 and IPv6 where the platform exposes
   it.

5. Add targeted IPv4 ARP checks.
   Use targeted SendARP-style checks on Windows and equivalent platform paths
   where available. This validates individual discovered hosts and helps refresh
   stale records without sweeping the whole subnet.

6. Add bounded ARP sweep.
   Sweep local IPv4 subnets only after interface selection. Default to /24
   sized work, rate-limit larger ranges, and require confirmation or a
   conservative cap for large subnets. Store ARP replies as evidence, not final
   identities, and do not substitute ICMP ping truth for ARP/NDP identity.

7. Add passive discovery listeners.
   Keep passive listeners for ARP, DHCP, mDNS, SSDP, WS-Discovery, LLMNR,
   NetBIOS, allowed SNMP responses, and Ocentra agent announcements while the
   service is running. Passive evidence should refresh last-seen state without
   constantly blasting the LAN, and ICMPv6/NDP observations should refresh
   IPv6-capable device evidence where platform adapters can expose them safely.

8. Add mDNS and DNS-SD queries.
   Query useful service types such as \_services.\_dns-sd.\_udp.local,
   \_workstation.\_tcp.local, \_smb.\_tcp.local, \_ssh.\_tcp.local,
   \_http.\_tcp.local, \_https.\_tcp.local, \_ipp.\_tcp.local,
   \_ipps.\_tcp.local, \_printer.\_tcp.local, \_scanner.\_tcp.local,
   \_airplay.\_tcp.local, \_raop.\_tcp.local, \_googlecast.\_tcp.local,
   \_androidtvremote2.\_tcp.local, \_adb.\_tcp.local,
   \_adb-tls-connect.\_tcp.local, \_hap.\_tcp.local, \_matter.\_tcp.local,
   \_esphomelib.\_tcp.local, \_sonos.\_tcp.local, \_roku-ecp.\_tcp.local,
   \_companion-link.\_tcp.local, \_ocentra-parent.\_tcp.local, and
   \_ocentra-agent.\_tcp.local. Store hostnames, instance names, service
   types, TXT fields, model/vendor hints, and confidence separately.

9. Add SSDP and UPnP discovery.
   Send bounded M-SEARCH queries and parse device description metadata when a
   discovered device offers it. Store friendly name, manufacturer, model,
   device type, UDN/UUID, and description URL as evidence. Use this to classify
   routers, TVs, consoles, printers, and other infrastructure without treating
   them as enrollable child devices, and keep any follow-up descriptor fetches
   bounded to known metadata URLs rather than generic crawling.

10. Add NetBIOS, LLMNR, and reverse DNS enrichment.
    Use these as name-enrichment sources only, especially for Windows-heavy
    homes. They can improve display names but must not prove child identity or
    ownership on their own.

11. Add light service probing only for discovered hosts.
   Probe only hosts found through earlier evidence. Limit ports to curated safe
   identity hints such as SSH, DNS, HTTP, HTTPS, SMB, AFP, IPP, Chromecast, web
   UI, printer, RPC/RDP/WinRM, media, and device-admin ports when the selected
   proof scope allows them. For HTTP/HTTPS collect only status, server header,
   title, redirect location, TLS certificate subject, and known descriptor
   links. Where the operator allows it, use bounded WS-Discovery or SNMP
   identity queries on already discovered hosts, and keep any OS-fingerprint
   module optional, explainable, and manual-gated rather than a default trust
   source. Do not crawl pages, do not perform blind full-network scans, do not
   brute-force communities or credentials, and do not let open ports or banners
   confirm child identity on their own.

12. Add OUI/vendor lookup.
   Map MAC prefixes to vendor evidence and mark mobile-private/randomized MAC
   suspicion when appropriate. Vendor can inform classification and display
   copy, but cannot prove owner, OS version, installed apps, child profile, or
   platform by itself.

13. Build the merge and de-duplication engine.
    Merge one physical device into one canonical record using strong keys:
    Ocentra agent device id, install id, stable pairing id, MAC on the same
    network, SSDP UDN, and stable mDNS instance id. Treat IP-only, hostname-only,
    vendor-only, type-only, or prior-scan-only matches as weak. Auto-merge only
    above the configured strong threshold; otherwise keep records separate or
    ask.

14. Build explainable classification.
   Classify devices as router, phone, tablet, laptop, desktop, printer, TV,
   game console, smart speaker, camera, NAS, IoT, Ocentra child device, or
   unknown. Every classification needs reasons and confidence, such as vendor,
   DHCP fingerprint, mDNS service type, SSDP/WSD type, hostname pattern, open
   service hint, child-agent platform, or manual parent label. Classification
   must stay evidence-fused and degrade to unknown/manual-required when the
   signals are weak or contradictory. Add install-eligibility states such as
   known installable, probably installable, known not installable, and unknown
   or needs confirmation, with reasons tied to allowed install paths rather
   than platform guesses alone.

15. Add the canonical household device store.
    Persist known devices and evidence in a durable local store so Devices,
    Policy, Activity, Network, Tracking, and AI screens reuse the same child
    targets. Store assignment, rename, ignore, revoked, stale, offline, and
    manual-required states. Persist classification reasons, installability
    reasons, and weak/strong/manual evidence without collapsing them into one
    guess. Keep a prior-scan snapshot or JSON history that can act as weak
    continuity evidence for stable households, stale/offline decisions, and
    merge hints across restarts, but never let yesterday's snapshot override
    stronger current evidence, manual parent decisions, or signed child proof.
    A router remains visible infrastructure, not an enrollable child-agent
    target.

16. Add parent-visible read models and events.
   Expose service-backed read models for scan state, discovered devices,
   trusted devices, route state, controller/observer authority, selected
   device readiness, child-agent presence, stale/offline state, classification
   reasons, installability state, and manual proof requirements. Emit events
   for interface changes, scan start/finish, evidence found, device
   found/updated, online/offline, unknown detected, agent discovered, and
   agent confirmed.

17. Add parent and child mDNS service advertisements.
    Parent advertises \_ocentra-parent.\_tcp.local with protocol version,
    family hash, and pairing state. Child advertises \_ocentra-agent.\_tcp.local
    with protocol version, opaque device id, platform, agent version, and paired
    state. Do not broadcast child names, emails, raw policy, or sensitive
    profile data in TXT records.

18. Add signed child-agent hello and heartbeat.
    After pairing, the child agent connects outward to the parent when possible
    and sends a signed hello containing protocol version, opaque device id,
    install id, family hash, optional child profile hash, platform, hostname,
    agent version, local IPs, available MACs, capabilities, nonce, and
    signature. Heartbeats update confirmed presence, route readiness, and
    offline/stale timers.

19. Add pairing, assignment, rename, ignore, revocation, and audit behavior.
    Anonymous LAN callers must be rejected. Pairing proof must be scoped to a
    device relationship, replay-resistant, origin-checked, route-checked, and
    audited. Parent assignment, rename, ignore, and revocation mutate the
    canonical household device store and must survive restart or fall back to a
    safe unpaired state.

20. Add proof gates and rollout validation.
   Validate Rust-owned contracts, Rust protocol parity, local service
   behavior, merge scoring, storage restart behavior,
   anonymous/wrong-origin/wrong-device rejection, revocation, child
   hello/heartbeat, randomized/private MAC handling, service-probe
   sanitization, no-crawl/no-full-scan behavior, no-platform-from-vendor
   rules, installability-state honesty, and portal read-model consumption.
   Tests must live in real organized test folders or crates, not inline
   source-owned tests, placeholder directories, or fake/mock readiness
   coverage. Record manual-required states for physical household LAN, mobile
   child-agent behavior, OS permissions, packet-mode adapters, and any
   platform claim that CI cannot prove.

## Implementation Order

The first concrete implementation slice should be:

1. Rust-owned contracts and real tests for evidence, source, device record,
   and merge result.
2. Interface detector and neighbor table reader.
3. Vendor lookup and merge/scoring engine.
4. Durable store and event/read-model stream.
5. ARP sweep, mDNS, and SSDP discovery.
6. Manual assign, rename, ignore, and revocation.
7. Child-agent advertisement, signed hello, and heartbeat.
8. Proof command and manual evidence checklist.

## Validation Expectations

- The required test and proof blueprint in
  [V0.9 LAN Discovery Test Blueprint](v0-9-lan-discovery-test-blueprint.md)
  must be followed for every implementation slice.
- Contract tests for valid and invalid LAN discovery payloads in real organized
  test folders or crates.
- Rust protocol parity tests before Rust service accepts or emits payloads.
- Service tests for neighbor ingestion, merge behavior, restart persistence,
  anonymous rejection, wrong-origin rejection, wrong-device rejection, signed
  hello acceptance, heartbeat, revocation, stale, and offline behavior.
- Proof script that writes explicit implemented, scaffold, unavailable,
  degraded, and manual-required states.
- Manual two-device proof before production household LAN readiness is claimed.

## Platform Delivery Notes

- Desktop/server/gateway:
  use packet-capable adapters such as native APIs or optional Npcap paths for
  ARP/passive capture where allowed, plus UDP/TCP probes, HTTP/TLS parsing, and
  bounded metadata fetches.
- Android:
  rely on platform-safe adapters such as `NsdManager`,
  `WifiManager.MulticastLock`, UDP SSDP sockets, TCP probes, and explicit
  permissions; do not pretend normal Android builds have unrestricted raw ARP
  power.
- iOS:
  rely on Bonjour or `Network.framework`, local-network permission, and bounded
  TCP/UDP discovery; do not pretend a normal iOS build can run unrestricted raw
  ARP sweep logic.

## Open Product Questions

- Which packet-mode dependency is acceptable for Windows production: Npcap,
  a native adapter-only path, or a staged optional capability?
- Should the first durable store live inside the existing service store or in a
  LAN-specific store module that later composes into the household device store?
- Should DHCP fingerprinting and MAC/OUI data ship with a local curated dataset,
  an optional update path, or a pluggable provider boundary?
- Should evidence-fusion weights live as fixed audited defaults, operator
  configuration, or a pluggable fingerprint provider boundary?
- Which manual proof artifacts are required before the UI may remove
  manual-required labels for physical household discovery?
- What is the minimum child-agent inventory packet for the first shipped
  pairing path: identity and heartbeat only, or CPU/GPU/memory/interface
  inventory too?

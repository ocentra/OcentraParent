# V0.9 LAN Discovery Test Blueprint

<!-- agent-capsule -->

> Agent Capsule
> Plan: `lan-plan`
> Doc: `V0.9 LAN Discovery Test Blueprint`
> Kind: test blueprint reference; read only when local expectations route here.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.

<!-- /agent-capsule -->

This is the companion requirement blueprint for the
[V0.9 LAN Discovery 20-Step Plan](v0-9-lan-discovery-20-step-plan.md). The
20-step plan defines what to build. This blueprint defines the tests, fixtures,
proof gates, and quality bars required while building it.

## Scope

Included:

- Parent scanner.
- Device evidence model.
- Merge and classification engine.
- Neighbor table, ARP, DHCP, mDNS, SSDP, UPnP/WSD descriptor paths, NetBIOS,
  LLMNR, reverse DNS, and bounded service-enrichment discovery.
- Rust child-agent discovery, signed hello, and heartbeat.
- Durable household device storage.
- Parent-visible device inventory and evidence read models.

Excluded:

- Browser control.
- Screen time.
- Website blocking.
- App blocking.
- AI classification.
- Remote desktop.
- Cloud relay.

## Core Test Principle

### LAN-TEST-001: Evidence-backed visible devices

Requirement: Every device shown in a parent-facing inventory must have at least
one evidence source.

Proof: Contract or read-model tests reject visible device records with an empty
evidence summary.

Acceptance: Unknown devices may be visible, but they must be labeled as
unknown, guessed, or manual-required and must expose their source evidence.

### LAN-TEST-002: Explainable merges

Requirement: Every automatic merge must record why it happened.

Proof: Merge tests assert score, reasons, source records, and whether the merge
was automatic, blocked, or manual-required.

Acceptance: The UI or diagnostics can explain the evidence behind one physical
device becoming one canonical record.

### LAN-TEST-003: Cryptographic child-agent confirmation

Requirement: A device can be marked confirmed by Ocentra Agent only after a
valid paired child-agent hello or heartbeat passes signature, nonce, family, and
route validation.

Proof: Child hello and heartbeat tests reject unsigned, replayed, expired,
wrong-family, wrong-device, and unpaired messages.

Acceptance: mDNS TXT records, hostnames, IPs, and MACs can suggest agent
presence, but they cannot confirm a child device.

### LAN-TEST-004: Reproducible scanner results

Requirement: Default CI tests must not depend on the user's real LAN.

Proof: Scanner tests use fixtures, controlled packet IO, controlled local responders, explicit timestamps,
and local test servers.

Acceptance: Real LAN proof is manual or explicitly tagged integration evidence,
not a hidden dependency of normal CI.

## Identity Requirements

### LAN-TEST-005: IP is not permanent identity

Requirement: IP address alone must never create permanent device identity or
trigger automatic merge.

Proof: Unit and E2E tests cover DHCP IP changes and IP reuse by a different
device.

Acceptance: A reused IP creates or updates evidence only; it does not overwrite
another device's identity.

### LAN-TEST-006: MAC confidence is bounded

Requirement: MAC evidence can support identity only within confidence rules, and
locally administered or randomized MACs must reduce confidence.

Proof: OUI and merge tests cover known vendors, unknown vendors, malformed MACs,
locally administered MACs, and multicast MAC rejection.

Acceptance: Phone privacy behavior is displayed honestly and does not produce
overconfident ownership or child-profile claims.

### LAN-TEST-007: Strong identity keys win

Requirement: Ocentra agent device id, install id, pairing id, SSDP UDN, and
stable mDNS instance id are stronger identity keys than IP, hostname, vendor, or
device type.

Proof: Merge scoring tests assert the score and auto-merge decision for each
strong and weak key.

Acceptance: Strong identity merges are deterministic, and weak evidence stays
manual-required or separate.

### LAN-TEST-008: Manual parent decisions persist

Requirement: Parent rename, assignment, ignore, and revocation decisions must
survive rescans, restarts, and weak contradictory evidence.

Proof: Store and rescan tests preserve manual fields while adding new evidence.

Acceptance: A rescan cannot erase parent intent or silently reassign a device to
another child.

### LAN-TEST-009: Records preserve multiple evidence values

Requirement: A device record must support multiple IPs, MACs, names, services,
and evidence sources.

Proof: Evidence model tests add repeated and multi-source evidence and verify
first-seen, last-seen, and source preservation.

Acceptance: Evidence history is not collapsed into a single lossy value.

## Merge Requirements

### LAN-TEST-010: Required merge outcomes

Requirement: Same Ocentra device id must merge, same install id must merge, and
same MAC on the same LAN should normally meet the auto-merge threshold.

Proof: Merge tests cover agent id, install id, same MAC, SSDP UDN, and mDNS
instance matches.

Acceptance: One physical child-agent device does not appear as duplicate
local-agent and LAN-neighbor rows.

### LAN-TEST-011: Required non-merge outcomes

Requirement: Same IP only, same hostname only, same vendor only, or same device
type only must not auto-merge.

Proof: Merge tests assert those cases score below the auto-merge threshold and
record weak reasons.

Acceptance: DHCP reuse and common hostnames do not corrupt inventory.

### LAN-TEST-012: Forbidden merges

Requirement: Different Ocentra agent device ids or different manually assigned
child ids must never auto-merge.

Proof: Merge tests return a forbidden merge state for conflicting agent or child
assignment identity.

Acceptance: Confirmed or parent-assigned devices cannot be merged away by later
weak LAN evidence.

## Scanner Protocol Requirements

### LAN-TEST-013: Interface filtering

Requirement: Scanner interface selection must ignore loopback, down,
disconnected, VPN, Docker, Hyper-V, WSL, and link-local-only interfaces by
default, while allowing explicit manual selection. The normalized interface map
must preserve gateway, DNS, DHCP, broadcast, subnet, and IPv6-prefix fields
when the platform exposes them.

Proof: Unit tests cover each interface type, default route preference, Wi-Fi,
Ethernet, and manual override.

Acceptance: The scanner does not send packets on unintended virtual networks
unless the user explicitly chooses them.

### LAN-TEST-014: Neighbor table normalization

Requirement: OS neighbor table readers must normalize platform output into the
same evidence shape.

Proof: Parser fixture tests cover Windows neighbor data, Linux /proc/net/arp,
Linux ip neigh, macOS arp -a, empty tables, malformed rows, duplicate rows, and
incomplete MACs.

Acceptance: Neighbor ingestion produces IP, MAC, interface, state, timestamp,
and source evidence without platform-specific leakage.

### LAN-TEST-015: ARP sweep uses controlled packet IO in CI

Requirement: ARP sweep logic must be tested through a packet IO abstraction, not
real packet drivers in default CI.

Proof: Integration tests use controlled packet IO to assert host range selection,
network/broadcast exclusion, response window behavior, malformed replies,
deduplication, and no-reply behavior.

Acceptance: Packet-mode driver availability does not decide whether CI passes.

### LAN-TEST-016: mDNS discovery uses fixtures and responders

Requirement: mDNS tests must cover service enumeration, A/AAAA, SRV, TXT, common
device services, and Ocentra agent service discovery.

Proof: Integration tests use fixture packets or controlled local responders for Apple,
Android, Chromecast, printer, workstation, and Ocentra agent cases.

Acceptance: mDNS enriches records without bad duplicates.

### LAN-TEST-017: SSDP discovery is bounded and safe

Requirement: SSDP tests must cover M-SEARCH, LOCATION parsing, description XML,
friendly name, manufacturer, model, device type, UDN/UUID, missing LOCATION,
bad XML, and timeout behavior.

Proof: Controlled UDP and HTTP responders serve router, TV, console, printer, missing,
malformed, and timeout fixtures.

Acceptance: SSDP enriches discovered devices without crashing or treating
routers as enrollable child-agent targets.

### LAN-TEST-018: Service probe is enrichment only

Requirement: TCP/HTTP/HTTPS probing must run only on discovered hosts and must
collect bounded identity hints only.

Proof: Integration tests use local controlled servers for closed port, HTTP title,
HTTPS certificate subject, timeout, max concurrency, and no link crawling.

Acceptance: Service probing cannot become broad port scanning or page crawling.

### LAN-TEST-037: Passive DHCP and fingerprint evidence

Requirement: Passive DHCP evidence must be parsed as bounded identity input
only, including hostname, vendor class, client id, and parameter-request
fingerprint when present.

Proof: Parser and integration fixtures cover normal DHCP lease traffic, missing
options, malformed options, repeated leases, private/randomized MAC clients,
and safe downgrade to unknown/manual-required when fingerprint evidence is thin.

Acceptance: DHCP can strengthen or weaken classification confidence, but it
cannot confirm child identity or overwrite a stronger manual/device record.

### LAN-TEST-038: WS-Discovery and metadata safety

Requirement: WS-Discovery probes and metadata parsing must remain bounded,
sanitized, and explainable.

Proof: Controlled UDP and HTTP fixtures cover Probe responses, Types, Scopes,
XAddrs, missing metadata, malformed XML, timeout, and ONVIF/printer/camera
examples.

Acceptance: WSD can enrich printer, scanner, camera, and Windows-adjacent
classification evidence without becoming a trust or assignment path.

### LAN-TEST-039: Evidence-fusion classifier honesty

Requirement: Classification and install-eligibility output must be explainable,
weighted, and degradable.

Proof: Merge/classifier tests cover router, extender/AP, Windows PC, Apple
device with private MAC, Android phone, Android TV/Chromecast, smart TV,
printer, NAS, camera, and generic IoT fixtures. Tests assert reasons,
confidence, and installability state.

Acceptance: The system never labels Windows, Android, iOS, or installable
status from MAC vendor alone, and weak or contradictory evidence stays
unknown/manual-required.

### LAN-TEST-040: No child confirmation from weak service evidence

Requirement: Open ports, banners, HTTP titles, redirects, TLS subjects, SSDP
descriptors, DHCP fingerprints, and OUI/vendor hints must never confirm a
child agent without signed/trusted proof.

Proof: Contract, service, and UI tests feed plausible weak evidence into the
pipeline and assert the device stays unconfirmed, non-enrollable, and visibly
explained as weak/manual-required.

Acceptance: Agentless discovery stays useful without pretending to be signed
child identity.

### LAN-TEST-041: Prior-scan continuity is weak evidence only

Requirement: Persisted JSON or store-backed prior-scan snapshots may strengthen
continuity, stale/offline state, and merge confidence, but they must remain
historical weak evidence rather than identity truth.

Proof: Store and merge tests cover same-device continuity across restart,
changed hostname, changed IP, conflicting current scan, stale-only historical
row, and a case where prior-scan history conflicts with stronger signed/manual
current evidence.

Acceptance: Prior scans help explain stable household devices, but they cannot
silently resurrect revoked state, overwrite manual decisions, or auto-confirm a
child device.

### LAN-TEST-042: Bounded SNMP and WSD identity queries

Requirement: Active WSD and SNMP identity queries must stay bounded,
allow-list-driven, and non-authoritative.

Proof: Integration fixtures cover WSD Probe/metadata responses, safe SNMP
identity queries such as `sysDescr`, `sysObjectID`, and `sysName`, timeout,
missing metadata, malformed payloads, and the rule that no credential or
community brute force is allowed.

Acceptance: WSD/SNMP can enrich routers, printers, NAS, cameras, and Windows
adjacent devices, but they remain weak-to-strong evidence inputs rather than
child confirmation paths.

### LAN-TEST-043: IPv6 and mobile adapter honesty

Requirement: IPv6 neighbor evidence and mobile-platform scanner capabilities
must be represented honestly rather than implied.

Proof: Adapter and contract tests cover IPv6/NDP evidence when the host
provides it, degraded/manual-required states when it does not, Android
multicast and TCP-probe boundaries, and iOS local-network/Bonjour boundaries.

Acceptance: The product can say what it can and cannot observe on Windows,
Android, and iOS without inventing parity it does not have.

## Child-Agent Requirements

### LAN-TEST-019: Signed hello validation

Requirement: Child hello validation must require protocol version, message type,
device id, install id, family hash, timestamp, nonce, signature, and paired
trust context.

Proof: Tests cover valid signed hello, missing required field, invalid
signature, wrong family hash, expired timestamp, replayed nonce, unknown future
version, unknown capability, and unknown platform handling.

Acceptance: Random LAN clients cannot impersonate child agents.

### LAN-TEST-020: Heartbeat state machine

Requirement: Heartbeats must update last-confirmed time and online state, while
missing heartbeats transition online to stale and offline without deleting the
device record.

Proof: Explicit timestamp tests cover valid heartbeat, wrong signature, wrong family,
timeout, late heartbeat recovery, stale, and offline transitions.

Acceptance: Presence display is deterministic and secure.

## Storage And Contract Requirements

### LAN-TEST-021: Durable store persistence

Requirement: The store must persist device records, evidence, manual name,
assignment, ignored state, revoked state, online state, first-seen, and
last-seen across restart.

Proof: SQLite or durable-store integration tests insert, update, reload,
migrate, ignore, revoke, and mark offline.

Acceptance: Inventory survives service restart and rescans.

### LAN-TEST-022: Device record contract

Requirement: The device record API shape must include id, display name,
assignment, confirmation state, type guess, confidence, IP evidence, MAC
evidence, name evidence, service evidence, evidence summary, online state, and
timestamps.

Proof: Contract tests assert required fields, optional field tolerance, enum
compatibility, ISO timestamps, confidence range, and non-empty evidence summary
for visible devices.

Acceptance: UI and backend cannot silently drift.

### LAN-TEST-023: LAN event stream contract

Requirement: LAN events must be replayable and include event id, timestamp,
session or scan id where relevant, event type, and affected device id for device
events.

Proof: Contract tests cover scan_started, scan_finished, evidence_found,
device_found, device_updated, device_online, device_offline, agent_discovered,
agent_confirmed, and unknown_device_detected.

Acceptance: Parent inventory state can be rebuilt from a snapshot plus events.

## Security Requirements

### LAN-TEST-024: Spoofed agent rejection

Requirement: Spoofed mDNS Ocentra agent announcements, valid-looking unsigned
hellos, copied signatures, wrong family messages, oversized TXT records, and
malformed TXT records must not confirm a device.

Proof: Security tests assert the device remains unconfirmed and diagnostics
record rejection without parser panic.

Acceptance: Discovery cannot be used to impersonate a child device.

### LAN-TEST-025: Parser robustness

Requirement: mDNS, SSDP, UPnP XML, ARP, NetBIOS, LLMNR, and child hello parsers
must reject malformed or oversized input without panic.

Proof: Fuzz, property, or fixture tests cover random bytes, invalid UTF-8,
recursive XML, oversized packets, external SSDP locations, path traversal names,
and unsafe HTML titles.

Acceptance: Malicious LAN packets cannot crash the scanner or poison UI output.

### LAN-TEST-026: Parent local API protection

Requirement: LAN APIs must reject unauthenticated calls, large bodies, unknown
routes, wrong origins, unsigned child hello payloads, and excessive request
rates.

Proof: API integration tests cover auth, body size, route rejection, CORS or
origin restriction, rate limiting, and signed hello acceptance.

Acceptance: Local API exposure is not an open LAN control surface.

## UI Requirements

### LAN-TEST-027: Empty and progressive scan UI

Requirement: The UI must handle empty inventory, scan start, device found,
device updated, and scan finished states without invented devices or duplicate
cards.

Proof: Playwright tests drive contract-backed recorded event streams for empty state and
progressive scan.

Acceptance: Users see real scan progress and stable card updates.

### LAN-TEST-028: Evidence panel UI

Requirement: Every device card must expose evidence details including IP, MAC,
name, service, source labels, first-seen, last-seen, and confidence reasons
when available.

Proof: Playwright tests open the evidence panel for unknown, guessed, and
confirmed devices.

Acceptance: Parent can inspect why Ocentra guessed or confirmed a device.

### LAN-TEST-029: Assignment and rename UI

Requirement: Unknown devices can be manually assigned, renamed, ignored, and
later shown with manual decision state without losing raw evidence.

Proof: Playwright and store tests cover assignment modal, child selection,
rename, ignore, rescan, and restart.

Acceptance: Parent decisions survive rescans and are visually distinct from
scanner evidence.

### LAN-TEST-030: Confirmed and offline UI

Requirement: Confirmed child devices must show confirmation source and online
state; offline devices must remain visible with last-seen state.

Proof: Playwright tests cover confirmed badge, Ocentra Agent online state,
offline event, stale event, and device return.

Acceptance: Offline does not mean forgotten, and guessed devices are not shown
as confirmed.

### LAN-TEST-031: Hostile display values are safe

Requirement: Long, malicious, emoji, HTML-like, and invalid/sanitized hostnames
must not break layout or execute script.

Proof: UI fixtures and Playwright regression tests cover long hostname,
HTML-in-hostname, duplicate hostname, no MAC, private MAC, and multi-IP cases.

Acceptance: Real LAN weirdness does not break the dashboard.

## E2E And Manual Proof Requirements

### LAN-TEST-032: Virtual lab E2E scenarios

Requirement: E2E tests must cover first scan with unknown devices, child-agent
confirmation, DHCP IP change, IP reuse by another device, randomized MAC phone,
and offline/online return.

Proof: Portable simulated E2E runs in CI; Linux network namespace E2E may run
where available.

Acceptance: Core user scenarios are proven without relying on the user's home
network.

### LAN-TEST-033: Real LAN manual validation

Requirement: Manual proof must be captured before production household LAN
readiness is claimed.

Proof: Manual validation artifacts record Windows, macOS, and Linux parent
scanner runs against router, Windows laptop, MacBook, Linux machine, iPhone,
Android phone, smart TV, printer, Chromecast or Google TV, game console when
available, and an Ocentra child-agent device.

Acceptance: Manual artifacts include found state, IP, MAC where available,
vendor, hostname, type guess, confidence, offline/online behavior, assignment
persistence, agent confirmation, and spoof rejection.

## Fixture Requirements

### LAN-TEST-034: Required fixture families

Requirement: Checked-in fixtures must cover ARP, mDNS, SSDP, child agent, API,
SQLite, and UI states.

Proof: Tests load fixtures for normal, empty, malformed, oversized, missing,
future-version, wrong-family, invalid-signature, long-name, same-IP, new-IP, and
confirmed-device cases.

Acceptance: Regression tests can reproduce real LAN edge cases without live
network dependencies.

## Performance And CI Requirements

### LAN-TEST-035: Performance gates

Requirement: The scanner must stay bounded on common home networks.

Proof: Performance tests or benchmark checks cover neighbor-table read under
100 ms, /24 ARP packet build under 50 ms, configurable ARP response window,
merge/classify 256 devices under 100 ms each, load 1000 stored devices under
500 ms, and UI rendering of 100 devices without freezing.

Acceptance: LAN discovery feels fast and does not punish larger home networks.

### LAN-TEST-036: CI validation gates

Requirement: Merge is not acceptable until formatting, clippy, unit,
integration, contract, security parser, schema snapshot, and Playwright fixture-backed
UI tests pass, or an explicit omission record is documented.

Proof: CI or local gate commands include Rust format, clippy with warnings
denied, workspace tests, focused integration/contract tests, and Playwright UI
tests.

Acceptance: CI fails on parser panic, invalid merge, contract drift, spoofed
agent confirmation, duplicate device cards, and ignored tests without a reason.

## Minimum Serious Test Set

The first implementation must not go below this baseline:

- Unit: interface filtering, ARP parser, OUI lookup, evidence update, merge
  scoring, classifier, installability scorer, child hello signature, heartbeat
  state.
- Integration: scanner pipeline, controlled ARP sweep, controlled mDNS,
  controlled SSDP/WSD, controlled DHCP/SNMP/service-probe responders, SQLite
  persistence, child hello endpoint.
- Contract: device record JSON, child hello JSON, LAN event stream JSON.
- E2E: first scan unknown devices, child agent confirms device, DHCP IP change,
  IP reused by different device, randomized MAC, offline/online, and weak
  service evidence that stays unconfirmed.
- Playwright: empty dashboard, progressive scan, evidence panel, assign unknown
  device, confirmed agent badge, offline status, malicious or long hostname,
  and installability/classification explanations.

## Final Quality Bar

LAN discovery is not solid unless:

- A device found by LAN scan is explainable.
- A device confirmed by agent is cryptographically trusted.
- A device guessed by heuristics is clearly marked as guessed.
- A device marked installable or not installable is backed by an explicit path
  or an explicit unknown/manual-required state.
- A device assigned by a parent is preserved.
- A bad packet cannot crash the scanner.
- A DHCP change cannot corrupt inventory.
- A child-agent spoof cannot trick the system.
- The UI shows evidence, not blind confidence.

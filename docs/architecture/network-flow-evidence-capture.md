<!-- agent-capsule -->

> Agent Capsule
> Doc: Network Flow Evidence Capture Architecture
> Kind: architecture/reference documentation; read only when selected by plan route, source router, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

# Network Flow Evidence Capture Architecture

Status: V0.5.3 research/spec. This document defines the network flow evidence
architecture before runtime implementation. It does not add feature code.

## Product Claim

Ocentra Parent may claim process-attributed network flow visibility only for
metadata that the child-device agent actually observes, normalizes, journals,
and replays into the local query store. Network flow evidence can support
parent-visible answers about destinations, ports, protocols, processes,
volume/count summaries, DNS attribution, and suspicious network patterns. It
cannot prove page content, chat content, search terms, exact browser URLs,
active browser tabs, or decrypted payloads.

The first product path is:

```text
Windows network adapters
  -> typed network flow observations
  -> encrypted local journal
  -> local SQLite query store
  -> network read models and digests
  -> portal, local AI references, and dry-run policy
```

Ocentra-hosted services are not the default store for child network activity.
Hosted surfaces may handle account, billing, release, pairing metadata, or
stateless parent-authorized report compilation later, but child flow evidence
and read models stay local/LAN-first unless a future custody feature explicitly
changes that boundary.

## Source Facts

Official Windows documentation establishes the implementation boundary:

| Fact                                                                                                                                                                      | Product impact                                                                                                                                                                       | Source                                                                                                                                      |
| ------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------- |
| Windows Filtering Platform is a network traffic processing platform with hooks into the network stack and a filtering engine.                                             | WFP is the right Windows family to evaluate for future observation or enforcement adapters, but use of WFP does not by itself imply content inspection or URL visibility.            | [About Windows Filtering Platform](https://learn.microsoft.com/en-us/windows/win32/fwp/about-windows-filtering-platform)                    |
| WFP supports per-application and per-connection filtering, packet and stream processing, IPv4/IPv6, stateful filtering, and enumeration/reporting of platform state.      | A WFP-backed adapter may produce strong connection metadata and later enforcement hooks, but the contract must encode what each adapter can actually prove.                          | [Windows Filtering Platform overview](https://learn.microsoft.com/en-us/windows/win32/fwp/windows-filtering-platform-start-page)            |
| Event Tracing for Windows lets controllers enable providers and consumers read events in real time or from trace files.                                                   | ETW can be considered for observation pipelines, but loss, buffering, provider availability, and decoding limits must become typed capability/degraded states.                       | [About Event Tracing](https://learn.microsoft.com/en-us/windows/win32/etw/about-event-tracing)                                              |
| Windows network tracing uses ETW providers from components such as Winsock, TCP/IP, NDIS, and packet capture, with chronological event delivery and correlation metadata. | ETW can help correlate network activity, but raw trace capture is not the normal product evidence store and must be reduced to typed flow metadata before portal, AI, or policy use. | [Network Tracing in Windows 7: Architecture](https://learn.microsoft.com/en-us/windows/win32/ndf/network-tracing-in-windows-7-architecture) |
| `GetExtendedTcpTable` retrieves TCP endpoints and can return owner PID tables for IPv4 and IPv6.                                                                          | IP Helper polling can provide low-risk process and endpoint snapshots for TCP, but it is a snapshot source and not a complete bandwidth, DNS, URL, or payload source.                | [GetExtendedTcpTable](https://learn.microsoft.com/en-us/windows/win32/api/iphlpapi/nf-iphlpapi-getextendedtcptable)                         |
| `GetExtendedUdpTable` retrieves UDP endpoints and can return owner PID tables for IPv4 and IPv6.                                                                          | UDP endpoint snapshots can support process/local-port attribution, but destination correlation and byte counts require stronger adapters or additional evidence.                     | [GetExtendedUdpTable](https://learn.microsoft.com/en-us/windows/win32/api/iphlpapi/nf-iphlpapi-getextendedudptable)                         |
| `Get-DnsClientCache` retrieves the local DNS client cache.                                                                                                                | DNS cache data can support best-effort domain attribution, but cache entries are host-level observations and must not be overclaimed as per-process or exact URL evidence.           | [Get-DnsClientCache](https://learn.microsoft.com/en-us/powershell/module/dnsclient/get-dnsclientcache?view=windowsserver2025-ps)            |

## Non-Negotiable Evidence Boundary

Network flow evidence may prove:

- Process id, process identity, executable reference, signature/hash reference,
  and publisher where available.
- Local IP and port, destination IP and port, transport protocol, TCP state,
  adapter/interface hints, route hints, timestamps, and connection lifecycle
  where available.
- DNS/domain attribution with confidence and source state, including
  domain-known, IP-only, cached-DNS, observed-DNS, reverse-DNS, and unknown.
- Counters such as connection count, first seen, last seen, duration, bytes sent,
  bytes received, repeated failures, and high-volume status where the adapter
  can prove them.
- VPN, proxy, tunnel, Tor-like, unknown adapter, and unusual background traffic
  indicators when derived from explicit evidence.

Network flow evidence must not claim:

- Decrypted HTTPS payloads.
- Full page URLs, URL paths, query strings, active browser tabs, page titles, or
  browser profiles.
- Request or response bodies.
- Search terms, form values, cookies, tokens, credentials, chat messages, video
  content, or page content.
- Child intent, safety classification, or policy result without a separate
  evidence-backed AI or policy contract.
- Per-process DNS attribution when the source only proves host-level DNS cache
  or host-level DNS events.

Unknown and encrypted states are first-class results. They must remain unknown
unless another evidence path proves a stronger claim.

## Adapter Strategy

The implementation should prefer layered observation instead of one broad packet
capture path.

### Phase 1 Snapshot Adapter

Use IP Helper endpoint snapshots for a conservative first adapter:

- TCP endpoint tables with owner PID.
- UDP endpoint tables with owner PID where available.
- Process inventory join through the existing process/window evidence boundary.
- DNS client cache or resolver observations as separate host-level evidence.
- Timestamped polling cadence with explicit snapshot freshness.

This phase can show current/recent endpoints and process correlation, but should
not claim complete flow duration, byte counts, or DNS attribution for every
connection unless the evidence proves it.

### Phase 2 Event Adapter

Add ETW network observation only after the contracts can represent:

- Provider enabled/disabled/unavailable state.
- Lost event counters or buffer pressure.
- Event decode failures.
- Event-to-process correlation confidence.
- Adapter startup/shutdown and privilege requirements.
- Reduction from raw trace events into typed flow observations before storage.

ETW trace files are not the normal evidence store. If temporary ETL files are
needed during adapter development or diagnostics, they must be local, bounded,
redacted from portal copy output, and deleted or rotated under explicit support
rules.

### Phase 3 WFP Adapter

Evaluate WFP for stronger real-time metadata and later enforcement. A WFP path
must stay behind a separate capability boundary because it may need elevated
permissions, driver packaging, signed components, and careful performance work.

WFP is also the likely future enforcement platform, so observation contracts
should avoid mixing observation with blocking decisions. A flow observation can
later be referenced by policy, but it is not a policy decision by itself.

## Components

Network capability detector:

- Reports whether snapshot, DNS, ETW, and WFP adapters are available,
  unsupported, disabled, permission-limited, degraded, or errored.
- Records platform, adapter id, source id, privilege mode, and degraded reason.
- Does not emit successful evidence when only capability detection ran.

Endpoint snapshot adapter:

- Reads TCP and UDP endpoint tables on a bounded cadence.
- Joins owner PID to process identity where available.
- Emits typed observations with snapshot time and freshness window.
- Marks short-lived missed flows as a known limitation instead of pretending
  complete coverage.

DNS/domain attribution adapter:

- Records DNS cache or DNS event evidence separately from flow evidence.
- Produces attribution candidates with source, confidence, TTL/freshness, and
  ambiguity state.
- Supports many-to-one and one-to-many IP/domain relationships.
- Never upgrades host-level DNS evidence into exact per-process or URL evidence
  without a deliberate join contract.

Flow correlator:

- Joins endpoint, process, DNS, adapter, and route evidence by typed ids.
- Emits correlation states such as process-attributed, process-unknown,
  domain-known, domain-ambiguous, IP-only, DNS-unavailable, and adapter-stale.
- Preserves raw source evidence references for parent-visible and AI summaries.

Journal writer and ingest:

- Writes normalized observations and summaries to the encrypted local journal
  before portal, AI, policy, or export uses them.
- Replays journal records into SQLite network read models.
- Treats SQLite as rebuildable query/index state, not the evidence source of
  truth.

Portal read model provider:

- Exposes validated service payloads for recent flows, top destinations, top
  processes, unusual network states, capability status, and copy/debug output.
- Redacts local private paths, raw trace file paths, bridge endpoints, secrets,
  and any future sensitive diagnostics by default.
- Labels source and custody state on each row or summary.

Local AI and policy reference provider:

- Supplies evidence ids and concise typed digests.
- Does not pass packet dumps, decrypted content, DNS cache dumps, raw ETL files,
  cookies, tokens, or private file paths.
- Requires AI output to reference stored evidence ids and to keep uncertainty in
  the result.

## Contract Set

The contract set should be added before runtime code consumes network evidence:

- `NetworkCaptureCapabilitySnapshot`.
- `NetworkObservationSource`.
- `NetworkAdapterStatus`.
- `NetworkEndpointObservation`.
- `NetworkProcessAttribution`.
- `NetworkDnsAttribution`.
- `NetworkFlowObservation`.
- `NetworkFlowCounterSummary`.
- `NetworkFlowCorrelationState`.
- `NetworkFlowCustodyState`.
- `NetworkUnusualIndicator`.
- `NetworkEvidenceDigest`.
- `NetworkFlowReadModel`.
- `NetworkFlowEvidenceReference`.
- `NetworkFlowCopyDebugPayload`.

The final names belong in the owning domain package, but the concepts should not
be skipped or hidden in runtime code.

## Flow Observation Shape

A network flow observation should include:

- Evidence id.
- Schema version.
- Observed at timestamp.
- Source id and adapter id.
- Device/host reference.
- Observation mode: snapshot, event, correlated-summary, or replayed-summary.
- Freshness/staleness timestamp.
- Platform capability status and degraded reason when applicable.
- Local endpoint: IP, port, address family, and interface reference where
  available.
- Remote endpoint: IP, port, address family, protocol, and TCP state where
  available.
- Process attribution: process id, process name, executable path reference,
  publisher/signature/hash reference, and attribution confidence.
- DNS attribution: domain, canonical name, record type, source, confidence,
  freshness, ambiguity, or unavailable reason.
- Counters: connection count, bytes sent, bytes received, duration, first seen,
  last seen, and failure count where available.
- Route/adapter hints: interface id/name reference, VPN/proxy/tunnel indicator,
  loopback/LAN/WAN/public/private classification where provable.
- Source evidence refs used by any correlation.
- Custody state and retention state.

## Attribution Truth Ladder

The implementation must distinguish levels of certainty:

| Level                | Meaning                                                                                                     | Parent-visible claim                                                                 |
| -------------------- | ----------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------ |
| Endpoint observed    | Adapter sees local/remote endpoint metadata.                                                                | Device communicated with an IP/port/protocol. Process and domain may be unknown.     |
| Process attributed   | Endpoint owner PID joins to a process identity.                                                             | Process appears to have network activity for the endpoint.                           |
| Domain candidate     | DNS cache or DNS event maps a domain to an IP in a relevant time window.                                    | Domain may be related to this flow, with confidence and ambiguity shown.             |
| Domain known         | A deliberate join contract proves a domain-to-flow relationship with acceptable freshness and no ambiguity. | Flow is attributed to a domain. Exact URL and content still unknown.                 |
| Summary derived      | Journaled observations replay into SQLite rollups.                                                          | Parent can see top destinations/processes and unusual indicators with evidence refs. |
| Policy/AI referenced | Local AI or dry-run policy consumes a typed digest that cites stored evidence ids.                          | Classification or rule result is based on stored metadata, not packet inspection.    |

If a source cannot prove a level, the system must keep the lower level and show
the missing or degraded state.

## DNS And Domain Correlation

DNS attribution must be modeled as evidence, not a guess.

Acceptable source states:

- `observed-dns-event`: DNS query/response evidence observed by a typed adapter.
- `dns-client-cache`: local cache entry with TTL/freshness limits.
- `reverse-dns`: PTR lookup result, explicitly marked as reverse lookup.
- `static-hosts`: local hosts file or configured resolver source when supported
  and explicitly collected.
- `parent-rule-domain`: parent-entered rule domain used for policy comparison,
  not evidence that the child visited that domain.
- `domain-unavailable`: DNS source disabled, unsupported, permission-limited, or
  errored.
- `domain-ambiguous`: multiple domains map to the same IP or freshness windows
  overlap.
- `ip-only`: no domain evidence exists.

DNS cache and reverse lookups are correlation aids. They do not prove which app
asked for the name, which browser tab was open, or which page path was loaded.

## Custody And Source States

Every read model and copy/debug payload should label where the data came from
and who controls it:

- `live-local-child-agent`: service response over loopback from the child
  device agent.
- `live-lan-child-agent`: service response over explicit LAN mode from the child
  device agent.
- `child-device-journal`: replayed from encrypted child-device journal.
- `child-device-query-store`: rebuilt local SQLite read model.
- `parent-device-cache`: parent device local cache or report cache.
- `parent-owned-export`: parent-approved storage/export source.
- `ocentra-hosted-non-activity`: account, entitlement, pairing, release, or
  notification metadata, not child activity evidence.
- `unavailable`: source not configured, unreachable, degraded, or out of scope.

No V0.5.3 network flow path should silently upload child network activity,
journals, SQLite databases, long-term reports, or parent rules to Ocentra-hosted
storage.

## Journal And SQLite Flow

Network evidence follows the same custody path as other activity:

```text
adapter observation
  -> evidence mapper and correlator
  -> network evidence envelope
  -> encrypted local journal write
  -> SQLite ingest
  -> recent flow and summary read models
  -> portal, local AI references, and dry-run policy
```

SQLite read models should be rebuildable from the journal and expose:

- Recent process-attributed flows.
- Recent IP-only flows.
- Recent domain-attributed flows.
- Top processes by connection count and byte totals where available.
- Top destinations by domain/IP and byte totals where available.
- New destinations by process and host.
- Repeated connection failures.
- VPN/proxy/tunnel indicators.
- Adapter capability and degraded states.
- Evidence ids and source refs for local AI and policy.

Portal and local AI paths must not read raw journal files, raw SQLite files,
ETL files, DNS cache dumps, packet captures, or OS command output directly.

## Parent-Visible Claims

Allowed parent-visible claims:

- "This process opened network connections."
- "This process connected to this IP/domain/port where available."
- "The domain attribution is known, ambiguous, IP-only, unavailable, or stale."
- "This destination is new for this device/process."
- "This process or adapter has unusual high-volume traffic."
- "This flow resembles VPN/proxy/tunnel behavior because these typed indicators
  were observed."
- "This policy or AI preview used these evidence ids."

Disallowed parent-visible claims from network flow alone:

- "The child opened this exact URL."
- "The active browser tab was this site."
- "The child watched this specific video."
- "The child sent this chat message."
- "The child searched for this term."
- "The content of the encrypted session was unsafe."
- "Ocentra inspected decrypted traffic."

Exact URL/tab claims must come from browser evidence. Screen or content
classification must come from a later explicitly enabled evidence path.

## Portal Behavior

The portal should show:

- Network capture capability and degraded states.
- Recent flows with source, custody, timestamp, process, destination, protocol,
  port, domain state, and counters where available.
- Source/custody labels for local loopback, LAN, child journal/query store,
  parent cache/export, hosted non-activity metadata, and unavailable states.
- IP-only, process-unknown, domain-ambiguous, encrypted-content-unavailable,
  adapter-unavailable, stale, and permission-limited states as visible states.
- Unusual network indicators with evidence refs and confidence/reason.
- Copy/debug output that includes evidence ids, timestamps, source ids,
  capability state, custody state, and redacted local references.

The portal must not:

- Run capture adapters or OS commands.
- Infer domains, content, exact URLs, or active tabs from rendering state.
- Query raw journal, SQLite, DNS cache, ETL, or packet files.
- Display fake network activity as if it came from the child agent.
- Present hosted web infrastructure as the default source of child network
  activity.

## Local AI And Policy Use

Local AI may classify only stored digests. Acceptable classifications include:

- likely VPN/proxy/tunnel;
- likely game traffic;
- likely software update traffic;
- unusual unknown process;
- new destination;
- repeated failure;
- ask parent.

AI must cite evidence ids and preserve uncertainty. It cannot invent bytes,
duration, destination, process ownership, domain attribution, decrypted content,
or exact URL. Policy decisions must consume typed flow summaries, parent rules,
and AI dry-run output only after the evidence is journaled and queryable.

## Acceptance Tests And Manual Validation

Contract tests:

- valid and invalid capability snapshots;
- valid and invalid endpoint observations;
- process-attributed, process-unknown, domain-known, domain-ambiguous, IP-only,
  DNS-unavailable, encrypted-content-unavailable, and adapter-unavailable states;
- byte/count summary presence and absence;
- custody/source state parsing;
- local AI evidence reference shape.

Rust/adapter tests:

- TCP snapshot maps owner PID and endpoints into typed observations;
- UDP snapshot maps owner PID and local endpoint state without overclaiming
  remote destination;
- DNS attribution remains host-level unless a join contract proves more;
- adapter errors become typed degraded status;
- stale snapshots do not appear as current;
- raw trace or command output is not exposed in service payloads.

Storage tests:

- network observations write to the encrypted journal;
- SQLite ingest rebuilds network read models from journal replay;
- duplicate evidence ids do not double-count summaries;
- query-store loss can be rebuilt from journal evidence;
- source/custody labels survive replay.

Portal tests:

- recent network panel shows only real service read models;
- IP-only, process-unknown, domain-ambiguous, stale, and adapter-unavailable
  states are visible;
- copy/debug output redacts sensitive local references and includes evidence
  ids;
- hosted/non-activity state is not shown as child activity storage.

Manual Windows validation:

1. Start the agent with network flow evidence enabled in dev mode.
2. Open a known process that creates TCP traffic to a safe test destination.
3. Confirm the journal records endpoint/process metadata before portal display.
4. Confirm SQLite read models show the recent flow and source/custody state.
5. Trigger a DNS-known case and an IP-only case.
6. Confirm exact URL, page content, and decrypted content fields are absent.
7. Confirm stale or unavailable adapters surface typed degraded states.
8. Open the portal on the lane-specific port and verify recent flows, summaries,
   unusual indicators, and redacted copy/debug output.

## Implementation Phases

Phase 0, this spec:

- Add architecture and acceptance plan.
- Do not implement runtime feature code.

Phase 1, contracts:

- Add TypeScript Effect Schema contracts in the owning domain package.
- Add Rust protocol structs only after TypeScript contracts and tests exist.
- Include capability, endpoint, process attribution, DNS attribution, flow
  observation, summary, digest, read model, custody, and failure contracts.

Phase 2, snapshot evidence:

- Add bounded Windows IP Helper snapshot adapter.
- Join endpoint owner PID to process identity through the existing process
  evidence boundary.
- Record unknown, stale, and unsupported states explicitly.

Phase 3, DNS/domain correlation:

- Add DNS evidence source and join contract.
- Model ambiguity, freshness, cache-only, reverse-DNS, and unavailable states.
- Keep DNS evidence separate from exact browser URL evidence.

Phase 4, journal and read models:

- Write network observations to the encrypted journal.
- Rebuild SQLite read models from journal replay.
- Add top processes, destinations, new destinations, and unusual indicators.

Phase 5, portal visibility:

- Add recent network activity and unusual network panels backed by service read
  models.
- Add source/custody labels and redacted copy/debug output.

Phase 6, stronger adapters:

- Add ETW or WFP adapters only after capability, privilege, loss, redaction,
  and performance states are contract-backed and validated.
- Keep enforcement separated from observation until policy milestones own it.

## Done Signal

V0.5.3 network flow evidence planning is done when the repo has a clear
architecture and acceptance plan for process-attributed network metadata,
DNS/domain/IP/port/process correlation, typed evidence contracts, encrypted
journal and SQLite read-model flow, local-first custody states, parent-visible
claims, and no decrypted-payload or exact-URL overclaim.

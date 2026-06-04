# Network And Domain Control

## Parent Outcome

Parents can understand network/domain activity at a safe summary level and set
rules for domains, categories, unusual traffic, VPN/proxy indicators, or
network-derived risks where attribution is proved.

## Ocentra Requirement

Network evidence is metadata-first. Ocentra must not claim decrypted HTTPS
payloads, page contents, message contents, or exact active tab from network
metadata alone.

## Roadmap And Expectations

- Roadmap: V0.4 network observation, V0.8 enforcement, V5 policy product.
- Expectations: [network flow](../expectations/network-flow-evidence.md),
  [policy](../expectations/policy.md),
  [enforcement](../expectations/enforcement.md).
- Supporting docs:
  [network settings inventory](../network-control-settings-inventory.md),
  [network plan](../plans/network-plan/README.md).
- Modules: `packages/activity-domain`, `packages/parent-domain`,
  `crates/agent-core`, `crates/agent-service`.

## Competitor Pressure

See [Competitor Capability Map](../competitor-capability-map.md), especially
web filtering/categories and remote parent access.

Web filters and network appliances often block domains/categories. Ocentra must
compete on control while staying clear about attribution confidence and privacy.

## Current Ocentra State

- Network flow summaries and unusual-indicator reconciliation exist in proof
  form.
- The V0.8 product-control spine keeps network/domain blocking report-only and
  manual-required, distinct from network observation and policy dry-run state.
- The V0.8 product-control runtime path now exposes that network/domain state
  through a Rust service WebSocket read model and typed agent-protocol adapter
  that link to browser/domain and OS-adapter proof without claiming host
  filtering.
- The V0.8 policy-dispatch proof now returns network/domain blocking as
  manual-required with stored network-flow evidence refs, route/source state,
  audit refs, and child reason codes.
- The V0.8 broad-adapter proof now exposes the network/domain runtime gate over
  a Rust service WebSocket read model and TypeScript protocol adapter while
  keeping host DNS/filter apply, rollback, and audit artifacts manual-required.
- The V0.8 supported-adapter runtime proof now marks Windows network/domain as
  an implemented observe-only policy handoff over stored flow evidence while
  host DNS/filter enforcement remains manual-required.
- The V0.8 enforcement integrity runtime audit now includes a
  network-domain-observe-only result with flow evidence refs and an explicit
  host-network-domain-filter manual-required row with required apply/rollback
  artifacts, preserving the no host-filter execution boundary.
- The full-scope network plan now records the end-state evidence,
  intervention, event-bus, analyzer, AI audit, risk-budget, proof-tier, UI, and
  workpack path without upgrading current runtime claims.
- E-D added the first network runtime event chain in `crates/agent-core` using
  the reusable `ocentra-eventing` crate and protocol-owned network event
  constants. The proof covers metadata-only flow/domain/classification,
  AI-audit, policy, enforcement dry-run/manual-required, audit, and portal
  read-model phases without host adapter execution.
- E-D extended the network runtime eventing proof to use the reusable
  `ocentra-eventing` no-subscriber queue/drain path and local typed
  request-response registry for a network review request. The proof remains
  in-process and does not claim broker/family-hub delivery, production
  retention, or host filtering.
- E-D added the first `activity-domain` network contract boundary proof for
  flow evidence, domain evidence, activity classification, A/B/C/D evidence
  grades, and policy/action capability gating. The proof keeps network-only
  exact URL/content claims rejected and keeps dry-run/manual-required states
  from authorizing adapter calls.
- E-D added `ocentra-network-evidence`, a reusable Rust network metadata crate,
  plus the first deterministic PCAP replay proof for classic
  Ethernet/IPv4/UDP DNS query metadata. The proof writes a fixture PCAP,
  expected domain evidence, and must-not-claim artifact without live capture or
  content inspection claims.
- E-D extended `ocentra-network-evidence` with fixture-backed packet parsing
  for Ethernet/IPv4 UDP, TCP, and ICMP plus DNS query/response parsing,
  including compressed response answer names and malformed-packet rejection.
- E-D added fixture-backed visibility parsers for TLS ClientHello SNI, plain
  HTTP Host, QUIC limited-visibility detection, and DoH/DoT resolver-candidate
  detection. These parsers keep exact URL, visited domain, page content, and
  decrypted payload unavailable unless stronger evidence exists.
- E-D added deterministic flow aggregation/sessionization in
  `ocentra-network-evidence`: parsed packet metadata rolls up into five-tuple
  sessions, reverse-direction traffic merges into the same session, idle
  timeouts split sessions, and packet/byte counters stay metadata-only without
  exact URL or decrypted payload claims.
- E-D added deterministic domain normalization and public-suffix proof in
  `ocentra-network-evidence`: DNS/SNI/HTTP-host style domain evidence can be
  lowercased, label-validated, matched to the longest known suffix, and reduced
  to a registrable domain without claiming exact URLs or decrypted content.
- E-D added deterministic domain/category intelligence in
  `ocentra-network-evidence`: exact or registrable domains can be matched to
  source-custody category records, freshness/staleness is explicit, and signed
  snapshot update policy rejects unsigned or older inputs without claiming live
  vendor feeds, exact URLs, or decrypted content.
- E-D added deterministic social/video/game/cloud-gaming classification in
  `ocentra-network-evidence`: fresh domain categories classify directly,
  CDN/process hints remain browser-confirmation-required, and matching browser
  confirmation can promote a candidate without network-only exact URL or
  decrypted-content claims.
- E-D added deterministic VPN/proxy/Tor/tunnel indicator classification in
  `ocentra-network-evidence`: adapter, proxy-port, Tor, and tunnel-protocol
  indicators produce candidate labels while encrypted-DNS-only evidence remains a
  negative tunnel proof and hidden destinations stay unclaimed.
- E-D added deterministic remote desktop/torrent/download candidate
  classification in `ocentra-network-evidence`: remote desktop, torrent, and
  large-download indicators produce candidate labels, while unattributed
  high-volume traffic stays uncertain and file names, exact URLs, and content
  stay unavailable.
- E-D added replay-backed process/app correlation in
  `ocentra-network-evidence`: PID flow evidence links to process snapshots and
  app inventory, process-name-only traffic remains a candidate, and
  adapter-unavailable/missing-process states remain explicit without browser URL
  or decrypted-content claims.
- E-D added a managed-browser correlation bridge in
  `ocentra-network-evidence`: exact URLs are attached only from matching managed
  browser page evidence, while network-only domains and mismatched browser
  evidence stay non-exact and no decrypted/page-content claim is made.
- E-D added Rust protocol-facing network/AI/policy/enforcement/audit/portal
  event contracts in `crates/agent-protocol`. The proof serializes exact
  chain refs, no exact URL/content claim boundaries, policy-decision-gated
  enforcement commands, manual-required adapter results, audit refs, and portal
  visibility state without claiming service delivery or host filtering.
- E-D added protocol-facing parent/controller and child-agent event contracts
  in `crates/agent-protocol` so the network cascade has typed parent/child
  handoff shapes available before runtime publish/transport work begins. This
  does not claim broker delivery, family-hub delivery, or child-agent runtime
  execution.
- E-D added an in-process parent/controller to child-agent runtime proof in
  `crates/agent-core`: validated parent intent events publish through the
  reusable event bus, child-command forward-requested/forwarded events preserve
  exact transport refs, child-agent receive publishes accepted/capability/health
  events, and parent read-model projection follows child runtime health. This
  does not claim broker/family-hub delivery, live network capture, analyzer
  execution, or host filtering.
- E-D added a service-backed enforcement journal/action proof for the network
  manual-required chain: the enforcement API records a pre-action audit activity
  row before adapter execution, then records the final adapter-result audit row
  into the encrypted journal and SQLite store. This proves local audit/read-model
  projection ordering, not host DNS/filter execution.
- Network/domain blocking is not broadly product-complete.
- Raw network control settings are preserved as design inputs, not
  product-complete implementation proof.

## Current Gap

Real OS/domain blocking adapter proof, broader DNS/VPN/proxy handling,
attribution quality, live PCAP/analyzer fixture proof, TypeScript/public export
parity and service wiring for network event contracts, AI detection and audit
proof, risk-budget proof, performance proof, and parent-facing rule UX remain.
Policy dispatch does not upgrade
network/domain blocking beyond
manual-required, and the broad-adapter proof and supported-adapter runtime proof
keep the same manual-required host-filter boundary. The integrity runtime audit
proves observe-only and manual-required state visibility, not live DNS, VPN,
packet capture, signature alert, production risk-budget, or host filter
enforcement. The E-D
runtime spine removes the private-bus blocker for an in-process metadata-only
chain and now proves local queue/drain plus request-response consumption of the
reusable eventing crate, service-side journal-before-action/final-audit
ordering, and typed in-process parent/controller to child-agent handoff.
Analyzer fixtures, broker/family-hub delivery, portal UI, and adapter
apply/rollback artifacts remain open.

## Checklist

- [x] Flow summary and flow-evidence contracts.
- [x] Domain/IP/protocol/process attribution status contracts.
      Replay-backed process/app correlation now links PID flow evidence to
      process snapshots and app inventory without claiming browser URL/content.
- [ ] VPN/proxy/tunnel indicators where available.
      Deterministic indicator classification now exists for VPN adapter,
      proxy-port, Tor, and tunnel-protocol candidates, with encrypted-DNS-only
      negative proof and no hidden-destination claim.
- [ ] Network category/risk targets.
      Domain normalization, public-suffix, and source-custody category
      intelligence proof now exist, along with deterministic
      social/video/game/cloud-gaming classifier candidates. Broader anomaly/risk
      targets and policy handoff remain open.
- [ ] PCAP fixture, Zeek-style summary, and Suricata/Snort-compatible alert
      proof.
      First deterministic DNS query PCAP replay, packet/DNS parser fixtures,
      TLS/HTTP/QUIC/DoH visibility fixtures, and flow/sessionization proof
      exist; analyzer comparison and signature alerts remain open.
- [x] Remote desktop/torrent/download candidate classifier.
      Deterministic indicator proof exists for remote desktop, torrent, and
      large-download candidates, with unattributed high-volume uncertainty and
      no file-name/content claim.
- [ ] Reusable Rust eventing, detection, AI audit, and risk-budget contracts.
      First E-D runtime spine exists for metadata-only flow events,
      manual-required/unavailable states, local no-subscriber queue/drain,
      local typed request-response, and Rust protocol-facing network event
      contracts plus parent/controller and child-agent protocol event
      contracts. Parent/controller validated-intent publishing, typed local
      child-command handoff, and child-agent receive/local publish proof now
      exist. Service-side enforcement audit ordering now proves
      journal-before-action and final adapter-result audit/store projection;
      production analyzer, AI model, broker/family-hub delivery, broader
      service wiring, and risk-budget fixtures remain.
- [ ] Policy preview over stored flow evidence.
- [ ] Adapter capability status.
- [x] Full-scope network plan, proof tiers, UI requirements, and workpacks.
- [ ] Real block/terminate/unavailable result.
- [x] No decrypted payload/page-content claim at the network contract boundary.
      Managed-browser correlation can attach exact URLs only from matching
      browser evidence, not from network metadata.

## Next AI Instructions

Separate observation, attribution, classification, and enforcement. Add unknown
states instead of guessing a process, site, or category. Host network/domain
blocking needs explicit apply, rollback, and audit artifacts before any claim
upgrade beyond the current integrity runtime audit. Analyzer alerts and AI audit
reports are evidence inputs only; policy and adapter proof remain the authority
for any action. Do not wire a network-only event bus; implement the reusable
Rust eventing plan first.

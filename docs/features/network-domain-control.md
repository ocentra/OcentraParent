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
  retention, or host filtering. E-D also added the generic eventing delivery
  decision proof for network routes: local routes carry typed subscriber filters,
  bounded queue/TTL/dead-letter/idempotency backpressure metadata, while
  broker/family-hub routes enumerate custody, auth, encryption, retention,
  replay, deletion, offset, dedupe, broker config, and family-hub identity/relay
  requirements before any product delivery claim.
- E-D added row10a broker delivery semantics proof in `agent-core`: the network
  runtime composes the generic delivery decision with queue idempotency and
  overflow/dead-letter behavior to prove local duplicate-safety semantics,
  replay plan refs, dropped-event audit refs, and zero duplicate
  enforcement-command or adapter-action counts. This remains a proof boundary
  only; live broker/family-hub transport, policy execution, adapter execution,
  and host filtering remain unclaimed.
- E-D added network-specific queue backpressure proof on top of the reusable
  `ocentra-eventing` queue: bounded overflow dead-letters the oldest queued flow
  and keeps the newest queued flow,
  TTL expiry dead-letters before dispatch through manual-clock proof, and
  queued/completed duplicate idempotency keys are rejected. Stored queued
  payloads remain metadata-only and do not claim exact URLs, decrypted content,
  adapter execution, broker/family-hub transport, or service event-chain
  streaming.
- E-D wired the service network read-model command to local runtime delivery:
  `agent.network.flow.read-model.get` now maps stored ActivityStore network rows
  into `agent-core` network runtime observations, publishes them through the
  local `ocentra-eventing` spine, and reports observed/delivered/failed,
  stored, dead-letter, manual-required, and enforcement-command event counts in
  the read-model payload. This is service-local delivery only; broker/family-hub
  transport, production retention/replay, policy execution, adapter execution,
  and host filtering remain unclaimed.
- E-D added service-visible network runtime event-chain streaming:
  `agent.network.runtime.event-chain.stream.get` reads stored ActivityStore
  network rows, republishes metadata-only observations through the local runtime,
  and returns `agent.network.runtime.event-chain.stream.reported` with
  protocol-shaped network/AI/policy/enforcement/audit/portal stream entries and
  counts. This remains service-local; broker/family-hub transport, production
  retention/replay/delete/export, live analyzer/model/policy execution, adapter
  execution, and host filtering remain unclaimed.
- E-D added service/query-store retention tombstone proof for network flow
  facts. `activity.network.retention.deleted` rows now stay visible as local
  deletion evidence refs while ActivityStore read models, service read-model
  payloads, and WebSocket event-chain streams suppress deleted active rows and
  report active, tombstone, and exportable row counts. This proves
  service-local custody/export accounting for stored network facts, not raw
  PCAP/live-capture retention or broker/family-hub deletion propagation.
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
- E-D added deterministic live-capture proof gate in
  `ocentra-network-evidence`: Windows Npcap, Linux libpcap, and macOS
  BPF/libpcap capture readiness is represented from driver, interface,
  permission, bounded-capture, clean-stop, quota-rotation,
  retention/delete/export, custody, and private-traffic-exclusion refs. Missing
  artifacts stay manual-required, unavailable/degraded states are explicit, and
  live driver invocation, unbounded capture, raw PCAP without custody,
  exact-content, policy/adapter authority, and enforcement-command claims are
  rejected.
- E-D added deterministic raw capture storage custody proof in
  `ocentra-network-evidence`: local raw artifact storage is authorized only when
  the row13 live-capture proof is proof-ready and raw artifact manifest,
  encrypted local storage, encryption-at-rest, quota rotation, retention,
  delete/export, custody-chain, and private-traffic-exclusion refs are present.
  Missing refs stay manual-required, unavailable/degraded live-capture states
  remain visible, and live capture execution, remote upload, raw PCAP without
  custody, exact-content, policy/adapter authority, and enforcement-command
  claims are rejected.
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
- E-D added deterministic unmanaged-browser correlation in
  `ocentra-network-evidence`: known or portable browser processes become
  process-only bypass evidence, browser-like process names stay candidate-only,
  managed-browser boundary and adapter-unavailable states remain explicit, and
  exact URL, active-tab, title, page-content, decrypted-payload, policy,
  adapter, and enforcement-command claims are rejected.
- E-D added deterministic app/game foreground/session correlation in
  `ocentra-network-evidence`: stored app/game evidence, session-summary,
  foreground, process-correlation, and launcher refs can confirm foreground or
  running sessions; launcher-only rows are guarded, candidates remain
  review-only, missing/unavailable states stay explicit, and exact URL,
  screen-content, AI-device-scanner, policy, adapter, and enforcement-command
  claims are rejected.
- E-D added deterministic screen-summary trigger planning in
  `ocentra-network-evidence`: a network cascade can queue a screen-summary job
  only when screen confirmation is recommended and parent settings, local
  encrypted queue, deletion, local runtime, debounce, and protected-surface
  guards allow it. Disabled/unavailable/manual-required states stay explicit,
  and raw-image retention, remote upload, screen content, policy, adapter, and
  enforcement-command claims are rejected.
- E-D added a deterministic evidence cascade router in
  `ocentra-network-evidence`: confirmed/candidate/weak signals are ranked,
  weak hints route to managed-browser/process/screen/local-AI next checks, and
  the router never authorizes policy or adapter action.
- E-D added a cross-slice evidence bundle builder in
  `ocentra-network-evidence`: trigger refs plus domain/category,
  managed-browser, process/app, screen, and local-AI suggestion refs can be
  assembled into one downstream evidence bundle after cascade routing. The
  builder preserves all evidence refs, records exact URL refs only from managed
  browser evidence, recommends local-AI review for weak signals, and rejects
  decrypted payload, network-only exact URL, policy-authority, or
  adapter-authority claims.
- E-D added a network-triggered local-AI queue planner in
  `ocentra-network-evidence`: weak or review-recommended bundles can enqueue a
  local-AI review job that carries only trigger refs, evidence refs, summary
  refs, queue refs, and model-runtime refs. Disabled, model-unavailable,
  queue-unavailable, and not-recommended states are explicit and do not carry a
  queue job, and the planner rejects raw packet payload, page content,
  decrypted payload, policy-authority, or adapter-authority claims.
- E-D added deterministic network AI detection fixture evaluation in
  `ocentra-network-evidence`: fixture labels, structured summary refs, evidence
  refs, and analyzer alert refs can be compared against model predictions with
  precision, recall, accuracy, and confidence-drift metrics. The proof is a
  quality gate only; it rejects model-execution, remote-AI, raw PCAP, exact URL,
  page-content, decrypted-payload, policy-authority, adapter-authority, and
  enforcement-command claims.
- E-D added deterministic network AI audit narrative proof in
  `ocentra-network-evidence`: detection refs, evidence refs, analyzer alert refs,
  and parent-rule refs produce parent-readable advisory narratives with
  review/confirmation/monitor recommendations. The proof preserves uncertainty
  codes and rejects remote-AI, raw PCAP, exact URL, page-content, private-message,
  search-query, decrypted-payload, policy-authority, adapter-authority, and
  enforcement-command claims.
- E-D added deterministic household risk-budget threshold proof in
  `ocentra-network-evidence`: AI audit reports, child/profile refs, household
  policy refs, prior-event refs, and adapter proof state map to ignore, monitor,
  ask-parent, warn-child, limit, block, or manual-required recommendations.
  Safe-behavior credit requires parent-rule cap, expiry, audit reason, and UI
  explanation refs; signature-only or missing-adapter cases stay manual-required
  for control actions, and no policy/adapter/enforcement authority is claimed.
- E-D added deterministic network performance benchmark proof in
  `ocentra-network-evidence`: fixture rows aggregate packet-to-summary,
  packet-to-detection, detection-to-cascade latency, event throughput, CPU,
  memory, disk, queue depth, dropped-event, and high-concurrency flow metrics.
  Dry-run/manual-required/unsupported/unavailable/degraded path states are
  preserved, and real-time response, production SLO, raw PCAP, exact URL,
  page-content, decrypted-payload, adapter-action, host-filtering, and
  enforcement-command claims are rejected.
- E-D added deterministic network security readiness proof in
  `ocentra-network-evidence`: threat model, privacy/compliance,
  retention/delete/export custody, key rotation, secret handling, rule/model
  provenance and rollback, support materials, staff training, staged rollout,
  and known-gap signoff refs produce internal-ready, production-blocked, or
  production-ready-with-external-signoff states. Production rollout stays
  blocked without external audit or penetration-test signoff, and default remote
  upload, raw PCAP without custody, exact-content, policy/adapter authority, and
  enforcement-command claims are rejected.
- E-D added an evidence-grade policy mapper in `ocentra-network-evidence`:
  A/B/C/D evidence plus parent rule refs, policy decision refs, evidence refs,
  and optional local-AI result refs map to dry-run, parent-review, or
  observe-only handoff states. The mapper never authorizes adapter actions or
  enforcement commands; B-grade block/limit requests route to parent review, and
  C/D grades remain non-enforcing.
- E-D added a parent notification candidate mapper in
  `ocentra-network-evidence`: policy handoff states map to candidate-only
  parent notification records that preserve notification, policy decision,
  parent rule, evidence, and optional local-AI refs. Provider delivery,
  sensitive payload transport, adapter authorization, and enforcement command
  publication are rejected.
- E-D added a service-backed parent portal network evidence drawer on the
  Activity product route. The drawer renders the real Rust service
  `agent.network.flow.read-model.reported` output, cites ActivityStore evidence
  refs from the network activity digest, shows metadata-only endpoint/domain/
  process attribution, and keeps exact URL, AI, policy, intervention, and
  retention facets explicitly not reported when no service refs exist.
- E-D added a deterministic DNS proxy/block/redirect adapter proof boundary in
  `ocentra-network-evidence`: grade-A block policy plus parent-rule, evidence,
  capability, adapter authorization, apply, result, rollback, and audit refs can
  become apply-ready. Dry-run, weak evidence, manual-required, and unavailable
  states stay non-executable, and exact URL, page content, decrypted payload,
  host DNS mutation, and enforcement-command claims remain rejected.
- E-D added Rust protocol-facing network/AI/policy/enforcement/audit/portal
  event contracts in `crates/agent-protocol`. The proof serializes exact
  chain refs, no exact URL/content claim boundaries, policy-decision-gated
  enforcement commands, manual-required adapter results, audit refs, and portal
  visibility state without claiming service delivery or host filtering.
- E-D added public TypeScript parity for the network runtime event contracts in
  `@ocentra-parent/agent-protocol-domain/network-runtime-events`. The Effect
  Schema contracts mirror the Rust network/AI/policy/enforcement/audit/portal
  event chain, reject exact URL/decrypted payload/message/search/adapter-action
  claims, and prove public package import visibility without claiming
  broker/family-hub delivery or service WebSocket streaming of the event chain.
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
- E-D added a network-specific portal source gate proof. The portal parser now
  consumes the service read-model active/tombstone/export counters and deletion
  evidence refs through shared agent-protocol field constants, the network drawer
  proof runs the real portal network read-model test, and the static gate proves
  `apps/portal/src` plus `packages/portal-domain/src` do not import eventing,
  publish network/policy/enforcement events, compute evidence grades, decide
  policy, or execute adapter/enforcement commands.
- E-D added a network portal manual/runtime state proof. The Rust service now
  carries runtime delivery counts inside the existing network activity digest,
  the TypeScript read-model schema parses that optional `runtimeDelivery`
  object, and the Activity network drawer renders row counts,
  observed/delivered/failed runtime counts, stored/dead-letter counts,
  manual-required rows, enforcement-command event counts, and retention
  tombstone/export/delete state without claiming risk-budget/performance UI or
  policy/adapter authority.
- E-D added an integrated event-plus-network product-path proof in
  `ocentra-network-evidence`: one deterministic artifact carries trigger refs,
  typed-event refs, evidence refs, local-AI queue refs, AI detection/audit refs,
  policy/risk-budget refs, adapter proof refs, audit refs, portal read-model
  refs, and retention/delete/export refs while proving weak or unavailable
  evidence cannot authorize adapter apply and AI/UI/network surfaces cannot
  bypass policy.
- E-D added a platform-claim manifest proof in `ocentra-network-evidence` that
  composes Windows Firewall/WFP, Android VpnService, Apple Network Extension
  macOS/iOS, and Linux nftables/eBPF/TUN gates into exact OS/device/permission claim rows,
  records missing artifacts as manual follow-ups, and rejects generic platform
  support, live adapter execution, UI policy authority, and enforcement-command
  claims.
- Network/domain blocking is not broadly product-complete.
- Raw network control settings are preserved as design inputs, not
  product-complete implementation proof.

## Current Gap

Real OS/domain blocking adapter execution, broader DNS/VPN/proxy handling,
attribution quality, live capture driver invocation, live analyzer fixture
proof, broker/family-hub transport, remote network runtime event-chain
delivery, raw PCAP/live-capture retention, production risk-budget service/UI
wiring,
production SLO validation,
external audit/deployment execution, full support-material authoring, and
parent-facing rule UX remain.
Policy dispatch does not upgrade
network/domain blocking beyond
manual-required, and the broad-adapter proof and supported-adapter runtime proof
keep the same manual-required host-filter boundary. The integrity runtime audit
proves observe-only and manual-required state visibility, not live DNS, VPN,
packet capture, signature alert, production risk-budget, or host filter
enforcement. The E-D
runtime spine removes the private-bus blocker for an in-process metadata-only
chain and now proves local queue/drain plus request-response consumption of the
reusable eventing crate, service-side network read-model delivery into the
local eventing runtime, service WebSocket streaming of protocol-shaped network
runtime event-chain entries, service/query-store retention tombstone filtering
with exportable-row accounting for stored network facts, service-side
journal-before-action/final-audit ordering, typed in-process parent/controller
to child-agent handoff, and cross-slice evidence bundle construction after
cascade routing, integrated trigger-to-retention product-path ref preservation,
plus
network-triggered local-AI queue planning with refs-only AI inputs,
evidence-grade policy handoff mapping, parent notification candidate mapping,
proof-gated DNS and Windows Firewall adapter apply/result/rollback/audit
entitlement/device proof gate, and a Linux nftables/eBPF/TUN distro proof gate.
The row11 aggregate manual/platform proof pack now ties those platform gates to
the required OS/device/permission artifacts, exact manual steps, command logs,
and manual-required labels before any platform claim can be upgraded. Row52 now
adds a platform-claim manifest that names exact Windows, Android, Apple
macOS/iOS, and Linux nftables/eBPF/TUN OS/device/permission refs plus manual
follow-ups for missing artifacts.
Broker/family-hub delivery implementation, local-AI model execution/worker
runtime, full policy engine execution, notification provider delivery, broader
parent-facing rule UX, live host DNS mutation/proxy installation, live Windows
Firewall mutation, live WFP driver/callout/packet blocking, live Android
VpnService filtering, live Apple Network Extension behavior, live Linux adapter
execution, platform adapter execution, and broader portal risk-budget/performance
UI rendering remain open.

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
- [x] PCAP fixture, Zeek-style summary, and Suricata/Snort-compatible alert
      proof.
      First deterministic DNS query PCAP replay, packet/DNS parser fixtures,
      TLS/HTTP/QUIC/DoH visibility fixtures, and flow/sessionization proof
      exist. Deterministic Zeek-style connection, DNS, HTTP, TLS, and SSL summary
      generation with approved analyzer comparison artifacts now exists.
      Suricata/Snort-compatible signature alert ingestion now records typed
      analyzer alerts with signature, rule-source, severity, timestamp, flow,
      evidence, and custody refs, while known false positives and signature-only
      alerts remain non-enforcing.
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
      network-triggered local-AI queue planning now keeps AI inputs to refs
      only, and evidence-grade policy mapping now proves dry-run/parent-review/
      observe-only handoffs with parent rule refs. Parent notification candidate
      mapping now preserves refs without provider delivery, and the Activity
      route now renders a service-backed network evidence drawer with missing
      exact URL/AI/policy/intervention/retention refs labeled as not reported.
      Row37 DNS adapter proof now models apply-ready, dry-run,
      manual-required, and unavailable states with required apply/result/
      rollback/audit refs while refusing host mutation and enforcement command
      claims. Row38 Windows Firewall adapter proof now models apply-ready,
      dry-run, manual-required, and unavailable states with required
      target/rule refs plus apply/result/rollback/audit refs while refusing
      live firewall mutation, command invocation, and enforcement command
      claims. Row39 Windows WFP proof gate now models lab-proof readiness from
      administrator permission, driver signing/package, provider-registration,
      layer-capability, rollback, lab-result, and audit refs while refusing
      live driver install, callout registration, packet blocking, and command
      invocation claims. Row40 Android VpnService proof gate now models
      physical-device readiness from VpnService declaration, user consent,
      package identity, virtual-interface, traffic-observation, rollback, and
      audit refs, with Device Owner proof required only when claimed.
      Row41 Apple Network Extension proof gate now models entitlement/device
      readiness from developer team, entitlement approval, provisioning,
      signing, device/TestFlight, extension declaration/configuration, rollback,
      and audit refs, with supervision/MDM proof required only when claimed.
      Row42 Linux nftables/eBPF/TUN proof gate now models distro readiness from
      distro/kernel, permission, adapter API, service-manager scope, rollback,
      lab-result, and audit refs while preserving the selected adapter kind.
      The row11 manual/platform proof pack aggregates rows 13 and 37-42 into
      `output/network-plan-proof/11-manual-platform-proof/11-manual-platform-proof.md`
      with host/device permission artifacts, exact manual steps, command logs,
      and manual-required labels while leaving live adapter execution and
      screenshots unclaimed until real host/device proof exists.
      Row45 eventing delivery-decision proof now keeps local network routes
      local-first with typed subscriber filters and bounded backpressure
      metadata, and row10 backpressure-depth proof now proves network-specific
      overflow dead-lettering, TTL expiry, and idempotency duplicate rejection.
      Row10a broker delivery semantics proof now preserves replay plan,
      dropped-event audit, and adapter-action ledger refs while proving duplicate
      idempotency rejection and zero duplicate adapter-action counts.
      Broker/family-hub delivery remains requirements-gated rather than
      implemented. Service network read-model delivery now publishes stored
      ActivityStore network rows through the local eventing runtime and exposes
      delivery counts in the service payload; service WebSocket event-chain
      streaming now returns protocol-shaped local runtime entries for stored
      rows, and service/query-store tombstones now hide deleted active rows
      while preserving local deletion evidence refs and exportable-row counts.
      Broker/family-hub, raw PCAP/live-capture retention, policy, adapter, and
      host-filter execution remain unclaimed. Row46 AI detection fixture proof
      now measures model
      predictions against labeled structured-summary fixtures with precision,
      recall, accuracy, and confidence-drift states while rejecting raw content
      and authority claims. Row47 AI audit narrative proof now emits
      parent-readable advisory recommendations with detection/evidence/analyzer/
      parent-rule citations and uncertainty states. Row48 household risk-budget
      threshold proof now maps AI audit reports, age/profile policy, prior
      events, safe-behavior credits, and adapter proof state into
      ignore/monitor/ask/warn/limit/block/manual-required recommendations
      without publishing enforcement commands. Row49 performance benchmark proof
      now aggregates latency, throughput, CPU, memory, disk, queue,
      dropped-event, and high-concurrency fixture metrics while preserving
      manual/unavailable/degraded path states and rejecting realtime/production
      claims. Row50 security readiness proof now gates production rollout on
      threat-model/privacy/compliance/retention/hardening/support/staged-rollout
      refs plus external audit or penetration-test signoff for production-ready
      state. Row11a now records the aggregate network hardening/support proof
      pack for key rotation, secret handling, rule/model provenance, rollback,
      parent/user guides, FAQ, support playbook, staff training, staged rollout,
      incident response, known-gap signoff, and external signoff refs. Row52
      platform-claim manifest proof now composes Windows Firewall/WFP, Android
      VpnService, Apple Network Extension macOS/iOS, and Linux nftables/eBPF/TUN
      gates into exact OS/device/permission claim rows, records manual
      follow-ups for missing artifacts, and rejects generic platform support,
      live adapter execution, UI policy authority, and enforcement-command
      claims. AI model execution, broker/family-hub delivery implementation,
      full policy engine execution, notification delivery, external
      audit/deployment execution, full support-material authoring, and portal
      risk-budget/performance UI rendering remain.
- [x] Parent portal network evidence drawer.
      The Activity route renders real Rust service network read-model output,
      ActivityStore evidence refs, endpoint/domain/process attribution, and
      unsupported-claim states without publishing policy or adapter commands.
      The 36c supplemental proof also renders service-carried runtime delivery,
      manual-required, enforcement-command count, and retention row state from
      the typed network activity digest; risk-budget/performance rendering
      remains a gap.
- [ ] Policy preview over stored flow evidence.
- [ ] Adapter capability status.
      Row37 DNS adapter, Row38 Windows Firewall, Row39 WFP, and Row40 Android
      VpnService plus Row41 Apple Network Extension and Row42 Linux proof gates
      model supported/lab-ready/physical-device-ready/Apple-device-ready/
      distro-ready, manual-required, and unavailable capability states for their
      adapter boundaries. Row11 now records the aggregate manual/platform proof
      pack for OS/device/permission artifacts and exact manual steps, and Row52
      accounts for those claims in one exact OS/device/permission manifest with
      manual follow-ups, but broader
      platform capability status and parent UI surfacing remain open.
- [x] DNS proxy/block/redirect adapter proof boundary.
      The Rust proof accepts apply-ready only with grade-A policy, parent-rule
      refs, evidence refs, supported capability, adapter authorization,
      apply/result/rollback artifacts, and audit refs; dry-run/manual/
      unavailable states remain non-executable and host DNS mutation is not
      claimed.
- [x] Windows Firewall adapter proof boundary.
      The Rust proof accepts apply-ready only with grade-A block policy,
      parent-rule refs, evidence refs, target/rule refs, supported capability,
      adapter authorization, apply/result/rollback artifacts, and audit refs;
      dry-run/manual/unavailable states remain non-executable and live
      firewall mutation or command invocation is not claimed.
- [x] Windows WFP research/proof gate.
      The Rust proof gate accepts lab-proof readiness only with grade-A block
      policy, parent-rule refs, evidence refs, target/provider/layer refs,
      administrator permission, driver signing/package, provider-registration,
      layer-capability, rollback, lab-result, and audit refs; research-only,
      manual-required, and unavailable states remain non-executable and live
      driver install, callout registration, packet blocking, or command
      invocation is not claimed.
- [x] Android VpnService adapter/proof gate.
      The Rust proof gate accepts physical-device readiness only with grade-A
      block policy, parent-rule refs, evidence refs, package/service refs,
      VpnService declaration, user consent, physical-device proof, package
      identity, virtual-interface, traffic-observation, rollback, and audit refs;
      Device Owner proof is required only when claimed, and live tunnel,
      filtering, packet block, or app/package correlation is not claimed.
- [x] Apple Network Extension adapter/proof gate.
      The Rust proof gate accepts entitlement/device readiness only with grade-A
      block policy, parent-rule refs, evidence refs, bundle/extension refs,
      developer team, entitlement approval, provisioning, signing,
      device/TestFlight, extension declaration/configuration, rollback, and
      audit refs; supervision/MDM proof is required only when claimed, and live
      Network Extension behavior, packet block, or app-level control is not
      claimed.
- [x] Linux nftables/eBPF/TUN adapter/proof gate.
      The Rust proof gate accepts distro readiness only with grade-A block
      policy, parent-rule refs, evidence refs, selected adapter kind,
      distro/kernel refs, permission, adapter API capability, adapter plan,
      service-manager scope, rollback, lab-result, and audit refs; generic Linux
      support, live adapter install, packet filtering, kernel hook load, TUN
      interface mutation, or service-manager install is not claimed.
- [x] Full-scope network plan, proof tiers, UI requirements, and workpacks.
- [ ] Real block/terminate/unavailable result.
- [x] No decrypted payload/page-content claim at the network contract boundary.
      Managed-browser correlation can attach exact URLs only from matching
      browser evidence, not from network metadata.
- [x] Unmanaged browser correlation remains process-only bypass/candidate
      evidence and cannot upgrade to exact URL, active-tab, title, page-content,
      policy, adapter, or enforcement-command claims.
- [x] App/game foreground/session correlation consumes stored evidence refs and
      keeps launcher-only and candidate rows non-authoritative before any policy,
      adapter, service, or portal claim.
- [x] Screen-summary trigger integration is parent-enabled and local-custody
      gated, with no raw-image retention, remote upload, policy, adapter, or
      enforcement authority.

## Next AI Instructions

Separate observation, attribution, classification, and enforcement. Add unknown
states instead of guessing a process, site, or category. Host network/domain
blocking needs explicit apply, rollback, and audit artifacts before any claim
upgrade beyond the current integrity runtime audit. Analyzer alerts and AI audit
reports are evidence inputs only; policy and adapter proof remain the authority
for any action. Do not wire a network-only event bus; implement the reusable
Rust eventing plan first.

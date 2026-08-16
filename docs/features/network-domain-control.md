<!-- agent-capsule -->

> Agent Capsule
> Doc: Network And Domain Control
> Kind: feature documentation; read only when selected by FEATURE_ROUTE_INDEX, PLAN_INDEX, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

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
  [network settings inventory](../plans/network-plan/workpacks/network-control-settings-inventory.md),
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
- E-D added row10b broker/family-hub remote delivery status proof in
  `agent-core`: broker and family-hub relay routes now materialize custody,
  publisher/subscriber auth, encryption, retention, replay, deletion, offset,
  dedupe, broker config, family-hub identity, and relay-policy refs into an
  explicit fixture-requirements-recorded-but-not-implemented status. The proof carries
  local idempotency/dead-letter evidence and keeps cross-process replay, remote
  retention/delete/export propagation, live broker/family-hub delivery, policy
  authority, side-effect authority, enforcement commands, adapter execution,
  and host filtering false.
- E-D added row10c remote event-chain journal/export proof in `agent-core`:
  local network runtime event-chain envelopes are written through the reusable
  `ocentra-eventing` NDJSON journal with hash-chain options, replayed as a
  projection-only export boundary, and tagged with journal, replay, export, and
  support-status refs. This still does not claim live broker/family-hub
  delivery, remote provider or child-device delivery, remote
  retention/delete/export propagation, policy authority, side-effect authority,
  adapter execution, enforcement-command publication, exact URL, decrypted
  payload, page content, or host filtering.
- E-D added row10d remote delivery receipt-ledger proof in `agent-core`:
  projection replay rows now produce deterministic local receipt records that
  preserve replay sequence, event id, event type, correlation id, event-chain
  journal refs, and local receipt-ack refs. The ledger marks receipt, replay,
  and support-status boundaries for future broker/family-hub delivery while
  keeping remote acknowledgement delivery, provider delivery, child-device
  delivery, policy authority, side-effect authority, adapter execution,
  enforcement-command publication, raw PCAP, exact URL, decrypted payload, page
  content, video content, private-message content, search-query content, and
  host filtering false for the row10d remote projection fixture. This is not a
  full available-metadata remote no-enforcement invariant.
- E-D added row10e remote delivery durable-envelope proof in `agent-core`: local
  receipt-ledger records now produce deterministic durable envelope records that
  preserve receipt sequence, event id, event type, correlation id,
  receipt-ledger refs, local receipt-ack refs, durable store refs, replay refs,
  delete/export readiness refs, and support-status refs. This marks the local
  durable envelope/store boundary for future broker/family-hub transport while
  keeping remote acknowledgement implementation, provider delivery,
  child-device delivery, remote delete/export propagation, product-ready remote
  delivery, policy authority, side-effect authority, adapter execution,
  enforcement-command publication, raw PCAP, exact URL, decrypted payload, page
  content, video content, private-message content, search-query content, and
  host filtering false.
- E-D added row10f remote delivery status bridge proof across `agent-protocol`,
  `agent-service`, and `agent-protocol-domain`: the service now answers
  `agent.network.remote-delivery.status.get` with
  `agent.network.remote-delivery.status.reported`, carrying broker/family-hub
  fixture requirement state, local idempotency/dead-letter proof,
  event-chain journal refs, receipt-ledger refs, local receipt-ack refs, and
  row10e durable envelope/store/replay/delete-export/support refs. The
  TypeScript parser rejects stale durable refs, missing requirement artifacts,
  live/product-ready delivery, remote acknowledgement, provider/child-device
  delivery, policy authority, side-effect authority, adapter execution,
  enforcement-command publication, raw PCAP, exact URL, decrypted payload, page
  content, video content, private-message content, search-query content, and
  host filtering claims.
- E-D added row10g remote delivery outbox handoff proof in `agent-core`: local
  durable envelope records now produce ordered prepared outbox candidates
  that preserve sequence, event id, event type, correlation id, durable-envelope
  refs, durable-store refs, receipt-ledger refs, local receipt-ack refs, outbox
  refs, handoff refs, replay refs, and support-status refs while rejecting
  duplicate durable envelope candidates before outbox preparation. The proof writes
  `output/network-plan-proof/10g-remote-delivery-outbox-handoff/proof-summary.json`
  and `test-results/network-remote-delivery-outbox-handoff-proof/proof.json`
  while keeping transport dispatch attempts, remote acknowledgements, live
  broker/family-hub delivery, provider/child-device delivery, remote
  delete/export propagation, product-ready remote delivery, policy authority,
  side-effect authority, adapter execution, enforcement-command publication,
  raw PCAP, exact URL, decrypted payload, page content, video content,
  private-message content, search-query content, and host filtering false.
- E-D added row10h remote delivery outbox status bridge proof across
  `agent-protocol`, `agent-service`, and `agent-protocol-domain`: the existing
  `agent.network.remote-delivery.status.get` /
  `agent.network.remote-delivery.status.reported` path preserves row10g
  prepared outbox refs, handoff refs, replay/support refs, prepared candidate
  counts, duplicate rejection, and zero dispatch/ack counters as typed
  read-only evidence. That evidence is now surfaced through the current row10k
  blocked-dispatch status payload. The TypeScript parser rejects stale outbox
  refs, stale row10h status refs, nonzero dispatch attempts, nonzero remote
  acknowledgements, mismatched prepared counts, live/product-ready delivery,
  policy authority, side-effect authority, adapter execution,
  enforcement-command publication, raw PCAP, exact URL, decrypted payload, page
  content, video content, private-message content, search-query content, and
  host filtering claims.
- E-D added row10i remote delivery dispatch-readiness proof in `agent-core`:
  prepared row10g outbox candidates now feed broker and family-hub dispatch
  gates that preserve eventing required-artifact refs and fixture-satisfied
  state while remaining manual-required until live transport implementation
  exists. The proof writes
  `output/network-plan-proof/10i-remote-delivery-dispatch-readiness/proof-summary.json`
  and
  `test-results/network-remote-delivery-dispatch-readiness-proof/proof.json`,
  keeps manual-required candidate count equal to prepared outbox count, keeps
  dispatch-ready candidates, dispatch attempts, and remote acknowledgements at
  zero, and rejects live broker/family-hub dispatch, product-ready delivery,
  policy authority, side-effect authority, adapter execution,
  enforcement-command publication, raw PCAP, exact URL, decrypted payload, page
  content, video content, private-message content, search-query content, and
  host filtering claims.
- E-D added row10j remote available-metadata no-enforcement invariant proof in
  `agent-core`: the row10b through row10i remote metadata chain now composes
  broker/family-hub requirement refs, event-chain journal/export refs,
  receipt-ledger/local-ack refs, durable envelope/store/delete-export refs,
  outbox handoff refs, and dispatch-readiness refs into one invariant that
  remains non-enforcing. The proof writes
  `output/network-plan-proof/10j-remote-delivery-no-enforcement-invariant/proof-summary.json`
  and
  `test-results/network-remote-delivery-no-enforcement-invariant-proof/proof.json`,
  rejects nonzero dispatch attempts, remote acknowledgements, live
  broker/family-hub delivery, product-ready delivery, policy authority,
  side-effect authority, adapter execution, enforcement-command publication,
  raw PCAP, exact URL, decrypted payload, page content, video content,
  private-message content, search-query content, and host filtering claims.
- E-D added row10k remote delivery transport dispatch-state proof across
  `agent-core`, `agent-protocol`, `agent-service`, and
  `agent-protocol-domain`: the row10j available-metadata invariant now feeds
  deterministic manual-required blocked dispatch records for every row10g
  prepared outbox candidate, and the existing remote-delivery status command
  serves a cached deterministic row10k protocol snapshot instead of rebuilding
  the full proof chain on every request. The proof writes
  `output/network-plan-proof/10k-remote-delivery-transport-dispatch-state/proof-summary.json`
  and
  `test-results/network-remote-delivery-transport-dispatch-state-proof/proof.json`,
  keeps blocked dispatch record count equal to source outbox and manual-required
  candidate counts, preserves outbox and handoff refs, rejects stale row10h
  status refs, and names a future transport seam without claiming live send or
  ack behavior. Dispatch-ready candidates, dispatch attempts, remote
  acknowledgements, broker/family-hub delivery, provider/child delivery, remote
  delete/export propagation, product-ready delivery, policy authority,
  side-effect authority, adapter execution, enforcement-command publication,
  raw PCAP, exact URL, decrypted payload, page content, video content,
  private-message content, search-query content, and host filtering remain
  false.
- E-D added row10l remote delivery fixture transport proof in `agent-core`:
  row10g prepared outbox candidates now produce fixture-only dispatch attempt
  and acknowledgement records that preserve event id, event type, correlation id,
  outbox refs, and handoff refs. The proof writes
  `output/network-plan-proof/10l-remote-delivery-fixture-transport/proof-summary.json`
  and
  `test-results/network-remote-delivery-fixture-transport-proof/proof.json`.
  Fixture acknowledgements are local proof records only and do not upgrade the
  service remote-delivery status payload or product support. Live
  broker/family-hub delivery, provider/child delivery, remote delete/export
  propagation, product-ready delivery, policy authority, side-effect authority,
  adapter execution, enforcement-command publication, raw PCAP, exact URL,
  decrypted payload, page content, video content, private-message content,
  search-query content, and host filtering remain false.
- E-D added row10m remote delete/export propagation readiness proof in
  `agent-core`: row10l fixture acknowledgements now produce proof-local remote
  delete and export readiness records that preserve event id, event type,
  correlation id, outbox refs, handoff refs, and fixture acknowledgement refs.
  The proof writes
  `output/network-plan-proof/10m-remote-delivery-delete-export-propagation/proof-summary.json`
  and
  `test-results/network-remote-delivery-delete-export-propagation-proof/proof.json`.
  The readiness records do not upgrade the service remote-delivery status
  payload or product support. Live broker/family-hub delivery, provider/child
  delivery, actual remote delete/export propagation, product-ready delivery,
  policy authority, side-effect authority, adapter execution,
  enforcement-command publication, raw PCAP, exact URL, decrypted payload, page
  content, video content, private-message content, search-query content, and
  host filtering remain false.
- E-D added row10n remote delete/export status bridge proof across
  `agent-protocol`, `agent-service`, and `agent-protocol-domain`: the existing
  `agent.network.remote-delivery.status.get` /
  `agent.network.remote-delivery.status.reported` path now carries row10m
  delete/export propagation readiness refs and record counts alongside the
  row10k blocked-dispatch refs under a row10n status identity. The proof writes
  `output/network-plan-proof/10n-remote-delivery-delete-export-status-bridge/proof-summary.json`
  and
  `test-results/network-remote-delivery-delete-export-status-bridge-proof/proof.json`.
  Rust and TypeScript parsers reject stale
  row10m refs, mismatched readiness counts, live/product-ready delivery, actual
  remote delete/export propagation, policy authority, side-effect authority,
  adapter execution, enforcement-command publication, raw PCAP, exact URL,
  decrypted payload, page content, video content, private-message content,
  search-query content, and host filtering claims.
- E-D added policy-preview stored-flow evidence proof across `agent-core`,
  `agent-service`, and `agent-protocol-domain`: stored ActivityStore network
  flow rows now feed the existing policy-preview read model, map
  `destinationDomain` to a domain policy target, resolve parent-rule contexts
  only when they cite the stored network activity event ref and match the event
  source device/platform scope, suppress retention-deleted flow rows before
  preview limits and rule matching, exclude stale/future/scheduled-without-proof
  contexts, apply the shared row34
  evidence-grade mapper so grade-B network block requests become parent-review
  ask-parent decisions, and serialize the latest dry-run decision through the
  service payload with `networkEvidenceGrade`,
  `networkRequestedPolicyAction`, `networkMappedPolicyAction`,
  `networkPolicyMappingMode`, and false network adapter/enforcement
  authorization flags. The proof writes
  `output/network-plan-proof/policy-preview-stored-flow-evidence/proof-summary.json`
  and `test-results/network-policy-preview-stored-flow-evidence-proof/proof.json`.
  It consumes the row34 evidence-grade mapper as the grade-specific dependency and
  does not claim AI execution, full policy-engine execution, adapter
  authorization, adapter action, enforcement-command publication, exact URL,
  decrypted payload, page content, video content, private-message content,
  search-query content, raw PCAP, or host filtering.
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
  the read-model payload. The same service payload now includes row51
  stored-flow product-path proof counts and refs: stored rows with a domain
  target derive row-scoped trigger/capture/ingest/typed-event refs through
  `ocentra-network-evidence` into policy-decision, action-result,
  retention/delete/export, and portal read-model refs. Captured network metadata
  events now carry durable local-DB evidence refs through the real ActivityStore
  into the same service product-path payload refs, while tombstoned rows and
  no-domain rows do not invent active policy/action refs. This is service-local
  proof metadata only; broker/family-hub transport, production
  retention/replay, policy execution, adapter execution, host filtering, exact
  URL/content, and external routing remain unclaimed.
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
- E-D added an aggregate full-network-plan proof pack that ties together
  contract, reusable eventing, parser/fixture, analyzer alert, AI/policy,
  adapter action, journal/read-model, UI, security-negative, performance,
  classification, remote-delivery through row10t external cross-process
  transport envelope/ack metadata, and end-to-end product-path proof artifacts
  under `output/network-plan-proof/full-network-plan/proof-summary.json` and
  `test-results/network-full-plan-proof/proof.json`. The proof keeps live
  capture drivers, live host adapter mutation/filtering, broker/family-hub
  delivery, provider/child delivery, product remote acknowledgements, remote
  delete/export propagation, production platform support, external audit signoff,
  exact content claims, and
  enforcement-command publication unclaimed.
- E-D added a row10o fixture transport status bridge proof:
  `network-remote-delivery-fixture-transport-status-bridge-proof` carries
  row10l fixture transport refs and fixture dispatch/ack counts through the
  existing typed remote-delivery status payload while keeping live dispatch
  attempts, live remote acknowledgements, broker/family-hub delivery,
  provider/child delivery, actual remote delete/export propagation,
  product-ready delivery, policy/side-effect authority, exact content, host
  filtering, adapter action, and enforcement-command publication unclaimed.
- E-D added a row10p provider/child readiness proof:
  `network-remote-delivery-provider-child-readiness-proof` maps row10l fixture
  acknowledgements into provider-route and child-device-route readiness records,
  surfaces provider/child readiness refs and manual-required unavailable states
  through the existing typed remote-delivery status payload, and keeps provider
  delivery artifacts, child-device delivery artifacts, live dispatch attempts,
  live remote acknowledgements, broker/family-hub delivery, actual remote
  delete/export propagation, product-ready delivery, policy/side-effect
  authority, exact content, host filtering, adapter action, and
  enforcement-command publication unclaimed.
- E-D added a row10q cross-process custody readiness proof:
  `network-remote-delivery-cross-process-custody-readiness-proof` maps row10p
  provider/child readiness records into proof-local cross-process replay,
  remote retention, remote delete custody, and remote export custody readiness
  records. The existing typed remote-delivery status payload now carries row10q
  refs and manual-required unavailable state while keeping cross-process replay
  artifacts, remote retention artifacts, remote delete/export custody artifacts,
  live broker/family-hub/provider/child delivery, actual remote delete/export
  propagation, product-ready delivery, policy/side-effect authority, exact
  content, host filtering, adapter action, and enforcement-command publication
  unclaimed.
- E-D added a row10r cross-process durable replay metadata proof:
  `network-remote-delivery-cross-process-replay-proof` turns row10e durable
  envelope records plus row10q custody/readiness records into deterministic
  replay records with durable-envelope refs, durable-store refs, receipt refs,
  row10q custody refs, replay-store refs, and replay-cursor refs. This moves
  cross-process replay from readiness-only evidence to local deterministic
  replay metadata while keeping live broker/family-hub/provider/child delivery,
  remote acknowledgement delivery, actual remote delete/export propagation,
  product-ready delivery, policy/side-effect authority, exact content, host
  filtering, adapter action, and enforcement-command publication unclaimed.
- E-D added a row10s cross-process replay status bridge proof:
  `network-remote-delivery-cross-process-replay-status-bridge-proof` exposes
  row10r deterministic replay metadata through the existing
  `agent.network.remote-delivery.status.get` /
  `agent.network.remote-delivery.status.reported` path. Rust protocol,
  service payload, and TypeScript parser coverage now carries row10r
  replay/store/cursor refs, replay record/store-write counts, cursor next
  sequence, and durable/custody match flags inside the current row10t status
  shape. The bridge keeps live broker/family-hub/provider/child delivery, remote
  acknowledgement delivery, remote delete/export propagation, product-ready
  delivery, policy/side-effect authority, exact content, host filtering, adapter
  action, and enforcement-command publication unclaimed.
- E-D added a row10t external cross-process transport proof:
  `network-remote-delivery-external-cross-process-transport-proof` turns row10r
  replay metadata into deterministic transport envelope and acknowledgement
  records with preserved durable-envelope refs, durable-store refs, row10r replay
  refs, and row10t transport refs. The existing
  `agent.network.remote-delivery.status.get` /
  `agent.network.remote-delivery.status.reported` path now carries the row10t
  status ref plus transport/envelope/ack refs, transport/envelope/ack counts,
  replay-match flags, and ack/envelope match flags. Live dispatch attempt and
  live remote acknowledgement counters remain zero, and the proof keeps live
  broker/family-hub/provider/child delivery, product remote acknowledgement
  delivery, actual remote delete/export propagation, product-ready delivery,
  policy/side-effect authority, exact content, host filtering, adapter action,
  and enforcement-command publication unclaimed.
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
- E-D added a service-visible row13a live-capture status bridge:
  `agent.network.live-capture.status.get` returns
  `agent.network.live-capture.status.reported` with the row13 proof-gate state,
  row03a raw-capture custody readiness refs, proof-ready/manual-required/
  unavailable/degraded platform counts, TypeScript parser rejection for stale
  refs/count drift/claim upgrades, and zero driver invocation, packet capture,
  raw-PCAP-without-custody, content, policy, adapter, enforcement, netstat
  substitution, remote-upload, and host-filter claims. The proof writes
  `output/network-plan-proof/13a-live-capture-service-readiness/proof-summary.json`
  and
  `test-results/network-live-capture-service-readiness-proof/proof.json`.
- E-D added a bounded row13b live-capture execution proof model:
  driver-backed execution is accepted only with proof-ready row13 capture refs
  plus driver invocation, interface observation, permission, bounded window,
  clean stop, custody, retention/delete/export, metadata-only sanitization, and
  private-traffic-exclusion refs. Windows metadata snapshots stay observable but
  cannot substitute for Npcap/libpcap capture. Raw artifact creation, raw PCAP
  without custody, exact content, policy, adapter, host-filter, and enforcement
  claims remain rejected.
- E-D added an Android physical target identity proof in
  `ocentra-network-evidence`: the named Galaxy S9 target must be reachable by
  read-only ADB connect, `adb devices -l`, and getprop probes, and its serial,
  product, model, device, Android release, ABI, command refs, and evidence refs
  must match before physical-device identity is proved. Missing ADB,
  disconnected targets, missing observations, and mismatches stay explicit. The
  proof rejects emulator-only product support, live VpnService execution,
  packet capture, packet blocking, app package correlation, Device Owner
  authority, production Android support, exact URL/content, adapter authority,
  host filtering, and enforcement-command claims.
- E-D added a bounded Windows Firewall lab execution proof in
  `ocentra-network-evidence`: an apply-ready row38 Windows Firewall adapter
  proof can be paired with an Ocentra-specific lab rule name, an RFC 5737
  TEST-NET remote address, administrator permission, and apply/verify-present/
  rollback/verify-removed command evidence before the lab proof can report
  executed-and-rolled-back. Without Windows host/admin/command evidence it stays
  manual-required or unavailable. The proof writes
  `output/network-plan-proof/38a-windows-firewall-lab-execution-proof/proof-summary.json`
  and
  `test-results/network-windows-firewall-lab-execution-proof/proof.json`, and
  rejects production enforcement, persistent firewall rules, policy execution,
  enforcement-command publication, exact URL, decrypted payload, and page
  content claims.
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
- E-D added an integrated event-plus-network product-path proof in
  `ocentra-network-evidence` and the service network read-model path: one
  deterministic artifact carries stored-row trigger refs, typed-event refs,
  evidence refs, local-AI queue refs, AI detection/audit refs,
  policy/risk-budget refs, adapter proof refs, audit refs, portal read-model
  refs, and retention/delete/export refs while proving weak or unavailable
  evidence cannot authorize adapter apply and AI/UI/network surfaces cannot
  bypass policy. Tombstoned rows and rows without domain targets do not invent
  policy/action refs.
- E-D added a platform-claim manifest proof in `ocentra-network-evidence` that
  composes Windows Firewall/WFP, Android VpnService, Apple Network Extension
  macOS/iOS, and Linux nftables/eBPF/TUN gates into fixture platform-scope,
  permission/entitlement, capability, and audit-ref rows, records missing
  artifacts as manual follow-ups, and rejects generic platform support, live
  adapter execution, UI policy authority, and enforcement-command claims.
- E-D added an action-result state proof in `ocentra-network-evidence` for
  network block/terminate/unavailable outcomes. Blocked and terminated result
  states require grade-A block policy refs, apply-ready adapter proof refs,
  adapter result artifacts, and audit refs; dry-run, manual-required,
  unavailable, weak-evidence, and invalid terminate-target states stay
  non-result, and live host mutation or enforcement-command publication is
  rejected.
- Network/domain blocking is not broadly product-complete.
- Raw network control settings are preserved as design inputs, not
  product-complete implementation proof.

## Current Gap

Real OS/domain blocking adapter execution, broader DNS/VPN/proxy handling,
attribution quality, production live capture driver support, live analyzer
fixture proof, broker/family-hub transport, remote network runtime event-chain
delivery, product remote acknowledgements, raw PCAP/live-capture retention,
Device Owner or other Android authority-enrolled proof,
production risk-budget service/UI wiring,
production SLO validation,
external audit/deployment execution, full support-material authoring, and
parent-facing rule UX remain.
The Windows Firewall lab execution proof narrows the Windows host proof gap to a
bounded, reversible TEST-NET lab rule only; product enforcement, persistent
rules, and parent-policy host filtering still require a production adapter path.
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
follow-ups for missing artifacts. Row52 also writes a local platform observation
artifact for this Windows host: Windows Firewall is read-only observed, Windows
WFP remains manual-required, Android SDK/emulator visibility is recorded without
execution, the row40a Android physical target identity proof records the named
Galaxy S9 as a read-only matching target, Linux WSL tool readiness is lab-ready,
and macOS/iOS stay unavailable or CI/manual-device-required from Windows. Row53 now records
block, terminate, dry-run, manual-required, and unavailable action-result states
from policy refs and adapter proof artifacts without claiming live adapter
execution or enforcement command publication.
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
- [x] VPN/proxy/tunnel indicators where available.
      Deterministic indicator classification now exists for VPN adapter,
      proxy-port, Tor, and tunnel-protocol candidates, with encrypted-DNS-only
      negative proof and no hidden-destination claim. E-D generated
      `output/network-plan-proof/24-vpn-proxy-tor-tunnel-classifier/proof-summary.json`
      and
      `test-results/network-vpn-proxy-tunnel-classifier-proof/proof.json`.
- [x] Network category/risk targets.
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
- [x] Reusable Rust eventing, detection, AI audit, and risk-budget contracts.
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
      Row40a Android physical target identity proof now records the named
      Galaxy S9 target through read-only ADB connect, `adb devices -l`, and
      getprop probes, requiring expected product/model/device/release/ABI refs
      to match before physical-device identity is proved. This does not claim
      live VpnService filtering, packet capture, packet blocking, app package
      correlation, production Android support, Device Owner authority, adapter
      authority, or enforcement-command publication.
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
      gates into fixture platform-scope, permission/entitlement, capability,
      and audit-ref rows, records manual follow-ups for missing artifacts, adds
      local Windows/Android/WSL observation evidence while keeping Apple
      CI/manual-unavailable on this host, and rejects generic platform support,
      live adapter execution, non-ready
      adapter authorization, UI policy authority, and enforcement-command
      claims. E-D portal status proof now
      renders service-backed network
      platform/capability state, active/tombstone/exportable row counts,
      retention delete refs, and degraded adapter state in the Activity network
      drawer while keeping policy, adapter, AI, exact URL, and enforcement refs
      not reported unless the service provides them. AI model execution,
      broker/family-hub delivery implementation, full policy engine execution,
      notification delivery, external audit/deployment execution, full
      support-material authoring, and true risk-budget/performance SLO UI
      rendering remain. E-D full-network proof now ties the consolidated
      eventing proof, network runtime proof, row46 AI detection, row47 AI audit,
      row48 risk-budget, row49 performance, row50 security, row51 product path,
      row52 platform claim, row53 action-result, and row10 remote-delivery
      non-enforcement artifacts together under
      `output/network-plan-proof/full-network-plan/proof-summary.json` and
      `test-results/network-full-plan-proof/proof.json`.
- [x] Parent portal network evidence drawer.
      The Activity route renders real Rust service network read-model output,
      ActivityStore evidence refs, endpoint/domain/process attribution, and
      unsupported-claim states without publishing policy or adapter commands.
      E-D portal status proof also renders service-backed platform/capability
      state, active/tombstone/exportable row counts, retention delete refs, and
      degraded adapter state without local risk scoring or adapter authority.
- [x] Policy preview over stored flow evidence.
      Stored ActivityStore network flow rows now feed the existing
      policy-preview read model and service payload with parent-rule evidence
      refs, source-device-scoped context filtering, stale/future/scheduled
      context rejection, pre-limit retention-deleted flow row suppression, row34
      evidence-grade mapping that downgrades grade-B block requests to
      parent-review ask-parent, dry-run decision state, and disabled enforcement
      handoff plus service payload provenance fields for network evidence grade,
      requested action, mapped action, mapping mode, and false adapter/
      enforcement authorization. The portal live-activity parser now retains
      those provenance fields and rejects adapter/enforcement authorization
      claims. Proof:
      `output/network-plan-proof/policy-preview-stored-flow-evidence/proof-summary.json`
      and
      `test-results/network-policy-preview-stored-flow-evidence-proof/proof.json`.
      Row34 is consumed as the evidence-grade policy mapper dependency; this item does
      not claim AI execution, full policy-engine execution, adapter
      authorization, enforcement command publication, exact URL/content, raw
      PCAP, or host filtering.
- [x] Adapter capability status.
      Row37 DNS adapter, Row38 Windows Firewall, Row39 WFP, and Row40 Android
      VpnService plus Row41 Apple Network Extension and Row42 Linux proof gates
      model supported/lab-ready/physical-device-ready/Apple-device-ready/
      distro-ready, manual-required, and unavailable capability states for their
      adapter boundaries. Row11 now records the aggregate manual/platform proof
      pack for OS/device/permission artifacts and exact manual steps, and Row52
      accounts for those claims in one platform-scope/permission/capability
      manifest with manual follow-ups. The Activity network drawer now surfaces
      read-model
      capability/platform status for the current service row; broader platform
      capability UX beyond the network drawer remains open. The E-D adapter
      capability status proof now projects target-specific supported/lab-ready/
      physical-device-ready/Apple-device-ready/distro-ready, dry-run,
      research-only, manual-required, and unavailable rows from the existing
      Row52 platform manifest through a locked target-to-status mapping. Proof:
      `output/network-plan-proof/adapter-capability-status/proof-summary.json`
      and `test-results/network-adapter-capability-status-proof/proof.json`.
      It rejects adapter authorization on non-ready status rows so dry-run,
      research-only, manual-required, and unavailable states cannot contradict
      their non-executable status.
      This proof keeps live adapter execution, host filtering, production
      platform support, broader platform capability UX, UI policy authority,
      and enforcement-command publication unclaimed.
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
- [x] Windows Firewall bounded lab execution proof.
      The Rust proof accepts executed-and-rolled-back only with an apply-ready
      row38 adapter proof, an Ocentra lab rule name, an RFC 5737 TEST-NET target,
      administrator permission, and apply/verify-present/rollback/verify-removed
      command evidence. It records manual-required or unavailable state when the
      host/admin/command evidence is absent and rejects production enforcement,
      persistent firewall rules, policy execution, enforcement commands, and
      exact-content claims.
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
      Row40a separately proves the named physical Android target identity
      through read-only ADB evidence and keeps VpnService execution, packet
      capture, packet blocking, app package correlation, Device Owner authority,
      production Android support, adapter authority, host filtering, and
      enforcement-command publication unclaimed.
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
- [x] Real block/terminate/unavailable result.
      Row53 action-result state proof records blocked, terminated, dry-run,
      manual-required, and unavailable result states from policy refs,
      apply-ready adapter proof refs, adapter result artifacts, and audit refs.
      Weak evidence, parent-review policy, invalid terminate targets,
      unavailable capabilities, exact URL/content claims, host mutation claims,
      and enforcement-command publication stay rejected.
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

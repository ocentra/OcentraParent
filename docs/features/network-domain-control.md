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
- E-D added the first `activity-domain` network contract boundary proof for
  flow evidence, domain evidence, activity classification, A/B/C/D evidence
  grades, and policy/action capability gating. The proof keeps network-only
  exact URL/content claims rejected and keeps dry-run/manual-required states
  from authorizing adapter calls.
- Network/domain blocking is not broadly product-complete.
- Raw network control settings are preserved as design inputs, not
  product-complete implementation proof.

## Current Gap

Real OS/domain blocking adapter proof, DNS/VPN/proxy handling, attribution
quality, PCAP/analyzer fixture proof, Rust protocol parity for network
contracts, AI detection and audit proof, risk-budget proof, performance proof,
and parent-facing rule UX remain. Policy dispatch does not upgrade
network/domain blocking beyond
manual-required, and the broad-adapter proof and supported-adapter runtime proof
keep the same manual-required host-filter boundary. The integrity runtime audit
proves observe-only and manual-required state visibility, not DNS, VPN, packet,
signature alert, production risk-budget, or host filter enforcement. The E-D
runtime spine removes the private-bus blocker for an in-process metadata-only
chain, but packet/analyzer fixtures, broker delivery, portal UI, and adapter
apply/rollback/audit artifacts remain open.

## Checklist

- [x] Flow summary and flow-evidence contracts.
- [x] Domain/IP/protocol/process attribution status contracts.
- [ ] VPN/proxy/tunnel indicators where available.
- [ ] Network category/risk targets.
- [ ] PCAP fixture, Zeek-style summary, and Suricata/Snort-compatible alert
      proof.
- [ ] Reusable Rust eventing, detection, AI audit, and risk-budget contracts.
      First E-D runtime spine exists for metadata-only flow events and
      manual-required/unavailable states; production analyzer, AI model, and
      risk-budget fixtures remain.
- [ ] Policy preview over stored flow evidence.
- [ ] Adapter capability status.
- [x] Full-scope network plan, proof tiers, UI requirements, and workpacks.
- [ ] Real block/terminate/unavailable result.
- [x] No decrypted payload/page-content claim at the network contract boundary.

## Next AI Instructions

Separate observation, attribution, classification, and enforcement. Add unknown
states instead of guessing a process, site, or category. Host network/domain
blocking needs explicit apply, rollback, and audit artifacts before any claim
upgrade beyond the current integrity runtime audit. Analyzer alerts and AI audit
reports are evidence inputs only; policy and adapter proof remain the authority
for any action. Do not wire a network-only event bus; implement the reusable
Rust eventing plan first.

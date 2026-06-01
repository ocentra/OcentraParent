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
  [network settings inventory](../network-control-settings-inventory.md).
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
- Network/domain blocking is not broadly product-complete.
- Raw network control settings are preserved as design inputs, not
  product-complete implementation proof.

## Current Gap

Real OS/domain blocking adapter proof, DNS/VPN/proxy handling, attribution
quality, and parent-facing rule UX remain.

## Checklist

- [ ] Flow summary contracts.
- [ ] Domain/IP/protocol/process attribution status.
- [ ] VPN/proxy/tunnel indicators where available.
- [ ] Network category/risk targets.
- [ ] Policy preview over stored flow evidence.
- [ ] Adapter capability status.
- [ ] Real block/terminate/unavailable result.
- [ ] No decrypted payload/page-content claim.

## Next AI Instructions

Separate observation, attribution, classification, and enforcement. Add unknown
states instead of guessing a process, site, or category.

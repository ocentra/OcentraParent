# Parent Assistant Actions

## Parent Outcome

A parent can ask Ocentra to explain activity, draft a rule, create a schedule,
preview a policy action, tune sensitivity, or prepare a report without learning
every setting first.

## Ocentra Requirement

The parent assistant is an operator over typed contracts. It can explain,
suggest, draft, preview, and request parent approval. It must not bypass
child-device validation or directly enforce.

## Roadmap And Expectations

- Roadmap: V4 reports and optional assistant, V5 parent policy product.
- Expectations: [parent assistant](../expectations/parent-assistant-chat.md),
  [AI](../expectations/ai.md), [policy](../expectations/policy.md),
  [data custody](../expectations/data-custody.md).
- Modules: `packages/parent-domain`, `packages/agent-protocol-domain`,
  `apps/portal`, `crates/agent-service`.

## Competitor Pressure

See [Competitor Capability Map](../competitor-capability-map.md), especially AI
assistant setup/control, reports/digests, and remote parent access.

Most competitors provide dashboards, alerts, and settings. Ocentra can
differentiate by letting high-tech and low-tech parents use natural language to
configure, explain, and preview rules.

## Current Ocentra State

- Parent assistant contracts and MIA/activity evidence context proof exist.
- Parent Assistant now uses the local service to load the latest saved Activity
  report history as evidence context when the command does not already carry a
  report payload, while preserving explicit report/history payload precedence.
- MIA/report-history evidence contexts now carry typed parent-owned custody and
  source labels, `rawChildEvidenceIncluded=false`, and
  `directEnforcementAllowed=false`.
- Parent Assistant answers and provider-status events now carry a typed
  provider route for local configured/degraded/unavailable and API
  authorized-unavailable/authorized-degraded states. The route keeps remote/API
  AI optional, cites saved report/read-model source refs, and preserves
  `childSafetyOrEnforcementUseAllowed=false`.
- `local-ai-parent-assistant-runtime-proof` records the focused local AI
  assistant runtime boundary: local answers use the shared provider scheduler,
  busy providers degrade or queue without duplicate runtime loads, unavailable
  states remain cited and explicit, API AI stays optional, and action
  preview/confirm cannot write policy or enforce directly.
- Finished portal chat/action flow is not done.

## Current Gap

Assistant threads, provider status UI, parent approval, child-agent validation,
real API provider adapters, and rule-writing flow are incomplete as one product
flow. Saved report citations, cited answers, action previews, and backend
provider routing have runtime proof, but the portal chat surface still needs
C-owned integration. MIA evidence context remains read-model/report citation
context only; it does not transfer raw child evidence, write policy, or enforce
on child devices.

## Checklist

- [ ] Assistant thread contract.
- [x] Evidence-cited answer contract.
- [x] Action preview contract.
- [ ] Draft rule/schedule/approval actions.
- [x] Provider status and degraded state.
- [ ] Parent confirmation before write.
- [ ] Child-agent validation of actions.
- [ ] Portal chat and audit.

## Next AI Instructions

Do not create assistant-only policy. Every suggested action must become a typed
preview, then a parent-confirmed intent, then child-agent validation.
